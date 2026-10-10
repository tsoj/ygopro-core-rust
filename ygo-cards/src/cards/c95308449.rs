//! Final Countdown
use super::prelude::*;

pub const ID: u32 = 95308449;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(2000))));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, |d| {
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_TURN_END);
        effect::set_count_limit(d, ge1, 1, 0, 0, 0);
        effect::set_operation(d, ge1, Some(operation(endop)));
        duel::register_effect(d, ge1, 0);
        let ge2 = effect::create_effect(d, c);
        effect::set_type(d, ge2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge2, EVENT_ADJUST);
        effect::set_operation(d, ge2, Some(operation(winop)));
        duel::register_effect(d, ge2, 0);
    });
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e1, ID);
    effect::set_operation(d, e1, Some(operation(checkop)));
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_label(d, e1, &[0]);
    effect::set_value(d, e1, 0);
    effect::set_target_range(d, e1, 1, 1);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 21);
    duel::register_effect(d, e1, a.tp);
    let descnum: u32 = if a.tp == card::get_owner(d, c) { 0 } else { 1 };
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_description(d, e2, aux::stringid(ID, descnum));
    effect::set_property(
        d,
        e2,
        EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_SET_AVAILABLE,
        0,
    );
    effect::set_code(d, e2, 1082946);
    effect::set_label_object(d, e2, Value::Effect(e1));
    effect::set_owner_player(d, e2, a.tp);
    effect::set_operation(d, e2, Some(operation(reset)));
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 21);
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp)` of the reset effect: `s.checkop(e:GetLabelObject(),...,e,...)`
fn reset(d: &mut Duel, a: &EffArgs) {
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    count_turn(d, lo, Some(a.e));
}
fn checkop(d: &mut Duel, a: &EffArgs) {
    count_turn(d, a.e, a.re);
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)` with the values of `e` and `re` that it is given
fn count_turn(d: &mut Duel, e: EffectId, re: Option<EffectId>) {
    let ct = effect::get_value(d, e).int() + 1;
    let h = effect::get_handler(d, e).nil();
    card::set_turn_counter(d, h, ct as u16);
    effect::set_value(d, e, ct);
    if ct == 20 {
        if let Some(re) = re {
            effect::reset(d, re);
        }
    }
}
fn endop(d: &mut Duel, a: &EffArgs) {
    for te in duel::get_player_effect(d, a.tp, ID) {
        // s.checkop(te, te:GetOwnerPlayer(), nil, 0, 0, nil, 0, 0)
        let _owner = effect::get_owner_player(d, te);
        count_turn(d, te, None);
    }
    winop(d, a);
}
fn winop(d: &mut Duel, a: &EffArgs) {
    // local t={} t[0]=0 t[1]=0
    let mut t = [0i64; 2];
    for te in duel::get_player_effect(d, a.tp, ID) {
        let p = effect::get_owner_player(d, te) as usize;
        let ct = effect::get_value(d, te).int();
        if ct == 20 {
            t[p] += 1;
            let label = effect::get_label(d, te) + 1;
            if label == 3 {
                effect::reset(d, te);
            }
        }
    }
    if t[0] > 0 || t[1] > 0 {
        if t[0] == t[1] {
            duel::win(d, PLAYER_NONE as u8, WIN_REASON_FINAL_COUNTDOWN);
        } else if t[0] > t[1] {
            duel::win(d, 0, WIN_REASON_FINAL_COUNTDOWN);
        } else {
            duel::win(d, 1, WIN_REASON_FINAL_COUNTDOWN);
        }
    }
}
