//! Gateway to Chaos
use super::prelude::*;

pub const ID: u32 = 40089744;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_BLACK_LUSTER_SOLDIER, SET_GAIA_THE_FIERCE_KNIGHT],
    counter_place_list: &[COUNTER_SPELL],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_SPELL)
    card::enable_counter_permit(d, c, COUNTER_SPELL as u16, None, None);
    // c:SetCounterLimit(COUNTER_SPELL,6)
    card::set_counter_limit(d, c, COUNTER_SPELL as u16, 6);
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // add counter
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetOperation(s.acop)
    effect::set_operation(d, e2, Some(operation(acop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // to hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCost(s.thcost)
    effect::set_cost(d, e3, Some(cost_target(thcost)));
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return ((c:IsSetCard(SET_BLACK_LUSTER_SOLDIER) and c:IsType(TYPE_RITUAL)) or c:IsSetCard(SET_GAIA_THE_FIERCE_KNIGHT))
    //   and c:IsMonster() and c:IsAbleToHand()
    ((card::is_set_card(d, c, &[SET_BLACK_LUSTER_SOLDIER as u16]) && card::is_type(d, c, TYPE_RITUAL)) || card::is_set_card(d, c, &[SET_GAIA_THE_FIERCE_KNIGHT as u16]))
        && card::is_monster(d, c)
        && card::is_able_to_hand(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsPreviousLocation(LOCATION_HAND|LOCATION_ONFIELD)
    card::is_monster(d, c) && card::is_previous_location(d, c, LOCATION_HAND | LOCATION_ONFIELD)
}
fn acop(d: &mut Duel, a: &EffArgs) {
    // local ct=eg:FilterCount(s.cfilter,nil)
    let eg = a.eg.clone().nil();
    let ct = group::filter_count(d, &eg, &cfilter, NO_EX);
    // if ct>0 then
    if ct > 0 {
        // e:GetHandler():AddCounter(COUNTER_SPELL,ct,true)
        let c = effect::get_handler(d, a.e).nil();
        card::add_counter(d, c, COUNTER_SPELL as u16, ct as u16, true);
    }
}
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (tp, e) = (a.tp, a.e);
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return e:GetHandler():IsCanRemoveCounter(tp,COUNTER_SPELL,3,REASON_COST) end
    if a.chk == Some(0) {
        return card::is_can_remove_counter(d, c, tp, COUNTER_SPELL as u16, 3, REASON_COST);
    }
    // e:GetHandler():RemoveCounter(tp,COUNTER_SPELL,3,REASON_COST)
    card::remove_counter(d, c, tp, COUNTER_SPELL as u16, 3, REASON_COST);
    true
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRitualSpell() and c:IsAbleToHand()
    card::is_ritual_spell(d, c) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
