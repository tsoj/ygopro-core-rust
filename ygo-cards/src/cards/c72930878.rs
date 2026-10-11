//! Black Sonic
use super::prelude::*;

pub const ID: u32 = 72930878;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_BLACKWING],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Can be activated from the hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_TRAP_ACT_IN_HAND);
    effect::set_condition(d, e2, Some(condition(handcon)));
    card::register_effect(d, c, e2, false);
}
fn handcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetFieldGroup(e:GetHandlerPlayer(),LOCATION_MZONE,0)
    let p = effect::get_handler_player(d, a.e);
    let g = duel::get_field_group(d, p, LOCATION_MZONE, 0);
    // return #g==3 and g:IsExists(aux.FaceupFilter(Card.IsSetCard,SET_BLACKWING),3,nil)
    g.len() == 3 && group::is_exists(d, &g, &|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_BLACKWING as u16]), 3, NO_EX)
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=Duel.GetAttackTarget()
    // return tc and tc:IsFaceup() and tc:IsControler(tp) and tc:IsSetCard(SET_BLACKWING)
    match duel::get_attack_target(d) {
        Some(tc) => card::is_faceup(d, tc) && card::is_controler(d, tc, a.tp) && card::is_set_card(d, tc, &[SET_BLACKWING as u16]),
        None => false,
    }
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_position(d, c, POS_FACEUP_ATTACK) && card::is_able_to_remove(d, c, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    let g = duel::get_matching_group(d, Some(&filter), a.tp, 0, LOCATION_MZONE, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, Some(&filter), a.tp, 0, LOCATION_MZONE, NO_EX);
    if !g.is_empty() {
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
