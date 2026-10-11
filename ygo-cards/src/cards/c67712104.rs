//! Marincess Crystal Heart
use super::prelude::*;

pub const ID: u32 = 67712104;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MARINCESS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_WATER, sc, st, Some(tp)))), 2, Some(2), None, None, None);
    card::enable_revive_limit(d, c);
    // immune
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(lua_fn(econ)));
    effect::set_value_fn(d, e1, lua_fn(efilter));
    card::register_effect(d, c, e1, false);
    // other immune
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(lua_fn(imcon)));
    effect::set_target(d, e2, Some(effect_card_fn(imtg)));
    effect::set_value_fn(d, e2, lua_fn(imval));
    card::register_effect(d, c, e2, false);
    // protect battle
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e3, EVENT_BE_BATTLE_TARGET);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_condition(d, e3, Some(condition(indescond)));
    effect::set_cost(d, e3, Some(cost_target(indescost)));
    effect::set_target(d, e3, Some(cost_target(indestg)));
    effect::set_operation(d, e3, Some(operation(indesop)));
    card::register_effect(d, c, e3, false);
}
fn econ(d: &mut Duel, a: Args) -> bool {
    let c = effect::get_handler(d, a.e(0)).nil();
    card::get_sequence(d, c) > 4
}
fn efilter(d: &mut Duel, a: Args) -> bool {
    let (e, te) = (a.e(0), a.e(1));
    effect::is_monster_effect(d, te) && effect::get_owner_player(d, te) != effect::get_handler_player(d, e)
}
fn imcon(d: &mut Duel, _a: Args) -> bool {
    let ph = duel::get_current_phase(d);
    ph == PHASE_DAMAGE || ph == PHASE_DAMAGE_CAL
}
fn imtg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let h = effect::get_handler(d, e).nil();
    card::get_battle_target(d, h) == Some(c)
}
fn imval(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::get_battle_target(d, h) != effect::get_owner(d, a.e(1))
}
fn indescond(d: &mut Duel, a: &EffArgs) -> bool {
    let Some(at) = duel::get_attack_target(d) else { return false };
    let c = effect::get_handler(d, a.e).nil();
    if at == c {
        return true;
    }
    let lg = card::get_linked_group(d, c);
    card::is_controler(d, at, a.tp) && card::is_faceup(d, at) && card::is_set_card(d, at, &[SET_MARINCESS as u16]) && group::is_contains(&lg, at)
}
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_MARINCESS as u16]) && card::is_monster(d, c) && card::is_able_to_grave_as_cost(d, c)
}
fn indescost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::discard_hand(d, a.tp, Some(&costfilter), 1, 1, REASON_COST, NO_EX);
    true
}
fn indestg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let at = duel::get_attack_target(d).nil();
    card::create_effect_relation(d, at, a.e);
    true
}
fn indesop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let at = duel::get_attack_target(d).nil();
    if card::is_relate_to_effect(d, at, a.e) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
        effect::set_value(d, e1, 1);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
        card::register_effect(d, at, e1, false);
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_FIELD);
        effect::set_code(d, e2, EFFECT_AVOID_BATTLE_DAMAGE);
        effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
        effect::set_target_range(d, e2, 1, 0);
        effect::set_reset(d, e2, RESET_PHASE | PHASE_DAMAGE, 1);
        duel::register_effect(d, e2, a.tp);
    }
}
