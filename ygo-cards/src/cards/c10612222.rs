//! Yosenjus' Sword Sting
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 10612222;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_YOSENJU],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_END_PHASE));
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0
    duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) == 0
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_YOSENJU) and c:IsMonster() and not c:IsPublic()
    card::is_set_card(d, c, &[SET_YOSENJU as u16]) && card::is_monster(d, c) && !card::is_public(d, c)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // return true
    true
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetCode)==#sg and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsAbleToHand),tp,0,LOCATION_ONFIELD,#sg,nil)
    let same_class = group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64) == sg.len();
    let res = same_class
        && duel::is_existing_target(
            d,
            Some(&|d, c| card::is_faceup(d, c) && card::is_able_to_hand(d, c, None)),
            tp,
            0,
            LOCATION_ONFIELD,
            sg.len() as u16,
            NO_EX,
        );
    (res, false)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsOnField() and aux.FaceupFilter(Card.IsAbleToHand)(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_on_field(d, chkc) && card::is_faceup(d, chkc) && card::is_able_to_hand(d, chkc, None);
    }
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_HAND,0,nil)
    let g = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_HAND, 0, NO_EX);
    // if chk==0 then
    if a.chk == Some(0) {
        // return aux.SelectUnselectGroup(g,e,tp,1,2,s.rescon,0)
        return matches!(
            aux::select_unselect_group(d, &g, Some(a.e), tp, Some(1), Some(2), Some(&rescon), 0, tp, None, None, None, false),
            Err(true)
        );
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,2,s.rescon,1,tp,HINTMSG_CONFIRM)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(1), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_CONFIRM as u64), None, None, false)
        .unwrap_or_default();
    // Duel.ConfirmCards(1-tp,sg)
    duel::confirm_cards(d, 1 - tp, sg.clone());
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local rg=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsAbleToHand),tp,0,LOCATION_ONFIELD,#sg,#sg,nil)
    let n = sg.len() as u16;
    let rg = duel::select_target(
        d,
        tp,
        Some(&|d, c| card::is_faceup(d, c) && card::is_able_to_hand(d, c, None)),
        tp,
        0,
        LOCATION_ONFIELD,
        n,
        n,
        false,
        NO_EX,
    );
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,rg,#rg,0,0)
    let rn = rg.as_ref().map_or(0, |g| g.len()) as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, rg, rn, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
