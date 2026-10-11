//! Majestic Mirage
use super::prelude::*;

pub const ID: u32 = 98020526;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID, CARD_STARDUST_DRAGON],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Apply effect
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_REMOVE);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
}
fn lffilter(d: &mut Duel, c: CardId, tp: u8, re: Option<EffectId>) -> bool {
    card::is_previous_position(d, c, POS_FACEUP as u32)
        && card::is_previous_controler(d, c, tp)
        && (card::is_reason(d, c, REASON_EFFECT)
            || (card::is_reason(d, c, REASON_COST) && re.is_some_and(|re| effect::is_activated(d, re))))
        && (card::is_code(d, c, &[CARD_STARDUST_DRAGON])
            || (card::lists_code(d, c, &[CARD_STARDUST_DRAGON]) && card::is_type(d, c, TYPE_SYNCHRO)))
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    a.rp == tp && {
        let eg = a.eg.clone().nil();
        group::is_exists(d, &eg, &|d, c| lffilter(d, c, tp, a.re), 1, NO_EX)
    }
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_faceup(d, c)
        && card::is_location(d, c, LOCATION_GRAVE | LOCATION_REMOVED)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_remove(d, c, None, None, None) && card::is_monster(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let eg = a.eg.clone().nil();
    let g0 = group::filter(d, &eg, &|d, c| lffilter(d, c, tp, a.re), NO_EX);
    let g = group::filter(d, &g0, &|d, c| spfilter(d, c, e, tp), NO_EX);
    let b1 = !g.is_empty()
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::get_flag_effect(d, tp, ID) == 0;
    let b2 = duel::is_existing_matching_card(d, Some(&rmfilter), tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX)
        && duel::get_flag_effect(d, tp, ID + 1) == 0;
    let b3 = duel::get_flag_effect(d, tp, ID + 2) == 0;
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    duel::set_target_card(d, g);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - tp, (LOCATION_MZONE | LOCATION_GRAVE) as i32);
    true
}
/// `function(e,re,ev) return math.floor(ev/2) end`
fn halve(_d: &mut Duel, a: Args) -> i64 {
    a.int(2).div_euclid(2)
}
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let eg = a.eg.clone().nil();
    let g0 = group::filter(d, &eg, &|d, c| lffilter(d, c, tp, a.re), NO_EX);
    let g1 = group::filter(d, &g0, &|d, c| spfilter(d, c, e, tp), NO_EX);
    let g = group::filter(d, &g1, &|d, c| card::is_relate_to_effect(d, c, e), NO_EX);
    let b1 = !g.is_empty()
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::get_flag_effect(d, tp, ID) == 0;
    let b2 = duel::is_existing_matching_card(d, Some(&rmfilter), tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX)
        && duel::get_flag_effect(d, tp, ID + 1) == 0;
    let b3 = duel::get_flag_effect(d, tp, ID + 2) == 0;
    if !(b1 || b2 || b3) {
        return;
    }
    let choice = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))]);
    if choice == Some(1) {
        duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        if sg.is_empty() || duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None) == 0 {
            return;
        }
    } else if choice == Some(2) {
        duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        let sg = duel::select_matching_card(d, tp, Some(&rmfilter), tp, 0, LOCATION_GRAVE | LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        if sg.is_empty() {
            return;
        }
        duel::hint_selection(d, sg.clone(), true);
        duel::remove(d, sg, POS_FACEUP as u8, REASON_EFFECT, None, None);
    } else if choice == Some(3) {
        duel::register_flag_effect(d, tp, ID + 2, RESET_PHASE | PHASE_END, 0, 1, 0);
        // Halve damage
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_description(d, e1, aux::stringid(ID, 4));
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
        effect::set_target_range(d, e1, 1, 0);
        effect::set_value_fn(d, e1, lua_fn(halve));
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        duel::register_effect(d, e1, tp);
    }
}
