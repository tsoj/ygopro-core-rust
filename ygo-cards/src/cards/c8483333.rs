//! Mosaic Manticore
use super::prelude::*;

pub const ID: u32 = 8483333;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // remove
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(condition(regcon)));
    effect::set_operation(d, e1, Some(operation(regop)));
    card::register_effect(d, c, e1, false);
}
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, c)
}
fn regop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 1);
    card::register_effect(d, c, e1, false);
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rc: CardId) -> bool {
    card::is_location(d, c, LOCATION_GRAVE)
        && card::is_reason(d, c, REASON_SUMMON)
        && card::get_reason_card(d, c) == Some(rc)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let c = effect::get_handler(d, a.e).nil();
    let mat = card::get_material(d, c);
    let g = group::filter(d, &mat, &|d, x| spfilter(d, x, a.e, a.tp, c), NO_EX);
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), n, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    if ft <= 0 {
        return;
    }
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    let c = effect::get_handler(d, a.e).nil();
    let mat = card::get_material(d, c);
    let mut g = group::filter(d, &mat, &|d, x| spfilter(d, x, a.e, tp, c), NO_EX);
    if !g.is_empty() {
        if g.len() as i32 > ft {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            g = group::select(d, &g, tp, ft as u16, ft as u16, false, NO_EX).nil();
        }
        let _ = group::get_first(d, &g);
        for tc in group::iter(d, &g) {
            duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_CANNOT_ATTACK_ANNOUNCE);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e1, false);
            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e2, EFFECT_DISABLE);
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e2, false);
            let e3 = effect::create_effect(d, c);
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e3, EFFECT_DISABLE_EFFECT);
            effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e3, false);
        }
        duel::special_summon_complete(d);
    }
}
