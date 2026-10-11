//! Contract Laundering
use super::prelude::*;

pub const ID: u32 = 46259438;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DARK_CONTRACT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy as many "Dark Contract" cards in your Spell & Trap Zone as possible, and if you do, draw the same number of cards you destroyed, then gain 1000 LP for each card you drew with this effect
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DRAW + CATEGORY_RECOVER);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e1, false);
}
fn dark_contract_filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DARK_CONTRACT as u16])
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&dark_contract_filter), tp, LOCATION_STZONE, 0, 1, NO_EX)
            && duel::is_player_can_draw(d, tp, 1);
    }
    let g = duel::get_matching_group(d, Some(&dark_contract_filter), tp, LOCATION_STZONE, 0, NO_EX);
    let dark_contract_count = g.len();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), dark_contract_count as u8, tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, dark_contract_count as i32);
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, (dark_contract_count * 1000) as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&dark_contract_filter), tp, LOCATION_STZONE, 0, NO_EX);
    if g.is_empty() {
        return;
    }
    let destroy_count = duel::destroy(d, g, REASON_EFFECT, None, None);
    if destroy_count > 0 {
        let draw_count = duel::draw(d, tp, destroy_count as u32, REASON_EFFECT);
        if draw_count > 0 {
            duel::break_effect(d);
            duel::recover(d, tp, draw_count as i64 * 1000, REASON_EFFECT, false, None);
        }
    }
}
