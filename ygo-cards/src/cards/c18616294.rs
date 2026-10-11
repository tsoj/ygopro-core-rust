//! Gray Layer
use super::prelude::*;

pub const ID: u32 = 18616294;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can Tribute this card and reveal 1 Machine monster in your hand; add 1 Machine monster from your Deck to your hand with a different Attribute than the revealed monster, then if you added a monster with 1200 or less ATK, you can Special Summon 1 Machine monster from your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND | CATEGORY_SEARCH | CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(
        d,
        e1,
        Some(cost::and(vec![
            cost::self_tribute(),
            cost::reveal(
                Some(std::sync::Arc::new(|d, c, _e, tp| revealfilter(d, c, tp))),
                false,
                Some(cost::Count::N(1)),
                Some(cost::Count::N(1)),
                Some(std::sync::Arc::new(|d, e, _tp, og| {
                    let tc = group::get_first(d, og).nil();
                    let attr = card::get_attribute(d, tc);
                    effect::set_label(d, e, &[attr as i64]);
                })),
                None,
            ),
        ])),
    );
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    // If a monster(s) in your possession is destroyed by card effect, while this card is in your GY: You can banish this card; destroy 1 card on the field
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD | EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    effect::set_code(d, e2, EVENT_DESTROYED);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(descon)));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
}
fn revealfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    let attr = card::get_attribute(d, c);
    card::is_race(d, c, RACE_MACHINE) && duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, attr)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
fn thfilter(d: &mut Duel, c: CardId, attr: u32) -> bool {
    card::is_race(d, c, RACE_MACHINE) && !card::is_attribute_except(d, c, attr, None, 0, None) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_race(d, c, RACE_MACHINE) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn thop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let attr = effect::get_label(d, a.e) as u32;
    let sc = duel::select_matching_card(d, a.tp, Some(&|d, c| thfilter(d, c, attr)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let Some(sc) = group::get_first(d, &sc) else { return };
    if duel::send_to_hand(d, sc, None, REASON_EFFECT, None) > 0 {
        duel::confirm_cards(d, 1 - a.tp, sc);
        duel::shuffle_hand(d, a.tp);
        if card::is_attack_below(d, sc, 1200)
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, Some(0), None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX)
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
        {
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
            let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
            if !g.is_empty() {
                duel::break_effect(d);
                duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
fn desconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    (card::is_previous_location(d, c, LOCATION_MZONE) || (card::is_monster_card(d, c) && !card::is_previous_location(d, c, LOCATION_ONFIELD)))
        && card::is_reason(d, c, REASON_EFFECT)
        && card::is_previous_controler(d, c, tp)
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let eg = a.eg.clone().nil();
    !group::is_contains(&eg, h) && group::is_exists(d, &eg, &|d, c| desconfilter(d, c, a.tp), 1, NO_EX)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let g = duel::get_field_group(d, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD);
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, a.tp, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let g = duel::select_matching_card(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::hint_selection(d, g.clone(), true);
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
