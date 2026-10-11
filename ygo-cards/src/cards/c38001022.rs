//! Barian's Seal
use super::prelude::*;

pub const ID: u32 = 38001022;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CXYZ, SET_NUMBER_C, SET_UMBRAL_HORROR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When your opponent activates a Spell/Trap Card, or monster effect, while you control a "CXyz" monster or a "Number C" monster with a number between "101" and "107" in its name: Negate the activation, then you can attach 1 monster from either field or GY to 1 Xyz Monster you control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_NEGATE)
    effect::set_category(d, e1, CATEGORY_NEGATE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.negcon)
    effect::set_condition(d, e1, Some(condition(negcon)));
    // e1:SetTarget(s.negtg)
    effect::set_target(d, e1, Some(cost_target(negtg)));
    // e1:SetOperation(s.negop)
    effect::set_operation(d, e1, Some(operation(negop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can banish this card from your GY; Special Summon 2 "Umbral Horror" monsters from your GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.negconfilter(c)`
fn negconfilter(d: &mut Duel, c: CardId) -> bool {
    // if c:IsFacedown() then return false end
    if card::is_facedown(d, c) {
        return false;
    }
    // local xyznumber=c.xyz_number
    let xyznumber = crate::card_info(d, c).xyz_number;
    // return c:IsSetCard(SET_CXYZ) or (c:IsSetCard(SET_NUMBER_C) and xyznumber and xyznumber>=101 and xyznumber<=107)
    card::is_set_card(d, c, &[SET_CXYZ as u16])
        || (card::is_set_card(d, c, &[SET_NUMBER_C as u16]) && xyznumber.is_some_and(|n| (101..=107).contains(&n)))
}
/// `s.negcon`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    a.rp == 1 - a.tp
        && (effect::is_monster_effect(d, re) || effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE))
        && duel::is_chain_negatable(d, a.ev as u8)
        && duel::is_existing_matching_card(d, Some(&negconfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.negtg`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, a.tp, 0);
    true
}
/// `s.attachfilter(c,tp)`
fn attachfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_monster(d, c) && duel::is_existing_matching_card(d, Some(&|d, x| xyzfilter(d, x, c, tp)), tp, LOCATION_MZONE, 0, 1, c.into())
}
/// `s.xyzfilter(c,mc,tp)`
fn xyzfilter(d: &mut Duel, c: CardId, mc: CardId, tp: u8) -> bool {
    card::is_xyz_monster(d, c) && card::is_faceup(d, c) && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), Some(REASON_EFFECT))
}
/// `s.negop`
fn negop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.attachfilter),tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,1,nil,tp)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if duel::negate_activation(d, a.ev as u8)
        && duel::is_existing_matching_card(
            d,
            Some(&aux::necro_valley_filter(|d, c| attachfilter(d, c, a.tp))),
            a.tp,
            LOCATION_MZONE | LOCATION_GRAVE,
            LOCATION_MZONE | LOCATION_GRAVE,
            1,
            NO_EX,
        )
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTACH)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATTACH as u64);
        // local mc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.attachfilter),tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,1,1,nil,tp):GetFirst()
        let sg = duel::select_matching_card(
            d,
            a.tp,
            Some(&aux::necro_valley_filter(|d, c| attachfilter(d, c, a.tp))),
            a.tp,
            LOCATION_MZONE | LOCATION_GRAVE,
            LOCATION_MZONE | LOCATION_GRAVE,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if not mc then return end
        let mc = match group::get_first(d, &sg) {
            Some(mc) => mc,
            None => return,
        };
        // Duel.HintSelection(mc)
        duel::hint_selection(d, mc, true);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
        // local xyzc=Duel.SelectMatchingCard(tp,s.xyzfilter,tp,LOCATION_MZONE,0,1,1,nil,mc,tp):GetFirst()
        let xg = duel::select_matching_card(d, a.tp, Some(&|d, c| xyzfilter(d, c, mc, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        // if not xyzc then return end
        let xyzc = match group::get_first(d, &xg) {
            Some(x) => x,
            None => return,
        };
        // Duel.HintSelection(xyzc)
        duel::hint_selection(d, xyzc, true);
        // if not xyzc:IsImmuneToEffect(e) then
        if !card::is_immune_to_effect(d, xyzc, a.e) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Overlay(xyzc,mc,true)
            duel::overlay(d, xyzc, CardSet::from_card(mc), true);
        }
    }
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_UMBRAL_HORROR as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>=2
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,2,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 >= 2
            && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 2, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<2 or Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 2 || duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_GRAVE,0,2,2,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 2, 2, false, NO_EX).nil();
    // if #g==2 then
    if g.len() == 2 {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
