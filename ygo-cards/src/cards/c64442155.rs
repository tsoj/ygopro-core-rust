//! Merciful Machine Angel
use super::prelude::*;

pub const ID: u32 = 64442155;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CYBER_ANGEL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_TODECK);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.costfilter`: "Cyber Angel" Ritual Monster
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_CYBER_ANGEL as u16]) && card::is_ritual_monster(d, c)
}
/// `s.cost`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&costfilter), 1, 1, true, None, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RELEASE as u64);
    let g = duel::select_release_group_cost(d, a.tp, Some(&costfilter), 1, 1, true, None, NO_EX);
    duel::release(d, g, REASON_COST, None);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 2);
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, 2);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let count = info[1].int() as u32;
    if duel::draw(d, p, count, REASON_EFFECT) == count as i32 {
        duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_TODECK as u64);
        let g = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_able_to_deck(d, c)), p, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        if !g.is_empty() {
            duel::break_effect(d);
            duel::send_to_deck(d, g, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
        }
    }
    if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    // Cannot Special Summon, except Ritual Monsters
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
    // Cannot Special Summon from the Main Deck check
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e2, CARD_EHERO_BLAZEMAN);
    effect::set_target_range(d, e2, 1, 0);
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e2, a.tp);
}
/// `s.splimit(e,c)`: restricted to Ritual Monsters for the rest of the turn
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    if card::is_monster(d, c) {
        !card::is_type(d, c, TYPE_RITUAL)
    } else {
        !card::is_original_type(d, c, TYPE_RITUAL)
    }
}
