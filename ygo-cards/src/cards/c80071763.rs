//! Dark Balter the Terrible
use super::prelude::*;

pub const ID: u32 = 80071763;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix(d, c, false, false, vec![fusion::Mat::Code(52860176), fusion::Mat::Code(38742075)]);
    // negate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DISABLE);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(discon)));
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    effect::set_target(d, e1, Some(cost_target(distg)));
    effect::set_operation(d, e1, Some(operation(disop)));
    card::register_effect(d, c, e1, false);
    // disable
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_BATTLED);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_operation(d, e2, Some(operation(disop2)));
    card::register_effect(d, c, e2, false);
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_status(d, c, STATUS_BATTLE_DESTROYED) {
        return false;
    }
    let re = a.re.nil();
    effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::get_active_type(d, re) == TYPE_SPELL && duel::is_chain_disablable(d, a.ev as u8)
}
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    true
}
fn disop(d: &mut Duel, a: &EffArgs) {
    duel::negate_effect(d, a.ev as u8);
}
fn disop2(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c);
    if let Some(bc) = bc {
        if card::is_status(d, bc, STATUS_BATTLE_DESTROYED) && !card::is_status(d, c, STATUS_BATTLE_DESTROYED) {
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_DISABLE);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_EXC_GRAVE, 1);
            card::register_effect(d, bc, e1, false);
            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD_EXC_GRAVE, 1);
            card::register_effect(d, bc, e2, false);
        }
    }
}
