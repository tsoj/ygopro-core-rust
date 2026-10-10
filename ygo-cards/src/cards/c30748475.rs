//! Destructive Daruma Karma Cannon
use super::prelude::*;

pub const ID: u32 = 30748475;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_TOGRAVE + CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return Duel.IsExistingMatchingCard(Card.IsCanTurnSet,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
        return duel::is_existing_matching_card(d, Some(&card::is_can_turn_set), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(Card.IsCanTurnSet,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&card::is_can_turn_set), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,#g,tp,POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g.clone()), g.len() as u8, a.tp, POS_FACEDOWN_DEFENSE as i32);
    // local tg=Duel.GetMatchingGroup(aux.NOT(Card.IsCanTurnSet),tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let tg = duel::get_matching_group(d, Some(&|d, c| !card::is_can_turn_set(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,tg,#tg,0,0)
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(tg.clone()), tg.len() as u8, 0, 0);
    }
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsCanTurnSet,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&card::is_can_turn_set), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g>0 and Duel.ChangePosition(g,POS_FACEDOWN_DEFENSE)>0 then
    if !g.is_empty() && duel::change_position(d, g, POS_FACEDOWN_DEFENSE as u8, None, None, None, false) > 0 {
        // local turn_p=Duel.GetTurnPlayer()
        let turn_p = duel::get_turn_player(d);
        // local g1=Duel.GetMatchingGroup(Card.IsFaceup,turn_p,LOCATION_MZONE,0,nil)
        let g1 = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), turn_p, LOCATION_MZONE, 0, NO_EX);
        // local g2=Duel.GetMatchingGroup(Card.IsFaceup,turn_p,0,LOCATION_MZONE,nil)
        let g2 = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), turn_p, 0, LOCATION_MZONE, NO_EX);
        // if #g1==0 and #g2==0 then return end
        if g1.is_empty() && g2.is_empty() {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if #g1>0 then
        if !g1.is_empty() {
            // Duel.SendtoGrave(g1,REASON_RULE,PLAYER_NONE,turn_p)
            duel::send_to_grave(d, g1, REASON_RULE, Some(PLAYER_NONE as u8), Some(turn_p));
        }
        // if #g2>0 then
        if !g2.is_empty() {
            // Duel.SendtoGrave(g2,REASON_RULE,PLAYER_NONE,1-turn_p)
            duel::send_to_grave(d, g2, REASON_RULE, Some(PLAYER_NONE as u8), Some(1 - turn_p));
        }
    }
}
