//! Megarock Dragon
use super::prelude::*;

pub const ID: u32 = 71544954;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // c:AddMustBeSpecialSummoned()
    card::add_must_be_special_summoned(d, c);
    // Must be Special Summoned (from your hand) by banishing any number of Rock monster(s) from your GY
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0, aux::stringid(ID, 0));
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e0, EFFECT_SPSUMMON_PROC);
    // e0:SetRange(LOCATION_HAND)
    effect::set_range(d, e0, LOCATION_HAND);
    // e0:SetCondition(s.spcon)
    effect::set_condition(d, e0, Some(lua_fn(spcon)));
    // e0:SetTarget(s.sptg)
    effect::set_target(d, e0, Some(lua_fn(sptg)));
    // e0:SetOperation(s.spop)
    effect::set_operation(d, e0, Some(lua_fn(spop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
}
/// `s.spconfilter(c,tp)`
fn spconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRace(RACE_ROCK) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true) and Duel.GetMZoneCount(tp,c)>0
    card::is_race(d, c, RACE_ROCK)
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, false)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    if a.card(1).is_none() {
        return true;
    }
    let e = a.e(0);
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // local g=Duel.GetMatchingGroup(s.spconfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| spconfilter(d, c, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // return #g>0 and aux.SelectUnselectGroup(g,e,tp,1,1,nil,0)
    !g.is_empty()
        && matches!(
            aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(1), None, 0, tp, None, None, None, false),
            Err(true)
        )
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let (e, tp) = (a.e(0), a.p(1));
    // local rg=Duel.GetMatchingGroup(s.spconfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,tp)
    let rg = duel::get_matching_group(d, Some(&|d, c| spconfilter(d, c, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(rg,e,tp,1,#rg,nil,1,tp,HINTMSG_REMOVE,nil,nil,true)
    let g = aux::select_unselect_group(
        d,
        &rg,
        Some(e),
        tp,
        Some(1),
        Some(rg.len()),
        None,
        1,
        tp,
        Some(HINTMSG_REMOVE as u64),
        None,
        None,
        true,
    )
    .unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive() (groups are values in Rust)
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let c = a.c(8);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Value::Group(g) = effect::get_label_object(d, e) else {
        return;
    };
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_COST, None, None);
    // local val=#g*700
    let val = g.len() as i64 * 700;
    // g:DeleteGroup()
    // The original ATK/DEF of this card become the number of Rock monsters banished to Special Summon it x 700
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_SET_BASE_ATTACK)
    effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(val)
    effect::set_value(d, e1, val);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE&~RESET_TOFIELD)
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_SET_BASE_DEFENSE)
    effect::set_code(d, e2, EFFECT_SET_BASE_DEFENSE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
