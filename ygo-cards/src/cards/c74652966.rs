//! Code of Soul
use super::prelude::*;

pub const ID: u32 = 74652966;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SALAMANGREAT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(lua_fn(|d, a: Args| {
        duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_link_monster(d, c)), a.p(1), LOCATION_MZONE, 0, 1, NO_EX)
    })));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Apply Reincarnation Link Summon effect
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_operation(d, e2, Some(operation(effop)));
    card::register_effect(d, c, e2, false);
    // Link Summon 1 Link-3 or higher Cyberse Link Monster
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    effect::set_range(d, e3, LOCATION_GRAVE);
    effect::set_hint_timing(d, e3, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    effect::set_condition(d, e3, Some(lua_fn(|d, a: Args| {
        duel::is_turn_player(d, 1 - a.p(1)) && duel::is_main_phase(d, None)
    })));
    effect::set_cost(d, e3, Some(cost::self_banish()));
    effect::set_target(d, e3, Some(cost_target(linktg)));
    effect::set_operation(d, e3, Some(operation(linkop)));
    card::register_effect(d, c, e3, false);
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if card::is_relate_to_effect(d, c, e) {
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
fn effop(d: &mut Duel, a: &EffArgs) {
    let (c, tp) = (effect::get_handler(d, a.e).nil(), a.tp);
    // Reincarnation Link Summon effect
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 3));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_range(d, e1, LOCATION_EXTRA);
    effect::set_condition(d, e1, Some(lua_fn(reinclinkcon)));
    effect::set_target(d, e1, Some(lua_fn(reinclinktg)));
    effect::set_operation(d, e1, Some(lua_fn(reinclinkop)));
    effect::set_value(d, e1, SUMMON_TYPE_LINK as i64);
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_GRANT);
    effect::set_target_range(d, e2, LOCATION_EXTRA, 0);
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| {
        card::is_set_card(d, c, &[SET_SALAMANGREAT as u16]) && card::is_link_monster(d, c)
    })));
    effect::set_label_object(d, e2, Value::Effect(e1));
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e2, tp);
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 4)), 0, None);
}
fn reincmatfilter(d: &mut Duel, c: CardId, lc: CardId, tp: u8) -> bool {
    let lc_codes = card::get_code(d, lc);
    card::is_faceup(d, c)
        && card::is_link_monster(d, c)
        && card::is_summon_code(d, c, Some(lc), SUMMON_TYPE_LINK as u64, tp, &lc_codes)
        && card::is_can_be_link_material(d, c, Some(lc), Some(tp))
        && duel::get_location_count_from_ex(d, tp, Some(tp), c.into(), duel::ExtraCard::Card(lc), None).0 > 0
}
fn reinclinkcon(d: &mut Duel, a: Args) -> bool {
    let Some(c) = a.card(1) else {
        return true;
    };
    let tp = card::get_controler(d, c);
    let g = duel::get_matching_group(d, Some(&|d, cc| reincmatfilter(d, cc, c, tp)), tp, LOCATION_MZONE, 0, NO_EX);
    let mut mustg = aux::get_must_be_material_group(d, tp, &g, tp, Some(c), &g, REASON_LINK as u32);
    if let Some(must) = a.v(2).as_group().cloned() {
        group::add_card(&mut mustg, &must);
    }
    ((mustg.len() == 1 && reincmatfilter(d, group::get_first(d, &mustg).nil(), c, tp)) || (mustg.is_empty() && !g.is_empty()))
        && !duel::has_flag_effect(d, tp, ID, None)
}
fn reinclinktg(d: &mut Duel, a: Args) -> bool {
    let (e, tp, c) = (a.e(0), a.p(1), a.c(9));
    let g = duel::get_matching_group(d, Some(&|d, cc| reincmatfilter(d, cc, c, tp)), tp, LOCATION_MZONE, 0, NO_EX);
    let mut mustg = aux::get_must_be_material_group(d, tp, &g, tp, Some(c), &g, REASON_LINK as u32);
    if let Some(must) = a.v(10).as_group().cloned() {
        group::add_card(&mut mustg, &must);
    }
    if !mustg.is_empty() {
        if mustg.len() > 1 {
            return false;
        }
        effect::set_label_object(d, e, Value::Group(mustg));
        return true;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_LMATERIAL as u64);
    let tc = group::select_unselect(d, &g, Some(&CardSet::new()), tp, false, true, None, None);
    if let Some(tc) = tc {
        effect::set_label_object(d, e, Value::Group(CardSet::from_card(tc)));
        true
    } else {
        false
    }
}
fn reinclinkop(d: &mut Duel, a: Args) -> Value {
    let (e, tp, c) = (a.e(0), a.p(1), a.c(8));
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    let mg = effect::get_label_object(d, e).as_group().cloned().nil();
    card::set_material(d, c, mg.clone());
    duel::send_to_grave(d, &mg, REASON_MATERIAL | REASON_LINK, None, None);
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    Value::Nil
}
fn linkfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_race(d, c, RACE_CYBERSE) && card::is_link_above(d, c, 3) && card::is_link_summonable(d, c, None, None, 0, 0)
}
fn linktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&linkfilter), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
fn linkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&linkfilter), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    if let Some(sc) = group::get_first(d, &g) {
        duel::link_summon(d, tp, sc, None, None, 0, 0);
    }
}
