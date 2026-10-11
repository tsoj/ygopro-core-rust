//! Invoked Sorath
//! 召喚獣ソラト
//! scripted by Naim
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 43989315;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ALEISTER, SET_INVOKED],
    material_setcode: &[SET_ALEISTER],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 "Aleister" monster + 1 FIRE or WIND monster
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_ALEISTER),aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_FIRE|ATTRIBUTE_WIND))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_ALEISTER as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_attribute_ctx(d, c, (ATTRIBUTE_FIRE | ATTRIBUTE_WIND) as u32, Some(fc), st, Some(tp)))),
        ],
    );
    // During the Main Phase (Quick Effect): You can target 1 Level 6 or lower monster in either GY; Special Summon it to your field in Defense Position, but negate its effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is in your GY: You can Fusion Summon 1 "Invoked" Fusion Monster from your Extra Deck, by banishing its materials from your field and/or GY, including this card in the GY
    // local fusion_params={
    //     fusfilter=function(c) return c:IsSetCard(SET_INVOKED) end,
    //     matfilter=aux.FALSE,
    //     extrafil=function(e,tp,mg) ... end,
    //     extraop=Fusion.BanishMaterial,
    //     gc=Fusion.ForcedHandler,
    //     extratg=function(e,tp,eg,ep,ev,re,r,rp,chk) ... end
    // }
    let fusion_params = fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_INVOKED as u16]))),
        matfilter: Some(fusion::mat_filter_fn(|_d, _c, _e, _tp, _chk| false)),
        extrafil: Some(Arc::new(extrafil)),
        extraop: Some(fusion::banish_material()),
        gc: Some(fusion::forced_handler()),
        extratg: Some(cost_target(extratg)),
        ..Default::default()
    };
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e2, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(Fusion.SummonEffTG(fusion_params))
    let tg = fusion::summon_eff_tg(d, fusion_params.clone());
    effect::set_target(d, e2, Some(tg));
    // e2:SetOperation(Fusion.SummonEffOP(fusion_params))
    let op = fusion::summon_eff_op(d, fusion_params);
    effect::set_operation(d, e2, Some(op));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function(e,tp,mg) ... end (the extrafil of fusion_params)
fn extrafil(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // local locations=not Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) and (LOCATION_MZONE|LOCATION_GRAVE) or LOCATION_MZONE
    let locations = if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_none() {
        LOCATION_MZONE | LOCATION_GRAVE
    } else {
        LOCATION_MZONE
    };
    // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToRemove),tp,locations,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)), tp, locations, 0, NO_EX);
    (Some(g), None)
}
// function(e,tp,eg,ep,ev,re,r,rp,chk) ... end (the extratg of fusion_params)
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,e:GetHandler(),1,tp,LOCATION_MZONE|LOCATION_GRAVE)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(h)), 1, a.tp, (LOCATION_MZONE | LOCATION_GRAVE) as i32);
    true
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(6) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_level_below(d, c, 6) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,1,nil,e,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, a.tp, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP_DEFENSE) then
    if card::is_relate_to_effect(d, tc, a.e) && duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None) {
        // --Negate its effects
        // tc:NegateEffects(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        card::negate_effects(d, tc, h, None, false, None);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
