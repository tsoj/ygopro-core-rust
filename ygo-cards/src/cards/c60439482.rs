//! Regulus' Arrow
use super::prelude::*;

pub const ID: u32 = 60439482;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_REGULUS_THE_PRINCE_OF_ENDYMION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    effect::set_target(d, e1, Some(cost_target(efftg)));
    effect::set_operation(d, e1, Some(operation(effop)));
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMING_BATTLE_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Regulus, the Prince of Endymion" or 1 monster that mentions it from your GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    card::get_battled_group_count(d, c) > 0
}
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let b1 = duel::is_existing_matching_card(d, Some(&desfilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    let chain = duel::check_event(d, EVENT_CHAINING);
    let event_chaining = chain.is_some();
    let event_value = chain.as_ref().map(|ev| ev.event_value as i64);
    let reason_effect = chain.as_ref().and_then(|ev| ev.reason_effect);
    let reason_card = match reason_effect {
        Some(re) => effect::get_handler(d, re),
        None => None,
    };
    let b2 = event_chaining
        && effect::is_has_type(d, reason_effect.nil(), EFFECT_TYPE_ACTIVATE)
        && (card::is_normal_spell(d, reason_card.nil()) || card::is_quick_play_spell(d, reason_card.nil()))
        && duel::is_chain_disablable(d, a.ev as u8);
    if a.chk == Some(0) {
        return b1 || b2;
    }
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]).map_or(0, |x| x as i64);
    effect::set_label(d, a.e, &[op, event_value.unwrap_or(0)]);
    if op == 1 {
        effect::set_category(d, a.e, CATEGORY_DESTROY);
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    } else if op == 2 {
        effect::set_category(d, a.e, CATEGORY_DISABLE);
        duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, a.tp, 0);
    }
    true
}
fn effop(d: &mut Duel, a: &EffArgs) {
    let labels = effect::get_labels(d, a.e);
    let op = labels.first().copied().unwrap_or(0);
    let event_value = labels.get(1).copied().unwrap_or(0);
    if op == 1 {
        // Destroy 1 monster your opponent controls that battled this turn
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        let g = duel::select_matching_card(d, a.tp, Some(&desfilter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        if !g.is_empty() {
            duel::hint_selection(d, g.clone(), true);
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    } else if op == 2 {
        // When a Normal or Quick-Play Spell Card is activated: Negate that effect
        duel::negate_effect(d, event_value as u8);
    }
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    (card::is_code(d, c, &[CARD_REGULUS_THE_PRINCE_OF_ENDYMION]) || card::lists_code(d, c, &[CARD_REGULUS_THE_PRINCE_OF_ENDYMION]))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, a.e, a.tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, a.tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn e1_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_REGULUS_THE_PRINCE_OF_ENDYMION])), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
