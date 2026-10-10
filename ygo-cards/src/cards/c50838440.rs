//! Three in One
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 50838440;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 3 Normal Monsters from your GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Set 1 appropriate Spell/Trap from your hand or Deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SET);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(settg)));
    effect::set_operation(d, e2, Some(operation(setop)));
    card::register_effect(d, c, e2, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    duel::is_phase(d, PHASE_END)
        && duel::is_turn_player(d, 1 - tp)
        && duel::get_field_group_count(d, 1 - tp, LOCATION_HAND | LOCATION_ONFIELD, 0)
            > duel::get_field_group_count(d, tp, LOCATION_HAND | LOCATION_ONFIELD, 0)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_type(d, c, TYPE_NORMAL) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 2
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 3, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 3, tp, LOCATION_GRAVE as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 3
        || duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()
    {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 3, 3, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
fn code_of(d: &mut Duel, c: CardId) -> i64 {
    card::get_original_code_rule(d, c).first().copied().map_or(0, |x| x as i64)
}
fn setfilter(d: &mut Duel, c: CardId, codes: &[i64]) -> bool {
    let code = |i: usize| codes.get(i).copied().nil() as u32;
    card::is_spell_trap(d, c)
        && card::is_s_setable(d, c, false, None)
        && card::lists_code(d, c, &[code(0)])
        && card::lists_code(d, c, &[code(1)])
        && card::lists_code(d, c, &[code(2)])
}
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let res = sg.len() == 3
        && group::get_class_count(d, sg, &|d, c| code_of(d, c)) == 3 && {
            let classes = group::get_class(d, sg, &|d, c| code_of(d, c));
            duel::is_existing_matching_card(d, Some(&|d, c| setfilter(d, c, &classes)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX)
        };
    (res, false)
}
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chkc().is_some() {
        return true;
    }
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_monster_card(d, c)), tp, LOCATION_ONFIELD, 0, NO_EX);
    if a.chk == Some(0) {
        return aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&rescon), 0, tp, None, None, None, false).is_err_and(|v| v);
    }
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&rescon), 1, tp, Some(HINTMSG_TARGET as u64), None, None, false).unwrap_or_default();
    duel::set_target_card(d, tg);
    true
}
fn setop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let tg = duel::get_target_cards(d, e).nil();
    if tg.len() != 3 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    let codes = group::get_class(d, &tg, &|d, c| code_of(d, c));
    let g = duel::select_matching_card(d, tp, Some(&|d, c| setfilter(d, c, &codes)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::s_set(d, tp, g, None, true);
    }
}
