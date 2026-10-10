//! Falling Down
use super::prelude::*;

pub const ID: u32 = 32919136;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARCHFIEND], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,1,aux.CheckStealEquip,s.eqlimit,nil,s.target)
    aux::add_equip_procedure(
        d,
        c,
        Some(1),
        Some(aux::equip_filter_fn(aux::check_steal_equip)),
        Some(effect_card_fn(eqlimit)),
        None,
        Some(std::sync::Arc::new(equip_tg)),
        None,
        None,
        None,
    );
    // damage
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(damcon)));
    effect::set_target(d, e2, Some(cost_target(damtg)));
    effect::set_operation(d, e2, Some(operation(damop)));
    card::register_effect(d, c, e2, false);
    //
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e3, EFFECT_SELF_DESTROY);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_condition(d, e3, Some(condition(descon)));
    card::register_effect(d, c, e3, false);
    // control
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e5, EFFECT_SET_CONTROL);
    effect::set_value_fn(d, e5, effect_card_value(ctval));
    card::register_effect(d, c, e5, false);
}
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    effect::get_handler_player(d, e) != card::get_controler(d, c) || card::get_equip_target(d, effect::get_handler(d, e).nil()) == Some(c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,tc)`: `aux.EquipTarget` calls it without `chk`, after the choice.
fn equip_tg(d: &mut Duel, _a: &EffArgs, tc: Option<CardId>) {
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, tc.map(CardSet::from_card), 1, 0, 0);
}
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, 800);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, a.tp, 800);
    true
}
fn damop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ARCHFIEND as u16])
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    !duel::is_existing_matching_card(d, Some(&desfilter), p, LOCATION_ONFIELD, 0, 1, NO_EX)
}
fn ctval(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    effect::get_handler_player(d, e) as i64
}
