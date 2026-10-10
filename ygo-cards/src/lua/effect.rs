//! `Effect.*` as scripts see it.

pub use ygo_core::api::effect::*;

use ygo_core::duel::Duel;
use ygo_core::types::*;

fn cost_tag(d: &Duel, e: EffectId, tag: u32) -> bool {
    get_cost(d, e).is_some_and(|f| f.tag & tag != 0)
}
/// `e:HasSelfToGraveCost()` (utility.lua `cost_tables`)
pub fn has_self_to_grave_cost(d: &mut Duel, e: EffectId) -> bool {
    cost_tag(d, e, crate::cost::TAG_SELF_TOGRAVE)
}
pub fn has_self_discard_cost(d: &mut Duel, e: EffectId) -> bool {
    cost_tag(d, e, crate::cost::TAG_SELF_DISCARD)
}
pub fn has_detach_cost(d: &mut Duel, e: EffectId) -> bool {
    cost_tag(d, e, crate::cost::TAG_DETACH)
}
pub fn has_self_change_position_cost(d: &mut Duel, e: EffectId) -> bool {
    cost_tag(d, e, crate::cost::TAG_SELF_CHANGEPOS)
}
/// `e:HasRemainFieldCost()`: the activation cost that keeps the card on the field.
pub fn has_remain_field_cost(d: &mut Duel, e: EffectId) -> bool {
    cost_tag(d, e, crate::cost::TAG_REMAIN_FIELD)
}
pub fn is_monster_effect(d: &mut Duel, e: EffectId) -> bool {
    is_active_type(d, e, crate::constants::TYPE_MONSTER)
}
pub fn is_spell_effect(d: &mut Duel, e: EffectId) -> bool {
    is_active_type(d, e, crate::constants::TYPE_SPELL)
}
pub fn is_trap_effect(d: &mut Duel, e: EffectId) -> bool {
    is_active_type(d, e, crate::constants::TYPE_TRAP)
}
pub fn is_spell_trap_effect(d: &mut Duel, e: EffectId) -> bool {
    is_active_type(d, e, crate::constants::TYPE_SPELL | crate::constants::TYPE_TRAP)
}
/// `e:IsGlobalEffect()`: its owner is the duel's temporary card (code 0).
pub fn is_global_effect(d: &mut Duel, e: EffectId) -> bool {
    match get_owner(d, e) {
        Some(c) => ygo_core::api::card::is_code(d, c, &[0]),
        None => false,
    }
}

/// `e:IsCard<prop>(...)` / `e:GetCard<prop>()` of `chain.lua`: the properties of the effect's card
/// (see `chain::effect_is_card`); these are the common ones as booleans.
pub fn is_card_setcode(d: &mut Duel, e: EffectId, setcodes: &[u16]) -> bool {
    crate::chain::effect_is_card(d, e, crate::chain::Prop::SetCard, &setcodes.iter().map(|&x| x as i64).collect::<Vec<_>>()).truthy()
}
pub fn is_card_code(d: &mut Duel, e: EffectId, codes: &[u32]) -> bool {
    crate::chain::effect_is_card(d, e, crate::chain::Prop::Code, &codes.iter().map(|&x| x as i64).collect::<Vec<_>>()).truthy()
}
pub fn is_card_type(d: &mut Duel, e: EffectId, t: u32) -> bool {
    crate::chain::effect_is_card(d, e, crate::chain::Prop::Type, &[t as i64]).truthy()
}
pub fn is_card_race(d: &mut Duel, e: EffectId, r: u64) -> bool {
    crate::chain::effect_is_card(d, e, crate::chain::Prop::Race, &[r as i64]).truthy()
}
pub fn is_card_attribute(d: &mut Duel, e: EffectId, a: u32) -> bool {
    crate::chain::effect_is_card(d, e, crate::chain::Prop::Attribute, &[a as i64]).truthy()
}
pub fn is_card_location(d: &mut Duel, e: EffectId, l: u32) -> bool {
    crate::chain::effect_is_card(d, e, crate::chain::Prop::Location, &[l as i64]).truthy()
}
pub fn is_card_controler(d: &mut Duel, e: EffectId, p: u8) -> bool {
    crate::chain::effect_is_card(d, e, crate::chain::Prop::Controler, &[p as i64]).truthy()
}
