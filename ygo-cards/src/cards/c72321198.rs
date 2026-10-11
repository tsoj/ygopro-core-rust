//! Arcana Force XII - The Hangman
use super::prelude::*;

pub const ID: u32 = 72321198;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARCANA_FORCE], toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When a card or effect is activated (Quick Effect): You can reveal this card in your hand; Special Summon 1 "Arcana Force" monster from your hand in Defense Position. You can only use this effect of "Arcana Force XII - The Hangman" once per turn
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost::self_reveal()));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // If this card is Summoned: Toss a coin
    let e2a = effect::create_effect(d, c);
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    effect::set_category(d, e2a, CATEGORY_COIN + CATEGORY_DESTROY + CATEGORY_DAMAGE);
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2a, EVENT_SUMMON_SUCCESS);
    effect::set_target(d, e2a, Some(cost_target(cointg)));
    effect::set_operation(d, e2a, Some(operation(coinop)));
    card::register_effect(d, c, e2a, false);
    let e2b = effect::clone(d, e2a);
    effect::set_code(d, e2b, EVENT_FLIP_SUMMON_SUCCESS);
    card::register_effect(d, c, e2b, false);
    let e2c = effect::clone(d, e2b);
    effect::set_code(d, e2c, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e2c, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_ARCANA_FORCE as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
fn cointg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.tp, 1);
    let g = duel::get_field_group(d, a.tp, LOCATION_MZONE, LOCATION_MZONE);
    if group::get_class_count(d, &g, &|d, c| card::get_controler(d, c) as i64) == 2 || duel::is_player_affected_by_effect(d, a.tp, CARD_LIGHT_BARRIER).is_some() {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, a.tp, 0);
    }
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, a.tp, LOCATION_MZONE as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, PLAYER_EITHER as u8, 0);
    true
}
fn coinop(d: &mut Duel, a: &EffArgs) {
    let coin: i32;
    if duel::is_player_affected_by_effect(d, a.tp, CARD_LIGHT_BARRIER).is_some() {
        let b1 = duel::is_existing_matching_card(d, None, a.tp, LOCATION_MZONE, 0, 1, NO_EX);
        let b2 = duel::is_existing_matching_card(d, None, a.tp, 0, LOCATION_MZONE, 1, NO_EX);
        let op = duel::select_effect(d, a.tp, &[(b1, aux::get_coin_effect_hint_string(COIN_HEADS as i32).nil()), (b2, aux::get_coin_effect_hint_string(COIN_TAILS as i32).nil())]);
        let Some(op) = op else { return };
        coin = if op == 1 { COIN_HEADS as i32 } else { COIN_TAILS as i32 };
    } else {
        coin = duel::toss_coin(d, a.tp, 1)[0];
    }
    if coin == COIN_HEADS as i32 {
        // Heads: Destroy 1 monster you control, and if you do, take damage equal to its original ATK
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        let sc = duel::select_matching_card(d, a.tp, None, a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        let Some(sc) = group::get_first(d, &sc) else { return };
        duel::hint_selection(d, CardSet::from_card(sc), true);
        if duel::destroy(d, sc, REASON_EFFECT, None, None) > 0 {
            duel::damage(d, a.tp, card::get_text_attack(d, sc) as i64, REASON_EFFECT, false, None);
        }
    } else if coin == COIN_TAILS as i32 {
        // Tails: Destroy 1 monster your opponent controls, and if you do, inflict damage to your opponent equal to its original ATK
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        let sc = duel::select_matching_card(d, a.tp, None, a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        let Some(sc) = group::get_first(d, &sc) else { return };
        duel::hint_selection(d, CardSet::from_card(sc), true);
        if duel::destroy(d, sc, REASON_EFFECT, None, None) > 0 {
            duel::damage(d, 1 - a.tp, card::get_text_attack(d, sc) as i64, REASON_EFFECT, false, None);
        }
    }
}
