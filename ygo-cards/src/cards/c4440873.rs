//! Drastic Drop Off
use super::prelude::*;

pub const ID: u32 = 4440873;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_HANDES);
    // e1:SetCode(EVENT_TO_HAND)
    effect::set_code(d, e1, EVENT_TO_HAND);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsPreviousLocation(LOCATION_DECK)
    card::is_controler(d, c, tp) && card::is_previous_location(d, c, LOCATION_DECK)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,1-tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, 1 - a.tp), 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, a.eg.clone().nil());
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,1-tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - a.tp, 1);
    true
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRelateToEffect(e) and c:IsControler(tp) and c:IsPreviousLocation(LOCATION_DECK)
    card::is_relate_to_effect(d, c, e) && card::is_controler(d, c, tp) && card::is_previous_location(d, c, LOCATION_DECK)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local sg=eg:Filter(s.filter,nil,e,1-tp)
    let sg = group::filter(d, &a.eg(), &|d, c| filter(d, c, a.e, 1 - a.tp), NO_EX);
    // if #sg==0 then
    if sg.is_empty() {
    // elseif #sg==1 then
    } else if sg.len() == 1 {
        // Duel.SendtoGrave(sg,REASON_EFFECT|REASON_DISCARD)
        duel::send_to_grave(d, sg, REASON_EFFECT | REASON_DISCARD, None, None);
    } else {
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_DISCARD)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_DISCARD as u64);
        // local dg=sg:Select(1-tp,1,1,nil)
        let dg = group::select(d, &sg, 1 - a.tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoGrave(dg,REASON_EFFECT|REASON_DISCARD)
        duel::send_to_grave(d, dg, REASON_EFFECT | REASON_DISCARD, None, None);
    }
}
