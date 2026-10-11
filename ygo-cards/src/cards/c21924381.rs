//! Heroic Advance
use super::prelude::*;

pub const ID: u32 = 21924381;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_HEROIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_BE_BATTLE_TARGET);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let dc = duel::get_attack_target(d).nil();
    card::is_faceup(d, dc) && card::is_controler(d, dc, a.tp) && card::is_set_card(d, dc, &[SET_HEROIC as u16])
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_level_below(d, c, 4) && card::is_set_card(d, c, &[SET_HEROIC as u16])
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        let at = duel::get_attack_target(d);
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, at.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    let at = duel::get_attack_target(d);
    duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, at.into());
    let atk_c = duel::get_attacker(d).nil();
    card::create_effect_relation(d, atk_c, a.e);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let atk_c = duel::get_attacker(d).nil();
    let tc = duel::first_target(d).nil();
    if card::is_faceup(d, tc)
        && card::is_relate_to_effect(d, tc, a.e)
        && !card::is_immune_to_effect(d, tc, a.e)
        && card::can_attack(d, atk_c)
        && card::is_relate_to_effect(d, atk_c, a.e)
        && !card::is_immune_to_effect(d, atk_c, a.e)
    {
        let h = effect::get_handler(d, a.e).nil();
        // ATK becomes doubled of its current ATK
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        let atk = card::get_attack(d, tc);
        effect::set_value(d, e1, (atk * 2) as i64);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 1);
        card::register_effect(d, tc, e1, false);
        let e2 = effect::create_effect(d, h);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
        effect::set_value(d, e2, 1);
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
        card::register_effect(d, tc, e2, false);
        let e3 = effect::clone(d, e2);
        card::register_effect(d, atk_c, e3, false);
        duel::calculate_damage(d, atk_c, Some(tc), false);
    }
}
