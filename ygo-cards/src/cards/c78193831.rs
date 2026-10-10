//! Buster Blader
use super::prelude::*;

pub const ID: u32 = 78193831;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atkup
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value_fn(d, e1, effect_card_value(val));
    card::register_effect(d, c, e1, false);
}
/// `function(e,c)`
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(s.filter,c:GetControler(),0,LOCATION_GRAVE|LOCATION_MZONE,nil)*500
    let tp = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&filter), tp, 0, LOCATION_GRAVE | LOCATION_MZONE, NO_EX) as i64 * 500
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON) and (c:IsLocation(LOCATION_GRAVE) or c:IsFaceup())
    card::is_race(d, c, RACE_DRAGON) && (card::is_location(d, c, LOCATION_GRAVE) || card::is_faceup(d, c))
}
