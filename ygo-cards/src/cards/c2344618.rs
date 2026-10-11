//! Lunalight Masquerade
use super::prelude::*;

pub const ID: u32 = 2344618;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LUNALIGHT], listed_names: &[CARD_POLYMERIZATION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    effect::set_cost(d, e0, Some(cost_target(actreg)));
    card::register_effect(d, c, e0, false);
    // Send 1 "Lunalight" monster from your Deck to the GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(tgcon)));
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    effect::set_operation(d, e1, Some(operation(tgop)));
    card::register_effect(d, c, e1, false);
    // Add 1 "Polymerization" from your GY or banishment to your hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_HANDES);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(thcon)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
/// `s.actreg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn actreg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, EFFECT_FLAG_OATH, 1, 0, 0);
    true
}
/// `function(e) return e:GetHandler():HasFlagEffect(id) end`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::has_flag_effect(d, h, ID, None)
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_LUNALIGHT as u16]) && card::is_monster(d, c) && card::is_able_to_grave(d, c, None)
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
/// `s.thconfilter(c,tp)`
fn thconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_LUNALIGHT as u16]) && card::is_fusion_summoned(d, c) && card::is_summon_player(d, c, tp) && card::is_faceup(d, c)
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| thconfilter(d, c, tp), 1, NO_EX)
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[CARD_POLYMERIZATION]) && card::is_faceup(d, c) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 1);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let nf = aux::necro_valley_filter(thfilter);
    let g = duel::select_matching_card(d, tp, Some(&nf), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    if g.is_empty() {
        return;
    }
    duel::hint_selection(d, g.clone(), true);
    if duel::send_to_hand(d, g, None, REASON_EFFECT, None) == 0 {
        return;
    }
    duel::shuffle_hand(d, tp);
    if duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, Some(REASON_EFFECT), None)), tp, LOCATION_HAND, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        duel::break_effect(d);
        if duel::discard_hand(d, tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX) == 0 {
            return;
        }
        let c = effect::get_handler(d, a.e).nil();
        aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 3)), 0, None);
        // Once this turn, if you Fusion Summon a "Lunalight" monster, you can also banish monsters from your GY as material
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        effect::set_code(d, e1, EFFECT_EXTRA_FUSION_MATERIAL);
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        effect::set_target_range(d, e1, LOCATION_GRAVE, 0);
        effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_able_to_remove(d, c, None, None, None) && card::is_monster(d, c))));
        effect::set_operation(d, e1, Some(fusion::banish_material()));
        effect::set_value_fn(
            d,
            e1,
            effect_card_fn(|d, e, c| card::is_set_card(d, c, &[SET_LUNALIGHT as u16]) && card::is_controler(d, c, effect::get_handler_player(d, e))),
        );
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        duel::register_effect(d, e1, tp);
    }
}
