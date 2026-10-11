//! Archfiend's Usurpation
use super::prelude::*;

pub const ID: u32 = 82997779;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARCHFIEND], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(efftg)));
    effect::set_operation(d, e1, Some(operation(effop)));
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e1, false);
}
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ARCHFIEND as u16]) && card::is_trap(d, c) && card::is_s_setable(d, c, true, None)
}
/// `Ritual.Target(ritual_params)`: the `ritual_params` of both the target and the operation.
fn ritual_params() -> ritual::RitualProc {
    ritual::RitualProc {
        lvtype: RITPROC_GREATER,
        filter: Some(std::sync::Arc::new(|d: &mut Duel, c: CardId| card::is_set_card(d, c, &[SET_ARCHFIEND as u16]))),
        location: Some(LOCATION_HAND | LOCATION_EXTRA),
        ..Default::default()
    }
}
/// The Lua arguments `(e,tp,eg,ep,ev,re,r,rp[,chk])` of a target or operation.
fn lua_args(a: &EffArgs, chk: Option<i64>) -> Vec<Value> {
    let mut v = vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        a.re.map_or(Value::Nil, Value::Effect),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ];
    if let Some(chk) = chk {
        v.push(Value::Int(chk));
    }
    v
}
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let ft = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0;
    let c = effect::get_handler(d, a.e).nil();
    let b1 = ((card::is_location(d, c, LOCATION_SZONE) && ft > 0) || ft > 1)
        && duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    let b2 = ritual::target(d, ritual_params()).call(d, &lua_args(a, Some(0))).truthy();
    if a.chk == Some(0) {
        return b1 || b2;
    }
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).map_or(0, |x| x as i64);
    effect::set_label(d, a.e, &[op]);
    if op == 1 {
        effect::set_category(d, a.e, CATEGORY_SET);
    } else if op == 2 {
        effect::set_category(d, a.e, CATEGORY_RELEASE + CATEGORY_SPECIAL_SUMMON);
        ritual::target(d, ritual_params()).call(d, &lua_args(a, Some(1)));
        duel::set_operation_info(d, 0, CATEGORY_RELEASE, None, 1, a.tp, (LOCATION_HAND | LOCATION_MZONE) as i32);
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_EXTRA) as i32);
    }
    true
}
fn effop(d: &mut Duel, a: &EffArgs) {
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // Set 1 "Archfiend" Trap from your Deck or GY, it can be activated this turn
        if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
        let f = aux::necro_valley_filter(|d, c| setfilter(d, c));
        let sc = duel::select_matching_card(d, a.tp, Some(&f), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        if let Some(sc) = group::get_first(d, &sc) {
            if duel::s_set(d, a.tp, sc, None, true) > 0 {
                // It can be activated this turn
                let c = effect::get_handler(d, a.e).nil();
                let e1 = effect::create_effect(d, c);
                effect::set_description(d, e1, aux::stringid(ID, 3));
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_CLIENT_HINT, 0);
                effect::set_code(d, e1, EFFECT_TRAP_ACT_IN_SET_TURN);
                effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
                card::register_effect(d, sc, e1, false);
            }
        }
    } else if op == 2 {
        // Ritual Summon 1 "Archfiend" Ritual Monster from your hand or face-up Extra Deck, by Tributing monsters from your hand or field whose total Levels equal or exceed its Level
        ritual::operation(d, ritual_params()).call(d, &lua_args(a, None));
    }
}
