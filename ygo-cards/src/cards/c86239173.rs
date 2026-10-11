//! Horned Saurus
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 86239173;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[50834074], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // "Kaitoptera" + 1 Dinosaur or Dragon monster
    fusion::add_proc_mix(d, c, true, true, vec![
        fusion::Mat::Code(50834074),
        fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_race_ctx(d, x, RACE_DINOSAUR | RACE_DRAGON, Some(fc), st, Some(tp)))),
    ]);
    // Place 1 Field Spell face-up in the Field Zone
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_LEAVE_GRAVE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond1)));
    effect::set_target(d, e1, Some(cost_target(pltg)));
    effect::set_operation(d, e1, Some(operation(plop)));
    card::register_effect(d, c, e1, false);
    // Can attack directly the turn it was Special Summoned
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_DIRECT_ATTACK);
    effect::set_condition(d, e2, Some(condition(cond2)));
    card::register_effect(d, c, e2, false);
    // Normal Summon 1 Dinosaur or Dragon monster
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_target(d, e3, Some(cost_target(nstg)));
    effect::set_operation(d, e3, Some(operation(nsop)));
    card::register_effect(d, c, e3, false);
}
/// `function(e) return e:GetHandler():IsFusionSummoned() end`
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, h)
}
/// `function(e) return e:GetHandler():IsStatus(STATUS_SPSUMMON_TURN) end`
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_status(d, h, STATUS_SPSUMMON_TURN)
}
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_field_spell(d, c) && !card::is_forbidden(d, c)
}
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&plfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_possible_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
fn plop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
    let f = aux::necro_valley_filter(plfilter);
    let g = duel::select_matching_card(d, a.tp, Some(&f), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let Some(tc) = group::get_first(d, &g) else { return };
    let fc = duel::get_field_card(d, a.tp, LOCATION_FZONE, 0);
    if let Some(fc) = fc {
        duel::send_to_grave(d, CardSet::from_card(fc), REASON_RULE, None, None);
        duel::break_effect(d);
    }
    duel::move_to_field(d, tc, a.tp, a.tp, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None);
}
fn nsfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_race(d, c, RACE_DINOSAUR | RACE_DRAGON) && card::is_summonable(d, c, true, None, 0, None)
}
fn nstg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&nsfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_MZONE) as i32);
    true
}
fn nsop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SUMMON as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&nsfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    if let Some(sc) = group::get_first(d, &g) {
        duel::summon(d, a.tp, sc, true, None, 0, None);
    }
}
