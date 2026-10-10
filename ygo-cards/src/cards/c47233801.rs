//! Dark Snake Syndrome
use super::prelude::*;

pub const ID: u32 = 47233801;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // damage
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    effect::set_condition(d, e2, Some(condition(damcon)));
    effect::set_target(d, e2, Some(cost_target(damtg)));
    effect::set_operation(d, e2, Some(operation(damop)));
    card::register_effect(d, c, e2, false);
}
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, PLAYER_ALL as u8, 0);
    true
}
fn damop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let dam = match card::get_flag_effect_label(d, c, ID).first().copied() {
        None => {
            card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD, 0, 0, 200, 0);
            200
        }
        Some(v) => {
            let nd = v * 2;
            card::set_flag_effect_label(d, c, ID, nd as u32);
            nd
        }
    };
    duel::damage(d, a.tp, dam, REASON_EFFECT, true, None);
    duel::damage(d, 1 - a.tp, dam, REASON_EFFECT, true, None);
    duel::rd_complete(d);
}
