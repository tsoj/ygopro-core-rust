//! Blazing Mirror Force
use super::prelude::*;

pub const ID: u32 = 75249652;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg).nil();
    card::is_controler(d, tc, 1 - a.tp)
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_attack_pos(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    let g = duel::get_matching_group(d, Some(&filter), a.tp, 0, LOCATION_MZONE, NO_EX);
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, Some(&filter), a.tp, 0, LOCATION_MZONE, NO_EX);
    if g.len() > 0 {
        duel::destroy(d, g, REASON_EFFECT, None, None);
        let dg = duel::get_operated_group(d);
        let _ = group::get_first(d, &dg);
        let mut atk: i64 = 0;
        for tc in group::iter(d, &dg) {
            let tatk = card::get_text_attack(d, tc) as i64;
            if tatk > 0 {
                atk += tatk;
            }
        }
        let dam = duel::damage(d, a.tp, lua_int(atk as f64 / 2.0), REASON_EFFECT, false, None) as i64;
        if duel::get_lp(d, a.tp) > 0 && dam > 0 {
            duel::break_effect(d);
            duel::damage(d, 1 - a.tp, dam, REASON_EFFECT, false, None);
        }
    }
}
