//! Machine King
use super::prelude::*;

pub const ID: u32 = 46700124;

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
/// `s.val(e,c)`
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsRace,RACE_MACHINE),c:GetControler(),LOCATION_MZONE,LOCATION_MZONE,nil)*100
    let p = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&aux::faceup_filter(|d, cc| card::is_race(d, cc, RACE_MACHINE))), p, LOCATION_MZONE, LOCATION_MZONE, NO_EX) as i64 * 100
}
