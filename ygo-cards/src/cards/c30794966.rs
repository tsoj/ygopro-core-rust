//! Hieratic Dragon of Asar
use super::prelude::*;

pub const ID: u32 = 30794966;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_HIERATIC],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon from hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetCondition(s.hspcon)
    effect::set_condition(d, e1, Some(lua_fn(hspcon)));
    // e1:SetTarget(s.hsptg)
    effect::set_target(d, e1, Some(lua_fn(hsptg)));
    // e1:SetOperation(s.hspop)
    effect::set_operation(d, e1, Some(lua_fn(hspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy replace
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
/// `s.spfilter1(c,tp)`
fn spfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_DRAGON) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_DRAGON) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.spfilter2(c,tp)`
fn spfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_NORMAL) and c:IsRace(RACE_DRAGON) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_type(d, c, TYPE_NORMAL) && card::is_race(d, c, RACE_DRAGON) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.chk(c,sg)`
fn chk(d: &mut Duel, c: CardId, sg: &CardSet) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_DRAGON) and sg:IsExists(s.spfilter2,1,nil,c)
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_DRAGON) && group::is_exists(d, sg, &|d, x| spfilter2(d, x), 1, NO_EX)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return aux.ChkfMMZ(1)(sg,e,tp,mg) and sg:IsExists(s.chk,1,nil,sg)
    let chkf = aux::chkf_mmz(1);
    let (ok, _) = chkf(d, sg, e, tp, mg, None);
    (ok && group::is_exists(d, sg, &|d, x| chk(d, x, sg), 1, NO_EX), false)
}
/// `s.hspcon(e,c)`
fn hspcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    let e = a.e(0);
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // local g1=Duel.GetMatchingGroup(s.spfilter1,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let g1 = duel::get_matching_group(d, Some(&spfilter1), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.spfilter2,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let g2 = duel::get_matching_group(d, Some(&spfilter2), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local g=g1:Clone()
    // g:Merge(g2)
    let mut g = g1.clone();
    group::add_card(&mut g, &g2);
    let _ = c;
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>-2 and #g1>0 and #g2>0 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0)
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= -2 || g1.is_empty() || g2.is_empty() {
        return false;
    }
    matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true))
}
/// `s.hsptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn hsptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local g1=Duel.GetMatchingGroup(s.spfilter1,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let mut g1 = duel::get_matching_group(d, Some(&spfilter1), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.spfilter2,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let g2 = duel::get_matching_group(d, Some(&spfilter2), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // local g=aux.SelectUnselectGroup(g1,e,tp,2,2,s.rescon,1,tp,HINTMSG_REMOVE,nil,nil,true)
    let g = aux::select_unselect_group(d, &g1, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.hspop(e,tp,eg,ep,ev,re,r,rp,c)`
fn hspop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Value::Group(g) = effect::get_label_object(d, e) else {
        return;
    };
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
}
/// `s.repfilter(c)`
fn repfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_HIERATIC) and not c:IsStatus(STATUS_DESTROY_CONFIRMED)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_HIERATIC as u16]) && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED)
}
/// `s.desreptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:IsReason(REASON_REPLACE) and Duel.CheckReleaseGroup(tp,s.repfilter,1,c) end
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_REPLACE) && duel::check_release_group(d, a.tp, Some(&repfilter), 1, duel::ReleaseArgs::default(), c.into(), false);
    }
    // if Duel.SelectEffectYesNo(tp,c,96) then
    if duel::select_effect_yes_no(d, a.tp, c, Some(96)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESREPLACE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESREPLACE as u64);
        // local g=Duel.SelectReleaseGroup(tp,s.repfilter,1,1,c)
        let g = duel::select_release_group(d, a.tp, Some(&repfilter), 1, 1, duel::ReleaseArgs::default(), c.into(), false).nil();
        // e:SetLabelObject(g:GetFirst())
        let first = group::get_first(d, &g);
        effect::set_label_object(d, a.e, first.map_or(Value::Nil, Value::Card));
        // return true
        return true;
    }
    // else return false end
    false
}
/// `s.desrepop(e,tp,eg,ep,ev,re,r,rp)`
fn desrepop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.Release(tc,REASON_EFFECT)
    duel::release(d, tc, REASON_EFFECT, None);
}
