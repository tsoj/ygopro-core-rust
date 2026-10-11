//! Diskblade Rider
use super::prelude::*;

pub const ID: u32 = 41113025;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish 1 Normal Trap and increase ATK
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_ATKCHANGE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    effect::set_operation(d, e1, Some(operation(rmop)));
    card::register_effect(d, c, e1, false);
}
/// `s.rmfilter(c)`: `c:IsNormalTrap() and c:IsAbleToRemove()`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_normal_trap(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && rmfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&rmfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let g = duel::select_target(d, a.tp, Some(&rmfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if tc:IsRelateToEffect(e) and tc:IsNormalTrap() and Duel.Remove(tc,POS_FACEUP,REASON_COST)>0
    //     and c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e)
        && card::is_normal_trap(d, tc)
        && duel::remove(d, tc, POS_FACEUP as u8, REASON_COST, None, None) > 0
        && card::is_faceup(d, c)
        && card::is_relate_to_effect(d, c, a.e)
    {
        // Increase ATK
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(500)
        effect::set_value(d, e1, 500);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END|RESET_OPPO_TURN)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END | RESET_OPPO_TURN, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
