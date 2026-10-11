//! Radiance of the Voiceless Voice
use super::prelude::*;

pub const ID: u32 = 86310763;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VOICELESS_VOICE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // Activate 1 of these effects
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(lua_fn(main_cond)));
    effect::set_target(d, e1, Some(cost_target(efftg)));
    effect::set_operation(d, e1, Some(operation(effop)));
    card::register_effect(d, c, e1, false);
}
fn main_cond(d: &mut Duel, _a: Args) -> bool {
    duel::is_main_phase(d, None)
}
fn todeckfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_deck(d, c)
        && (card::is_ritual_spell(d, c)
            || (card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_WARRIOR | RACE_DRAGON) && card::is_ritual_monster(d, c)))
}
fn silenforcfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    card::is_set_card(d, c, &[SET_VOICELESS_VOICE as u16])
        && card::is_monster(d, c)
        && (card::is_able_to_hand(d, c, None) || (ft > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_WARRIOR | RACE_DRAGON) && card::is_ritual_monster(d, c)
}
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - a.tp);
    }
    let (e, tp) = (a.e, a.tp);
    let ft0 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    let b1 = duel::is_existing_matching_card(d, Some(&todeckfilter), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| silenforcfilter(d, c, e, tp, ft0)), tp, LOCATION_DECK, 0, 1, NO_EX);
    let c = effect::get_handler(d, e).nil();
    let ct = duel::get_matching_group_count(d, Some(&cfilter), tp, LOCATION_MZONE, 0, NO_EX);
    let b2 = ct > 0 && duel::is_existing_target(d, None, tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    if a.chk == Some(0) {
        return b1 || b2;
    }
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).unwrap_or(0) as i64;
    effect::set_label(d, e, &[op]);
    if op == 1 {
        effect::set_category(d, e, CATEGORY_TODECK + CATEGORY_SEARCH + CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
        effect::set_property(d, e, 0, 0);
        duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
        duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
        duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    } else if op == 2 {
        effect::set_category(d, e, CATEGORY_DESTROY);
        effect::set_property(d, e, EFFECT_FLAG_CARD_TARGET, 0);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        let mut g = duel::select_target(d, tp, None, tp, 0, LOCATION_ONFIELD, 1, ct as u16, false, NO_EX).nil();
        group::add_card(&mut g, &CardSet::from_card(c));
        let n = g.len() as u8;
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, tp, 0);
    }
    true
}
fn effop(d: &mut Duel, a: &EffArgs) {
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // Shuffle 1 LIGHT Warrior or Dragon Ritual Monster or 1 Ritual Spell from your hand or GY into the Deck
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
        let g = duel::select_matching_card(
            d,
            a.tp,
            Some(&aux::necro_valley_filter(|d, c| todeckfilter(d, c))),
            a.tp,
            LOCATION_HAND | LOCATION_GRAVE,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        let Some(tdc) = group::get_first(d, &g) else { return };
        if card::is_location(d, tdc, LOCATION_GRAVE) {
            duel::hint_selection(d, tdc, true);
        } else {
            duel::confirm_cards(d, 1 - a.tp, tdc);
        }
        if duel::send_to_deck(d, tdc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0 && card::is_location(d, tdc, LOCATION_DECK) {
            let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 3) as u64);
            let sg = duel::select_matching_card(d, a.tp, Some(&|d, c| silenforcfilter(d, c, a.e, a.tp, ft)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            if let Some(sc) = group::get_first(d, &sg) {
                let e = a.e;
                let tp = a.tp;
                aux::to_hand_or_else(
                    d,
                    sc,
                    a.tp,
                    Some(&|d: &mut Duel, _c: CardId| ft > 0 && card::is_can_be_special_summoned(d, sc, e, 0, tp, false, false, None, None, None)),
                    Some(&|d: &mut Duel, _g: &CardSet| {
                        duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
                        0
                    }),
                    Some(aux::stringid(ID, 4)),
                );
            }
        }
    } else if op == 2 {
        // Destroy both the targets and this card
        let c = effect::get_handler(d, a.e).nil();
        if !card::is_relate_to_effect(d, c, a.e) {
            return;
        }
        let mut g = duel::get_target_cards(d, a.e).nil();
        if g.is_empty() {
            return;
        }
        group::add_card(&mut g, &CardSet::from_card(c));
        if !g.is_empty() {
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    }
}
