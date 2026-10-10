//! Noh-P.U.N.K. Ogre Dance
use super::prelude::*;

pub const ID: u32 = 81914447;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Gain ATK
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(atkcon)));
    effect::set_operation(d, e1, Some(operation(atkop)));
    card::register_effect(d, c, e1, false);
    // Special Summon self
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // Search
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_HAND | LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_cost(d, e3, Some(cost::self_to_grave()));
    effect::set_target(d, e3, Some(cost_target(thtg)));
    effect::set_operation(d, e3, Some(operation(thop)));
    card::register_effect(d, c, e3, false);
}

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_PUNK],
    ..ScriptInfo::DEFAULT
};

fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    if a.rp != 1 - a.tp {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    let h = effect::get_handler(d, re).nil();
    card::get_base_attack(d, h) > 0
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let re_h = effect::get_handler(d, a.re.nil()).nil();
    let atk = card::get_base_attack(d, re_h);
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) && atk > 0 && card::is_controler(d, re_h, 1 - a.tp) {
        // Update ATK
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, atk as i64);
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        card::register_effect(d, c, e1, false);
    }
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&|d, c| card::is_set_card(d, c, &[SET_PUNK as u16])), 1, 1, false, Some(&aux::release_check_mmz), NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RELEASE as u64);
    let g = duel::select_release_group_cost(d, a.tp, Some(&|d, c| card::is_set_card(d, c, &[SET_PUNK as u16])), 1, 1, false, Some(&aux::release_check_mmz), NO_EX);
    duel::release(d, g, REASON_COST, None);
    true
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, LOCATION_HAND as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_PUNK as u16]) && !card::is_level(d, c, &[8]) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, &g);
    }
}
