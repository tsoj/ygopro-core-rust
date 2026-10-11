//! Ancient Warriors Saga - Defense of Changban
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 4810585;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ANCIENT_WARRIORS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e0, 0, Some(TIMING_BATTLE_START));
    card::register_effect(d, c, e0, false);
    // If your "Ancient Warriors" monster battles, your opponent cannot activate Spells/Traps until the end of the Damage Step
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_target_range(d, e1, 0, 1);
    effect::set_condition(d, e1, Some(lua_fn(cannotactcon)));
    effect::set_value_fn(d, e1, lua_fn(e1_value));
    card::register_effect(d, c, e1, false);
    // Make monsters your opponent controls unable to target "Ancient Warriors" monsters for attacks this turn
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_hint_timing(d, e2, 0, Some(TIMING_BATTLE_START));
    effect::set_condition(d, e2, Some(lua_fn(e2_condition)));
    effect::set_cost(d, e2, Some(cost_target(cannotatkcost)));
    effect::set_operation(d, e2, Some(operation(cannotatkop)));
    card::register_effect(d, c, e2, false);
    // Special Summon 1 "Ancient Warriors" monster from your Deck
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e3, LOCATION_GRAVE);
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_condition(d, e3, Some(lua_fn(e3_condition)));
    effect::set_cost(d, e3, Some(cost::self_banish()));
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
}
fn cannotactcon(d: &mut Duel, a: Args<'_>) -> bool {
    let tp = effect::get_handler_player(d, a.e(0));
    match duel::get_battle_monster(d, tp).0 {
        Some(bc) => card::is_set_card(d, bc, &[SET_ANCIENT_WARRIORS as u16]) && card::is_faceup(d, bc),
        None => false,
    }
}
fn e1_value(d: &mut Duel, a: Args<'_>) -> bool {
    effect::is_has_type(d, a.e(1), EFFECT_TYPE_ACTIVATE)
}
fn e2_condition(d: &mut Duel, a: Args<'_>) -> bool {
    let tp = a.p(1);
    duel::is_turn_player(d, 1 - tp) && duel::is_phase(d, PHASE_BATTLE_START) && !duel::has_flag_effect(d, tp, ID, None)
}
fn cannotatkcost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, c) && card::is_status(d, c, STATUS_EFFECT_ENABLED);
    }
    duel::send_to_grave(d, c, REASON_COST, None, None);
    true
}
fn cannotatkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::has_flag_effect(d, tp, ID, None) {
        return;
    }
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    let c = effect::get_handler(d, a.e).nil();
    // Monsters your opponent controls cannot target "Ancient Warriors" monsters for attacks this turn
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    effect::set_value_fn(d, e1, effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_ANCIENT_WARRIORS as u16]) && card::is_faceup(d, c)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
    aux::register_client_hint(d, c, 0, tp, 0, 1, Some(aux::stringid(ID, 2)), 0, None);
}
fn e3_condition(d: &mut Duel, a: Args<'_>) -> bool {
    let tp = a.p(1);
    let at = duel::get_attacker(d).nil();
    card::is_controler(d, at, 1 - tp)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_ANCIENT_WARRIORS as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
