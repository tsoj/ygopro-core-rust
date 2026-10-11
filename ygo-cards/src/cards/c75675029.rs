//! Memory Crush King
use super::prelude::*;

pub const ID: u32 = 75675029;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // remove
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE | CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}

fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    a.ep != a.tp && duel::get_attack_target(d).is_none()
}

fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPE_SYNCHRO) && card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, true, false)
}

fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let g = duel::get_matching_group(d, Some(&filter), a.tp, 0, LOCATION_MZONE | LOCATION_GRAVE, NO_EX);
    if !g.is_empty() {
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g.clone()), g.len() as u8, 0, 0);
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, (g.len() * 1000) as i32);
    }
    true
}

fn op(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, Some(&filter), a.tp, 0, LOCATION_MZONE | LOCATION_GRAVE, NO_EX);
    let ct = duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    if ct != 0 {
        duel::damage(d, 1 - a.tp, ct as i64 * 1000, REASON_EFFECT, false, None);
    }
}
