//! Starliege Seyfert
use super::prelude::*;

pub const ID: u32 = 15381421;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 dragon, whose level equals to sent monster(s)'s level
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.thcost)
    effect::set_cost(d, e1, Some(cost_target(thcost)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Recover 1 level 8 DARK/LIGHT dragon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.thtg2)
    effect::set_target(d, e2, Some(cost_target(thtg2)));
    // e2:SetOperation(s.thop2)
    effect::set_operation(d, e2, Some(operation(thop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thcfilter(c)`
fn thcfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:GetOriginalLevel()>0 and c:IsAbleToGraveAsCost()
    //     and (c:IsFaceup() or not c:IsLocation(LOCATION_MZONE))
    card::is_race(d, c, RACE_DRAGON) && card::get_original_level(d, c) > 0 && card::is_able_to_grave_as_cost(d, c) && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_MZONE))
}
/// `s.thfilter(c,lv)`
fn thfilter(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:IsLevel(lv) and c:IsAbleToHand()
    card::is_race(d, c, RACE_DRAGON) && card::is_level(d, c, &[lv]) && card::is_able_to_hand(d, c, None)
}
/// `s.thcheck(sg,e,tp)`
fn thcheck(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return #sg>0 and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,sg:GetSum(Card.GetOriginalLevel))
    if sg.is_empty() {
        return (false, false);
    }
    let lv = group::get_sum(d, sg, &|d, c| card::get_original_level(d, c) as i64);
    (duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c, lv as u32)), tp, LOCATION_DECK, 0, 1, NO_EX), false)
}
/// `s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(s.thcfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&thcfilter), tp, LOCATION_HAND | LOCATION_MZONE, 0, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,1,#g,s.thcheck,0) end
    if a.chk == Some(0) {
        return matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(g.len()), Some(&thcheck), 0, tp, None, None, None, false), Err(true));
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,#g,s.thcheck,1,tp,HINTMSG_TOGRAVE,s.thcheck)
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(g.len()), Some(&thcheck), 1, tp, Some(HINTMSG_TOGRAVE as u64), Some(&thcheck), None, false)
        .ok()
        .nil();
    // e:SetLabel(sg:GetSum(Card.GetOriginalLevel))
    let sum = group::get_sum(d, &sg, &|d, c| card::get_original_level(d, c) as i64);
    effect::set_label(d, e, &[sum]);
    // Duel.SendtoGrave(sg,REASON_COST)
    duel::send_to_grave(d, &sg, REASON_COST, None, None);
    true
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,e:GetLabel())
    let lv = effect::get_label(d, a.e);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| thfilter(d, c, lv as u32)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
/// `s.thfilter2(c)`
fn thfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:IsAttribute(ATTRIBUTE_LIGHT+ATTRIBUTE_DARK) and c:IsLevel(8) and c:IsAbleToHand()
    card::is_race(d, c, RACE_DRAGON) && card::is_attribute(d, c, ATTRIBUTE_LIGHT + ATTRIBUTE_DARK) && card::is_level(d, c, &[8]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.thfilter2(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && thfilter2(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter2,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter2), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter2,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&thfilter2), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, 0, 0);
    true
}
/// `s.thop2(e,tp,eg,ep,ev,re,r,rp)`
fn thop2(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
