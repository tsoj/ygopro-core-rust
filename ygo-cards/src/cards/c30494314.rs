//! Red-Headed Oni
use super::prelude::*;

pub const ID: u32 = 30494314;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Detach 1 Xyz Material from a monster on the field, increase the level of a monster monster's Level by 1 and its ATK by 300
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_LVCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let rc = effect::get_handler(d, a.re.nil()).nil();
    card::is_previous_location(d, h, LOCATION_GRAVE) && card::is_race(d, rc, RACE_ZOMBIE)
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_ZOMBIE) && !card::is_type(d, c, TYPE_XYZ)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::check_remove_overlay_card(d, a.tp, 1, 1, 1, REASON_EFFECT, None) && duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    if duel::remove_overlay_card(d, a.tp, 1, 1, 1, 1, REASON_EFFECT, None) == 0 {
        return;
    }
    let tc = duel::first_target(d).nil();
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        effect::set_value(d, e1, 1);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, false);
        let e2 = effect::create_effect(d, h);
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e2, 300);
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e2, false);
    }
}
