//! Des Wombat
use super::prelude::*;

pub const ID: u32 = 9637706;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_value_fn(d, e1, lua_fn(damval));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_NO_EFFECT_DAMAGE);
    card::register_effect(d, c, e2, false);
}
/// `function(e,re,val,r,rp,rc)`
fn damval(_d: &mut Duel, a: Args) -> i64 {
    let (val, r) = (a.int(2), a.int(3) as u32);
    if (r & REASON_EFFECT) != 0 {
        0
    } else {
        val
    }
}
