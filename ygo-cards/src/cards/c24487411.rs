//! Detonate Deleter
//! scripted by andré
use super::prelude::*;

pub const ID: u32 = 24487411;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Link Summon
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_CYBERSE),2,3)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_CYBERSE as u64, sc, st, Some(tp)))),
        2,
        Some(3),
        None,
        None,
        None,
    );

    // Destroy battling monster
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_BATTLE_START);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_target(d, e1, Some(cost_target(dtarget)));
    effect::set_operation(d, e1, Some(operation(doperation)));
    card::register_effect(d, c, e1, false);

    // Tribute to destroy 1 monster your opponent controls
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_cost(d, e2, Some(cost_target(tdcost)));
    effect::set_target(d, e2, Some(cost_target(tdtarget)));
    effect::set_operation(d, e2, Some(operation(tdoperation)));
    card::register_effect(d, c, e2, false);
}

fn dtarget(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_battle_target(d, h);
    if a.chk == Some(0) {
        return match tc {
            Some(tc) => card::is_faceup(d, tc) && !(card::is_link_monster(d, tc) && card::is_link_above(d, tc, 3)),
            None => false,
        };
    }
    let tc = tc.nil();
    let p = card::get_controler(d, tc);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(tc)), 1, p, LOCATION_MZONE as i32);
    true
}

fn doperation(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_battle_target(d, h).nil();
    if card::is_relate_to_battle(d, tc) {
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}

fn cfilter(d: &mut Duel, c: CardId, tp: u8, cg: &CardSet) -> bool {
    group::is_contains(cg, c) && duel::is_existing_target(d, None, tp, 0, LOCATION_MZONE, 1, c.into())
}

fn tdcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    let cg = card::get_linked_group(d, h);
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, tp, &cg)), 1, 1, false, None, NO_EX);
    }
    let g = duel::select_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, tp, &cg)), 1, 1, false, None, NO_EX);
    duel::release(d, g, REASON_COST, None);
    true
}

fn tdtarget(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MZONE);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let tg = duel::select_target(d, tp, None, tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, tg, 1, 1 - tp, LOCATION_MZONE as i32);
    true
}

fn tdoperation(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
