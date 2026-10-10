//! Royal Decree
use super::prelude::*;

pub const ID: u32 = 51452091;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    card::register_effect(d, c, e1, false);
    // Negate traps
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_DISABLE);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, LOCATION_SZONE, LOCATION_SZONE);
    effect::set_target(d, e2, Some(effect_card_fn(distarget)));
    card::register_effect(d, c, e2, false);
    // Negate trap effects
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_operation(d, e3, Some(operation(disop)));
    card::register_effect(d, c, e3, false);
    // Disable trap monster
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    effect::set_code(d, e4, EFFECT_DISABLE_TRAPMONSTER);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_target_range(d, e4, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e4, Some(effect_card_fn(distarget)));
    card::register_effect(d, c, e4, false);
    aux::double_snare_validity(d, Some(c), LOCATION_SZONE, None);
}
/// `function(e,c) return c~=e:GetHandler() and c:IsTrap() end`
fn distarget(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let h = effect::get_handler(d, e).nil();
    c != h && card::is_trap(d, c)
}
fn disop(d: &mut Duel, a: &EffArgs) {
    // local tl=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)
    let tl = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int();
    let re = a.re.nil();
    // if tl==LOCATION_SZONE and re:IsTrapEffect() and re:GetHandler()~=e:GetHandler() then
    if tl == LOCATION_SZONE as i64 && effect::is_trap_effect(d, re) {
        let rh = effect::get_handler(d, re).nil();
        let h = effect::get_handler(d, a.e).nil();
        if rh != h {
            // Duel.NegateEffect(ev)
            duel::negate_effect(d, a.ev as u8);
        }
    }
}
