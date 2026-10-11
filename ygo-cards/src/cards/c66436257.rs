//! Volcanic Counter
use super::prelude::*;

pub const ID: u32 = 66436257;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`: `return ep==tp`
fn cond(_d: &mut Duel, a: &EffArgs) -> bool {
    a.ep == a.tp
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.filter(c)`: `return c:IsAttribute(ATTRIBUTE_FIRE) and c:GetCode()~=id`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_FIRE) && card::get_code(d, c).first().copied() != Some(ID)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // if not Duel.IsPlayerAffectedByEffect(e:GetHandlerPlayer(),CARD_SPIRIT_ELIMINATION)
    //     and Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_EFFECT)~=0
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_GRAVE,0,1,nil) then
    let hp = effect::get_handler_player(d, a.e);
    let affected = duel::is_player_affected_by_effect(d, hp, CARD_SPIRIT_ELIMINATION).is_some();
    if !affected {
        let h = effect::get_handler(d, a.e).nil();
        let removed = duel::remove(d, CardSet::from_card(h), POS_FACEUP as u8, REASON_EFFECT, None, None);
        if removed != 0 && duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX) {
            // Duel.Damage(1-tp,ev,REASON_EFFECT)
            duel::damage(d, 1 - a.tp, a.ev as i64, REASON_EFFECT, false, None);
        }
    }
}
