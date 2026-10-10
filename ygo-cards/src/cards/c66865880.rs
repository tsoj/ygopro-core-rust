//! Marshmallon Glasses
use super::prelude::*;

pub const ID: u32 = 66865880;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[31305911],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    //
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(con)));
    effect::set_value_fn(d, e2, effect_card_fn(atlimit));
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[31305911])
}
/// `s.con(e)`
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&cfilter), p, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.atlimit(e,c)`
fn atlimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_facedown(d, c) || card::get_code(d, c).first().copied() != Some(31305911)
}
