//! Reeshaddoll Wendikurhu
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 8852158;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SHADDOLL], material_setcode: &[SET_SHADDOLL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // c:AddMustFirstBeFusionSummoned()
    card::add_must_first_be_fusion_summoned(d, c);
    // Fusion Materials: 1 "Shaddoll" monster + 1 WIND monster
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_SHADDOLL),s.matfilter)
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_set_card_ctx(d, x, &[SET_SHADDOLL as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| matfilter(d, x, fc, st, tp))),
        ],
    );
    // Change any number of face-down monsters on the field to face-up Defense Position
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.postg)
    effect::set_target(d, e1, Some(cost_target(postg)));
    // e1:SetOperation(s.posop)
    effect::set_operation(d, e1, Some(operation(posop)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 "Shaddoll" card from your GY to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.matfilter(c,lc,sumtype,tp)
fn matfilter(d: &mut Duel, c: CardId, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WIND,lc,sumtype,tp) or c:IsHasEffect(4904633)
    card::is_attribute_ctx(d, c, ATTRIBUTE_WIND, Some(lc), sumtype, Some(tp)) || !card::is_has_effect(d, c, 4904633, None).is_empty()
}
// function s.posfilter(c)
fn posfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPosition(POS_FACEDOWN_DEFENSE) and c:IsCanChangePosition()
    card::is_position(d, c, POS_FACEDOWN_DEFENSE) && card::is_can_change_position(d, c)
}
// function s.postg(e,tp,eg,ep,ev,re,r,rp,chk)
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.posfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&posfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,nil,1,PLAYER_EITHER,POS_FACEUP_DEFENSE|POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, None, 1, PLAYER_EITHER as u8, (POS_FACEUP_DEFENSE | POS_FACEDOWN_DEFENSE) as i32);
    true
}
// function s.posop(e,tp,eg,ep,ev,re,r,rp,chk)
fn posop(d: &mut Duel, a: &EffArgs) {
    // local facedown_ct=Duel.GetMatchingGroupCount(s.posfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let facedown_ct = duel::get_matching_group_count(d, Some(&posfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if facedown_ct==0 then return end
    if facedown_ct == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_POSCHANGE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.posfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,facedown_ct,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&posfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, facedown_ct as u16, false, NO_EX).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.HintSelection(g)
    duel::hint_selection(d, g.clone(), true);
    // if Duel.ChangePosition(g,POS_FACEUP_DEFENSE)==0 then return end
    if duel::change_position(d, g.clone(), POS_FACEUP_DEFENSE as u8, None, None, None, false) == 0 {
        return;
    }
    // local og=Duel.GetOperatedGroup()
    let og = duel::get_operated_group(d);
    // local flip_count=og:FilterCount(Card.IsType,nil,TYPE_FLIP)
    let flip_count = group::filter_count(d, &og, &|d, c| card::is_type(d, c, TYPE_FLIP), NO_EX);
    // if flip_count>0 and Duel.IsExistingMatchingCard(Card.IsCanTurnSet,tp,LOCATION_MZONE,LOCATION_MZONE,1,og)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if flip_count > 0
        && duel::is_existing_matching_card(d, Some(&card::is_can_turn_set), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, (&og).into())
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_POSCHANGE as u64);
        // local pos_g=Duel.SelectMatchingCard(tp,Card.IsCanTurnSet,tp,LOCATION_MZONE,LOCATION_MZONE,1,flip_count,og)
        let pos_g = duel::select_matching_card(d, a.tp, Some(&card::is_can_turn_set), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, flip_count as u16, false, (&og).into()).nil();
        // if #pos_g>0 then
        if !pos_g.is_empty() {
            // Duel.HintSelection(pos_g)
            duel::hint_selection(d, pos_g.clone(), true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.ChangePosition(pos_g,POS_FACEDOWN_DEFENSE)
            duel::change_position(d, pos_g, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
        }
    }
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SHADDOLL) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_SHADDOLL as u16]) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && thfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, a.tp, 0);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(tc), None, REASON_EFFECT, None);
    }
}
