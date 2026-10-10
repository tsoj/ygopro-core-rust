//! Simorgh, Bird of Divinity
use super::prelude::*;

pub const ID: u32 = 14989021;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // tribute limit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_TRIBUTE_LIMIT)
    effect::set_code(d, e2, EFFECT_TRIBUTE_LIMIT);
    // e2:SetValue(s.tlimit)
    effect::set_value_fn(d, e2, effect_card_fn(tlimit));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // damage
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.damtg)
    effect::set_target(d, e3, Some(cost_target(damtg)));
    // e3:SetOperation(s.damop)
    effect::set_operation(d, e3, Some(operation(damop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.tlimit(e,c)`
fn tlimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsAttribute(ATTRIBUTE_WIND)
    !card::is_attribute(d, c, ATTRIBUTE_WIND)
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,PLAYER_ALL,0)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, PLAYER_ALL as u8, 0);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local c1=Duel.GetMatchingGroupCount(Card.IsSpellTrap,tp,LOCATION_ONFIELD,0,nil)
    let c1 = duel::get_matching_group_count(d, Some(&|d, c| card::is_spell_trap(d, c)), a.tp, LOCATION_ONFIELD, 0, NO_EX) as i64;
    // if c1<2 then Duel.Damage(tp,1000-c1*500,REASON_EFFECT) end
    if c1 < 2 {
        duel::damage(d, a.tp, 1000 - c1 * 500, REASON_EFFECT, false, None);
    }
    // local c2=Duel.GetMatchingGroupCount(Card.IsSpellTrap,1-tp,LOCATION_ONFIELD,0,nil)
    let c2 = duel::get_matching_group_count(d, Some(&|d, c| card::is_spell_trap(d, c)), 1 - a.tp, LOCATION_ONFIELD, 0, NO_EX) as i64;
    // if c2<2 then Duel.Damage(1-tp,1000-c2*500,REASON_EFFECT) end
    if c2 < 2 {
        duel::damage(d, 1 - a.tp, 1000 - c2 * 500, REASON_EFFECT, false, None);
    }
}
