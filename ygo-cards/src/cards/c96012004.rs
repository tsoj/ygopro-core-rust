//! Lucky Chance
use super::prelude::*;

pub const ID: u32 = 96012004;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Toss a coin and draw 1 card
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DRAW);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_F);
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_condition(d, e2, Some(condition(coincon)));
    effect::set_operation(d, e2, Some(operation(coinop)));
    card::register_effect(d, c, e2, false);
}
fn coincon(d: &mut Duel, a: &EffArgs) -> bool {
    let info = duel::get_operation_info(d, a.ev, CATEGORY_COIN);
    if let Some(info) = info {
        if info.op_param == 1 && effect::is_active_type(d, a.re.nil(), TYPE_MONSTER) {
            effect::set_label_object(d, a.e, Value::Effect(a.re.nil()));
            return true;
        }
    }
    false
}
fn coinop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let res = duel::announce_coin(d, a.tp, None);
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_TOSS_COIN);
    effect::set_condition(d, e1, Some(condition(drcon)));
    effect::set_operation(d, e1, Some(operation(drop_op)));
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_reset(d, e1, RESET_CHAIN, 1);
    let label_object = effect::get_label_object(d, a.e);
    effect::set_label_object(d, e1, label_object);
    effect::set_label(d, e1, &[res as i64]);
    duel::register_effect(d, e1, a.tp);
}
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    let results = duel::get_coin_result(d);
    let res = results.first().copied();
    let extra = results.get(1).copied();
    extra.is_none()
        && a.re == effect::get_label_object(d, a.e).as_effect()
        && res.map(|r| r as i64) == Some(effect::get_label(d, a.e))
}
fn drop_op(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    duel::draw(d, a.tp, 1, REASON_EFFECT);
}
