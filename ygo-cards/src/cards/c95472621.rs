//! Big Burn
//! 大火葬
use super::prelude::*;

pub const ID: u32 = 95472621;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_BECOME_TARGET)
    effect::set_code(d, e1, EVENT_BECOME_TARGET);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_GRAVE) and c:IsMonster()
    card::is_location(d, c, LOCATION_GRAVE) && card::is_monster(d, c)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp~=tp and eg:IsExists(s.cfilter,1,nil)
    if a.rp == a.tp {
        return false;
    }
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &cfilter, 1, NO_EX)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToRemove() and aux.SpElimFilter(c)
    card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, false, false)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX);
    }
    // if Duel.IsPlayerAffectedByEffect(e:GetHandlerPlayer(),CARD_SPIRIT_ELIMINATION) then
    let hp = effect::get_handler_player(d, a.e);
    if duel::is_player_affected_by_effect(d, hp, CARD_SPIRIT_ELIMINATION).is_some() {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,PLAYER_ALL,LOCATION_MZONE)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, PLAYER_ALL as u8, LOCATION_MZONE as i32);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,PLAYER_ALL,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, PLAYER_ALL as u8, LOCATION_GRAVE as i32);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,nil)
    let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE, NO_EX);
    // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
