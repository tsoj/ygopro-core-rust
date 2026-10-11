//! Altergeist Haunted Rock
use super::prelude::*;

pub const ID: u32 = 2547033;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ALTERGEIST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Negate the effect of an opponent's Trap card
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DISABLE + CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(discon)));
    effect::set_cost(d, e2, Some(cost_target(discost)));
    effect::set_target(d, e2, Some(cost_target(distg)));
    effect::set_operation(d, e2, Some(operation(disop)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EFFECT_TRAP_ACT_IN_SET_TURN);
    effect::set_property(d, e3, EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_condition(d, e3, Some(lua_fn(e3cond)));
    card::register_effect(d, c, e3, false);
    aux::double_snare_validity(d, Some(c), LOCATION_SZONE, None);
    aux::global_check(d, ID, move |d| {
        // activatable if set by Altergeist
        let ge = effect::create_effect(d, c);
        effect::set_type(d, ge, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge, EVENT_SSET);
        effect::set_operation(d, ge, Some(operation(global_op)));
        duel::register_effect(d, ge, 0);
    });
}

fn e3cond(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::has_flag_effect(d, h, ID, None)
}

fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ALTERGEIST as u16]) && card::is_monster(d, c) && card::is_able_to_grave(d, c, None)
}

fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_HAND as i32);
    true
}

fn activate(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&tgfilter), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}

fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    a.ep == 1 - a.tp
        && effect::is_trap_effect(d, re)
        && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
        && duel::is_chain_negatable(d, a.ev as u8)
}

fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ALTERGEIST as u16]) && card::is_monster(d, c) && card::is_able_to_grave_as_cost(d, c)
}

fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&cfilter), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}

fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_destructable(d, rh, None) && card::is_relate_to_effect(d, rh, re) {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}

fn disop(d: &mut Duel, a: &EffArgs) {
    if duel::negate_effect(d, a.ev as u8) {
        let re = a.re.nil();
        let rh = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rh, re) {
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}

fn global_op(d: &mut Duel, a: &EffArgs) {
    let Some(re) = a.re else { return };
    let rh = effect::get_handler(d, re).nil();
    if !card::is_set_card(d, rh, &[SET_ALTERGEIST as u16]) {
        return;
    }
    let eg = a.eg.clone().nil();
    for ec in group::iter(d, &eg) {
        card::register_flag_effect(d, ec, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
}
