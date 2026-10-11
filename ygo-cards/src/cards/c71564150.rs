//! Perditious Puppeteer
use super::prelude::*;

pub const ID: u32 = 71564150;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Take control all of opponent's level 3 or lower monsters
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_CONTROL);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, a.tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsLevelBelow(3) and c:IsControlerCanBeChanged()
    card::is_faceup(d, c) && card::is_level_below(d, c, 3) && card::is_controler_can_be_changed(d, c, false, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,nil,1,1-tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,0,LOCATION_MZONE,ft,ft,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, 0, LOCATION_MZONE, ft as u16, ft as u16, false, NO_EX).nil();
    // Duel.GetControl(g,tp,PHASE_END,1)
    duel::get_control(d, g, a.tp, PHASE_END as u16, 1, None, None);
    // local og=Duel.GetOperatedGroup()
    let og = duel::get_operated_group(d);
    // local tc=og:GetFirst()
    let _ = group::get_first(d, &og);
    // for tc in aux.Next(og) do
    for tc in group::iter(d, &og) {
        // Cannot be tributed
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3303)
        effect::set_description(d, e1, 3303);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UNRELEASABLE_SUM)
        effect::set_code(d, e1, EFFECT_UNRELEASABLE_SUM);
        // e1:SetReset(RESET_EVENT|(RESETS_STANDARD_PHASE_END&~RESET_TURN_SET))
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TURN_SET), 1);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_UNRELEASABLE_NONSUM)
        effect::set_code(d, e2, EFFECT_UNRELEASABLE_NONSUM);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // Cannot activate their effects
        // local e3=e1:Clone()
        let e3 = effect::clone(d, e1);
        // e3:SetDescription(3302)
        effect::set_description(d, e3, 3302);
        // e3:SetCode(EFFECT_CANNOT_TRIGGER)
        effect::set_code(d, e3, EFFECT_CANNOT_TRIGGER);
        // tc:RegisterEffect(e3)
        card::register_effect(d, tc, e3, false);
        // Cannot be used as synchro material
        // local e4=e1:Clone()
        let e4 = effect::clone(d, e1);
        // e4:SetDescription(3310)
        effect::set_description(d, e4, 3310);
        // e4:SetCode(EFFECT_CANNOT_BE_SYNCHRO_MATERIAL)
        effect::set_code(d, e4, EFFECT_CANNOT_BE_SYNCHRO_MATERIAL);
        // tc:RegisterEffect(e4)
        card::register_effect(d, tc, e4, false);
    }
}
