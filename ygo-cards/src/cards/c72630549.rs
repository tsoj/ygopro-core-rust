//! Chaos Command Magician
use super::prelude::*;

pub const ID: u32 = 72630549;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // disable
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_DISABLE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e1, Some(effect_card_fn(distg)));
    card::register_effect(d, c, e1, false);
    // disable effect
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_operation(d, e2, Some(operation(disop)));
    card::register_effect(d, c, e2, false);
}
/// `s.distg(e,c)`
fn distg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // if c:GetCardTargetCount()~=1 then return false end
    if card::get_card_target_count(d, c) != 1 {
        return false;
    }
    // return c:GetFirstCardTarget()==e:GetHandler()
    card::get_first_card_target(d, c) == effect::get_handler(d, e)
}
/// `s.disop`
fn disop(d: &mut Duel, a: &EffArgs) {
    // if not re:IsActiveType(TYPE_EFFECT) then return end
    let re = a.re.nil();
    if !effect::is_active_type(d, re, TYPE_EFFECT) {
        return;
    }
    // if not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return end
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned());
    // if not g or #g~=1 then return end
    let Some(g) = g else { return };
    if g.len() != 1 {
        return;
    }
    // if g:GetFirst()==e:GetHandler() then
    if group::get_first(d, &g) == effect::get_handler(d, a.e) {
        // Duel.NegateEffect(ev)
        duel::negate_effect(d, a.ev as u8);
    }
}
