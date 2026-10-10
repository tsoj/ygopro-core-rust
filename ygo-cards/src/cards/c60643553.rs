//! Ghost Sister & Spooky Dogwood
use super::prelude::*;

pub const ID: u32 = 60643553;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // recover
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_RECOVER);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER as u32));
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost::self_discard()));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::get_current_phase(d) < PHASE_END
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    // e0
    let e0 = effect::create_effect(d, c);
    effect::set_description(d, e0, aux::stringid(ID, 0));
    effect::set_type(d, e0, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e0, EVENT_PHASE + PHASE_END);
    effect::set_property(d, e0, EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_count_limit(d, e0, 1, ID, 1, 0);
    effect::set_condition(d, e0, Some(condition(damcon)));
    effect::set_operation(d, e0, Some(operation(damop)));
    effect::set_reset(d, e0, RESET_PHASE | PHASE_END, 1);
    effect::set_label_object(d, e0, Value::Effect(e0));
    duel::register_effect(d, e0, tp);
    // e1
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(condition(reccon1)));
    effect::set_operation(d, e1, Some(operation(recop1)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
    // sp_summon effect
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e2, Some(condition(regcon)));
    effect::set_operation(d, e2, Some(operation(regop)));
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e2, tp);
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EVENT_CHAIN_SOLVED);
    effect::set_condition(d, e3, Some(condition(reccon2)));
    effect::set_operation(d, e3, Some(operation(recop2)));
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e3, tp);
    effect::set_label_object(d, e2, Value::Effect(e3));
}
fn filter(d: &mut Duel, c: CardId, sp: u8) -> bool {
    if card::is_location(d, c, LOCATION_MZONE) {
        card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT) && card::is_location(d, c, LOCATION_MZONE) && card::get_summon_player(d, c) == sp
    } else {
        card::is_previous_position(d, c, POS_FACEUP)
            && (card::get_previous_type_on_field(d, c) & TYPE_EFFECT) != 0
            && card::is_previous_location(d, c, LOCATION_MZONE)
            && card::get_summon_player(d, c) == sp
    }
}
fn reccon1(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let ph = duel::get_current_phase(d);
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| filter(d, c, 1 - tp), 1, NO_EX)
        && (!effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIONS) || effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_CONTINUOUS))
        && ph >= PHASE_MAIN1
        && ph <= PHASE_MAIN2
}
fn sum(d: &mut Duel, c: CardId) -> i64 {
    if card::is_location(d, c, LOCATION_MZONE) {
        card::get_attack(d, c) as i64
    } else {
        card::get_previous_attack_on_field(d, c) as i64
    }
}
fn recop1(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| filter(d, c, 1 - tp), NO_EX);
    if !g.is_empty() {
        let total = group::get_sum(d, &g, &|d, c| sum(d, c));
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        if duel::recover(d, tp, total, REASON_EFFECT, false, None) != 0 {
            duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
    }
}
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let ph = duel::get_current_phase(d);
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| filter(d, c, 1 - tp), 1, NO_EX)
        && effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIONS)
        && !effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_CONTINUOUS)
        && ph >= PHASE_MAIN1
        && ph <= PHASE_MAIN2
}
fn regop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| filter(d, c, 1 - tp), NO_EX);
    duel::register_flag_effect(d, tp, ID, RESET_CHAIN, 0, 1, 0);
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    let sum_v = group::get_sum(d, &g, &|d, c| sum(d, c));
    let lab = effect::get_label(d, lo);
    effect::set_label(d, lo, &[sum_v + lab]);
}
fn reccon2(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_flag_effect(d, a.tp, ID) > 0
}
fn recop2(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::reset_flag_effect(d, tp, ID);
    let rec = effect::get_label(d, a.e);
    effect::set_label(d, a.e, &[0]);
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    if duel::recover(d, tp, rec, REASON_EFFECT, false, None) != 0 {
        duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
    }
}
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_flag_effect(d, a.tp, ID + 1) == 0
}
fn damop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    let lp = duel::get_lp(d, tp);
    duel::set_lp(d, tp, lua_int(lp as f64 / 2.0) as i32);
}
