//! Spirit of Flames
use super::prelude::*;

pub const ID: u32 = 13522325;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    card::register_effect(d, c, e1, false);
    // atk
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_condition(d, e2, Some(condition(atkcon)));
    effect::set_value(d, e2, 300);
    card::register_effect(d, c, e2, false);
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tp=Duel.GetTurnPlayer()
    let tp = duel::get_turn_player(d);
    // return tp==e:GetHandler():GetControler() and Duel.IsBattlePhase()
    let h = effect::get_handler(d, a.e).nil();
    tp == card::get_controler(d, h) && duel::is_battle_phase(d, None)
}
/// `s.spfilter(c,tp)`: the Lua passes `ATTRIBUTE_WIND` as the second argument, which the function never uses
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_FIRE) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_attribute(d, c, ATTRIBUTE_FIRE) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local rg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE,0,nil,ATTRIBUTE_WIND)
    let rg = duel::get_matching_group(d, Some(&spfilter), tp, LOCATION_GRAVE, 0, NO_EX);
    // return aux.SelectUnselectGroup(rg,e,tp,1,1,aux.ChkfMMZ(1),0)
    let chkf = aux::chkf_mmz(1);
    matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(1), Some(&chkf), 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local rg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE,0,nil,ATTRIBUTE_WIND)
    let rg = duel::get_matching_group(d, Some(&spfilter), tp, LOCATION_GRAVE, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(rg,e,tp,1,1,aux.ChkfMMZ(1),1,tp,HINTMSG_REMOVE,nil,nil,true)
    let chkf = aux::chkf_mmz(1);
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(1), Some(&chkf), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive() (groups are values in Rust)
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else { return };
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // g:DeleteGroup() (groups are values in Rust)
}
