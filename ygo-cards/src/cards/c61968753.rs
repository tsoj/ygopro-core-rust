//! Bubble Shuffle
use super::prelude::*;

pub const ID: u32 = 61968753;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ELEMENTAL_HERO], listed_names: &[79979666], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Tribute "Elemental HERO Bubbleman", then special summon 1 "Elemental HERO" monster from hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_POSITION+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.postg)
    effect::set_target(d, e1, Some(cost_target(postg)));
    // e1:SetOperation(s.posop)
    effect::set_operation(d, e1, Some(operation(posop)));
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.filter1(c,ft)
fn filter1(d: &mut Duel, c: CardId, ft: i32) -> bool {
    // return c:IsPosition(POS_FACEUP_ATTACK) and c:IsCanChangePosition() and c:IsCode(79979666)
    //     and (ft>0 or c:GetSequence()<5)
    card::is_position(d, c, POS_FACEUP_ATTACK)
        && card::is_can_change_position(d, c)
        && card::is_code(d, c, &[79979666])
        && (ft > 0 || card::get_sequence(d, c) < 5)
}
// function s.filter2(c)
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPosition(POS_FACEUP_ATTACK) and c:IsCanChangePosition()
    card::is_position(d, c, POS_FACEUP_ATTACK) && card::is_can_change_position(d, c)
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ELEMENTAL_HERO) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.postg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return ft>-1 and Duel.IsExistingTarget(s.filter1,tp,LOCATION_MZONE,0,1,nil,ft)
    //     and Duel.IsExistingTarget(s.filter2,tp,0,LOCATION_MZONE,1,nil)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return ft > -1
            && duel::is_existing_target(d, Some(&|d, x| filter1(d, x, ft)), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&filter2), tp, 0, LOCATION_MZONE, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
    // local g1=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_MZONE,0,1,1,nil,ft)
    let mut g1 = duel::select_target(d, tp, Some(&|d, x| filter1(d, x, ft)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // e:SetLabelObject(g1:GetFirst())
    let first = group::get_first(d, &g1);
    effect::set_label_object(d, e, first.map_or(Value::Nil, Value::Card));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
    // local g2=Duel.SelectTarget(tp,s.filter2,tp,0,LOCATION_MZONE,1,1,nil)
    let g2 = duel::select_target(d, tp, Some(&filter2), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g1,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g1), 2, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
// function s.pfilter(c,e)
fn pfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsPosition(POS_FACEUP_ATTACK) and c:IsRelateToEffect(e)
    card::is_position(d, c, POS_FACEUP_ATTACK) && card::is_relate_to_effect(d, c, e)
}
// function s.posop(e,tp,eg,ep,ev,re,r,rp)
fn posop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS):Filter(s.pfilter,nil,e)
    let tg = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    let g = group::filter(d, &tg, &|d, x| pfilter(d, x, e), NO_EX);
    // if #g==2 then
    if g.len() == 2 {
        // Duel.ChangePosition(g,POS_FACEUP_DEFENSE)
        duel::change_position(d, g, POS_FACEUP_DEFENSE as u8, None, None, None, false);
        // local tc=e:GetLabelObject()
        let tc = effect::get_label_object(d, e).as_card().nil();
        // if not tc:IsImmuneToEffect(e) and tc:IsReleasable() and (Duel.GetLocationCount(tp,LOCATION_MZONE)>0 or tc:GetSequence()<5) then
        if !card::is_immune_to_effect(d, tc, e)
            && card::is_releasable(d, tc, None, None)
            && (duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 || card::get_sequence(d, tc) < 5)
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
            let sg = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Release(tc,REASON_EFFECT)
            duel::release(d, tc, REASON_EFFECT, None);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
