//! ZW - Lightning Blade
use super::prelude::*;

pub const ID: u32 = 45082499;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_UTOPIA, SET_ZW], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // equip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_MZONE);
    effect::set_condition(d, e1, Some(lua_fn(eqcon_fn)));
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    effect::set_operation(d, e1, Some(operation(eqop)));
    card::register_effect(d, c, e1, false);
    // aux.AddZWEquipLimit(c,s.eqcon,function(tc,c,tp) return s.filter(tc) and tc:IsControler(tp) end,s.equipop,e1)
    aux::add_zw_equip_limit(
        d,
        c,
        Some(lua_fn(eqcon_fn)),
        lua_fn(equipval),
        lua_fn(|d, a: Args| equipop(d, a.c(0), a.e(1), a.p(2), a.c(3))),
        e1,
        None,
        None,
        None,
    );
    // "ZW" cards cannot be destroyed by your opponent's effects while this card is equipped to a monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, LOCATION_ONFIELD, 0);
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_ZW as u16]))));
    effect::set_condition(d, e2, Some(lua_fn(|d, a: Args| {
        let h = effect::get_handler(d, a.e(0)).nil();
        card::get_equip_target(d, h).is_some()
    })));
    effect::set_value_fn(d, e2, aux::indoval());
    card::register_effect(d, c, e2, false);
    // Destruction replacement for the equipped monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e3, EFFECT_DESTROY_SUBSTITUTE);
    effect::set_value_fn(d, e3, lua_fn(repval));
    card::register_effect(d, c, e3, false);
}
/// `function(e) return e:GetHandler():CheckUniqueOnField(e:GetHandlerPlayer()) end` (`s.eqcon`)
fn eqcon_fn(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let h = effect::get_handler(d, e).nil();
    card::check_unique_on_field(d, h, effect::get_handler_player(d, e), None, None)
}
/// `s.filter(c)`: `c:IsFaceup() and c:IsSetCard(SET_UTOPIA)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_UTOPIA as u16])
}
/// `function(tc,c,tp) return s.filter(tc) and tc:IsControler(tp) end`
fn equipval(d: &mut Duel, a: Args) -> bool {
    let (tc, tp) = (a.c(0), a.p(2));
    filter(d, tc) && card::is_controler(d, tc, tp)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // if c:IsLocation(LOCATION_MZONE) and c:IsFacedown() then return end
    if card::is_location(d, c, LOCATION_MZONE) && card::is_facedown(d, c) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc_opt = duel::first_target(d);
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 or not tc:IsRelateToEffect(e) or tc:IsFacedown()
    //     or tc:GetControler()~=tp or not c:CheckUniqueOnField(tp) then
    let ok = if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        false
    } else {
        let tc = tc_opt.nil();
        card::is_relate_to_effect(d, tc, e) && !card::is_facedown(d, tc) && card::get_controler(d, tc) == tp && card::check_unique_on_field(d, c, tp, None, None)
    };
    if !ok {
        // Duel.SendtoGrave(c,REASON_EFFECT)
        duel::send_to_grave(d, c, REASON_EFFECT, None, None);
        return;
    }
    // s.equipop(c,e,tp,tc)
    equipop(d, c, e, tp, tc_opt.nil());
}
/// `s.equipop(c,e,tp,tc)`
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) {
    // if not aux.EquipAndLimitRegister(c,e,tp,tc) then return end
    if !aux::equip_and_limit_register(d, c, e, tp, tc, None, None) {
        return;
    }
    // atkup
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(1200)
    effect::set_value(d, e1, 1200);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.repval(e,re,r,rp)`: `(r&REASON_EFFECT)~=0`
fn repval(_d: &mut Duel, a: Args) -> bool {
    (a.int(2) as u32 & REASON_EFFECT) != 0
}
