//! Kaiser Vorse Raider
use super::prelude::*;

pub const ID: u32 = 93927067;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon proc
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    card::register_effect(d, c, e1, false);
    // atkup
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    effect::set_condition(d, e2, Some(aux::bdocon()));
    effect::set_operation(d, e2, Some(operation(atkop)));
    card::register_effect(d, c, e2, false);
    // atkdown
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYED);
    effect::set_operation(d, e3, Some(operation(desop)));
    card::register_effect(d, c, e3, false);
}
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    let tp = card::get_controler(d, c);
    duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) == 0 && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, 500);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        card::register_effect(d, c, e1, false);
    }
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = card::get_battle_target(d, c).nil();
    if !card::is_relate_to_battle(d, tc) || card::is_facedown(d, tc) {
        return;
    }
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    effect::set_value(d, e1, -500);
    card::register_effect(d, tc, e1, false);
}
