//! Blue-Eyes Toon Dragon
use super::prelude::*;

pub const ID: u32 = 53183600;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[15259703], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Special summon
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Destroy itself
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD | EFFECT_TYPE_CONTINUOUS);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    effect::set_condition(d, e2, Some(condition(sdescon)));
    effect::set_operation(d, e2, Some(operation(sdesop)));
    card::register_effect(d, c, e2, false);
    // Direct attack
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EFFECT_DIRECT_ATTACK);
    effect::set_condition(d, e3, Some(condition(dircon)));
    card::register_effect(d, c, e3, false);
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e4, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    effect::set_condition(d, e4, Some(condition(atcon)));
    effect::set_value_fn(d, e4, effect_card_fn(atlimit));
    card::register_effect(d, c, e4, false);
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e5, EFFECT_CANNOT_DIRECT_ATTACK);
    effect::set_condition(d, e5, Some(condition(atcon)));
    card::register_effect(d, c, e5, false);
    // Cannot attack
    let e6 = effect::create_effect(d, c);
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE | EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e6, EVENT_SPSUMMON_SUCCESS);
    effect::set_property(d, e6, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_operation(d, e6, Some(operation(atklimit)));
    card::register_effect(d, c, e6, false);
    // Attack cost
    let e7 = effect::create_effect(d, c);
    effect::set_type(d, e7, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e7, EFFECT_ATTACK_COST);
    effect::set_cost(d, e7, Some(lua_fn(atcost)));
    effect::set_operation(d, e7, Some(operation(atop)));
    card::register_effect(d, c, e7, false);
}
fn spcon(d: &mut Duel, a: Args) -> bool {
    let c = match a.card(1) {
        None => return true,
        Some(c) => c,
    };
    let tp = card::get_controler(d, c);
    let ra = duel::ReleaseArgs { use_hand: Some(false), max: Some(2), check_field: true, to_check: Some(c), toplayer: Some(tp), zone: None, use_oppo: false, cancelable: false };
    duel::check_release_group(d, tp, Some(&|_, _| true), 2, ra, NO_EX, false)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[15259703])), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
fn sptg(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    let e = a.e(0);
    let _ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None);
    let ra = duel::ReleaseArgs { use_hand: Some(false), max: None, check_field: true, to_check: None, toplayer: None, zone: None, use_oppo: false, cancelable: true };
    let g = duel::select_release_group(d, tp, Some(&|_, _| true), 2, 2, ra, NO_EX, false);
    if let Some(g) = g {
        effect::set_label_object(d, e, Value::Group(g));
        return true;
    }
    false
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let g = effect::get_label_object(d, a.e);
    let Value::Group(g) = g else { return };
    duel::release(d, g, REASON_COST, None);
}
fn sfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_reason(d, c, REASON_DESTROY)
        && card::is_previous_position(d, c, POS_FACEUP)
        && card::get_previous_code_on_field(d, c).first() == Some(&15259703)
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
}
fn sdescon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &sfilter, 1, NO_EX)
}
fn sdesop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    duel::destroy(d, h, REASON_EFFECT, None, None);
}
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_TOON)
}
fn dircon(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    !duel::is_existing_matching_card(d, Some(&atkfilter), p, 0, LOCATION_MZONE, 1, NO_EX)
}
fn atcon(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&atkfilter), p, 0, LOCATION_MZONE, 1, NO_EX)
}
fn atlimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    !card::is_type(d, c, TYPE_TOON) || card::is_facedown(d, c)
}
fn atklimit(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    let h = effect::get_handler(d, a.e).nil();
    card::register_effect(d, h, e1, false);
}
fn atcost(d: &mut Duel, a: Args) -> bool {
    duel::check_lp_cost(d, a.p(2), 500)
}
fn atop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if duel::is_attack_cost_paid(d) != 2 && card::is_location(d, h, LOCATION_MZONE) {
        duel::pay_lp_cost(d, a.tp, 500);
        duel::attack_cost_paid(d, None);
    }
}
