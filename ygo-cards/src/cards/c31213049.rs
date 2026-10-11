//! XX-clusion
use super::prelude::*;

pub const ID: u32 = 31213049;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate activation
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.negcon)
    effect::set_condition(d, e1, Some(condition(negcon)));
    // e1:SetTarget(s.negtg)
    effect::set_target(d, e1, Some(cost_target(negtg)));
    // e1:SetOperation(s.negop)
    effect::set_operation(d, e1, Some(operation(negop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and re:GetActivateLocation()==LOCATION_HAND
    //     and re:IsMonsterEffect() and Duel.IsChainNegatable(ev)
    let re = a.re.nil();
    a.rp == 1 - a.tp
        && effect::get_activate_location(d, re) == LOCATION_HAND
        && effect::is_monster_effect(d, re)
        && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.NegateActivation(ev) and Duel.GetFieldGroupCount(1-tp,LOCATION_HAND,0)>0 then
    if duel::negate_activation(d, a.ev as u8) && duel::get_field_group_count(d, 1 - tp, LOCATION_HAND, 0) > 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
        // Duel.DiscardHand(1-tp,nil,1,1,REASON_EFFECT|REASON_DISCARD,nil)
        duel::discard_hand(d, 1 - tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
    }
}
