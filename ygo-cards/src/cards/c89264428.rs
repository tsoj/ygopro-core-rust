//! Ursarctic Big Dipper
use super::prelude::*;

pub const ID: u32 = 89264428;
const COUNTER_BIG_DIPPER: u16 = 0x204;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_URSARCTIC], counter_place_list: &[0x204], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_counter_permit(d, c, COUNTER_BIG_DIPPER, None, None);
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // Once per turn, if your "Ursarctic" monster would Tribute a monster(s) to activate its effect, you can banish 1 Level 7 or higher "Ursarctic" monster from your GY instead
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EFFECT_COST_REPLACE);
    effect::set_range(d, e1, LOCATION_FZONE);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(lua_fn(repcon)));
    effect::set_value_fn(d, e1, lua_fn(repval));
    effect::set_operation(d, e1, Some(lua_fn(repop)));
    card::register_effect(d, c, e1, false);
    // Each time a monster(s) is Special Summoned, place 1 counter on this card
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_operation(d, e2, Some(operation(add_counter_op)));
    card::register_effect(d, c, e2, false);
    // Take control of 1 monster your opponent controls
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_CONTROL);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e3, LOCATION_FZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_cost(d, e3, Some(cost_target(ctrlcost)));
    effect::set_target(d, e3, Some(cost_target(ctrltg)));
    effect::set_operation(d, e3, Some(operation(ctrlop)));
    card::register_effect(d, c, e3, false);
}
/// `s.extracon(base,e,tp,eg,ep,ev,re,r,rp,c)` of the Lua `extracon` argument, when there is one: `not extracon or extracon(...)`
fn extracon_ok(d: &mut Duel, ex: &[Value], c: CardId) -> bool {
    let v = |i: usize| ex.get(i).cloned().unwrap_or(Value::Nil);
    match ex.get(1) {
        Some(Value::Fn(f)) => {
            let full = vec![v(0), v(2), v(3), v(4), v(5), v(6), v(7), v(8), v(9), Value::Card(c)];
            f.call(d, &full).truthy()
        }
        _ => true,
    }
}
/// `s.repconfilter(c,extracon,base,e,tp,eg,ep,ev,re,r,rp)`; `ex` holds `extracon, base, e, tp, ...` in that order
fn repconfilter(d: &mut Duel, c: CardId, ex: &[Value]) -> bool {
    card::is_level_above(d, c, 7) && card::is_set_card(d, c, &[SET_URSARCTIC as u16]) && card::is_able_to_remove_as_cost(d, c, None) && extracon_ok(d, ex, c)
}
/// `s.repcon(e)`
fn repcon(d: &mut Duel, a: Args) -> bool {
    let p = effect::get_handler_player(d, a.e(0));
    duel::is_existing_matching_card(d, Some(&|d, cc| repconfilter(d, cc, &[])), p, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.repval(base,extracon,e,tp,eg,ep,ev,re,r,rp,chk)`
fn repval(d: &mut Duel, a: Args) -> bool {
    let c = effect::get_handler(d, a.e(2)).nil();
    let p = effect::get_handler_player(d, a.e(2));
    let ex = a.0.to_vec();
    card::is_set_card(d, c, &[SET_URSARCTIC as u16]) && card::is_monster(d, c) && duel::is_existing_matching_card(d, Some(&|d, cc| repconfilter(d, cc, &ex)), p, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.repop(base,extracon,e,tp,eg,ep,ev,re,r,rp,chk)`
fn repop(d: &mut Duel, a: Args) {
    let tp = a.p(3);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let ex = a.0.to_vec();
    let g = duel::select_matching_card(d, tp, Some(&|d, cc| repconfilter(d, cc, &ex)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST | REASON_REPLACE, None, None);
}
/// `function(e) e:GetHandler():AddCounter(COUNTER_BIG_DIPPER,1) end`
fn add_counter_op(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    card::add_counter(d, h, COUNTER_BIG_DIPPER, 1, false);
}
/// `s.ctrlcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn ctrlcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_can_remove_counter(d, c, tp, COUNTER_BIG_DIPPER, 7, REASON_COST);
    }
    let n = card::get_counter(d, c, COUNTER_BIG_DIPPER) as u16;
    card::remove_counter(d, c, tp, COUNTER_BIG_DIPPER, n, REASON_COST);
    true
}
fn ctrlconfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_URSARCTIC as u16]) && card::is_type(d, c, TYPE_SYNCHRO) && card::is_faceup(d, c)
}
fn can_be_changed(d: &mut Duel, c: CardId) -> bool {
    card::is_controler_can_be_changed(d, c, false, None)
}
fn ctrltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && can_be_changed(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&ctrlconfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
            && duel::is_existing_target(d, Some(&can_be_changed), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    let g = duel::select_target(d, tp, Some(&can_be_changed), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, tp, 0);
    true
}
fn ctrlop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::get_control(d, tc, tp, 0, 0, None, None);
    }
}
