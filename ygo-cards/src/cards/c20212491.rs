//! Stray Purrely Street
use super::prelude::*;

pub const ID: u32 = 20212491;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PURRELY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // "Purrely" monsters cannot be targeted the turn they are Special Summoned
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    effect::set_target(d, e2, Some(effect_card_fn(tgfilter)));
    effect::set_value_fn(d, e2, aux::tgoval());
    card::register_effect(d, c, e2, false);
    // Special Summon 1 Level 1 "Purrely" monster from Deck or GY
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    effect::set_range(d, e3, LOCATION_FZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(spcond)));
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
    // Attach 1 "Purrely" Quick-Play Spell to 1 "Purrely" Xyz Monster on the field
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e4, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e4, LOCATION_FZONE);
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    effect::set_target(d, e4, Some(cost_target(attachtg)));
    effect::set_operation(d, e4, Some(operation(attachop)));
    card::register_effect(d, c, e4, false);
}
fn tgfilter(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_PURRELY) and c:IsStatus(STATUS_SPSUMMON_TURN)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_PURRELY as u16]) && card::is_status(d, c, STATUS_SPSUMMON_TURN)
}
fn xyzfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_previous_position(d, c, POS_FACEUP)
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_previous_set_card(d, c, &[SET_PURRELY as u16])
        && card::is_previous_controler(d, c, tp)
        && card::get_reason_player(d, c) == 1 - tp
}
fn spcond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.xyzfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| xyzfilter(d, c, a.tp), 1, NO_EX)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_PURRELY as u16])
        && card::is_level(d, c, &[1])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let f = aux::necro_valley_filter(|d: &mut Duel, c: CardId| spfilter(d, c, e, tp));
    let g = duel::select_matching_card(d, tp, Some(&f), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_PURRELY) and c:IsType(TYPE_XYZ)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_PURRELY as u16]) && card::is_type(d, c, TYPE_XYZ)
}
fn atchfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_PURRELY) and c:IsSpell() and c:IsType(TYPE_QUICKPLAY)
    card::is_set_card(d, c, &[SET_PURRELY as u16]) && card::is_spell(d, c) && card::is_type(d, c, TYPE_QUICKPLAY)
}
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && cfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&cfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&atchfilter), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    duel::select_target(d, tp, Some(&cfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
fn attachop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && card::is_type(d, tc, TYPE_XYZ) {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_XMATERIAL as u64);
        let f = aux::necro_valley_filter(atchfilter);
        let g = duel::select_matching_card(d, tp, Some(&f), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        if g.is_empty() {
            return;
        }
        duel::overlay(d, tc, &g, false);
    }
}
