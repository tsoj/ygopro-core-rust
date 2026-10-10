//! Thunder Sea Horse
//! サンダー・シーホース
use super::prelude::*;

pub const ID: u32 = 48049769;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetActivityCount(tp,ACTIVITY_SPSUMMON)==0 and c:IsDiscardable() end
    if a.chk == Some(0) {
        return duel::get_activity_count(d, tp, &[ACTIVITY_SPSUMMON])[0] == 0 && card::is_discardable(d, c, None, None);
    }
    // Duel.SendtoGrave(c,REASON_COST|REASON_DISCARD)
    duel::send_to_grave(d, c, REASON_COST | REASON_DISCARD, None, None);
    // oath effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,1),nil)
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    true
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_race(d, c, RACE_THUNDER)
        && card::is_attribute(d, c, ATTRIBUTE_LIGHT)
        && card::is_level(d, c, &[4])
        && card::is_attack_below(d, c, 1600)
        && card::is_able_to_hand(d, c, None)
}
/// `s.rescon(sg,e,tp,mg)`: `sg:GetClassCount(Card.GetCode)==1`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let res = group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64) == 1;
    (res, false)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_DECK, 0, NO_EX);
        // return aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0)
        let rs: aux::Rescon = &rescon;
        return matches!(aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(rs), 0, tp, None, None, None, false), Err(true));
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, tp, LOCATION_DECK as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_DECK, 0, NO_EX);
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // local hg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_ATOHAND)
    let rs: aux::Rescon = &rescon;
    let hg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(rs), 1, tp, Some(HINTMSG_ATOHAND as u64), None, None, false).unwrap_or_default();
    // if #hg==2 then
    if hg.len() == 2 {
        // Duel.SendtoHand(hg,nil,REASON_EFFECT)
        duel::send_to_hand(d, hg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,hg)
        duel::confirm_cards(d, 1 - tp, hg);
    }
}
