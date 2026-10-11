//! Armor Ninjitsu Art of Alchemy
use super::prelude::*;

pub const ID: u32 = 16272453;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NINJITSU_ART], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DRAW | CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn set_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_NINJITSU_ART as u16])
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&set_faceup), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 2);
    }
    let c = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&set_faceup), tp, LOCATION_ONFIELD, 0, c.into());
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 2);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&set_faceup), tp, LOCATION_ONFIELD, 0, c.into());
    let ct = duel::destroy(d, g, REASON_EFFECT, None, None);
    if ct > 0 {
        duel::break_effect(d);
        duel::draw(d, tp, 2, REASON_EFFECT);
    }
}
