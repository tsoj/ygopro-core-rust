//! Double Payback
use super::prelude::*;

pub const ID: u32 = 5914184;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[0x1a], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_counter_permit(d, c, 0x1a, None, None);
    // counter
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_DAMAGE);
    effect::set_condition(d, e1, Some(condition(actcon)));
    effect::set_operation(d, e1, Some(operation(actop)));
    card::register_effect(d, c, e1, false);
}
fn actcon(_d: &mut Duel, a: &EffArgs) -> bool {
    a.ep == a.tp && a.tp != a.rp && a.ev >= 1000 && (a.r & REASON_EFFECT) != 0
}
fn actop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        let ct = a.ev / 1000;
        card::add_counter(d, c, 0x1a, ct as u16, false);
        // damage
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, aux::stringid(ID, 0));
        effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        effect::set_range(d, e1, LOCATION_SZONE);
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        effect::set_condition(d, e1, Some(condition(damcon)));
        effect::set_target(d, e1, Some(cost_target(damtg)));
        effect::set_operation(d, e1, Some(operation(damop)));
        if duel::is_turn_player(d, a.tp) {
            effect::set_label(d, e1, &[0]);
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 1);
        } else {
            let tc = duel::get_turn_count(d, None) as i64;
            effect::set_label(d, e1, &[tc]);
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 2);
        }
        card::register_effect(d, c, e1, false);
    }
}
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp) && (duel::get_turn_count(d, None) as i64) != effect::get_label(d, a.e)
}
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, 1 - a.tp);
    let h = effect::get_handler(d, a.e).nil();
    let dam = card::get_counter(d, h, 0x1a) as i64 * 2000;
    duel::set_target_param(d, dam as u32);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(h)), 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dam as i32);
    true
}
fn damop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    if duel::destroy(d, h, REASON_EFFECT, None, None) != 0 {
        let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
        duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
    }
}
