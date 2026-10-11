//! Inzektor Earwig
use super::prelude::*;

pub const ID: u32 = 38450736;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_INZEKTOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip 1 "Inzektor" monster from your hand or GY to this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.AddEREquipLimit(c,nil,s.eqval,s.equipop,e1)
    aux::add_er_equip_limit(
        d,
        c,
        None,
        lua_fn(|d, a| eqval(d, a.c(0), a.p(2))),
        lua_fn(|d, a| equipop(d, a.c(0), a.e(1), a.p(2), a.c(3))),
        e1,
        None,
        None,
        None,
    );
    // While this card is equipped to a monster, that monster gains ATK and DEF equal to this card's ATK and DEF
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetValue(1000)
    effect::set_value(d, e2, 1000);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // e3:SetValue(1000)
    effect::set_value(d, e3, 1000);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // The monster this card was equipped to gains 1000 ATK
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    // e3:SetCondition(s.atkcon)
    effect::set_condition(d, e3, Some(condition(atkcon)));
    // e3:SetTarget(s.atktg)
    effect::set_target(d, e3, Some(cost_target(atktg)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.eqval(ec,c,tp)`
fn eqval(d: &mut Duel, ec: CardId, tp: u8) -> bool {
    // return ec:IsControler(tp) and ec:IsSetCard(SET_INZEKTOR)
    card::is_controler(d, ec, tp) && card::is_set_card(d, ec, &[SET_INZEKTOR as u16])
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_INZEKTOR) and c:IsMonster() and not c:IsForbidden()
    card::is_set_card(d, c, &[SET_INZEKTOR as u16]) && card::is_monster(d, c) && !card::is_forbidden(d, c)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //   and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_GRAVE|LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_GRAVE | LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,nil,1,tp,LOCATION_GRAVE|LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_HAND) as i32);
    true
}
/// `s.equipop(c,e,tp,tc)`
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) {
    // c:EquipByEffectAndLimitRegister(e,tp,tc,nil,true)
    card::equip_by_effect_and_limit_register(d, c, e, tp, tc, None, Some(true));
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // if c:IsFacedown() or not c:IsRelateToEffect(e) then return end
    if card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.filter),tp,LOCATION_GRAVE|LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(filter)), a.tp, LOCATION_GRAVE | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // s.equipop(c,e,tp,tc)
        equipop(d, c, a.e, a.tp, tc);
    }
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ec=c:GetEquipTarget()
    let ec = card::get_equip_target(d, c);
    // e:SetLabelObject(ec)
    effect::set_label_object(d, a.e, match ec {
        Some(x) => Value::Card(x),
        None => Value::Nil,
    });
    // return ec and c:IsLocation(LOCATION_GRAVE) and ec:IsFaceup() and ec:IsLocation(LOCATION_MZONE)
    match ec {
        Some(ec) => card::is_location(d, c, LOCATION_GRAVE) && card::is_faceup(d, ec) && card::is_location(d, ec, LOCATION_MZONE),
        None => false,
    }
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local ec=e:GetLabelObject()
    let ec = effect::get_label_object(d, a.e).as_card();
    // Duel.SetTargetCard(ec)
    duel::set_target_card(d, ec.nil());
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local ec=e:GetLabelObject()
    let ec = effect::get_label_object(d, a.e).as_card().nil();
    // if ec:IsLocation(LOCATION_MZONE) and ec:IsFaceup() and ec:IsRelateToEffect(e) then
    if card::is_location(d, ec, LOCATION_MZONE) && card::is_faceup(d, ec) && card::is_relate_to_effect(d, ec, a.e) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let c = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(1000)
        effect::set_value(d, e1, 1000);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // ec:RegisterEffect(e1)
        card::register_effect(d, ec, e1, false);
    }
}
