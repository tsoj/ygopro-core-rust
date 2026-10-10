//! Alligator's Sword Dragon
use super::prelude::*;

pub const ID: u32 = 3366982;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,88819587,64428736)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(88819587), fusion::Mat::Code(64428736)]);
    // Can attack directly
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_DIRECT_ATTACK);
    effect::set_condition(d, e2, Some(condition(dircon)));
    card::register_effect(d, c, e2, false);
}
/// `s.dircon(e)`
fn dircon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ATTRIBUTE_EARTH_WATER_FIRE=ATTRIBUTE_EARTH|ATTRIBUTE_WATER|ATTRIBUTE_FIRE
    let att = ATTRIBUTE_EARTH | ATTRIBUTE_WATER | ATTRIBUTE_FIRE;
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,e:GetHandlerPlayer(),0,LOCATION_MZONE,nil)
    let p = effect::get_handler_player(d, a.e);
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), p, 0, LOCATION_MZONE, NO_EX);
    // local ct=#g
    let ct = g.len();
    // return ct>0 and g:FilterCount(Card.IsAttribute,nil,ATTRIBUTE_EARTH_WATER_FIRE)==ct
    ct > 0 && group::filter_count(d, &g, &|d, c| card::is_attribute(d, c, att), NO_EX) == ct
}
