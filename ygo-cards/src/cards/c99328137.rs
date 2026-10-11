//! Ice Knight
use super::prelude::*;

pub const ID: u32 = 99328137;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Gains 400 ATK for each of your aqua monsters
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_value_fn(d, e1, effect_card_value(atk_value));
    card::register_effect(d, c, e1, false);
    // Gain an extra normal summon for WATER monsters
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_operation(d, e2, Some(operation(sumop)));
    card::register_effect(d, c, e2, false);
}
// function(e,c) return Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsRace,RACE_AQUA),c:GetControler(),LOCATION_MZONE,0,nil)*400 end
fn atk_value(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    let p = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&|d, x| card::is_faceup(d, x) && card::is_race(d, x, RACE_AQUA)), p, LOCATION_MZONE, 0, NO_EX) as i64 * 400
}
// function s.sumop(e,tp,eg,ep,ev,re,r,rp)
fn sumop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    // Extra Summon
    let e1 = effect::create_effect(d, h);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_target_range(d, e1, LOCATION_HAND | LOCATION_MZONE, 0);
    effect::set_code(d, e1, EFFECT_EXTRA_SUMMON_COUNT);
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_attribute(d, c, ATTRIBUTE_WATER))));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
    // Cannot Normal/Special Summon, except WATER monsters
    let e2 = effect::create_effect(d, h);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_SUMMON);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    effect::set_target_range(d, e2, 1, 0);
    effect::set_target(d, e2, Some(lua_fn(splimit)));
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e2, a.tp);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_CANNOT_SPECIAL_SUMMON);
    duel::register_effect(d, e3, a.tp);
    aux::register_client_hint(d, h, 0, a.tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
}
// function s.splimit(e,c,sump,sumtype,sumpos,targetp) return not c:IsAttribute(ATTRIBUTE_WATER) end
fn splimit(d: &mut Duel, a: Args) -> bool {
    !card::is_attribute(d, a.c(1), ATTRIBUTE_WATER)
}
