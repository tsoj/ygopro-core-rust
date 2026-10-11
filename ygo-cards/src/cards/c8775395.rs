//! Evil Eyes Unleashed
use super::prelude::*;

pub const ID: u32 = 8775395;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_EVIL_EYE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send 2 cards to the GY and add 1 "Evil Eye" Equip card to the hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Equip 1 "Evil Eye" card to 1 "Evil Eye" Link Monster
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_EQUIP);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(eqpcond)));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(eqptg)));
    effect::set_operation(d, e2, Some(operation(eqpop)));
    card::register_effect(d, c, e2, false);
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_EVIL_EYE as u16]) && card::is_equip_spell(d, c) && card::is_able_to_hand(d, c, None)
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_EVIL_EYE as u16]) && card::is_able_to_grave_as_cost(d, c)
}
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsMonster,nil)==1
    (group::filter_count(d, sg, &|d, c| card::is_monster(d, c), NO_EX) == 1, false)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let mut g = duel::get_matching_group(d, Some(&tgfilter), tp, LOCATION_DECK, 0, NO_EX);
    let hg = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    if hg.len() == 1 {
        group::remove_card(&mut g, &hg);
    }
    if a.chk == Some(0) {
        return !hg.is_empty()
            && matches!(
                aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    let dg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, false)
        .unwrap_or_default();
    duel::send_to_grave(d, &dg, REASON_COST, None, None);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &g);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Lose 500 LP each time you activate a non-"Evil Eye" card or effect
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
    effect::set_operation(d, e1, Some(operation(damop)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
}
fn damop(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    if !card::is_set_card(d, rc, &[SET_EVIL_EYE as u16]) && a.rp == a.tp {
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        let lp = duel::get_lp(d, a.tp);
        duel::set_lp(d, a.tp, lp - 500);
    }
}
fn lnkfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_link_summoned(d, c) && card::is_set_card(d, c, &[SET_EVIL_EYE as u16]) && card::is_summon_player(d, c, tp)
}
fn eqpcond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| lnkfilter(d, c, tp), 1, NO_EX)
}
fn eqtcfilter(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    card::is_set_card(d, c, &[SET_EVIL_EYE as u16]) && card::is_link_monster(d, c) && card::is_faceup(d, c) && card::check_equip_target(d, ec, c)
}
fn eqfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_EVIL_EYE as u16])
        && card::is_equip_spell(d, c)
        && card::check_unique_on_field(d, c, tp, None, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| eqtcfilter(d, x, c)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn eqptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| eqfilter(d, c, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
fn eqpop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 == 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| eqfilter(d, c, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let Some(ec) = group::get_first(d, &sel) else { return };
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    let sel2 = duel::select_matching_card(d, tp, Some(&|d, c| eqtcfilter(d, c, ec)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    if let Some(tc) = group::get_first(d, &sel2) {
        duel::equip(d, tp, ec, tc, true, false);
    }
}
