//! Lancer Archfiend
use super::prelude::*;

pub const ID: u32 = 79418153;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Make a monster inflict piercing damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_TRIGGER_O+EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_FIELD);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.piercecon)
    effect::set_condition(d, e1, Some(condition(piercecon)));
    // e1:SetTarget(s.piercetg)
    effect::set_target(d, e1, Some(cost_target(piercetg)));
    // e1:SetOperation(s.pierceop)
    effect::set_operation(d, e1, Some(operation(pierceop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.piercecon(e,tp,eg,ep,ev,re,r,rp)`
fn piercecon(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let at = duel::get_attacker(d).nil();
    // local d=Duel.GetAttackTarget()
    let dt = duel::get_attack_target(d);
    // return d and a:IsControler(tp) and d:IsDefensePos()
    match dt {
        None => false,
        Some(dt) => card::is_controler(d, at, a.tp) && card::is_defense_pos(d, dt),
    }
}
/// `s.piercetg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn piercetg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.GetAttacker():CreateEffectRelation(e)
    let at = duel::get_attacker(d).nil();
    card::create_effect_relation(d, at, a.e);
    true
}
/// `s.pierceop(e,tp,eg,ep,ev,re,r,rp)`
fn pierceop(d: &mut Duel, a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let at = duel::get_attacker(d).nil();
    // if a:IsRelateToEffect(e) and a:IsFaceup() then
    if card::is_relate_to_effect(d, at, a.e) && card::is_faceup(d, at) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_PIERCE)
        effect::set_code(d, e1, EFFECT_PIERCE);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
        // a:RegisterEffect(e1)
        card::register_effect(d, at, e1, false);
    }
}
