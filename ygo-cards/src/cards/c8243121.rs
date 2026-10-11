//! Lyrilusc - Phantom Feathers
//! ＬＬ－比翼の麗鳥
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 8243121;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LYRILUSC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // change stats
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_LVCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_LVCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // e1:SetTarget(s.atktg1)
    effect::set_target(d, e1, Some(cost_target(atktg1)));
    // e1:SetOperation(s.atkop1)
    effect::set_operation(d, e1, Some(operation(atkop1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // change atk
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_TRIGGER_O+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_FIELD);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.atkcon2)
    effect::set_condition(d, e2, Some(condition(atkcon2)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.atktg2)
    effect::set_target(d, e2, Some(cost_target(atktg2)));
    // e2:SetOperation(s.atkop2)
    effect::set_operation(d, e2, Some(operation(atkop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atkfilter1(c)`
fn atkfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_LYRILUSC)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_LYRILUSC as u16])
}
/// `s.atktg1(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.atkfilter1(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && atkfilter1(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil)
    //     and Duel.IsExistingTarget(s.atkfilter1,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX)
            && duel::is_existing_target(d, Some(&atkfilter1), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.atkfilter1,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, a.tp, Some(&atkfilter1), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.atkop1(e,tp,eg,ep,ev,re,r,rp)`
fn atkop1(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and tc:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc)) {
        return;
    }
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // for oc in g:Iter() do
    for oc in group::iter(d, &g) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(tc:GetAttack())
        let atk = card::get_attack(d, tc) as i64;
        effect::set_value(d, e1, atk);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // oc:RegisterEffect(e1)
        card::register_effect(d, oc, e1, false);
        // if oc:HasLevel() then
        if card::has_level(d, oc) {
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_CHANGE_LEVEL)
            effect::set_code(d, e2, EFFECT_CHANGE_LEVEL);
            // e2:SetValue(1)
            effect::set_value(d, e2, 1);
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // oc:RegisterEffect(e2)
            card::register_effect(d, oc, e2, false);
        }
        // if oc:IsType(TYPE_XYZ) then
        if card::is_type(d, oc, TYPE_XYZ) {
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            // e3:SetCode(EFFECT_CHANGE_RANK)
            effect::set_code(d, e3, EFFECT_CHANGE_RANK);
            // e3:SetValue(1)
            effect::set_value(d, e3, 1);
            // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
            // oc:RegisterEffect(e3)
            card::register_effect(d, oc, e3, false);
        }
    }
}
/// `s.atkcon2(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d).nil();
    // local d=Duel.GetAttackTarget()
    // return a:IsControler(1-tp) and d and d:IsControler(tp) and d:IsFaceup() and d:IsSetCard(SET_LYRILUSC)
    if !card::is_controler(d, att, 1 - a.tp) {
        return false;
    }
    let Some(dt) = duel::get_attack_target(d) else {
        return false;
    };
    card::is_controler(d, dt, a.tp) && card::is_faceup(d, dt) && card::is_set_card(d, dt, &[SET_LYRILUSC as u16])
}
/// `s.atktg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.GetAttacker():CreateEffectRelation(e)
    let att = duel::get_attacker(d).nil();
    card::create_effect_relation(d, att, a.e);
    true
}
/// `s.atkop2(e,tp,eg,ep,ev,re,r,rp)`
fn atkop2(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d).nil();
    // local d=Duel.GetAttackTarget()
    let dt = duel::get_attack_target(d);
    // if a:IsRelateToEffect(e) and a:IsFaceup()
    //     and d:IsRelateToBattle() and d:IsFaceup() then
    if card::is_relate_to_effect(d, att, a.e) && card::is_faceup(d, att) {
        let dt = dt.nil();
        if card::is_relate_to_battle(d, dt) && card::is_faceup(d, dt) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
            effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
            // e1:SetValue(a:GetAttack())
            let atk = card::get_attack(d, att) as i64;
            effect::set_value(d, e1, atk);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // d:RegisterEffect(e1)
            card::register_effect(d, dt, e1, false);
        }
    }
}
