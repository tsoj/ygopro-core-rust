//! Performage Damage Juggler
use super::prelude::*;

pub const ID: u32 = 68819554;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_PERFORMAGE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // negate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_NEGATE);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_condition(d, e1, Some(condition(negcon)));
    effect::set_cost(d, e1, Some(cost_target(effcost)));
    effect::set_target(d, e1, Some(cost_target(negtg)));
    effect::set_operation(d, e1, Some(operation(negop)));
    card::register_effect(d, c, e1, false);
    // reduce damage
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_condition(d, e2, Some(condition(damcon)));
    effect::set_cost(d, e2, Some(cost_target(effcost)));
    effect::set_operation(d, e2, Some(operation(damop)));
    card::register_effect(d, c, e2, false);
    // search
    let e3 = effect::create_effect(d, c);
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_GRAVE);
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_cost(d, e3, Some(cost::self_banish()));
    effect::set_target(d, e3, Some(cost_target(thtg)));
    effect::set_operation(d, e3, Some(operation(thop)));
    card::register_effect(d, c, e3, false);
}
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_discardable(d, c, None, None);
    }
    duel::send_to_grave(d, c, REASON_COST | REASON_DISCARD, None, None);
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    true
}
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    if !duel::is_chain_negatable(d, a.ev as u8) {
        return false;
    }
    let eg = a.eg.clone().map_or(Value::Nil, Value::Group);
    aux::damcon1()
        .call(d, &[Value::Effect(a.e), Value::Int(a.tp as i64), eg, Value::Int(a.ep as i64), Value::Int(a.ev as i64), Value::effect(a.re), Value::Int(a.r as i64), Value::Int(a.rp as i64)])
        .truthy()
}
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    if card::is_relate_to_effect(d, rc, re) && card::is_destructable(d, rc, None) {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
fn negop(d: &mut Duel, a: &EffArgs) {
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let rc = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rc, re) {
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
fn damcon(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_battle_phase(d, None)
}
fn damop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_AVOID_BATTLE_DAMAGE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE_CAL, 1);
    duel::register_effect(d, e1, a.tp);
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_PERFORMAGE as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None) && !card::is_code(d, c, &[ID])
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
