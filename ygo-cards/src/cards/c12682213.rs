//! Iron Thunder
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 12682213;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate an activation, destroy the card and all cards in the same column
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
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
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
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // local loc=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)
    let loc = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int();
    // return loc&LOCATION_ONFIELD>0 and (re:IsMonsterEffect() or re:IsHasType(EFFECT_TYPE_ACTIVATE))
    //   and Duel.IsChainNegatable(ev)
    if (loc as u32 & LOCATION_ONFIELD) == 0 {
        return false;
    }
    let re = a.re.nil();
    (effect::is_monster_effect(d, re) || effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)) && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.PayLPCost(tp,Duel.GetLP(tp)//2)
    let half = duel::get_lp(d, a.tp).div_euclid(2) as u32;
    duel::pay_lp_cost(d, a.tp, half);
    true
}
/// `local exc=(e:IsHasType(EFFECT_TYPE_ACTIVATE) and c:IsRelateToEffect(e)) and c or nil`
fn exc_card(d: &mut Duel, e: EffectId) -> Option<CardId> {
    let c = effect::get_handler(d, e).nil();
    if effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) && card::is_relate_to_effect(d, c, e) {
        Some(c)
    } else {
        None
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
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if rc:IsDestructable() and rc:IsRelateToEffect(re) then
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, re) {
        // local exc=(e:IsHasType(EFFECT_TYPE_ACTIVATE) and c:IsRelateToEffect(e)) and c or nil
        let exc = exc_card(d, a.e);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg+rc:GetColumnGroup():RemoveCard(exc),1,0,0)
        let mut colg = card::get_column_group(d, rc, 0, 0);
        group::remove_card(&mut colg, &CardSet::from_card(exc.nil()));
        let mut tg = a.eg.clone().nil();
        group::add_card(&mut tg, &colg);
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(tg), 1, 0, 0);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if not (Duel.NegateActivation(ev) and rc:IsRelateToEffect(re)) then return end
    if !(duel::negate_activation(d, a.ev as u8) && card::is_relate_to_effect(d, rc, re)) {
        return;
    }
    // local exc=(e:IsHasType(EFFECT_TYPE_ACTIVATE) and c:IsRelateToEffect(e)) and c or nil
    let exc = exc_card(d, a.e);
    // local colg=rc:GetColumnGroup()
    let mut colg = card::get_column_group(d, rc, 0, 0);
    // if exc then colg:RemoveCard(exc) end
    if let Some(exc) = exc {
        group::remove_card(&mut colg, &CardSet::from_card(exc));
    }
    // if Duel.Destroy(eg,REASON_EFFECT)>0 and #colg>0 then
    let eg = a.eg.clone().nil();
    if duel::destroy(d, eg, REASON_EFFECT, None, None) > 0 && !colg.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Destroy(colg,REASON_EFFECT)
        duel::destroy(d, colg, REASON_EFFECT, None, None);
    }
}
