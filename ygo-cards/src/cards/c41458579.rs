//! Musakani Magatama
use super::prelude::*;

pub const ID: u32 = 41458579;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SIX_SAMURAI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_SIX_SAMURAI)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SIX_SAMURAI as u16])
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // if not Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE,0,1,nil) then return false end
    if !duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c)), a.tp, LOCATION_MZONE, 0, 1, NO_EX) {
        return false;
    }
    // if tp==ep or not Duel.IsChainNegatable(ev) then return false end
    if a.tp == a.ep || !duel::is_chain_negatable(d, a.ev as u8) {
        return false;
    }
    // if not re:IsMonsterEffect() and not re:IsHasType(EFFECT_TYPE_ACTIVATE) then return false end
    let re = a.re.nil();
    if !effect::is_monster_effect(d, re) && !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        return false;
    }
    // local ex,tg,tc=Duel.GetOperationInfo(ev,CATEGORY_DESTROY)
    // return ex and tg~=nil and tc>0
    match duel::get_operation_info(d, a.ev, CATEGORY_DESTROY) {
        Some(info) => info.op_cards.is_some() && info.op_count > 0,
        None => false,
    }
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
    let h = effect::get_handler(d, a.re.nil()).nil();
    if card::is_destructable(d, h, None) && card::is_relate_to_effect(d, h, a.re.nil()) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let h = effect::get_handler(d, a.re.nil()).nil();
        if card::is_relate_to_effect(d, h, a.re.nil()) {
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
