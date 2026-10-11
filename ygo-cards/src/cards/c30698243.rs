//! Red Hypernova Dragon
use super::prelude::*;

pub const ID: u32 = 30698243;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[97489701], synchro_nt_required: Some(1), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 4 Tuners + 1+ non-Tuner Synchro Monsters
    synchro::add_procedure(
        d,
        c,
        None,
        4,
        4,
        Some(synchro::non_tuner_ex(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_SYNCHRO, sc, st, Some(tp)))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // Synchro Summon uses multiple Tuners
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e0, EFFECT_MULTIPLE_TUNERS);
    card::register_effect(d, c, e0, false);
    // Must first be Synchro Summoned with the above materials
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    effect::set_value_fn(d, e1, lua_fn(splimit));
    card::register_effect(d, c, e1, false);
    // Gains 500 ATK for each Tuner in your GY
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    card::register_effect(d, c, e2, false);
    // Your opponent cannot target this card with card effects
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_code(d, e3, EFFECT_CANNOT_BE_EFFECT_TARGET);
    effect::set_value_fn(d, e3, aux::tgoval());
    card::register_effect(d, c, e3, false);
    // Cannot be destroyed by your opponent's card effects
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_value_fn(d, e4, aux::indoval());
    card::register_effect(d, c, e4, false);
    // Once per turn (Quick Effect): You can activate this effect; banish this card, also banish as many cards from your opponent's field and GY as possible, then you can Special Summon 1 "Red Nova Dragon" from your Extra Deck (this is treated as a Synchro Summon)
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 0));
    effect::set_category(d, e5, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e5, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e5, EVENT_FREE_CHAIN);
    effect::set_range(d, e5, LOCATION_MZONE);
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    effect::set_hint_timing(d, e5, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_target(d, e5, Some(cost_target(rmtg)));
    effect::set_operation(d, e5, Some(operation(rmop)));
    card::register_effect(d, c, e5, false);
}
fn splimit(d: &mut Duel, a: Args) -> bool {
    let c = effect::get_handler(d, a.e(0)).nil();
    if !card::is_location(d, c, LOCATION_EXTRA) {
        return true;
    }
    let st = a.int(3);
    (st & SUMMON_TYPE_SYNCHRO as i64) == SUMMON_TYPE_SYNCHRO as i64 && a.v(1).as_effect().is_none()
}
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    let p = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&|d, x| card::is_type(d, x, TYPE_TUNER)), p, LOCATION_GRAVE, 0, NO_EX) as i64 * 500
}
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, c, None, None, None);
    }
    let mut g = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), a.tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, NO_EX);
    group::add_card(&mut g, &CardSet::from_card(c));
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, a.tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[97489701])
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_SYNCHRO, tp, false, false, None, None, None)
}
fn rmop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let mut g = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), a.tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, NO_EX);
    if card::is_relate_to_effect(d, c, a.e) && card::is_able_to_remove(d, c, None, None, None) {
        group::add_card(&mut g, &CardSet::from_card(c));
    }
    if g.is_empty()
        || duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_EFFECT, None, None) == 0
        || !group::is_exists(d, &g, &|d, x| card::is_location(d, x, LOCATION_REMOVED), 1, NO_EX)
    {
        return;
    }
    let sg = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, NO_EX);
    if sg.is_empty() || !duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let sel = group::select(d, &sg, a.tp, 1, 1, false, NO_EX).nil();
    let Some(sc) = group::get_first(d, &sel) else {
        return;
    };
    card::set_material(d, sc, CardSet::new());
    duel::break_effect(d);
    if duel::special_summon(d, sc, SUMMON_TYPE_SYNCHRO, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        card::complete_procedure(d, sc);
    }
}
