//! Horn of Light
use super::prelude::*;

pub const ID: u32 = 38552107;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // Atk up
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // e2:SetValue(800)
    effect::set_value(d, e2, 800);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // to deck
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e4, CATEGORY_TODECK);
    // e4:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e4, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e4, EVENT_TO_GRAVE);
    // e4:SetCondition(s.tdcon)
    effect::set_condition(d, e4, Some(condition(tdcon)));
    // e4:SetCost(Cost.PayLP(500))
    effect::set_cost(d, e4, Some(cost::pay_lp(cost::PayLp::Amount(500))));
    // e4:SetTarget(s.tdtg)
    effect::set_target(d, e4, Some(cost_target(tdtg)));
    // e4:SetOperation(s.tdop)
    effect::set_operation(d, e4, Some(operation(tdop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.tdcon(e,tp,eg,ep,ev,re,r,rp)`
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD)
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_ONFIELD)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsAbleToDeck() end
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_deck(d, h);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        // Duel.SendtoDeck(e:GetHandler(),nil,SEQ_DECKTOP,REASON_EFFECT)
        duel::send_to_deck(d, h, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    }
}
