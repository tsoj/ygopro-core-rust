//! Cosmic Blazar Dragon
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 21123811;
pub const INFO: ScriptInfo = ScriptInfo { synchro_tuner_required: Some(1), synchro_nt_required: Some(2), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_SYNCHRO),1,1,Synchro.NonTunerEx(Card.IsType,TYPE_SYNCHRO),2,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_SYNCHRO, sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner_ex(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_SYNCHRO, sc, st, Some(tp)))),
        2,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(aux.synlimit)
    effect::set_value_fn(d, e1, aux::synlimit());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // negate
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.discon)
    effect::set_condition(d, e2, Some(condition(discon)));
    // e2:SetCost(s.cost)
    effect::set_cost(d, e2, Some(cost_target(cost)));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // disable spsummon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DISABLE_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DISABLE_SUMMON + CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EVENT_SUMMON)
    effect::set_code(d, e3, EVENT_SUMMON);
    // e3:SetCondition(s.dscon)
    effect::set_condition(d, e3, Some(condition(dscon)));
    // e3:SetCost(s.cost)
    effect::set_cost(d, e3, Some(cost_target(cost)));
    // e3:SetTarget(s.dstg)
    effect::set_target(d, e3, Some(cost_target(dstg)));
    // e3:SetOperation(s.dsop)
    effect::set_operation(d, e3, Some(operation(dsop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EVENT_FLIP_SUMMON)
    effect::set_code(d, e4, EVENT_FLIP_SUMMON);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e3:Clone()
    let e5 = effect::clone(d, e3);
    // e5:SetCode(EVENT_SPSUMMON)
    effect::set_code(d, e5, EVENT_SPSUMMON);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // negate attack
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e6, aux::stringid(ID, 2));
    // e6:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e6, EFFECT_TYPE_QUICK_O);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e6, EVENT_ATTACK_ANNOUNCE);
    // e6:SetCountLimit(1)
    effect::set_count_limit(d, e6, 1, 0, 0, 0);
    // e6:SetCondition(s.negcon)
    effect::set_condition(d, e6, Some(condition(negcon)));
    // e6:SetCost(s.cost)
    effect::set_cost(d, e6, Some(cost_target(cost)));
    // e6:SetOperation(s.negop)
    effect::set_operation(d, e6, Some(operation(negop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp~=tp and not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) and Duel.IsChainNegatable(ev)
    let h = effect::get_handler(d, a.e).nil();
    a.rp != a.tp && !card::is_status(d, h, STATUS_BATTLE_DESTROYED) && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemoveAsCost() end
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None);
    }
    // if Duel.Remove(c,POS_FACEUP,REASON_COST+REASON_TEMPORARY)~=0 then
    if duel::remove(d, c, POS_FACEUP as u8, REASON_COST + REASON_TEMPORARY, None, None) != 0 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // e1:SetLabelObject(c)
        effect::set_label_object(d, e1, Value::Card(c));
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetOperation(s.retop)
        effect::set_operation(d, e1, Some(operation(retop)));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
    true
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsDestructable() and re:GetHandler():IsRelateToEffect(re) then
    let rh = effect::get_handler(d, a.re.nil()).nil();
    if card::is_destructable(d, rh, None) && card::is_relate_to_effect(d, rh, a.re.nil()) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) && card::is_relate_to_effect(d, effect::get_handler(d, a.re.nil()).nil(), a.re.nil()) {
        // Duel.Destroy(eg,REASON_EFFECT)
        duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
    }
}
/// `s.dscon(e,tp,eg,ep,ev,re,r,rp)`
fn dscon(d: &mut Duel, a: &EffArgs) -> bool {
    // return tp~=ep and Duel.GetCurrentChain()==0
    a.tp != a.ep && duel::get_current_chain(d, false) == 0
}
/// `s.dstg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn dstg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE_SUMMON,eg,#eg,0,0)
    let n = a.eg.clone().nil().len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DISABLE_SUMMON, a.eg.clone(), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,#eg,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), n, 0, 0);
    true
}
/// `s.dsop(e,tp,eg,ep,ev,re,r,rp)`
fn dsop(d: &mut Duel, a: &EffArgs) {
    // Duel.NegateSummon(eg)
    duel::negate_summon(d, a.eg());
    // Duel.Destroy(eg,REASON_EFFECT)
    duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateAttack() then
    if duel::negate_attack(d) {
        // Duel.SkipPhase(1-tp,PHASE_BATTLE,RESET_PHASE|PHASE_BATTLE,1)
        duel::skip_phase(d, 1 - a.tp, PHASE_BATTLE, RESET_PHASE | PHASE_BATTLE, 1, 0);
    }
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // Duel.ReturnToField(e:GetLabelObject())
    let c = effect::get_label_object(d, a.e).as_card().nil();
    duel::return_to_field(d, c, None, None);
}
