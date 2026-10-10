//! Evil★Twin's Trouble Sunny
use super::prelude::*;

pub const ID: u32 = 93672138;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_KI_SIKIL, SET_LIL_LA, SET_EVIL_TWIN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // 2+ monsters, including an "Evil★Twin" monster
    link::add_procedure(d, c, None, 2, Some(4), Some(group_check(|d, g, lc, st, tp| lcheck(d, g, lc, st, tp))), None, None);
    // Special Summon up to 1 "Ki-sikil" and "Lil-la" monster each
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    effect::set_cost(d, e1, Some(cost::self_tribute()));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Send 1 card on the field to the GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost_target(tgcost)));
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    effect::set_operation(d, e2, Some(operation(tgop)));
    card::register_effect(d, c, e2, false);
}
/// `s.lcheck(g,lc,sumtype,tp)`
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, st: u64, tp: u8) -> bool {
    group::is_exists(d, g, &|d, c| card::is_set_card_ctx(d, c, &[SET_EVIL_TWIN as u16], Some(lc), st, Some(tp)), 1, NO_EX)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_KI_SIKIL as u16, SET_LIL_LA as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sprescon(sg)`
fn sprescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let res = group::filter_count(d, sg, &|d, c| card::is_set_card(d, c, &[SET_KI_SIKIL as u16]), NO_EX) < 2
        && group::filter_count(d, sg, &|d, c| card::is_set_card(d, c, &[SET_LIL_LA as u16]), NO_EX) < 2;
    (res, false)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        let h = effect::get_handler(d, e).nil();
        return duel::get_m_zone_count(d, tp, h.into(), None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let mut ft = std::cmp::min(2, duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0);
    if ft < 1 {
        return;
    }
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, NO_EX);
    if g.is_empty() {
        return;
    }
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(ft as usize), Some(&sprescon), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
    if !sg.is_empty() {
        duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
fn tgcostfilter(d: &mut Duel, c: CardId, ct: usize) -> bool {
    card::is_set_card(d, c, &[SET_EVIL_TWIN as u16])
        && card::is_able_to_grave_as_cost(d, c)
        && card::is_monster(d, c)
        && (!card::is_location(d, c, LOCATION_MZONE) || (card::is_faceup(d, c) && ct > 1))
}
/// `aux.bfgcost(e,tp,eg,ep,ev,re,r,rp,chk)` called from this card's cost
fn bfgcost(d: &mut Duel, a: &EffArgs, chk: i64) -> bool {
    let args = [
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
        Value::Int(chk),
    ];
    aux::bfgcost().call(d, &args).truthy()
}
fn tgcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let g = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    let n = g.len();
    if a.chk == Some(0) {
        return n > 0
            && bfgcost(d, a, 0)
            && duel::is_existing_matching_card(d, Some(&|d, c| tgcostfilter(d, c, n)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_MZONE, 0, 1, NO_EX);
    }
    bfgcost(d, a, 1);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let cg = duel::select_matching_card(d, tp, Some(&|d, c| tgcostfilter(d, c, n)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    duel::send_to_grave(d, cg, REASON_COST, None, None);
    true
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g), 1, 0, 0);
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::hint_selection(d, g.clone(), true);
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
