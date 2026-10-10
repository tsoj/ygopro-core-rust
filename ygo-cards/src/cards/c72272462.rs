//! Despian Quaeritis
use super::prelude::*;

pub const ID: u32 = 72272462;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DESPIA], listed_names: &[CARD_ALBAZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 "Despia" monster + 1 LIGHT or DARK monster
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_DESPIA as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK, Some(fc), st, Some(tp)))),
        ],
    );
    // Change the ATK of all monsters currently on the field to 0 until the end of this turn, except Level 8 or higher Fusion Monsters
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    effect::set_target(d, e1, Some(cost_target(atktg)));
    effect::set_operation(d, e1, Some(operation(atkop)));
    card::register_effect(d, c, e1, false);
    // Add to your hand, or Special Summon, 1 "Fallen of Albaz" or 1 "Despia" monster, from your Deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(thspcon)));
    effect::set_target(d, e2, Some(cost_target(thsptg)));
    effect::set_operation(d, e2, Some(operation(thspop)));
    card::register_effect(d, c, e2, false);
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    card::has_non_zero_attack(d, c) && !(card::is_level_above(d, c, 8) && card::is_type(d, c, TYPE_FUSION))
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&atkfilter), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    let n = g.len();
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(g), n as u8, tp, 0);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&atkfilter), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    if g.is_empty() {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    for tc in group::iter(d, &g) {
        // Their ATK becomes 0 until the end of this turn
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        effect::set_value(d, e1, 0);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.thspcon(e,tp,eg,ep,ev,re,r,rp)`
fn thspcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    a.rp == 1 - tp && card::is_reason(d, c, REASON_EFFECT) && card::is_previous_position(d, c, POS_FACEUP) && card::is_previous_controler(d, c, tp)
}
/// `s.thspfilter(c,e,tp,mzone_chk)`
fn thspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mzone_chk: bool) -> bool {
    (card::is_set_card(d, c, &[SET_DESPIA as u16]) || card::is_code(d, c, &[CARD_ALBAZ]))
        && card::is_monster(d, c)
        && (card::is_able_to_hand(d, c, None) || (mzone_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
/// `s.thsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        let mzone_chk = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
        return duel::is_existing_matching_card(d, Some(&|d, c| thspfilter(d, c, e, tp, mzone_chk)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thspop(e,tp,eg,ep,ev,re,r,rp)`
fn thspop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local mzone_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let mzone_chk = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    let g = duel::select_matching_card(d, tp, Some(&|d, c| thspfilter(d, c, e, tp, mzone_chk)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let Some(sc) = group::get_first(d, &g) else { return };
    aux::to_hand_or_else(
        d,
        sc,
        tp,
        Some(&|d, _c| mzone_chk && card::is_can_be_special_summoned(d, sc, e, 0, tp, false, false, None, None, None)),
        Some(&|d, _g| {
            duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            0
        }),
        Some(aux::stringid(ID, 3)),
    );
}
