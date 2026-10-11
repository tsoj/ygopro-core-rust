//! Performapal Bot-Eyes Lizard
use super::prelude::*;

pub const ID: u32 = 72708264;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ODD_EYES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // copy
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(copycon)));
    effect::set_cost(d, e1, Some(cost_target(copycost)));
    effect::set_operation(d, e1, Some(operation(copyop)));
    card::register_effect(d, c, e1, false);
}
fn copycon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_status(d, c, STATUS_SUMMON_TURN + STATUS_SPSUMMON_TURN)
}
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ODD_EYES as u16]) && card::is_monster(d, c) && card::is_able_to_grave_as_cost(d, c)
}
fn copycost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&costfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    duel::send_to_grave(d, g.clone(), REASON_COST, None, None);
    let code = card::get_code(d, group::get_first(d, &g).nil())[0];
    effect::set_label(d, a.e, &[code as i64]);
    true
}
fn copyop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    effect::set_value(d, e1, effect::get_label(d, a.e));
    card::register_effect(d, c, e1, false);
}
