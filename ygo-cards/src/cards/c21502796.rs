//! Ryko, Lightsworn Hunter
use super::prelude::*;

pub const ID: u32 = 21502796;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DECKDES+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DECKDES + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,3)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, a.tp, 3);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,LOCATION_ONFIELD,LOCATION_ONFIELD)
    let g = duel::get_field_group(d, tp, LOCATION_ONFIELD, LOCATION_ONFIELD);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.HintSelection(sg)
        duel::hint_selection(d, sg.clone(), true);
        // Duel.Destroy(sg,REASON_EFFECT)
        duel::destroy(d, sg, REASON_EFFECT, None, None);
    }
    // Duel.DiscardDeck(tp,3,REASON_EFFECT)
    duel::discard_deck(d, tp, 3, REASON_EFFECT);
}
