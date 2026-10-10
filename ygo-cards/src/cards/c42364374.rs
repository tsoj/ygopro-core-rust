//! Arsenal Bug
use super::prelude::*;

pub const ID: u32 = 42364374;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk def
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_SET_ATTACK);
    effect::set_value(d, e1, 1000);
    effect::set_condition(d, e1, Some(condition(con)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_SET_DEFENSE);
    card::register_effect(d, c, e2, false);
}
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_INSECT),c:GetControler(),LOCATION_MZONE,0,1,c)
    let p = card::get_controler(d, c);
    !duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_race(d, c, RACE_INSECT))), p, LOCATION_MZONE, 0, 1, c.into())
}
