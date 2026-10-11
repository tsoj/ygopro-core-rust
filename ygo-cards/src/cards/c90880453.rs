//! Elfnotes: A Plea for Help
use super::prelude::*;

pub const ID: u32 = 90880453;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Your opponent cannot target the monster in your center Main Monster Zone with card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(function(e,c) return c:IsSequence(2) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_sequence(d, c, &[2]))));
    // e1:SetValue(aux.tgoval)
    effect::set_value_fn(d, e1, aux::tgoval());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // FIRE/EARTH: If your Synchro Monster attacks a Defense Position monster, inflict piercing battle damage to your opponent
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e2, EFFECT_PIERCE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(function(e,c) return c:IsSynchroMonster() end)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_synchro_monster(d, c))));
    // e2:SetCondition(s.effcon)
    effect::set_condition(d, e2, Some(condition(effcon)));
    // e2:SetLabel(ATTRIBUTE_FIRE|ATTRIBUTE_EARTH)
    effect::set_label(d, e2, &[(ATTRIBUTE_FIRE | ATTRIBUTE_EARTH) as i64]);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // WATER/WIND: Other face-up Spells/Traps you control cannot be destroyed by your opponent's card effects
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    // e3:SetTargetRange(LOCATION_ONFIELD,0)
    effect::set_target_range(d, e3, LOCATION_ONFIELD, 0);
    // e3:SetTarget(function(e,c) return c:IsSpellTrap() and c~=e:GetHandler() end)
    effect::set_target(d, e3, Some(effect_card_fn(|d, e, c| card::is_spell_trap(d, c) && c != effect::get_handler(d, e).nil())));
    // e3:SetValue(aux.indoval)
    effect::set_value_fn(d, e3, aux::indoval());
    // e3:SetLabel(ATTRIBUTE_WATER|ATTRIBUTE_WIND)
    effect::set_label(d, e3, &[(ATTRIBUTE_WATER | ATTRIBUTE_WIND) as i64]);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // LIGHT/DARK: You take no battle damage
    // local e4=e2:Clone()
    let e4 = effect::clone(d, e2);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e4, EFFECT_AVOID_BATTLE_DAMAGE);
    // e4:SetTargetRange(1,0)
    effect::set_target_range(d, e4, 1, 0);
    // e4:SetTarget(aux.TRUE)
    effect::set_target(d, e4, Some(effect_card_fn(|_d, _e, _c| true)));
    // e4:SetLabel(ATTRIBUTE_LIGHT|ATTRIBUTE_DARK)
    effect::set_label(d, e4, &[(ATTRIBUTE_LIGHT | ATTRIBUTE_DARK) as i64]);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.effconfilter(c,attr)`
fn effconfilter(d: &mut Duel, c: CardId, attr: u32) -> bool {
    // return c:IsSequence(2) and c:IsAttribute(attr) and c:IsFaceup()
    card::is_sequence(d, c, &[2]) && card::is_attribute(d, c, attr) && card::is_faceup(d, c)
}
/// `s.effcon(e)`
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.effconfilter,e:GetHandlerPlayer(),LOCATION_MMZONE,0,1,nil,e:GetLabel())
    let p = effect::get_handler_player(d, a.e);
    let attr = effect::get_label(d, a.e) as u32;
    duel::is_existing_matching_card(d, Some(&|d, c| effconfilter(d, c, attr)), p, LOCATION_MMZONE, 0, 1, NO_EX)
}
