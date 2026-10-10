//! Aromage Bergamot
use super::prelude::*;

pub const ID: u32 = 85967160;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // While your LP is higher than your opponent's, if a Plant-Type monster you control attacks a Defense Position monster, inflict piercing battle damage to your opponent
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e1, EFFECT_PIERCE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetCondition(s.pccon)
    effect::set_condition(d, e1, Some(condition(pccon)));
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsRace,RACE_PLANT))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_PLANT))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Once per turn, if you gain LP: This card gains 1000 ATK and DEF until the end of your opponent's turn.
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_RECOVER)
    effect::set_code(d, e2, EVENT_RECOVER);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return ep==tp end)
    effect::set_condition(d, e2, Some(condition(|_d, a| a.ep == a.tp)));
    // e2:SetTarget(s.adtg)
    effect::set_target(d, e2, Some(cost_target(adtg)));
    // e2:SetOperation(s.adop)
    effect::set_operation(d, e2, Some(operation(adop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.pccon(e)`
fn pccon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e);
    // return Duel.GetLP(tp)>Duel.GetLP(1-tp)
    duel::get_lp(d, tp) > duel::get_lp(d, 1 - tp)
}
/// `s.adtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn adtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return ep==tp and e:GetHandler():IsRelateToEffect(e) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return a.ep == a.tp && card::is_relate_to_effect(d, h, a.e);
    }
    true
}
/// `s.adop(e,tp,eg,ep,ev,re,r,rp)`
fn adop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(1000)
        effect::set_value(d, e1, 1000);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END|RESET_OPPO_TURN)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END | RESET_OPPO_TURN, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_UPDATE_DEFENSE)
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
}
