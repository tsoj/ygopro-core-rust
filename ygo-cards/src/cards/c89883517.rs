//! Mistaken Accusation
use super::prelude::*;

pub const ID: u32 = 89883517;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let ct1 = duel::get_field_group_count(d, a.tp, LOCATION_ONFIELD | LOCATION_HAND, 0);
    let ct2 = duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD | LOCATION_HAND);
    ct1 < ct2
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && chkc != c;
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, c.into());
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, a.e) || card::is_facedown(d, tc) {
        return;
    }
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    effect::set_target_range(d, e1, 1, 1);
    effect::set_value_fn(d, e1, lua_fn(aclimit));
    let code = card::get_code(d, tc)[0] as i64;
    effect::set_label(d, e1, &[code]);
    duel::register_effect(d, e1, a.tp);
    let e2 = effect::create_effect(d, h);
    let fid = card::get_field_id(d, tc) as i64;
    effect::set_label(d, e2, &[fid]);
    duel::register_effect(d, e2, a.tp);
    effect::set_label_object(d, e1, Value::Effect(e2));
}
fn aclimit(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let re = a.e(1);
    let rc = effect::get_handler(d, re).nil();
    let label = effect::get_label(d, e);
    if !card::is_code(d, rc, &[label as u32]) {
        return false;
    }
    if !card::is_on_field(d, rc) {
        return true;
    }
    let lo = effect::get_label_object(d, e).as_effect().nil();
    let lo_label = effect::get_label(d, lo);
    card::get_field_id(d, rc) as i64 != lo_label
}
