//! Black Feather Whirlwind
use super::prelude::*;

pub const ID: u32 = 7602800;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_BLACK_WINGED_DRAGON],
    listed_series: &[SET_BLACKWING as u32],
    counter_list: &[COUNTER_FEATHER as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Special Summon 1 monster that is banished or in the GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // Replace destruction
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_DESTROY_REPLACE);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_target(d, e3, Some(cost_target(desreptg)));
    effect::set_value_fn(d, e3, effect_card_fn(desrepval));
    effect::set_operation(d, e3, Some(operation(desrepop)));
    card::register_effect(d, c, e3, false);
}
fn spcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_faceup(d, c)
        && card::is_summon_player(d, c, tp)
        && card::is_attribute(d, c, ATTRIBUTE_DARK)
        && card::is_type(d, c, TYPE_SYNCHRO)
        && card::is_summon_location(d, c, LOCATION_EXTRA)
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| spcfilter(d, c, a.tp), 1, NO_EX)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, atk: i64) -> bool {
    (card::is_set_card(d, c, &[SET_BLACKWING as u16]) || card::is_code(d, c, &[CARD_BLACK_WINGED_DRAGON]))
        && card::is_faceup(d, c)
        && (card::get_attack(d, c) as i64) < atk
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let eg = a.eg.clone().nil();
    let filtered = group::filter(d, &eg, &|d, c| spcfilter(d, c, tp), NO_EX);
    let atk = match group::get_max_group(d, &filtered, &|d, c| card::get_attack(d, c) as i64) {
        Some((_, v)) => v,
        None => 0,
    };
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE | LOCATION_REMOVED) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, e, tp, atk);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp, atk)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp, atk)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, effect::get_handler(d, a.e).nil(), a.e) && card::is_relate_to_effect(d, tc, a.e) {
        duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_controler(d, c, tp)
        && card::is_attribute(d, c, ATTRIBUTE_DARK)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_faceup(d, c)
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && !card::is_reason(d, c, REASON_REPLACE)
}
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return group::is_exists(d, &eg, &|d, x| repfilter(d, x, tp), 1, NO_EX)
            && duel::is_can_remove_counter(d, tp, 1, 0, COUNTER_FEATHER as u16, 1, REASON_EFFECT);
    }
    duel::select_effect_yes_no(d, tp, c, Some(96))
}
fn desrepval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let p = effect::get_handler_player(d, e);
    repfilter(d, c, p)
}
fn desrepop(d: &mut Duel, a: &EffArgs) {
    duel::remove_counter(d, a.tp, 1, 0, COUNTER_FEATHER as u16, 1, REASON_EFFECT | REASON_REPLACE);
}
