//! Curse of the Circle
use super::prelude::*;

pub const ID: u32 = 12863633;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // remove
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_REMOVE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MZONE);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    let _ = duel::select_target(d, tp, None, tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, e) {
        let c = effect::get_handler(d, e).nil();
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, c);
        effect::set_property(d, e1, EFFECT_FLAG_ABSOLUTE_TARGET + EFFECT_FLAG_PLAYER_TARGET, 0);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        effect::set_code(d, e1, EFFECT_CANNOT_RELEASE);
        effect::set_range(d, e1, LOCATION_MZONE);
        effect::set_absolute_range(d, e1, tp, 0, 1);
        effect::set_target(d, e1, Some(effect_card_fn(rellimit)));
        card::register_effect(d, tc, e1, false);
        // local e3=Effect.CreateEffect(e:GetHandler())
        let e3 = effect::create_effect(d, c);
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e3, EFFECT_CANNOT_BE_SYNCHRO_MATERIAL);
        effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
        effect::set_label(d, e3, &[(1 - tp) as i64]);
        effect::set_value_fn(d, e3, effect_card_fn(synlimit));
        card::register_effect(d, tc, e3, false);
    }
}
/// `s.rellimit(e,c,tp,sumtp)`
fn rellimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let h = effect::get_handler(d, e).nil();
    c == h
}
/// `s.synlimit(e,c)`
fn synlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let lbl = effect::get_label(d, e);
    card::is_controler(d, c, lbl as u8)
}
