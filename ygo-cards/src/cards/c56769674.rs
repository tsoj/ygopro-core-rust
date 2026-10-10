//! DNA Transplant
use super::prelude::*;

pub const ID: u32 = 56769674;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_DRAW_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_DRAW_PHASE));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // race
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetCode(EFFECT_CHANGE_ATTRIBUTE)
    effect::set_code(d, e2, EFFECT_CHANGE_ATTRIBUTE);
    // e2:SetValue(s.value)
    effect::set_value_fn(d, e2, effect_card_value(value));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // e1:SetLabelObject(e2)
    effect::set_label_object(d, e1, Value::Effect(e2));
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTRIBUTE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATTRIBUTE as u64);
    // local rc=Duel.AnnounceAttribute(tp,1,ATTRIBUTE_ALL)
    let rc = duel::announce_attribute(d, a.tp, 1, ATTRIBUTE_ALL);
    // e:GetLabelObject():SetLabel(rc)
    let race = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_label(d, race, &[rc as i64]);
    // e:GetHandler():SetHint(CHINT_ATTRIBUTE,rc)
    let c = effect::get_handler(d, a.e).nil();
    card::set_hint(d, c, CHINT_ATTRIBUTE as u8, rc as u64);
    true
}
/// `s.value(e,c)`
fn value(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // return e:GetLabel()
    effect::get_label(d, e)
}
