//! Karakuri Ninja mdl 7749 "Nanashick"
use super::prelude::*;

pub const ID: u32 = 50615578;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_KARAKURI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // must attack
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_MUST_ATTACK);
    card::register_effect(d, c, e1, false);
    // pos change
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_POSITION);
    effect::set_type(d, e3, EFFECT_TYPE_TRIGGER_F + EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EVENT_BE_BATTLE_TARGET);
    effect::set_operation(d, e3, Some(operation(posop)));
    card::register_effect(d, c, e3, false);
    // draw
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_category(d, e4, CATEGORY_DRAW);
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_type(d, e4, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_SINGLE);
    effect::set_code(d, e4, EVENT_SUMMON_SUCCESS);
    effect::set_target(d, e4, Some(cost_target(drtg)));
    effect::set_operation(d, e4, Some(operation(drop_op)));
    card::register_effect(d, c, e4, false);
}
fn posop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        duel::change_position(d, c, POS_FACEUP_DEFENSE as u8, Some(0), Some(POS_FACEUP_ATTACK as u8), Some(0), false);
    }
}
fn drfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_position(d, c, POS_FACEUP_DEFENSE) && card::is_set_card(d, c, &[SET_KARAKURI as u16])
}
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let ct = duel::get_matching_group_count(d, Some(&drfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    if a.chk == Some(0) {
        return ct > 0 && duel::is_player_can_draw(d, a.tp, ct as u32);
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, ct as u32);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, ct as i32);
    true
}
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let ct = duel::get_matching_group_count(d, Some(&drfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    duel::draw(d, p, ct as u32, REASON_EFFECT);
}
