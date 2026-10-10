//! Amphibious Bugroth MK-3
use super::prelude::*;

pub const ID: u32 = 64342551;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_UMI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    effect::set_condition(d, e1, Some(lua_fn(dircon)));
    card::register_effect(d, c, e1, false);
}
/// `s.dircon(e)`
fn dircon(d: &mut Duel, _a: Args) -> bool {
    // return Duel.IsEnvironment(CARD_UMI)
    duel::is_environment(d, CARD_UMI, None, None)
}
