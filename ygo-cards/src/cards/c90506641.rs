//! Soul Unseal
use super::prelude::*;

pub const ID: u32 = 90506641;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SEARCH + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.thcon)
    effect::set_condition(d, e1, Some(condition(thcon)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_NORMAL)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_NORMAL)
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(1-tp,LOCATION_MZONE,0)>0 and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::get_field_group_count(d, 1 - a.tp, LOCATION_MZONE, 0) > 0
        && duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_NORMAL) and (c:IsAbleToHand() or c:IsAbleToRemove())
    card::is_type(d, c, TYPE_NORMAL) && (card::is_able_to_hand(d, c, None) || card::is_able_to_remove(d, c, None, None, None))
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsAbleToHand,nil)>0 and sg:FilterCount(Card.IsAbleToRemove,nil)>3
    let res = group::filter_count(d, sg, &|d, c| card::is_able_to_hand(d, c, None), NO_EX) > 0
        && group::filter_count(d, sg, &|d, c| card::is_able_to_remove(d, c, None, None, None), NO_EX) > 3;
    (res, false)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,5,5,s.rescon,chk) end
    if a.chk == Some(0) {
        let rs: aux::Rescon = &rescon;
        return matches!(aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(5), Some(5), Some(rs), 0, a.tp, None, None, None, false), Err(true));
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,4,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 4, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    // if #g<5 then return end
    if g.len() < 5 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local sg=aux.SelectUnselectGroup(g,e,tp,5,5,s.rescon,1,tp,HINTMSG_ATOHAND)
    let rs: aux::Rescon = &rescon;
    let mut sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(5), Some(5), Some(rs), 1, a.tp, Some(HINTMSG_ATOHAND as u64), None, None, false).unwrap_or_default();
    // if #sg~=5 then return end
    if sg.len() != 5 {
        return;
    }
    // local sc
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let sc = if group::filter_count(d, &sg, &|d, c| card::is_able_to_hand(d, c, None), NO_EX) > 0 {
        // sc=sg:FilterSelect(tp,Card.IsAbleToHand,1,1,nil)
        group::filter_select(d, &sg, a.tp, &|d, c| card::is_able_to_hand(d, c, None), 1, 1, false, NO_EX).nil()
    } else {
        // sc=sg:Select(tp,1,1,nil)
        group::select(d, &sg, a.tp, 1, 1, false, NO_EX).nil()
    };
    // sg=sg-sc
    group::remove_card(&mut sg, &sc);
    // if #sg>0 and Duel.SendtoHand(sc,nil,REASON_EFFECT)==0 then
    if !sg.is_empty() && duel::send_to_hand(d, sc.clone(), None, REASON_EFFECT, None) == 0 {
        // Duel.SendtoGrave(sc,REASON_RULE)
        duel::send_to_grave(d, sc, REASON_RULE, None, None);
    }
    // Duel.Remove(sg,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, sg, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
