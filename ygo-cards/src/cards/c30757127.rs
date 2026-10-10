//! Destiny HERO - Dangerous
use super::prelude::*;

pub const ID: u32 = 30757127;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DESTINY_HERO], material_setcode: &[SET_HERO, SET_DESTINY_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_DESTINY_HERO as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(ffilter)),
        ],
    );
    // atk
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOGRAVE | CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP as u32, Some((TIMING_DAMAGE_STEP | TIMING_END_PHASE) as u32));
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    effect::set_cost(d, e1, Some(cost_target(atkcost)));
    effect::set_target(d, e1, Some(cost_target(atktg)));
    effect::set_operation(d, e1, Some(operation(atkop)));
    card::register_effect(d, c, e1, false);
}
fn ffilter(d: &mut Duel, c: CardId, fc: CardId, st: u64, tp: u8) -> bool {
    card::is_attribute_ctx(d, c, ATTRIBUTE_DARK, Some(fc), st, Some(tp)) && card::is_type_ctx(d, c, TYPE_EFFECT, Some(fc), st, Some(tp))
}
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_discardable(d, c, None, None) && duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, c.into())
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_DESTINY_HERO as u16]) && card::is_monster(d, c) && card::is_able_to_grave(d, c, None)
}
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::discard_hand(d, tp, Some(&|d, c| cfilter(d, c, tp)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_DESTINY_HERO as u16]) && card::is_faceup(d, c)
}
fn ctfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_DESTINY_HERO as u16]) && card::is_monster(d, c)
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() && duel::send_to_grave(d, g.clone(), REASON_EFFECT, None, None) != 0 && card::is_location(d, group::get_first(d, &g).nil(), LOCATION_GRAVE) {
        let tg = duel::get_matching_group(d, Some(&atkfilter), tp, LOCATION_MZONE, 0, NO_EX);
        if tg.is_empty() {
            return;
        }
        let ct = duel::get_matching_group_count(d, Some(&ctfilter), tp, LOCATION_GRAVE, 0, NO_EX) as i64;
        for tc in group::iter(d, &tg) {
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            effect::set_value(d, e1, ct * 200);
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, tc, e1, false);
        }
    }
}
