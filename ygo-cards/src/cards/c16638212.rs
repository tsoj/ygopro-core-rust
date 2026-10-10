//! D.D. Sprite
use super::prelude::*;

pub const ID: u32 = 16638212;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    if a.card(1).is_none() {
        return true;
    }
    // return Duel.IsExistingMatchingCard(aux.AND(Card.IsAbleToRemoveAsCost,Card.IsFaceup),e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let p = effect::get_handler_player(d, a.e(0));
    duel::is_existing_matching_card(
        d,
        Some(&|d, x| card::is_able_to_remove_as_cost(d, x, None) && card::is_faceup(d, x)),
        p,
        LOCATION_MZONE,
        0,
        1,
        NO_EX,
    )
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local rg=Duel.GetMatchingGroup(Card.IsAbleToRemoveAsCost,tp,LOCATION_MZONE,0,nil):Filter(Card.IsFaceup,nil)
    let rg0 = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_remove_as_cost(d, x, None)), tp, LOCATION_MZONE, 0, NO_EX);
    let rg = group::filter(d, &rg0, &|d, x| card::is_faceup(d, x), NO_EX);
    // local g=aux.SelectUnselectGroup(rg,e,tp,1,1,aux.ChkfMMZ(1),1,tp,HINTMSG_REMOVE,nil,nil,true)
    let chkf = aux::chkf_mmz(1);
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(1), Some(&chkf), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive()
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
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else {
        return;
    };
    // Duel.Remove(g,POS_FACEUP,REASON_COST+REASON_TEMPORARY)
    duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_COST + REASON_TEMPORARY, None, None);
    // g:GetFirst():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,0)
    let first = group::get_first(d, &g).nil();
    card::register_flag_effect(d, first, ID, RESET_EVENT | RESETS_STANDARD, 0, 0, 0, 0);
    // local e1=Effect.CreateEffect(c)
    let c = a.c(8);
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetReset(RESET_EVENT|(RESETS_STANDARD_DISABLE&~RESET_TOFIELD)|RESET_PHASE|PHASE_STANDBY)
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD) | RESET_PHASE | PHASE_STANDBY, 1);
    // e1:SetOperation(s.retop)
    effect::set_operation(d, e1, Some(operation(retop)));
    // e1:SetLabelObject(g:GetFirst())
    effect::set_label_object(d, e1, Value::Card(first));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // g:DeleteGroup()  (no-op: groups are values)
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // if e:GetLabelObject():GetFlagEffect(id)~=0 then
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    if card::get_flag_effect(d, tc, ID) != 0 {
        // Duel.ReturnToField(e:GetLabelObject())
        duel::return_to_field(d, tc, None, None);
    }
}
