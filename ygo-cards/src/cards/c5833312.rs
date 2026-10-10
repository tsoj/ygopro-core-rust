//! Duel Academy
use super::prelude::*;

pub const ID: u32 = 5833312;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // Destroy 1 card your opponent controls if you activate a Trap Card
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
    effect::set_range(d, e1, LOCATION_FZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(descon)));
    effect::set_target(d, e1, Some(cost_target(destg)));
    effect::set_operation(d, e1, Some(operation(desop)));
    card::register_effect(d, c, e1, false);
    // Inflict 1000 damage to your opponent if you activate a Spell
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(damcon)));
    effect::set_target(d, e2, Some(cost_target(damtg)));
    effect::set_operation(d, e2, Some(operation(damop)));
    card::register_effect(d, c, e2, false);
    // 1 monster you control gains 1000 ATK if you activate a monster effect
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e3, EVENT_CHAIN_SOLVED);
    effect::set_range(d, e3, LOCATION_FZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(atkcon)));
    effect::set_target(d, e3, Some(cost_target(atktg)));
    effect::set_operation(d, e3, Some(operation(atkop)));
    card::register_effect(d, c, e3, false);
}
/// `s.typecheck(types)`
fn typecheck(d: &mut Duel, types: u64) -> bool {
    duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, types)),
        0,
        LOCATION_MZONE,
        LOCATION_MZONE,
        1,
        NO_EX,
    )
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    let c = effect::get_handler(d, a.e).nil();
    typecheck(d, RACE_WARRIOR | RACE_BEAST | RACE_PYRO)
        && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE as u32)
        && effect::is_trap_effect(d, re)
        && a.rp == a.tp
        && !card::is_disabled(d, c)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, effect::get_description(d, e));
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, tp, None, tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
    }
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let re = a.re.nil();
    typecheck(d, RACE_DINOSAUR | RACE_SEASERPENT | RACE_THUNDER)
        && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE as u32)
        && effect::is_spell_effect(d, re)
        && a.rp == a.tp
        && effect::get_handler(d, re) != Some(c)
        && !card::is_disabled(d, c)
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, effect::get_description(d, a.e));
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 1000);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    duel::damage(d, 1 - a.tp, 1000, REASON_EFFECT, false, None);
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let re = a.re.nil();
    typecheck(d, RACE_MACHINE | RACE_FAIRY | RACE_FIEND)
        && effect::is_monster_effect(d, re)
        && a.rp == a.tp
        && !card::is_disabled(d, c)
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && card::is_faceup(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, effect::get_description(d, e));
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    let _ = duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let c = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    let tc = duel::first_target(d).nil();
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, e) {
        // Increase ATK
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, 1000);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, false);
    }
}
