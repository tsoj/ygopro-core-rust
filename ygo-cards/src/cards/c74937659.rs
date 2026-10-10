//! Subterror Behemoth Fiendess
use super::prelude::*;

pub const ID: u32 = 74937659;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SUBTERROR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    card::enable_revive_limit(d, c);
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_FLIP, sc, st, Some(tp)))), 2, Some(2), None, None, None);
    // atkup
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_MATERIAL_CHECK);
    effect::set_value_fn(d, e1, effect_card_value(matcheck));
    card::register_effect(d, c, e1, false);
    // to grave
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    effect::set_operation(d, e2, Some(operation(tgop)));
    card::register_effect(d, c, e2, false);
    // search
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e3, EVENT_FLIP);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(thcon)));
    effect::set_target(d, e3, Some(cost_target(thtg)));
    effect::set_operation(d, e3, Some(operation(thop)));
    card::register_effect(d, c, e3, false);
}
fn matfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_SUBTERROR as u16]) && card::get_original_level(d, c) as i64 >= 0
}
/// `s.matcheck(e,c)`: a value function that returns nothing (0)
fn matcheck(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    let mat = card::get_material(d, c);
    let g = group::filter(d, &mat, &matfilter, NO_EX);
    let atk = group::get_sum(d, &g, &|d, x| card::get_original_level(d, x) as i64);
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e1, atk * 100);
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
    card::register_effect(d, c, e1, false);
    0
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPE_FLIP) && card::is_able_to_grave(d, c, None)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, zone: u32) -> bool {
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), Some(tp), Some(zone))
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let zone = card::get_linked_zone(d, c, Some(tp));
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, tp, zone)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() && duel::send_to_grave(d, &g, REASON_EFFECT, None, None) != 0 {
        let first = group::get_first(d, &g).nil();
        if card::is_location(d, first, LOCATION_GRAVE) {
            let zone = card::get_linked_zone(d, effect::get_handler(d, a.e).nil(), Some(tp));
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            let sg = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp, zone)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
            if zone != 0 && !sg.is_empty() {
                duel::special_summon(d, &sg, 0, tp, tp, false, false, POS_FACEDOWN_DEFENSE as u8, Some(zone));
            }
        }
    }
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let lg = card::get_linked_group(d, effect::get_handler(d, a.e).nil());
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|_d, x| group::is_contains(&lg, x), 1, NO_EX)
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPE_FLIP) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let nv = aux::necro_valley_filter(thfilter);
    let g = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
