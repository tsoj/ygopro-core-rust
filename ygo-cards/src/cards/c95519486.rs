//! The Six Samurai - Zanji
use super::prelude::*;

pub const ID: u32 = 95519486;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_SIX_SAMURAI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e1, EVENT_DAMAGE_STEP_END);
    // e1:SetCondition(s.descon)
    effect::set_condition(d, e1, Some(condition(descon)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy replace
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTarget(s.desreptg)
    effect::set_target(d, e2, Some(cost_target(desreptg)));
    // e2:SetOperation(s.desrepop)
    effect::set_operation(d, e2, Some(operation(desrepop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_SIX_SAMURAI) and not c:IsCode(id)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SIX_SAMURAI as u16]) && !card::is_code(d, c, &[ID])
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // return e:GetHandler()==Duel.GetAttacker() and Duel.GetAttackTarget()
    //     and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    let h = effect::get_handler(d, e).nil();
    if duel::get_attacker(d) != Some(h) {
        return false;
    }
    if duel::get_attack_target(d).is_none() {
        return false;
    }
    duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetAttackTarget():IsRelateToBattle() end
    if a.chk == Some(0) {
        let t = duel::get_attack_target(d).nil();
        return card::is_relate_to_battle(d, t);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,Duel.GetAttackTarget(),1,0,0)
    let t = duel::get_attack_target(d).map(CardSet::from_card);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, t, 1, 0, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, _a: &EffArgs) {
    // local d=Duel.GetAttackTarget()
    let t = duel::get_attack_target(d).nil();
    // if d:IsRelateToBattle() then
    if card::is_relate_to_battle(d, t) {
        // Duel.Destroy(d,REASON_EFFECT)
        duel::destroy(d, CardSet::from_card(t), REASON_EFFECT, None, None);
    }
}
// function s.repfilter(c,e)
fn repfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_SIX_SAMURAI) and c:IsDestructable(e)
    //     and not c:IsStatus(STATUS_DESTROY_CONFIRMED+STATUS_BATTLE_DESTROYED)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_SIX_SAMURAI as u16])
        && card::is_destructable(d, c, Some(e))
        && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED + STATUS_BATTLE_DESTROYED)
}
// function s.desreptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return not c:IsReason(REASON_REPLACE) and c:IsOnField() and c:IsFaceup()
    //     and Duel.IsExistingMatchingCard(s.repfilter,tp,LOCATION_MZONE,0,1,c,e) end
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_REPLACE)
            && card::is_on_field(d, c)
            && card::is_faceup(d, c)
            && duel::is_existing_matching_card(d, Some(&|d, x| repfilter(d, x, e)), tp, LOCATION_MZONE, 0, 1, c.into());
    }
    // if Duel.SelectEffectYesNo(tp,c,96) then
    if duel::select_effect_yes_no(d, tp, c, Some(96)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESREPLACE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESREPLACE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.repfilter,tp,LOCATION_MZONE,0,1,1,c,e)
        let g = duel::select_matching_card(d, tp, Some(&|d, x| repfilter(d, x, e)), tp, LOCATION_MZONE, 0, 1, 1, false, c.into()).nil();
        // e:SetLabelObject(g:GetFirst())
        let first = group::get_first(d, &g);
        effect::set_label_object(d, e, first.map_or(Value::Nil, Value::Card));
        // g:GetFirst():SetStatus(STATUS_DESTROY_CONFIRMED,true)
        card::set_status(d, first.nil(), STATUS_DESTROY_CONFIRMED, true);
        // return true
        return true;
    }
    // else return false end
    false
}
// function s.desrepop(e,tp,eg,ep,ev,re,r,rp)
fn desrepop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, e).as_card().nil();
    // tc:SetStatus(STATUS_DESTROY_CONFIRMED,false)
    card::set_status(d, tc, STATUS_DESTROY_CONFIRMED, false);
    // Duel.Destroy(tc,REASON_EFFECT|REASON_REPLACE)
    duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT | REASON_REPLACE, None, None);
}
