//! Rescueroid
use super::prelude::*;

pub const ID: u32 = 24311595;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ROID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsLocation(LOCATION_GRAVE) and c:IsReason(REASON_BATTLE) and c:IsPreviousControler(tp)
    //     and c:IsSetCard(SET_ROID) and c:IsAbleToHand()
    card::is_location(d, c, LOCATION_GRAVE)
        && card::is_reason(d, c, REASON_BATTLE)
        && card::is_previous_controler(d, c, tp)
        && card::is_set_card(d, c, &[SET_ROID as u16])
        && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=eg:Filter(s.filter,nil,tp)
        let eg = a.eg.clone().nil();
        let g = group::filter(d, &eg, &|d, c| filter(d, c, a.tp), NO_EX);
        // e:SetLabelObject(g:GetFirst())
        let first = group::get_first(d, &g);
        effect::set_label_object(d, a.e, first.map_or(Value::Nil, Value::Card));
        // return #g~=0
        return !g.is_empty();
    }
    // Duel.SetTargetCard(e:GetLabelObject())
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    duel::set_target_card(d, tc);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,e:GetLabelObject(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(tc)), 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if e:GetHandler():IsRelateToEffect(e) and tc:IsRelateToEffect(e) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        let tc = duel::first_target(d).nil();
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SendtoHand(tc,nil,REASON_EFFECT)
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        }
    }
}
