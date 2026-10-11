//! Meklord Army of Wisel
use super::prelude::*;

pub const ID: u32 = 39648965;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MEKLORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atkup
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(s.val)
    effect::set_value_fn(d, e1, effect_card_value(val));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // pierce
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_TRIGGER_O+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_FIELD);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.piercecon)
    effect::set_condition(d, e2, Some(condition(piercecon)));
    // e2:SetTarget(s.piercetg)
    effect::set_target(d, e2, Some(cost_target(piercetg)));
    // e2:SetOperation(s.pierceop)
    effect::set_operation(d, e2, Some(operation(pierceop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.atkfilter(c)
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_MEKLORD)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MEKLORD as u16])
}
// function s.val(e,c)
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(s.atkfilter,0,LOCATION_MZONE,LOCATION_MZONE,c)*100
    duel::get_matching_group_count(d, Some(&atkfilter), 0, LOCATION_MZONE, LOCATION_MZONE, c.into()) as i64 * 100
}
// function s.piercecon(e,tp,eg,ep,ev,re,r,rp)
fn piercecon(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let dt = duel::get_attack_target(d);
    // return d and a:IsControler(tp) and a~=e:GetHandler() and d:IsDefensePos() and a:IsSetCard(SET_MEKLORD)
    let Some(dt) = dt else {
        return false;
    };
    let att = att.nil();
    card::is_controler(d, att, a.tp)
        && Some(att) != effect::get_handler(d, a.e)
        && card::is_defense_pos(d, dt)
        && card::is_set_card(d, att, &[SET_MEKLORD as u16])
}
// function s.piercetg(e,tp,eg,ep,ev,re,r,rp,chk)
fn piercetg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.GetAttacker():CreateEffectRelation(e)
    let att = duel::get_attacker(d).nil();
    card::create_effect_relation(d, att, a.e);
    true
}
// function s.pierceop(e,tp,eg,ep,ev,re,r,rp)
fn pierceop(d: &mut Duel, a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d).nil();
    // if a:IsRelateToEffect(e) and a:IsFaceup() then
    if card::is_relate_to_effect(d, att, a.e) && card::is_faceup(d, att) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_PIERCE)
        effect::set_code(d, e1, EFFECT_PIERCE);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // a:RegisterEffect(e1)
        card::register_effect(d, att, e1, false);
    }
}
