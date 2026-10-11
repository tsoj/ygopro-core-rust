//! Dragunity Lord - Georgius
use super::prelude::*;

pub const ID: u32 = 70522875;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRAGUNITY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2+ monsters, including a Tuner
    link::add_procedure(d, c, None, 2, Some(4), Some(group_check(|d, g, lc, st, tp| tunermatcheck(d, g, lc, st, tp))), None, None);
    // Your opponent cannot activate the effects of monsters in the GY while this card is equipped with a "Dragunity" card
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, 0, 1);
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        let eg = card::get_equip_group(d, h);
        group::is_exists(d, &eg, &|d, c| card::is_set_card(d, c, &[SET_DRAGUNITY as u16]), 1, NO_EX)
    })));
    effect::set_value_fn(d, e1, lua_fn(|d, a| {
        let re = a.e(1);
        if !effect::is_monster_effect(d, re) {
            return false;
        }
        let h = effect::get_handler(d, re).nil();
        card::is_location(d, h, LOCATION_GRAVE)
    }));
    card::register_effect(d, c, e1, false);
    // Special Summon 1 Level 5 or higher "Dragunity" monster from your Extra Deck or GY
    let e2a = effect::create_effect(d, c);
    effect::set_description(d, e2a, aux::stringid(ID, 0));
    effect::set_category(d, e2a, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2a, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2a, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e2a, 1, ID, 0, 0);
    effect::set_condition(d, e2a, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_link_summoned(d, h)
    })));
    effect::set_target(d, e2a, Some(cost_target(sptg)));
    effect::set_operation(d, e2a, Some(operation(spop)));
    card::register_effect(d, c, e2a, false);
    let e2b = effect::clone(d, e2a);
    effect::set_type(d, e2b, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_range(d, e2b, LOCATION_MZONE);
    effect::set_condition(d, e2b, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        let eg = a.eg.clone().nil();
        !group::is_contains(&eg, h) && group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, 1 - a.tp), 1, NO_EX)
    })));
    card::register_effect(d, c, e2b, false);
    // Negate an opponent's activated Spell/Trap Card or effect
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_DISABLE);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e3, EVENT_CHAINING);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_condition(d, e3, Some(condition(|d, a| {
        a.ep == 1 - a.tp && effect::is_spell_trap_effect(d, a.re.nil()) && duel::is_chain_disablable(d, a.ev as u8)
    })));
    effect::set_cost(d, e3, Some(cost_target(discost)));
    effect::set_target(d, e3, Some(cost_target(|d, a| {
        if a.chk == Some(0) {
            return true;
        }
        duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, a.tp, 0);
        true
    })));
    effect::set_operation(d, e3, Some(operation(|d, a| {
        duel::negate_effect(d, a.ev as u8);
    })));
    card::register_effect(d, c, e3, false);
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}
fn tunermatcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    group::is_exists(d, g, &|d, c| card::is_type_ctx(d, c, TYPE_TUNER, Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mmz_chk: bool) -> bool {
    if !(card::is_level_above(d, c, 5) && card::is_set_card(d, c, &[SET_DRAGUNITY as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)) {
        return false;
    }
    if card::is_location(d, c, LOCATION_EXTRA) {
        duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
    } else {
        mmz_chk
    }
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        let mmz = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, mmz)), tp, LOCATION_EXTRA | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_EXTRA | LOCATION_GRAVE) as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let mmz = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp, mmz))), tp, LOCATION_EXTRA | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if g.len() > 0 {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_equip_card(d, c) && card::is_able_to_grave_as_cost(d, c)), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_equip_card(d, c) && card::is_able_to_grave_as_cost(d, c)), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
