//! Familiar-Possessed - Wynn
use super::prelude::*;

pub const ID: u32 = 31764353;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[37744402], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon proc
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND|LOCATION_DECK)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_DECK);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter1(c)`
fn spfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCode(37744402) and c:IsAbleToGraveAsCost()
    card::is_faceup(d, c) && card::is_code(d, c, &[37744402]) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.spfilter2(c)`
fn spfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_WIND) and c:IsAbleToGraveAsCost()
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_WIND) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, c: Option<CardId>) -> (bool, bool) {
    // return aux.ChkfMMZ(1)(sg,e,tp,mg) and sg:IsExists(s.chk,1,nil,sg)
    let (valid, _) = aux::chkf_mmz(1)(d, sg, e, tp, mg, c);
    (valid && group::is_exists(d, sg, &|d, x| chk(d, x, sg), 1, NO_EX), false)
}
/// `s.chk(c,sg)`
fn chk(d: &mut Duel, c: CardId, sg: &CardSet) -> bool {
    // return c:IsCode(37744402) and sg:IsExists(Card.IsAttribute,1,c,ATTRIBUTE_WIND)
    card::is_code(d, c, &[37744402]) && group::is_exists(d, sg, &|d, x| card::is_attribute(d, x, ATTRIBUTE_WIND), 1, c.into())
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
    // local g1=Duel.GetMatchingGroup(s.spfilter1,tp,LOCATION_MZONE,0,nil)
    let g1 = duel::get_matching_group(d, Some(&spfilter1), tp, LOCATION_MZONE, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.spfilter2,tp,LOCATION_MZONE,0,nil)
    let g2 = duel::get_matching_group(d, Some(&spfilter2), tp, LOCATION_MZONE, 0, NO_EX);
    // local g=g1:Clone()
    // g:Merge(g2)
    let mut g = g1.clone();
    group::add_card(&mut g, &g2);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>-2 and #g1>0 and #g2>0 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -2
        && !g1.is_empty()
        && !g2.is_empty()
        && matches!(
            aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false),
            Err(true)
        )
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, e);
    // local g1=Duel.GetMatchingGroup(s.spfilter1,tp,LOCATION_MZONE,0,nil)
    let mut g1 = duel::get_matching_group(d, Some(&spfilter1), tp, LOCATION_MZONE, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.spfilter2,tp,LOCATION_MZONE,0,nil)
    let g2 = duel::get_matching_group(d, Some(&spfilter2), tp, LOCATION_MZONE, 0, NO_EX);
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // local g=aux.SelectUnselectGroup(g1,e,tp,2,2,s.rescon,1,tp,HINTMSG_TOGRAVE)
    let g = aux::select_unselect_group(d, &g1, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, false)
        .unwrap_or_default();
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
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else {
        return;
    };
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    // Duel.ShuffleDeck(tp)
    duel::shuffle_deck(d, tp);
    // --pierce
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e1, EFFECT_PIERCE);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE&~RESET_TOFIELD)
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
    // e:GetHandler():RegisterEffect(e1)
    card::register_effect(d, h, e1, false);
    // g:DeleteGroup()
}
