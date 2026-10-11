//! Chronomaly Aztec Mask Golem
use super::prelude::*;

pub const ID: u32 = 94766498;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CHRONOMALY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // Special Summon
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(lua_fn(hspcon)));
    card::register_effect(d, c, e1, false);
    duel::add_custom_activity_counter(d, ID, ACTIVITY_CHAIN, lua_fn(chainfilter));
}
/// `s.chainfilter(re, tp, cid)`
fn chainfilter(d: &mut Duel, a: Args) -> bool {
    let re = a.e(0);
    !(effect::is_spell_effect(d, re) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && {
        let h = effect::get_handler(d, re).nil();
        card::is_set_card(d, h, &[SET_CHRONOMALY as u16])
    })
}
/// `s.hspcon(e, c)`
fn hspcon(d: &mut Duel, a: Args) -> bool {
    let Some(c) = a.card(1) else { return true };
    let tp = card::get_controler(d, c);
    duel::get_custom_activity_count(d, ID, tp, ACTIVITY_CHAIN) > 0 && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
}
