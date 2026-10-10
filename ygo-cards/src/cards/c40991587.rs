//! The Lady in Wight
use super::prelude::*;

pub const ID: u32 = 40991587;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_SKULL_SERVANT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Name becomes "Skull Servant" while in GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetValue(CARD_SKULL_SERVANT)
    effect::set_value(d, e1, CARD_SKULL_SERVANT as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Other level 3 or lower zombie monsters cannot be destroyed by battle
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetTarget(s.etarget)
    effect::set_target(d, e2, Some(effect_card_fn(etarget)));
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Other level 3 or lower zombie monsters are unaffected by spells/traps
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e3, EFFECT_IMMUNE_EFFECT);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    // e3:SetTarget(s.etarget)
    effect::set_target(d, e3, Some(effect_card_fn(etarget)));
    // e3:SetValue(s.efilter)
    effect::set_value_fn(d, e3, lua_fn(|d, a| efilter(d, a.e(1))));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.etarget(e,c)`
fn etarget(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:GetCode()~=id and c:IsRace(RACE_ZOMBIE) and c:IsLevelBelow(3)
    card::get_code(d, c).first().copied() != Some(ID) && card::is_race(d, c, RACE_ZOMBIE) && card::is_level_below(d, c, 3)
}
/// `s.efilter(e,te)`
fn efilter(d: &mut Duel, te: EffectId) -> bool {
    // return te:IsSpellTrapEffect() and not te:GetHandler():IsCode(4064256)
    effect::is_spell_trap_effect(d, te) && {
        let h = effect::get_handler(d, te).nil();
        !card::is_code(d, h, &[4064256])
    }
}
