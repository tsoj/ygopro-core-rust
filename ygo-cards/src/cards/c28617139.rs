//! Simorgh Onslaught
//! scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 28617139;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SIMORGH], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_SEARCH + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // level down
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.rvtg)
    effect::set_target(d, e2, Some(cost_target(rvtg)));
    // e2:SetOperation(s.rvop)
    effect::set_operation(d, e2, Some(operation(rvop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsDiscardable() and c:IsRace(RACE_WINGEDBEAST)
    card::is_discardable(d, c, None, None) && card::is_race(d, c, RACE_WINGEDBEAST)
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, e).nil();
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_HAND, 0, 1, h.into());
    }
    // Duel.DiscardHand(tp,s.cfilter,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, tp, Some(&cfilter), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SIMORGH) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_SIMORGH as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
        // return g:GetClassCount(Card.GetAttribute)>=2
        return group::get_class_count(d, &g, &|d, c| card::get_attribute(d, c) as i64) >= 2;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, tp, LOCATION_DECK as i32);
    true
}
// function s.attr(c) -- Card.GetAttribute as a number, for aux.dpcheck
fn attr_value(d: &mut Duel, c: CardId) -> i64 {
    card::get_attribute(d, c) as i64
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if g:GetClassCount(Card.GetAttribute)>=2 then
    if group::get_class_count(d, &g, &attr_value) >= 2 {
        // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dpcheck(Card.GetAttribute),1,tp,HINTMSG_ATOHAND)
        let sg = aux::select_unselect_group(
            d,
            &g,
            Some(e),
            tp,
            Some(2),
            Some(2),
            Some(&aux::dpcheck(attr_value)),
            1,
            tp,
            Some(HINTMSG_ATOHAND as u64),
            None,
            None,
            false,
        )
        .unwrap_or_default();
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - tp, sg);
    }
}
// function s.rvfilter(c)
fn rvfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_WINGEDBEAST) and c:IsLevelAbove(2) and not c:IsPublic()
    card::is_race(d, c, RACE_WINGEDBEAST) && card::is_level_above(d, c, 2) && !card::is_public(d, c)
}
// function s.hfilter(c,code)
fn hfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsCode(code) and c:IsLevelAbove(2)
    card::is_code(d, c, &[code]) && card::is_level_above(d, c, 2)
}
// function s.rvtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rvfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&rvfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    true
}
// function s.rvop(e,tp,eg,ep,ev,re,r,rp)
fn rvop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local g=Duel.SelectMatchingCard(tp,s.rvfilter,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&rvfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g.clone());
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    // local hg=Duel.GetFieldGroup(tp,LOCATION_HAND,0):Filter(s.hfilter,nil,g:GetFirst():GetCode())
    let fg = duel::get_field_group(d, tp, LOCATION_HAND, 0);
    let first = group::get_first(d, &g).nil();
    let code = card::get_code(d, first)[0];
    let hg = group::filter(d, &fg, &|d, x| hfilter(d, x, code), NO_EX);
    // local tc=hg:GetFirst()
    let _ = group::get_first(d, &hg);
    // for tc in aux.Next(hg) do
    for tc in group::iter(d, &hg) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_LEVEL)
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        // e1:SetValue(-1)
        effect::set_value(d, e1, -1);
        // e1:SetReset(RESET_EVENT|(RESETS_STANDARD_PHASE_END&~RESET_TOFIELD))
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TOFIELD), 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_TO_HAND)
    effect::set_code(d, e2, EVENT_TO_HAND);
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // e2:SetLabel(g:GetFirst():GetCode())
    effect::set_label(d, e2, &[code as i64]);
    // e2:SetOperation(s.hlvop)
    effect::set_operation(d, e2, Some(operation(hlvop)));
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
}
// function s.hlvfilter(c,tp,code)
fn hlvfilter(d: &mut Duel, c: CardId, tp: u8, code: u32) -> bool {
    // return c:IsLevelAbove(1) and c:IsControler(tp) and c:IsCode(code)
    card::is_level_above(d, c, 1) && card::is_controler(d, c, tp) && card::is_code(d, c, &[code])
}
// function s.hlvop(e,tp,eg,ep,ev,re,r,rp)
fn hlvop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local hg=eg:Filter(s.hlvfilter,nil,tp,e:GetLabel())
    let eg = a.eg.clone().nil();
    let code = effect::get_label(d, e) as u32;
    let hg = group::filter(d, &eg, &|d, c| hlvfilter(d, c, tp, code), NO_EX);
    // local tc=hg:GetFirst()
    let _ = group::get_first(d, &hg);
    // for tc in aux.Next(hg) do
    for tc in group::iter(d, &hg) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_LEVEL)
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        // e1:SetValue(-1)
        effect::set_value(d, e1, -1);
        // e1:SetReset(RESET_EVENT|(RESETS_STANDARD_PHASE_END&~RESET_TOFIELD))
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TOFIELD), 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
