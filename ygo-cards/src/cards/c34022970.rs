//! Ext Ryzeal
use super::prelude::*;

pub const ID: u32 = 34022970;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RYZEAL], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from the hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(lua_fn(selfspcon)));
    effect::set_target(d, e1, Some(cost_target(selfsptg)));
    effect::set_operation(d, e1, Some(operation(selfspop)));
    card::register_effect(d, c, e1, false);
    // Add 1 FIRE Thunder monster from your Deck to your hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e3, false);
}
fn selfspconfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPE_XYZ) && card::is_able_to_grave_as_cost(d, c)
}
fn selfspcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(_c) = a.card(1) else { return true };
    let tp = effect::get_handler_player(d, a.e(0));
    let g = duel::get_matching_group(d, Some(&selfspconfilter), tp, LOCATION_EXTRA, 0, NO_EX);
    !g.is_empty() && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
}
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let rg = duel::get_matching_group(d, Some(&selfspconfilter), tp, LOCATION_EXTRA, 0, NO_EX);
    let g = aux::select_unselect_group(d, &rg, Some(a.e), tp, Some(1), Some(1), Some(&aux::chkf_mmz(1)), 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, true).unwrap_or_default();
    if !g.is_empty() {
        // g:KeepAlive()
        effect::set_label_object(d, a.e, Value::Group(g));
        return true;
    }
    false
}
fn selfspop(d: &mut Duel, a: &EffArgs) {
    let Some(g) = effect::get_label_object(d, a.e).as_group().cloned() else { return };
    duel::send_to_grave(d, g, REASON_COST, None, None);
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
    duel::register_effect(d, e1, a.tp);
    // Clock Lizard Check
    aux::add_temp_lizard_check(d, c, a.tp, Some(effect_card_fn(|d, _e, c| !(card::is_original_type(d, c, TYPE_XYZ) && card::is_original_rank(d, c, 4)))), None, None, None, None);
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_FIRE) && card::is_race(d, c, RACE_THUNDER) && card::is_able_to_hand(d, c, None)
}
fn thconfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && !(card::is_level(d, c, &[4]) || card::is_rank(d, c, &[4]))
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX) && !duel::is_existing_matching_card(d, Some(&thconfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, g);
    }
}
