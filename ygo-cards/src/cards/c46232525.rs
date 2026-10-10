//! The Claw of Hermos
use super::prelude::*;

pub const ID: u32 = 46232525;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // add code
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_ADD_CODE);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_value(d, e2, 10000070);
    card::register_effect(d, c, e2, false);
}
/// `s.tgfilter(c,e,tp)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    if !card::is_monster(d, c) {
        return false;
    }
    let race = card::get_race(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, race, Some(c))), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,race,mc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, race: u64, mc: Option<CardId>) -> bool {
    card::is_type(d, c, TYPE_FUSION)
        && crate::card_info(d, c).material_race.is_some()
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.map_or(NO_EX, |m| m.into()), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
        && Some(race) == crate::card_info(d, c).material_race
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        let e = a.e;
        return duel::is_existing_matching_card(d, Some(&|d, c| tgfilter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FMATERIAL as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| tgfilter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    if let Some(tc) = tc {
        if !card::is_immune_to_effect(d, tc, e) {
            if card::is_on_field(d, tc) && card::is_facedown(d, tc) {
                duel::confirm_cards(d, 1 - tp, tc);
            }
            let race = card::get_race(d, tc);
            duel::send_to_grave(d, tc, REASON_EFFECT, None, None);
            if !card::is_location(d, tc, LOCATION_GRAVE) {
                return;
            }
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            let sg = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, race, None)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
            if let Some(sc) = group::get_first(d, &sg) {
                duel::break_effect(d);
                duel::special_summon(d, sc, 0, tp, tp, true, false, POS_FACEUP as u8, None);
                card::complete_procedure(d, sc);
            }
        }
    }
}
