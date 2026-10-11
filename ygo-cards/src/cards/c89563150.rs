//! Shining Silver Force
use super::prelude::*;

pub const ID: u32 = 89563150;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_condition(d, e1, Some(condition(discon)));
    effect::set_target(d, e1, Some(cost_target(distg)));
    effect::set_operation(d, e1, Some(operation(disop)));
    card::register_effect(d, c, e1, false);
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    if a.ep == a.tp
        || !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
        || !effect::is_trap_effect(d, re)
        || !duel::is_chain_negatable(d, a.ev as u8)
    {
        return false;
    }
    if duel::get_operation_info(d, a.ev, CATEGORY_DAMAGE).is_some() {
        return true;
    }
    match duel::get_operation_info(d, a.ev, CATEGORY_RECOVER) {
        None => false,
        Some(info) => {
            let cp = info.op_player;
            if cp != PLAYER_ALL as u8 {
                duel::is_player_affected_by_effect(d, cp, EFFECT_REVERSE_RECOVER).is_some()
            } else {
                duel::is_player_affected_by_effect(d, 0, EFFECT_REVERSE_RECOVER).is_some()
                    || duel::is_player_affected_by_effect(d, 1, EFFECT_REVERSE_RECOVER).is_some()
            }
        }
    }
}
fn dfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_spell_trap(d, c)
}
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    let mut g = duel::get_matching_group(d, Some(&dfilter), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    group::add_card(&mut g, &a.eg.clone().nil());
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
fn disop(d: &mut Duel, a: &EffArgs) {
    if duel::negate_activation(d, a.ev as u8) {
        let mut g = duel::get_matching_group(d, Some(&dfilter), a.tp, 0, LOCATION_ONFIELD, NO_EX);
        group::add_card(&mut g, &a.eg.clone().nil());
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
