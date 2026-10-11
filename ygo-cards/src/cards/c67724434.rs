//! Double Wild
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 67724434;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 Level 10 monster from your Deck to your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_HANDES + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    let race = card::get_original_race(d, c);
    card::is_level_below(d, c, 5)
        && card::is_faceup(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, race)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
fn thfilter(d: &mut Duel, c: CardId, race: u64) -> bool {
    card::is_level(d, c, &[10]) && card::is_original_race(d, c, race) && card::is_able_to_hand(d, c, None)
}
fn gyfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, race: u64) -> bool {
    card::is_discardable(d, c, Some(REASON_EFFECT), None)
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, race)), tp, LOCATION_HAND, 0, 1, c.into())
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, race: u64) -> bool {
    card::is_race(d, c, race) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::get_controler(d, chkc) == a.tp && card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc, a.tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    duel::select_target(d, a.tp, Some(&|d, c| tgfilter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_HANDES, None, 1, a.tp, 1);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tc = duel::first_target(d).nil();
    if !(card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc)) {
        return;
    }
    let race = card::get_original_race(d, tc);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let hg = duel::select_matching_card(d, tp, Some(&|d, c| thfilter(d, c, race)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !hg.is_empty() && duel::send_to_hand(d, hg.clone(), None, REASON_EFFECT, None) > 0 {
        duel::confirm_cards(d, 1 - tp, hg);
        duel::shuffle_hand(d, tp);
        let dg = duel::get_matching_group(d, Some(&|d, c| gyfilter(d, c, a.e, tp, race)), tp, LOCATION_HAND, 0, NO_EX);
        if !dg.is_empty() && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
            duel::break_effect(d);
            if duel::discard_hand(d, tp, Some(&|d, c| gyfilter(d, c, a.e, tp, race)), 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX) > 0 {
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
                let sg = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp, race)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
                if !sg.is_empty() {
                    duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
                }
            }
        }
    }
    if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // You cannot Special Summon for the rest of this turn after this card resolves, except monsters with the same original Type as the targeted monster
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_description(d, e1, aux::stringid(ID, 2));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(effect_card_fn(move |d, _e, c| !card::is_original_race(d, c, race))));
    effect::set_reset(d, e1, RESET_PHASE + PHASE_END, 1);
    duel::register_effect(d, e1, tp);
}
