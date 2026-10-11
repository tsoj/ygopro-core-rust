//! Absorbing Jar
use super::prelude::*;

pub const ID: u32 = 3900605;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown()
    card::is_facedown(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_SZONE, LOCATION_SZONE, NO_EX);
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_SZONE, LOCATION_SZONE, NO_EX);
    duel::destroy(d, g, REASON_EFFECT, None, None);
    let dg = duel::get_operated_group(d);
    let ct1 = group::filter_count(d, &dg, &|d, c| card::is_controler(d, c, tp), NO_EX);
    let ct2 = dg.len() - ct1;
    duel::break_effect(d);
    if ct1 != 0 {
        duel::draw(d, tp, ct1 as u32, REASON_EFFECT);
    }
    if ct2 != 0 {
        duel::draw(d, 1 - tp, ct2 as u32, REASON_EFFECT);
    }
    // cannot set
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_MSET);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(effect_card_fn(|_d, _e, _c| true)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_CANNOT_SSET);
    duel::register_effect(d, e2, tp);
    let e3 = effect::clone(d, e1);
    effect::set_code(d, e3, EFFECT_CANNOT_TURN_SET);
    duel::register_effect(d, e3, tp);
    let e4 = effect::clone(d, e1);
    effect::set_code(d, e4, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target(d, e4, Some(lua_fn(sumlimit)));
    duel::register_effect(d, e4, tp);
}
fn sumlimit(_d: &mut Duel, a: Args) -> bool {
    // function(e,c,sump,sumtype,sumpos,targetp) return (sumpos&POS_FACEDOWN)~=0 end
    let sumpos = a.int(4) as u32;
    (sumpos & POS_FACEDOWN) != 0
}
