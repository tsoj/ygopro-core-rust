//! Metaphys Ragnarok
use super::prelude::*;

pub const ID: u32 = 19476824;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_METAPHYS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // remove
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    effect::set_operation(d, e1, Some(operation(rmop)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);
    // spsummon
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e3, EVENT_BATTLE_DAMAGE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_condition(d, e3, Some(condition(spcon)));
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
}
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let rg = duel::get_decktop_group(d, a.tp, 3);
    if a.chk == Some(0) {
        return group::filter_count(d, &rg, &|d, c| card::is_able_to_remove(d, c, None, None, None), NO_EX) == 3;
    }
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(rg), 3, 0, 0);
    true
}
fn rmop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let g = duel::get_decktop_group(d, a.tp, 3);
    if !g.is_empty() && duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None) != 0 && card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        let og = duel::get_operated_group(d);
        let oc = group::filter_count(d, &og, &|d, c| card::is_set_card(d, c, &[SET_METAPHYS as u16]), NO_EX);
        if oc == 0 {
            return;
        }
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, (oc * 300) as i64);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        card::register_effect(d, c, e1, false);
    }
}
fn spcon(_d: &mut Duel, a: &EffArgs) -> bool {
    a.ep != a.tp
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_METAPHYS as u16]) && card::is_level_above(d, c, 5) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    if let Some(tc) = tc {
        if duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) != 0 {
            card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
            let h = effect::get_handler(d, a.e).nil();
            let e2 = effect::create_effect(d, h);
            effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
            effect::set_count_limit(d, e2, 1, 0, 0, 0);
            effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
            let turn = duel::get_turn_count(d, None) as i64;
            effect::set_label(d, e2, &[turn + 1]);
            effect::set_label_object(d, e2, Value::Card(tc));
            effect::set_condition(d, e2, Some(condition(descon)));
            effect::set_operation(d, e2, Some(operation(desop)));
            duel::register_effect(d, e2, a.tp);
        }
    }
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    if card::get_flag_effect(d, tc, ID) != 0 {
        duel::get_turn_count(d, None) as i64 == effect::get_label(d, a.e)
    } else {
        effect::reset(d, a.e);
        false
    }
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
