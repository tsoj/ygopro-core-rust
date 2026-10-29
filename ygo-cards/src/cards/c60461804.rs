//! Destiny HERO - Destroyer Phoenix Enforcer
use super::prelude::*;

pub const ID: u32 = 60461804;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_HERO, SET_DESTINY_HERO],
    material_setcode: &[SET_HERO, SET_DESTINY_HERO],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcFun2(c,s.matfilter,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_DESTINY_HERO),true)
    fusion::add_proc_fun2(
        d,
        c,
        fusion::fus_mat(|d, c, fc, st, tp| {
            // s.matfilter(c,fc,sumtype,tp): c:IsSetCard(SET_HERO,fc,sumtype,tp) and c:IsLevelAbove(6)
            card::is_set_card_ctx(d, c, &[SET_HERO as u16], Some(fc), st, Some(tp)) && card::is_level_above(d, c, 6)
        }),
        fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_DESTINY_HERO as u16], Some(fc), st, Some(tp))),
        true,
    );
    // Decrease the ATK of monster your opponent controls
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy 2 cards on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special summon 1 "destiny HERO" monster during the next Standby Phase
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.atkval(e,c)`: `Duel.GetMatchingGroupCount(Card.IsSetCard,e:GetHandler():GetControler(),LOCATION_GRAVE,0,nil,SET_HERO)*-200`
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    let h = effect::get_handler(d, e).nil();
    let p = card::get_controler(d, h);
    duel::get_matching_group_count(d, Some(&|d, cc| card::is_set_card(d, cc, &[SET_HERO as u16])), p, LOCATION_GRAVE, 0, NO_EX) as i64 * -200
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsControler,1,nil,tp)
    (group::is_exists(d, sg, &|d, cc| card::is_controler(d, cc, tp), 1, NO_EX), false)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local rg=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let rg = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return matches!(
            aux::select_unselect_group(d, &rg, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false),
            Err(true)
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,rg,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(rg), 2, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local rg=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let rg = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // local g=aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,1,tp,HINTMSG_DESTROY)
    let g = aux::select_unselect_group(d, &rg, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_DESTROY as u64), None, None, false)
        .unwrap_or_default();
    // if #g==2 then
    if g.len() == 2 {
        // Duel.HintSelection(g,true)
        duel::hint_selection(d, g.clone(), true);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`: `(r&REASON_EFFECT+REASON_BATTLE)~=0`
fn spcon(_d: &mut Duel, a: &EffArgs) -> bool {
    (a.r & (REASON_EFFECT + REASON_BATTLE)) != 0
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetLabel(Duel.GetTurnCount())
    let turn = duel::get_turn_count(d, None) as i64;
    effect::set_label(d, e1, &[turn]);
    // e1:SetCondition(s.spcon1)
    effect::set_condition(d, e1, Some(condition(spcon1)));
    // e1:SetOperation(s.spop1)
    effect::set_operation(d, e1, Some(operation(spop1)));
    // if Duel.IsTurnPlayer(tp) then
    if duel::is_turn_player(d, tp) {
        // e1:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN,1)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 1);
    } else {
        // e1:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN,1)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_SELF_TURN, 1);
    }
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.spcon1(e,tp,eg,ep,ev,re,r,rp)`
fn spcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetTurnCount()~=e:GetLabel()
    duel::get_turn_count(d, None) as i64 != effect::get_label(d, a.e)
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_DESTINY_HERO) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_DESTINY_HERO as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spop1(e,tp,eg,ep,ev,re,r,rp)`
fn spop1(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.filter),tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let nv = aux::necro_valley_filter(move |d, c| filter(d, c, e, tp));
    let g = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
