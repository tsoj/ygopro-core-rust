//! Level Modulation
use super::prelude::*;

pub const ID: u32 = 61850482;

// s.listed_series={SET_LV}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LV], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Opponent draws 2, then you special summon 1 "LV" monster from GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DRAW);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_LV) and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_set_card(d, c, &[SET_LV as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && filter(d, chkc, e, tp);
    }
    // if chk==0 then return Duel.IsPlayerCanDraw(1-tp,2) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, 1 - tp, 2)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,0,0,1-tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, 1 - tp, 2);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Draw(1-tp,2,REASON_EFFECT)
    duel::draw(d, 1 - tp, 2, REASON_EFFECT);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // if Duel.SpecialSummon(tc,0,tp,tp,true,false,POS_FACEUP)==0 then return end
    if duel::special_summon(d, tc, 0, tp, tp, true, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Cannot attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(3206)
    effect::set_description(d, e1, 3206);
    // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // tc:RegisterEffect(e1,true)
    card::register_effect(d, tc, e1, true);
    // Cannot apply its effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(3314)
    effect::set_description(d, e2, 3314);
    // e2:SetProperty(EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e2, EFFECT_FLAG_CLIENT_HINT, 0);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e2, EFFECT_DISABLE);
    // e2:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
    // tc:RegisterEffect(e2,true)
    card::register_effect(d, tc, e2, true);
    // Cannot activate its effects
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(3302)
    effect::set_description(d, e3, 3302);
    // e3:SetProperty(EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e3, EFFECT_FLAG_CLIENT_HINT, 0);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_CANNOT_TRIGGER)
    effect::set_code(d, e3, EFFECT_CANNOT_TRIGGER);
    // e3:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
    // tc:RegisterEffect(e3,true)
    card::register_effect(d, tc, e3, true);
}
