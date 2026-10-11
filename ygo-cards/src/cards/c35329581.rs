//! Shadow Impulse
use super::prelude::*;

pub const ID: u32 = 35329581;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 Synchro monster from the Extra Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLocation(LOCATION_GRAVE) and c:IsCanBeEffectTarget(e) and c:IsPreviousLocation(LOCATION_MZONE)
    //     and c:IsPreviousControler(tp) and c:IsReason(REASON_DESTROY) and c:IsReason(REASON_BATTLE|REASON_EFFECT)
    //     and c:IsType(TYPE_SYNCHRO) and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c)
    card::is_location(d, c, LOCATION_GRAVE)
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_controler(d, c, tp)
        && card::is_reason(d, c, REASON_DESTROY)
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && card::is_type(d, c, TYPE_SYNCHRO)
        && duel::is_existing_matching_card(d, Some(&|d, sc| spfilter(d, sc, e, tp, c)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,tc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) -> bool {
    // return c:IsType(TYPE_SYNCHRO) and c:GetLevel()==tc:GetLevel()
    //     and c:GetRace()==tc:GetRace() and c:GetCode()~=tc:GetCode()
    //     and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_type(d, c, TYPE_SYNCHRO)
        && card::get_level(d, c) == card::get_level(d, tc)
        && card::get_race(d, c) == card::get_race(d, tc)
        && card::get_code(d, c)[0] != card::get_code(d, tc)[0]
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return eg:IsContains(chkc) and s.filter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&a.eg.clone().nil(), chkc) && filter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return eg:IsExists(s.filter,1,nil,e,tp) end
    if a.chk == Some(0) {
        return group::is_exists(d, &a.eg.clone().nil(), &|d, c| filter(d, c, a.e, a.tp), 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local g=eg:FilterSelect(tp,s.filter,1,1,nil,e,tp)
    let g = group::filter_select(d, &a.eg.clone().nil(), a.tp, &|d, c| filter(d, c, a.e, a.tp), 1, 1, false, NO_EX).nil();
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or tc:IsFacedown() then return end
    if !card::is_relate_to_effect(d, tc, a.e) || card::is_facedown(d, tc) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,tc)
    let sg = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp, tc)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
