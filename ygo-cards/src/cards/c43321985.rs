//! Ancient Fairy Life Dragon
use super::prelude::*;

pub const ID: u32 = 43321985;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[5414777, 28903523, CARD_ANCIENT_FAIRY_DRAGON],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1+ non-Tuner monsters
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Draw 1 card, or add 1 LIGHT Beast, Plant, or Fairy monster, or 1 "Eternal Sunshine" from your Deck to your hand instead
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(lua_fn(|d, a| card::is_synchro_summoned(d, effect::get_handler(d, a.e(0)).nil()))));
    effect::set_target(d, e1, Some(cost_target(drthtg)));
    effect::set_operation(d, e1, Some(operation(drthop)));
    card::register_effect(d, c, e1, false);
    // Your "Ancient Fairy Dragon" and monsters that mention it can attack while in face-up Defense Position. If they do, apply their DEF for damage calculation
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_DEFENSE_ATTACK);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    effect::set_target(
        d,
        e2,
        Some(effect_card_fn(|d, _e, c| card::is_code(d, c, &[CARD_ANCIENT_FAIRY_DRAGON]) || card::lists_code(d, c, &[CARD_ANCIENT_FAIRY_DRAGON]))),
    );
    effect::set_value(d, e2, 1);
    card::register_effect(d, c, e2, false);
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    ((card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_BEAST | RACE_PLANT | RACE_FAIRY)) || card::is_code(d, c, &[28903523]))
        && card::is_able_to_hand(d, c, None)
}
fn drthtg(d: &mut Duel, a: &EffArgs) -> bool {
    let world_of_spirits_chk = duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[5414777])), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    let b1 = !world_of_spirits_chk && duel::is_player_can_draw(d, a.tp, 1);
    let b2 = world_of_spirits_chk && duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    if a.chk == Some(0) {
        return b1 || b2;
    }
    if b1 {
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    } else {
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    }
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn drthop(d: &mut Duel, a: &EffArgs) {
    let world_of_spirits_chk = duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[5414777])), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    if !world_of_spirits_chk {
        duel::draw(d, a.tp, 1, REASON_EFFECT);
    } else {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        if !g.is_empty() {
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            duel::confirm_cards(d, 1 - a.tp, g);
        }
    }
}
