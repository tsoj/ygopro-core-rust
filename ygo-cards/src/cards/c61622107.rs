//! Bubble Crash
use super::prelude::*;

pub const ID: u32 = 61622107;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Make players send cards to the GY until hand and field have only 5 card
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,LOCATION_ONFIELD|LOCATION_HAND,0)>=6 or Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD|LOCATION_HAND)>=6
    duel::get_field_group_count(d, a.tp, LOCATION_ONFIELD | LOCATION_HAND, 0) >= 6
        || duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD | LOCATION_HAND) >= 6
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,0,PLAYER_ALL,LOCATION_ONFIELD|LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 0, PLAYER_ALL as u8, (LOCATION_ONFIELD | LOCATION_HAND) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local ct=Duel.GetFieldGroupCount(tp,LOCATION_ONFIELD|LOCATION_HAND,0)
    let mut ct = duel::get_field_group_count(d, a.tp, LOCATION_ONFIELD | LOCATION_HAND, 0);
    // local g1=Group.CreateGroup()
    let mut g1 = CardSet::new();
    // if ct>=6 then
    if ct >= 6 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local exc=c:IsRelateToEffect(e) and c or nil
        let exc = if card::is_relate_to_effect(d, c, a.e) { c.into() } else { NO_EX };
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
        // g1=Duel.SelectMatchingCard(tp,nil,tp,LOCATION_ONFIELD|LOCATION_HAND,0,ct-5,ct-5,exc)
        g1 = duel::select_matching_card(d, a.tp, None, a.tp, LOCATION_ONFIELD | LOCATION_HAND, 0, (ct - 5) as u16, (ct - 5) as u16, false, exc).nil();
    }
    // ct=Duel.GetFieldGroupCount(1-tp,LOCATION_ONFIELD|LOCATION_HAND,0)
    ct = duel::get_field_group_count(d, 1 - a.tp, LOCATION_ONFIELD | LOCATION_HAND, 0);
    // local g2=Group.CreateGroup()
    let mut g2 = CardSet::new();
    // if ct>=6 then
    if ct >= 6 {
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_TOGRAVE as u64);
        // g2=Duel.SelectMatchingCard(1-tp,nil,1-tp,LOCATION_ONFIELD|LOCATION_HAND,0,ct-5,ct-5,nil)
        g2 = duel::select_matching_card(d, 1 - a.tp, None, 1 - a.tp, LOCATION_ONFIELD | LOCATION_HAND, 0, (ct - 5) as u16, (ct - 5) as u16, false, NO_EX).nil();
    }
    // Duel.SendtoGrave(g1,REASON_RULE,PLAYER_NONE,1-tp)
    duel::send_to_grave(d, g1, REASON_RULE, Some(PLAYER_NONE as u8), Some(1 - a.tp));
    // Duel.SendtoGrave(g2,REASON_RULE,PLAYER_NONE,1-tp)
    duel::send_to_grave(d, g2, REASON_RULE, Some(PLAYER_NONE as u8), Some(1 - a.tp));
}
