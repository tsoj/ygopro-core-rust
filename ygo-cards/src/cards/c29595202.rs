//! Runick Allure
use super::prelude::*;

pub const ID: u32 = 29595202;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Banish the top card of your opponent's Deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_REMOVE);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD | EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_condition(d, e2, Some(condition(rmcon)));
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    effect::set_operation(d, e2, Some(operation(rmop)));
    card::register_effect(d, c, e2, false);
}
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    let h = effect::get_handler(d, re).nil();
    effect::is_spell_effect(d, re) && card::is_type(d, h, TYPE_QUICKPLAY) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - a.tp, LOCATION_DECK as i32);
    true
}
fn rmop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_decktop_group(d, 1 - a.tp, 1);
    if g.is_empty() {
        return;
    }
    duel::disable_shuffle_check(d, true);
    duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
