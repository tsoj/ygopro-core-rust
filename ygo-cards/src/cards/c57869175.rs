//! Fortune Lady Past
use super::prelude::*;

pub const ID: u32 = 57869175;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FORTUNE_LADY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Increase ATK/DEF by 200 per Level
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_SET_ATTACK);
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_SET_DEFENSE);
    card::register_effect(d, c, e2, false);
    // Increase its own level during the Standby Phase
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_code(d, e3, EVENT_PHASE | PHASE_STANDBY);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(lvcon)));
    effect::set_operation(d, e3, Some(operation(lvop)));
    card::register_effect(d, c, e3, false);
    // Increase the level of a "Fortune Lady" monster
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_category(d, e4, CATEGORY_REMOVE);
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    effect::set_target(d, e4, Some(cost_target(rmtg)));
    effect::set_operation(d, e4, Some(operation(rmop)));
    card::register_effect(d, c, e4, false);
}
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    card::get_level(d, c) as i64 * 200
}
fn lvcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    duel::is_turn_player(d, a.tp) && card::has_level(d, c) && card::is_level_below(d, c, 11)
}
fn lvop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, a.e) || card::is_level_above(d, c, 12) {
        return;
    }
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
    effect::set_value(d, e1, 1);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
    card::register_effect(d, c, e1, false);
}
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_race(d, c, RACE_SPELLCASTER) && card::is_able_to_remove(d, c, None, None, None) && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND))
}
fn lvfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_faceup(d, c)
        && card::has_level(d, c)
        && card::is_set_card(d, c, &[SET_FORTUNE_LADY as u16])
        && duel::is_existing_matching_card(d, Some(&rmfilter), tp, LOCATION_MZONE | LOCATION_GRAVE | LOCATION_HAND, 0, 1, c.into())
}
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && lvfilter(d, chkc, a.tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| lvfilter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_APPLYTO as u64);
    duel::select_target(d, a.tp, Some(&|d, c| lvfilter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 0, (LOCATION_HAND | LOCATION_GRAVE | LOCATION_MZONE) as i32);
    true
}
fn rmop(d: &mut Duel, a: &EffArgs) {
    let Some(tc) = duel::first_target(d) else { return };
    if !(card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e)) {
        return;
    }
    let ch = if card::get_level(d, tc) == 1 {
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0)])
    } else {
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)])
    };
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let zone = LOCATION_MZONE | LOCATION_HAND | LOCATION_GRAVE;
    let rg = if ch == 0 {
        duel::select_matching_card(d, a.tp, Some(&rmfilter), a.tp, zone, 0, 1, 99, false, tc.into())
    } else {
        let max = card::get_level(d, tc) as i64 - 1;
        duel::select_matching_card(d, a.tp, Some(&rmfilter), a.tp, zone, 0, 1, max as u16, false, tc.into())
    };
    let rg = rg.nil();
    if !rg.is_empty() {
        let mut ct = duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None);
        if ct > 0 {
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
            effect::set_value(d, e1, ct as i64);
            if Some(tc) == effect::get_handler(d, a.e) {
                effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
            } else {
                effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            }
            if ch == 1 {
                ct = -ct;
            }
            effect::set_value(d, e1, ct as i64);
            card::register_effect(d, tc, e1, false);
        }
    }
}
