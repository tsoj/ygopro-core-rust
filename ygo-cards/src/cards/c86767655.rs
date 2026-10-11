//! Ms. Judge
use super::prelude::*;

pub const ID: u32 = 86767655;
pub const INFO: ScriptInfo = ScriptInfo { toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Toss a coin and negate an effect when it resolves
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e0, EVENT_CHAINING);
    effect::set_range(d, e0, LOCATION_MZONE);
    effect::set_operation(d, e0, Some(aux::chainreg()));
    card::register_effect(d, c, e0, false);
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(discon)));
    effect::set_operation(d, e1, Some(operation(disop)));
    card::register_effect(d, c, e1, false);
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    a.rp != a.tp && duel::is_chain_disablable(d, a.ev as u8) && card::get_flag_effect(d, c, 1) > 0
}
fn disop(d: &mut Duel, a: &EffArgs) {
    let res = duel::toss_coin(d, a.tp, 2);
    if duel::count_heads(&res) == 2 {
        duel::negate_effect(d, a.ev as u8);
    }
}
