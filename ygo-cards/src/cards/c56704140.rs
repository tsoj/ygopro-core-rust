//! Geomancer of the Ice Barrier
use super::prelude::*;

pub const ID: u32 = 56704140;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Make this card untargetable for attacks
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_NO_TURN_RESET)
    effect::set_property(d, e1, EFFECT_FLAG_NO_TURN_RESET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(aux.IceBarrierDiscardCost(nil,true))
    effect::set_cost(d, e1, Some(aux::ice_barrier_discard_cost(None, true, None, None)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local attr=Duel.AnnounceAttribute(tp,1,ATTRIBUTE_ALL)
    let attr = duel::announce_attribute(d, a.tp, 1, ATTRIBUTE_ALL);
    // e:SetLabel(attr)
    effect::set_label(d, a.e, &[attr as i64]);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFacedown() or not c:IsRelateToEffect(e) then return end
    if !card::is_faceup(d, c) || !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local attr=e:GetLabel()
    let attr = effect::get_label(d, a.e);
    // c:SetHint(CHINT_ATTRIBUTE,attr)
    card::set_hint(d, c, CHINT_ATTRIBUTE as u8, attr as u64);
    // Cannot be targeted for attacks
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_BATTLE_TARGET);
    // e1:SetLabel(attr)
    effect::set_label(d, e1, &[attr]);
    // e1:SetValue(s.tgval)
    effect::set_value_fn(d, e1, effect_card_fn(tgval));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tgval(e,c)`
fn tgval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsAttribute(e:GetLabel()) and not c:IsImmuneToEffect(e)
    card::is_attribute(d, c, effect::get_label(d, e) as u32) && !card::is_immune_to_effect(d, c, e)
}
