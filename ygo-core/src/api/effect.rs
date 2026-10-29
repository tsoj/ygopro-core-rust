//! `Effect.*` (`libeffect.cpp`).

use crate::constants::*;
use crate::duel::Duel;
use crate::types::*;

/// `Effect.CreateEffect(c)`
pub fn create_effect(d: &mut Duel, c: CardId) -> EffectId {
    let e = d.new_effect();
    let rp = d.core.reason_player;
    let fx = d.effect_mut(e);
    fx.effect_owner = rp;
    fx.owner = Some(c);
    e
}
/// `Effect.GlobalEffect()`
pub fn global_effect(d: &mut Duel) -> EffectId {
    let e = d.new_effect();
    let tc = d.temp_card;
    let fx = d.effect_mut(e);
    fx.effect_owner = 0;
    fx.owner = Some(tc);
    e
}
/// `e:Clone()`
pub fn clone(d: &mut Duel, e: EffectId) -> EffectId {
    e.clone_effect(d, false)
}
/// `e:Reset()`
pub fn reset(d: &mut Duel, e: EffectId) {
    let fx = d.effect(e);
    if fx.owner.is_none() {
        return;
    }
    if fx.is_flag(EFFECT_FLAG_FIELD_ONLY) {
        d.remove_effect(e);
    } else if let Some(h) = fx.handler {
        h.remove_effect(d, e);
    } else if !d.core.reseted_effects.contains(&e) {
        d.core.reseted_effects.push(e);
    }
}
pub fn get_field_id(d: &Duel, e: EffectId) -> u32 {
    d.effect(e).id
}
pub fn set_description(d: &mut Duel, e: EffectId, v: u64) {
    d.effect_mut(e).description = v;
}
pub fn set_code(d: &mut Duel, e: EffectId, v: u32) {
    d.effect_mut(e).code = v;
}
pub fn set_range(d: &mut Duel, e: EffectId, v: u32) {
    let mut range = v as u16;
    let (mm, em, m) = (LOCATION_MMZONE as u16, LOCATION_EMZONE as u16, LOCATION_MZONE as u16);
    let (fz, st, pz, s) = (LOCATION_FZONE as u16, LOCATION_STZONE as u16, LOCATION_PZONE as u16, LOCATION_SZONE as u16);
    if range & (mm | em) == (mm | em) {
        range |= m;
    }
    if range & m != 0 {
        range &= !(mm | em);
    }
    if range & (fz | st | pz) == (fz | st | pz) {
        range |= s;
    }
    if range & s != 0 {
        range &= !(fz | st | pz);
    }
    d.effect_mut(e).range = range;
}
pub fn set_target_range(d: &mut Duel, e: EffectId, s: u32, o: u32) {
    let fx = d.effect_mut(e);
    fx.s_range = s as u16;
    fx.o_range = o as u16;
    fx.flag[0] &= !EFFECT_FLAG_ABSOLUTE_TARGET;
}
pub fn set_absolute_range(d: &mut Duel, e: EffectId, playerid: u8, s: u32, o: u32) {
    let fx = d.effect_mut(e);
    if playerid == 0 {
        fx.s_range = s as u16;
        fx.o_range = o as u16;
    } else {
        fx.s_range = o as u16;
        fx.o_range = s as u16;
    }
    fx.flag[0] |= EFFECT_FLAG_ABSOLUTE_TARGET;
}
/// `e:SetCountLimit(count, code_or_{code, hopt_index}, flag)`
pub fn set_count_limit(d: &mut Duel, e: EffectId, count: u8, code: u32, hopt_index: u8, flag: u8) {
    assert!(count != 0, "The count must not be 0");
    let hopt_index = if code != 0 { hopt_index } else { 0 };
    let mut flag = flag;
    if flag & EFFECT_COUNT_CODE_CHAIN as u8 != 0 && code == 0 {
        flag |= EFFECT_COUNT_CODE_SINGLE as u8;
    }
    let fx = d.effect_mut(e);
    fx.flag[0] |= EFFECT_FLAG_COUNT_LIMIT;
    fx.count_limit = count;
    fx.count_limit_max = count;
    fx.count_code = code;
    fx.count_flag = flag;
    fx.count_hopt_index = hopt_index;
}
pub fn set_reset(d: &mut Duel, e: EffectId, v: u32, c: u16) {
    let mut v = v;
    let c = if c == 0 { 1 } else { c };
    if v & RESET_PHASE != 0 && v & (RESET_SELF_TURN | RESET_OPPO_TURN) == 0 {
        v |= RESET_SELF_TURN | RESET_OPPO_TURN;
    }
    let fx = d.effect_mut(e);
    fx.reset_flag = v;
    fx.reset_count = c;
}
pub fn set_type(d: &mut Duel, e: EffectId, v: u32) {
    let mut v = v as u16;
    let actions = EFFECT_TYPE_ACTIONS as u16;
    if v & 0x0ff0 != 0 {
        v |= actions;
    } else {
        v &= !actions;
    }
    if v as u32 & (EFFECT_TYPE_ACTIVATE | EFFECT_TYPE_IGNITION | EFFECT_TYPE_QUICK_O | EFFECT_TYPE_QUICK_F) != 0 {
        v |= EFFECT_TYPE_FIELD as u16;
    }
    let fx = d.effect_mut(e);
    if v as u32 & EFFECT_TYPE_ACTIVATE != 0 {
        fx.range = (LOCATION_SZONE + LOCATION_FZONE + LOCATION_HAND) as u16;
    }
    if v as u32 & EFFECT_TYPE_FLIP != 0 {
        fx.code = EVENT_FLIP;
        if v as u32 & EFFECT_TYPE_TRIGGER_O == 0 {
            v |= EFFECT_TYPE_TRIGGER_F as u16;
        }
    }
    fx.type_ = v;
}
pub fn set_property(d: &mut Duel, e: EffectId, v1: u32, v2: u32) {
    let fx = d.effect_mut(e);
    fx.flag[0] = (fx.flag[0] & 0x4f) | (v1 & !0x4f);
    fx.flag[1] = v2;
}
pub fn set_label(d: &mut Duel, e: EffectId, labels: &[i64]) {
    d.effect_mut(e).label = labels.to_vec();
}
pub fn set_label_object(d: &mut Duel, e: EffectId, v: Value) {
    d.effect_mut(e).label_object = v;
}
pub fn set_category(d: &mut Duel, e: EffectId, v: u64) {
    d.effect_mut(e).category = v;
}
/// `e:SetHintTiming(vs, vo = vs)`
pub fn set_hint_timing(d: &mut Duel, e: EffectId, vs: u32, vo: Option<u32>) {
    d.effect_mut(e).hint_timing = [vs, vo.unwrap_or(vs)];
}
pub fn set_condition(d: &mut Duel, e: EffectId, f: Option<LuaFn>) {
    e.set_condition(d, f);
}
pub fn set_target(d: &mut Duel, e: EffectId, f: Option<LuaFn>) {
    d.effect_mut(e).target = f;
}
pub fn set_cost(d: &mut Duel, e: EffectId, f: Option<LuaFn>) {
    d.effect_mut(e).cost = f;
}
pub fn set_operation(d: &mut Duel, e: EffectId, f: Option<LuaFn>) {
    d.effect_mut(e).operation = f;
}
/// `e:SetValue(v)` with a number or boolean.
pub fn set_value(d: &mut Duel, e: EffectId, v: i64) {
    let fx = d.effect_mut(e);
    fx.value_fn = None;
    if v > i32::MAX as i64 || v < i32::MIN as i64 {
        // (a value that does not fit is kept as a function returning it)
        fx.value = 0;
        fx.value_fn = Some(LuaFn::new(move |_, _| Value::Int(v)));
        fx.flag[0] |= EFFECT_FLAG_FUNC_VALUE;
    } else {
        fx.flag[0] &= !EFFECT_FLAG_FUNC_VALUE;
        fx.value = v as i32;
    }
}
/// `e:SetValue(f)` with a function.
pub fn set_value_fn(d: &mut Duel, e: EffectId, f: LuaFn) {
    let fx = d.effect_mut(e);
    fx.value = 0;
    fx.value_fn = Some(f);
    fx.flag[0] |= EFFECT_FLAG_FUNC_VALUE;
}
pub fn set_owner_player(d: &mut Duel, e: EffectId, p: u8) {
    if p == 0 || p == 1 {
        d.effect_mut(e).effect_owner = p;
    }
}
pub fn get_description(d: &Duel, e: EffectId) -> u64 {
    d.effect(e).description
}
pub fn get_code(d: &Duel, e: EffectId) -> u32 {
    d.effect(e).code
}
pub fn get_range(d: &Duel, e: EffectId) -> u32 {
    d.effect(e).range as u32
}
pub fn get_target_range(d: &Duel, e: EffectId) -> (u32, u32) {
    let fx = d.effect(e);
    (fx.s_range as u32, fx.o_range as u32)
}
/// `(count_limit, count_limit_max, count_code, count_flag, count_hopt_index)`
pub fn get_count_limit(d: &Duel, e: EffectId) -> (u32, u32, u32, u32, u32) {
    let fx = d.effect(e);
    (fx.count_limit as u32, fx.count_limit_max as u32, fx.count_code, fx.count_flag as u32, fx.count_hopt_index as u32)
}
pub fn get_reset(d: &Duel, e: EffectId) -> (u32, u32) {
    let fx = d.effect(e);
    (fx.reset_flag, fx.reset_count as u32)
}
pub fn get_type(d: &Duel, e: EffectId) -> u32 {
    d.effect(e).type_ as u32
}
pub fn get_property(d: &Duel, e: EffectId) -> (u32, u32) {
    let fx = d.effect(e);
    (fx.flag[0], fx.flag[1])
}
/// `e:GetLabel()`: all labels (0 if there is none).
pub fn get_labels(d: &Duel, e: EffectId) -> Vec<i64> {
    let l = &d.effect(e).label;
    if l.is_empty() { vec![0] } else { l.clone() }
}
/// `e:GetLabel()` (the first label).
pub fn get_label(d: &Duel, e: EffectId) -> i64 {
    d.effect(e).label.first().copied().unwrap_or(0)
}
pub fn get_label_object(d: &Duel, e: EffectId) -> Value {
    d.effect(e).label_object.clone()
}
pub fn get_category(d: &Duel, e: EffectId) -> u64 {
    d.effect(e).category
}
pub fn get_owner(d: &Duel, e: EffectId) -> Option<CardId> {
    d.effect(e).owner.map(|_| e.get_owner(d))
}
pub fn get_handler(d: &Duel, e: EffectId) -> Option<CardId> {
    e.get_handler_opt(d)
}
pub fn get_owner_player(d: &Duel, e: EffectId) -> u8 {
    e.get_owner_player(d)
}
pub fn get_handler_player(d: &Duel, e: EffectId) -> u8 {
    e.get_handler_player(d)
}
pub fn get_hint_timing(d: &Duel, e: EffectId) -> (u32, u32) {
    let fx = d.effect(e);
    (fx.hint_timing[0], fx.hint_timing[1])
}
pub fn get_condition(d: &Duel, e: EffectId) -> Option<LuaFn> {
    d.effect(e).condition().clone()
}
pub fn get_target(d: &Duel, e: EffectId) -> Option<LuaFn> {
    d.effect(e).target.clone()
}
pub fn get_cost(d: &Duel, e: EffectId) -> Option<LuaFn> {
    d.effect(e).cost.clone()
}
/// `e:GetValue()`: the function, or the number.
pub fn get_value(d: &Duel, e: EffectId) -> Value {
    let fx = d.effect(e);
    if fx.is_flag(EFFECT_FLAG_FUNC_VALUE) {
        fx.value_fn.clone().map_or(Value::Nil, Value::Fn)
    } else {
        Value::Int(fx.value as i64)
    }
}
pub fn get_operation(d: &Duel, e: EffectId) -> Option<LuaFn> {
    d.effect(e).operation.clone()
}
pub fn get_active_type(d: &mut Duel, e: EffectId) -> u32 {
    e.get_active_type(d)
}
pub fn is_active_type(d: &mut Duel, e: EffectId, t: u32) -> bool {
    e.get_active_type(d) & t != 0
}
pub fn is_has_property(d: &Duel, e: EffectId, f1: u32, f2: u32) -> bool {
    let fx = d.effect(e);
    (f1 == 0 || fx.flag[0] & f1 != 0) && (f2 == 0 || fx.flag[1] & f2 != 0)
}
pub fn is_has_category(d: &Duel, e: EffectId, c: u64) -> bool {
    d.effect(e).category & c != 0
}
pub fn is_has_type(d: &Duel, e: EffectId, t: u32) -> bool {
    d.effect(e).type_ as u32 & (t & 0xffff) != 0
}
pub fn is_activatable(d: &mut Duel, e: EffectId, playerid: u8, neglect_loc: bool, neglect_target: bool) -> bool {
    let ne = d.nil_event.clone();
    e.is_activateable(d, playerid, &ne, false, false, neglect_target, neglect_loc, false)
}
pub fn is_activated(d: &Duel, e: EffectId) -> bool {
    d.effect(e).type_ & 0x7f0 != 0
}
pub fn get_activate_location(d: &Duel, e: EffectId) -> u32 {
    d.effect(e).active_location as u32
}
pub fn get_activate_sequence(d: &Duel, e: EffectId) -> u32 {
    d.effect(e).active_sequence as u32
}
pub fn check_count_limit(d: &mut Duel, e: EffectId, p: u8) -> bool {
    e.check_count_limit(d, p)
}
pub fn use_count_limit(d: &mut Duel, e: EffectId, p: u8, count: u32, oath_only: bool) {
    if !oath_only || d.effect(e).count_flag as u32 & EFFECT_COUNT_CODE_OATH != 0 {
        for _ in 0..count {
            e.dec_count(d, p as u32);
        }
    }
}
pub fn restore_count_limit(d: &mut Duel, e: EffectId, p: u8, count: u32, oath_only: bool) {
    if !oath_only || d.effect(e).count_flag as u32 & EFFECT_COUNT_CODE_OATH != 0 {
        for _ in 0..count {
            e.inc_count(d, p as u32);
        }
    }
}
/// `e:IsDeleted()`
pub fn is_deleted(d: &Duel, e: EffectId) -> bool {
    !d.effect_exists(e)
}
