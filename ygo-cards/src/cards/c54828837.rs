//! Cerulean Skyfire
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 54828837;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[6007213, 32491822, 69890967], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Procedure enhancement
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, ID);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e2, 1, 0);
    effect::set_range(d, e2, LOCATION_SZONE);
    card::register_effect(d, c, e2, false);
    // Negate effect
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_condition(d, e3, Some(condition(negcon)));
    effect::set_operation(d, e3, Some(operation(negop)));
    card::register_effect(d, c, e3, false);
    // No damage
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_code(d, e4, EVENT_LEAVE_FIELD);
    effect::set_condition(d, e4, Some(condition(ndcon)));
    effect::set_operation(d, e4, Some(operation(ndop)));
    card::register_effect(d, c, e4, false);
    aux::double_snare_validity(d, Some(c), LOCATION_SZONE, None);
}
/// `function(c) return c:IsFaceup() and c:IsCode(32491822) and c:IsAttackPos() end`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[32491822]) && card::is_attack_pos(d, c)
}
/// `function(e,tp,eg,ep,ev,re,r,rp) ... end`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
        && a.rp != a.tp
        && effect::is_spell_trap_effect(d, a.re.nil())
        && duel::is_chain_disablable(d, a.ev as u8)
        && card::get_flag_effect(d, effect::get_handler(d, a.e).nil(), ID) == 0
}
fn negop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    if duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        if duel::negate_effect(d, a.ev as u8) {
            duel::break_effect(d);
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
            let g = duel::select_matching_card(d, tp, Some(&cfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
            let tc = group::get_first(d, &g).nil();
            duel::change_position(d, tc, POS_FACEUP_DEFENSE as u8, None, None, None, false);
        }
    }
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk)` etc.: `s.ndfilter(c,tp)`
fn ndfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    let cd = card::get_previous_code_on_field(d, c).first().copied();
    matches!(cd, Some(6007213 | 32491822 | 69890967))
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
        && card::is_previous_position(d, c, POS_FACEUP as u32)
        && card::get_previous_controler(d, c) == tp
}
fn ndcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| ndfilter(d, c, tp), 1, NO_EX)
}
fn ndop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    aux::register_client_hint(d, h, EFFECT_FLAG_OATH, tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_value(d, e1, 0);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_NO_EFFECT_DAMAGE);
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e2, tp);
}
