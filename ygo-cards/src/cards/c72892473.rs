//! Card Destruction
use super::prelude::*;

pub const ID: u32 = 72892473;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_HANDES + CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // local h1=Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)
        let mut h1 = duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0);
        // if e:GetHandler():IsLocation(LOCATION_HAND) then h1=h1-1 end
        let c = effect::get_handler(d, a.e).nil();
        if card::is_location(d, c, LOCATION_HAND) {
            h1 -= 1;
        }
        // local h2=Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)
        let h2 = duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND);
        // return (h1+h2>0) and (Duel.IsPlayerCanDraw(tp,h1) or h1==0) and (Duel.IsPlayerCanDraw(1-tp) or h2==0)
        return h1 + h2 > 0
            && (duel::is_player_can_draw(d, a.tp, h1 as u32) || h1 == 0)
            && (duel::is_player_can_draw(d, 1 - a.tp, 0) || h2 == 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,PLAYER_ALL,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, PLAYER_ALL as u8, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,PLAYER_ALL,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, PLAYER_ALL as u8, 1);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local h1=Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)
    let h1 = duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0);
    // local h2=Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)
    let h2 = duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND);
    // local g=Duel.GetFieldGroup(tp,LOCATION_HAND,LOCATION_HAND)
    let g = duel::get_field_group(d, a.tp, LOCATION_HAND, LOCATION_HAND);
    // Duel.SendtoGrave(g,REASON_EFFECT|REASON_DISCARD)
    duel::send_to_grave(d, g, REASON_EFFECT | REASON_DISCARD, None, None);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.Draw(tp,h1,REASON_EFFECT)
    duel::draw(d, a.tp, h1 as u32, REASON_EFFECT);
    // Duel.Draw(1-tp,h2,REASON_EFFECT)
    duel::draw(d, 1 - a.tp, h2 as u32, REASON_EFFECT);
}
