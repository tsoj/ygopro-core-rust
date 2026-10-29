//! Gandora-G the Dragon of Destruction
use super::prelude::*;

pub const ID: u32 = 2333466;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_SHINING_SARCOPHAGUS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Gains 300 ATK for each banished card
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value_fn(d, e1, effect_value(|d, _e| duel::get_field_group_count(d, 0, LOCATION_REMOVED, LOCATION_REMOVED) as i64 * 300));
    card::register_effect(d, c, e1, false);
    // Special Summon this card from your hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // Destroy as many other cards on the field as possible
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_DESTROY | CATEGORY_REMOVE | CATEGORY_SPECIAL_SUMMON | CATEGORY_LVCHANGE);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_cost(d, e3, Some(cost_target(descost)));
    effect::set_target(d, e3, Some(cost_target(destg)));
    effect::set_operation(d, e3, Some(operation(desop)));
    card::register_effect(d, c, e3, false);
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, x| card::is_code(d, x, &[CARD_SHINING_SARCOPHAGUS]))), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let lp = duel::get_lp(d, a.tp);
    duel::pay_lp_cost(d, a.tp, lp.div_euclid(2) as u32);
    true
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_level_below(d, c, 7) && card::lists_code(d, c, &[CARD_SHINING_SARCOPHAGUS]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let h = effect::get_handler(d, e).nil();
    let rg = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, h.into());
    if a.chk == Some(0) {
        return !rg.is_empty()
            && duel::get_m_zone_count(d, tp, (&rg).into(), None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    let cnt = rg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(rg.clone()), cnt, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(rg), cnt, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    let exc = if card::is_relate_to_effect(d, c, e) { c.into() } else { NO_EX };
    let rg = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, exc);
    let ct = duel::destroy(d, rg, REASON_EFFECT, Some(LOCATION_REMOVED), None);
    if ct == 0 || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let Some(sc) = group::get_first(d, &g) else { return };
    duel::break_effect(d);
    if duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
        // Increase its Level by the number of cards destroyed by this effect
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        effect::set_value(d, e1, ct as i64);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, sc, e1, false);
    }
    duel::special_summon_complete(d);
}
