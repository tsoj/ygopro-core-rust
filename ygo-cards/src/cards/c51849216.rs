//! Dragunity Oubliette
use super::prelude::*;

pub const ID: u32 = 51849216;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRAGUNITY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_REMOVE+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_REMOVE + CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_SYNCHRO) and c:IsSetCard(SET_DRAGUNITY)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_SYNCHRO) && card::is_set_card(d, c, &[SET_DRAGUNITY as u16])
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // if ep==tp or not Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil) then return false end
    if a.ep == a.tp || !duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX) {
        return false;
    }
    // return Duel.IsChainNegatable(ev) and re:IsHasType(EFFECT_TYPE_ACTIVATE)
    let re = a.re.nil();
    duel::is_chain_negatable(d, a.ev as u8) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // local relation=rc:IsRelateToEffect(re)
    let relation = card::is_relate_to_effect(d, rc, re);
    // if chk==0 then return rc:IsAbleToRemove(tp) or (not relation and Duel.IsPlayerCanRemove(tp)) end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, rc, Some(a.tp), None, None) || (!relation && duel::is_player_can_remove(d, a.tp, None));
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    if relation {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,rc,1,rc:GetControler(),rc:GetLocation())
        let controler = card::get_controler(d, rc);
        let location = card::get_location(d, rc) as i32;
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(rc)), 1, controler, location);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,0,0,rc:GetPreviousLocation())
        let prev = card::get_previous_location(d, rc) as i32;
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 0, 0, prev);
    }
    true
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_DRAGUNITY) and c:IsLevel(10)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DRAGUNITY as u16]) && card::is_level(d, c, &[10])
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re)
    //   and Duel.Remove(eg,POS_FACEUP,REASON_EFFECT)>0 then
    if !duel::negate_activation(d, a.ev as u8) {
        return;
    }
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    if !card::is_relate_to_effect(d, rc, re) {
        return;
    }
    if duel::remove(d, a.eg(), POS_FACEUP as u8, REASON_EFFECT, None, None) <= 0 {
        return;
    }
    // local og=Duel.GetOperatedGroup()
    let og = duel::get_operated_group(d);
    // if not og:GetFirst():IsLocation(LOCATION_REMOVED) then return end
    let first = group::get_first(d, &og).nil();
    if !card::is_location(d, first, LOCATION_REMOVED) {
        return;
    }
    // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsFaceup),tp,LOCATION_REMOVED,LOCATION_REMOVED,nil)
    let ct = duel::get_matching_group_count(d, Some(&aux::faceup_filter(|d, c| card::is_faceup(d, c))), tp, LOCATION_REMOVED, LOCATION_REMOVED, NO_EX) as i64;
    // if ct>0 and Duel.IsExistingMatchingCard(s.atkfilter,tp,LOCATION_MZONE,0,1,nil)
    //   and Duel.SelectEffectYesNo(tp,e:GetHandler()) then
    if ct > 0 && duel::is_existing_matching_card(d, Some(&atkfilter), tp, LOCATION_MZONE, 0, 1, NO_EX) {
        let c = effect::get_handler(d, a.e).nil();
        if duel::select_effect_yes_no(d, tp, c, None) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
            // local tc=Duel.SelectMatchingCard(tp,aux.FaceupFilter(Card.IsSetCard,SET_DRAGUNITY),tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
            let sel = duel::select_matching_card(d, tp, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_DRAGUNITY as u16]))), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
            let tc = group::get_first(d, &sel).nil();
            // Duel.HintSelection(Group.FromCards(tc))
            duel::hint_selection(d, CardSet::from_card(tc), true);
            // Increase ATK
            // local e1=Effect.CreateEffect(e:GetHandler())
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // e1:SetValue(ct*100)
            effect::set_value(d, e1, ct * 100);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
