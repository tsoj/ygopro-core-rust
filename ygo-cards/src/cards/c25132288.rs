//! Light End Dragon
use super::prelude::*;

pub const ID: u32 = 25132288;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTunerEx(Card.IsAttribute,ATTRIBUTE_LIGHT),1,99)
    synchro::add_procedure(
        d,
        c,
        None,
        1,
        1,
        Some(synchro::non_tuner_ex(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT, sc, st, Some(tp)))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // addown
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
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
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=c:GetBattleTarget()
    let tc = card::get_battle_target(d, c);
    // e:SetLabelObject(tc)
    effect::set_label_object(d, e, tc.map_or(Value::Nil, Value::Card));
    // return tc and c:GetAttack()>=500 and c:GetDefense()>=500 and tc:IsFaceup() and (tc:GetAttack()>0 or tc:GetDefense()>0)
    let Some(tc) = tc else {
        return false;
    };
    card::get_attack(d, c) >= 500
        && card::get_defense(d, c) >= 500
        && card::is_faceup(d, tc)
        && (card::get_attack(d, tc) > 0 || card::get_defense(d, tc) > 0)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // e:GetLabelObject():CreateEffectRelation(e)
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    card::create_effect_relation(d, tc, a.e);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, e);
    // if c:IsRelateToEffect(e) and c:IsFaceup() and c:GetAttack()>=500 and c:GetDefense()>=500 then
    if card::is_relate_to_effect(d, c, e) && card::is_faceup(d, c) && card::get_attack(d, c) >= 500 && card::get_defense(d, c) >= 500 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_COPY_INHERIT)
        effect::set_property(d, e1, EFFECT_FLAG_COPY_INHERIT, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(-500)
        effect::set_value(d, e1, -500);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_UPDATE_DEFENSE)
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
        // if tc:IsRelateToBattle() and tc:IsRelateToEffect(e) and tc:IsFaceup()
        //     and not c:IsHasEffect(EFFECT_REVERSE_UPDATE) then
        let tc = tc.as_card().nil();
        if card::is_relate_to_battle(d, tc)
            && card::is_relate_to_effect(d, tc, e)
            && card::is_faceup(d, tc)
            && card::is_has_effect(d, c, EFFECT_REVERSE_UPDATE, None).is_empty()
        {
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            // e3:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
            // e3:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
            // e3:SetValue(-1500)
            effect::set_value(d, e3, -1500);
            // tc:RegisterEffect(e3)
            card::register_effect(d, tc, e3, false);
            // local e4=e3:Clone()
            let e4 = effect::clone(d, e3);
            // e4:SetCode(EFFECT_UPDATE_DEFENSE)
            effect::set_code(d, e4, EFFECT_UPDATE_DEFENSE);
            // tc:RegisterEffect(e4)
            card::register_effect(d, tc, e4, false);
        }
    }
}
