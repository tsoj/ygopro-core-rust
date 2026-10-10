//! Shooting Quasar Dragon
use super::prelude::*;

pub const ID: u32 = 35952884;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[24696097], synchro_tuner_required: Some(1), synchro_nt_required: Some(2), ..ScriptInfo::DEFAULT };

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
    // multi attack
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(s.mtcon)
    effect::set_condition(d, e2, Some(condition(mtcon)));
    // e2:SetOperation(s.mtop)
    effect::set_operation(d, e2, Some(operation(mtop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e3, EFFECT_MATERIAL_CHECK);
    // e3:SetValue(s.valcheck)
    effect::set_value_fn(d, e3, lua_fn(valcheck));
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // negate
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.discon)
    effect::set_condition(d, e3, Some(condition(discon)));
    // e3:SetTarget(s.distg)
    effect::set_target(d, e3, Some(cost_target(distg)));
    // e3:SetOperation(s.disop)
    effect::set_operation(d, e3, Some(operation(disop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Special Summon
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetType(EFFECT_TYPE_TRIGGER_O+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_SINGLE);
    // e4:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e4, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    // e4:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e4, EVENT_LEAVE_FIELD);
    // e4:SetCondition(s.sumcon)
    effect::set_condition(d, e4, Some(condition(sumcon)));
    // e4:SetTarget(s.sumtg)
    effect::set_target(d, e4, Some(cost_target(sumtg)));
    // e4:SetOperation(s.sumop)
    effect::set_operation(d, e4, Some(operation(sumop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.mfilter(c)`
fn mfilter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsType(TYPE_TUNER)
    !card::is_type(d, c, TYPE_TUNER)
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, a: Args) -> Value {
    let (e, c) = (a.e(0), a.c(1));
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // local ct=g:FilterCount(s.mfilter,nil)
    let ct = group::filter_count(d, &g, &mfilter, NO_EX) as i64;
    // e:GetLabelObject():SetLabel(ct)
    let e2 = effect::get_label_object(d, e).as_effect().nil();
    effect::set_label(d, e2, &[ct]);
    Value::Nil
}
/// `s.mtcon(e,tp,...)`
fn mtcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsSynchroSummoned() and e:GetLabel()>0
    let h = effect::get_handler(d, a.e).nil();
    card::is_synchro_summoned(d, h) && effect::get_label(d, a.e) > 0
}
/// `s.mtop(e,tp,...)`
fn mtop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_EXTRA_ATTACK)
    effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
    // e1:SetValue(ct-1)
    effect::set_value(d, e1, ct - 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.discon(e,tp,...)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) and Duel.IsChainNegatable(ev)
    let h = effect::get_handler(d, a.e).nil();
    !card::is_status(d, h, STATUS_BATTLE_DESTROYED) && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    let eg = a.eg.clone().nil();
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, Some(eg.clone()), 1, 0, 0);
    // if re:GetHandler():IsDestructable() and re:GetHandler():IsRelateToEffect(re) then
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(eg), 1, 0, 0);
    }
    true
}
/// `s.disop(e,tp,eg,...)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let rc = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rc, re) {
            // Duel.Destroy(eg,REASON_EFFECT)
            let eg = a.eg.clone().nil();
            duel::destroy(d, eg, REASON_EFFECT, None, None);
        }
    }
}
/// `s.sumcon(e,tp,...)`
fn sumcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousPosition(POS_FACEUP) and e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD)
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_position(d, h, POS_FACEUP) && card::is_previous_location(d, h, LOCATION_ONFIELD)
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(24696097) and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,true)
    card::is_code(d, c, &[24696097])
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, true, None, None, None)
}
/// `s.sumtg(e,tp,...,chk)`
fn sumtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.sumop(e,tp,...)`
fn sumop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetFirstMatchingCard(s.filter,tp,LOCATION_EXTRA,0,nil,e,tp)
    let tg = duel::get_first_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, NO_EX);
    // if tg then
    if let Some(tg) = tg {
        // Duel.SpecialSummon(tg,0,tp,tp,false,true,POS_FACEUP)
        duel::special_summon(d, tg, 0, a.tp, a.tp, false, true, POS_FACEUP as u8, None);
    }
}
