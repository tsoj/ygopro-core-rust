//! Dinowrestler Giga Spinosavate
use super::prelude::*;

pub const ID: u32 = 58672736;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    synchro::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_DINOSAUR, sc, st, Some(tp)))), 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    card::enable_revive_limit(d, c);
    // actlimit upon attacking
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, 0, 1);
    effect::set_value_fn(d, e1, lua_fn(aclimit));
    effect::set_condition(d, e1, Some(condition(actcon)));
    card::register_effect(d, c, e1, false);
    // atk limit (opponent)
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    effect::set_value_fn(d, e2, effect_card_fn(atlimit));
    card::register_effect(d, c, e2, false);
    // destroy
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_DESTROY);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_target(d, e3, Some(cost_target(destg)));
    effect::set_operation(d, e3, Some(operation(desop)));
    card::register_effect(d, c, e3, false);
    // destroy replace
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    effect::set_code(d, e4, EFFECT_DESTROY_REPLACE);
    effect::set_target(d, e4, Some(cost_target(reptg)));
    effect::set_operation(d, e4, Some(operation(repop)));
    card::register_effect(d, c, e4, false);
}
/// `s.aclimit(e,re,tp)`: `return re:IsHasType(EFFECT_TYPE_ACTIVATE)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
/// `s.actcon(e)`
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e);
    duel::get_attacker(d) == h || duel::get_attack_target(d) == h
}
/// `s.atlimit(e,c)`: `return c~=e:GetHandler()`
fn atlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    effect::get_handler(d, e) != Some(c)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, a.tp, None, a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
    }
}
fn repfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    card::is_destructable(d, c, Some(e)) && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_REPLACE)
            && !card::is_reason(d, c, REASON_RULE)
            && card::is_on_field(d, c)
            && card::is_faceup(d, c)
            && duel::is_existing_matching_card(d, Some(&|d, x| repfilter(d, x, a.e)), a.tp, LOCATION_ONFIELD, 0, 1, c.into());
    }
    if duel::select_effect_yes_no(d, a.tp, c, Some(96)) {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESREPLACE as u64);
        let g = duel::select_matching_card(d, a.tp, Some(&|d, x| repfilter(d, x, a.e)), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, c.into()).nil();
        duel::set_target_card(d, g.clone());
        let first = group::get_first(d, &g).nil();
        card::set_status(d, first, STATUS_DESTROY_CONFIRMED, true);
        true
    } else {
        false
    }
}
fn repop(d: &mut Duel, _a: &EffArgs) {
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned()).nil();
    let first = group::get_first(d, &g).nil();
    card::set_status(d, first, STATUS_DESTROY_CONFIRMED, false);
    duel::destroy(d, g, REASON_EFFECT | REASON_REPLACE, None, None);
}
