//! Medicurius the Power Patron of Illusions
use super::prelude::*;

pub const ID: u32 = 4063756;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,nil,2,3,s.linkmatcheck)
    link::add_procedure(
        d,
        c,
        None,
        2,
        Some(3),
        Some(std::sync::Arc::new(|d: &mut Duel, g: &CardSet, lc: CardId, st: u64, tp: u8| linkmatcheck(d, g, lc, st, tp))),
        None,
        None,
    );
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_DISABLE + CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.commoncondition(1))
    effect::set_condition(d, e1, Some(condition(|d, a| commoncondition(d, a, 1))));
    // e1:SetTarget(s.disableatktg)
    effect::set_target(d, e1, Some(cost_target(disableatktg)));
    // e1:SetOperation(s.disableatkop)
    effect::set_operation(d, e1, Some(operation(disableatkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.commoncondition(2))
    effect::set_condition(d, e2, Some(condition(|d, a| commoncondition(d, a, 2))));
    // e2:SetValue(function(e,te) return te:GetOwnerPlayer()==1-e:GetHandlerPlayer() and te:IsActivated() end)
    effect::set_value_fn(d, e2, lua_fn(immune_value));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_REMOVE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) and e:GetHandler():GetLinkedGroupCount()==3 end)
    effect::set_condition(d, e3, Some(condition(e3_cond)));
    // e3:SetTarget(s.bantg)
    effect::set_target(d, e3, Some(cost_target(bantg)));
    // e3:SetOperation(s.banop)
    effect::set_operation(d, e3, Some(operation(banop)));
    // e3:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some((TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.linkmatcheck(g,lc,sumtype,tp)`
fn linkmatcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsType,1,nil,TYPE_EXTRA,lc,sumtype,tp)
    group::is_exists(d, g, &|d, c| card::is_type_ctx(d, c, TYPE_EXTRA, Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
/// `s.commoncondition(minimum_number)`: `e:GetHandler():GetLinkedGroupCount()>=minimum_number`
fn commoncondition(d: &mut Duel, a: &EffArgs, minimum_number: usize) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_linked_group_count(d, h) >= minimum_number
}
/// `s.disableatktg(e,tp,...,chk)`
fn disableatktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(g.clone()), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,g,#g,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(g.clone()), g.len() as u8, tp, 0);
    true
}
/// `s.disableatkop(e,tp,...)`
fn disableatkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // tc:NegateEffects(c)
        card::negate_effects(d, tc, c, None, false, None);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(tc:GetAttack()/2)
        let half = (card::get_attack(d, tc) as f64 / 2.0).round() as i64;
        effect::set_value(d, e1, half);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// e2's value: `function(e,te) return te:GetOwnerPlayer()==1-e:GetHandlerPlayer() and te:IsActivated() end`
fn immune_value(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let te = a.e(1);
    effect::get_owner_player(d, te) == 1 - effect::get_handler_player(d, e) && effect::is_activated(d, te)
}
/// e3's condition: `function(e,tp) return Duel.IsTurnPlayer(1-tp) and e:GetHandler():GetLinkedGroupCount()==3 end`
fn e3_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    duel::is_turn_player(d, 1 - a.tp) && card::get_linked_group_count(d, h) == 3
}
/// `s.bantg(e,tp,...,chk)`
fn bantg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemove() end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, c, None, None, None);
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,nil)
    let mut g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_ONFIELD, NO_EX);
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g.clone()), g.len() as u8, tp, 0);
    true
}
/// `s.banop(e,tp,...)`
fn banop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,nil)
    let mut g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_ONFIELD, NO_EX);
    // if c:IsRelateToEffect(e) and c:IsAbleToRemove() then g:AddCard(c) end
    if card::is_relate_to_effect(d, c, a.e) && card::is_able_to_remove(d, c, None, None, None) {
        group::add_card(&mut g, &CardSet::from_card(c));
    }
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
