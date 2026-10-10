//! Dekoichi the Battlechanted Locomotive
use super::prelude::*;

pub const ID: u32 = 87621407;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[8715625], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[8715625])
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let ct = duel::get_matching_group_count(d, Some(&filter), a.tp, LOCATION_ONFIELD, 0, NO_EX) as u32;
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, ct + 1);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, (ct + 1) as i32);
    true
}
fn op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let n = info[1].int() as u32;
    duel::draw(d, p, n, REASON_EFFECT);
}
