//! The Weather Auroral Canvas
use super::prelude::*;

pub const ID: u32 = 52834429;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_THE_WEATHER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Can only control one
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // effect gain (not registered on the card: the grant effect below gives it to "The Weather" monsters)
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_REMOVE + CATEGORY_DRAW);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_TO_HAND);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(rmcon)));
    effect::set_cost(d, e2, Some(cost_target(announcecost)));
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    effect::set_operation(d, e2, Some(operation(rmop)));
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_GRANT);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    effect::set_target(d, e3, Some(effect_card_fn(eftg)));
    effect::set_label_object(d, e3, Value::Effect(e2));
    card::register_effect(d, c, e3, false);
}
/// `s.eftg(e,c)`
fn eftg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let h = effect::get_handler(d, e).nil();
    let g = card::get_column_group(d, h, 1, 1);
    card::is_type(d, c, TYPE_EFFECT) && card::is_set_card(d, c, &[SET_THE_WEATHER as u16]) && card::get_sequence(d, c) < 5 && group::is_contains(&g, c)
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(_d: &mut Duel, a: &EffArgs) -> bool {
    a.eg.clone().nil().len() == 1
}
/// `aux.bfgcost(e,tp,eg,ep,ev,re,r,rp,chk)` with the arguments of the current call
fn bfgcost_call(d: &mut Duel, a: &EffArgs, chk: i64) -> bool {
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
/// `s.announcecost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn announcecost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return bfgcost_call(d, a, 0);
    }
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    bfgcost_call(d, a, a.chk.unwrap_or(1));
    true
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let ec = group::get_first(d, &eg);
    if a.chk == Some(0) {
        let Some(ec) = ec else {
            return false;
        };
        if !card::is_able_to_remove(d, ec, None, None, None) {
            return false;
        }
        let p = card::get_controler(d, ec);
        return duel::is_player_can_draw(d, p, 1);
    }
    let ec = ec.nil();
    let p = card::get_controler(d, ec);
    duel::set_target_player(d, p);
    duel::set_target_param(d, 1);
    effect::set_label_object(d, a.e, Value::Card(ec));
    card::create_effect_relation(d, ec, a.e);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(ec)), 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, p, 1);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let Some(tc) = effect::get_label_object(d, a.e).as_card() else {
        return;
    };
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let count = info[1].int() as u32;
    if duel::remove(d, CardSet::from_card(tc), POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 {
        duel::draw(d, p, count, REASON_EFFECT);
    }
}
