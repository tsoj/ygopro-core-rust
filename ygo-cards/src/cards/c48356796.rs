//! Mystical Cards of Light
use super::prelude::*;

pub const ID: u32 = 48356796;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.accon)
    effect::set_condition(d, e1, Some(condition(accon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Effect Draw
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_DRAW_COUNT)
    effect::set_code(d, e2, EFFECT_DRAW_COUNT);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetValue(2)
    effect::set_value(d, e2, 2);
    // e2:SetCondition(s.drawcon)
    effect::set_condition(d, e2, Some(condition(drawcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Effect Disable Field
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_DISABLE_FIELD)
    effect::set_code(d, e3, EFFECT_DISABLE_FIELD);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(LOCATION_SZONE,0)
    effect::set_target_range(d, e3, LOCATION_SZONE, 0);
    // e3:SetOperation(s.disop)
    effect::set_operation(d, e3, Some(lua_fn(|d, a: Args| disop(d, a.int(1) as u8))));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.accon(e,tp,eg,ep,ev,re,r,rp)
fn accon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=Duel.GetLocationCount(tp,LOCATION_SZONE,PLAYER_NONE,0)
    let c = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, Some(PLAYER_NONE as u8), Some(0), None).0;
    // if e:GetHandler():IsFacedown() then return c>0 end
    let h = effect::get_handler(d, a.e).nil();
    if card::is_facedown(d, h) {
        return c > 0;
    }
    // return c>1
    c > 1
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(id) and c:IsFaceup()
    card::is_code(d, c, &[ID]) && card::is_faceup(d, c)
}
// function s.drawcon(e)
fn drawcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.filter,e:GetHandlerPlayer(),LOCATION_SZONE,0,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&filter), p, LOCATION_SZONE, 0, 1, h.into())
}
// function s.disop(e,tp,eg,ep,ev,re,r,rp)
fn disop(d: &mut Duel, tp: u8) -> Value {
    // return Duel.SelectDisableField(tp,1,LOCATION_SZONE,0,0)
    let zones = duel::select_disable_field(d, tp, 1, LOCATION_SZONE, 0, Some(0), false);
    Value::Int(zones as i64)
}
