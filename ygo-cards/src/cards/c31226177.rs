//! Imduk the World Chalice Dragon
use super::prelude::*;

pub const ID: u32 = 31226177;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WORLD_CHALICE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    card::enable_revive_limit(d, c);
    link::add_procedure(d, c, Some(mat_filter(matfilter)), 1, Some(1), None, None, None);
    // extra summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_EXTRA_SUMMON_COUNT);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, LOCATION_HAND | LOCATION_MZONE, 0);
    effect::set_target(d, e1, Some(effect_card_fn(set_world_chalice_target)));
    card::register_effect(d, c, e1, false);
    // destroy
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_BATTLE_START);
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
    // spsummon
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    effect::set_condition(d, e3, Some(condition(spcon2)));
    effect::set_target(d, e3, Some(cost_target(sptg2)));
    effect::set_operation(d, e3, Some(operation(spop2)));
    card::register_effect(d, c, e3, false);
}
/// `aux.TargetBoolFunction(Card.IsSetCard,SET_WORLD_CHALICE)`
fn set_world_chalice_target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_WORLD_CHALICE as u16])
}
/// `s.matfilter(c,lc,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    card::is_type_ctx(d, c, TYPE_NORMAL, lc, sumtype, Some(tp)) && !card::is_type_ctx(d, c, TYPE_TOKEN, lc, sumtype, Some(tp))
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c);
    if a.chk == Some(0) {
        return match bc {
            Some(bc) => {
                let lg = card::get_linked_group(d, c);
                group::is_contains(&lg, bc)
            }
            None => false,
        };
    }
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, bc.map(CardSet::from_card), 1, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c).nil();
    if card::is_relate_to_battle(d, bc) {
        duel::destroy(d, bc, REASON_EFFECT, None, None);
    }
}
fn spcon2(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_ONFIELD)
}
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_WORLD_CHALICE as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter2(d, x, a.e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
fn spop2(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter2(d, x, a.e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
