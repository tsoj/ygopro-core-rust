//! Ice Barrier
use super::prelude::*;

pub const ID: u32 = 34293667;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DISABLE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Send 1 Level 5 or higher WATER monster from your Deck to the GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOGRAVE + CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    effect::set_operation(d, e2, Some(operation(tgop)));
    card::register_effect(d, c, e2, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let (_, bc) = duel::get_battle_monster(d, tp);
    if a.chk == Some(0) {
        let Some(bc) = bc else { return false };
        return card::is_faceup(d, bc)
            && (card::is_attack_above(d, bc, 1)
                || card::is_negatable_monster(d, bc)
                || card::is_has_effect(d, bc, EFFECT_CANNOT_CHANGE_POSITION, None).is_empty());
    }
    let bc = bc.nil();
    duel::set_target_card(d, bc);
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(bc)), 1, tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(CardSet::from_card(bc)), 1, tp, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let bc = duel::first_target(d).nil();
    if !(card::is_relate_to_effect(d, bc, a.e) && card::is_faceup(d, bc) && card::is_relate_to_battle(d, bc) && card::is_controler(d, bc, 1 - a.tp)) {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    // Negate its effects
    card::negate_effects(d, bc, c, None, false, None);
    // Change its ATK to 0
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, 3313);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
    effect::set_value(d, e1, 0);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, bc, e1, false);
    // Cannot change its battle position
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_CANNOT_CHANGE_POSITION);
    card::register_effect(d, bc, e2, false);
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_level_above(d, c, 5) && card::is_attribute(d, c, ATTRIBUTE_WATER) && card::is_able_to_grave(d, c, None)
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_WATER) && card::is_able_to_hand(d, c, None)
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    let ct = if duel::is_turn_player(d, tp) { 2 } else { 1 };
    // Cannot Special Summon monsters, except WATER monsters
    let e1 = effect::create_effect(d, h);
    effect::set_description(d, e1, aux::stringid(ID, 2));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_attribute(d, c, ATTRIBUTE_WATER))));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_SELF_TURN, ct);
    duel::register_effect(d, e1, tp);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let tgc = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tgc = group::get_first(d, &tgc);
    let Some(tgc) = tgc else { return };
    if !(duel::send_to_grave(d, tgc, REASON_EFFECT, None, None) > 0 && card::is_location(d, tgc, LOCATION_GRAVE)) {
        return;
    }
    let thg = duel::get_matching_group(d, Some(&aux::necro_valley_filter(thfilter)), tp, LOCATION_GRAVE, 0, NO_EX);
    if !thg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        let sg = group::select(d, &thg, tp, 1, 1, false, NO_EX).nil();
        if !sg.is_empty() {
            duel::break_effect(d);
            duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
            duel::confirm_cards(d, 1 - tp, sg);
        }
    }
}
