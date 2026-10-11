//! Evil★Twin GG EZ
use super::prelude::*;

pub const ID: u32 = 34365442;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_KI_SIKIL, SET_LIL_LA],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e0, 0, Some(TIMING_END_PHASE));
    card::register_effect(d, c, e0, false);
    // Increase ATK
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_hint_timing(d, e1, TIMING_BATTLE_START, Some(TIMING_BATTLE_START));
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_cost(d, e1, Some(cost_target(atkcost)));
    effect::set_target(d, e1, Some(cost_target(atktg)));
    effect::set_operation(d, e1, Some(operation(atkop)));
    card::register_effect(d, c, e1, false);
    // Negate effect that would destroy
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DISABLE);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e2, Some(condition(negcond)));
    effect::set_cost(d, e2, Some(cost_target(negcost)));
    effect::set_target(d, e2, Some(cost_target(negtg)));
    effect::set_operation(d, e2, Some(operation(negop)));
    card::register_effect(d, c, e2, false);
}
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    (card::is_set_card(d, c, &[SET_KI_SIKIL as u16]) || card::is_set_card(d, c, &[SET_LIL_LA as u16]))
        && card::get_base_attack(d, c) > 0
        && duel::is_existing_target(d, Some(&|d, x| atktgfilter(d, x)), tp, LOCATION_MZONE, 0, 1, c.into())
}
fn atktgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && (card::is_set_card(d, c, &[SET_KI_SIKIL as u16]) || card::is_set_card(d, c, &[SET_LIL_LA as u16]))
}
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    effect::set_label(d, a.e, &[1]);
    true
}
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && atktgfilter(d, chkc);
    }
    if a.chk == Some(0) {
        if effect::get_label(d, a.e) != 1 {
            return false;
        }
        effect::set_label(d, a.e, &[0]);
        return duel::check_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, tp)), 1, 1, false, None, NO_EX);
    }
    let rg = duel::select_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, tp)), 1, 1, false, None, NO_EX);
    duel::release(d, rg.clone(), REASON_COST, None);
    let first = group::get_first(d, &rg).nil();
    let base = card::get_base_attack(d, first);
    effect::set_label(d, a.e, &[base as i64]);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    duel::select_target(d, tp, Some(&atktgfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            let label = effect::get_label(d, a.e);
            effect::set_value(d, e1, label);
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, tc, e1, false);
        }
    }
}
fn negcond(d: &mut Duel, a: &EffArgs) -> bool {
    if !duel::is_chain_disablable(d, a.ev as u8) {
        return false;
    }
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    if card::is_disabled(d, rc) {
        return false;
    }
    if effect::is_has_category(d, re, CATEGORY_NEGATE) {
        let info = duel::get_chain_info(d, (a.ev as i64 - 1) as u8, &[CHAININFO_TRIGGERING_EFFECT]);
        let prev = info.first().and_then(|v| v.as_effect()).nil();
        if effect::is_has_type(d, prev, EFFECT_TYPE_ACTIVATE) {
            return false;
        }
    }
    // local ex,tg,tc=Duel.GetOperationInfo(ev,CATEGORY_DESTROY)
    // return ex and tg~=nil and tc+tg:FilterCount(Card.IsOnField,nil)-#tg>0
    match duel::get_operation_info(d, a.ev, CATEGORY_DESTROY) {
        None => false,
        Some(info) => match info.op_cards {
            None => false,
            Some(tg) => {
                let on_field = group::filter_count(d, &tg, &|d, c| card::is_on_field(d, c), NO_EX) as i64;
                info.op_count as i64 + on_field - tg.len() as i64 > 0
            }
        },
    }
}
fn negcost_filter(d: &mut Duel, c: CardId) -> bool {
    (card::is_set_card(d, c, &[SET_KI_SIKIL as u16]) || card::is_set_card(d, c, &[SET_LIL_LA as u16])) && card::is_monster(d, c)
}
fn negcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&negcost_filter), 1, 1, false, None, NO_EX);
    }
    let g = duel::select_release_group_cost(d, a.tp, Some(&negcost_filter), 1, 1, false, None, NO_EX);
    duel::release(d, g, REASON_COST, None);
    true
}
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let rc = effect::get_handler(d, a.re.nil()).nil();
        return !card::is_status(d, rc, STATUS_DISABLED);
    }
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    true
}
fn negop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    duel::negate_effect(d, a.ev as u8);
}
