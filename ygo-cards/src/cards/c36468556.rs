//! Ceasefire
use super::prelude::*;

pub const ID: u32 = 36468556;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsFaceup() and c:IsType(TYPE_EFFECT)) or c:IsPosition(POS_FACEDOWN_DEFENSE)
    (card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT)) || card::is_position(d, c, POS_FACEDOWN_DEFENSE)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_EFFECT)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // local dam=Duel.GetMatchingGroupCount(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)*500
    let dam = duel::get_matching_group_count(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX) as i64 * 500;
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(dam)
    duel::set_target_param(d, dam as u32);
    // if dam>0 then Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam) end
    if dam > 0 {
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dam as i32);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsPosition,tp,LOCATION_MZONE,LOCATION_MZONE,nil,POS_FACEDOWN_DEFENSE)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_position(d, c, POS_FACEDOWN_DEFENSE)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.ChangePosition(g,0,0,0,POS_FACEUP_DEFENSE,true)
    duel::change_position(d, g, 0, Some(0), Some(0), Some(POS_FACEUP_DEFENSE as u8), true);
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local dam=Duel.GetMatchingGroupCount(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)*500
    let dam = duel::get_matching_group_count(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX) as i64 * 500;
    // Duel.Damage(p,dam,REASON_EFFECT)
    duel::damage(d, p, dam, REASON_EFFECT, false, None);
}
