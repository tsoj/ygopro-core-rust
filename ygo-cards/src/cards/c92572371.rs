//! Brotherhood of the Fire Fist - Buffalo
use super::prelude::*;

pub const ID: u32 = 92572371;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FIRE_FIST, SET_FIRE_FORMATION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_GRAVE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost::replaceable(cost_target(spcost), Some(lua_fn(replace_extracon)))));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
}
fn replace_extracon(d: &mut Duel, a: Args) -> bool {
    // function(_,_,tp) return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end
    duel::get_location_count(d, a.p(2), LOCATION_MZONE as u8, None, None, None).0 > 0
}
fn spcostfilter(d: &mut Duel, c: CardId) -> bool {
    (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
        && card::is_set_card(d, c, &[SET_FIRE_FIST as u16, SET_FIRE_FORMATION as u16])
        && card::is_able_to_grave_as_cost(d, c)
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let g = duel::get_matching_group(d, Some(&spcostfilter), a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0, NO_EX);
    if a.chk == Some(0) {
        return matches!(
            aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&aux::chkf_mmz(1)), 0, a.tp, None, None, None, false),
            Err(true)
        );
    }
    let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&aux::chkf_mmz(1)), 1, a.tp, Some(HINTMSG_TOGRAVE as u64), None, None, false)
        .unwrap_or_default();
    duel::send_to_grave(d, sg, REASON_COST, None, None);
    true
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        duel::special_summon(d, h, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
