//! Red Dragon Archfiend's Chain
use super::prelude::*;

pub const ID: u32 = 92936364;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_RED_DRAGON_ARCHFIEND], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can activate this card the turn it was Set, by revealing 1 "Red Dragon Archfiend" in your Extra Deck
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e0, EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_code(d, e0, EFFECT_TRAP_ACT_IN_SET_TURN);
    effect::set_value_fn(d, e0, lua_fn(e0_value));
    effect::set_condition(d, e0, Some(lua_fn(e0_cond)));
    card::register_effect(d, c, e0, false);
    // Activate this card by revealing any number of monsters in your hand, then target Effect Monsters your opponent controls equal to the number of Tuners revealed +1; they lose 100 ATK for each card revealed, also their effects are negated
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DISABLE + CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(condition(stat_change_cond)));
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    effect::set_label_object(d, e1, Value::Effect(e0));
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, Some(TIMING_DAMAGE_STEP | TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e1, false);
    // They lose 100 ATK for each card revealed
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e2, Some(aux::persistent_target_filter()));
    effect::set_value_fn(d, e2, lua_fn(move |d: &mut Duel, _a: Args| effect::get_label(d, e1)));
    card::register_effect(d, c, e2, false);
    // Also their effects are negated
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_DISABLE);
    card::register_effect(d, c, e3, false);
}
fn handcostfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && !card::is_public(d, c)
}
fn extracostfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[CARD_RED_DRAGON_ARCHFIEND]) && !card::is_public(d, c)
}
fn faceup_effect_monster(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_effect_monster(d, c)
}
fn e0_value(d: &mut Duel, a: Args) {
    effect::set_label(d, a.e(0), &[1]);
}
fn e0_cond(d: &mut Duel, a: Args) -> bool {
    let p = effect::get_handler_player(d, a.e(0));
    duel::is_existing_matching_card(d, Some(&extracostfilter), p, LOCATION_EXTRA, 0, 1, NO_EX)
}
fn stat_change_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    aux::stat_change_damage_step_condition(d)
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let label_obj = effect::get_label_object(d, a.e).as_effect().nil();
    let hg = duel::get_matching_group(d, Some(&handcostfilter), a.tp, LOCATION_HAND, 0, NO_EX);
    let faceup = aux::faceup_filter(|d, c| card::is_effect_monster(d, c));
    let max_target_count = duel::get_target_count(d, Some(&faceup), a.tp, 0, LOCATION_MZONE, NO_EX);
    let rescon = |_d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>| -> (bool, bool) {
        (group::filter_count(_d, sg, &|d, c| card::is_type(d, c, TYPE_TUNER), NO_EX) + 1 <= max_target_count, false)
    };
    if a.chk == Some(0) {
        effect::set_label(d, label_obj, &[0]);
        return !hg.is_empty()
            && max_target_count > 0
            && matches!(
                aux::select_unselect_group(d, &hg, Some(a.e), a.tp, Some(1), Some(hg.len()), Some(&rescon), 0, a.tp, None, None, None, false),
                Err(true)
            );
    }
    if effect::get_label(d, label_obj) > 0 {
        effect::set_label(d, label_obj, &[0]);
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
        let g = duel::select_matching_card(d, a.tp, Some(&extracostfilter), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        duel::confirm_cards(d, 1 - a.tp, g);
        duel::shuffle_extra(d, a.tp);
    }
    let sg = aux::select_unselect_group(d, &hg, Some(a.e), a.tp, Some(1), Some(hg.len()), Some(&rescon), 1, a.tp, Some(HINTMSG_CONFIRM as u64), Some(&rescon), None, false)
        .unwrap_or_default();
    duel::confirm_cards(d, 1 - a.tp, sg.clone());
    duel::shuffle_hand(d, a.tp);
    effect::set_label(d, a.e, &[sg.len() as i64 * -100]);
    let tuners = group::filter_count(d, &sg, &|d, c| card::is_type(d, c, TYPE_TUNER), NO_EX) as i64 + 1;
    chain::set_effect_data(d, a.e, None, chain::field::CARD, Value::Int(tuners));
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && card::is_effect_monster(d, chkc) && card::is_faceup(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&faceup_effect_monster), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    let ct = chain::effect_data(d, a.e, None, chain::field::CARD).int() as u16;
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_NEGATE as u64);
    let g = duel::select_target(d, a.tp, Some(&faceup_effect_monster), a.tp, 0, LOCATION_MZONE, ct, ct, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(g), 1, a.tp, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let mut tg = duel::get_target_cards(d, a.e).nil();
    group::match_(d, &mut tg, &|d, c| card::is_faceup(d, c), NO_EX);
    if !tg.is_empty() {
        let c = effect::get_handler(d, a.e).nil();
        for tc in group::iter(d, &tg) {
            card::set_card_target(d, c, tc);
        }
        // If the last of those monsters is not on the field, destroy this card
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
        effect::set_code(d, e1, EFFECT_SELF_DESTROY);
        effect::set_range(d, e1, LOCATION_SZONE);
        effect::set_condition(d, e1, Some(lua_fn(no_card_target)));
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, c, e1, false);
    }
}
fn no_card_target(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::get_card_target(d, h).is_empty()
}
