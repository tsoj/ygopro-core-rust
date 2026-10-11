//! Griffoh
use super::prelude::*;

pub const ID: u32 = 97462632;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_LIGHT_AND_DARKNESS_RITUAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // (Quick Effect): You can discard this card, then activate 1 of these effects;
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost::self_discard()));
    effect::set_target(d, e1, Some(cost_target(efftg)));
    effect::set_operation(d, e1, Some(operation(effop)));
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e1, false);
    // For the Ritual Summon of exactly 1 Level 8 Ritual Monster with a card effect that requires use of monsters, this card can be used as the entire Tribute
    ritual::add_whole_level_tribute(d, c, std::sync::Arc::new(|d, c, _e| card::is_level(d, c, &[8])));
}
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    (card::is_quick_play_spell(d, c) || card::is_trap(d, c)) && card::lists_code(d, c, &[CARD_LIGHT_AND_DARKNESS_RITUAL]) && card::is_s_setable(d, c, false, None)
}
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let b1 = true;
    let b2 = duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    if a.chk == Some(0) {
        return b1 || b2;
    }
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    effect::set_label(d, a.e, &[op.map_or(0, |x| x as i64)]);
    if op == Some(1) {
        effect::set_category(d, a.e, 0);
    } else if op == Some(2) {
        effect::set_category(d, a.e, CATEGORY_SET);
        duel::set_operation_info(d, 0, CATEGORY_SET, None, 1, a.tp, LOCATION_DECK as i32);
    }
    true
}
fn effop(d: &mut Duel, a: &EffArgs) {
    let op = effect::get_label(d, a.e);
    let c = effect::get_handler(d, a.e).nil();
    if op == 1 {
        // The next battle or effect damage you take this turn will become 0
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, aux::stringid(ID, 3));
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
        effect::set_target_range(d, e1, 1, 0);
        effect::set_value_fn(d, e1, lua_fn(damval));
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        duel::register_effect(d, e1, a.tp);
    } else if op == 2 {
        // Set 1 Quick-Play Spell or Trap that mentions "Ritual of Light and Darkness" from your Deck. It can be activated this turn
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
        let g = duel::select_matching_card(d, a.tp, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        if let Some(sc) = group::get_first(d, &g) {
            if duel::s_set(d, a.tp, sc, None, true) > 0 {
                let eff_code = if card::is_quick_play_spell(d, sc) { EFFECT_QP_ACT_IN_SET_TURN } else { EFFECT_TRAP_ACT_IN_SET_TURN };
                // It can be activated this turn
                let e2 = effect::create_effect(d, c);
                effect::set_description(d, e2, aux::stringid(ID, 4));
                effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
                effect::set_property(d, e2, EFFECT_FLAG_SET_AVAILABLE | EFFECT_FLAG_CLIENT_HINT, 0);
                effect::set_code(d, e2, eff_code);
                effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
                card::register_effect(d, sc, e2, false);
            }
        }
    }
}
fn damval(d: &mut Duel, a: Args) -> i64 {
    let r = a.int(3) as u32;
    if r & (REASON_BATTLE | REASON_EFFECT) > 0 {
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        effect::reset(d, a.e(0));
        return 0;
    }
    a.int(2)
}
