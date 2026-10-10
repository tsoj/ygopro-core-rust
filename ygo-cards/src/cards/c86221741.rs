//! Raidraptor - Ultimate Falcon
use super::prelude::*;

pub const ID: u32 = 86221741;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RAIDRAPTOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz Summon
    xyz::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_WINGEDBEAST, sc, st, Some(tp)))), Some(10), 3, None, None, None, None, false, None);
    card::enable_revive_limit(d, c);
    // Unaffected by other cards' effects
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value_fn(d, e1, lua_fn(efilter));
    card::register_effect(d, c, e1, false);
    // Minus 1000 ATK for opp cards, also prevent effect activation
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
    // Minus 1000 ATK for opp cards, optionally inflict damage
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(atkcon)));
    effect::set_target(d, e3, Some(cost_target(atktg)));
    effect::set_operation(d, e3, Some(operation(atkop)));
    card::register_effect(d, c, e3, false);
}
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return te:GetOwner()~=e:GetOwner()
    effect::get_owner(d, a.e(1)) != effect::get_owner(d, a.e(0))
}
fn op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tp = a.tp;
    // All monsters your opponent controls will lose 1000 ATK
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    effect::set_value(d, e1, -1000);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
    // Your opponent's cards and effects cannot be activated
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    effect::set_target_range(d, e2, 0, 1);
    effect::set_value(d, e2, 1);
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e2, tp);
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let g = card::get_overlay_group(d, c);
    group::is_exists(d, &g, &|d, cc| card::is_set_card(d, cc, &[SET_RAIDRAPTOR as u16]), 1, NO_EX)
}
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let tp = a.tp;
    if !duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX) {
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 1000);
    }
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    if g.len() > 0 {
        let h = effect::get_handler(d, a.e).nil();
        for sc in group::iter(d, &g) {
            // Lose 1000 ATK
            let e1 = effect::create_effect(d, h);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            effect::set_value(d, e1, -1000);
            card::register_effect(d, sc, e1, false);
        }
    } else {
        duel::damage(d, 1 - tp, 1000, REASON_EFFECT, false, None);
    }
}
