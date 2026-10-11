//! Seventh Force
use super::prelude::*;

pub const ID: u32 = 83293635;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_NUMBER, SET_NUMBER_C, SET_BARIANS, SET_RANK_UP_MAGIC, SET_SEVENTH],
    listed_names: &[ID],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Number C" monster from your Extra Deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_SEARCH + CATEGORY_TOHAND);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_DESTROYED);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && (card::is_previous_location(d, c, LOCATION_MZONE) || (card::is_monster(d, c) && !card::is_previous_location(d, c, LOCATION_SZONE)))
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    duel::is_battle_phase(d, None) && group::is_exists(d, &eg, &|d, c| cfilter(d, c), 1, NO_EX)
}
fn targetfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_NUMBER as u16]) && card::is_type(d, c, TYPE_XYZ)
        && duel::is_existing_matching_card(d, Some(&|d, sc| spfilter(d, sc, e, tp, c)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, xc: CardId) -> bool {
    let xrace = card::get_race(d, xc);
    let xrank = card::get_rank(d, xc);
    card::is_set_card(d, c, &[SET_NUMBER_C as u16])
        && card::is_race(d, c, xrace)
        && card::is_rank(d, c, &[xrank + 1])
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && targetfilter(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| targetfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| targetfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(g), 1, tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    ((card::is_set_card(d, c, &[SET_BARIANS as u16, SET_SEVENTH as u16]) && card::is_spell_trap(d, c))
        || (card::is_set_card(d, c, &[SET_RANK_UP_MAGIC as u16]) && card::is_quick_play_spell(d, c)))
        && card::is_able_to_hand(d, c, None)
        && !card::is_code(d, c, &[ID])
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp, tc)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    if let Some(sc) = group::get_first(d, &sel) {
        if duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
            duel::overlay(d, sc, tc, false);
            let no = crate::card_info(d, sc).xyz_number;
            let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
            let no_ok = matches!(no, Some(n) if (101..=107).contains(&n));
            if no_ok && card::is_set_card(d, sc, &[SET_NUMBER_C as u16]) && !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
                let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
                duel::break_effect(d);
                duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
                duel::confirm_cards(d, 1 - tp, sg.clone());
            }
        }
    }
}
