//! Prank-Kids Pandemonium
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 79059098;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_PRANK_KIDS],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Fusion.CreateSummonEff(c,aux.FilterBoolFunction(Card.IsSetCard,SET_PRANK_KIDS),nil,nil,nil,nil,s.stage2)
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_PRANK_KIDS as u16]))),
            stage2: Some(Arc::new(|d: &mut Duel, e: EffectId, tc: Option<CardId>, tp: u8, sg: Option<&CardSet>, chk: i64| stage2(d, e, tc, tp, sg, chk))),
            ..Default::default()
        },
    );
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.stage2(e,tc,tp,sg,chk)
fn stage2(d: &mut Duel, e: EffectId, _tc: Option<CardId>, tp: u8, _sg: Option<&CardSet>, chk: i64) {
    // if chk==2 then
    if chk == 2 {
        // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then
        if effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
            effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
            // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
            effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
            // e1:SetTargetRange(1,0)
            effect::set_target_range(d, e1, 1, 0);
            // e1:SetTarget(s.splimit)
            effect::set_target(d, e1, Some(effect_card_fn(splimit)));
            // e1:SetReset(RESET_PHASE|PHASE_END)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, tp);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_CANNOT_SUMMON)
            effect::set_code(d, e2, EFFECT_CANNOT_SUMMON);
            // Duel.RegisterEffect(e2,tp)
            duel::register_effect(d, e2, tp);
            // aux.RegisterClientHint(e:GetHandler(),nil,tp,1,0,aux.Stringid(id,1),nil)
            aux::register_client_hint(d, h, 0, tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
        }
    }
}
// function s.condition()
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsMainPhase()
    duel::is_main_phase(d, None)
}
// function s.splimit(e,c)
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsSetCard(SET_PRANK_KIDS)
    !card::is_set_card(d, c, &[SET_PRANK_KIDS as u16])
}
