//! Great Maju Garzett
use super::prelude::*;

pub const ID: u32 = 47942531;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Tribute check
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_MATERIAL_CHECK);
    effect::set_value_fn(d, e1, lua_fn(valcheck));
    card::register_effect(d, c, e1, false);
    // The ATK of this card becomes twice the original ATK of 1 monster that you Tributed for its Tribute Summon
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_SUMMON_COST);
    effect::set_operation(d, e2, Some(operation(facechk)));
    effect::set_label_object(d, e2, Value::Effect(e1));
    card::register_effect(d, c, e2, false);
}
fn valcheck(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let c = a.c(1);
    let g = card::get_material(d, c);
    let tc = group::get_first(d, &g);
    let mut atk: i64 = 0;
    if let Some(tc) = tc {
        atk = card::get_text_attack(d, tc) as i64 * 2;
    }
    if atk < 0 {
        atk = 0;
    }
    if effect::get_label(d, e) == 1 {
        effect::set_label(d, e, &[0]);
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SET_ATTACK);
        effect::set_value(d, e1, atk);
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
        card::register_effect(d, c, e1, false);
    }
}
fn facechk(d: &mut Duel, a: &EffArgs) {
    let le = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_label(d, le, &[1]);
}
