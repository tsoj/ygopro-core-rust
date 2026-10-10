//! Nekroz Kaleidoscope
use super::prelude::*;

pub const ID: u32 = 51124303;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NEKROZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // search
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_SEARCH + CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_condition(d, e2, Some(condition(thcon)));
    effect::set_cost(d, e2, Some(cost_target(thcost)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
    // s.ritual_matching_function[c]=aux.FilterEqualFunction(Card.IsSetCard,SET_NEKROZ)
    // (Lua: `Card.IsSetCard(target)==SET_NEKROZ` compares a boolean with a number: always false)
    crate::card_var::set(d, c, crate::card_var::RITUAL_MATCHING_FUNCTION, Value::Fn(LuaFn::new(|_d, _a| Value::Bool(false))));
}
/// `s.spfilter(c,e,tp,mc)`: a Ritual Monster the material `mc` can Ritual Summon
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId) -> bool {
    card::is_set_card(d, c, &[SET_NEKROZ as u16])
        && card::is_ritual_monster(d, c)
        && {
            let custom = crate::card_info(d, c).ritual_custom_check;
            match custom {
                None => true,
                Some(f) => f(d, e, tp, &CardSet::from_card(mc), c).0,
            }
        }
        && {
            let mf = crate::card_info(d, c).mat_filter;
            match mf {
                None => true,
                Some(f) => f(d, mc, tp),
            }
        }
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_RITUAL, tp, false, true, None, None, None)
        && card::is_can_be_ritual_material(d, mc, Some(c), None)
}
/// `s.rfilter(c,mc)`
fn rfilter(d: &mut Duel, c: CardId, mc: CardId) -> bool {
    let mlv = card::get_ritual_level(d, mc, c);
    if mlv == card::get_level(d, mc) {
        return false;
    }
    let lv = card::get_level(d, c);
    lv == (mlv & 0xffff) || lv == (mlv >> 16)
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    let sg = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, e, tp, c)), tp, LOCATION_HAND, 0, c.into());
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    if card::is_location(d, c, LOCATION_MZONE) {
        ft += 1;
    }
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    let lv = card::get_level(d, c) as i64;
    group::is_exists(d, &sg, &|d, x| rfilter(d, x, c), 1, NO_EX) || group::check_with_sum_equal(d, &sg, &|d, x| card::get_level(d, x) as i64, lv as u32, 1, ft).0
}
/// `s.mfilter(c)`
fn mfilter(d: &mut Duel, c: CardId) -> bool {
    card::has_level(d, c) && card::is_able_to_grave(d, c, None)
}
/// `s.mzfilter(c,tp)`
fn mzfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_location(d, c, LOCATION_MZONE) && card::is_controler(d, c, tp) && card::get_sequence(d, c) < 5
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
        if ft < 0 {
            return false;
        }
        let mut mg = duel::get_ritual_material(d, tp, true);
        if ft > 0 {
            let mg2 = duel::get_matching_group(d, Some(&mfilter), tp, LOCATION_EXTRA, 0, NO_EX);
            group::add_card(&mut mg, &mg2);
        } else {
            group::match_(d, &mut mg, &|d, c| mzfilter(d, c, tp), NO_EX);
        }
        return group::is_exists(d, &mg, &|d, c| filter(d, c, a.e, tp), 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    if ft < 0 {
        return;
    }
    let mut mg = duel::get_ritual_material(d, tp, true);
    if ft > 0 {
        let mg2 = duel::get_matching_group(d, Some(&mfilter), tp, LOCATION_EXTRA, 0, NO_EX);
        group::add_card(&mut mg, &mg2);
    } else {
        group::match_(d, &mut mg, &|d, c| card::is_location(d, c, LOCATION_MZONE), NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
    let mat = group::filter_select(d, &mg, tp, &|d, c| filter(d, c, a.e, tp), 1, 1, false, NO_EX).nil();
    let Some(mc) = group::get_first(d, &mat) else { return };
    let sg = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, a.e, tp, mc)), tp, LOCATION_HAND, 0, mc.into());
    if card::is_location(d, mc, LOCATION_MZONE) {
        ft += 1;
    }
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    let mc_lv = card::get_level(d, mc) as i64;
    let b1 = group::is_exists(d, &sg, &|d, x| rfilter(d, x, mc), 1, NO_EX);
    let b2 = group::check_with_sum_equal(d, &sg, &|d, x| card::get_level(d, x) as i64, mc_lv as u32, 1, ft).0;
    if b1 && (!b2 || duel::select_yes_no(d, tp, aux::stringid(ID, 0))) {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let tg = group::filter_select(d, &sg, tp, &|d, x| rfilter(d, x, mc), 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &tg).nil();
        card::set_material(d, tc, mat.clone());
        if !card::is_location(d, mc, LOCATION_EXTRA) {
            duel::release_ritual_material(d, &mat, false);
        } else {
            duel::send_to_grave(d, mat.clone(), REASON_EFFECT | REASON_MATERIAL | REASON_RITUAL, None, None);
        }
        duel::break_effect(d);
        duel::special_summon(d, tc, SUMMON_TYPE_RITUAL, tp, tp, false, true, POS_FACEUP as u8, None);
        card::complete_procedure(d, tc);
    } else {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let tg = group::select_with_sum_equal(d, &sg, tp, &|d, x| card::get_level(d, x) as i64, mc_lv as u32, 1, ft).nil();
        for tc in group::iter(d, &tg) {
            card::set_material(d, tc, mat.clone());
        }
        if !card::is_location(d, mc, LOCATION_EXTRA) {
            duel::release_ritual_material(d, &mat, false);
        } else {
            duel::send_to_grave(d, mat.clone(), REASON_EFFECT | REASON_MATERIAL | REASON_RITUAL, None, None);
        }
        duel::break_effect(d);
        for tc in group::iter(d, &tg) {
            duel::special_summon_step(d, tc, SUMMON_TYPE_RITUAL, tp, tp, false, true, POS_FACEUP as u8, None);
            card::complete_procedure(d, tc);
        }
        duel::special_summon_complete(d);
    }
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) == 0
}
/// `c:IsSetCard(SET_NEKROZ) and c:IsMonster() and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_NEKROZ as u16]) && card::is_monster(d, c) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None) && duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let mut g = duel::select_matching_card(d, tp, Some(&cfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    group::add_card(&mut g, &CardSet::from_card(c));
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_NEKROZ as u16]) && card::is_spell(d, c) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
