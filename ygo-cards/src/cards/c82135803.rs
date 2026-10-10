//! Fiendsmith's Desirae
use super::prelude::*;

pub const ID: u32 = 82135803;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[60764609], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Fusion Materials
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::Code(60764609), 1), (fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| matfilter(d, c, fc, st, tp))), 2)]);
    // Negate the effects of face-up cards on the field
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DISABLE);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_hint_timing(d, e1, 0, Some((TIMING_MAIN_END | TIMINGS_CHECK_MONSTER) as u32));
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(distg)));
    effect::set_operation(d, e1, Some(operation(disop)));
    card::register_effect(d, c, e1, false);
    // Send 1 card on the field to the GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost_target(tgcost)));
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    effect::set_operation(d, e2, Some(operation(tgop)));
    card::register_effect(d, c, e2, false);
}
fn matfilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT, Some(fc), sumtype, Some(tp)) && card::is_race_ctx(d, c, RACE_FIEND, Some(fc), sumtype, Some(tp))
}
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        let eg = card::get_equip_group(d, c);
        let fu = group::filter(d, &eg, &|d, x| card::is_faceup(d, x), NO_EX);
        return group::get_sum(d, &fu, &|d, x| card::get_link(d, x) as i64) > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| card::is_negatable(d, x)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, None, 1, a.tp, LOCATION_ONFIELD as i32);
    true
}
fn disop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let eg = card::get_equip_group(d, c);
    let fu = group::filter(d, &eg, &|d, x| card::is_faceup(d, x), NO_EX);
    let ct = group::get_sum(d, &fu, &|d, x| card::get_link(d, x) as i64);
    if ct == 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, x| card::is_negatable(d, x)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, ct as u16, false, NO_EX).nil();
    if g.is_empty() {
        return;
    }
    duel::hint_selection(d, g.clone(), true);
    for tc in group::iter(d, &g) {
        // Negate its effects until the end of this turn
        card::negate_effects(d, tc, c, Some(RESET_PHASE | PHASE_END), true, None);
    }
}
fn tgcostfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_FIEND) && card::is_able_to_deck_or_extra_as_cost(d, c)
}
fn tgcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgcostfilter), tp, LOCATION_GRAVE, 0, 1, c.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    let g = duel::select_matching_card(d, tp, Some(&tgcostfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, c.into()).nil();
    duel::hint_selection(d, g.clone(), true);
    duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_COST, None);
    true
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| card::is_able_to_grave(d, x, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, x| card::is_able_to_grave(d, x, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, g, 1, a.tp, 0);
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::send_to_grave(d, tc, REASON_EFFECT, None, None);
    }
}
