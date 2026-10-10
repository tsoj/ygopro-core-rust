//! Cyber Dragon
use super::prelude::*;

pub const ID: u32 = 70095154;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    card::register_effect(d, c, e1, false);
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // return Duel.GetFieldGroupCount(c:GetControler(),LOCATION_MZONE,0)==0
    let p = card::get_controler(d, c);
    if duel::get_field_group_count(d, p, LOCATION_MZONE, 0) != 0 {
        return false;
    }
    // and Duel.GetFieldGroupCount(c:GetControler(),0,LOCATION_MZONE)>0
    let p = card::get_controler(d, c);
    if duel::get_field_group_count(d, p, 0, LOCATION_MZONE) <= 0 {
        return false;
    }
    // and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    let p = card::get_controler(d, c);
    duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0
}
