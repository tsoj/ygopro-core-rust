//! Isolde, Two Tales of the Noble Knights
use super::prelude::*;

pub const ID: u32 = 59934749;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_WARRIOR),2,2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_WARRIOR, sc, st, Some(tp)))), 2, Some(2), None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(thcon)));
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    // special summon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, c)
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_race(d, c, RACE_WARRIOR) && card::is_able_to_hand(d, c, None)
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
    let Some(tc) = group::get_first(d, &g) else {
        return;
    };
    if duel::send_to_hand(d, tc, None, REASON_EFFECT, None) != 0 && card::is_location(d, tc, LOCATION_HAND) {
        duel::confirm_cards(d, 1 - a.tp, tc);
        let c = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        effect::set_code(d, e1, EFFECT_CANNOT_SUMMON);
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        effect::set_target_range(d, e1, 1, 0);
        effect::set_target(d, e1, Some(effect_card_fn(sumlimit)));
        let code = card::get_code(d, tc)[0] as i64;
        effect::set_label(d, e1, &[code]);
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        duel::register_effect(d, e1, a.tp);
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
        duel::register_effect(d, e2, a.tp);
        let e3 = effect::clone(d, e1);
        effect::set_code(d, e3, EFFECT_CANNOT_MSET);
        duel::register_effect(d, e3, a.tp);
        let e4 = effect::clone(d, e1);
        effect::set_code(d, e4, EFFECT_CANNOT_ACTIVATE);
        effect::set_value_fn(d, e4, lua_fn(aclimit));
        duel::register_effect(d, e4, a.tp);
    }
}
fn sumlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    card::is_code(d, c, &[effect::get_label(d, e) as u32])
}
fn aclimit(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    let rh = effect::get_handler(d, re).nil();
    card::is_code(d, rh, &[effect::get_label(d, a.e(0)) as u32]) && effect::is_monster_effect(d, re)
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    effect::set_label(d, a.e, &[100]);
    true
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_spell(d, c) && card::is_type(d, c, TYPE_EQUIP) && card::is_able_to_grave_as_cost(d, c)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: i64) -> bool {
    card::is_race(d, c, RACE_WARRIOR)
        && card::is_level_below(d, c, lv as u32)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(0xff))
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let _ = effect::get_handler(d, a.e);
    if a.chk == Some(0) {
        if effect::get_label(d, a.e) != 100 {
            return false;
        }
        effect::set_label(d, a.e, &[0]);
        let cg = duel::get_matching_group(d, Some(&cfilter), a.tp, LOCATION_DECK, 0, NO_EX);
        let lv = group::get_class_count(d, &cg, &|d, c| card::get_code(d, c)[0] as i64) as i64;
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, lv)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    let mut cg = duel::get_matching_group(d, Some(&cfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    let lv = group::get_class_count(d, &cg, &|d, c| card::get_code(d, c)[0] as i64) as i64;
    let tg =duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, lv)), a.tp, LOCATION_DECK, 0, NO_EX);
    // lvt: the distinct levels 1 to 12 of the matching monsters, in ascending order
    let mut lvt: Vec<u64> = Vec::new();
    for tc in group::iter(d, &tg) {
        let tlv = card::get_level(d, tc);
        if (1..=12).contains(&tlv) && !lvt.contains(&(tlv as u64)) {
            lvt.push(tlv as u64);
        }
    }
    lvt.sort();
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
    let lv = duel::announce_number(d, a.tp, &lvt).nil().0;
    let mut rg1 = CardSet::new();
    for _ in 0..lv {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
        let rg2 = group::select(d, &cg, a.tp, 1, 1, false, NO_EX).nil();
        let first = group::get_first(d, &rg2).nil();
        let codes = card::get_code(d, first);
        group::remove(d, &mut cg, &|d, c| card::is_code(d, c, &codes), NO_EX);
        group::add_card(&mut rg1, &rg2);
    }
    duel::send_to_grave(d, rg1, REASON_COST, None, None);
    effect::set_label(d, a.e, &[lv as i64]);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn sfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: i64) -> bool {
    card::is_race(d, c, RACE_WARRIOR)
        && card::is_level(d, c, &[lv as u32])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(0xff))
}
fn spop(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    let lv = effect::get_label(d, a.e);
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| sfilter(d, c, a.e, a.tp, lv)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if let Some(tc) = group::get_first(d, &g) {
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
