//! Garbage Collector
use super::prelude::*;

pub const ID: u32 = 11801343;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // bounce and summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    card::register_effect(d, c, e1, false);
}

fn thfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_faceup(d, c)
        && card::is_race(d, c, RACE_CYBERSE as u64)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
        && card::is_able_to_hand_as_cost(d, c)
        && card::is_monster(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, c2| spfilter(d, c2, e, tp, c)), tp, LOCATION_DECK, 0, 1, NO_EX)
}

fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) -> bool {
    if !card::is_race(d, c, RACE_CYBERSE as u64) {
        return false;
    }
    let lv = card::get_level(d, tc);
    if !card::is_level(d, c, &[lv]) {
        return false;
    }
    let codes = card::get_code(d, tc);
    !card::is_code(d, c, &codes) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}

fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp)
            && card::is_location(d, chkc, LOCATION_MZONE)
            && thfilter(d, chkc, e, tp)
            && chkc != c;
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c2| thfilter(d, c2, e, tp)), tp, LOCATION_MZONE, 0, 1, c.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    let g = duel::select_target(d, tp, Some(&|d, c2| thfilter(d, c2, e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, c.into());
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}

fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let tc = duel::first_target(d).nil();
    if duel::send_to_hand(d, tc, None, REASON_EFFECT, None) != 0
        && card::is_location(d, tc, LOCATION_HAND)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, tc)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX);
        if let Some(g) = g {
            if !g.is_empty() {
                duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
