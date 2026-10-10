//! Mad Reloader
use super::prelude::*;

pub const ID: u32 = 31034919;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Draw
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLocation(LOCATION_GRAVE) and e:GetHandler():IsReason(REASON_BATTLE)
    let h = effect::get_handler(d, a.e).nil();
    card::is_location(d, h, LOCATION_GRAVE) && card::is_reason(d, h, REASON_BATTLE)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,2,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 2, a.tp, LOCATION_HAND as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp,chk)`
fn op(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)<2 then return end
    if duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) < 2 {
        return;
    }
    // if Duel.DiscardHand(tp,nil,2,2,REASON_EFFECT)~=0 then
    if duel::discard_hand(d, a.tp, None, 2, 2, REASON_EFFECT, NO_EX) != 0 {
        // local ct=Duel.GetOperatedGroup():FilterCount(Card.IsLocation,nil,LOCATION_GRAVE)
        let g = duel::get_operated_group(d);
        let ct = group::filter_count(d, &g, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
        // if ct==2 then
        if ct == 2 {
            // Duel.Draw(tp,2,REASON_EFFECT)
            duel::draw(d, a.tp, 2, REASON_EFFECT);
        }
    }
}
