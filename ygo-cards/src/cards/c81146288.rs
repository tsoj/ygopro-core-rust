//! Rosaria, the Stately Fallen Angel
use super::prelude::*;

pub const ID: u32 = 81146288;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Special Summon procedure
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
    // Negate the effect of all cards currently on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_DISABLE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsLocation,1,nil,LOCATION_HAND)
    //     and (sg:IsExists(Card.IsLocation,1,nil,LOCATION_GRAVE) or (sg:IsExists(aux.SpElimFilter,1,nil,true)))
    let hand = group::is_exists(d, sg, &|d, x| card::is_location(d, x, LOCATION_HAND), 1, NO_EX);
    let gy = group::is_exists(d, sg, &|d, x| card::is_location(d, x, LOCATION_GRAVE), 1, NO_EX)
        || group::is_exists(d, sg, &|d, x| aux::sp_elim_filter(d, x, true, false), 1, NO_EX);
    (hand && gy, false)
}
// function s.spfilter(c,tp)
fn spfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsLevelAbove(7) and c:IsRace(RACE_PLANT) and c:IsAbleToRemoveAsCost()
    //     and (c:IsLocation(LOCATION_HAND) or (aux.SpElimFilter(c,true) and Duel.GetMZoneCount(tp,c)>0))
    card::is_level_above(d, c, 7)
        && card::is_race(d, c, RACE_PLANT as u64)
        && card::is_able_to_remove_as_cost(d, c, None)
        && (card::is_location(d, c, LOCATION_HAND)
            || (aux::sp_elim_filter(d, c, true, false) && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0))
}
// function s.spcon(e,c)
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    let _ = c;
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    let h = effect::get_handler(d, e).nil();
    // local rg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND,0,e:GetHandler(),tp)
    let mut rg = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, tp)), tp, LOCATION_HAND, 0, h.into());
    // local rg2=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,tp)
    let rg2 = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // rg:Merge(rg2)
    group::add_card(&mut rg, &rg2);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>-2 and #rg>0 and #rg2>0
    //     and aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,0)
    let rs: aux::Rescon = &rescon;
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -2
        && !rg.is_empty()
        && !rg2.is_empty()
        && matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(rs), 0, tp, None, None, None, false), Err(true))
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,c)
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let h = effect::get_handler(d, e).nil();
    // local rg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_GRAVE,0,e:GetHandler(),tp)
    let rg = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, tp)), tp, LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE, 0, h.into());
    // local g=aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,1,tp,HINTMSG_REMOVE,nil,nil,true)
    let rs: aux::Rescon = &rescon;
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(rs), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true).unwrap_or_default();
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
// function s.spop(e,tp,eg,ep,ev,re,r,rp,c)
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    if let Value::Group(g) = effect::get_label_object(d, e) {
        // Duel.Remove(g,POS_FACEUP,REASON_COST)
        duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    }
}
// function s.distg(e,tp,eg,ep,ev,re,r,rp,chk)
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsNegatable,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_negatable(d, c)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, h.into());
    }
    true
}
// function s.disop(e,tp,eg,ep,ev,re,r,rp)
fn disop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(Card.IsNegatable,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,c)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_negatable(d, x)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // Negate effects
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // if tc:IsType(TYPE_TRAPMONSTER) then
        if card::is_type(d, tc, TYPE_TRAPMONSTER) {
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            // e3:SetCode(EFFECT_DISABLE_TRAPMONSTER)
            effect::set_code(d, e3, EFFECT_DISABLE_TRAPMONSTER);
            // e3:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e3)
            card::register_effect(d, tc, e3, false);
        }
    }
}
