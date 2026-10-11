//! Toon Terror
use super::prelude::*;

pub const ID: u32 = 53094821;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[15259703], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate the activation of a monster effect or spell/trap card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY);
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
    // return c:IsFaceup() and c:IsCode(15259703)
    card::is_faceup(d, c) && card::is_code(d, c, &[15259703])
}
/// `s.cfilter2(c)`
fn cfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_TOON)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_TOON)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if not Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_ONFIELD,0,1,nil)
    //   or not Duel.IsExistingMatchingCard(s.cfilter2,tp,LOCATION_MZONE,0,1,nil)
    //   or not Duel.IsChainNegatable(ev) then return false end
    if !duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        || !duel::is_existing_matching_card(d, Some(&cfilter2), tp, LOCATION_MZONE, 0, 1, NO_EX)
        || !duel::is_chain_negatable(d, a.ev as u8)
    {
        return false;
    }
    // return re:IsMonsterEffect() or re:IsHasType(EFFECT_TYPE_ACTIVATE)
    let re = a.re.nil();
    effect::is_monster_effect(d, re) || effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
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
    let rh = effect::get_handler(d, re).nil();
    if card::is_destructable(d, rh, None) && card::is_relate_to_effect(d, rh, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let rh = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rh, re) {
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg.clone().nil(), REASON_EFFECT, None, None);
        }
    }
}
