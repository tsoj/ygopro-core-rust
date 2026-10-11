//! Dinomorphia Brute
//! ダイノルフィア・ブルート
use super::prelude::*;

pub const ID: u32 = 99414629;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DINOMORPHIA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E as u32));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // No effect damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCondition(s.nodamcon)
    effect::set_condition(d, e2, Some(condition(nodamcon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetOperation(s.nodamop)
    effect::set_operation(d, e2, Some(operation(nodamop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.PayLPCost(tp,Duel.GetLP(tp)//2)
    let half = duel::get_lp(d, a.tp).div_euclid(2) as u32;
    duel::pay_lp_cost(d, a.tp, half);
    true
}
/// `s.desfilter(c,tp)`
fn desfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return (c:IsFaceup() and c:IsSetCard(SET_DINOMORPHIA) and c:IsMonster() and c:IsControler(tp)) or c:IsControler(1-tp)
    (card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DINOMORPHIA as u16]) && card::is_monster(d, c) && card::is_controler(d, c, tp))
        || card::is_controler(d, c, 1 - tp)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsControler,nil,tp)==1
    (group::filter_count(d, sg, &|d, c| card::is_controler(d, c, tp), NO_EX) == 1, false)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local rg=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil,tp)
    let rg = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, tp)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return matches!(
            aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false),
            Err(true)
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,rg,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(rg), 2, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local rg=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil,tp)
    let rg = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, tp)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // local g=aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,1,tp,HINTMSG_DESTROY)
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_DESTROY as u64), None, None, false).unwrap_or_default();
    // if #g==2 then
    if g.len() == 2 {
        // Duel.HintSelection(g,true)
        duel::hint_selection(d, g.clone(), true);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
/// `s.nodamcon(e,tp,eg,ep,ev,re,r,rp)`
fn nodamcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and Duel.GetLP(tp)<=2000
    a.rp == 1 - a.tp && duel::get_lp(d, a.tp) <= 2000
}
/// `function(e,re,val,r,rp,rc) ... end` (EFFECT_CHANGE_DAMAGE value)
fn damval(d: &mut Duel, a: Args) -> i64 {
    // if (r&REASON_EFFECT)~=0 and rp~=e:GetOwnerPlayer() then return 0
    // else return val end
    let e = a.e(0);
    let val = a.int(2);
    let r = a.int(3) as u32;
    let rp = a.int(4) as u8;
    if (r & REASON_EFFECT) != 0 && rp != effect::get_owner_player(d, e) {
        0
    } else {
        val
    }
}
/// `s.nodamop(e,tp,eg,ep,ev,re,r,rp)`
fn nodamop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // No effect damage this turn
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CHANGE_DAMAGE)
    effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(s.damval)
    effect::set_value_fn(d, e1, lua_fn(damval));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_NO_EFFECT_DAMAGE)
    effect::set_code(d, e2, EFFECT_NO_EFFECT_DAMAGE);
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
}
