//! Gaia Dragon, the Thunder Charger
use super::prelude::*;

pub const ID: u32 = 91949988;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    xyz::add_procedure(d, c, None, Some(7), 2, Some(xyz::alter_filter(|d, c, _tp, _xyzc| ovfilter(d, c))), Some(aux::stringid(ID, 0)), None, None, false, None);
    card::enable_revive_limit(d, c);
    // pierce
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_PIERCE);
    card::register_effect(d, c, e1, false);
}
fn ovfilter(d: &mut Duel, c: CardId) -> bool {
    let rk = card::get_rank(d, c);
    card::is_faceup(d, c) && (rk == 5 || rk == 6)
}
