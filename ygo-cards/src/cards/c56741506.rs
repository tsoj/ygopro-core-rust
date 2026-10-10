//! Sky Striker Ace - Azalea Temperance
use super::prelude::*;

pub const ID: u32 = 56741506;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SKY_STRIKER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Link Summon procedure
    link::add_procedure(d, c, None, 2, Some(4), Some(group_check(lcheck)), None, None);
    // Must first be Link Summoned
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    effect::set_range(d, e0, LOCATION_EXTRA);
    effect::set_value_fn(d, e0, aux::lnklimit());
    card::register_effect(d, c, e0, false);
    // Equip an opponent's monster with 2500 or less ATK to this card
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(eqcost)));
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    effect::set_operation(d, e1, Some(operation(eqop)));
    card::register_effect(d, c, e1, false);
    aux::add_er_equip_limit(d, c, None, lua_fn(eqlimit), lua_fn(equipop_fn), e1, None, None, None);
    // Special Summon 1 other "Sky Striker" monster from your hand or GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
/// `s.lcheck(g,lc,sumtype,tp)`: `g:IsExists(Card.IsType,1,nil,TYPE_LINK,lc,sumtype,tp)`
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    group::is_exists(d, g, &|d, c| card::is_type_ctx(d, c, TYPE_LINK, Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
fn eqcostfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_spell(d, c) && card::is_able_to_remove_as_cost(d, c, None)
}
fn eqcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&eqcostfilter), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&eqcostfilter), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    duel::remove(d, &g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_attack_below(d, c, 2500) && card::is_faceup(d, c) && card::is_able_to_change_controler(d, c)
}
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&eqfilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    let g = duel::select_target(d, a.tp, Some(&eqfilter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, g, 1, a.tp, 0);
    true
}
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        let c = effect::get_handler(d, a.e).nil();
        card::equip_by_effect_and_limit_register(d, c, a.e, a.tp, tc, None, None);
    }
}
/// `function(ec,c,tp) return ec:IsAttackBelow(2500) and ec:IsFaceup() end`
fn eqlimit(d: &mut Duel, a: Args) -> bool {
    let ec = a.c(0);
    card::is_attack_below(d, ec, 2500) && card::is_faceup(d, ec)
}
/// `Card.EquipByEffectAndLimitRegister(c, e, tp, tc)`
fn equipop_fn(d: &mut Duel, a: Args) {
    let c = a.c(0);
    let e = a.e(1);
    let tp = a.p(2);
    let tc = a.c(3);
    card::equip_by_effect_and_limit_register(d, c, e, tp, tc, None, None);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_SKY_STRIKER as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        let (e, tp) = (a.e, a.tp);
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, c.into());
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    let exc = if card::is_relate_to_effect(d, c, a.e) { c.into() } else { NO_EX };
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let f = aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, tp));
    let g = duel::select_matching_card(d, tp, Some(&f), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, exc).nil();
    if !g.is_empty() {
        duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
