//! Blackwing - Chinook the Snow Blast
use super::prelude::*;

pub const ID: u32 = 34976176;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_BLACK_WINGED_DRAGON],
    listed_series: &[SET_BLACKWING as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send 1 "Blackwing" Synchro Monster or "Black-Winged Dragon" to the GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_ATKCHANGE + CATEGORY_DISABLE);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_MZONE | LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(|d, a| !quickcon(d, a))));
    effect::set_cost(d, e1, Some(cost::self_to_grave()));
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    effect::set_operation(d, e1, Some(operation(tgop)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e2, TIMING_DAMAGE_STEP, Some(TIMING_DAMAGE_STEP | TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    effect::set_condition(d, e2, Some(condition(quickcon)));
    card::register_effect(d, c, e2, false);
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_grave(d, c, None) && ((card::is_set_card(d, c, &[SET_BLACKWING as u16]) && card::is_type(d, c, TYPE_SYNCHRO)) || card::is_code(d, c, &[CARD_BLACK_WINGED_DRAGON]))
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_EXTRA, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
    let tg = duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_EXTRA as i32);
    let n = tg.as_ref().map_or(0, |g| g.len()) as u8;
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, tg, n, 0, 0);
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX);
    let g = match g {
        Some(g) if !g.is_empty() => g,
        _ => return,
    };
    if duel::send_to_grave(d, g.clone(), REASON_EFFECT, None, None) < 1 {
        return;
    }
    if !card::is_location(d, group::get_first(d, &g).nil(), LOCATION_GRAVE) {
        return;
    }
    let tc = duel::first_target(d).nil();
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // Negate effects
        duel::negate_related_chain(d, tc, RESET_TURN_SET);
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_DISABLE);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        effect::set_value(d, e2, RESET_TURN_SET as i64);
        card::register_effect(d, tc, e2, false);
        // Reduce ATK
        let e3 = effect::clone(d, e1);
        effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
        effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_value(d, e3, -700);
        card::register_effect(d, tc, e3, false);
    }
}
fn quickconfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_SYNCHRO) && card::is_attribute(d, c, ATTRIBUTE_DARK)
}
fn quickcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&quickconfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX) && aux::stat_change_damage_step_condition(d)
}
