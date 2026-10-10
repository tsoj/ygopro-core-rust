//! Mermail Abyssgaios
use super::prelude::*;

pub const ID: u32 = 74371660;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_WATER),7,2)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_WATER, sc, st, Some(tp)))),
        Some(7),
        2,
        None,
        None,
        None,
        None,
        false,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // atklimit
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsLevelAbove,5))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_level_above(d, c, 5))));
    // e1:SetCondition(s.dscon)
    effect::set_condition(d, e1, Some(condition(dscon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // negate
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER));
    // e2:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.dscon(e)`
fn dscon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetOverlayCount()~=0
    let h = effect::get_handler(d, a.e).nil();
    card::get_overlay_count(d, h) != 0
}
/// `s.filter(c,atk)`
fn filter(d: &mut Duel, c: CardId, atk: i64) -> bool {
    // return c:IsNegatableMonster() and c:GetAttack()<atk
    card::is_negatable_monster(d, c) && (card::get_attack(d, c) as i64) < atk
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,0,LOCATION_MZONE,1,nil,e:GetHandler():GetAttack()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        let atk = card::get_attack(d, h) as i64;
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, atk)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_MZONE,nil,c:GetAttack())
    let atk = card::get_attack(d, c) as i64;
    let g = duel::get_matching_group(d, Some(&|d, x| filter(d, x, atk)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // local tc=g:GetFirst()
    let _ = group::get_first(d, &g);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        // e2:SetValue(RESET_TURN_SET)
        effect::set_value(d, e2, RESET_TURN_SET as i64);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // Duel.NegateRelatedChain(tc,RESET_TURN_SET)
        duel::negate_related_chain(d, tc, RESET_TURN_SET);
    }
}
