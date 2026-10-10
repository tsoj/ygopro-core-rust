//! Tindangle Acute Cerberus
use super::prelude::*;

pub const ID: u32 = 75119040;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[94365540, 75119041], listed_series: &[SET_TINDANGLE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_TINDANGLE as u16], sc, st, Some(tp)))),
        3,
        Some(3),
        None,
        None,
        None,
    );
    // atk
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(atkcon)));
    effect::set_value(d, e1, 3000);
    card::register_effect(d, c, e1, false);
    // atk
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    card::register_effect(d, c, e2, false);
    // special summon
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e3, EVENT_PHASE | PHASE_BATTLE);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(spcon)));
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_TINDANGLE as u16]) && card::is_monster(d, c)
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let p = card::get_controler(d, h);
    let g = duel::get_matching_group(d, Some(&cfilter), p, LOCATION_GRAVE, 0, NO_EX);
    group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 3 && group::is_exists(d, &g, &|d, c| card::is_code(d, c, &[94365540]), 1, NO_EX)
}
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_TINDANGLE as u16])
}
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    let lg = card::get_linked_group(d, c);
    group::filter_count(d, &lg, &atkfilter, NO_EX) as i64 * 500
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_attack_announced_count(d, h) > 0
}
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_TINDANGLE as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(1),
        race: Some(RACE_FIEND),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), None, None, 0)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), None, None, 0)
    {
        let token = duel::create_token(d, tp, ID + 1).nil();
        duel::special_summon(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
