//! ZW - Sylphid Wing
use super::prelude::*;

pub const ID: u32 = 95886782;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_UTOPIA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // Equip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetRange(LOCATION_HAND|LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_MZONE);
    // e1:SetCondition(s.eqcon)
    effect::set_condition(d, e1, Some(condition(|d, a| eqcon(d, a.e))));
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.AddZWEquipLimit(c,s.eqcon,function(tc,c,tp) return s.filter(tc) and tc:IsControler(tp) end,s.equipop,e1)
    aux::add_zw_equip_limit(
        d,
        c,
        Some(lua_fn(|d, a: Args| eqcon(d, a.e(0)))),
        lua_fn(|d, a: Args| {
            let tc = a.c(0);
            let tp = a.p(2);
            filter(d, tc) && card::is_controler(d, tc, tp)
        }),
        lua_fn(|d, a: Args| {
            equipop(d, a.c(0), a.e(1), a.p(2), a.c(3));
            Value::Nil
        }),
        e1,
        None,
        None,
        None,
    );
    // Increase ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Detach replace
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_OVERLAY_REMOVE_REPLACE)
    effect::set_code(d, e3, EFFECT_OVERLAY_REMOVE_REPLACE);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCondition(s.rcon)
    effect::set_condition(d, e3, Some(condition(rcon)));
    // e3:SetOperation(s.rop)
    effect::set_operation(d, e3, Some(operation_count(rop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.eqcon(e)`: `return e:GetHandler():CheckUniqueOnField(e:GetHandlerPlayer())`
fn eqcon(d: &mut Duel, e: EffectId) -> bool {
    let h = effect::get_handler(d, e).nil();
    let p = effect::get_handler_player(d, e);
    card::check_unique_on_field(d, h, p, None, None)
}
/// `s.filter(c)`: `return c:IsFaceup() and c:IsSetCard(SET_UTOPIA)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_UTOPIA as u16])
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil) end
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
    let tp = a.tp;
    let e = a.e;
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
    let tc = duel::first_target(d);
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 or not tc:IsRelateToEffect(e) or tc:IsFacedown()
    //     or tc:GetControler()~=tp or not c:CheckUniqueOnField(tp) then
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0
        || !card::is_relate_to_effect(d, tc.nil(), e)
        || card::is_facedown(d, tc.nil())
        || card::get_controler(d, tc.nil()) != tp
        || !card::check_unique_on_field(d, c, tp, None, None)
    {
        // Duel.SendtoGrave(c,REASON_EFFECT)
        duel::send_to_grave(d, c, REASON_EFFECT, None, None);
        return;
    }
    // s.equipop(c,e,tp,tc)
    equipop(d, c, e, tp, tc.nil());
}
/// `s.equipop(c,e,tp,tc)`
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) {
    // if not aux.EquipAndLimitRegister(c,e,tp,tc) then return end
    if !aux::equip_and_limit_register(d, c, e, tp, tc, None, None) {
        return;
    }
    // Increase ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(800)
    effect::set_value(d, e1, 800);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re and re:IsActivated() and e:GetHandler():GetEquipTarget() and eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp)
    let Some(re) = a.re else { return false };
    if !effect::is_activated(d, re) {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    if card::get_equip_target(d, h).is_none() {
        return false;
    }
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, 1 - tp), 1, NO_EX)
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // local ec=c:GetEquipTarget()
        let ec = card::get_equip_target(d, c).nil();
        // ec:UpdateAttack(1600,nil,c)
        card::update_attack(d, ec, 1600, None, Some(c), None);
    }
}
/// `s.rcon(e,tp,eg,ep,ev,re,r,rp)`
fn rcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    let re = a.re.nil();
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, re).nil();
    // return (r&REASON_COST)~=0 and re:IsActivated() and re:IsActiveType(TYPE_XYZ)
    //     and rc:GetEquipGroup():IsContains(c) and c:IsAbleToGraveAsCost()
    //     and ep==e:GetOwnerPlayer() and ev>=1 and rc:GetOverlayCount()>=ev-1
    (a.r & REASON_COST) != 0
        && effect::is_activated(d, re)
        && effect::is_active_type(d, re, TYPE_XYZ)
        && group::is_contains(&card::get_equip_group(d, rc), c)
        && card::is_able_to_grave_as_cost(d, c)
        && a.ep == effect::get_owner_player(d, a.e)
        && a.ev >= 1
        && card::get_overlay_count(d, rc) as i64 >= a.ev as i64 - 1
}
/// `s.rop(e,tp,eg,ep,ev,re,r,rp)`: `return Duel.SendtoGrave(e:GetHandler(),REASON_COST)`
/// (a number, which is always true in Lua)
fn rop(d: &mut Duel, a: &EffArgs) -> i64 {
    let h = effect::get_handler(d, a.e).nil();
    duel::send_to_grave(d, h, REASON_COST, None, None) as i64
}
