//! Morphing Jar #2
use super::prelude::*;
use std::collections::HashMap;

pub const ID: u32 = 79106360;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TODECK + CATEGORY_TOGRAVE + CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, PLAYER_ALL as u8, LOCATION_MZONE as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, PLAYER_ALL as u8, LOCATION_DECK as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, PLAYER_ALL as u8, LOCATION_DECK as i32);
    true
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    !card::is_status(d, c, STATUS_BATTLE_DESTROYED) && card::is_able_to_deck(d, c)
}
fn filter2(d: &mut Duel, c: CardId) -> bool {
    card::is_location(d, c, LOCATION_DECK) && card::is_monster(d, c)
}
fn op(d: &mut Duel, a: &EffArgs) {
    let mut rg = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    duel::send_to_deck(d, rg, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    rg = duel::get_operated_group(d);
    group::match_(d, &mut rg, &filter2, NO_EX);
    let ct1 = group::filter_count(d, &rg, &|d, c| card::is_controler(d, c, a.tp), NO_EX);
    let ct2 = rg.len() - ct1;
    if ct1 > 0 {
        duel::shuffle_deck(d, a.tp);
    }
    if ct2 > 0 {
        duel::shuffle_deck(d, 1 - a.tp);
    }
    duel::break_effect(d);
    let mut g1 = None;
    let mut g2 = None;
    if ct1 > 0 {
        g1 = sp(d, a.e, a.tp, ct1);
    }
    if ct2 > 0 {
        g2 = sp(d, a.e, 1 - a.tp, ct2);
    }
    duel::special_summon_complete(d);
    if let Some(g1) = g1 {
        duel::shuffle_set_card(d, &g1);
    }
    if let Some(g2) = g2 {
        duel::shuffle_set_card(d, &g2);
    }
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_level_below(d, c, 4) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None)
}
fn sp(d: &mut Duel, e: EffectId, tp: u8, ct: usize) -> Option<CardSet> {
    let deck = duel::get_field_group(d, tp, LOCATION_DECK, 0);
    let dt = deck.len() as i64;
    if dt == 0 {
        return None;
    }
    let mut dlist: HashMap<u32, CardId> = HashMap::new();
    for tc in group::iter(d, &deck) {
        if card::is_monster(d, tc) {
            let seq = card::get_sequence(d, tc);
            dlist.insert(seq, tc);
        }
    }
    let mut i = dt - 1;
    let mut a = 0;
    let mut last: Option<CardId> = None;
    let mut g = CardSet::new();
    while a < ct && i >= 0 {
        if let Some(&tc) = dlist.get(&(i as u32)) {
            group::add_card(&mut g, &CardSet::from_card(tc));
            last = Some(tc);
            a += 1;
        }
        i -= 1;
    }
    let conf = dt - card::get_sequence(d, last.nil()) as i64;
    duel::confirm_decktop(d, tp, conf as u32);
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    group::match_(d, &mut g, &|d, c| spfilter(d, c, e, tp), NO_EX);
    if g.len() as i64 > ft as i64 {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        g = group::select(d, &g, tp, ft as u16, ft as u16, false, NO_EX).nil();
    }
    for tc in group::iter(d, &g) {
        duel::disable_shuffle_check(d, true);
        duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEDOWN_DEFENSE as u8, None);
    }
    if conf - g.len() as i64 > 0 {
        duel::discard_deck(d, tp, (conf - g.len() as i64) as u16, REASON_EFFECT | REASON_EXCAVATE);
    }
    Some(g)
}
