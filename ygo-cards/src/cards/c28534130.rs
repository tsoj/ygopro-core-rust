//! Salamangreat Burning Shell
use super::prelude::*;

pub const ID: u32 = 28534130;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SALAMANGREAT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Salamangreat" monster from hand, then Link Summon 1 "Salamangreat" Link Monster using materials you control, including the Summoned monster
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Return 1 "Salamangreat" Link Monster from your GY to the Extra Deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TODECK);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    effect::set_operation(d, e2, Some(operation(tdop)));
    card::register_effect(d, c, e2, false);
}
fn fg_group(d: &mut Duel, tp: u8) -> CardSet {
    duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_can_be_link_material(d, c, None, None)), tp, LOCATION_MZONE, 0, NO_EX)
}
fn spfilter2(d: &mut Duel, c: CardId, mc: CardId, fg: &CardSet) -> bool {
    let mut m = fg.clone();
    group::add_card(&mut m, &CardSet::from_card(mc));
    card::is_set_card(d, c, &[SET_SALAMANGREAT as u16]) && card::is_link_summonable(d, c, Some(CardSet::from_card(mc)), Some(m), 0, 0)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, fg: &CardSet) -> bool {
    card::is_set_card(d, c, &[SET_SALAMANGREAT as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::is_existing_matching_card(d, Some(&|d, cc| spfilter2(d, cc, c, fg)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let fg = fg_group(d, tp);
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp, &fg)), tp, LOCATION_HAND, 0, 1, NO_EX)
            && duel::is_player_can_special_summon_count(d, tp, 2);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, (LOCATION_HAND | LOCATION_EXTRA) as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    let fg = fg_group(d, tp);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp, &fg)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let Some(tc) = group::get_first(d, &g) else {
        return;
    };
    if !duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_DISABLE);
    effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
    card::register_effect(d, tc, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
    card::register_effect(d, tc, e2, false);
    duel::special_summon_complete(d);
    let tg = duel::get_matching_group(d, Some(&|d, cc| spfilter2(d, cc, tc, &fg)), tp, LOCATION_EXTRA, 0, NO_EX);
    if !tg.is_empty() {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let sg = group::select(d, &tg, tp, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sg).nil();
        let e3 = effect::create_effect(d, c);
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
        effect::set_reset(d, e3, RESET_EVENT + (RESETS_STANDARD & !RESET_TOFIELD), 1);
        effect::set_operation(d, e3, Some(operation(regop)));
        card::register_effect(d, sc, e3, false);
        duel::link_summon(d, tp, sc, Some(CardSet::from_card(tc)), None, 0, 0);
    }
}
fn regop(d: &mut Duel, a: &EffArgs) {
    let rc = effect::get_owner(d, a.e).nil();
    let c = effect::get_handler(d, a.e).nil();
    // Cannot attack
    let e1 = effect::create_effect(d, rc);
    effect::set_description(d, e1, 3206);
    effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    card::register_effect(d, c, e1, false);
    // Cannot activate its effects
    let e2 = effect::clone(d, e1);
    effect::set_description(d, e2, 3302);
    effect::set_code(d, e2, EFFECT_CANNOT_TRIGGER);
    card::register_effect(d, c, e2, false);
    effect::reset(d, a.e);
}
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_SALAMANGREAT as u16]) && card::is_type(d, c, TYPE_LINK) && card::is_able_to_extra(d, c)
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && tdfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tdfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    let g = duel::select_target(d, a.tp, Some(&tdfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, a.tp, 0);
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
