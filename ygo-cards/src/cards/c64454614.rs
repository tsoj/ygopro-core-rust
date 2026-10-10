//! Arcana Extra Joker
use super::prelude::*;

pub const ID: u32 = 64454614;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Link summon procedure
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_WARRIOR),3,3,s.lcheck)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_WARRIOR, sc, st, Some(tp)))), 3, Some(3), Some(group_check(lcheck)), None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Negate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_NEGATE);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(negcon)));
    effect::set_cost(d, e1, Some(cost_target(negcost)));
    effect::set_target(d, e1, Some(cost_target(negtg)));
    effect::set_operation(d, e1, Some(operation(negop)));
    card::register_effect(d, c, e1, false);
    // Special summon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // aux.DoubleSnareValidity(c,LOCATION_MZONE)
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}
/// `s.lcheck(g,lc,sumtype,tp)`: `g:CheckDifferentProperty(Card.GetCode,lc,sumtype,tp)`
fn lcheck(d: &mut Duel, g: &CardSet, _lc: CardId, _sumtype: u64, _tp: u8) -> bool {
    group::check_different_property(d, g, &|d, c| card::get_code(d, c).into_iter().map(i64::from).collect())
}
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_status(d, c, STATUS_BATTLE_DESTROYED) {
        return false;
    }
    let Some(re) = a.re else {
        return false;
    };
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    let mut lg = card::get_linked_group(d, c);
    group::add_card(&mut lg, &CardSet::from_card(c));
    let tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS as u32])[0].as_group().cloned();
    match tg {
        Some(tg) => group::is_exists(d, &lg, &|_d, x| group::is_contains(&tg, x), 1, NO_EX) && duel::is_chain_negatable(d, a.ev as u8),
        None => false,
    }
}
fn costfilter(d: &mut Duel, c: CardId, tpe: u32) -> bool {
    card::is_type(d, c, tpe) && card::is_discardable(d, c, None, None)
}
fn negcost(d: &mut Duel, a: &EffArgs) -> bool {
    let rtype = effect::get_active_type(d, a.re.nil()) & (TYPE_MONSTER | TYPE_SPELL | TYPE_TRAP);
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| costfilter(d, c, rtype)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::discard_hand(d, a.tp, Some(&|d, c| costfilter(d, c, rtype)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    true
}
fn negop(d: &mut Duel, a: &EffArgs) {
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && card::is_relate_to_effect(d, effect::get_handler(d, re).nil(), re) {
            duel::send_to_grave(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_location(d, c, LOCATION_GRAVE) && card::is_reason(d, c, REASON_BATTLE) && card::is_link_summoned(d, c)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_type(d, c, TYPE_NORMAL)
        && card::is_level(d, c, &[4])
        && card::is_race(d, c, RACE_WARRIOR)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(0xff))
        && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, c.into())
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_level(d, c, &[4]) && card::is_race(d, c, RACE_WARRIOR) && card::is_able_to_hand(d, c, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g1 = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g1.is_empty() && duel::special_summon(d, g1.clone(), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) != 0 {
        duel::confirm_cards(d, 1 - a.tp, g1);
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        let g2 = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        if !g2.is_empty() {
            duel::send_to_hand(d, g2, Some(a.tp), REASON_EFFECT, None);
        }
    }
}
