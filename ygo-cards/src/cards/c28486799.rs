//! Backup Squad
use super::prelude::*;

pub const ID: u32 = 28486799;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // draw
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_DRAW);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_DAMAGE);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_condition(d, e2, Some(condition(drcon)));
    effect::set_target(d, e2, Some(cost_target(drtg)));
    effect::set_operation(d, e2, Some(operation(drop_op)));
    card::register_effect(d, c, e2, false);
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    if !(a.ep == a.tp && a.rp != a.tp && a.ev >= 1000) {
        return false;
    }
    // (re or (Duel.GetAttacker() and Duel.GetAttacker():IsControler(1-tp)))
    if a.re.is_some() {
        return true;
    }
    match duel::get_attacker(d) {
        Some(atk) => card::is_controler(d, atk, 1 - a.tp),
        None => false,
    }
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local d=math.floor(ev/1000)
    let dr = a.ev.div_euclid(1000);
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, dr as u32);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, dr as i32);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let dr = info[1].int();
    if dr > 0 {
        let _ = duel::draw(d, p, dr as u32, REASON_EFFECT);
    }
}
