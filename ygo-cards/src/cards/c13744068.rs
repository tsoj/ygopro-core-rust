//! Berserk Archfiend
use super::prelude::*;

pub const ID: u32 = 13744068;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from the hand and destroy 2 monsters you control
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Gain ATK equal to the total original ATK of your opponent's monster
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_CUSTOM + ID);
    effect::set_target(d, e2, Some(cost_target(atktg)));
    effect::set_operation(d, e2, Some(operation(atkop)));
    card::register_effect(d, c, e2, false);
}
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    (group::is_exists(d, sg, &|d, c| card::is_race(d, c, RACE_FIEND), 1, NO_EX), false)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chkc().is_some() {
        return false;
    }
    let c = effect::get_handler(d, e).nil();
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x) && card::is_can_be_effect_target(d, x, Some(e))), tp, LOCATION_MZONE, 0, NO_EX);
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
            && !g.is_empty()
            && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true));
    }
    let dg = aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_TARGET as u64), Some(&rescon), None, false).unwrap_or_default();
    duel::set_target_card(d, dg.clone());
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, LOCATION_HAND as i32);
    let n = dg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), n, tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if !(card::is_relate_to_effect(d, c, e) && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0) {
        return;
    }
    let g = duel::get_target_cards(d, e).nil();
    if !g.is_empty() && duel::destroy(d, g, REASON_EFFECT, None, None) > 0 {
        let og = duel::get_operated_group(d);
        duel::raise_single_event(d, c, EVENT_CUSTOM + ID, Some(e), REASON_EFFECT, tp, tp, og.len() as u32);
    }
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::get_base_attack(d, c) > 0
}
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp, ev) = (a.e, a.tp, a.ev);
    if let Some(chkc) = a.chkc() {
        return ev == 1 && card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MZONE) && cfilter(d, chkc);
    }
    let c = effect::get_handler(d, e).nil();
    if a.chk == Some(0) {
        return card::is_relate_to_effect(d, c, e)
            && duel::is_existing_target(d, Some(&cfilter), tp, 0, LOCATION_MZONE, ev as u16, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    let g = duel::select_target(d, tp, Some(&cfilter), tp, 0, LOCATION_MZONE, ev as u16, ev as u16, false, NO_EX).nil();
    let atk = group::get_sum(d, &g, &|d, x| card::get_base_attack(d, x) as i64);
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(c)), 1, tp, atk as i32);
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if !(card::is_faceup(d, c) && card::is_relate_to_effect(d, c, e)) {
        return;
    }
    let tg = duel::get_target_cards(d, e).nil();
    let g = group::filter(d, &tg, &|d, x| card::is_faceup(d, x), NO_EX);
    if g.is_empty() {
        return;
    }
    let atk = group::get_sum(d, &g, &|d, x| card::get_base_attack(d, x) as i64);
    let ct = if duel::is_turn_player(d, tp) { 3 } else { 2 };
    // Gains ATK equal to those monsters' total original ATK
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e1, atk);
    effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, ct);
    card::register_effect(d, c, e1, false);
}
