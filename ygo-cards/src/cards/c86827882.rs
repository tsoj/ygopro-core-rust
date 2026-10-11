//! Nordic Relic Megingjord
use super::prelude::*;

pub const ID: u32 = 86827882;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NORDIC, SET_AESIR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Targeted "Aesir" or "Nordic" monster has its ATK/DEF doubled, but cannot attack directly
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    effect::set_condition(d, e1, Some(condition(stat_change_cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn stat_change_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    aux::stat_change_damage_step_condition(d)
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_NORDIC as u16, SET_AESIR as u16])
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    let mut exc = None;
    if duel::is_phase(d, PHASE_DAMAGE) && duel::get_attack_target(d).is_none() {
        exc = duel::get_attacker(d);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, exc.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, exc.into());
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        let h = effect::get_handler(d, a.e).nil();
        // ATK/DEF becomes doubled of original ATK/DEF
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        let atk = card::get_base_attack(d, tc) * 2;
        effect::set_value(d, e1, atk as i64);
        card::register_effect(d, tc, e1, false);
        let e2 = effect::create_effect(d, h);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        let def = card::get_base_defense(d, tc) * 2;
        effect::set_value(d, e2, def as i64);
        card::register_effect(d, tc, e2, false);
        // Cannot attack directly this turn
        let e3 = effect::create_effect(d, h);
        effect::set_description(d, e3, 3207);
        effect::set_property(d, e3, EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e3, EFFECT_CANNOT_DIRECT_ATTACK);
        effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e3, false);
    }
}
