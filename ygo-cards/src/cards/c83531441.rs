//! Dante, Traveler of the Burning Abyss
use super::prelude::*;

pub const ID: u32 = 83531441;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BURNING_ABYSS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 3 monsters
    xyz::add_procedure(d, c, None, Some(3), 2, None, None, None, None, false, None);
    // This card gains 500 ATK for each sent to the GY for the cost
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_cost(d, e1, Some(cost::and(vec![cost::detach_from_self(cost::Count::N(1), None, None), cost_target(atkcost)])));
    effect::set_operation(d, e1, Some(operation(atkop)));
    card::register_effect(d, c, e1, false);
    // If this card attacks, it is changed to Defense Position at the end of the Battle Phase
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_PHASE + PHASE_BATTLE);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(attacked_cond)));
    effect::set_operation(d, e2, Some(operation(attacked_op)));
    card::register_effect(d, c, e2, false);
    // Add 1 other "Burning Abyss" card from your GY to your hand
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_TOHAND);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    effect::set_target(d, e3, Some(cost_target(thtg)));
    effect::set_operation(d, e3, Some(operation(thop)));
    card::register_effect(d, c, e3, false);
}
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck_as_cost(d, a.tp, 1);
    }
    let max_ct = std::cmp::min(duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0), 3);
    let op = if max_ct == 1 { 1 } else { duel::announce_number_range(d, a.tp, Some(1), Some(max_ct as u32), &[]).nil().0 as i64 };
    effect::set_label(d, a.e, &[op]);
    duel::discard_deck(d, a.tp, op as u16, REASON_COST);
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // Until the end of this turn, this card gains 500 ATK for each card sent to the GY this way
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, effect::get_label(d, a.e) * 500);
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        card::register_effect(d, c, e1, false);
    }
}
fn attacked_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::get_attacked_count(d, c) > 0
}
fn attacked_op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    duel::change_position(d, c, POS_FACEUP_DEFENSE as u8, None, None, None, false);
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_BURNING_ABYSS as u16]) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && chkc != c && thfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, c.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, c.into());
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, a.tp, 0);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
