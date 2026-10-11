//! White Mirror
use super::prelude::*;

pub const ID: u32 = 19885332;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon 1 Fish monster and search 1 monster with the same name
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_level_below(d, c, 4) && card::is_race(d, c, RACE_FISH) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.thfilter(c,tc)
fn thfilter(d: &mut Duel, c: CardId, tc: CardId) -> bool {
    let codes = card::get_original_code_rule(d, tc);
    card::is_original_code_rule(d, c, &codes) && card::is_able_to_hand(d, c, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && spfilter(d, chkc, a.e, a.tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, LOCATION_GRAVE as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let _c = effect::get_handler(d, a.e);
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e)
            && duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) != 0
            && duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, tc)), a.tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0))
        {
            duel::break_effect(d);
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
            let g = duel::select_matching_card(d, a.tp, Some(&|d, x| thfilter(d, x, tc)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            duel::confirm_cards(d, 1 - a.tp, g);
        }
    }
}
