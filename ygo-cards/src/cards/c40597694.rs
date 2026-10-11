//! Scatter Fusion
//! スキャッター・フュージョン
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 40597694;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GEM_KNIGHT], ..ScriptInfo::DEFAULT };

/// `s.fusfilter(c)`
fn fusfilter(d: &mut Duel, c: CardId, _tp: u8) -> bool {
    // return c:IsType(TYPE_FUSION) and c:IsSetCard(SET_GEM_KNIGHT) and not c:IsRace(RACE_ROCK)
    card::is_type(d, c, TYPE_FUSION) && card::is_set_card(d, c, &[SET_GEM_KNIGHT as u16]) && !card::is_race(d, c, RACE_ROCK)
}

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);

    // Fusion Summon
    // local params = {fusfilter=s.fusfilter,matfilter=aux.FALSE,extrafil=s.extrafil,stage2=s.stage2,extratg=s.extratg}
    let params = fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(fusfilter)),
        matfilter: Some(Arc::new(|_d, _c, _e, _tp, _chk| false)),
        extrafil: Some(Arc::new(extrafil)),
        stage2: Some(Arc::new(stage2)),
        extratg: Some(cost_target(extratg)),
        ..Default::default()
    };
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(_,tp) return Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0 end)
    effect::set_condition(d, e2, Some(condition(fus_cond)));
    // e2:SetTarget(Fusion.SummonEffTG(params))
    let tg = fusion::summon_eff_tg(d, params.clone());
    effect::set_target(d, e2, Some(tg));
    // e2:SetOperation(Fusion.SummonEffOP(params))
    let op = fusion::summon_eff_op(d, params);
    effect::set_operation(d, e2, Some(op));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);

    // Destroy
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e3:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(_,tp) return Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0 end`
fn fus_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE) > 0
}
/// `s.extrafil(e,tp,mg1)`
fn extrafil(d: &mut Duel, _e: EffectId, tp: u8, _mg1: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToGrave),tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_grave(d, c, None)), tp, LOCATION_DECK, 0, NO_EX);
    (Some(g), None)
}
/// `s.stage2(e,tc,tp,sg,chk)`
fn stage2(d: &mut Duel, e: EffectId, tc: Option<CardId>, tp: u8, _sg: Option<&CardSet>, chk: i64) {
    if chk == 1 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, e).nil();
        // c:SetCardTarget(tc)
        card::set_card_target(d, c, tc.nil());
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e1, aux::stringid(ID, 1));
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
        effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetTarget(function(_,c) return not c:IsSetCard(SET_GEM_KNIGHT) and c:IsLocation(LOCATION_EXTRA) end)
        effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_set_card(d, c, &[SET_GEM_KNIGHT as u16]) && card::is_location(d, c, LOCATION_EXTRA))));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
        // aux.addTempLizardCheck(c,tp,function(_,c) return not c:IsSetCard(SET_GEM_KNIGHT) end)
        aux::add_temp_lizard_check(d, c, tp, Some(lua_fn(lizard_filter)), None, None, None, None);
    }
}
/// `function(_,c) return not c:IsSetCard(SET_GEM_KNIGHT) end`
fn lizard_filter(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    !card::is_set_card(d, c, &[SET_GEM_KNIGHT as u16])
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,0,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 0, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.desfilter(c,rc)`
fn desfilter(d: &mut Duel, c: CardId, rc: CardId) -> bool {
    // return rc:GetCardTarget():IsContains(c)
    let targets = card::get_card_target(d, rc);
    group::is_contains(&targets, c)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:GetCardTargetCount()>0 then
    if card::get_card_target_count(d, c) > 0 {
        // local dg=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,c)
        let dg = duel::get_matching_group(d, Some(&|d, x| desfilter(d, x, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
        // Duel.Destroy(dg,REASON_EFFECT)
        duel::destroy(d, dg, REASON_EFFECT, None, None);
    }
}
