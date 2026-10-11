//! Madolche Anjelly
use super::prelude::*;

pub const ID: u32 = 34680482;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MADOLCHE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When sent to GY, shuffle itself into deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TODECK);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    effect::set_condition(d, e1, Some(condition(retcon)));
    effect::set_target(d, e1, Some(cost_target(rettg)));
    effect::set_operation(d, e1, Some(operation(retop)));
    card::register_effect(d, c, e1, false);
    // Special summon 1 "Madolche" monster from deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_reason(d, h, REASON_DESTROY) && card::get_reason_player(d, h) != a.tp && card::is_previous_controler(d, h, a.tp)
}
fn rettg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let h = effect::get_handler(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_TODECK, h.map(CardSet::from_card), 1, 0, 0);
    true
}
fn retop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        duel::send_to_deck(d, h, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_MADOLCHE as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    let h = effect::get_handler(d, e).nil();
    if card::get_sequence(d, h) < 5 {
        ft += 1;
    }
    if a.chk == Some(0) {
        return ft > 0 && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    let c = effect::get_handler(d, e);
    if let Some(tc) = tc {
        if duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
            let turn = duel::get_turn_count(d, None) as i64;
            card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, turn, 0);
            // Cannot be destroyed by battle
            let e1 = effect::create_effect(d, c.nil());
            effect::set_description(d, e1, 3000);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
            effect::set_value(d, e1, 1);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e1, false);
            // Shuffle it into deck during end phase of next turn
            let turn2 = duel::get_turn_count(d, None) as i64;
            let e2 = effect::create_effect(d, c.nil());
            effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
            effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
            effect::set_label(d, e2, &[turn2]);
            effect::set_label_object(d, e2, Value::Card(tc));
            effect::set_condition(d, e2, Some(condition(tdcon)));
            effect::set_operation(d, e2, Some(operation(tdop)));
            effect::set_reset(d, e2, RESET_PHASE | PHASE_END | RESET_SELF_TURN, 2);
            effect::set_count_limit(d, e2, 1, 0, 0, 0);
            duel::register_effect(d, e2, tp);
        }
    }
}
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    let label = effect::get_label(d, a.e);
    let tc = effect::get_label_object(d, a.e).as_card();
    duel::get_turn_count(d, None) as i64 != label
        && duel::is_turn_player(d, a.tp)
        && card::get_flag_effect_label(d, tc.nil(), ID).first().copied() == Some(label)
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
}
