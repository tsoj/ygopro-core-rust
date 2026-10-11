//! Fire Formation - Yoko
use super::prelude::*;

pub const ID: u32 = 36499284;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Increase ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsRace,RACE_BEASTWARRIOR))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_BEASTWARRIOR))));
    // e2:SetValue(100)
    effect::set_value(d, e2, 100);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.disfilter`
fn disfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_BEASTWARRIOR) and c:IsDiscardable(REASON_EFFECT)
    card::is_race(d, c, RACE_BEASTWARRIOR) && card::is_discardable(d, c, Some(REASON_EFFECT), None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsOnField() and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_on_field(d, chkc) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // if Duel.IsExistingTarget(Card.IsFaceup,tp,0,LOCATION_ONFIELD,1,nil)
    //   and Duel.IsExistingMatchingCard(s.disfilter,tp,LOCATION_HAND,0,1,nil)
    //   and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&disfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0))
    {
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, a.e, CATEGORY_DESTROY);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        // e:SetOperation(s.activate)
        effect::set_operation(d, a.e, Some(operation(activate)));
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectTarget(tp,Card.IsFaceup,tp,0,LOCATION_ONFIELD,1,1,nil)
        let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    } else {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
        // e:SetOperation(nil)
        effect::set_operation(d, a.e, None);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::get_first_target(d).first().copied().nil();
    // if Duel.DiscardHand(tp,s.disfilter,1,1,REASON_EFFECT|REASON_DISCARD)>0 and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if duel::discard_hand(d, a.tp, Some(&disfilter), 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX) > 0
        && card::is_relate_to_effect(d, tc, a.e)
        && card::is_faceup(d, tc)
    {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
