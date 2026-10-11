//! Last Counter
use super::prelude::*;

pub const ID: u32 = 86049351;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BATTLIN_BOXER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId, pos: u32) -> bool {
    card::is_position(d, c, pos) && card::is_set_card(d, c, &[SET_BATTLIN_BOXER as u16])
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let (bc1, bc2) = duel::get_battle_monster(d, a.tp);
    let Some(bc1) = bc1 else {
        return false;
    };
    if !(card::is_faceup(d, bc1) && card::is_set_card(d, bc1, &[SET_BATTLIN_BOXER as u16])) {
        return false;
    }
    let Some(bc2) = bc2 else {
        return false;
    };
    if !card::is_faceup(d, bc2) {
        return false;
    }
    let pos = if duel::get_attacker(d) == Some(bc1) { POS_FACEUP_ATTACK } else { POS_FACEUP };
    effect::set_label_object(d, a.e, Value::Card(bc1));
    duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, pos)), a.tp, LOCATION_MZONE, 0, 1, bc1.into())
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let bc = effect::get_label_object(d, a.e).as_card().nil();
    let bt = card::get_battle_target(d, bc).nil();
    let atk = card::get_base_attack(d, bt);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, a.tp, atk);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    if !(duel::negate_attack(d) && card::is_relate_to_battle(d, tc)) {
        return;
    }
    if card::is_controler(d, tc, tp) && duel::send_to_grave(d, tc, REASON_EFFECT, None, None) > 0 && card::is_location(d, tc, LOCATION_GRAVE) {
        let bc = card::get_battle_target(d, tc).nil();
        if !(card::is_relate_to_battle(d, bc) && card::is_faceup(d, bc) && card::is_controler(d, bc, 1 - tp)) {
            return;
        }
        let pos = if duel::get_attacker(d) == Some(tc) { POS_FACEUP_ATTACK } else { POS_FACEUP };
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, c| cfilter(d, c, pos)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        let Some(sc) = group::get_first(d, &g) else {
            return;
        };
        duel::hint_selection(d, sc, true);
        let atk = card::get_base_attack(d, bc);
        let atk = if atk < 0 { 0 } else { atk };
        // Increase ATK
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, atk as i64);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, sc, e1, false);
        let (bc, sc) = if duel::get_attack_target(d) == Some(bc) { (sc, bc) } else { (bc, sc) };
        if card::is_can_be_battle_target(d, sc, bc) && !card::is_immune_to_effect(d, bc, a.e) && !card::is_immune_to_effect(d, sc, a.e) {
            duel::calculate_damage(d, bc, Some(sc), false);
            duel::break_effect(d);
            duel::damage(d, tp, atk as i64, REASON_EFFECT, false, None);
        }
    }
}
