//! Synchro Call
use super::prelude::*;

pub const ID: u32 = 89974904;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId, tc: CardId) -> bool {
    card::is_race(d, c, RACE_DRAGON | RACE_FIEND)
        && card::is_attribute(d, c, ATTRIBUTE_DARK)
        && card::is_synchro_summonable(d, c, Some(CardSet::from_card(tc)), None, 0, 0)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_can_be_special_summoned(d, c, e, 182, tp, false, false, None, None, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| cfilter(d, x, c)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        return duel::is_player_can_special_summon_count(d, tp, 2)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 2, tp, LOCATION_EXTRA as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, e) && duel::special_summon_step(d, tc, 182, tp, tp, false, false, POS_FACEUP as u8, None) {
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_DISABLE);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e1, false);
            let e2 = effect::clone(d, e1);
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            card::register_effect(d, tc, e2, false);
            duel::special_summon_complete(d);
            let g = duel::get_matching_group(d, Some(&|d, x| cfilter(d, x, tc)), tp, LOCATION_EXTRA, 0, NO_EX);
            if !g.is_empty() {
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
                let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
                let sc = group::get_first(d, &sg).nil();
                duel::synchro_summon(d, tp, sc, Some(CardSet::from_card(tc)), None, 0, 0);
            }
        }
    }
}
