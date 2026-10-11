//! Emperor Sem
use super::prelude::*;

pub const ID: u32 = 21672573;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCondition(s.tdcon1)
    effect::set_condition(d, e1, Some(condition(tdcon1)));
    // e1:SetTarget(s.tdtg)
    effect::set_target(d, e1, Some(cost_target(tdtg)));
    // e1:SetOperation(s.tdop)
    effect::set_operation(d, e1, Some(operation(tdop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_MSET)
    effect::set_code(d, e2, EVENT_MSET);
    // e2:SetCondition(s.tdcon2)
    effect::set_condition(d, e2, Some(condition(tdcon2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.tdcon1(e,tp,eg,ep,ev,re,r,rp)`
fn tdcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:GetFirst()~=e:GetHandler() and ep==tp and eg:GetFirst():IsTributeSummoned()
    let eg = a.eg.clone().nil();
    let first = group::get_first(d, &eg);
    let h = effect::get_handler(d, a.e);
    first != h && a.ep == a.tp && card::is_tribute_summoned(d, first.nil())
}
/// `s.tdcon2(e,tp,eg,ep,ev,re,r,rp)`
fn tdcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:GetFirst():GetMaterialCount()~=0 and ep==tp
    let eg = a.eg.clone().nil();
    let first = group::get_first(d, &eg).nil();
    card::get_material_count(d, first) != 0 && a.ep == a.tp
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToDeck,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_deck(d, c)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g1=Duel.SelectTarget(tp,Card.IsAbleToDeck,tp,LOCATION_GRAVE,0,1,1,nil)
    let mut g1 = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_deck(d, c)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if Duel.IsExistingTarget(Card.IsAbleToDeck,tp,0,LOCATION_GRAVE,1,nil) and Duel.SelectYesNo(1-tp,aux.Stringid(id,1)) then
    if duel::is_existing_target(d, Some(&|d, c| card::is_able_to_deck(d, c)), a.tp, 0, LOCATION_GRAVE, 1, NO_EX)
        && duel::select_yes_no(d, 1 - a.tp, aux::stringid(ID, 1))
    {
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_TODECK as u64);
        // local g2=Duel.SelectTarget(1-tp,Card.IsAbleToDeck,1-tp,LOCATION_GRAVE,0,1,1,nil)
        let g2 = duel::select_target(d, 1 - a.tp, Some(&|d, c| card::is_able_to_deck(d, c)), 1 - a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // g1:Merge(g2)
        group::add_card(&mut g1, &g2);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g1,#g1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g1.clone()), g1.len() as u8, 0, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) or e:GetHandler():IsFacedown() then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) || card::is_facedown(d, h) {
        return;
    }
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // Duel.SendtoDeck(g,nil,SEQ_DECKTOP,REASON_EFFECT)
    duel::send_to_deck(d, g, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
}
