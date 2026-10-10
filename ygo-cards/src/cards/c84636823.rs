//! Spell Canceller
use super::prelude::*;

pub const ID: u32 = 84636823;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot trigger
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, 1, 1);
    effect::set_value_fn(d, e1, lua_fn(aclimit));
    card::register_effect(d, c, e1, false);
    // disable
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_DISABLE);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target_range(d, e2, LOCATION_SZONE, LOCATION_SZONE);
    effect::set_target(d, e2, Some(effect_card_fn(distg)));
    card::register_effect(d, c, e2, false);
    // disable effect
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_operation(d, e3, Some(operation(disop)));
    card::register_effect(d, c, e3, false);
}
/// `function(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // return re:IsSpellEffect() and (re:GetHandler():IsOnField() or re:IsHasType(EFFECT_TYPE_ACTIVATE))
    let re = a.e(1);
    if !effect::is_spell_effect(d, re) {
        return false;
    }
    let h = effect::get_handler(d, re).nil();
    card::is_on_field(d, h) || effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
fn distg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsSpell()
    card::is_spell(d, c)
}
fn disop(d: &mut Duel, a: &EffArgs) {
    // local tl=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)
    let tl = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int() as u32;
    if (tl & LOCATION_SZONE) != 0 && effect::is_spell_effect(d, a.re.nil()) {
        duel::negate_effect(d, a.ev as u8);
    }
}
