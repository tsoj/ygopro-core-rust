//! Stray Asmodian
use super::prelude::*;

pub const ID: u32 = 86209650;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // battle destroyed
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e1, CATEGORY_RECOVER);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLocation(LOCATION_GRAVE) and e:GetHandler():IsReason(REASON_BATTLE)
    let c = effect::get_handler(d, a.e).nil();
    card::is_location(d, c, LOCATION_GRAVE) && card::is_reason(d, c, REASON_BATTLE)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,PLAYER_ALL,800)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, PLAYER_ALL as u8, 800);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // Duel.Recover(tp,800,REASON_EFFECT)
    duel::recover(d, a.tp, 800, REASON_EFFECT, false, None);
    // Duel.Recover(1-tp,800,REASON_EFFECT)
    duel::recover(d, 1 - a.tp, 800, REASON_EFFECT, false, None);
}
