//! Limit Overdrive
use super::prelude::*;

pub const ID: u32 = 35014241;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cfilter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_SYNCHRO)
        && card::is_type(d, c, TYPE_TUNER)
        && card::is_able_to_extra_as_cost(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, x| cfilter2(d, x, e, tp, c)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn cfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) -> bool {
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_SYNCHRO)
        && !card::is_type(d, c, TYPE_TUNER)
        && card::is_able_to_extra_as_cost(d, c)
        && {
            let lv = card::get_level(d, c) + card::get_level(d, tc);
            let mg = two_group(c, tc);
            duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, lv, Some(&mg))), tp, LOCATION_EXTRA, 0, 1, NO_EX)
        }
}
/// `Group.FromCards(c, tc)`
fn two_group(c: CardId, tc: CardId) -> CardSet {
    let mut g = CardSet::from_card(c);
    group::add_card(&mut g, &CardSet::from_card(tc));
    g
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: u32, mg: Option<&CardSet>) -> bool {
    card::is_type(d, c, TYPE_SYNCHRO)
        && card::get_level(d, c) == lv
        && {
            let ex = match mg {
                Some(m) => m.into(),
                None => NO_EX,
            };
            duel::get_location_count_from_ex(d, tp, Some(tp), ex, duel::ExtraCard::Card(c), None).0 > 0
        }
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    effect::set_label(d, e, &[1]);
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| cfilter1(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    let mut g1 = duel::select_matching_card(d, tp, Some(&|d, c| cfilter1(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    let tc1 = group::get_first(d, &g1).nil();
    let g2 = duel::select_matching_card(d, tp, Some(&|d, c| cfilter2(d, c, e, tp, tc1)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc2 = group::get_first(d, &g2).nil();
    let lv = card::get_level(d, tc1) + card::get_level(d, tc2);
    effect::set_label(d, e, &[lv as i64]);
    group::add_card(&mut g1, &g2);
    duel::send_to_deck(d, g1, None, SEQ_DECKTOP as i32, REASON_COST, None);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        if effect::get_label(d, a.e) != 1 {
            return false;
        }
        effect::set_label(d, a.e, &[0]);
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let lv = effect::get_label(d, e);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, lv as u32, None)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, true, false, POS_FACEUP as u8, None);
    }
}
