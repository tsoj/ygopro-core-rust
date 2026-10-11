//! Brotherhood of the Fire Fist - Eland
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 61472381;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    card::enable_revive_limit(d, c);
    // Set 1 "Fire Formation" spell/trap from deck or GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(setcost)));
    effect::set_target(d, e1, Some(cost_target(settg)));
    effect::set_operation(d, e1, Some(operation(setop)));
    card::register_effect(d, c, e1, false);

    // Negate the effect of opponent's activated monster effect
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DISABLE);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(negcon)));
    effect::set_cost(d, e2, Some(cost::replaceable(cost_target(negcost), None)));
    effect::set_target(d, e2, Some(cost_target(negtg)));
    effect::set_operation(d, e2, Some(operation(negop)));
    card::register_effect(d, c, e2, false);
}

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[93754402, ID],
    listed_series: &[SET_FIRE_FORMATION, SET_FIRE_FIST],
    ..ScriptInfo::DEFAULT
};

fn setcfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_discardable(d, c, None, None)
}

fn setcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&setcfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::discard_hand(d, tp, Some(&setcfilter), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}

fn setfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_FIRE_FORMATION as u16]) && card::is_spell_trap(d, c) && card::is_s_setable(d, c, false, None)
}

fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    true
}

fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    let nv = aux::necro_valley_filter(setfilter);
    let g = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    if let Some(g) = g {
        if !g.is_empty() {
            let tc = group::get_first(d, &g).nil();
            duel::s_set(d, tp, tc, None, true);
        }
    }
}

fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let h = effect::get_handler(d, e).nil();
    if card::is_status(d, h, STATUS_BATTLE_DESTROYED) {
        return false;
    }
    if a.ep != 1 - tp {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    duel::is_chain_disablable(d, a.ev as u8)
}

fn negcostfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_FIRE_FIST as u16, SET_FIRE_FORMATION as u16])
        && !card::is_code(d, c, &[ID])
        && card::is_able_to_grave_as_cost(d, c)
}

fn negcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&negcostfilter), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&negcostfilter), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX);
    let g = g.nil();
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}

fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    true
}

fn negop(d: &mut Duel, a: &EffArgs) {
    duel::negate_effect(d, a.ev as u8);
}
