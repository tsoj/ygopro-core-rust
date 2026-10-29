//! Rai-Jin
use super::prelude::*;

pub const ID: u32 = 37829468;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetUniqueOnField(1,1,id)
    card::set_unique_on_field(d, c, 1, 1, card::UniqueCode::Code(ID), None);
    // atkup
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    effect::set_target(d, e1, Some(effect_card_fn(atktg)));
    effect::set_value_fn(d, e1, effect_value(atkval));
    card::register_effect(d, c, e1, false);
    // destroy
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_condition(d, e2, Some(condition(descon)));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
}
/// `s.atktg(e,c)`
fn atktg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_LIGHT)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    let p = effect::get_handler_player(d, e);
    duel::get_matching_group_count(d, Some(&|d, cc| card::is_attribute(d, cc, ATTRIBUTE_LIGHT)), p, LOCATION_GRAVE, 0, NO_EX) as i64 * 100
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_LIGHT)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return true;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    if let Some(tc) = duel::first_target(d) {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
