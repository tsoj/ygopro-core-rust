//! No.52 Diamond Crab King
use super::prelude::*;

pub const ID: u32 = 7194917;
pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(52), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // xyz summon
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    card::enable_revive_limit(d, c);
    // atk/def
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_operation(d, e1, Some(operation(adop)));
    card::register_effect(d, c, e1, false);
    // pos
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_PHASE | PHASE_BATTLE);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(poscon1)));
    effect::set_operation(d, e2, Some(operation(posop1)));
    card::register_effect(d, c, e2, false);
    // pos
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_DAMAGE_STEP_END);
    effect::set_condition(d, e3, Some(condition(poscon2)));
    effect::set_operation(d, e3, Some(operation(posop2)));
    card::register_effect(d, c, e3, false);
}
/// `s.adop(e,tp,eg,ep,ev,re,r,rp)`
fn adop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        effect::set_value(d, e1, 3000);
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        card::register_effect(d, c, e1, false);
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
        effect::set_value(d, e2, 0);
        effect::set_reset(d, e2, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        card::register_effect(d, c, e2, false);
    }
}
/// `s.poscon1(e,tp,eg,ep,ev,re,r,rp)`
fn poscon1(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_attacked_count(d, h) > 0
}
/// `s.posop1(e,tp,eg,ep,ev,re,r,rp)`
fn posop1(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if card::is_attack_pos(d, h) {
        duel::change_position(d, h, POS_FACEUP_DEFENSE as u8, None, None, None, false);
    }
}
/// `s.poscon2(e,tp,eg,ep,ev,re,r,rp)`
fn poscon2(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    Some(h) == duel::get_attack_target(d) && card::get_overlay_count(d, h) == 0
}
/// `s.posop2(e,tp,eg,ep,ev,re,r,rp)`
fn posop2(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_battle(d, c) && card::is_defense_pos(d, c) {
        duel::change_position(d, c, POS_FACEUP_ATTACK as u8, None, None, None, false);
    }
}
