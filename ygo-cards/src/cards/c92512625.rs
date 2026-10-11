//! Solemn Scolding
use super::prelude::*;

pub const ID: u32 = 92512625;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate(summon)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DISABLE_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DISABLE_SUMMON + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_SUMMON)
    effect::set_code(d, e1, EVENT_SUMMON);
    // e1:SetCondition(s.condition1)
    effect::set_condition(d, e1, Some(condition(condition1)));
    // e1:SetCost(Cost.PayLP(3000))
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(3000))));
    // e1:SetTarget(s.target1)
    effect::set_target(d, e1, Some(cost_target(target1)));
    // e1:SetOperation(s.activate1)
    effect::set_operation(d, e1, Some(operation(activate1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_FLIP_SUMMON)
    effect::set_code(d, e2, EVENT_FLIP_SUMMON);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EVENT_SPSUMMON)
    effect::set_code(d, e3, EVENT_SPSUMMON);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Activate(effect)
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e4, EFFECT_TYPE_ACTIVATE);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetCondition(s.condition2)
    effect::set_condition(d, e4, Some(condition(condition2)));
    // e4:SetCost(Cost.PayLP(3000))
    effect::set_cost(d, e4, Some(cost::pay_lp(cost::PayLp::Amount(3000))));
    // e4:SetTarget(s.target2)
    effect::set_target(d, e4, Some(cost_target(target2)));
    // e4:SetOperation(s.activate2)
    effect::set_operation(d, e4, Some(operation(activate2)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() and c:GetSequence()<5
    card::is_facedown(d, c) && card::get_sequence(d, c) < 5
}
/// `s.condition1(e,tp,eg,ep,ev,re,r,rp)`
fn condition1(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetCurrentChain(true)==0
    //     and not Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_SZONE,0,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    duel::get_current_chain(d, true) == 0
        && !duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_SZONE, 0, 1, h.into())
}
/// `s.target1(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE_SUMMON,eg,#eg,0,0)
    let eg = a.eg.clone().nil();
    duel::set_operation_info(d, 0, CATEGORY_DISABLE_SUMMON, Some(eg.clone()), eg.len() as u8, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,#eg,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(eg.clone()), eg.len() as u8, 0, 0);
    true
}
/// `s.activate1(e,tp,eg,ep,ev,re,r,rp)`
fn activate1(d: &mut Duel, a: &EffArgs) {
    // Duel.NegateSummon(eg)
    duel::negate_summon(d, a.eg());
    // Duel.Destroy(eg,REASON_EFFECT)
    duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
}
/// `s.condition2(e,tp,eg,ep,ev,re,r,rp)`
fn condition2(d: &mut Duel, a: &EffArgs) -> bool {
    // return (re:IsMonsterEffect() or re:IsHasType(EFFECT_TYPE_ACTIVATE)) and Duel.IsChainNegatable(ev)
    //     and not Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_SZONE,0,1,e:GetHandler())
    let re_ok = effect::is_monster_effect(d, a.re.nil()) || effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE);
    if !re_ok {
        return false;
    }
    if !duel::is_chain_negatable(d, a.ev as u8) {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    !duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_SZONE, 0, 1, h.into())
}
/// `s.target2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsDestructable() and re:GetHandler():IsRelateToEffect(re) then
    let rh = effect::get_handler(d, a.re.nil()).nil();
    if card::is_destructable(d, rh, None) && card::is_relate_to_effect(d, rh, a.re.nil()) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.activate2(e,tp,eg,ep,ev,re,r,rp)`
fn activate2(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let rh = effect::get_handler(d, a.re.nil()).nil();
        if card::is_relate_to_effect(d, rh, a.re.nil()) {
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
