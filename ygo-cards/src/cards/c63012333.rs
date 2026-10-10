//! Soul-Absorbing Bone Tower
use super::prelude::*;

pub const ID: u32 = 63012333;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot be target
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CANNOT_BE_BATTLE_TARGET);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(atklm)));
    effect::set_value_fn(d, e1, aux::imval2());
    card::register_effect(d, c, e1, false);
    // damage
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DECKDES);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
}
fn zombie_faceup(d: &mut Duel, c: CardId) -> bool {
    // aux.FaceupFilter(Card.IsRace,RACE_ZOMBIE)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_ZOMBIE)
}
fn atklm(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_ZOMBIE),c:GetControler(),LOCATION_MZONE,0,1,c)
    let c = effect::get_handler(d, a.e).nil();
    let p = card::get_controler(d, c);
    duel::is_existing_matching_card(d, Some(&zombie_faceup), p, LOCATION_MZONE, 0, 1, c.into())
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return not eg:IsContains(e:GetHandler()) and eg:IsExists(aux.FaceupFilter(Card.IsRace,RACE_ZOMBIE),1,nil)
    let eg = a.eg.clone().nil();
    let c = effect::get_handler(d, a.e).nil();
    !group::is_contains(&eg, c) && group::is_exists(d, &eg, &zombie_faceup, 1, NO_EX)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, 1 - a.tp, 2);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    duel::discard_deck(d, 1 - a.tp, 2, REASON_EFFECT);
}
