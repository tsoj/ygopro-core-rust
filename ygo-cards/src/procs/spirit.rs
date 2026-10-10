//! `Spirit.*` (`proc_spirit.lua`): Spirit monsters return to the hand in the End Phase.

use crate::constants::*;
use crate::lua::{card, duel, effect};
use crate::prelude::*;

/// `FLAG_SPIRIT_RETURN`
pub const FLAG_SPIRIT_RETURN: u32 = 2;

/// `Spirit.AddProcedure(c, events...)`: return to the hand in the End Phase of the turn one of
/// `events` happened (returns the effects: the mandatory and optional return, then the flag
/// effects)
pub fn add_procedure(d: &mut Duel, c: CardId, events: &[u32]) -> Vec<EffectId> {
    // Return this card to the hand during the End Phase
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, 1105);
    effect::set_category(d, e1, CATEGORY_TOHAND);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(|d, a| common(d, a.e) && !maynot(d, a.e))));
    effect::set_target(d, e1, Some(cost_target(mandatory_return_target)));
    effect::set_operation(d, e1, Some(return_operation()));
    card::register_effect(d, c, e1, false);
    // Optional return in case of "SPIRIT_MAYNOT_RETURN" effects
    let e2 = effect::clone(d, e1);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_condition(d, e2, Some(condition(|d, a| common(d, a.e) && maynot(d, a.e))));
    effect::set_target(d, e2, Some(cost_target(optional_return_target)));
    card::register_effect(d, c, e2, false);
    let mut out = vec![e1, e2];
    // Effects that register the flags
    for &event in events {
        let fe1 = effect::create_effect(d, c);
        effect::set_type(d, fe1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
        effect::set_property(d, fe1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, fe1, event);
        effect::set_operation(
            d,
            fe1,
            Some(lua_fn(|d, a| {
                let h = effect::get_handler(d, a.e(0)).nil();
                card::register_flag_effect(d, h, FLAG_SPIRIT_RETURN, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
            })),
        );
        card::register_effect(d, c, fe1, false);
        out.push(fe1);
    }
    out
}
/// `Spirit.CommonCondition(e)`
fn common(d: &mut Duel, e: EffectId) -> bool {
    let c = effect::get_handler(d, e).nil();
    card::has_flag_effect(d, c, FLAG_SPIRIT_RETURN, None) && card::is_has_effect(d, c, EFFECT_SPIRIT_DONOT_RETURN, None).is_empty()
}
fn maynot(d: &mut Duel, e: EffectId) -> bool {
    let c = effect::get_handler(d, e).nil();
    !card::is_has_effect(d, c, EFFECT_SPIRIT_MAYNOT_RETURN, None).is_empty()
}
/// `Spirit.MandatoryReturnCondition`
pub fn mandatory_return_condition() -> LuaFn {
    condition(|d, a| common(d, a.e) && !maynot(d, a.e))
}
/// `Spirit.OptionalReturnCondition`
pub fn optional_return_condition() -> LuaFn {
    condition(|d, a| common(d, a.e) && maynot(d, a.e))
}
fn mandatory_return_target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let c = effect::get_handler(d, a.e).nil();
    card::reset_flag_effect(d, c, FLAG_SPIRIT_RETURN);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn optional_return_target(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    card::reset_flag_effect(d, c, FLAG_SPIRIT_RETURN);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `Spirit.MandatoryReturnTarget` / `Spirit.OptionalReturnTarget`
pub fn mandatory_return_target_fn() -> LuaFn {
    cost_target(mandatory_return_target)
}
pub fn optional_return_target_fn() -> LuaFn {
    cost_target(optional_return_target)
}
/// `Spirit.ReturnOperation`
pub fn return_operation() -> LuaFn {
    operation(|d, a| {
        let c = effect::get_handler(d, a.e).nil();
        if card::is_relate_to_effect(d, c, a.e) {
            duel::send_to_hand(d, c, None, REASON_EFFECT, None);
        }
    })
}
