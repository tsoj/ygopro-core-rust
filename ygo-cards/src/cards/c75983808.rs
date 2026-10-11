//! Chaos Mystic Box
use super::prelude::*;

pub const ID: u32 = 75983808;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_LIGHT_AND_DARKNESS_RITUAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When your opponent activates a card or effect that targets another card(s) you control: ...
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_condition(d, e1, Some(condition(thdescon)));
    effect::set_target(d, e1, Some(cost_target(thdestg)));
    effect::set_operation(d, e1, Some(operation(thdesop)));
    card::register_effect(d, c, e1, false);
    // If this card in its owner's possession is destroyed by an opponent's card: ...
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_DESTROYED);
    effect::set_condition(d, e2, Some(condition(e2cond)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
fn thdesconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_controler(d, c, tp) && card::is_on_field(d, c) && card::is_able_to_hand(d, c, None)
}
fn thdescon(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    if !(a.rp != a.tp && effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0)) {
        return false;
    }
    let c = effect::get_handler(d, a.e).nil();
    let tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned();
    match tg {
        Some(tg) => group::is_exists(d, &tg, &|d, x| thdesconfilter(d, x, a.tp), 1, c.into()),
        None => false,
    }
}
fn thdestg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, tp, LOCATION_ONFIELD, 0, 1, c.into());
    }
    let g = duel::get_field_group(d, tp, LOCATION_ONFIELD, LOCATION_ONFIELD);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_ONFIELD as i32);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
fn thconfilter(d: &mut Duel, c: CardId, re: EffectId, tp: u8) -> bool {
    card::is_relate_to_effect(d, c, re) && card::is_controler(d, c, tp)
}
fn handspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    card::is_monster(d, c) && card::lists_code(d, c, &[CARD_LIGHT_AND_DARKNESS_RITUAL]) && !card::is_code(d, c, &[code]) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
fn thdesop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let re = a.re.nil();
    let chain_tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    let tg = group::filter(d, &chain_tg, &|d, x| thconfilter(d, x, re, tp), NO_EX);
    if tg.is_empty() {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    let sel = group::filter_select(d, &tg, tp, &|d, x| card::is_able_to_hand(d, x, None), 1, 1, false, NO_EX).nil();
    let Some(sc) = group::get_first(d, &sel) else { return };
    if card::is_faceup(d, sc) {
        duel::hint_selection(d, sc, true);
    } else {
        duel::confirm_cards(d, 1 - tp, sc);
    }
    if duel::send_to_hand(d, sc, None, REASON_EFFECT, None) > 0 && card::is_location(d, sc, LOCATION_HAND) {
        let sc_ctrl = card::get_controler(d, sc);
        duel::shuffle_hand(d, sc_ctrl);
        let c = effect::get_handler(d, a.e).nil();
        let exc = if card::is_relate_to_effect(d, c, a.e) { c.into() } else { NO_EX };
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        let g = duel::select_matching_card(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, exc).nil();
        if g.is_empty() {
            return;
        }
        duel::hint_selection(d, &g, true);
        let code = card::get_code(d, sc)[0];
        if duel::destroy(d, &g, REASON_EFFECT, None, None) > 0
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| handspfilter(d, x, a.e, tp, code)), tp, LOCATION_HAND, 0, 1, NO_EX)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
        {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            let sg = duel::select_matching_card(d, tp, Some(&|d, x| handspfilter(d, x, a.e, tp, code)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
            if !sg.is_empty() {
                duel::break_effect(d);
                duel::special_summon(d, &sg, 0, tp, tp, true, false, POS_FACEUP as u8, None);
            }
        }
    }
}
fn ritualspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_ritual_monster(d, c) && card::lists_code(d, c, &[CARD_LIGHT_AND_DARKNESS_RITUAL]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, true, None, None, None)
}
fn e2cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    a.rp != a.tp && card::is_previous_controler(d, c, a.tp)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| ritualspfilter(d, c, a.e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| ritualspfilter(d, c, a.e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, &g, 0, tp, tp, false, true, POS_FACEUP as u8, None);
    }
}
