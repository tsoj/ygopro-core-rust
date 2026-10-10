//! Number 39: Utopia Double
//! Logical Nonsense
use super::prelude::*;

pub const ID: u32 = 62517849;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_UTOPIA],
    listed_names: &[94770493],
    xyz_number: Some(39),
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz Summon
    // Xyz.AddProcedure(c,nil,4,2)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Add 1 "Double or Nothing!" from deck, then special summon 1 "Utopia" Xyz monster from extra deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_END_PHASE));
    card::register_effect(d, c, e1, false);
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[94770493]) && card::is_able_to_hand(d, c, None)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId) -> bool {
    card::is_type(d, c, TYPE_XYZ)
        && card::is_set_card(d, c, &[SET_UTOPIA as u16])
        && !card::is_code(d, c, &[ID])
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(0xff))
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        let pg = aux::get_must_be_material_group_opt(d, a.tp, Some(&CardSet::from_card(c)), a.tp, None, None, REASON_XYZ);
        let pg_ok = pg.is_empty() || (pg.len() == 1 && pg.contains(c));
        return pg_ok
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, a.tp, c)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// Add 1 "Double or Nothing!", then Xyz summon 1 "Utopia" Xyz monster by using this card, and if you do, double its ATK
fn op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if g.is_empty() {
        return;
    }
    duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
    duel::confirm_cards(d, 1 - a.tp, g);
    duel::shuffle_hand(d, a.tp);
    duel::break_effect(d);
    let _pg = aux::get_must_be_material_group_opt(d, a.tp, Some(&CardSet::from_card(c)), a.tp, None, None, REASON_XYZ);
    if !(card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) && card::is_controler(d, c, a.tp) && !card::is_immune_to_effect(d, c, a.e)) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let sg = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp, c)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let Some(sc) = group::get_first(d, &sg) else {
        return;
    };
    duel::break_effect(d);
    card::set_material(d, sc, CardSet::from_card(c));
    duel::overlay(d, sc, c, false);
    if duel::special_summon(d, sc, SUMMON_TYPE_XYZ, a.tp, a.tp, false, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    card::complete_procedure(d, sc);
    // Cannot attack directly
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, 3207);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, sc, e1, false);
    // Double its ATK
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_SET_ATTACK);
    effect::set_value(d, e2, card::get_text_attack(d, sc) as i64 * 2);
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, sc, e2, false);
}
