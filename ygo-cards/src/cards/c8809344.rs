//! Outer Entity Nyarla
use super::prelude::*;

pub const ID: u32 = 8809344;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,nil,4,2)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // rankup
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.rkcon)
    effect::set_condition(d, e1, Some(condition(rkcon)));
    // e1:SetCost(s.rkcost)
    effect::set_cost(d, e1, Some(cost_target(rkcost)));
    // e1:SetOperation(s.rkop)
    effect::set_operation(d, e1, Some(operation(rkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // attach
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.rkcon(e,tp,eg,ep,ev,re,r,rp)`
fn rkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsXyzSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_xyz_summoned(d, h)
}
/// `s.rkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rkcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), tp, LOCATION_HAND as u32, 0, 1, NO_EX);
    }
    // local ct=Duel.DiscardHand(tp,Card.IsDiscardable,1,60,REASON_COST|REASON_DISCARD)
    let ct = duel::discard_hand(d, tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 60, REASON_COST | REASON_DISCARD, NO_EX);
    // e:SetLabel(ct)
    effect::set_label(d, a.e, &[ct as i64]);
    true
}
/// `s.rkop(e,tp,eg,ep,ev,re,r,rp)`
fn rkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_UPDATE_RANK)
        effect::set_code(d, e2, EFFECT_UPDATE_RANK);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // e2:SetValue(e:GetLabel())
        let label = effect::get_label(d, a.e);
        effect::set_value(d, e2, label);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsType(TYPE_XYZ) and e:GetHandler():GetOverlayCount()>0
    let h = effect::get_handler(d, a.e).nil();
    card::is_type(d, h, TYPE_XYZ) && card::get_overlay_count(d, h) > 0
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and chkc:IsMonster() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && card::is_monster(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsMonster,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&card::is_monster), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,Card.IsMonster,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&card::is_monster), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(g), 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_relate_to_effect(d, tc.nil(), a.e) {
        let tc = tc.nil();
        // local og=c:GetOverlayGroup()
        let og = card::get_overlay_group(d, c);
        // if #og==0 then return end
        if og.is_empty() {
            return;
        }
        // Duel.SendtoGrave(og,REASON_EFFECT)
        duel::send_to_grave(d, og, REASON_EFFECT, None, None);
        // Duel.Overlay(c,tc)
        duel::overlay(d, c, tc, false);
        // if c:IsFacedown() then return end
        if card::is_facedown(d, c) {
            return;
        }
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_COPY_INHERIT)
        effect::set_property(d, e1, EFFECT_FLAG_COPY_INHERIT, 0);
        // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
        effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
        // e1:SetValue(tc:GetOriginalAttribute())
        effect::set_value(d, e1, card::get_original_attribute(d, tc) as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_CHANGE_RACE)
        effect::set_code(d, e2, EFFECT_CHANGE_RACE);
        // e2:SetValue(tc:GetOriginalRace())
        effect::set_value(d, e2, card::get_original_race(d, tc) as i64);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
}
