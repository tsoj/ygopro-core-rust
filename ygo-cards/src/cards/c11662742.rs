//! Gellenduo
//! ジェルエンデュオ
use super::prelude::*;

pub const ID: u32 = 11662742;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot be destroyed by battle
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy this face-up card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_DAMAGE)
    effect::set_code(d, e2, EVENT_DAMAGE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,0,EFFECT_COUNT_CODE_CHAIN)
    effect::set_count_limit(d, e2, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return ep==tp and r&(REASON_BATTLE|REASON_EFFECT)>0 end)
    effect::set_condition(d, e2, Some(condition(|_d, a| a.ep == a.tp && (a.r & (REASON_BATTLE | REASON_EFFECT)) > 0)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // This card can be treated as 2 Tributes for the Tribute Summon of a LIGHT Fairy monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_DOUBLE_TRIBUTE)
    effect::set_code(d, e3, EFFECT_DOUBLE_TRIBUTE);
    // e3:SetValue(function(e,c) return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_FAIRY) end)
    effect::set_value_fn(d, e3, effect_card_fn(|d, _e, c| card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_FAIRY as u64)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, h.map(CardSet::from_card), 1, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // Duel.Destroy(c,REASON_EFFECT)
        duel::destroy(d, c, REASON_EFFECT, None, None);
    }
}
