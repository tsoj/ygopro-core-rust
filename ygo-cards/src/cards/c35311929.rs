//! Fengli the Soldrapom
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 35311929;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon and halve ATK/DEF
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Replace destruction
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_target(d, e2, Some(cost_target(desreptg)));
    // The Lua script sets s.desrepop, which is never defined, so the operation is nil.
    effect::set_operation(d, e2, None);
    card::register_effect(d, c, e2, false);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_DECK) && (a.r & REASON_EFFECT) != 0 && effect::is_monster_effect(d, a.re.nil())
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, LOCATION_GRAVE as i32);
    true
}
/// `aux.FaceupFilter(Card.IsRace,RACE_PLANT)`
fn plant_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_PLANT)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    if duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) <= 0 {
        return;
    }
    if !duel::is_existing_matching_card(d, Some(&plant_faceup), tp, LOCATION_MZONE, 0, 1, c.into()) {
        return;
    }
    if !duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX) {
        return;
    }
    if !duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, x| card::is_faceup(d, x)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::break_effect(d);
        duel::hint_selection(d, &g, true);
        let tc = group::get_first(d, &g).nil();
        // Halve ATK
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        let atk = card::get_attack(d, tc);
        effect::set_value(d, e1, atk.div_euclid(2) as i64);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, false);
        // Halve DEF
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
        let def = card::get_defense(d, tc);
        effect::set_value(d, e2, def.div_euclid(2) as i64);
        card::register_effect(d, tc, e2, false);
    }
}
/// `s.desrepfilter(c)`
fn desrepfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_race(d, c, RACE_PLANT) && card::is_able_to_grave(d, c, None)
}
/// `s.desreptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_REPLACE)
            && (card::is_reason(d, c, REASON_BATTLE) || card::is_reason(d, c, REASON_EFFECT))
            && duel::is_existing_matching_card(d, Some(&desrepfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    if duel::select_effect_yes_no(d, tp, c, Some(96)) {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        let g = duel::select_matching_card(d, tp, Some(&desrepfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        duel::send_to_grave(d, g, REASON_EFFECT | REASON_REPLACE, None, None);
        return true;
    }
    false
}
