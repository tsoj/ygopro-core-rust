//! The Tricky
use super::prelude::*;

pub const ID: u32 = 14778250;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can Special Summon this card (from your hand) by discarding 1 card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
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
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g=Duel.GetMatchingGroup(Card.IsDiscardable,tp,LOCATION_HAND,0,c)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_discardable(d, x, None, None)), tp, LOCATION_HAND, 0, c.into());
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and aux.SelectUnselectGroup(g,e,tp,1,1,nil,0)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(1), None, 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.c(9);
    // local g=Duel.GetMatchingGroup(Card.IsDiscardable,tp,LOCATION_HAND,0,c)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_discardable(d, x, None, None)), tp, LOCATION_HAND, 0, c.into());
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,1,nil,1,tp,HINTMSG_DISCARD,nil,nil,true)
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(1), None, 1, tp, Some(HINTMSG_DISCARD as u64), None, None, true).unwrap_or_default();
    // if sg and #sg>0 then
    if !sg.is_empty() {
        // e:SetLabelObject(sg)
        effect::set_label_object(d, e, Value::Group(sg));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local sg=e:GetLabelObject()
    // if sg and #sg>0 then
    let Some(sg) = effect::get_label_object(d, e).as_group().cloned() else { return };
    if !sg.is_empty() {
        // Duel.SendtoGrave(sg,REASON_DISCARD|REASON_COST)
        duel::send_to_grave(d, sg, REASON_DISCARD | REASON_COST, None, None);
    }
}
