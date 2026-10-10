//! Draw Discharge
use super::prelude::*;

pub const ID: u32 = 26557451;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DAMAGE + CATEGORY_REMOVE + CATEGORY_HANDES);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_DRAW);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(_d: &mut Duel, a: &EffArgs) -> bool {
    a.ep != a.tp && (a.r & REASON_EFFECT) != 0 && a.rp == a.ep
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, a.tp, 30459350).is_none();
    }
    duel::set_target_card(d, a.eg());
    if let Some(eg) = a.eg.clone() {
        if group::is_exists(d, &eg, &card::is_monster, 1, NO_EX) {
            let tc = group::get_first(d, &eg).nil();
            let atk = card::get_attack(d, tc);
            duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, atk);
            duel::set_operation_info(d, 0, CATEGORY_REMOVE + CATEGORY_HANDES, None, 0, 1 - a.tp, 1);
        }
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if let Some(g) = duel::get_target_cards(d, a.e) {
        if !g.is_empty() {
            duel::confirm_cards(d, tp, &g);
            if group::is_exists(d, &g, &card::is_monster, 1, NO_EX) {
                duel::break_effect(d);
                let mut atk: i64 = 0;
                let _ = group::get_first(d, &g);
                for tc in group::iter(d, &g) {
                    let tatk = card::get_attack(d, tc);
                    if tatk > 0 {
                        atk += tatk as i64;
                    }
                }
                if duel::damage(d, 1 - tp, atk, REASON_EFFECT, false, None) != 0 {
                    duel::remove(d, &g, POS_FACEUP as u8, REASON_EFFECT, None, None);
                }
            }
            duel::shuffle_hand(d, 1 - tp);
        }
    }
}
