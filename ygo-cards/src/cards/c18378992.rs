//! Malefic Selector
use super::prelude::*;

pub const ID: u32 = 18378992;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MALEFIC], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 2 "Malefic" cards from your Deck to your hand, except "Malefic Selector", with different names from each other and from the banished cards
    // local e1=Effect.CreateEffect(c)
    // e1:SetDescription(aux.Stringid(id,0))
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    // e1:SetCode(EVENT_FREE_CHAIN)
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    // e1:SetCost(s.cost)
    // e1:SetTarget(s.target)
    // e1:SetOperation(s.activate)
    // c:RegisterEffect(e1)
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
// function s.costfilter(c)
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MALEFIC) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_set_card(d, c, &[SET_MALEFIC as u16]) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // if #sg~=2 then return false end
    if sg.len() != 2 {
        return (false, false);
    }
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil,sg:GetFirst():GetCode(),sg:GetNext():GetCode())
    let order = group::iter(d, sg);
    let code1 = card::get_code(d, order[0])[0];
    let code2 = card::get_code(d, order[1])[0];
    let g = duel::get_matching_group(d, Some(&|d, c| thfilter(d, c, code1, code2)), tp, LOCATION_DECK, 0, NO_EX);
    // return g:GetClassCount(Card.GetCode)>1
    (group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) > 1, false)
}
// function s.thfilter(c,code1,code2)
fn thfilter(d: &mut Duel, c: CardId, code1: u32, code2: u32) -> bool {
    // return c:IsSetCard(SET_MALEFIC) and not c:IsCode(id,code1,code2) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_MALEFIC as u16]) && !card::is_code(d, c, &[ID, code1, code2]) && card::is_able_to_hand(d, c, None)
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // e:SetLabel(-100)
    effect::set_label(d, a.e, &[-100]);
    // local g=Duel.GetMatchingGroup(s.costfilter,tp,LOCATION_GRAVE|LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&costfilter), tp, LOCATION_GRAVE | LOCATION_MZONE, 0, NO_EX);
    let rs: aux::Rescon = &rescon;
    // if chk==0 then return #g>=2 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return g.len() >= 2 && matches!(aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(rs), 0, tp, None, None, None, false), Err(true));
    }
    // local rg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_REMOVE)
    let rg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(rs), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, false).unwrap_or_default();
    // Duel.Remove(rg,POS_FACEUP,REASON_COST)
    duel::remove(d, rg.clone(), POS_FACEUP as u8, REASON_COST, None, None);
    // e:SetLabel(rg:GetFirst():GetCode(),rg:GetNext():GetCode())
    let order = group::iter(d, &rg);
    let mut labels = vec![card::get_code(d, order[0])[0] as i64];
    labels.extend(card::get_code(d, order[1]).iter().map(|&x| x as i64));
    effect::set_label(d, a.e, &labels);
    true
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        // local res=e:GetLabel()==-100
        let res = effect::get_label(d, a.e) == -100;
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return res
        return res;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, tp, LOCATION_DECK as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local code1,code2=e:GetLabel()
    let labels = effect::get_labels(d, a.e);
    let code1 = labels.first().copied().unwrap_or(0) as u32;
    let code2 = labels.get(1).copied().unwrap_or(0) as u32;
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil,code1,code2)
    let g = duel::get_matching_group(d, Some(&|d, c| thfilter(d, c, code1, code2)), tp, LOCATION_DECK, 0, NO_EX);
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dncheck,1,tp,HINTMSG_ATOHAND)
    let rs0 = aux::dncheck();
    let rs: aux::Rescon = &rs0;
    let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(rs), 1, tp, Some(HINTMSG_ATOHAND as u64), None, None, false).unwrap_or_default();
    // if #sg==2 then
    if sg.len() == 2 {
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - tp, sg);
    }
}
