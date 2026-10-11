//! Element Doom
use super::prelude::*;

pub const ID: u32 = 23118924;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // disable
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_BATTLED);
    effect::set_condition(d, e1, Some(condition(discon)));
    effect::set_operation(d, e1, Some(operation(disop)));
    card::register_effect(d, c, e1, false);
    // chain attack
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    effect::set_condition(d, e2, Some(condition(atcon)));
    effect::set_operation(d, e2, Some(operation(atop)));
    card::register_effect(d, c, e2, false);
}
fn earth_filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_EARTH)
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c);
    match bc {
        Some(bc) => {
            card::is_status(d, bc, STATUS_BATTLE_DESTROYED)
                && !card::is_status(d, c, STATUS_BATTLE_DESTROYED)
                && duel::is_existing_matching_card(d, Some(&earth_filter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
        }
        None => false,
    }
}
fn disop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c).nil();
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
fn atcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let vals = vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ];
    if !aux::bdocon().call(d, &vals).truthy() {
        return false;
    }
    let c = effect::get_handler(d, a.e).nil();
    if !card::can_chain_attack(d, c, None, false) {
        return false;
    }
    duel::is_existing_matching_card(d, Some(&earth_filter_wind), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
fn earth_filter_wind(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_WIND)
}
fn atop(d: &mut Duel, _a: &EffArgs) {
    duel::chain_attack(d, None);
}
