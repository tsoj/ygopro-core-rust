//! Tierra, Source of Destruction
//! 創星神 tierra
use super::prelude::*;

pub const ID: u32 = 91588074;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
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
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_CANNOT_DISABLE_SPSUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_DISABLE_SPSUMMON);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // cannot special summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e3, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // to deck
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e4, CATEGORY_TODECK);
    // e4:SetType(EFFECT_TYPE_TRIGGER_F+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_TRIGGER_F + EFFECT_TYPE_SINGLE);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // e4:SetTarget(s.tdtg)
    effect::set_target(d, e4, Some(cost_target(tdtg)));
    // e4:SetOperation(s.tdop)
    effect::set_operation(d, e4, Some(operation(tdop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `local LOC_HAND_FIELD_GRAVE_EXTRA=LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE|LOCATION_EXTRA`
const LOC_HAND_FIELD_GRAVE_EXTRA: u32 = LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE | LOCATION_EXTRA;
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, c: Option<CardId>) -> (bool, bool) {
    // return aux.ChkfMMZ(1)(sg,e,tp,mg) and sg:GetClassCount(Card.GetCode)==#sg, sg:GetClassCount(Card.GetCode)~=#sg
    let mmz = aux::chkf_mmz(1)(d, sg, e, tp, mg, c).0;
    let cc = group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64);
    (mmz && cc == sg.len(), cc != sg.len())
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local g=Duel.GetMatchingGroup(Card.IsAbleToDeckOrExtraAsCost,tp,LOCATION_HAND|LOCATION_ONFIELD,0,c)
    let g = duel::get_matching_group(d, Some(&card::is_able_to_deck_or_extra_as_cost), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, c.into());
    // return (ft>0 or g:IsExists(aux.NOT(Card.IsInExtraMZone),(-ft)+1,nil)) and g:GetClassCount(Card.GetCode)>9
    (ft > 0 || group::is_exists(d, &g, &|d, c| !card::is_in_extra_m_zone(d, c, None), (1 - ft) as u16, NO_EX))
        && group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) > 9
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local c=... (the summoned card is the argument at index 9, `c`)
    let c = a.card(9);
    // local g=Duel.GetMatchingGroup(Card.IsAbleToDeckOrExtraAsCost,tp,LOCATION_HAND|LOCATION_ONFIELD,0,c)
    let g = duel::get_matching_group(d, Some(&card::is_able_to_deck_or_extra_as_cost), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, c.into());
    // local rg=aux.SelectUnselectGroup(g,e,tp,10,10,s.rescon,1,tp,HINTMSG_REMOVE,nil,nil,true)
    let rg = aux::select_unselect_group(d, &g, Some(e), tp, Some(10), Some(10), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true).unwrap_or_default();
    // if #rg>0 then
    if !rg.is_empty() {
        // rg:KeepAlive()
        // e:SetLabelObject(rg)
        effect::set_label_object(d, e, Value::Group(rg));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    // local rg=e:GetLabelObject()
    // if not rg then return end
    let Value::Group(rg) = effect::get_label_object(d, a.e(0)) else {
        return;
    };
    // Duel.SendtoDeck(rg,nil,SEQ_DECKSHUFFLE,REASON_COST)
    duel::send_to_deck(d, rg, None, SEQ_DECKSHUFFLE as i32, REASON_COST, None);
    // rg:DeleteGroup()
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsLocation(LOCATION_GRAVE|LOCATION_ONFIELD|LOCATION_HAND) or (c:IsFaceup() and c:IsType(TYPE_PENDULUM))) and c:IsAbleToDeck()
    (card::is_location(d, c, LOCATION_GRAVE | LOCATION_ONFIELD | LOCATION_HAND) || (card::is_faceup(d, c) && card::is_type(d, c, TYPE_PENDULUM)))
        && card::is_able_to_deck(d, c)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(s.tdfilter,tp,LOC_HAND_FIELD_GRAVE_EXTRA,LOC_HAND_FIELD_GRAVE_EXTRA,e:GetHandler())
    let handler = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&tdfilter), a.tp, LOC_HAND_FIELD_GRAVE_EXTRA, LOC_HAND_FIELD_GRAVE_EXTRA, handler.into());
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    // Duel.SetChainLimit(aux.FALSE)
    duel::set_chain_limit(d, aux_false());
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.tdfilter,tp,LOC_HAND_FIELD_GRAVE_EXTRA,LOC_HAND_FIELD_GRAVE_EXTRA,e:GetHandler())
    let handler = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&tdfilter), a.tp, LOC_HAND_FIELD_GRAVE_EXTRA, LOC_HAND_FIELD_GRAVE_EXTRA, handler.into());
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
}
