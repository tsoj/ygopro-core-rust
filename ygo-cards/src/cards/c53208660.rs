//! Pendulum Call
use super::prelude::*;

pub const ID: u32 = 53208660;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAGICIAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 2 "Magician" monsters with different names from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_SEARCH + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_CHAIN,s.chainfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_CHAIN, lua_fn(chainfilter));
}
/// `s.chainfilter(re,tp,cid)`
fn chainfilter(d: &mut Duel, a: Args) -> bool {
    let re = a.e(0);
    let cid = a.int(2) as u8;
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, re).nil();
    // local loc=Duel.GetChainInfo(cid,CHAININFO_TRIGGERING_LOCATION)
    let loc = duel::get_chain_info(d, cid, &[CHAININFO_TRIGGERING_LOCATION])[0].int();
    // return not (re:GetActiveType()==(TYPE_PENDULUM|TYPE_SPELL) and not re:IsHasType(EFFECT_TYPE_ACTIVATE)
    //   and (loc&LOCATION_PZONE)==LOCATION_PZONE and rc:IsSetCard(SET_MAGICIAN))
    let blocked = effect::get_active_type(d, re) == TYPE_PENDULUM | TYPE_SPELL
        && !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
        && (loc & LOCATION_PZONE as i64) == LOCATION_PZONE as i64
        && card::is_set_card(d, rc, &[SET_MAGICIAN as u16]);
    !blocked
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetCustomActivityCount(id,tp,ACTIVITY_CHAIN)==0
    duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_CHAIN) == 0
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), a.tp, LOCATION_HAND, 0, 1, h.into());
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, a.tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MAGICIAN) and c:IsType(TYPE_PENDULUM) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_MAGICIAN as u16]) && card::is_type(d, c, TYPE_PENDULUM) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
        // return g:GetClassCount(Card.GetCode)>=2
        return group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 2;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    // if #g>=2 then
    if g.len() >= 2 {
        // local thg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dncheck,1,tp,HINTMSG_ATOHAND)
        let thg = aux::select_unselect_group(
            d,
            &g,
            Some(a.e),
            a.tp,
            Some(2),
            Some(2),
            Some(&aux::dncheck()),
            1,
            a.tp,
            Some(HINTMSG_ATOHAND as u64),
            None,
            None,
            false,
        )
        .unwrap_or_default();
        // if #thg>0 then
        if !thg.is_empty() {
            // Duel.SendtoHand(thg,nil,REASON_EFFECT)
            duel::send_to_hand(d, thg.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,thg)
            duel::confirm_cards(d, 1 - a.tp, thg);
        }
    }
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
        // e1:SetTargetRange(LOCATION_PZONE,0)
        effect::set_target_range(d, e1, LOCATION_PZONE, 0);
        // e1:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_MAGICIAN))
        effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_MAGICIAN as u16]))));
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
