//! Reptilianne Servant
use super::prelude::*;

pub const ID: u32 = 16008155;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // self destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_SELF_DESTROY)
    effect::set_code(d, e1, EFFECT_SELF_DESTROY);
    // e1:SetCondition(s.sdcon)
    effect::set_condition(d, e1, Some(lua_fn(sdcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // be target
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_BECOME_TARGET)
    effect::set_code(d, e2, EVENT_BECOME_TARGET);
    // e2:SetOperation(s.desop1)
    effect::set_operation(d, e2, Some(operation(desop1)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e3, EVENT_CHAIN_SOLVED);
    // e3:SetOperation(s.desop2)
    effect::set_operation(d, e3, Some(operation(desop2)));
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCode(EVENT_BATTLED)
    effect::set_code(d, e4, EVENT_BATTLED);
    // e4:SetOperation(s.desop3)
    effect::set_operation(d, e4, Some(operation(desop3)));
    // e4:SetLabelObject(e2)
    effect::set_label_object(d, e4, Value::Effect(e2));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // cannot summon
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    // e5:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e5, EFFECT_CANNOT_SUMMON);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e5, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e5:SetTargetRange(1,1)
    effect::set_target_range(d, e5, 1, 1);
    // e5:SetValue(1)
    effect::set_value(d, e5, 1);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
// function s.sdcon(e)
fn sdcon(d: &mut Duel, a: Args) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e(0)).nil();
    // return c:GetOwnerTargetCount()>0 or Duel.IsExistingMatchingCard(Card.IsFaceup,0,LOCATION_MZONE,LOCATION_MZONE,1,e:GetHandler())
    card::get_owner_target_count(d, c) > 0
        || duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x)), 0, LOCATION_MZONE, LOCATION_MZONE, 1, c.into())
}
// function s.desop1(e,tp,eg,ep,ev,re,r,rp)
fn desop1(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if re:GetHandler():IsSpellTrap() and c:IsLocation(LOCATION_MZONE) and c:IsFaceup() then
    let re = a.re.nil();
    let re_h = effect::get_handler(d, re).nil();
    if card::is_spell_trap(d, re_h) && card::is_location(d, c, LOCATION_MZONE) && card::is_faceup(d, c) {
        // e:SetLabelObject(re)
        effect::set_label_object(d, a.e, Value::Effect(re));
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
    }
}
// function s.desop2(e,tp,eg,ep,ev,re,r,rp)
fn desop2(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if re==e:GetLabelObject():GetLabelObject() and c:IsRelateToEffect(re) then
    let e2 = effect::get_label_object(d, a.e).as_effect().nil();
    let linked = effect::get_label_object(d, e2).as_effect();
    if a.re == linked && card::is_relate_to_effect(d, c, a.re.nil()) {
        // if Duel.IsPhase(PHASE_DAMAGE) and not Duel.IsDamageCalculated() then
        if duel::is_phase(d, PHASE_DAMAGE) && !duel::is_damage_calculated(d) {
            // e:GetLabelObject():SetLabel(1)
            effect::set_label(d, e2, &[1]);
        } else {
            // if not c:IsDisabled() then Duel.Destroy(c,REASON_EFFECT) end
            if !card::is_disabled(d, c) {
                duel::destroy(d, c, REASON_EFFECT, None, None);
            }
        }
    }
}
// function s.desop3(e,tp,eg,ep,ev,re,r,rp)
fn desop3(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local des=e:GetLabelObject():GetLabel()
    let e2 = effect::get_label_object(d, a.e).as_effect().nil();
    let des = effect::get_label(d, e2);
    // e:GetLabelObject():SetLabel(0)
    effect::set_label(d, e2, &[0]);
    // if des==1 and not c:IsDisabled() then
    if des == 1 && !card::is_disabled(d, c) {
        // Duel.Destroy(c,REASON_EFFECT)
        duel::destroy(d, c, REASON_EFFECT, None, None);
    }
}
