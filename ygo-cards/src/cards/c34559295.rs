//! Iron Draw
use super::prelude::*;

pub const ID: u32 = 34559295;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Draw 2 cards
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_MACHINE) and c:IsType(TYPE_EFFECT)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_MACHINE) && card::is_type(d, c, TYPE_EFFECT)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, 0, NO_EX);
    // return #g==2 and g:FilterCount(s.cfilter,nil)==2
    g.len() == 2 && group::filter_count(d, &g, &cfilter, NO_EX) == 2
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,2) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 2);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(2)
    duel::set_target_param(d, 2);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local spc=Duel.GetActivityCount(tp,ACTIVITY_SPSUMMON)
    let spc = duel::get_activity_count(d, a.tp, &[ACTIVITY_SPSUMMON])[0];
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let count = info[1].int() as u32;
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, p, count, REASON_EFFECT);
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.limittg)
    effect::set_target(d, e1, Some(lua_fn(limittg)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetLabel(spc)
    effect::set_label(d, e1, &[spc]);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_LEFT_SPSUMMON_COUNT)
    effect::set_code(d, e2, EFFECT_LEFT_SPSUMMON_COUNT);
    // e2:SetValue(s.countval)
    effect::set_value_fn(d, e2, lua_fn(countval));
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,1),nil)
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
}
/// `s.limittg(e,c,tp)`
fn limittg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(2);
    // local sp=Duel.GetActivityCount(tp,ACTIVITY_SPSUMMON)
    let sp = duel::get_activity_count(d, tp, &[ACTIVITY_SPSUMMON])[0];
    // return sp-e:GetLabel()>=1
    sp - effect::get_label(d, e) >= 1
}
/// `s.countval(e,re,tp)`
fn countval(d: &mut Duel, a: Args) -> i64 {
    let e = a.e(0);
    let tp = a.p(2);
    // local sp=Duel.GetActivityCount(tp,ACTIVITY_SPSUMMON)
    let sp = duel::get_activity_count(d, tp, &[ACTIVITY_SPSUMMON])[0];
    let label = effect::get_label(d, e);
    // if sp-e:GetLabel()>=1 then return 0 else return 1-sp+e:GetLabel() end
    if sp - label >= 1 { 0 } else { 1 - sp + label }
}
