//! Jurrac Titano
use super::prelude::*;

pub const ID: u32 = 85028288;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_JURRAC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot special summon
    let e1 = effect::create_effect(d, c);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e1, false);
    // cannot be target
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_value_fn(d, e2, lua_fn(efilter));
    card::register_effect(d, c, e2, false);
    // atkup
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_cost(d, e3, Some(cost_target(atkcost)));
    effect::set_operation(d, e3, Some(operation(atkop)));
    card::register_effect(d, c, e3, false);
}
/// `function(e,re,rp) return re:GetHandler():IsType(TYPE_TRAP+TYPE_MONSTER) end`
fn efilter(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    let h = effect::get_handler(d, re).nil();
    card::is_type(d, h, TYPE_TRAP + TYPE_MONSTER)
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_attack_below(d, c, 1700)
        && card::is_set_card(d, c, &[SET_JURRAC as u16])
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, false)
}
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, c.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&cfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, c.into()).nil();
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, 1000);
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        card::register_effect(d, c, e1, false);
    }
}
