//! S-Force World Sorting
use super::prelude::*;

pub const ID: u32 = 11642993;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[11642993], listed_series: &[SET_S_FORCE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e0, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e0, false);
    // If you would banish a card(s) from the hand to activate the effect of an "S-Force" monster you control, you can send 1 "S-Force" card from your Deck to the GY instead, except "S-Force Sorted World"
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EFFECT_COST_REPLACE);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(repcon)));
    effect::set_value_fn(d, e1, lua_fn(repval));
    effect::set_operation(d, e1, Some(lua_fn(repop)));
    card::register_effect(d, c, e1, false);
    // If another card(s) is banished: You can target 1 card in your opponent's field or GY; banish it, then you can move 1 "S-Force" monster you control to another of your Main Monster Zones
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_REMOVE);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_REMOVE);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(bancond)));
    effect::set_target(d, e2, Some(cost_target(bantg)));
    effect::set_operation(d, e2, Some(operation(banop)));
    card::register_effect(d, c, e2, false);
}
/// `s.repcostfilter(c,extracon,base,e,tp,eg,ep,ev,re,r,rp)`. `raw` holds the arguments after `c`
/// (`None` where the Lua passes only `c`): `[base, extracon, e, tp, eg, ep, ev, re, r, rp, ...]`.
fn repcostfilter(d: &mut Duel, c: CardId, raw: Option<&[Value]>) -> bool {
    if !(card::is_set_card(d, c, &[SET_S_FORCE as u16]) && !card::is_code(d, c, &[ID]) && card::is_able_to_grave_as_cost(d, c)) {
        return false;
    }
    let Some(raw) = raw else { return true };
    match raw.get(1) {
        Some(Value::Fn(f)) => {
            // extracon(base,e,tp,eg,ep,ev,re,r,rp,c)
            let args = [
                raw[0].clone(),
                raw[2].clone(),
                raw[3].clone(),
                raw[4].clone(),
                raw[5].clone(),
                raw[6].clone(),
                raw[7].clone(),
                raw[8].clone(),
                raw[9].clone(),
                Value::Card(c),
            ];
            f.call(d, &args).truthy()
        }
        _ => true,
    }
}
fn repcon(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&|d, c| repcostfilter(d, c, None)), p, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.repval(base,extracon,e,tp,eg,ep,ev,re,r,rp,chk)`
fn repval(d: &mut Duel, a: Args) -> bool {
    let raw: Vec<Value> = a.0.to_vec();
    let e = a.e(2);
    let tp = a.p(3);
    if a.int(10) == 0 {
        let c = effect::get_handler(d, e).nil();
        return card::is_faceup(d, c)
            && card::is_controler(d, c, tp)
            && card::is_location(d, c, LOCATION_MZONE)
            && card::is_set_card(d, c, &[SET_S_FORCE as u16])
            && duel::is_existing_matching_card(d, Some(&|d, c| repcostfilter(d, c, Some(&raw[..]))), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    chain::is_triggering(d, 0, chain::Prop::Controler, &[tp as i64]).truthy()
        && chain::is_triggering(d, 0, chain::Prop::Location, &[LOCATION_MZONE as i64]).truthy()
        && chain::is_triggering(d, 0, chain::Prop::Position, &[POS_FACEUP as i64]).truthy()
        && chain::is_triggering(d, 0, chain::Prop::SetCard, &[SET_S_FORCE as i64]).truthy()
        && duel::is_existing_matching_card(d, Some(&|d, c| repcostfilter(d, c, Some(&raw[..]))), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.repop(base,extracon,e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: Args) -> Value {
    let raw: Vec<Value> = a.0.to_vec();
    let tp = a.p(3);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| repcostfilter(d, c, Some(&raw[..]))), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    duel::send_to_grave(d, &g, REASON_COST, None, None);
    Value::Nil
}
fn bancond(d: &mut Duel, a: &EffArgs) -> bool {
    group::is_exists(d, &a.eg(), &|d, c| card::is_location(d, c, LOCATION_REMOVED), 1, NO_EX)
}
fn bantg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp)
            && card::is_location(d, chkc, LOCATION_ONFIELD | LOCATION_GRAVE)
            && card::is_able_to_remove(d, chkc, None, None, None);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, tp, 0);
    true
}
fn banop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    let Some(tc) = duel::first_target(d) else { return };
    if card::is_relate_to_effect(d, tc, e)
        && duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0
        && card::is_location(d, tc, LOCATION_REMOVED)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    {
        let g = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_S_FORCE as u16]))), tp, LOCATION_MZONE, 0, NO_EX);
        if g.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            return;
        }
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 3));
        let sc = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        let Some(sc) = group::get_first(d, &sc) else { return };
        duel::hint_selection(d, sc, true);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOZONE as u64);
        let zone = duel::select_disable_field(d, tp, 1, LOCATION_MZONE, 0, Some(0), false);
        duel::break_effect(d);
        if card::is_immune_to_effect(d, tc, e) {
            return;
        }
        if zone == 0 {
            // math.log(nil, 2) is an error in Lua
            crate::lua_error("bad argument #1 to 'log' (number expected, got nil)");
        }
        duel::move_sequence(d, sc, zone.trailing_zeros() as u8, None);
    }
}
