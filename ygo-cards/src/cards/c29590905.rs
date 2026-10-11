//! Super Junior Confrontation
use super::prelude::*;

pub const ID: u32 = 29590905;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:GetFirst():IsControler(1-tp)
    let eg = a.eg.clone().nil();
    let first = group::get_first(d, &eg).nil();
    card::is_controler(d, first, 1 - a.tp)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsPosition,tp,LOCATION_MZONE,0,1,nil,POS_FACEUP_DEFENSE) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_position(d, c, POS_FACEUP_DEFENSE)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g1=Duel.GetMatchingGroup(Card.IsPosition,tp,0,LOCATION_MZONE,nil,POS_FACEUP_ATTACK)
    let g1 = duel::get_matching_group(d, Some(&|d, c| card::is_position(d, c, POS_FACEUP_ATTACK)), tp, 0, LOCATION_MZONE, NO_EX);
    // local g2=Duel.GetMatchingGroup(Card.IsPosition,tp,LOCATION_MZONE,0,nil,POS_FACEUP_DEFENSE)
    let g2 = duel::get_matching_group(d, Some(&|d, c| card::is_position(d, c, POS_FACEUP_DEFENSE)), tp, LOCATION_MZONE, 0, NO_EX);
    // if #g1==0 or #g2==0 then return end
    if g1.is_empty() || g2.is_empty() {
        return;
    }
    // local ga=g1:GetMinGroup(Card.GetAttack)
    let mut ga = group::get_min_group(d, &g1, &|d, c| card::get_attack(d, c) as i64).map(|x| x.0).unwrap_or_default();
    // local gd=g2:GetMinGroup(Card.GetDefense)
    let mut gd = group::get_min_group(d, &g2, &|d, c| card::get_defense(d, c) as i64).map(|x| x.0).unwrap_or_default();
    // if #ga>1 then
    if ga.len() > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 0));
        // ga=ga:Select(tp,1,1,nil)
        ga = group::select(d, &ga, tp, 1, 1, false, NO_EX).nil();
    }
    // if #gd>1 then
    if gd.len() > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
        // gd=gd:Select(tp,1,1,nil)
        gd = group::select(d, &gd, tp, 1, 1, false, NO_EX).nil();
    }
    // Duel.NegateAttack()
    duel::negate_attack(d);
    // local a=ga:GetFirst()
    let a_card = group::get_first(d, &ga);
    // local d=gd:GetFirst()
    let d_card = group::get_first(d, &gd);
    // if a:CanAttack() and not a:IsImmuneToEffect(e) and not d:IsImmuneToEffect(e) then
    if card::can_attack(d, a_card.nil()) && !card::is_immune_to_effect(d, a_card.nil(), a.e) && !card::is_immune_to_effect(d, d_card.nil(), a.e) {
        let a_card = a_card.nil();
        // Duel.CalculateDamage(a,d)
        duel::calculate_damage(d, a_card, Some(d_card.nil()), false);
        // Duel.SkipPhase(1-tp,PHASE_BATTLE,RESET_PHASE|PHASE_BATTLE_STEP,1)
        duel::skip_phase(d, 1 - tp, PHASE_BATTLE, RESET_PHASE | PHASE_BATTLE_STEP, 1, 0);
    }
}
