//! Human-Wave Tactics
use super::prelude::*;

pub const ID: u32 = 30353551;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, TIMING_END_PHASE, None);
    card::register_effect(d, c, e1, false);
    // special summon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
    aux::global_check(d, ID, |d| {
        set_s_var(d, ID, 0, Value::Int(0));
        set_s_var(d, ID, 1, Value::Int(0));
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_BATTLED);
        effect::set_operation(d, ge1, Some(operation(checkop)));
        duel::register_effect(d, ge1, 0);
        aux::add_values_reset(d, reset_values);
    });
}
fn reset_values(d: &mut Duel) {
    set_s_var(d, ID, 0, Value::Int(0));
    set_s_var(d, ID, 1, Value::Int(0));
}
fn bump(d: &mut Duel, p: u8) {
    let k = p as u64;
    let v = s_var(d, ID, k).int();
    set_s_var(d, ID, k, Value::Int(v + 1));
}
fn checkop(d: &mut Duel, _a: &EffArgs) {
    let a = duel::get_attacker(d).nil();
    let dd = duel::get_attack_target(d);
    if card::is_status(d, a, STATUS_BATTLE_DESTROYED) && card::is_type(d, a, TYPE_NORMAL) && card::is_level_below(d, a, 2) {
        let p = card::get_controler(d, a);
        bump(d, p);
    }
    if let Some(dd) = dd {
        if card::is_status(d, dd, STATUS_BATTLE_DESTROYED) && card::is_type(d, dd, TYPE_NORMAL) && card::is_level_below(d, dd, 2) {
            let p = card::get_controler(d, dd);
            bump(d, p);
        }
    }
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_type(d, c, TYPE_NORMAL) && card::is_level_below(d, c, 2) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let ct = s_var(d, ID, tp as u64).int();
    if a.chk == Some(0) {
        return ct > 0 && (ct == 1 || duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none());
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, ct as u8, tp, LOCATION_DECK as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    let tp = a.tp;
    let ct = s_var(d, ID, tp as u64).int();
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let mut g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, ct as u16, ct as u16, false, NO_EX).nil();
    if g.is_empty() {
        return;
    }
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    if ft > 1 && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    if ft <= 0 {
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    } else if ft as usize >= g.len() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    } else {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let sg = group::select(d, &g, tp, ft as u16, ft as u16, false, NO_EX).nil();
        duel::special_summon(d, sg.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None);
        group::remove_card(&mut g, &sg);
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
