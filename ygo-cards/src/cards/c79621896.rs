//! Archfiend's Origin
use super::prelude::*;

pub const ID: u32 = 79621896;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_SUMMONED_SKULL], listed_series: &[SET_ARCHFIEND], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If this card is in your hand: You can Tribute 1 other "Archfiend" monster from your hand or field; Special Summon this card
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(selfspcost)));
    effect::set_target(d, e1, Some(cost_target(selfsptg)));
    effect::set_operation(d, e1, Some(operation(selfspop)));
    card::register_effect(d, c, e1, false);
    // If this card is Normal or Special Summoned: You can Special Summon 1 Level 6 Fiend monster with 2500 ATK from your hand, Deck, or GY
    let e2a = effect::create_effect(d, c);
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    effect::set_category(d, e2a, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2a, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2a, EVENT_SUMMON_SUCCESS);
    effect::set_count_limit(d, e2a, 1, ID, 1, 0);
    effect::set_target(d, e2a, Some(cost_target(lv6fiendsptg)));
    effect::set_operation(d, e2a, Some(operation(lv6fiendspop)));
    card::register_effect(d, c, e2a, false);
    let e2b = effect::clone(d, e2a);
    effect::set_code(d, e2b, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e2b, false);
    // When your opponent activates a monster effect and you control "Summoned Skull" (Quick Effect): You can negate the activation, and if you do, destroy that card
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_NEGATE + CATEGORY_DESTROY);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_code(d, e3, EVENT_CHAINING);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    effect::set_condition(d, e3, Some(condition(negcon)));
    effect::set_target(d, e3, Some(cost_target(negtg)));
    effect::set_operation(d, e3, Some(operation(negop)));
    card::register_effect(d, c, e3, false);
}
fn selfspcost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let f = |d: &mut Duel, c: CardId| card::is_set_card(d, c, &[SET_ARCHFIEND as u16]);
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&f), 1, 1, true, Some(&|d, sg, tp, exg| aux::release_check_mmz(d, sg, tp, exg)), c.into());
    }
    let g = duel::select_release_group_cost(d, a.tp, Some(&f), 1, 1, true, Some(&|d, sg, tp, exg| aux::release_check_mmz(d, sg, tp, exg)), c.into());
    duel::release(d, g, REASON_COST, None);
    true
}
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn selfspop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn lv6fiendspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_level(d, c, &[6]) && card::is_race(d, c, RACE_FIEND) && card::is_attack(d, c, &[2500]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn lv6fiendsptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| lv6fiendspfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
fn lv6fiendspop(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let f = aux::necro_valley_filter(|d, c| lv6fiendspfilter(d, c, a.e, a.tp));
    let g = duel::select_matching_card(d, a.tp, Some(&f), a.tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    a.ep == 1 - a.tp
        && effect::is_monster_effect(d, a.re.nil())
        && duel::is_chain_negatable(d, a.ev as u8)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_SUMMONED_SKULL])), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let rc = effect::get_handler(d, a.re.nil()).nil();
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, a.tp, 0);
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, a.re.nil()) {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, a.tp, 0);
    }
    true
}
fn negop(d: &mut Duel, a: &EffArgs) {
    if duel::negate_activation(d, a.ev as u8) && card::is_relate_to_effect(d, effect::get_handler(d, a.re.nil()).nil(), a.re.nil()) {
        duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
    }
}
