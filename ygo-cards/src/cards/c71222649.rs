//! Twilight Specter Rayrage
use super::prelude::*;

pub const ID: u32 = 71222649;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    pendulum::add_procedure(d, c, None, None);
    // If a Pendulum Monster(s) you control would be destroyed by an opponent's card effect, you can destroy this card instead of 1 of those Pendulum Monsters.
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EFFECT_DESTROY_REPLACE);
    effect::set_range(d, e1, LOCATION_PZONE);
    effect::set_target(d, e1, Some(cost_target(reptg)));
    effect::set_operation(d, e1, Some(operation(repop)));
    effect::set_value_fn(d, e1, effect_card_fn(|d, e, c| {
        let g = effect::get_label_object(d, e).as_group().cloned().nil();
        group::get_first(d, &g) == Some(c)
    }));
    card::register_effect(d, c, e1, false);
    // If this card is in your hand: You can target 1 Pendulum Monster you control; place that Pendulum Monster in your Pendulum Zone, and if you do, Special Summon this card
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // (Quick Effect): You can Tribute this card, then target 1 card in your Pendulum Zone; it cannot be destroyed by your opponent's card effects this turn
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_cost(d, e3, Some(cost::self_tribute()));
    effect::set_target(d, e3, Some(cost_target(efftg)));
    effect::set_operation(d, e3, Some(operation(effop)));
    card::register_effect(d, c, e3, false);
}
fn repfilter(d: &mut Duel, c: CardId, tp: u8, opp: u8) -> bool {
    card::is_pendulum_monster(d, c)
        && card::is_controler(d, c, tp)
        && card::is_on_field(d, c)
        && card::is_reason_player(d, c, opp)
        && card::is_faceup(d, c)
        && card::is_reason(d, c, REASON_EFFECT)
        && !card::is_reason(d, c, REASON_REPLACE)
}
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let opp = 1 - tp;
    let c = effect::get_handler(d, e).nil();
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return !duel::has_flag_effect(d, tp, ID, None)
            && card::is_destructable(d, c, Some(e))
            && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED)
            && group::is_exists(d, &eg, &|d, x| repfilter(d, x, tp, opp), 1, NO_EX);
    }
    if duel::select_effect_yes_no(d, tp, c, Some(96)) {
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        let eg = a.eg.clone().nil();
        let mut g = group::filter(d, &eg, &|d, x| repfilter(d, x, tp, opp), NO_EX);
        if g.len() > 1 {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESREPLACE as u64);
            g = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            duel::hint_selection(d, g.clone(), true);
        }
        effect::set_label_object(d, e, Value::Group(g));
        card::set_status(d, c, STATUS_DESTROY_CONFIRMED, true);
        duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        true
    } else {
        false
    }
}
fn repop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    card::set_status(d, c, STATUS_DESTROY_CONFIRMED, false);
    duel::destroy(d, c, REASON_EFFECT | REASON_REPLACE, None, None);
}
fn plfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_pendulum_monster(d, c) && card::is_faceup(d, c) && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && plfilter(d, chkc, tp);
    }
    let c = effect::get_handler(d, e).nil();
    if a.chk == Some(0) {
        return duel::check_pendulum_zones(d, tp)
            && duel::is_existing_target(d, Some(&|d, x| plfilter(d, x, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    duel::select_target(d, tp, Some(&|d, x| plfilter(d, x, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, e)
        && card::is_faceup(d, tc)
        && !card::is_immune_to_effect(d, tc, e)
        && duel::move_to_field(d, tc, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None)
    {
        let c = effect::get_handler(d, e).nil();
        // It cannot activate its Pendulum Effects this turn
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, aux::stringid(ID, 2));
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
        if card::is_relate_to_effect(d, c, e) {
            duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_PZONE);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, LOCATION_PZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_APPLYTO as u64);
    duel::select_target(d, tp, None, tp, LOCATION_PZONE, 0, 1, 1, false, NO_EX);
    true
}
fn effop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        // It cannot be destroyed by your opponent's card effects this turn
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_description(d, e1, 3060);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
        effect::set_value_fn(d, e1, aux::indoval());
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
    }
}
