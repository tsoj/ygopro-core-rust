//! Ground Xeno
use super::prelude::*;

pub const ID: u32 = 67523044;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Fusion Summon 1 Dinosaur Fusion Monster
    // local params={aux.FilterBoolFunction(Card.IsRace,RACE_DINOSAUR)}
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    let tg = fusion::summon_eff_tg(d, fus_params());
    effect::set_target(d, e2, Some(tg));
    let op = fusion::summon_eff_op(d, fus_params());
    effect::set_operation(d, e2, Some(op));
    card::register_effect(d, c, e2, false);
}
fn fus_params() -> fusion::SummonEff {
    fusion::SummonEff {
        // aux.FilterBoolFunction(Card.IsRace,RACE_DINOSAUR)
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_race(d, c, RACE_DINOSAUR))),
        ..Default::default()
    }
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_race(d, c, RACE_DINOSAUR) && card::is_type(d, c, TYPE_NORMAL | TYPE_TUNER) && card::is_able_to_hand(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let hg = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !hg.is_empty() && duel::send_to_hand(d, hg.clone(), None, REASON_EFFECT, None) > 0 {
        duel::confirm_cards(d, 1 - a.tp, hg);
        duel::shuffle_hand(d, a.tp);
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        let dg = duel::select_matching_card(d, a.tp, None, a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        if !dg.is_empty() {
            duel::break_effect(d);
            duel::destroy(d, dg, REASON_EFFECT, None, None);
        }
    }
}
