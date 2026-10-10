//! Network Trap Hole
use super::prelude::*;

pub const ID: u32 = 84640866;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return not c:IsSummonPlayer(tp) and (c:IsSummonLocation(LOCATION_DECK) or c:IsSummonLocation(LOCATION_GRAVE))
    //     and c:IsAbleToRemove(tp,POS_FACEDOWN) and c:IsLocation(LOCATION_MZONE)
    !card::is_summon_player(d, c, tp)
        && (card::is_summon_location(d, c, LOCATION_DECK) || card::is_summon_location(d, c, LOCATION_GRAVE))
        && card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None)
        && card::is_location(d, c, LOCATION_MZONE)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=eg:Filter(s.filter,nil,tp)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| filter(d, c, a.tp), NO_EX);
    // local ct=#g
    let ct = g.len();
    // if chk==0 then return ct>0 end
    if a.chk == Some(0) {
        return ct > 0;
    }
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, &eg);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,ct,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), ct as u8, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=eg:Filter(s.filter,nil,tp):Filter(Card.IsRelateToEffect,nil,e)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| filter(d, c, a.tp), NO_EX);
    let g = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Remove(g,POS_FACEDOWN,REASON_EFFECT)
        duel::remove(d, g, POS_FACEDOWN as u8, REASON_EFFECT, None, None);
    }
}
