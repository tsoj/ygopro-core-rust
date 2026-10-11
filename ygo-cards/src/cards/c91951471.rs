//! Vanquish Soul Dust Devil
//! Scripted by Larry126
use super::prelude::*;

pub const ID: u32 = 91951471;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VANQUISH_SOUL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_VANQUISH_SOUL) and c:IsCanChangePosition()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_VANQUISH_SOUL as u16]) && card::is_can_change_position(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.tgfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tgfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
    // local g=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&tgfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, g, 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_POSITION,nil,1,1-tp,POS_FACEDOWN_DEFENSE)
    duel::set_possible_operation_info(d, 0, CATEGORY_POSITION, None, 1, 1 - tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.ChangePosition(tc,POS_FACEUP_DEFENSE,POS_FACEDOWN_DEFENSE,POS_FACEUP_ATTACK,POS_FACEUP_ATTACK)>0 then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::change_position(
            d,
            tc,
            POS_FACEUP_DEFENSE as u8,
            Some(POS_FACEDOWN_DEFENSE as u8),
            Some(POS_FACEUP_ATTACK as u8),
            Some(POS_FACEUP_ATTACK as u8),
            false,
        ) > 0
    {
        // local ct=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_VANQUISH_SOUL),tp,LOCATION_MZONE,0,nil):GetClassCount(Card.GetCode)
        let own = duel::get_matching_group(
            d,
            Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_VANQUISH_SOUL as u16]))),
            tp,
            LOCATION_MZONE,
            0,
            NO_EX,
        );
        let ct = group::get_class_count(d, &own, &|d, c| card::get_code(d, c)[0] as i64);
        // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsCanTurnSet),tp,0,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_can_turn_set(d, c))), tp, 0, LOCATION_MZONE, NO_EX);
        // if ct>0 and #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if ct > 0 && g.len() > 0 && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
            // local fg=g:Select(tp,1,ct,nil)
            let fg = group::select(d, &g, tp, 1, ct as u16, false, NO_EX).nil();
            // Duel.HintSelection(fg,true)
            duel::hint_selection(d, fg.clone(), true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.ChangePosition(fg,POS_FACEDOWN_DEFENSE)
            duel::change_position(d, fg, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
        }
    }
}
