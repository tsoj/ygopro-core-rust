//! Psychic Reactor
use super::prelude::*;

pub const ID: u32 = 3146695;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(regcon)));
    effect::set_operation(d, e1, Some(operation(regop)));
    card::register_effect(d, c, e1, false);
}
fn psychic_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_PSYCHIC)
}
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&psychic_faceup), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn regop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&psychic_faceup), tp, LOCATION_MZONE, 0, NO_EX);
    let _ = group::get_first(d, &g);
    for tc in group::iter(d, &g) {
        card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
    // g:KeepAlive()
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_BATTLED);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    effect::set_label_object(d, e1, Value::Group(g.clone()));
    duel::register_effect(d, e1, tp);
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_TURN_END);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_label_object(d, e2, Value::Group(g));
    effect::set_operation(d, e2, Some(operation(reset)));
    duel::register_effect(d, e2, tp);
}
// s.filter (c:GetFlagEffect(id)>0 and g:IsContains(c)) is only reachable through `d and ...`,
// where `d` is a nil global: the Lua never calls it, so it is left out.
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let attacker = duel::get_attacker(d);
    let attack_target = duel::get_attack_target(d);
    let mut g = CardSet::new();
    for c in [attacker, attack_target].into_iter().flatten() {
        group::add_card(&mut g, &CardSet::from_card(c));
    }
    if a.chk == Some(0) {
        // `d and g:IsExists(...)`: `d` is a global that is nil here, so the check is false.
        return false;
    }
    let rg = group::filter(d, &g, &|d, c| card::is_relate_to_battle(d, c), NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(rg.clone()), rg.len() as u8, 0, 0);
    true
}
fn op(d: &mut Duel, _a: &EffArgs) {
    let attacker = duel::get_attacker(d);
    let attack_target = duel::get_attack_target(d);
    let mut g = CardSet::new();
    for c in [attacker, attack_target].into_iter().flatten() {
        group::add_card(&mut g, &CardSet::from_card(c));
    }
    let rg = group::filter(d, &g, &|d, c| card::is_relate_to_battle(d, c), NO_EX);
    duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
fn reset(d: &mut Duel, a: &EffArgs) {
    // e:GetLabelObject():DeleteGroup() does nothing here; e:Reset()
    effect::reset(d, a.e);
}
