//! Crimson Ninja
use super::prelude::*;

pub const ID: u32 = 14618326;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy 1 Set Trap card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`: `return c:IsTrap() or (c:IsFacedown() and c:IsLocation(LOCATION_SZONE) and c:GetSequence()~=5)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_trap(d, c) || (card::is_facedown(d, c) && card::is_location(d, c, LOCATION_SZONE) && card::get_sequence(d, c) != 5)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && filter(d, chkc);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // if #g>0 and g:GetFirst():IsFaceup() then
    if !g.is_empty() {
        let first = group::get_first(d, &g).nil();
        if card::is_faceup(d, first) {
            // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
            duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
        }
    }
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // if tc:IsFacedown() then Duel.ConfirmCards(tp,tc) end
            if card::is_facedown(d, tc) {
                duel::confirm_cards(d, a.tp, tc);
            }
            // if tc:IsTrap() then Duel.Destroy(tc,REASON_EFFECT) end
            if card::is_trap(d, tc) {
                duel::destroy(d, tc, REASON_EFFECT, None, None);
            }
        }
    }
}
