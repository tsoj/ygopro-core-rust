//! Xyz Wrath
use super::prelude::*;

pub const ID: u32 = 99064191;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target1)
    effect::set_target(d, e1, Some(cost_target(target1)));
    // e1:SetOperation(s.activate1)
    effect::set_operation(d, e1, Some(operation(activate1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // instant(chain)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetCondition(s.condition2)
    effect::set_condition(d, e2, Some(condition(condition2)));
    // e2:SetCost(s.cost2)
    effect::set_cost(d, e2, Some(cost_target(cost2)));
    // e2:SetTarget(s.target2)
    effect::set_target(d, e2, Some(cost_target(target2)));
    // e2:SetOperation(s.activate2)
    effect::set_operation(d, e2, Some(operation(activate2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(c) return c:IsFaceup() and c:IsType(TYPE_XYZ) end` (aux.FaceupFilter(Card.IsType,TYPE_XYZ))
fn faceup_xyz(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ)
}
/// `s.target1(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target1(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // e:SetLabel(0)
    effect::set_label(d, e, &[0]);
    // local ct=Duel.GetCurrentChain()
    let ct = duel::get_current_chain(d, false);
    // if ct==1 or not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,TYPE_XYZ),tp,LOCATION_MZONE,0,1,nil)
    //     or not Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil) then return false end
    if ct == 1
        || !duel::is_existing_matching_card(d, Some(&faceup_xyz), tp, LOCATION_MZONE, 0, 1, NO_EX)
        || !duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), tp, LOCATION_HAND, 0, 1, NO_EX)
    {
        return false;
    }
    // local pe=Duel.GetChainInfo(ct-1,CHAININFO_TRIGGERING_EFFECT)
    let pe = duel::get_chain_info(d, (ct - 1) as u8, &[CHAININFO_TRIGGERING_EFFECT])[0].as_effect().nil();
    // local tc=pe:GetHandler()
    let tc = effect::get_handler(d, pe).nil();
    // if pe:IsActiveType(TYPE_EFFECT) and tc:IsLevelAbove(5)
    //     and Duel.IsChainDisablable(ct-1) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if effect::is_active_type(d, pe, TYPE_EFFECT)
        && card::is_level_above(d, tc, 5)
        && duel::is_chain_disablable(d, (ct - 1) as u8)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
        duel::discard_hand(d, tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_DISABLE,tc,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(CardSet::from_card(tc)), 1, 0, 0);
        // if tc:IsRelateToEffect(pe) then
        if card::is_relate_to_effect(d, tc, pe) {
            // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,0,0)
            duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(tc)), 1, 0, 0);
        }
        // e:SetLabel(1)
        effect::set_label(d, e, &[1]);
        // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        let h = effect::get_handler(d, e).nil();
        card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
    // (falls off the end when chk ~= 0: the value is not used)
    true
}
/// `s.activate1(e,tp,eg,ep,ev,re,r,rp)`
fn activate1(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if e:GetLabel()~=1 or not c:IsRelateToEffect(e) then return end
    if effect::get_label(d, e) != 1 || !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local ct=Duel.GetCurrentChain()
    let ct = duel::get_current_chain(d, false);
    // local te=Duel.GetChainInfo(ct-1,CHAININFO_TRIGGERING_EFFECT)
    let te = duel::get_chain_info(d, (ct - 1) as u8, &[CHAININFO_TRIGGERING_EFFECT])[0].as_effect().nil();
    // local tc=te:GetHandler()
    let tc = effect::get_handler(d, te).nil();
    // if Duel.NegateEffect(ct-1) and tc:IsRelateToEffect(te) then
    if duel::negate_effect(d, (ct - 1) as u8) && card::is_relate_to_effect(d, tc, te) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
    }
}
/// `s.condition2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn condition2(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,TYPE_XYZ),tp,LOCATION_MZONE,0,1,nil)
    //     and re:IsActiveType(TYPE_EFFECT) and re:GetHandler():IsLevelAbove(5) and Duel.IsChainDisablable(ev)
    let re = a.re.nil();
    duel::is_existing_matching_card(d, Some(&faceup_xyz), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
        && effect::is_active_type(d, re, TYPE_EFFECT)
        && {
            let rc = effect::get_handler(d, re).nil();
            card::is_level_above(d, rc, 5)
        }
        && duel::is_chain_disablable(d, a.ev as u8)
}
/// `s.cost2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, a.tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.target2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    // if chk==0 then return e:GetHandler():GetFlagEffect(id)==0 end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, e).nil();
        return card::get_flag_effect(d, h, ID) == 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsDestructable() and re:GetHandler():IsRelateToEffect(re) then
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
    let h = effect::get_handler(d, e).nil();
    card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    true
}
/// `s.activate2(e,tp,eg,ep,ev,re,r,rp)`
fn activate2(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, h, e) {
        return;
    }
    // if Duel.NegateEffect(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_effect(d, a.ev as u8) {
        let re = a.re.nil();
        let rc = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rc, re) {
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
