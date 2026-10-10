//! Machina Redeployment
use super::prelude::*;

pub const ID: u32 = 86852702;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_MACHINA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate (option 1)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost(s.cfilter1))
    effect::set_cost(d, e1, Some(cost_target(cost(cfilter1))));
    // e1:SetTarget(s.target(s.filter1))
    effect::set_target(d, e1, Some(cost_target(target(filter1))));
    // e1:SetOperation(s.activate(s.filter1))
    effect::set_operation(d, e1, Some(operation(activate(filter1))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // activate (option 2)
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCost(s.cost(s.cfilter2))
    effect::set_cost(d, e2, Some(cost_target(cost(cfilter2))));
    // e2:SetTarget(s.target(s.filter2))
    effect::set_target(d, e2, Some(cost_target(target(filter2))));
    // e2:SetOperation(s.activate(s.filter2))
    effect::set_operation(d, e2, Some(operation(activate(filter2))));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.cfilter1(c)
fn cfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsDiscardable()
    card::is_discardable(d, c, None, None)
}
// function s.cfilter2(c)
fn cfilter2(d: &mut Duel, c: CardId) -> bool {
    // return s.cfilter1(c) and c:IsSetCard(SET_MACHINA)
    cfilter1(d, c) && card::is_set_card(d, c, &[SET_MACHINA as u16])
}
// function s.filter1(c)
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MACHINA) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_MACHINA as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.filter2(c)
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MACHINA) and not c:IsCode(id) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_MACHINA as u16]) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
// function s.cost(cfil)
// return function(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost(cfil: fn(&mut Duel, CardId) -> bool) -> impl Fn(&mut Duel, &EffArgs) -> bool + Send + Sync + 'static {
    move |d, a| {
        // if chk==0 then return Duel.IsExistingMatchingCard(cfil,tp,LOCATION_HAND,0,1,e:GetHandler()) end
        if a.chk == Some(0) {
            let h = effect::get_handler(d, a.e).nil();
            return duel::is_existing_matching_card(d, Some(&|d, c| cfil(d, c)), a.tp, LOCATION_HAND, 0, 1, h.into());
        }
        // Duel.DiscardHand(tp,cfil,1,1,REASON_COST|REASON_DISCARD)
        duel::discard_hand(d, a.tp, Some(&|d, c| cfil(d, c)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
        true
    }
}
// function s.target(fil)
// return function(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(fil: fn(&mut Duel, CardId) -> bool) -> impl Fn(&mut Duel, &EffArgs) -> bool + Send + Sync + 'static {
    move |d, a| {
        // local g=Duel.GetMatchingGroup(fil,tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| fil(d, c)), a.tp, LOCATION_DECK, 0, NO_EX);
        // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,2,2,aux.dncheck,0) end
        if a.chk == Some(0) {
            let rs = aux::dncheck();
            return matches!(aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rs), 0, a.tp, None, None, None, false), Err(true));
        }
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,2,0,0)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 2, 0, 0);
        true
    }
}
// function s.activate(fil)
// return function(e,tp,eg,ep,ev,re,r,rp)
fn activate(fil: fn(&mut Duel, CardId) -> bool) -> impl Fn(&mut Duel, &EffArgs) + Send + Sync + 'static {
    move |d, a| {
        let tp = a.tp;
        // local g=Duel.GetMatchingGroup(fil,tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| fil(d, c)), tp, LOCATION_DECK, 0, NO_EX);
        // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dncheck,1,tp,HINTMSG_ATOHAND)
        let rs = aux::dncheck();
        if let Ok(sg) = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&rs), 1, tp, Some(HINTMSG_ATOHAND as u64), None, None, false) {
            // if #sg==2 then
            if sg.len() == 2 {
                // Duel.SendtoHand(sg,nil,REASON_EFFECT)
                duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
                // Duel.ConfirmCards(1-tp,sg)
                duel::confirm_cards(d, 1 - tp, sg);
            }
        }
    }
}
