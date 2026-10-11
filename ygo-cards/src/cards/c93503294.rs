//! Altergeist Primebanshee
use super::prelude::*;

pub const ID: u32 = 93503294;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ALTERGEIST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    card::enable_revive_limit(d, c);
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_ALTERGEIST as u16], sc, st, Some(tp)))),
        2,
        Some(2),
        None,
        None,
        None,
    );
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // to hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(thcon)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}

fn spcon(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
}

fn spcfilter(d: &mut Duel, c: CardId, tp: u8, zone: u32) -> bool {
    card::is_set_card(d, c, &[SET_ALTERGEIST as u16])
        && duel::get_m_zone_count(d, tp, c.into(), Some(tp), Some(LOCATION_REASON_TOFIELD), Some(zone)).0 > 0
}

fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let zone = card::get_linked_zone(d, c, Some(a.tp));
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&|d, x| spcfilter(d, x, a.tp, zone)), 1, 1, false, None, c.into());
    }
    let g = duel::select_release_group_cost(d, a.tp, Some(&|d, x| spcfilter(d, x, a.tp, zone)), 1, 1, false, None, c.into());
    duel::release(d, g, REASON_COST, None);
    true
}

fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_ALTERGEIST as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}

fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}

fn spop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    let zone = card::get_linked_zone(d, h, Some(a.tp));
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, Some(a.tp), Some(LOCATION_REASON_TOFIELD), Some(zone)).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, Some(zone));
    }
}

fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_ONFIELD)
}

fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ALTERGEIST as u16]) && card::is_able_to_hand(d, c, None)
}

fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && thfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let sg = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, sg, 1, 0, 0);
    true
}

fn thop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::get_first_target(d).first().copied().nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
