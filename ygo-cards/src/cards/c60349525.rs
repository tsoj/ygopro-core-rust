//! Cracking Dragon
use super::prelude::*;

pub const ID: u32 = 60349525;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot be destroyed by battle with a monster with equal or lower Level
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_value_fn(d, e1, lua_fn(indes_value));
    card::register_effect(d, c, e1, false);
    // When your opponent Normal or Special Summons exactly 1 monster ...: make that monster lose ATK
    let e2a = effect::create_effect(d, c);
    effect::set_description(d, e2a, aux::stringid(ID, 0));
    effect::set_category(d, e2a, CATEGORY_ATKCHANGE + CATEGORY_DAMAGE);
    effect::set_type(d, e2a, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2a, EVENT_SUMMON_SUCCESS);
    effect::set_range(d, e2a, LOCATION_MZONE);
    effect::set_condition(d, e2a, Some(condition(atkcon)));
    effect::set_target(d, e2a, Some(cost_target(atktg)));
    effect::set_operation(d, e2a, Some(operation(atkop)));
    card::register_effect(d, c, e2a, false);
    let e2b = effect::clone(d, e2a);
    effect::set_code(d, e2b, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e2b, false);
}
fn indes_value(d: &mut Duel, a: Args) -> bool {
    let (e, c) = (a.e(0), a.c(1));
    let lvl = card::get_level(d, effect::get_handler(d, e).nil());
    card::is_level_below(d, c, lvl)
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    if group::get_count(&eg) != 1 || group::is_contains(&eg, h) {
        return false;
    }
    let sc = group::get_first(d, &eg).nil();
    card::is_summon_player(d, sc, 1 - a.tp) && card::has_level(d, sc) && card::is_faceup(d, sc)
}
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let eg = a.eg.clone().nil();
    let sc = group::get_first(d, &eg).nil();
    let val = (card::get_level(d, sc) as i64 * 200) as i32;
    card::create_effect_relation(d, sc, a.e);
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(sc)), 1, a.tp, val);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, val);
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    let sc = group::get_first(d, &eg).nil();
    if card::is_relate_to_effect(d, sc, a.e) && card::is_faceup(d, sc) {
        let prev_atk = card::get_attack(d, sc);
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        let lvl = card::get_level(d, sc) as i64;
        effect::set_value(d, e1, -lvl * 200);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, sc, e1, false);
        duel::adjust_instantly(d, Some(sc));
        let atk_diff = prev_atk as i64 - card::get_attack(d, sc) as i64;
        if atk_diff > 0 {
            duel::damage(d, 1 - a.tp, atk_diff, REASON_EFFECT, false, None);
        }
    }
}
