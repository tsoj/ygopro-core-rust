//! Vector Pendulum, the Dracoverlord
use super::prelude::*;

pub const ID: u32 = 69512157;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    pendulum::add_procedure(d, c, None, None);
    // disable
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_DISABLE);
    effect::set_range(d, e2, LOCATION_PZONE);
    effect::set_target_range(d, e2, 0, LOCATION_PZONE);
    card::register_effect(d, c, e2, false);
    // disable effect
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e3, LOCATION_PZONE);
    effect::set_operation(d, e3, Some(operation(disop)));
    card::register_effect(d, c, e3, false);
}
fn disop(d: &mut Duel, a: &EffArgs) {
    // local p,loc=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_CONTROLER,CHAININFO_TRIGGERING_LOCATION)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_CONTROLER, CHAININFO_TRIGGERING_LOCATION]);
    let p = info[0].int() as u8;
    let loc = info[1].int() as u32;
    let re = a.re.nil();
    if effect::get_active_type(d, re) == TYPE_PENDULUM + TYPE_SPELL && p != a.tp && (loc & LOCATION_PZONE) != 0 {
        duel::negate_effect(d, a.ev as u8);
    }
}
