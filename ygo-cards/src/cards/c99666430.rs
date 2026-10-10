//! Jinzo - Layered
use super::prelude::*;

pub const ID: u32 = 99666430;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,nil,6,2)
    xyz::add_procedure(d, c, None, Some(6), 2, None, None, None, None, false, None);
    // Take control of 1 face-up monster your opponent controls
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(s.ctrltg)
    effect::set_target(d, e1, Some(cost_target(ctrltg)));
    // e1:SetOperation(s.ctrlop)
    effect::set_operation(d, e1, Some(operation(ctrlop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Tribute 1 monster, and if you do, destroy 1 face-up card on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_RELEASE+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_RELEASE + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsTrap),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_trap(d, c)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
    })));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.ctrltg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn ctrltg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsFaceup() and chkc:IsControlerCanBeChanged() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && card::is_faceup(d, chkc) && card::is_controler_can_be_changed(d, chkc, false, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsControlerCanBeChanged),tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c) && card::is_controler_can_be_changed(d, c, false, None)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsControlerCanBeChanged),tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c) && card::is_controler_can_be_changed(d, c, false, None)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, a.tp, 0);
    true
}
/// `s.ctrlop(e,tp,eg,ep,ev,re,r,rp)`
fn ctrlop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.GetControl(tc,tp,PHASE_END,1)>0 then
    if card::is_relate_to_effect(d, tc, a.e) && duel::get_control(d, tc, a.tp, PHASE_END as u16, 1, None, None) > 0 {
        // It cannot activate its effects or declare an attack
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(3302)
        effect::set_description(d, e1, 3302);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_TRIGGER)
        effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESETS_STANDARD_PHASE_END|RESET_CONTROL)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END | RESET_CONTROL, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetDescription(3206)
        effect::set_description(d, e2, 3206);
        // e2:SetCode(EFFECT_CANNOT_ATTACK)
        effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
/// `s.tribfilter(c)`
fn tribfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsReleasableByEffect() and Duel.IsExistingMatchingCard(Card.IsFaceup,0,LOCATION_ONFIELD,LOCATION_ONFIELD,1,c+c:GetEquipGroup())
    card::is_releasable_by_effect(d, c, None, None) && {
        let mut g = card::get_equip_group(d, c);
        group::add_card(&mut g, &CardSet::from_card(c));
        duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x)), 0, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, (&g).into())
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroup(tp,s.tribfilter,1,nil) end
    if a.chk == Some(0) {
        return duel::check_release_group(d, a.tp, Some(&tribfilter), 1, duel::ReleaseArgs::default(), NO_EX, false);
    }
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local rg=Duel.SelectReleaseGroup(tp,s.tribfilter,1,1,nil)
    let rg = duel::select_release_group(d, a.tp, Some(&tribfilter), 1, 1, duel::ReleaseArgs::default(), NO_EX, false).nil();
    // if #rg>0 and Duel.Release(rg,REASON_EFFECT)>0 then
    if !rg.is_empty() && duel::release(d, rg, REASON_EFFECT, None) > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        // local dg=Duel.SelectMatchingCard(tp,Card.IsFaceup,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
        let dg = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        // Duel.HintSelection(dg)
        duel::hint_selection(d, dg.clone(), true);
        // Duel.Destroy(dg,REASON_EFFECT)
        duel::destroy(d, dg, REASON_EFFECT, None, None);
    }
}
