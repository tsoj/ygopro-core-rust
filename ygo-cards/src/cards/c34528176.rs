//! Dragunity Knight - Areus
use super::prelude::*;

pub const ID: u32 = 34528176;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRAGUNITY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1+ non-Tuner Synchro Monsters
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner_ex(|d, x, sc, st, tp| card::is_type_ctx(d, x, TYPE_SYNCHRO, sc, st, Some(tp)))), 1, 99, None, None, None, None, None);
    // Negate the effects of face-up cards your opponent controls, up to the number of face-up Monster Cards in your Spell & Trap Zone, until the end of this turn
    let e1a = effect::create_effect(d, c);
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    effect::set_category(d, e1a, CATEGORY_DISABLE);
    effect::set_type(d, e1a, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1a, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1a, LOCATION_MZONE);
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    effect::set_condition(d, e1a, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        !card::has_equip_card(d, h)
    })));
    effect::set_target(d, e1a, Some(cost_target(distg)));
    effect::set_operation(d, e1a, Some(operation(disop)));
    card::register_effect(d, c, e1a, false);
    // This is a Quick Effect if this card is equipped with an Equip Card
    let e1b = effect::clone(d, e1a);
    effect::set_type(d, e1b, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1b, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1b, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::has_equip_card(d, h)
    })));
    effect::set_hint_timing(d, e1b, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    card::register_effect(d, c, e1b, false);
    // Special Summon this card, then you can Special Summon 1 "Dragunity" Tuner from your GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::get_equip_target(d, h).is_some()
    })));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - tp) && card::is_negatable(d, chkc);
    }
    let ct = duel::get_matching_group_count(d, Some(&aux::faceup_filter(|d, c| card::is_monster_card(d, c))), tp, LOCATION_STZONE, 0, NO_EX);
    if a.chk == Some(0) {
        return ct > 0 && duel::is_existing_target(d, Some(&card::is_negatable), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
    let g = duel::select_target(d, tp, Some(&card::is_negatable), tp, 0, LOCATION_ONFIELD, 1, ct as u16, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, tp, 0);
    true
}
fn disop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tg = duel::get_target_cards(d, a.e).nil();
    for tc in group::iter(d, &tg) {
        // Negate their effects until the end of this turn
        card::negate_effects(d, tc, c, Some(RESETS_STANDARD_PHASE_END), true, None);
    }
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_DRAGUNITY as u16]) && card::is_type(d, c, TYPE_TUNER) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if card::is_relate_to_effect(d, c, e)
        && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, e, tp))), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, e, tp))), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        if g.len() > 0 {
            duel::break_effect(d);
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
