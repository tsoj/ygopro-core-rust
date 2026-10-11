//! Guardian Dreadscythe
use super::prelude::*;

pub const ID: u32 = 18175965;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[34022290, 81954378], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --cannot special summon
    let e1 = effect::create_effect(d, c);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e1, false);
    // --summon limit
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_CANNOT_SUMMON);
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_CANNOT_MSET);
    card::register_effect(d, c, e3, false);
    // --special summon
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 0));
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    effect::set_property(d, e4, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e4, EVENT_TO_GRAVE);
    effect::set_range(d, e4, LOCATION_HAND);
    effect::set_condition(d, e4, Some(condition(spcon)));
    effect::set_target(d, e4, Some(cost_target(sptg)));
    effect::set_operation(d, e4, Some(operation(spop)));
    card::register_effect(d, c, e4, false);
    // --equip
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 1));
    effect::set_category(d, e5, CATEGORY_EQUIP);
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e5, EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e5, EVENT_SPSUMMON_SUCCESS);
    effect::set_target(d, e5, Some(cost_target(eqtg)));
    effect::set_operation(d, e5, Some(operation(eqop)));
    card::register_effect(d, c, e5, false);
    // --disable summon
    let e6 = effect::create_effect(d, c);
    effect::set_type(d, e6, EFFECT_TYPE_FIELD);
    effect::set_range(d, e6, LOCATION_MZONE);
    effect::set_code(d, e6, EFFECT_CANNOT_SUMMON);
    effect::set_property(d, e6, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e6, 1, 0);
    card::register_effect(d, c, e6, false);
    let e7 = effect::clone(d, e6);
    effect::set_code(d, e7, EFFECT_CANNOT_SPECIAL_SUMMON);
    card::register_effect(d, c, e7, false);
    // --special summon 2
    let e8 = effect::create_effect(d, c);
    effect::set_description(d, e8, aux::stringid(ID, 2));
    effect::set_category(d, e8, CATEGORY_HANDES + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e8, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e8, EVENT_TO_GRAVE);
    effect::set_condition(d, e8, Some(condition(spcon2)));
    effect::set_target(d, e8, Some(cost_target(sptg2)));
    effect::set_operation(d, e8, Some(operation(spop2)));
    card::register_effect(d, c, e8, false);
}
// function s.cfilter(c,tp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_faceup(d, c)
        && card::is_controler(d, c, tp)
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && card::is_reason(d, c, REASON_DESTROY)
        && card::is_code(d, c, &[34022290])
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, a.tp), 1, NO_EX)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return false;
        }
        let h = effect::get_handler(d, a.e).nil();
        return card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, true, false, None, None, None);
    }
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    if duel::special_summon(d, c, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None) != 0 {
        card::complete_procedure(d, c);
    }
}
// function s.filter(c,ec)
fn filter(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    card::is_code(d, c, &[81954378]) && card::check_equip_target(d, c, ec)
}
// function s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
            return false;
        }
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, h)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.eqop(e,tp,eg,ep,ev,re,r,rp)
fn eqop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0
        || card::is_facedown(d, c)
        || !card::is_relate_to_effect(d, c, a.e)
    {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| filter(d, x, c)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        let first = group::get_first(d, &g).nil();
        duel::equip(d, a.tp, first, c, true, false);
    }
}
// function s.spcon2(e,tp,eg,ep,ev,re,r,rp)
fn spcon2(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_ONFIELD)
}
// function s.sptg2(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg2(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 1);
    true
}
// function s.spop2(e,tp,eg,ep,ev,re,r,rp)
fn spop2(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e);
    if duel::discard_hand(d, a.tp, None, 1, 1, REASON_EFFECT, NO_EX) != 0 {
        let og = duel::get_operated_group(d);
        let ct = group::filter_count(d, &og, &|d, x| card::is_location(d, x, LOCATION_GRAVE), NO_EX);
        if ct > 0 && card::is_relate_to_effect(d, c.nil(), a.e) && card::is_status(d, c.nil(), STATUS_PROC_COMPLETE) {
            duel::special_summon(d, c.nil(), 0, a.tp, a.tp, true, true, POS_FACEUP as u8, None);
        }
    }
}
