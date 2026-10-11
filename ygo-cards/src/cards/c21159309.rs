//! Majestic Dragon
use super::prelude::*;

pub const ID: u32 = 21159309;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_MAJESTIC],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CANNOT_BE_SYNCHRO_MATERIAL);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_value_fn(d, e1, lua_fn(synlimit));
    card::register_effect(d, c, e1, false);
}
/// `function(e,c)`
fn synlimit(d: &mut Duel, a: Args) -> bool {
    match a.card(1) {
        None => false,
        Some(c) => !card::is_set_card(d, c, &[SET_MAJESTIC as u16]),
    }
}
