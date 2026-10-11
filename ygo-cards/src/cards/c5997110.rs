//! D/D Count Surveyor
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 5997110;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DD, SET_DDD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum Summon procedure
    pendulum::add_procedure(d, c, None, None);

    // Tribute 2 monsters your opponent controls and another one gains ATK/DEF equal to the total ATK/DEF the Tributed monsters had on the field
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_RELEASE + CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e1, LOCATION_PZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(tributecon)));
    effect::set_target(d, e1, Some(cost_target(tributetg)));
    effect::set_operation(d, e1, Some(operation(tributeop)));
    card::register_effect(d, c, e1, false);

    // Special Summon this card from your hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_cost(d, e2, Some(cost::discard(Some(Arc::new(|d, c, _e, _tp| card::is_set_card(d, c, &[SET_DD as u16]) && card::is_monster(d, c))), true, None, None, None)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);

    // Add 1 "D/D" monster with 0 ATK or DEF from your Deck to your hand
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_target(d, e3, Some(cost_target(thtg)));
    effect::set_operation(d, e3, Some(operation(thop)));
    card::register_effect(d, c, e3, false);
    let e4 = effect::clone(d, e3);
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e4, false);
}
fn tributeconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_pendulum_summoned(d, c) && card::is_summon_player(d, c, tp) && card::is_set_card(d, c, &[SET_DDD as u16])
}
fn tributecon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| tributeconfilter(d, c, tp), 1, NO_EX)
}
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let by_attr = group::get_class_count(d, sg, &|d, c| card::get_attribute(d, c) as i64) == 1;
    let by_race = group::get_class_count(d, sg, &|d, c| card::get_race(d, c) as i64) == 1;
    let ok = (by_attr || by_race) && group::is_exists(d, sg, &|d, c| card::is_releasable_by_effect(d, c, None, None), 2, NO_EX);
    (ok, false)
}
fn tributetg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chkc().is_some() {
        return false;
    }
    let e = a.e;
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_can_be_effect_target(d, c, Some(e))), tp, 0, LOCATION_MZONE, NO_EX);
    let rs: aux::Rescon = &rescon;
    if a.chk == Some(0) {
        return g.len() >= 3 && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(rs), 0, tp, None, None, None, false), Err(true));
    }
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(rs), 1, tp, Some(HINTMSG_TARGET as u64), None, None, false).unwrap_or_default();
    duel::set_target_card(d, tg.clone());
    duel::set_operation_info(d, 0, CATEGORY_RELEASE, Some(tg.clone()), 2, tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(tg.clone()), 1, tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_DEFCHANGE, Some(tg), 1, tp, 0);
    true
}
fn tributeop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tg0 = duel::get_target_cards(d, a.e).nil();
    let mut tg = group::filter(d, &tg0, &|d, c| card::is_faceup(d, c), NO_EX);
    if tg.len() != 3 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
    let sg = group::filter_select(d, &tg, tp, &|d, c| card::is_releasable_by_effect(d, c, None, None), 2, 2, false, NO_EX).nil();
    if sg.len() != 2 {
        return;
    }
    duel::hint_selection(d, sg.clone(), true);
    group::remove_card(&mut tg, &sg);
    let atk = group::get_sum(d, &sg, &|d, c| card::get_attack(d, c) as i64);
    let def = group::get_sum(d, &sg, &|d, c| card::get_defense(d, c) as i64);
    if duel::release(d, sg, REASON_EFFECT, None) == 2 {
        let c = effect::get_handler(d, a.e).nil();
        let atkdef_c = group::get_first(d, &tg);
        if atk > 0 {
            card::update_attack(d, atkdef_c.nil(), atk, Some(RESET_EVENT | RESETS_STANDARD), Some(c), None);
        }
        if def > 0 {
            card::update_defense(d, atkdef_c.nil(), def, Some(RESET_EVENT | RESETS_STANDARD), Some(c), None);
        }
    }
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_DD as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None) && (card::is_attack(d, c, &[0]) || card::is_defense(d, c, &[0]))
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
