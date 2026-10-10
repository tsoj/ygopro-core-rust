//! Gryphon Wing
use super::prelude::*;

pub const ID: u32 = 55608151;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[18144506], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DISABLE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp~=tp and re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:GetHandler():IsCode(18144506) and Duel.IsChainDisablable(ev)
    if a.rp == a.tp {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        return false;
    }
    let h = effect::get_handler(d, re).nil();
    card::is_code(d, h, &[18144506]) && duel::is_chain_disablable(d, a.ev as u8)
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_spell_trap(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let tp = a.tp;
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    let g = duel::get_matching_group(d, Some(&filter), tp, 0, LOCATION_ONFIELD, NO_EX);
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::negate_effect(d, a.ev as u8) {
        let g = duel::get_matching_group(d, Some(&filter), tp, 0, LOCATION_ONFIELD, NO_EX);
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
