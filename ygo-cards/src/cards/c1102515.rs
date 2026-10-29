//! Dark Mimic LV3
use super::prelude::*;

pub const ID: u32 = 1102515;

// s.listed_names={74713516}; s.LVnum=3 (no s.LVset: not declared, see report)
pub const INFO: ScriptInfo = ScriptInfo { lv: Some((3, SET_DARK_MIMIC)), listed_names: &[74713516], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // draw
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if card::get_summon_type(d, c) == SUMMON_TYPE_SPECIAL + 1 {
        effect::set_label(d, a.e, &[2]);
    } else {
        effect::set_label(d, a.e, &[1]);
    }
    card::is_location(d, c, LOCATION_GRAVE) && card::is_reason(d, c, REASON_BATTLE)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let label = effect::get_label(d, a.e);
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, label as u32);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, label as i32);
    true
}
fn op(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
