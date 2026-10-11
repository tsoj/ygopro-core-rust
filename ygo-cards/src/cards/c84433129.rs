//! Star Ryzeal
use super::prelude::*;

pub const ID: u32 = 84433129;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RYZEAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can Special Summon this card (from your hand) by detaching 1 material from a monster you control
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(lua_fn(selfspcon)));
    effect::set_target(d, e1, Some(cost_target(selfsptg)));
    effect::set_operation(d, e1, Some(operation(selfspop)));
    card::register_effect(d, c, e1, false);
    // Set 1 "Ryzeal" Spell/Trap from your Deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SET);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(settg)));
    effect::set_operation(d, e2, Some(operation(setop)));
    card::register_effect(d, c, e2, false);
}
fn selfspcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(_c) = a.card(1) else { return true };
    let tp = effect::get_handler_player(d, a.e(0));
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && duel::check_remove_overlay_card(d, tp, 1, 0, 1, REASON_COST, None)
}
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVEXYZ as u64);
    let og = duel::get_overlay_group(d, tp, 1, 0, None);
    let g = group::select(d, &og, tp, 0, 1, false, NO_EX).nil();
    if !g.is_empty() {
        // g:KeepAlive()
        effect::set_label_object(d, a.e, Value::Group(g));
        return true;
    }
    false
}
fn selfspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let Some(g) = effect::get_label_object(d, a.e).as_group().cloned() else { return };
    let first = group::get_first(d, &g).nil();
    let xyzc = card::get_overlay_target(d, first).nil();
    duel::send_to_grave(d, g, REASON_COST, None, None);
    duel::raise_single_event(d, xyzc, EVENT_DETACH_MATERIAL, Some(a.e), 0, 0, 0, 0);
    duel::raise_event(d, CardSet::from_card(xyzc), EVENT_DETACH_MATERIAL, Some(a.e), REASON_EFFECT, tp, tp, 0);
    // the summoned card (index 8 of the operation)
    let c = a.chkc().nil();
    // You cannot Special Summon from the Extra Deck for the rest of this turn, except Rank 4 Xyz Monsters
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 2));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !(card::is_type(d, c, TYPE_XYZ) && card::is_rank(d, c, &[4])))));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
    // "Clock Lizard" check
    aux::add_temp_lizard_check(d, c, tp, Some(effect_card_fn(|d, _e, c| !(card::is_original_type(d, c, TYPE_XYZ) && card::is_original_rank(d, c, 4)))), None, None, None, None);
}
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_RYZEAL as u16]) && card::is_spell_trap(d, c) && card::is_s_setable(d, c, false, None)
}
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    let g = duel::select_matching_card(d, tp, Some(&setfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::s_set(d, tp, g, None, true);
    }
}
