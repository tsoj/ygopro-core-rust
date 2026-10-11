//! Magicore Warrior of the Relics
use super::prelude::*;

pub const ID: u32 = 66078354;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_ADVENTURER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot attack unless you control an "Adventurer Token"
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    effect::set_condition(d, e1, Some(condition(not_bravecon)));
    card::register_effect(d, c, e1, false);
    // Special Summon itself from the hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(bravecon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // Set Trap that lists "Adventurer Token" from the Deck
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_SET);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e3, EVENT_PHASE | PHASE_BATTLE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_condition(d, e3, Some(condition(setcon)));
    effect::set_target(d, e3, Some(cost_target(settg)));
    effect::set_operation(d, e3, Some(operation(setop)));
    card::register_effect(d, c, e3, false);
    aux::global_check(d, ID, move |d| {
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_BATTLED);
        effect::set_operation(d, ge1, Some(operation(checkop)));
        duel::register_effect(d, ge1, 0);
    });
}
fn checkop(d: &mut Duel, a: &EffArgs) {
    let (attacker, target) = duel::get_battle_monster(d, a.tp);
    if let Some(attacker) = attacker {
        if card::lists_code(d, attacker, &[TOKEN_ADVENTURER]) {
            let p = card::get_controler(d, attacker);
            duel::register_flag_effect(d, p, ID, RESET_PHASE | PHASE_BATTLE, 0, 1, 0);
        }
    }
    if let Some(target) = target {
        if card::lists_code(d, target, &[TOKEN_ADVENTURER]) {
            let p = card::get_controler(d, target);
            duel::register_flag_effect(d, p, ID, RESET_PHASE | PHASE_BATTLE, 0, 1, 0);
        }
    }
}
fn bravecon(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[TOKEN_ADVENTURER]))), p, LOCATION_ONFIELD, 0, 1, NO_EX)
}
fn not_bravecon(d: &mut Duel, a: &EffArgs) -> bool {
    !bravecon(d, a)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, None, None, None);
    }
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn setcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_flag_effect(d, a.tp, ID) > 0
}
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_trap(d, c) && card::lists_code(d, c, &[TOKEN_ADVENTURER]) && card::is_s_setable(d, c, false, None) && !card::is_forbidden(d, c)
}
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
fn setop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if g.is_empty() {
        return;
    }
    duel::s_set(d, a.tp, g, None, true);
}
