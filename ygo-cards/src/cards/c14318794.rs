//! Life Absorbing Machine
use super::prelude::*;

pub const ID: u32 = 14318794;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, TIMING_STANDBY_PHASE, Some(0));
    card::register_effect(d, c, e1, false);
    // recover
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_RECOVER);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    effect::set_condition(d, e2, Some(condition(reccon)));
    effect::set_target(d, e2, Some(cost_target(rectg)));
    effect::set_operation(d, e2, Some(operation(recop)));
    card::register_effect(d, c, e2, false);
    aux::global_check(d, ID, |d| {
        // s[0]={} s[1]={} s[2]={} s[3]={}
        for i in 0..4 {
            set_s_var(d, ID, i, Value::Multi(vec![]));
        }
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_PAY_LPCOST);
        effect::set_operation(d, ge1, Some(operation(checkop)));
        duel::register_effect(d, ge1, 0);
        aux::add_values_reset(d, reset_values);
    });
}
/// The list `s[i]` (a Lua table of numbers).
fn list(d: &Duel, i: u64) -> Vec<i64> {
    match s_var(d, ID, i) {
        Value::Multi(v) => v.iter().map(|x| x.int()).collect(),
        _ => Vec::new(),
    }
}
fn to_value(t: Vec<i64>) -> Value {
    Value::Multi(t.into_iter().map(Value::Int).collect())
}
/// `aux.AddValuesReset` function
fn reset_values(d: &mut Duel) {
    let p = duel::get_turn_player(d) as u64;
    // s[p+2]={table.unpack(s[p])}
    let v = s_var(d, ID, p);
    set_s_var(d, ID, p + 2, v);
    // s[p]={}
    set_s_var(d, ID, p, Value::Multi(vec![]));
}
fn checkop(d: &mut Duel, a: &EffArgs) {
    if a.ep == duel::get_turn_player(d) {
        // local val=math.ceil(ev/2)
        let val = (a.ev as i64 + 1) / 2;
        // table.insert(s[ep],val)
        let mut t = list(d, a.ep as u64);
        t.push(val);
        set_s_var(d, ID, a.ep as u64, to_value(t));
    }
}
fn reccon(d: &mut Duel, a: &EffArgs) -> bool {
    a.tp == duel::get_turn_player(d)
}
fn rectg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        // #s[tp+2]>0 and (c:GetFlagEffect(id)==0 or s[tp+2][c:GetFlagEffectLabel(id)+1])
        return !list(d, tp as u64 + 2).is_empty()
            && (card::get_flag_effect(d, c, ID) == 0 || {
                let label = card::get_flag_effect_label(d, c, ID).first().copied().nil();
                (label as usize) < list(d, tp as u64 + 2).len()
            });
    }
    let rec: Option<i64>;
    if card::get_flag_effect(d, c, ID) == 0 {
        rec = list(d, tp as u64 + 2).first().copied();
        card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY, 0, 1, 1, 0);
    } else {
        let label = card::get_flag_effect_label(d, c, ID).first().copied().nil();
        rec = list(d, tp as u64 + 2).get(label as usize).copied();
        card::set_flag_effect_label(d, c, ID, (label + 1) as u32);
    }
    let rec = rec.nil();
    duel::set_target_player(d, tp);
    duel::set_target_param(d, rec as u32);
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, rec as i32);
    true
}
fn recop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::recover(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
