//! Battlin' Boxer Headgeared
use super::prelude::*;

pub const ID: u32 = 79867938;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BATTLIN_BOXER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // send to grave
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    //
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_COUNT);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_value_fn(d, e2, lua_fn(valcon));
    card::register_effect(d, c, e2, false);
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_BATTLIN_BOXER as u16]) && card::is_able_to_grave(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_grave(d, &g, REASON_EFFECT, None, None);
    }
}
fn valcon(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():IsAttackPos() and (r&REASON_BATTLE)~=0
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_attack_pos(d, h) && (a.int(2) as u32 & REASON_BATTLE) != 0
}
