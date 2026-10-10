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
