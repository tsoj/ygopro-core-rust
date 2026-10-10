//! Mermaid Knight
use super::prelude::*;

pub const ID: u32 = 24435369;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_UMI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
    effect::set_condition(d, e1, Some(condition(dircon)));
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
}
fn dircon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsEnvironment(CARD_UMI)
    duel::is_environment(d, CARD_UMI, None, None)
}
