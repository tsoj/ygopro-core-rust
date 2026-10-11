//! True Exodia
use super::prelude::*;

pub const ID: u32 = 37984331;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FORBIDDEN_ONE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // lose condition
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD | EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_UNCOPYABLE | EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EVENT_ADJUST);
    effect::set_operation(d, e1, Some(operation(loseop)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_NORMAL) && card::is_set_card(d, c, &[SET_FORBIDDEN_ONE as u16])
}
fn loseop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, c.into());
    if group::filter_count(d, &g, &filter, NO_EX) == g.len()
        && g.len() == 4
        && group::get_class_count(d, &g, &|d, x| card::get_code(d, x)[0] as i64) == 4
    {
        duel::win(d, 1 - card::get_controler(d, c), WIN_REASON_TRUE_EXODIA);
    }
}
