//! Daigusto Emeral
use super::prelude::*;

pub const ID: u32 = 581014;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    // Activate 1 of these effects
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    effect::set_target(d, e1, Some(cost_target(efftg)));
    effect::set_operation(d, e1, Some(operation(effop)));
    card::register_effect(d, c, e1, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_non_effect_monster(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        if !(card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp)) {
            return false;
        }
        let label = effect::get_label(d, e);
        if label == 1 {
            return card::is_monster(d, chkc) && card::is_able_to_deck(d, chkc);
        } else if label == 2 {
            return spfilter(d, chkc, e, tp);
        }
    }
    let b1 = duel::is_player_can_draw(d, tp, 1)
        && duel::is_existing_target(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_deck(d, c)), tp, LOCATION_GRAVE, 0, 3, NO_EX);
    let b2 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    if a.chk == Some(0) {
        return b1 || b2;
    }
    let choice = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).nil();
    effect::set_label(d, e, &[choice as i64]);
    if choice == 1 {
        effect::set_category(d, e, CATEGORY_TODECK + CATEGORY_DRAW);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        let g = duel::select_target(d, tp, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_deck(d, c)), tp, LOCATION_GRAVE, 0, 3, 3, false, NO_EX);
        duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 3, tp, 0);
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    } else if choice == 2 {
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, 0);
    }
    true
}
fn effop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let label = effect::get_label(d, e);
    if label == 1 {
        // Shuffle 3 monsters from your GY into the Deck, then draw 1 card
        let tg = duel::get_target_cards(d, e).unwrap_or_else(CardSet::new);
        if tg.len() != 3 || duel::send_to_deck(d, tg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) != 3 {
            return;
        }
        let og = duel::get_operated_group(d);
        if group::is_exists(d, &og, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX) {
            duel::shuffle_deck(d, tp);
        }
        let ct = group::filter_count(d, &og, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), NO_EX);
        if ct == 3 {
            duel::break_effect(d);
            duel::draw(d, tp, 1, REASON_EFFECT);
        }
    } else if label == 2 {
        // Special Summon 1 non-Effect Monster from your GY
        let tc = duel::first_target(d).nil();
        if card::is_relate_to_effect(d, tc, e) {
            duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
