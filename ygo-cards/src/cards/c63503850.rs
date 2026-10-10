//! Brute Enforcer
use super::prelude::*;

pub const ID: u32 = 63503850;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),2,2)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))),
        2,
        Some(2),
        None,
        None,
        None,
    );
    // destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD,nil)
    duel::discard_hand(d, a.tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsOnField() and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_on_field(d, chkc) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,Card.IsFaceup,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
/// `s.cfilter(c,ty)`
fn cfilter(d: &mut Duel, c: CardId, ty: u32) -> bool {
    // return c:GetOriginalType() & ty ~=0 and c:IsDiscardable(REASON_EFFECT)
    (card::get_original_type(d, c) & ty) != 0 && card::is_discardable(d, c, Some(REASON_EFFECT), None)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    // if not (tc:IsRelateToEffect(e) and tc:IsFaceup()) then return end
    let tc = duel::first_target(d).nil();
    if !(card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc)) {
        return;
    }
    // if Duel.IsChainDisablable(0) then
    if duel::is_chain_disablable(d, 0) {
        // local g=Duel.GetMatchingGroup(s.cfilter,tp,0,LOCATION_HAND,nil,tc:GetOriginalType()&(TYPE_MONSTER|TYPE_SPELL|TYPE_TRAP))
        let ty = card::get_original_type(d, tc) & (TYPE_MONSTER | TYPE_SPELL | TYPE_TRAP);
        let g = duel::get_matching_group(d, Some(&|d, c| cfilter(d, c, ty)), tp, 0, LOCATION_HAND, NO_EX);
        // if #g>0 and Duel.SelectYesNo(1-tp,aux.Stringid(id,1)) then
        if !g.is_empty() && duel::select_yes_no(d, 1 - tp, aux::stringid(ID, 1)) {
            // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_DISCARD)
            duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_DISCARD as u64);
            // local sg=g:Select(1-tp,1,1,nil)
            let sg = group::select(d, &g, 1 - tp, 1, 1, false, NO_EX).nil();
            // Duel.SendtoGrave(sg,REASON_EFFECT|REASON_DISCARD)
            duel::send_to_grave(d, sg, REASON_EFFECT | REASON_DISCARD, None, None);
            // return
            return;
        }
    }
    // Duel.Destroy(tc,REASON_EFFECT)
    duel::destroy(d, tc, REASON_EFFECT, None, None);
}
