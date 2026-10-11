//! Disenchanter
use super::prelude::*;

pub const ID: u32 = 76137614;

pub const INFO: ScriptInfo = ScriptInfo { counter_list: &[COUNTER_SPELL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Remove 1 Spell Counter from anywhere on the field, and if you do, return 1 face-up Spell on the field to the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.rthtg)
    effect::set_target(d, e1, Some(cost_target(rthtg)));
    // e1:SetOperation(s.rthop)
    effect::set_operation(d, e1, Some(operation(rthop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.rthfilter(c)
fn rthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpell() and c:IsFaceup() and c:IsAbleToHand()
    card::is_spell(d, c) && card::is_faceup(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.rthtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and s.rthfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && rthfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsCanRemoveCounter(tp,1,1,COUNTER_SPELL,1,REASON_EFFECT)
    //     and Duel.IsExistingTarget(s.rthfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_can_remove_counter(d, a.tp, 1, 1, COUNTER_SPELL as u16, 1, REASON_EFFECT)
            && duel::is_existing_target(d, Some(&rthfilter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.rthfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&rthfilter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, a.tp, 0);
    true
}
/// `s.rthop(e,tp,eg,ep,ev,re,r,rp)`
fn rthop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if Duel.RemoveCounter(tp,1,1,COUNTER_SPELL,1,REASON_EFFECT) and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if duel::remove_counter(d, a.tp, 1, 1, COUNTER_SPELL as u16, 1, REASON_EFFECT) {
        let tc = tc.nil();
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            // Duel.SendtoHand(tc,nil,REASON_EFFECT)
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        }
    }
}
