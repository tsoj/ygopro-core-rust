//! Hyper Hammerhead
use super::prelude::*;

pub const ID: u32 = 2671330;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // return hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_DAMAGE_STEP_END);
    effect::set_condition(d, e1, Some(condition(retcon)));
    effect::set_target(d, e1, Some(cost_target(rettg)));
    effect::set_operation(d, e1, Some(operation(retop)));
    card::register_effect(d, c, e1, false);
}
/// `s.retcon(e,...)`
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local t=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let t = card::get_battle_target(d, h);
    // e:SetLabelObject(t)
    effect::set_label_object(d, a.e, t.map_or(Value::Nil, Value::Card));
    // return t and t:IsRelateToBattle()
    match t {
        Some(t) => card::is_relate_to_battle(d, t),
        None => false,
    }
}
/// `s.rettg(e,...,chk)`
fn rettg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,e:GetLabelObject(),1,0,0)
    let t = effect::get_label_object(d, a.e).as_card();
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, t.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.retop(e,...)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // if e:GetLabelObject():IsRelateToBattle() then
    let t = effect::get_label_object(d, a.e).as_card().nil();
    if card::is_relate_to_battle(d, t) {
        // Duel.SendtoHand(e:GetLabelObject(),nil,REASON_EFFECT)
        duel::send_to_hand(d, t, None, REASON_EFFECT, None);
    }
}
