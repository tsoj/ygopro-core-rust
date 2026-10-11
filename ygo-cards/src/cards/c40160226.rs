//! Mecha Sea Dragon Plesion
use super::prelude::*;

pub const ID: u32 = 40160226;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon with no tribute
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_SUMMON_PROC);
    effect::set_condition(d, e1, Some(lua_fn(ntcon)));
    card::register_effect(d, c, e1, false);
    // destroy
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_cost(d, e2, Some(cost_target(cost)));
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(operation_fn)));
    card::register_effect(d, c, e2, false);
}
fn ntcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // return minc==0 and c:GetLevel()>4 and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_SEASERPENT),c:GetControler(),LOCATION_MZONE,0,1,nil)
    let tp = card::get_controler(d, c);
    a.int(2) == 0
        && card::get_level(d, c) > 4
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_race(d, c, RACE_SEASERPENT))), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_WATER)
}
fn filter(d: &mut Duel, c: CardId, e: Option<EffectId>) -> bool {
    card::is_faceup(d, c) && (e.is_none() || card::is_can_be_effect_target(d, c, e))
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let dg = duel::get_matching_group(d, Some(&|d, c| filter(d, c, Some(a.e))), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&cfilter), 1, 1, false, Some(&|d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| aux::release_check_target(d, sg, tp, exg, &dg)), NO_EX);
    }
    let g = duel::select_release_group_cost(d, a.tp, Some(&cfilter), 1, 1, false, Some(&|d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| aux::release_check_target(d, sg, tp, exg, &dg)), NO_EX);
    duel::release(d, g, REASON_COST, None);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - a.tp) && filter(d, chkc, None);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, None)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, None)), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
