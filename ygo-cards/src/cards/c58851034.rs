//! Cursed Seal of the Forbidden Spell
use super::prelude::*;

pub const ID: u32 = 58851034;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:IsSpellEffect() and re:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.IsChainNegatable(ev)
    let re = a.re.nil();
    effect::is_spell_effect(d, re) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpell() and c:IsDiscardable()
    card::is_spell(d, c) && card::is_discardable(d, c, None, None)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if Duel.IsPlayerAffectedByEffect(tp,EFFECT_DISCARD_COST_CHANGE) then return true end
    if duel::is_player_affected_by_effect(d, a.tp, EFFECT_DISCARD_COST_CHANGE).is_some() {
        return true;
    }
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,s.cfilter,1,1,REASON_COST|REASON_DISCARD,nil)
    duel::discard_hand(d, a.tp, Some(&|d, c| cfilter(d, c)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsDestructable() and re:GetHandler():IsRelateToEffect(re) then
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) and Duel.Destroy(eg,REASON_EFFECT)~=0 then
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let rc = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rc, re) && duel::destroy(d, a.eg(), REASON_EFFECT, None, None) != 0 {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
            effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
            // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
            effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
            // e1:SetTargetRange(0,1)
            effect::set_target_range(d, e1, 0, 1);
            // e1:SetValue(s.aclimit)
            effect::set_value_fn(d, e1, lua_fn(aclimit));
            // e1:SetLabel(re:GetHandler():GetCode())
            let codes: Vec<i64> = card::get_code(d, rc).into_iter().map(|x| x as i64).collect();
            effect::set_label(d, e1, &codes);
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, a.tp);
        }
    }
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    let (e, re) = (a.e(0), a.e(1));
    // return re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:GetHandler():IsCode(e:GetLabel())
    if !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        return false;
    }
    let h = effect::get_handler(d, re).nil();
    let codes: Vec<u32> = effect::get_labels(d, e).into_iter().map(|x| x as u32).collect();
    card::is_code(d, h, &codes)
}
