//! Barrier Statue of the Drought
use super::prelude::*;

pub const ID: u32 = 19740112;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // disable spsummon
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 1, 1);
    effect::set_target(d, e1, Some(lua_fn(sumlimit)));
    card::register_effect(d, c, e1, false);
}
/// `s.sumlimit(e,c,sump,sumtype,sumpos,targetp)`
fn sumlimit(d: &mut Duel, a: Args) -> bool {
    // return c:GetAttribute()~=ATTRIBUTE_EARTH
    card::get_attribute(d, a.c(1)) != ATTRIBUTE_EARTH
}
