//! Solitary Sword of Poison
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 93377803;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Duel.EnableGlobalFlag(GLOBALFLAG_SELF_TOGRAVE)
    duel::enable_global_flag(d, GLOBALFLAG_SELF_TOGRAVE);
    // c:SetUniqueOnField(LOCATION_ONFIELD,0,id)
    card::set_unique_on_field(d, c, LOCATION_ONFIELD as u8, 0, card::UniqueCode::Code(ID), None);
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Equip limit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EFFECT_EQUIP_LIMIT)
    effect::set_code(d, e2, EFFECT_EQUIP_LIMIT);
    // e2:SetValue(s.eqlimit)
    effect::set_value_fn(d, e2, effect_card_fn(eqlimit));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // self destroy
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCode(EFFECT_SELF_TOGRAVE)
    effect::set_code(d, e3, EFFECT_SELF_TOGRAVE);
    // e3:SetCondition(s.sdcon)
    effect::set_condition(d, e3, Some(condition(sdcon)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // double atk/def
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e4, EFFECT_TYPE_EQUIP);
    // e4:SetCode(EFFECT_SET_BASE_ATTACK)
    effect::set_code(d, e4, EFFECT_SET_BASE_ATTACK);
    // e4:SetCondition(s.adcon)
    effect::set_condition(d, e4, Some(condition(adcon)));
    // e4:SetValue(s.atkval)
    effect::set_value_fn(d, e4, effect_card_value(atkval));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EFFECT_SET_BASE_DEFENSE)
    effect::set_code(d, e5, EFFECT_SET_BASE_DEFENSE);
    // e5:SetValue(s.defval)
    effect::set_value_fn(d, e5, effect_card_value(defval));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsControler(e:GetHandlerPlayer())
    let p = effect::get_handler_player(d, e);
    card::is_controler(d, c, p)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if e:GetHandler():IsRelateToEffect(e) and tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:IsControler(tp) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        let tc = tc.nil();
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && card::is_controler(d, tc, a.tp) {
            // Duel.Equip(tp,e:GetHandler(),tc)
            duel::equip(d, a.tp, h, tc, true, false);
        }
    }
}
/// `s.sdcon(e)`
fn sdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=c:GetEquipTarget()
    let tc = card::get_equip_target(d, c);
    // return tc and Duel.IsExistingMatchingCard(nil,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,tc)
    match tc {
        None => false,
        Some(tc) => {
            let p = effect::get_handler_player(d, a.e);
            duel::is_existing_matching_card(d, None, p, LOCATION_MZONE, 0, 1, tc.into())
        }
    }
}
/// `s.adcon(e)`
fn adcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if Duel.GetCurrentPhase()~=PHASE_DAMAGE_CAL then return false end
    if duel::get_current_phase(d) != PHASE_DAMAGE_CAL {
        return false;
    }
    // local eqc=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let eqc = card::get_equip_target(d, h);
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let dt = duel::get_attack_target(d);
    // return d and (a==eqc or d==eqc)
    match dt {
        None => false,
        Some(dt) => att == eqc || Some(dt) == eqc,
    }
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetBaseAttack()*2
    (card::get_base_attack(d, c) * 2) as i64
}
/// `s.defval(e,c)`
fn defval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetBaseDefense()*2
    (card::get_base_defense(d, c) * 2) as i64
}
