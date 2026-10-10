//! Aqua Spirit
use super::prelude::*;

pub const ID: u32 = 40916023;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    card::register_effect(d, c, e1, false);
    // pos
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    effect::set_condition(d, e2, Some(condition(poscon)));
    effect::set_target(d, e2, Some(cost_target(postg)));
    effect::set_operation(d, e2, Some(operation(posop)));
    card::register_effect(d, c, e2, false);
}
fn spfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_WATER)
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, false)
        && (duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            || (card::is_location(d, c, LOCATION_MZONE) && card::get_sequence(d, c) < 5))
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    if a.card(1).is_none() {
        return true;
    }
    let tp = effect::get_handler_player(d, e);
    let rg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    ft > -1
        && !rg.is_empty()
        && matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(1), None, 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let _c = effect::get_handler(d, e);
    let rg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(1), None, 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true)
        .unwrap_or_default();
    if !g.is_empty() {
        // g:KeepAlive() (groups are values in Rust)
        effect::set_label_object(d, e, Value::Group(g));
        return true;
    }
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else { return };
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // g:DeleteGroup()
}
fn poscon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && card::is_faceup(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g), n, 0, 0);
    true
}
fn posop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            duel::change_position(d, tc, POS_FACEUP_DEFENSE as u8, Some(0), Some(POS_FACEUP_ATTACK as u8), Some(0), false);
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_CANNOT_CHANGE_POSITION);
            effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TURN_SET), 1);
            card::register_effect(d, tc, e1, false);
        }
    }
}
