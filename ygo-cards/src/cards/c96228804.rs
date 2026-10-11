//! Regulus, the Prince of Endymion
use super::prelude::*;

pub const ID: u32 = 96228804;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_REGULUS_THE_PRINCE_OF_ENDYMION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // The first time this card would be destroyed by battle each turn, it is not destroyed
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_COUNT);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_value_fn(d, e1, lua_fn(indes_value));
    card::register_effect(d, c, e1, false);
    // Lose LP equal to the revealed Spellcaster monster's Level x 300, and if you do, Special Summon this card
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_cost(
        d,
        e2,
        Some(cost::reveal(
            Some(std::sync::Arc::new(|d, c, _e, _tp| card::is_race(d, c, RACE_SPELLCASTER) && card::has_level(d, c))),
            true,
            Some(cost::Count::N(1)),
            Some(cost::Count::N(1)),
            Some(std::sync::Arc::new(|d, e, _tp, og| {
                let tc = group::get_first(d, og).nil();
                let lv = card::get_level(d, tc);
                effect::set_label(d, e, &[lv as i64]);
            })),
            None,
        )),
    );
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // Add 1 Spell that mentions "Regulus, the Prince of Endymion" from your Deck to your hand
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_target(d, e3, Some(cost_target(thtg)));
    effect::set_operation(d, e3, Some(operation(thop)));
    card::register_effect(d, c, e3, false);
}
fn indes_value(_d: &mut Duel, a: Args) -> bool {
    (a.int(2) & REASON_BATTLE as i64) > 0
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let lp = duel::get_lp(d, a.tp) as i64 - effect::get_label(d, a.e) * 300;
    duel::set_lp(d, a.tp, lp as i32);
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_spell(d, c) && card::lists_code(d, c, &[CARD_REGULUS_THE_PRINCE_OF_ENDYMION]) && card::is_able_to_hand(d, c, None)
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
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
