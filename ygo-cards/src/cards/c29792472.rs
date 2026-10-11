//! Yad'al-Hecahands
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 29792472;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_HECAHANDS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When your opponent activates a Spell/Trap Card, while you control a "Hecahands" monster: Negate the activation, and if you do, destroy that card, then you can Set the destroyed Spell/Trap to your field. You can only activate 1 "Yad'al-Hecahands" per turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY + CATEGORY_SET);
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
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and re:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_HECAHANDS),tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsChainNegatable(ev)
    a.rp == 1 - a.tp
        && effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE)
        && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_HECAHANDS as u16]))), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, a.tp, 0);
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil()).nil();
    // if rc:IsRelateToEffect(re) and rc:IsDestructable() then
    if card::is_relate_to_effect(d, rc, a.re.nil()) && card::is_destructable(d, rc, None) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, a.tp, 0);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_SET,rc,1,tp,0)
        duel::set_possible_operation_info(d, 0, CATEGORY_SET, Some(CardSet::from_card(rc)), 1, a.tp, 0);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil()).nil();
    // if Duel.NegateActivation(ev) and rc:IsRelateToEffect(re) and Duel.Destroy(eg,REASON_EFFECT)>0 and not rc:IsLocation(LOCATION_HAND|LOCATION_DECK)
    //     and aux.nvfilter(rc) and rc:IsSSetable() and rc:IsSpellTrap() and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if duel::negate_activation(d, a.ev as u8)
        && card::is_relate_to_effect(d, rc, a.re.nil())
        && duel::destroy(d, a.eg(), REASON_EFFECT, None, None) > 0
        && !card::is_location(d, rc, LOCATION_HAND | LOCATION_DECK)
        && aux::nvfilter(d, rc)
        && card::is_s_setable(d, rc, false, None)
        && card::is_spell_trap(d, rc)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SSet(tp,rc)
        duel::s_set(d, a.tp, CardSet::from_card(rc), None, true);
    }
}
