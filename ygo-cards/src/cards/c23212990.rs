//! Six Strike - Thunder Blast
use super::prelude::*;

pub const ID: u32 = 23212990;

pub const INFO: ScriptInfo = ScriptInfo { counter_list: &[COUNTER_BUSHIDO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsCanRemoveCounter(tp,1,0,COUNTER_BUSHIDO,1,REASON_COST) end
    if a.chk == Some(0) {
        return duel::is_can_remove_counter(d, a.tp, 1, 0, COUNTER_BUSHIDO as u16, 1, REASON_COST);
    }
    // Duel.RemoveCounter(tp,1,0,COUNTER_BUSHIDO,1,REASON_COST)
    duel::remove_counter(d, a.tp, 1, 0, COUNTER_BUSHIDO as u16, 1, REASON_COST);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local label=e:GetLabel()
    let label = effect::get_label(d, a.e);
    // if chkc then return chkc:IsControler(1-tp) and ((label==1 and chkc:IsLocation(LOCATION_MZONE))
    //   or (label==2 and chkc:IsOnField() and chkc:IsAbleToHand())) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp)
            && ((label == 1 && card::is_location(d, chkc, LOCATION_MZONE))
                || (label == 2 && card::is_on_field(d, chkc) && card::is_able_to_hand(d, chkc, None)));
    }
    // local b1=Duel.IsExistingTarget(nil,tp,0,LOCATION_MZONE,1,nil)
    let b1 = duel::is_existing_target(d, None, tp, 0, LOCATION_MZONE, 1, NO_EX);
    // local b2=Duel.IsExistingTarget(Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,1,nil)
    let b2 = duel::is_existing_target(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,1)}, {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).nil();
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    if op == 1 {
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, a.e, CATEGORY_DESTROY);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectTarget(tp,nil,tp,0,LOCATION_MZONE,1,1,nil)
        let g = duel::select_target(d, tp, None, tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    } else {
        // e:SetCategory(CATEGORY_TOHAND)
        effect::set_category(d, a.e, CATEGORY_TOHAND);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
        // local g=Duel.SelectTarget(tp,Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,1,1,nil)
        let g = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, 0, 0);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and tc:IsControler(1-tp)) then return end
    if !(card::is_relate_to_effect(d, tc, a.e) && card::is_controler(d, tc, 1 - a.tp)) {
        return;
    }
    if effect::get_label(d, a.e) == 1 {
        // Destroy 1 monster your opponent controls
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    } else {
        // Return 1 card your opponent controls to the hand
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
