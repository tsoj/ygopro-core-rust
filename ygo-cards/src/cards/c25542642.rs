//! Phantom Knights' Fog Blade
//! 幻影霧剣
use super::prelude::*;

pub const ID: u32 = 25542642;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_THE_PHANTOM_KNIGHTS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddPersistentProcedure(c,nil,aux.FaceupFilter(Card.IsType,TYPE_EFFECT),CATEGORY_DISABLE,nil,nil,TIMINGS_CHECK_MONSTER,nil,nil,s.target)
    aux::add_persistent_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT))),
        Some(CATEGORY_DISABLE),
        None,
        None,
        Some(TIMINGS_CHECK_MONSTER),
        None,
        None,
        Some(std::sync::Arc::new(target)),
        None,
        false,
    );
    // Targeted monster has its effects negated, also cannot attack
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e3, EFFECT_CANNOT_ATTACK);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    // e3:SetTarget(aux.PersistentTargetFilter)
    effect::set_target(d, e3, Some(aux::persistent_target_filter()));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e5=e3:Clone()
    let e5 = effect::clone(d, e3);
    // e5:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e5, EFFECT_DISABLE);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // Targeted monster cannot be targeted for attack
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e4, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e4, LOCATION_MZONE, LOCATION_MZONE);
    // e4:SetValue(aux.PersistentTargetFilter)
    effect::set_value_fn(d, e4, aux::persistent_target_filter());
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Destroy itself if targeted monster leaves field
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e6, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e6:SetRange(LOCATION_SZONE)
    effect::set_range(d, e6, LOCATION_SZONE);
    // e6:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e6, EVENT_LEAVE_FIELD);
    // e6:SetCondition(s.descon)
    effect::set_condition(d, e6, Some(condition(descon)));
    // e6:SetOperation(s.desop)
    effect::set_operation(d, e6, Some(operation(desop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // Special summon 1 "The Phantom Knights" monster from GY
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e7, aux::stringid(ID, 0));
    // e7:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e7, CATEGORY_SPECIAL_SUMMON);
    // e7:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e7, EFFECT_FLAG_CARD_TARGET, 0);
    // e7:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e7, EFFECT_TYPE_QUICK_O);
    // e7:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e7, EVENT_FREE_CHAIN);
    // e7:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e7, LOCATION_GRAVE);
    // e7:SetCountLimit(1,id)
    effect::set_count_limit(d, e7, 1, ID, 0, 0);
    // e7:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e7, Some(cost::self_banish()));
    // e7:SetTarget(s.sptg)
    effect::set_target(d, e7, Some(cost_target(sptg)));
    // e7:SetOperation(s.spop)
    effect::set_operation(d, e7, Some(operation(spop)));
    // e7:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e7, 0, Some(TIMING_END_PHASE));
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,tc,chk)
fn target(d: &mut Duel, _a: &EffArgs, tc: Option<CardId>, chk: i64) -> bool {
    // if chk==0 then return true end
    if chk == 0 {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, tc.map(CardSet::from_card), 1, 0, 0);
    true
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsStatus(STATUS_DESTROY_CONFIRMED) then return false end
    if card::is_status(d, c, STATUS_DESTROY_CONFIRMED) {
        return false;
    }
    // local tc=c:GetFirstCardTarget()
    // return tc and eg:IsContains(tc)
    match card::get_first_card_target(d, c) {
        Some(tc) => a.eg.clone().nil().contains(tc),
        None => false,
    }
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
    let c = effect::get_handler(d, a.e).nil();
    duel::destroy(d, CardSet::from_card(c), REASON_EFFECT, None, None);
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_THE_PHANTOM_KNIGHTS) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_THE_PHANTOM_KNIGHTS as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::special_summon(d, CardSet::from_card(tc), 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
    {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let c = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3300)
        effect::set_description(d, e1, 3300);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        // e1:SetValue(LOCATION_REMOVED)
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        // tc:RegisterEffect(e1,true)
        card::register_effect(d, tc, e1, true);
    }
}
