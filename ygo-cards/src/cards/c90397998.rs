//! The Six Samurai - Kamon
use super::prelude::*;

pub const ID: u32 = 90397998;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SIX_SAMURAI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(descon)));
    effect::set_cost(d, e1, Some(cost_target(descost)));
    effect::set_target(d, e1, Some(cost_target(destg)));
    effect::set_operation(d, e1, Some(operation(desop)));
    card::register_effect(d, c, e1, false);
    // Destroy replace
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target(d, e2, Some(cost_target(desreptg)));
    effect::set_operation(d, e2, Some(operation(desrepop)));
    card::register_effect(d, c, e2, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SIX_SAMURAI as u16]) && card::get_code(d, c)[0] != ID
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::get_attack_announced_count(d, h) == 0;
    }
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    effect::set_property(d, e1, EFFECT_FLAG_OATH, 0);
    card::register_effect(d, h, e1, false);
    true
}
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_spell_trap(d, c)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && desfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&desfilter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, a.tp, Some(&desfilter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d);
    if !duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX) {
        return;
    }
    let tc = tc.nil();
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
fn repfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_SIX_SAMURAI as u16])
        && card::is_destructable(d, c, Some(e))
        && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED + STATUS_BATTLE_DESTROYED)
}
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_REPLACE)
            && card::is_on_field(d, c)
            && card::is_faceup(d, c)
            && duel::is_existing_matching_card(d, Some(&|d, x| repfilter(d, x, a.e)), a.tp, LOCATION_MZONE, 0, 1, c.into());
    }
    if duel::select_effect_yes_no(d, a.tp, c, Some(96)) {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESREPLACE as u64);
        let g = duel::select_matching_card(d, a.tp, Some(&|d, x| repfilter(d, x, a.e)), a.tp, LOCATION_MZONE, 0, 1, 1, false, c.into()).nil();
        let tc = group::get_first(d, &g).nil();
        effect::set_label_object(d, a.e, Value::Card(tc));
        card::set_status(d, tc, STATUS_DESTROY_CONFIRMED, true);
        true
    } else {
        false
    }
}
fn desrepop(d: &mut Duel, a: &EffArgs) {
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    card::set_status(d, tc, STATUS_DESTROY_CONFIRMED, false);
    duel::destroy(d, tc, REASON_EFFECT | REASON_REPLACE, None, None);
}
