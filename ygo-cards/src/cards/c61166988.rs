//! Wild Nature's Release
use super::prelude::*;

pub const ID: u32 = 61166988;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:HasNonZeroDefense() and c:IsRace(RACE_BEAST|RACE_BEASTWARRIOR)
    card::has_non_zero_defense(d, c) && card::is_race(d, c, RACE_BEAST | RACE_BEASTWARRIOR)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if tc:IsFaceup() then
    let tc = duel::first_target(d).nil();
    if card::is_faceup(d, tc) {
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        let def = card::get_defense(d, tc) as i64;
        effect::set_value(d, e1, def);
        card::register_effect(d, tc, e1, false);
        let h = effect::get_handler(d, a.e).nil();
        let e2 = effect::create_effect(d, h);
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
        effect::set_range(d, e2, LOCATION_MZONE);
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        effect::set_count_limit(d, e2, 1, 0, 0, 0);
        effect::set_operation(d, e2, Some(operation(desop)));
        card::register_effect(d, tc, e2, false);
    }
}
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::destroy(d, h, REASON_EFFECT, None, None);
}
