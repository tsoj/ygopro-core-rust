//! Vampire Domination
use super::prelude::*;

pub const ID: u32 = 68688135;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VAMPIRE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY+CATEGORY_RECOVER)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY + CATEGORY_RECOVER);
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
    // return c:IsFaceup() and c:IsSetCard(SET_VAMPIRE)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_VAMPIRE as u16])
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    //     and (re:IsMonsterEffect() or re:IsHasType(EFFECT_TYPE_ACTIVATE)) and Duel.IsChainNegatable(ev)
    let re = a.re.nil();
    duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
        && (effect::is_monster_effect(d, re) || effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE))
        && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    // if re:GetHandler():IsDestructable() and re:GetHandler():IsRelateToEffect(re) then
    if card::is_destructable(d, rh, None) && card::is_relate_to_effect(d, rh, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
        // if re:GetHandler():IsMonster() then
        if card::is_monster(d, rh) {
            // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,re:GetHandler():GetBaseAttack())
            let atk = card::get_base_attack(d, rh);
            duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, atk);
        }
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re)
    //     and Duel.Destroy(eg,REASON_EFFECT)~=0 and re:GetHandler():IsMonster() and re:GetHandler():GetBaseAttack()>0 then
    if duel::negate_activation(d, a.ev as u8)
        && card::is_relate_to_effect(d, rh, re)
        && duel::destroy(d, a.eg.clone().nil(), REASON_EFFECT, None, None) != 0
        && card::is_monster(d, rh)
        && card::get_base_attack(d, rh) > 0
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Recover(tp,re:GetHandler():GetBaseAttack(),REASON_EFFECT)
        let atk = card::get_base_attack(d, rh);
        duel::recover(d, a.tp, atk as i64, REASON_EFFECT, false, None);
    }
}
