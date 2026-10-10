//! Jenis, Lightsworn Mender
use super::prelude::*;

pub const ID: u32 = 83725008;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LIGHTSWORN], ..ScriptInfo::DEFAULT };

// s[0] / s[1] (keys 0 and 1: per player, a Lightsworn card was sent from the Deck by an effect)
// s.discard (key 2: the global effects are registered once per duel)
const VAR_DISCARD: u64 = 2;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // recover&damage
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DAMAGE + CATEGORY_RECOVER);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(lpcon)));
    effect::set_target(d, e1, Some(cost_target(lptg)));
    effect::set_operation(d, e1, Some(operation(lpop)));
    card::register_effect(d, c, e1, false);
    if s_var(d, ID, VAR_DISCARD).is_nil() {
        set_s_var(d, ID, VAR_DISCARD, Value::Bool(true));
        set_s_var(d, ID, 0, Value::Bool(false));
        set_s_var(d, ID, 1, Value::Bool(false));
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
        effect::set_code(d, e2, EVENT_PHASE_START | PHASE_DRAW);
        effect::set_operation(d, e2, Some(operation(reset)));
        duel::register_effect(d, e2, 0);
        let e3 = effect::create_effect(d, c);
        effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
        effect::set_code(d, e3, EVENT_TO_GRAVE);
        effect::set_operation(d, e3, Some(operation(set)));
        duel::register_effect(d, e3, 0);
    }
}
fn reset(d: &mut Duel, _a: &EffArgs) {
    set_s_var(d, ID, 0, Value::Bool(false));
    set_s_var(d, ID, 1, Value::Bool(false));
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::get_previous_location(d, c) == LOCATION_DECK
}
fn set(d: &mut Duel, a: &EffArgs) {
    let re = match a.re {
        Some(re) => re,
        None => return,
    };
    let rc = effect::get_handler(d, re).nil();
    if (a.r & REASON_EFFECT) > 0 && card::is_set_card(d, rc, &[SET_LIGHTSWORN as u16]) && group::is_exists(d, &a.eg.clone().nil(), &cfilter, 1, NO_EX) {
        set_s_var(d, ID, a.rp as u64, Value::Bool(true));
    }
}
fn lpcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_turn_player(d) == a.tp && s_var(d, ID, a.tp as u64).truthy()
}
fn lptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, 500);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 500);
    true
}
fn lpop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::get_controler(d, c) != a.tp || !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    duel::recover(d, a.tp, 500, REASON_EFFECT, false, None);
    duel::damage(d, 1 - a.tp, 500, REASON_EFFECT, false, None);
}
