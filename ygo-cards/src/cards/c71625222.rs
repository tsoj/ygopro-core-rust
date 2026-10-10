//! Time Wizard
use super::prelude::*;

pub const ID: u32 = 71625222;

pub const INFO: ScriptInfo = ScriptInfo { toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy monsters on the field
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_COIN + CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_target(d, e1, Some(cost_target(destg)));
    effect::set_operation(d, e1, Some(operation(desop)));
    card::register_effect(d, c, e1, false);
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let g = duel::get_field_group(d, a.tp, LOCATION_MZONE, LOCATION_MZONE);
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.tp, 1);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 1, a.tp, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if duel::call_coin(d, a.tp) {
        let g = duel::get_field_group(d, a.tp, 0, LOCATION_MZONE);
        duel::destroy(d, g, REASON_EFFECT, None, None);
        duel::raise_event(d, c, EVENT_CUSTOM + ID, Some(a.e), 0, 0, a.tp, 0);
    } else {
        let g = duel::get_field_group(d, a.tp, LOCATION_MZONE, 0);
        duel::destroy(d, g, REASON_EFFECT, None, None);
        let og = duel::get_operated_group(d);
        let dg = group::filter(d, &og, &|d, c| card::is_previous_position(d, c, POS_FACEUP), NO_EX);
        let sum = group::get_sum(d, &dg, &|d, c| card::get_previous_attack_on_field(d, c) as i64);
        if sum > 0 {
            duel::damage(d, a.tp, lua_int(sum as f64 / 2.0), REASON_EFFECT, false, None);
        }
    }
}
