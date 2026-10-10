//! Just Desserts
use super::prelude::*;

pub const ID: u32 = 24068492;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_DRAW_PHASE));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.TRUE,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // local dam=Duel.GetFieldGroupCount(1-tp,LOCATION_MZONE,0)*500
    let dam = duel::get_field_group_count(d, 1 - a.tp, LOCATION_MZONE, 0) * 500;
    // Duel.SetTargetParam(dam)
    duel::set_target_param(d, dam as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dam);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local dam=Duel.GetFieldGroupCount(1-tp,LOCATION_MZONE,0)*500
    let dam = duel::get_field_group_count(d, 1 - a.tp, LOCATION_MZONE, 0) * 500;
    // Duel.Damage(p,dam,REASON_EFFECT)
    duel::damage(d, p, dam as i64, REASON_EFFECT, false, None);
}
