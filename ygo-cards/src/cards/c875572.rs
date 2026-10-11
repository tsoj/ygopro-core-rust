//! Crowley, the Gifted Magistus
use super::prelude::*;

pub const ID: u32 = 875572;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[86120751], listed_series: &[SET_MAGISTUS, SET_INVOKED], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If this face-up card is used as Fusion Material, its name can be treated as "Aleister the Invoker"
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e1, EFFECT_ADD_CODE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value(d, e1, 86120751);
    effect::set_operation(d, e1, Some(lua_fn(e1_op)));
    card::register_effect(d, c, e1, false);
    // Special Summon this card when it is added to the hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_TO_HAND);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(lua_fn(e2_cond)));
    effect::set_target(d, e2, Some(cost_target(selfsptg)));
    effect::set_operation(d, e2, Some(operation(selfspop)));
    card::register_effect(d, c, e2, false);
    // Fusion Summon 1 "Magistus" or "Invoked" Fusion Monster from your Extra Deck, using monsters from your hand or field as material
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_condition(d, e3, Some(lua_fn(e3_cond)));
    let tg = fusion::summon_eff_tg(d, fusion::SummonEff { fusfilter: Some(fusion::fus_filter(fus_filter_fn)), ..Default::default() });
    effect::set_target(d, e3, Some(tg));
    let op = fusion::summon_eff_op(d, fusion::SummonEff { fusfilter: Some(fusion::fus_filter(fus_filter_fn)), ..Default::default() });
    effect::set_operation(d, e3, Some(op));
    card::register_effect(d, c, e3, false);
    let e4 = effect::clone(d, e3);
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e4, false);
}
fn e1_op(_d: &mut Duel, a: Args) -> bool {
    let sumtype = a.int(1);
    (sumtype & MATERIAL_FUSION as i64) > 0 || (sumtype & SUMMON_TYPE_FUSION as i64) > 0
}
fn e2_cond(d: &mut Duel, a: Args) -> bool {
    let c = effect::get_handler(d, a.e(0)).nil();
    !card::is_reason(d, c, REASON_DRAW)
}
fn e3_cond(d: &mut Duel, _a: Args) -> bool {
    !duel::is_phase(d, PHASE_DAMAGE)
}
fn fus_filter_fn(d: &mut Duel, c: CardId, _tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_MAGISTUS as u16, SET_INVOKED as u16])
}
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn selfspop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
