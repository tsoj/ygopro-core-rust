//! Performapal Kuribohble
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 69181753;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`: `return Duel.GetAttacker():GetControler()~=tp and Duel.GetAttackTarget()==nil`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let attacker = duel::get_attacker(d).nil();
    card::get_controler(d, attacker) != a.tp && duel::get_attack_target(d).is_none()
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) != 0 {
        // local a=Duel.GetAttacker()
        let at = duel::get_attacker(d).nil();
        // if a:CanAttack() and not a:IsImmuneToEffect(e) then
        if card::can_attack(d, at) && !card::is_immune_to_effect(d, at, a.e) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            // e1:SetCode(EFFECT_REVERSE_DAMAGE)
            effect::set_code(d, e1, EFFECT_REVERSE_DAMAGE);
            // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
            effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
            // e1:SetRange(LOCATION_MZONE)
            effect::set_range(d, e1, LOCATION_MZONE);
            // e1:SetTargetRange(1,0)
            effect::set_target_range(d, e1, 1, 0);
            // e1:SetValue(1)
            effect::set_value(d, e1, 1);
            // e1:SetReset(RESET_PHASE|PHASE_DAMAGE)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
            // Duel.CalculateDamage(a,c)
            duel::calculate_damage(d, at, Some(c), false);
        }
    }
}
