//! Altitude Knight
use super::prelude::*;

pub const ID: u32 = 27632240;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // send to grave
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e1, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e1:SetCode(EVENT_TO_HAND)
    effect::set_code(d, e1, EVENT_TO_HAND);
    // e1:SetCondition(s.tgcon1)
    effect::set_condition(d, e1, Some(condition(tgcon1)));
    // e1:SetTarget(s.tgtg)
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    // e1:SetOperation(s.tgop)
    effect::set_operation(d, e1, Some(operation(tgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_TO_DECK)
    effect::set_code(d, e2, EVENT_TO_DECK);
    // e2:SetCondition(s.tgcon2)
    effect::set_condition(d, e2, Some(condition(tgcon2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.cfilter1(c)
fn cfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPreviousLocation(LOCATION_MZONE)
    card::is_previous_location(d, c, LOCATION_MZONE)
}
// function s.tgcon1(e,tp,eg,ep,ev,re,r,rp)
fn tgcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter1,1,nil)
    group::is_exists(d, &a.eg.clone().nil(), &cfilter1, 1, NO_EX)
}
// function s.cfilter2(c)
fn cfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsLocation(LOCATION_DECK)
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_location(d, c, LOCATION_DECK)
}
// function s.tgcon2(e,tp,eg,ep,ev,re,r,rp)
fn tgcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter2,1,nil)
    group::is_exists(d, &a.eg.clone().nil(), &cfilter2, 1, NO_EX)
}
// function s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.tgfilter(c)
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAttribute(ATTRIBUTE_WIND) and c:IsAbleToGrave()
    card::is_monster(d, c) && card::is_attribute(d, c, ATTRIBUTE_WIND) && card::is_able_to_grave(d, c, None)
}
// function s.tgop(e,tp,eg,ep,ev,re,r,rp)
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFacedown() or not c:IsRelateToEffect(e) then return end
    if card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, &g, REASON_EFFECT, None, None);
    }
}
