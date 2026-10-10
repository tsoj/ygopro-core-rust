//! Lava Battleguard
use super::prelude::*;

pub const ID: u32 = 20394040;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[40453765], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_value_fn(d, e1, effect_card_value(value));
    card::register_effect(d, c, e1, false);
}
fn value(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    let tp = card::get_controler(d, c);
    let f = aux::faceup_filter(|d, c| card::is_code(d, c, &[40453765]));
    duel::get_matching_group_count(d, Some(&f), tp, LOCATION_MZONE, 0, NO_EX) as i64 * 500
}
