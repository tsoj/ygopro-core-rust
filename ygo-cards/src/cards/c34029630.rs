//! Pitch-Black Power Stone
use super::prelude::*;

pub const ID: u32 = 34029630;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_SPELL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_counter_permit(d, c, COUNTER_SPELL as u16, None, None);
    // Activate
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_COUNTER);
    effect::set_type(d, e2, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
    // counter
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_COUNTER);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(cond)));
    effect::set_target(d, e3, Some(cost_target(target2)));
    effect::set_operation(d, e3, Some(operation(op)));
    card::register_effect(d, c, e3, false);
    // self destroy
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_code(d, e4, EFFECT_SELF_DESTROY);
    effect::set_condition(d, e4, Some(lua_fn(descon)));
    card::register_effect(d, c, e4, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && filter(d, chkc);
    }
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_can_add_counter(d, a.tp, Some((COUNTER_SPELL as u16, 3, c)));
    }
    card::add_counter(d, c, COUNTER_SPELL as u16, 3, false);
    if !duel::is_duel_type(d, DUEL_USE_TRAPS_IN_NEW_CHAIN) && cond(d, a) && target2(d, &EffArgs { chk: Some(0), ..a.clone() }) && duel::select_effect_yes_no(d, a.tp, c, Some(94)) {
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
        duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, c.into());
        card::register_flag_effect(d, c, 0, RESET_CHAIN, EFFECT_FLAG_CLIENT_HINT, 1, 0, 65);
        card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    } else {
        effect::set_property(d, a.e, 0, 0);
    }
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_can_remove_counter(d, c, a.tp, COUNTER_SPELL as u16, 1, REASON_EFFECT) && card::is_can_add_counter(d, tc, COUNTER_SPELL as u16, 1, false, 0) {
            card::remove_counter(d, c, a.tp, COUNTER_SPELL as u16, 1, REASON_EFFECT);
            card::add_counter(d, tc, COUNTER_SPELL as u16, 1, false);
        }
    }
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_can_add_counter(d, c, COUNTER_SPELL as u16, 1, false, 0)
}
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && filter(d, chkc);
    }
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::get_flag_effect(d, c, ID) == 0
            && card::is_can_remove_counter(d, c, a.tp, COUNTER_SPELL as u16, 1, REASON_EFFECT)
            && duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
    duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, c.into());
    true
}
fn descon(d: &mut Duel, a: Args) -> bool {
    let c = effect::get_handler(d, a.e(0)).nil();
    card::get_counter(d, c, COUNTER_SPELL as u16) == 0
}
