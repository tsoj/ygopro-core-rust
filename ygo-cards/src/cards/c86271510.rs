//! Darton the Mechanical Monstrosity
use super::prelude::*;

pub const ID: u32 = 86271510;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Change original ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(cost_target(atktg)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.atkfilter(c)
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and not c:IsAttack(c:GetBaseAttack())
    let base = card::get_base_attack(d, c) as u32;
    card::is_faceup(d, c) && !card::is_attack(d, c, &[base])
}
// function s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.atkfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&atkfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    true
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsFacedown() or not c:IsRelateToEffect(e) then return end
    if card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.atkfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&atkfilter), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g<1 then return end
    if g.is_empty() {
        return;
    }
    // local sum=0
    let mut sum: i64 = 0;
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // sum=sum+(math.abs(tc:GetBaseAttack()-tc:GetAttack()))
        sum += (card::get_base_attack(d, tc) as i64 - card::get_attack(d, tc) as i64).abs();
    }
    // --Change original ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_BASE_ATTACK)
    effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
    // e1:SetValue(sum)
    effect::set_value(d, e1, sum);
    // e1:SetReset(RESETS_STANDARD_PHASE_END|RESET_OPPO_TURN,1)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END | RESET_OPPO_TURN, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
