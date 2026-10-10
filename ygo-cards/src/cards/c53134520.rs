//! Phantasmal Summoning Beast
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 53134520;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[6007213, 32491822, 69890967, 89190953], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Search 1 "Uria, Lord of Searing Flames", "Hamon, Lord of Striking Thunder", or "Raviel, Lord of Phantasms"
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost::self_tribute()));
    effect::set_target(d, e1, Some(cost_target(mzthtg)));
    effect::set_operation(d, e1, Some(operation(mzthop)));
    card::register_effect(d, c, e1, false);
    // Search 1 "Dimension Fusion Destruction"
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(gythtg)));
    effect::set_operation(d, e2, Some(operation(gythop)));
    card::register_effect(d, c, e2, false);
}
fn mzthfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[6007213, 32491822, 69890967]) && card::is_able_to_hand(d, c, None)
}
fn mzthtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&mzthfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_level(d, c, &[10])
        && card::is_race(d, c, RACE_PYRO | RACE_THUNDER | RACE_FIEND)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
        && {
            let atk = card::get_attack(d, c) as u32;
            card::is_defense(d, c, &[atk])
        }
        && card::is_attack_above(d, c, 0)
        && card::is_defense_above(d, c, 0)
}
fn mzthop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(mzthfilter)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    let g = g.nil();
    if group::get_count(&g) == 0 || duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) == 0 {
        return;
    }
    duel::confirm_cards(d, 1 - tp, g);
    duel::shuffle_hand(d, tp);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let sg = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        let Some(sc) = group::get_first(d, &sg) else { return };
        duel::break_effect(d);
        if duel::special_summon_step(d, sc, 0, tp, tp, true, false, POS_FACEUP as u8, None) {
            // Cannot attack directly this turn
            let h = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, h);
            effect::set_description(d, e1, 3207);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, sc, e1, false);
        }
        duel::special_summon_complete(d);
    }
}
fn gythfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[89190953]) && card::is_able_to_hand(d, c, None)
}
fn gythtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&gythfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn gythop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&gythfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if group::get_count(&g) > 0 {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, g);
    }
}
