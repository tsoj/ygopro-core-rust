//! `proc_normal.lua`: Normal Summon/Set procedures with Tributes (`aux.AddNormalSummonProcedure`,
//! `aux.AddNormalSetProcedure`).

use crate::constants::*;
use crate::lua::{card, duel, effect, group, NO_EX};
use crate::prelude::*;
use std::sync::Arc;

/// `f(c, tp)`: the monsters that can be Tributed
pub type TributeFilter = Arc<dyn Fn(&mut Duel, CardId, u8) -> bool + Send + Sync>;
/// `sumop(g, e, tp, eg, ep, ev, re, r, rp, c, minc, zone, relzone, exeff)` (the arguments after `rp`
/// are in `EffArgs::extra`)
pub type SumOp = Arc<dyn Fn(&mut Duel, &CardSet, &EffArgs) + Send + Sync>;

#[allow(clippy::too_many_arguments)]
fn add_procedure(d: &mut Duel, c: CardId, set: bool, ns: bool, opt: bool, min: Option<u16>, max: Option<u16>, val: Option<u32>, desc: Option<u64>, f: Option<TributeFilter>, sumop: Option<SumOp>) -> EffectId {
    let val = val.unwrap_or(SUMMON_TYPE_TRIBUTE);
    let e1 = effect::create_effect(d, c);
    if let Some(desc) = desc {
        effect::set_description(d, e1, desc);
    }
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    if ns && opt {
        effect::set_code(d, e1, if set { EFFECT_SET_PROC } else { EFFECT_SUMMON_PROC });
    } else {
        effect::set_code(d, e1, if set { EFFECT_LIMIT_SET_PROC } else { EFFECT_LIMIT_SUMMON_PROC });
        use crate::class;
        if let Some(min) = min {
            let cur = class::get(d, c, class::MIN_TRIBUTE_REQ);
            if cur.is_nil() || (min as i64) < cur.int() {
                class::set(d, c, class::MIN_TRIBUTE_REQ, Value::Int(min as i64));
            }
        }
        if let Some(max) = max {
            let cur = class::get(d, c, class::MAX_TRIBUTE_REQ);
            if cur.is_nil() || (max as i64) > cur.int() {
                class::set(d, c, class::MAX_TRIBUTE_REQ, Value::Int(max as i64));
            }
        }
    }
    if ns {
        // (`AddNormalSetProcedure` passes no `opt` to its condition)
        effect::set_condition(d, e1, Some(summon_condition1(min, max, f.clone(), opt && !set)));
        effect::set_target(d, e1, Some(summon_target(min, max, f)));
        effect::set_operation(d, e1, Some(summon_operation(sumop)));
    } else {
        effect::set_condition(d, e1, Some(lua_fn(|_d, a| a.card(1).is_none())));
    }
    effect::set_value(d, e1, val as i64);
    card::register_effect(d, c, e1, false);
    e1
}
/// `aux.AddNormalSummonProcedure(c, ns, opt, min, max, val, desc, f, sumop)`
#[allow(clippy::too_many_arguments)]
pub fn add_normal_summon_procedure(d: &mut Duel, c: CardId, ns: bool, opt: bool, min: Option<u16>, max: Option<u16>, val: Option<u32>, desc: Option<u64>, f: Option<TributeFilter>, sumop: Option<SumOp>) -> EffectId {
    add_procedure(d, c, false, ns, opt, min, max, val, desc, f, sumop)
}
/// `aux.AddNormalSetProcedure(c, ns, opt, min, max, val, desc, f, sumop)`
#[allow(clippy::too_many_arguments)]
pub fn add_normal_set_procedure(d: &mut Duel, c: CardId, ns: bool, opt: bool, min: Option<u16>, max: Option<u16>, val: Option<u32>, desc: Option<u64>, f: Option<TributeFilter>, sumop: Option<SumOp>) -> EffectId {
    add_procedure(d, c, true, ns, opt, min, max, val, desc, f, sumop)
}
fn maplevel(level: u32) -> u16 {
    if (5..=6).contains(&level) {
        1
    } else if level >= 7 {
        2
    } else {
        0
    }
}
fn tribute_group(d: &mut Duel, c: CardId, relzone: u32, tp: u8, f: &Option<TributeFilter>) -> CardSet {
    let mut mg = duel::get_tribute_group(d, c);
    group::match_(d, &mut mg, &|d, x| super::is_zone(d, x, relzone, tp), NO_EX);
    if let Some(f) = f {
        group::match_(d, &mut mg, &|d, x| f(d, x, tp), NO_EX);
    }
    mg
}
fn number(v: &Value) -> Option<u32> {
    if v.is_nil() { None } else { Some(v.int() as u32) }
}
/// `aux.NormalSummonCondition1(min, max, f, opt)`: `function(e, c, minc, zone, relzone, exeff)`
fn summon_condition1(min: Option<u16>, max: Option<u16>, f: Option<TributeFilter>, opt: bool) -> LuaFn {
    lua_fn(move |d, a| {
        let Some(c) = a.card(1) else { return true };
        let tp = card::get_controler(d, c);
        let minc = a.int(2);
        let zone = number(a.v(3));
        let relzone = a.int(4) as u32;
        let mg = tribute_group(d, c, relzone, tp, &f);
        let tributes = maplevel(card::get_level(d, c));
        let min = min.unwrap_or_else(|| crate::lua_error("attempt to compare nil with number"));
        (!opt || (tributes > 0 && Some(tributes) != max)) && minc <= min as i64 && duel::check_tribute(d, c, min, max, Some(&mg), Some(tp), zone)
    })
}
/// `aux.NormalSummonTarget(min, max, f)`: `function(e,tp,eg,ep,ev,re,r,rp,chk,c,minc,zone,relzone,exeff)`
fn summon_target(min: Option<u16>, max: Option<u16>, f: Option<TributeFilter>) -> LuaFn {
    lua_fn(move |d, a| {
        let (e, tp) = (a.e(0), a.p(1));
        let c = a.c(9);
        let zone = number(a.v(11));
        let relzone = a.int(12) as u32;
        let mg = tribute_group(d, c, relzone, tp, &f);
        let cancel = duel::is_summon_cancelable(d);
        let min = min.unwrap_or_else(|| crate::lua_error("Parameter 3 should be \"Int\" but is \"nil\"."));
        let max = max.unwrap_or_else(|| crate::lua_error("Parameter 4 should be \"Int\" but is \"nil\"."));
        match duel::select_tribute(d, tp, c, min, max, Some(&mg), Some(tp), zone, cancel) {
            Some(g) if !g.is_empty() => {
                effect::set_label_object(d, e, Value::Group(g));
                true
            }
            _ => false,
        }
    })
}
/// `aux.NormalSummonOperation(min, max, sumop)`: `function(e,tp,eg,ep,ev,re,r,rp,c,minc,zone,relzone,exeff)`
fn summon_operation(sumop: Option<SumOp>) -> LuaFn {
    LuaFn::new(move |d, args| {
        let a = EffArgs::new(args);
        let c = a.extra.first().and_then(|v| v.as_card()).nil();
        let g = effect::get_label_object(d, a.e).as_group().cloned().unwrap_or_default();
        card::set_material(d, c, g.clone());
        duel::release(d, &g, REASON_SUMMON + REASON_MATERIAL, None);
        if let Some(op) = &sumop {
            op(d, &g, &a);
        }
        Value::Nil
    })
}
