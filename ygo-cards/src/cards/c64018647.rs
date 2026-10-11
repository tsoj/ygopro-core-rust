//! Breath of Resurrection
use super::prelude::*;

pub const ID: u32 = 64018647;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_ADVENTURER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon monsters from hand/GY and equip 1 Equip Spell to a monster you control
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_EQUIP);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(condition(con)));
    effect::set_target(d, e1, Some(cost_target(tg)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[TOKEN_ADVENTURER]))), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn tg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 0, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_EQUIP, None, 0, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
fn eqfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_type(d, c, TYPE_EQUIP)
        && card::check_unique_on_field(d, c, tp, None, None)
        && card::lists_code(d, c, &[TOKEN_ADVENTURER])
        && !card::is_forbidden(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, cc| eqfilter2(d, cc, c)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn eqfilter2(d: &mut Duel, c: CardId, tc: CardId) -> bool {
    card::is_faceup(d, c) && card::check_equip_target(d, tc, c)
}
fn op(d: &mut Duel, a: &EffArgs) {
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    if ft <= 0 {
        return;
    }
    let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp))), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, NO_EX);
    if g.is_empty() {
        return;
    }
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    ft = ft.min(2);
    let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(ft as usize), Some(&aux::dncheck()), 1, a.tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
    if !sg.is_empty() && duel::special_summon(d, sg.clone(), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        for tc in group::iter(d, &sg) {
            // Banish it when it leaves the field
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            effect::set_description(d, e1, 3300);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_CLIENT_HINT, 0);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
            effect::set_value(d, e1, LOCATION_REMOVED as i64);
            card::register_effect(d, tc, e1, true);
        }
        if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&aux::necro_valley_filter(|d, c| eqfilter(d, c, a.tp))), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
        {
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
            let sel = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(|d, c| eqfilter(d, c, a.tp))), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
            let Some(ec) = group::get_first(d, &sel) else { return };
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
            let tg = duel::select_matching_card(d, a.tp, Some(&|d, c| eqfilter2(d, c, ec)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
            if tg.is_empty() {
                return;
            }
            duel::hint_selection(d, tg.clone(), true);
            let ectg = group::get_first(d, &tg).nil();
            duel::break_effect(d);
            duel::equip(d, a.tp, ec, ectg, true, false);
        }
    }
}
