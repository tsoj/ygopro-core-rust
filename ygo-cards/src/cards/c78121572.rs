//! Alchemist of Black Spells
use super::prelude::*;

pub const ID: u32 = 78121572;

pub const INFO: ScriptInfo = ScriptInfo { counter_list: &[COUNTER_SPELL as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // counter
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_COUNTER);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(con)));
    effect::set_target(d, e1, Some(cost_target(tg)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_position(d, h, POS_FACEUP_ATTACK)
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_can_add_counter(d, c, COUNTER_SPELL as u16, 1, false, 0)
}
fn tg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
    let _ = duel::select_target(d, tp, Some(&filter), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, 1, 0, COUNTER_SPELL as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d);
    if card::is_relate_to_effect(d, c, a.e) && card::is_position(d, c, POS_FACEUP_ATTACK) {
        duel::change_position(d, c, POS_FACEUP_DEFENSE as u8, Some(POS_FACEUP_DEFENSE as u8), Some(POS_FACEUP_DEFENSE as u8), Some(POS_FACEUP_DEFENSE as u8), false);
        let tc = tc.nil();
        if card::is_relate_to_effect(d, tc, a.e) {
            card::add_counter(d, tc, COUNTER_SPELL as u16, 1, false);
        }
    }
}
