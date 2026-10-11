//! Super Vehicroid - Mobile Base
use super::prelude::*;

pub const ID: u32 = 17745969;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ROID], material_setcode: &[SET_ROID as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,s.matfilter,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_ROID))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| matfilter(d, c, fc, st, tp))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_ROID as u16], Some(fc), st, Some(tp)))),
        ],
    );
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // to hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetTarget(s.mvtg)
    effect::set_target(d, e2, Some(cost_target(mvtg)));
    // e2:SetOperation(s.mvop)
    effect::set_operation(d, e2, Some(operation(mvop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.matfilter(c,fc,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsType(TYPE_FUSION,fc,sumtype,tp) and c:IsSetCard(SET_ROID,fc,sumtype,tp)
    card::is_type_ctx(d, c, TYPE_FUSION, Some(fc), sumtype, Some(tp)) && card::is_set_card_ctx(d, c, &[SET_ROID as u16], Some(fc), sumtype, Some(tp))
}
/// `s.spfilter1(c,e,tp,loc)`
fn spfilter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8, loc: u32) -> bool {
    // return c:IsFaceup() and Duel.IsExistingMatchingCard(s.spfilter2,tp,loc,0,1,nil,e,tp,c:GetAttack())
    card::is_faceup(d, c) && {
        let atk = card::get_attack(d, c);
        duel::is_existing_matching_card(d, Some(&|d, x| spfilter2(d, x, e, tp, atk)), tp, loc, 0, 1, NO_EX)
    }
}
/// `s.spfilter2(c,e,tp,atk)`
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, atk: i32) -> bool {
    // if c:IsLocation(LOCATION_EXTRA) and Duel.GetLocationCountFromEx(tp,tp,nil,c)==0 then return false end
    if card::is_location(d, c, LOCATION_EXTRA) && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 == 0 {
        return false;
    }
    // return c:IsSetCard(SET_ROID) and c:IsAttackBelow(atk) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_ROID as u16]) && card::is_attack_below(d, c, atk) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `local loc=LOCATION_EXTRA; if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then loc=loc|LOCATION_DECK end`
fn summon_location(d: &mut Duel, tp: u8) -> u32 {
    let mut loc = LOCATION_EXTRA;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        loc |= LOCATION_DECK;
    }
    loc
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let loc = summon_location(d, tp);
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and s.spfilter1(chkc,e,tp,loc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MZONE) && spfilter1(d, chkc, e, tp, loc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.spfilter1,tp,0,LOCATION_MZONE,1,nil,e,tp,loc) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| spfilter1(d, c, e, tp, loc)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.spfilter1,tp,0,LOCATION_MZONE,1,1,nil,e,tp,loc)
    let _ = duel::select_target(d, tp, Some(&|d, c| spfilter1(d, c, e, tp, loc)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,loc)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, loc as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let loc = summon_location(d, tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter2,tp,loc,0,1,1,nil,e,tp,tc:GetAttack())
        let atk = card::get_attack(d, tc);
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter2(d, c, e, tp, atk)), tp, loc, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
/// `s.mvfilter(c)`
fn mvfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_ROID) and c:IsAbleToHand() and c:GetSequence()<5
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ROID as u16]) && card::is_able_to_hand(d, c, None) && card::get_sequence(d, c) < 5
}
/// `s.mvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn mvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.mvfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && mvfilter(d, chkc);
    }
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingTarget(s.mvfilter,tp,LOCATION_MZONE,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&mvfilter), tp, LOCATION_MZONE, 0, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.mvfilter,tp,LOCATION_MZONE,0,1,1,e:GetHandler())
    let g = duel::select_target(d, tp, Some(&mvfilter), tp, LOCATION_MZONE, 0, 1, 1, false, h.into());
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    true
}
/// `s.mvop(e,tp,eg,ep,ev,re,r,rp)`
fn mvop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and tc:IsControler(tp) and Duel.SendtoHand(tc,nil,REASON_EFFECT)>0
    //     and tc:IsLocation(LOCATION_HAND) and c:IsFaceup() and c:IsRelateToEffect(e) then
    let Some(tc) = tc else {
        return;
    };
    if card::is_relate_to_effect(d, tc, e)
        && card::is_controler(d, tc, tp)
        && duel::send_to_hand(d, tc, None, REASON_EFFECT, None) > 0
        && card::is_location(d, tc, LOCATION_HAND)
        && card::is_faceup(d, c)
        && card::is_relate_to_effect(d, c, e)
    {
        // local seq=tc:GetPreviousSequence()
        let seq = card::get_previous_sequence(d, tc);
        // Duel.MoveSequence(c,seq)
        duel::move_sequence(d, c, seq as u8, None);
    }
}
