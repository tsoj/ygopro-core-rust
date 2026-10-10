//! Ojama Pink
use super::prelude::*;

pub const ID: u32 = 42517468;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_OJAMA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Draw and discard
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_HANDES);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_ONFIELD) || card::is_previous_location(d, c, LOCATION_HAND)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1) && duel::is_player_can_draw(d, 1 - a.tp, 1);
    }
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, PLAYER_ALL as u8, 1);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, PLAYER_ALL as u8, 1);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let _c = effect::get_handler(d, a.e);
    let h1 = duel::draw(d, tp, 1, REASON_EFFECT);
    let h2 = duel::draw(d, 1 - tp, 1, REASON_EFFECT);
    if h1 > 0 || h2 > 0 {
        duel::break_effect(d);
    }
    let mut sealbool = false;
    if h1 > 0 {
        duel::shuffle_hand(d, tp);
        if duel::discard_hand(d, tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX) > 0 {
            let dc = group::get_first(d, &duel::get_operated_group(d)).nil();
            if card::is_set_card(d, dc, &[SET_OJAMA as u16]) {
                sealbool = true;
            }
        }
    }
    if h2 > 0 {
        duel::shuffle_hand(d, 1 - tp);
        duel::discard_hand(d, 1 - tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
    }
    if sealbool && duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        duel::break_effect(d);
        let mut zone = duel::select_disable_field(d, tp, 1, 0, LOCATION_MZONE, Some(0), false);
        if tp == 1 {
            zone = ((zone & 0xffff) << 16) | ((zone >> 16) & 0xffff);
        }
        let e1 = effect::create_effect(d, effect::get_handler(d, a.e).nil());
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        effect::set_code(d, e1, EFFECT_DISABLE_FIELD);
        effect::set_value(d, e1, zone as i64);
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 1);
        duel::register_effect(d, e1, tp);
    }
}
