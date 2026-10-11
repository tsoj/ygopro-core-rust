//! Infernity Queen
//! インフェルニティ・クイーン
use super::prelude::*;

pub const ID: u32 = 53246495;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Make 1 DARK monster you control able to attack directly
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp) return Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)==0 and Duel.IsAbleToEnterBP() end)
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    // e1:SetCost(s.diratkcost)
    effect::set_cost(d, e1, Some(cost_target(diratkcost)));
    // e1:SetTarget(s.diratktg)
    effect::set_target(d, e1, Some(cost_target(diratktg)));
    // e1:SetOperation(s.diratkop)
    effect::set_operation(d, e1, Some(operation(diratkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `function(e,tp) return Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)==0 and Duel.IsAbleToEnterBP() end`
fn e1_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) == 0 && duel::is_able_to_enter_bp(d)
}
/// `s.diratkcostfilter(c)`
fn diratkcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_attribute(d, c, ATTRIBUTE_DARK)
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, false)
}
/// `s.diratkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn diratkcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.diratkcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&diratkcostfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.diratkcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&diratkcostfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.diratktgfilter(c)`
fn diratktgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK) and c:IsFaceup() and not c:IsHasEffect(EFFECT_DIRECT_ATTACK)
    card::is_attribute(d, c, ATTRIBUTE_DARK)
        && card::is_faceup(d, c)
        && card::is_has_effect(d, c, EFFECT_DIRECT_ATTACK, None).is_empty()
}
/// `s.diratktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn diratktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.diratktgfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && diratktgfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.diratktgfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&diratktgfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_APPLYTO)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_APPLYTO as u64);
    // Duel.SelectTarget(tp,s.diratktgfilter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&diratktgfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.diratkop(e,tp,eg,ep,ev,re,r,rp)`
fn diratkop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, e) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(3205)
        effect::set_description(d, e1, 3205);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_DIRECT_ATTACK)
        effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
