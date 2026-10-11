//! Gagagatag
use super::prelude::*;

pub const ID: u32 = 917796;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GAGAGA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn gagaga_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GAGAGA as u16])
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&gagaga_faceup), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let sg = duel::get_matching_group(d, Some(&gagaga_faceup), a.tp, LOCATION_MZONE, 0, NO_EX);
    let atk = sg.len() as i64 * 500;
    let c = effect::get_handler(d, a.e).nil();
    let _tc = group::get_first(d, &sg);
    for tc in group::iter(d, &sg) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, atk);
        effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD + RESET_PHASE + PHASE_STANDBY, 2);
        card::register_effect(d, tc, e1, false);
    }
}
