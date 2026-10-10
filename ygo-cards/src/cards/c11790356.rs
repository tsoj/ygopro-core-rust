//! Buster Dragon
use super::prelude::*;

pub const ID: u32 = 11790356;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_BUSTER_BLADER], listed_series: &[SET_BUSTER_BLADER, SET_DESTRUCTION_SWORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1+ non-Tuner monsters
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // All monsters your opponent controls become Dragons
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CHANGE_RACE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    effect::set_value(d, e1, RACE_DRAGON as i64);
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Buster Blader" from the GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // Equip 1 "Destruction Sword" monster from your GY to 1 "Buster Blader" monster you control
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_category(d, e3, CATEGORY_EQUIP);
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_hint_timing(d, e3, 0, Some(TIMINGS_CHECK_MONSTER_E));
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(|d, a| duel::is_turn_player(d, 1 - a.tp))));
    effect::set_target(d, e3, Some(cost_target(target)));
    effect::set_operation(d, e3, Some(operation(operation_fn)));
    card::register_effect(d, c, e3, false);
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    !duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_BUSTER_BLADER as u16])), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[CARD_BUSTER_BLADER]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_BUSTER_BLADER as u16]) && card::is_monster(d, c) && card::is_faceup(d, c)
}
fn filter2(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_DESTRUCTION_SWORD as u16]) && card::is_monster(d, c) && !card::is_forbidden(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && cfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&cfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&filter2), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    duel::select_target(d, tp, Some(&cfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
fn eqlimit(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let c = a.c(1);
    effect::get_label_object(d, e).as_card() == Some(c)
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let tc = duel::first_target(d);
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    let sc = duel::select_matching_card(d, tp, Some(&filter2), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if let Some(sc) = group::get_first(d, &sc) {
        if !duel::equip(d, tp, sc, tc.nil(), true, false) {
            return;
        }
        let handler = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, handler);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        effect::set_value_fn(d, e1, lua_fn(eqlimit));
        effect::set_label_object(d, e1, tc.map_or(Value::Nil, Value::Card));
        card::register_effect(d, sc, e1, false);
    }
}
