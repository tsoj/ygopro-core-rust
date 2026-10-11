//! Timelord Progenitor Vorpgate
use super::prelude::*;

pub const ID: u32 = 67508932;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Synchro Summon Procedure
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Cannot be destroyed battle or effects
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    card::register_effect(d, c, e2, false);
    // No battle bamage from attacks involving this card
    let e3 = effect::clone(d, e1);
    effect::set_code(d, e3, EFFECT_AVOID_BATTLE_DAMAGE);
    card::register_effect(d, c, e3, false);
    // Banish all monsters your opponent controls
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 0));
    effect::set_category(d, e4, CATEGORY_REMOVE);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e4, EVENT_DAMAGE_STEP_END);
    effect::set_condition(d, e4, Some(condition(rmcond)));
    effect::set_target(d, e4, Some(cost_target(rmtg)));
    effect::set_operation(d, e4, Some(operation(rmop)));
    card::register_effect(d, c, e4, false);
    // Special summon banished monsters
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 1));
    effect::set_category(d, e5, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e5, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e5, LOCATION_MZONE);
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    effect::set_condition(
        d,
        e5,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::get_flag_effect(d, h, ID + 1) > 0
        })),
    );
    effect::set_target(d, e5, Some(cost_target(sptg)));
    effect::set_operation(d, e5, Some(operation(spop)));
    card::register_effect(d, c, e5, false);
}
fn rmcond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_summon_location(d, h) & LOCATION_EXTRA == LOCATION_EXTRA
}
fn is_able_to_remove(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_remove(d, c, None, None, None)
}
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&is_able_to_remove), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    let g = duel::get_matching_group(d, Some(&is_able_to_remove), tp, 0, LOCATION_MZONE, NO_EX);
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, LOCATION_MZONE as u8, (1 - tp) as i32);
    true
}
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&is_able_to_remove), tp, 0, LOCATION_MZONE, NO_EX);
    if duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 {
        let og = duel::get_operated_group(d);
        for oc in group::iter(d, &og) {
            let fid = card::get_field_id(d, c);
            card::register_flag_effect(d, oc, ID, RESETS_STANDARD_PHASE_END, 0, 1, fid as i64, 0);
        }
        card::register_flag_effect(d, c, ID + 1, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CHANGE_BATTLE_DAMAGE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 0, 1);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    effect::set_value(d, e1, HALF_DAMAGE as i64);
    duel::register_effect(d, e1, tp);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    let h = effect::get_handler(d, e).nil();
    let fid = card::get_field_id(d, h);
    card::get_flag_effect_label(d, c, ID).first().copied() == Some(fid as i64)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let (e, tp) = (a.e, a.tp);
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_REMOVED, LOCATION_REMOVED, NO_EX);
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), n, tp, LOCATION_REMOVED as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let mut ft = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0;
    let tg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_REMOVED, LOCATION_REMOVED, NO_EX);
    if ft <= 0 || tg.is_empty() {
        return;
    }
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = group::select(d, &tg, tp, ft as u16, ft as u16, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, 1 - tp, false, false, POS_FACEUP as u8, None);
    }
}
