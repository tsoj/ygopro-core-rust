//! Onomatopaira
//! オノマト連携
use super::prelude::*;

pub const ID: u32 = 6595475;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GAGAGA, SET_GOGOGO, SET_DODODO, SET_ZUBABA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
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
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToGraveAsCost,tp,LOCATION_HAND,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&card::is_able_to_grave_as_cost), tp, LOCATION_HAND, 0, 1, h.into());
    }
    // Duel.DiscardHand(tp,Card.IsAbleToGraveAsCost,1,1,REASON_COST)
    duel::discard_hand(d, tp, Some(&card::is_able_to_grave_as_cost), 1, 1, REASON_COST, NO_EX);
    true
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and (c:IsSetCard(SET_GAGAGA) or c:IsSetCard(SET_GOGOGO) or c:IsSetCard(SET_DODODO) or c:IsSetCard(SET_ZUBABA)) and c:IsAbleToHand()
    card::is_monster(d, c)
        && (card::is_set_card(d, c, &[SET_GAGAGA as u16])
            || card::is_set_card(d, c, &[SET_GOGOGO as u16])
            || card::is_set_card(d, c, &[SET_DODODO as u16])
            || card::is_set_card(d, c, &[SET_ZUBABA as u16]))
        && card::is_able_to_hand(d, c, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // local t={SET_GAGAGA,SET_GOGOGO,SET_DODODO,SET_ZUBABA}
    let t = [SET_GAGAGA, SET_GOGOGO, SET_DODODO, SET_ZUBABA];
    // local count = 0
    let mut count = 0usize;
    // for i, set in ipairs(t) do
    for set in t {
        // if sg:IsExists(Card.IsSetCard,1,nil,set) then count = count + 1 end
        if group::is_exists(d, sg, &|d: &mut Duel, c| card::is_set_card(d, c, &[set as u16]), 1, NO_EX) {
            count += 1;
        }
    }
    // return count >= #sg
    (count >= group::get_count(sg), false)
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_DECK, 0, NO_EX);
    // if #g==0 then return end
    if group::get_count(&g) == 0 {
        return;
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,1,2,s.rescon,1,tp,HINTMSG_ATOHAND)
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_ATOHAND as u64), None, None, false).unwrap_or_default();
    // if #tg==0 then return end
    if group::get_count(&tg) == 0 {
        return;
    }
    // Duel.SendtoHand(tg,nil,REASON_EFFECT)
    duel::send_to_hand(d, tg.clone(), None, REASON_EFFECT, None);
    // Duel.ConfirmCards(1-tp,tg)
    duel::confirm_cards(d, 1 - tp, tg);
}
