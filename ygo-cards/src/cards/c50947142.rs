//! Scrap-Iron Signal
use super::prelude::*;

pub const ID: u32 = 50947142;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.negcon)
    effect::set_condition(d, e1, Some(condition(negcon)));
    // e1:SetTarget(s.negtg)
    effect::set_target(d, e1, Some(cost_target(negtg)));
    // e1:SetOperation(s.negop)
    effect::set_operation(d, e1, Some(operation(negop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `function s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // if not (c:IsFaceup() and c:IsType(TYPE_SYNCHRO)) then return false end
    if !(card::is_faceup(d, c) && card::is_type(d, c, TYPE_SYNCHRO)) {
        return false;
    }
    // local mt=c:GetMetatable()
    let mt = crate::card_info(d, c);
    // local ct=0
    let mut ct = 0;
    // if mt.synchro_tuner_required then ct=ct+mt.synchro_tuner_required end
    if let Some(x) = mt.synchro_tuner_required {
        ct += x;
    }
    // if mt.synchro_nt_required then ct=ct+mt.synchro_nt_required end
    if let Some(x) = mt.synchro_nt_required {
        ct += x;
    }
    // return ct>0
    ct > 0
}
/// `function(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp and re:IsMonsterEffect() and Duel.IsChainNegatable(ev)
    //     and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    a.ep == 1 - a.tp
        && effect::is_monster_effect(d, a.re.nil())
        && duel::is_chain_negatable(d, a.ev as u8)
        && duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    true
}
/// `function(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.NegateActivation(ev)
    duel::negate_activation(d, a.ev as u8);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if c:IsSSetable(true) and e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if card::is_s_setable(d, c, true, None) && effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // c:CancelToGrave()
        card::cancel_to_grave(d, c, true);
        // Duel.ChangePosition(c,POS_FACEDOWN)
        duel::change_position(d, c, POS_FACEDOWN as u8, None, None, None, false);
        // Duel.RaiseEvent(c,EVENT_SSET,e,REASON_EFFECT,tp,tp,0)
        duel::raise_event(d, c, EVENT_SSET, Some(a.e), REASON_EFFECT, tp, tp, 0);
    }
}
