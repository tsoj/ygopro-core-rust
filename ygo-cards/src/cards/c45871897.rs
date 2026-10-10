//! Lost Guardian
use super::prelude::*;

pub const ID: u32 = 45871897;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // change defense
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_BASE_DEFENSE)
    effect::set_code(d, e1, EFFECT_SET_BASE_DEFENSE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.defval)
    effect::set_value_fn(d, e1, effect_card_value(defval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.defval(e,c)`
fn defval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsRace,RACE_ROCK),c:GetControler(),LOCATION_REMOVED,0,nil)*700
    let controler = card::get_controler(d, c);
    let n = duel::get_matching_group_count(d, Some(&aux::faceup_filter(|d, c| card::is_race(d, c, RACE_ROCK))), controler, LOCATION_REMOVED as u32, 0, NO_EX);
    n as i64 * 700
}
