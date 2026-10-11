//! Brotherhood of the Fire Fist - Cardinal
use super::prelude::*;

pub const ID: u32 = 58504745;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FIRE_FIST, SET_FIRE_FORMATION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_FIRE_FIST),4,2)
    xyz::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_FIRE_FIST as u16], sc, st, Some(tp)))), Some(4), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // todeck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCost(Cost.DetachFromSelf(2))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(2), None, None)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter1(c)`
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsLocation(LOCATION_GRAVE) or c:IsFaceup()) and (c:IsSetCard(SET_FIRE_FIST) or c:IsSetCard(SET_FIRE_FORMATION)) and c:IsAbleToDeck()
    (card::is_location(d, c, LOCATION_GRAVE) || card::is_faceup(d, c))
        && (card::is_set_card(d, c, &[SET_FIRE_FIST as u16]) || card::is_set_card(d, c, &[SET_FIRE_FORMATION as u16]))
        && card::is_able_to_deck(d, c)
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsLocation(LOCATION_GRAVE) or c:IsFaceup()) and c:IsAbleToDeck()
    (card::is_location(d, c, LOCATION_GRAVE) || card::is_faceup(d, c)) && card::is_able_to_deck(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter1,tp,LOCATION_GRAVE|LOCATION_ONFIELD,0,2,nil)
    //     and Duel.IsExistingTarget(s.filter2,tp,0,LOCATION_GRAVE|LOCATION_ONFIELD,2,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter1), tp, LOCATION_GRAVE | LOCATION_ONFIELD, 0, 2, NO_EX)
            && duel::is_existing_target(d, Some(&filter2), tp, 0, LOCATION_GRAVE | LOCATION_ONFIELD, 2, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g1=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_GRAVE|LOCATION_ONFIELD,0,2,2,nil)
    let mut g1 = duel::select_target(d, tp, Some(&filter1), tp, LOCATION_GRAVE | LOCATION_ONFIELD, 0, 2, 2, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g2=Duel.SelectTarget(tp,s.filter2,tp,0,LOCATION_GRAVE|LOCATION_ONFIELD,2,2,nil)
    let g2 = duel::select_target(d, tp, Some(&filter2), tp, 0, LOCATION_GRAVE | LOCATION_ONFIELD, 2, 2, false, NO_EX).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g1,4,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g1), 4, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).unwrap_or_default();
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
}
