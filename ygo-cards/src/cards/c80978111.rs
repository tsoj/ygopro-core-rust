//! Flying "C"
use super::prelude::*;

pub const ID: u32 = 80978111;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);

    // Opponent cannot Special Summon monsters except by Xyz Summon
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_code(d, e3, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e3, 1, 0);
    effect::set_target(d, e3, Some(lua_fn(splimit)));
    card::register_effect(d, c, e3, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let g = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &g, &|d, c| card::is_summon_player(d, c, 1 - tp), 1, NO_EX)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, a.e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), Some(1 - tp), None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), None, None).0 > 0 {
        duel::special_summon(d, c, 0, tp, 1 - tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
/// `function(e,c,tp,sumtp,sumpos)`
fn splimit(_d: &mut Duel, a: Args) -> bool {
    let sumtp = a.int(3) as u64;
    (sumtp & SUMMON_TYPE_XYZ as u64) == SUMMON_TYPE_XYZ as u64
}
