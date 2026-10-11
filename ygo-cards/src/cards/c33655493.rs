//! Belial - Marquis of Darkness
use super::prelude::*;

pub const ID: u32 = 33655493;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Your opponent cannot target any face-up monster you control, except "Belial - Marquis of Darkness", for an attack, or with Spell/Trap effects
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD);
    // e1a:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e1a, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e1a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1a, LOCATION_MZONE);
    // e1a:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1a, 0, LOCATION_MZONE);
    // e1a:SetValue(function(e,c) return c:IsFaceup() and not c:IsCode(id) end)
    effect::set_value_fn(d, e1a, effect_card_fn(|d, _e, c| not_belial_faceup(d, c)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=Effect.CreateEffect(c)
    let e1b = effect::create_effect(d, c);
    // e1b:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1b, EFFECT_TYPE_FIELD);
    // e1b:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1b, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1b:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1b, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1b:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1b, LOCATION_MZONE);
    // e1b:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1b, LOCATION_MZONE, 0);
    // e1b:SetTarget(function(e,c) return c:IsFaceup() and not c:IsCode(id) end)
    effect::set_target(d, e1b, Some(effect_card_fn(|d, _e, c| not_belial_faceup(d, c))));
    // e1b:SetValue(function(e,re,rp) return rp==1-e:GetHandlerPlayer() and re:IsSpellTrapEffect() end)
    effect::set_value_fn(d, e1b, lua_fn(spelltrap_value));
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
}
/// `c:IsFaceup() and not c:IsCode(id)`
fn not_belial_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && !card::is_code(d, c, &[ID])
}
/// `function(e,re,rp) return rp==1-e:GetHandlerPlayer() and re:IsSpellTrapEffect() end`
fn spelltrap_value(d: &mut Duel, a: Args) -> bool {
    let (e, re, rp) = (a.e(0), a.e(1), a.p(2));
    rp == 1 - effect::get_handler_player(d, e) && effect::is_spell_trap_effect(d, re)
}
