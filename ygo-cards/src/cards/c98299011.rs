//! Gift of The Mystical Elf
use super::prelude::*;

pub const ID: u32 = 98299011;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_RECOVER);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_DRAW_PHASE));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_MZONE,LOCATION_MZONE)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, tp, LOCATION_MZONE, LOCATION_MZONE) > 0;
    }
    let rec = duel::get_field_group_count(d, tp, LOCATION_MZONE, LOCATION_MZONE) * 300;
    duel::set_target_player(d, tp);
    duel::set_target_param(d, rec as u32);
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, rec);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    let rec = duel::get_field_group_count(d, tp, LOCATION_MZONE, LOCATION_MZONE) * 300;
    duel::recover(d, p, rec as i64, REASON_EFFECT, false, None);
}
