//! Number 32: Shark Drake
use super::prelude::*;

pub const ID: u32 = 65676461;

pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(32), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    card::enable_revive_limit(d, c);
    // Xyz summon procedure
    xyz::add_procedure(d, c, None, Some(4), 3, None, None, None, None, false, None);
    // Make itself be able to make a second attack
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    effect::set_condition(d, e1, Some(condition(atcon)));
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_target(d, e1, Some(cost_target(attg)));
    effect::set_operation(d, e1, Some(operation(atop)));
    card::register_effect(d, c, e1, false);
}
fn atcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c);
    duel::get_attacker(d) == Some(c)
        && card::is_relate_to_battle(d, c)
        && card::is_status(d, c, STATUS_OPPO_BATTLE)
        && card::is_location(d, bc.nil(), LOCATION_GRAVE)
        && card::is_monster(d, bc.nil())
}
fn attg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        if duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return false;
        }
        let bc = duel::get_attack_target(d).nil();
        return card::is_can_be_special_summoned(d, bc, a.e, 0, tp, false, false, Some(POS_FACEUP_ATTACK as u8), Some(1 - tp), None);
    }
    let bc = duel::get_attack_target(d).nil();
    card::create_effect_relation(d, bc, a.e);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(bc)), 1, 0, 0);
    true
}
fn atop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let bc = duel::get_attack_target(d).nil();
    if !card::is_relate_to_effect(d, bc, a.e) {
        return;
    }
    if duel::special_summon_step(d, bc, 0, a.tp, 1 - a.tp, false, false, POS_FACEUP_ATTACK as u8, None) {
        // Special summoned monster loses 1000 ATK
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, -1000);
        effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
        card::register_effect(d, bc, e1, false);
        duel::special_summon_complete(d);
        if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
            duel::break_effect(d);
            // Can make a second attack
            let e1 = effect::create_effect(d, c);
            effect::set_description(d, e1, 3201);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
            effect::set_value(d, e1, 1);
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, c, e1, false);
        }
    }
}
