//! Summon Gate
use super::prelude::*;

pub const ID: u32 = 29724053;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // summon count limit
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e2, 1, 1);
    effect::set_target(d, e2, Some(lua_fn(sumlimit)));
    card::register_effect(d, c, e2, false);
    //
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_code(d, e3, CARD_SUMMON_GATE);
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e3, 1, 1);
    effect::set_value_fn(d, e3, lua_fn(remainval));
    card::register_effect(d, c, e3, false);
    aux::global_check(d, ID, |d| {
        // s[0]=3 s[1]=3
        set_s_var(d, ID, 0, Value::Int(3));
        set_s_var(d, ID, 1, Value::Int(3));
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
        effect::set_code(d, ge1, EVENT_SPSUMMON_SUCCESS);
        effect::set_operation(d, ge1, Some(operation(checkop)));
        duel::register_effect(d, ge1, 0);
        aux::add_values_reset(d, reset_values);
    });
}
/// `s[i]`
fn sv(d: &Duel, i: u64) -> i64 {
    s_var(d, ID, i).int()
}
/// `aux.AddValuesReset` function
fn reset_values(d: &mut Duel) {
    set_s_var(d, ID, 0, Value::Int(3));
    set_s_var(d, ID, 1, Value::Int(3));
}
/// `function(e,c,sump,sumtype,sumpos,targetp,se)`
fn sumlimit(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    let sump = a.int(2) as u64;
    card::is_location(d, c, LOCATION_EXTRA) && sv(d, sump) <= 0
}
/// `function(sump) return math.max(s[sump],0) end`
fn remainval(d: &mut Duel, a: Args) -> i64 {
    let sump = a.int(0) as u64;
    sv(d, sump).max(0)
}
/// `function(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    // local tc=eg:GetFirst()
    let _ = group::get_first(d, &eg);
    for tc in group::iter(d, &eg) {
        if card::is_previous_location(d, tc, LOCATION_EXTRA) {
            let p = card::get_summon_player(d, tc);
            // s[p]=s[p]-1
            let v = sv(d, p as u64) - 1;
            set_s_var(d, ID, p as u64, Value::Int(v));
        }
    }
}
