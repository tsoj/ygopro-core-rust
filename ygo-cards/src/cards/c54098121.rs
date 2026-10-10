//! Mysterious Puppeteer
use super::prelude::*;

pub const ID: u32 = 54098121;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // recover
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_RECOVER);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(con)));
    effect::set_target(d, e1, Some(cost_target(tg)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_FLIP_SUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);
}
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // return not eg:IsContains(e:GetHandler())
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    !group::is_contains(&eg, h)
}
fn tg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, 500);
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, 500);
    true
}
fn op(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::recover(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
