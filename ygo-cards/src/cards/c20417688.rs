//! Stars Align across the Milky Way
use super::prelude::*;

pub const ID: u32 = 20417688;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E as u32));
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Copy Ritual Spell effect
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E as u32));
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(cptg)));
    effect::set_operation(d, e2, Some(operation(cpop)));
    card::register_effect(d, c, e2, false);
}
/// The arguments of an effect function, then `extra` (the values a Lua call adds)
fn args_with(a: &EffArgs, extra: Vec<Value>) -> Vec<Value> {
    let mut v = vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ];
    v.extend(extra);
    v
}
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    !card::is_public(d, c) && card::is_ritual_monster(d, c) && duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, c)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
fn thfilter(d: &mut Duel, c: CardId, rc: CardId) -> bool {
    let lv = card::get_level(d, rc);
    let codes = card::get_code(d, rc);
    card::is_ritual_monster(d, c) && card::is_able_to_hand(d, c, None) && card::is_level(d, c, &[lv]) && !card::is_code(d, c, &codes)
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    effect::set_label(d, a.e, &[1]);
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| cfilter(d, c, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    duel::confirm_cards(d, 1 - tp, g.clone());
    duel::shuffle_hand(d, tp);
    duel::set_target_card(d, g);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let res = effect::get_label(d, a.e) != 0;
        effect::set_label(d, a.e, &[0]);
        return res;
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let rc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, rc, a.e) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| thfilter(d, c, rc)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, g);
    }
}
fn cpfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_ritual_spell(d, c) && card::is_able_to_grave_as_cost(d, c) && card::check_activate_effect(d, c, true, true, false).is_some()
}
fn cptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        // local te=e:GetLabelObject(); local tg=te:GetTarget(); return tg(e,tp,eg,ep,ev,re,r,rp,0,chkc)
        let te = effect::get_label_object(d, a.e).as_effect().nil();
        let tg = effect::get_target(d, te).nil();
        return tg.call(d, &args_with(a, vec![Value::Int(0), Value::Card(chkc)])).truthy();
    }
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cpfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&cpfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let first = group::get_first(d, &g).nil();
    let te = card::check_activate_effect(d, first, true, true, false).nil().0;
    effect::set_label_object(d, a.e, Value::Effect(te));
    duel::send_to_grave(d, g, REASON_COST, None, None);
    let (p1, p2) = effect::get_property(d, te);
    effect::set_property(d, a.e, p1, p2);
    let tg = effect::get_target(d, te);
    if let Some(tg) = tg {
        tg.call(d, &args_with(a, vec![Value::Int(1)]));
    }
    duel::clear_operation_info(d, 0);
    true
}
fn cpop(d: &mut Duel, a: &EffArgs) {
    // local te=e:GetLabelObject()
    let te = match effect::get_label_object(d, a.e).as_effect() {
        Some(te) => te,
        None => return,
    };
    // local op=te:GetOperation()
    if let Some(op) = effect::get_operation(d, te) {
        op.call(d, &args_with(a, vec![]));
    }
}
