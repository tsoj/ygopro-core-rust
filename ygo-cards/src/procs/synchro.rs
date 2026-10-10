//! `Synchro.*` (`proc_synchro.lua`): Synchro Summons. `Synchro.CheckP41`/`CheckP42` are memoized
//! as in `core_ext/lua/ygoai_overrides.lua` (the oracle runs that version).

use super::MatFilter;
use crate::aux;
use crate::constants::*;
use crate::lua::{card, duel, effect, group, NO_EX};
use crate::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

/// `SUMMON_TYPE_SYNCHRO|MATERIAL_SYNCHRO`
const ST: u64 = SUMMON_TYPE_SYNCHRO as u64 | MATERIAL_SYNCHRO;

/// `req1`/`req2`/`reqm`: `function(g, sc, tp)`
pub type ReqFn = Arc<dyn Fn(&mut Duel, &CardSet, CardId, u8) -> bool + Send + Sync>;
pub fn req_fn(f: impl Fn(&mut Duel, &CardSet, CardId, u8) -> bool + Send + Sync + 'static) -> ReqFn {
    Arc::new(f)
}

/// `Synchro.NonTuner(f, a, b, c)`: a non-Tuner matching `f(c)` (the arguments go into the closure)
pub fn non_tuner(f: Option<CardFilter>) -> MatFilter {
    Arc::new(move |d, c, sc, _st, tp| card::is_not_tuner(d, c, sc.nil(), tp) && f.as_ref().is_none_or(|f| f(d, c)))
}
/// `Synchro.NonTunerEx(f, val)` / `Synchro.NonTunerEx2(f, ...)`: a non-Tuner matching
/// `f(c, scard, sumtype, tp)`
pub fn non_tuner_ex(f: impl Fn(&mut Duel, CardId, Option<CardId>, u64, u8) -> bool + Send + Sync + 'static) -> MatFilter {
    Arc::new(move |d, c, sc, st, tp| card::is_not_tuner(d, c, sc.nil(), tp) && f(d, c, sc, st, tp))
}
/// `Synchro.NonTunerCode(...)`
pub fn non_tuner_code(codes: &'static [u32]) -> MatFilter {
    Arc::new(move |d, c, sc, st, tp| card::is_not_tuner(d, c, sc.nil(), tp) && card::is_summon_code(d, c, sc, st, tp, codes))
}

/// Script state (`Synchro.Send`, `Synchro.CheckAdditional`)
mod vars {
    use crate::ns::AUX;
    pub const SEND: (u32, u64) = (AUX, 300);
    pub const CHECK_ADDITIONAL: (u32, u64) = (AUX, 301);
}
/// `Synchro.Send = v`: where the next Synchro Summon sends its materials (1: to the GY, returned;
/// 2: banished; 3: banished face-down; 4: to the hand; 5: to the Deck; 6: destroyed)
pub fn set_send(d: &mut Duel, v: i64) {
    d.set_script_var(vars::SEND, Value::Int(v));
}
/// `Synchro.CheckAdditional = f` (`f(tp, sg, sc)`, `None`: nil)
pub fn set_check_additional(d: &mut Duel, f: Option<LuaFn>) {
    d.set_script_var(vars::CHECK_ADDITIONAL, f.map_or(Value::Nil, Value::Fn));
}

#[derive(Clone)]
struct Params {
    f1: Option<MatFilter>,
    min1: usize,
    max1: usize,
    f2: Option<MatFilter>,
    min2: usize,
    max2: usize,
    sub1: Option<MatFilter>,
    sub2: Option<MatFilter>,
    req1: Option<ReqFn>,
    req2: Option<ReqFn>,
    reqm: Option<ReqFn>,
}

/// `Synchro.AddProcedure(c, f1, min1, max1, f2, min2, max2, sub1, sub2, req1, req2, reqm)`:
/// `min1`..`max1` Tuners (`f1`) + `min2`..`max2` non-Tuners (`f2`)
#[allow(clippy::too_many_arguments)]
pub fn add_procedure(
    d: &mut Duel,
    c: CardId,
    f1: Option<MatFilter>,
    min1: usize,
    max1: usize,
    f2: Option<MatFilter>,
    min2: usize,
    max2: usize,
    sub1: Option<MatFilter>,
    sub2: Option<MatFilter>,
    req1: Option<ReqFn>,
    req2: Option<ReqFn>,
    reqm: Option<ReqFn>,
) -> EffectId {
    let p = Params { f1, min1, max1, f2, min2, max2, sub1, sub2, req1, req2, reqm };
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_description(d, e1, 1172);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_range(d, e1, LOCATION_EXTRA);
    effect::set_condition(d, e1, Some(condition_fn(p.clone())));
    effect::set_target(d, e1, Some(target_fn(p)));
    effect::set_operation(d, e1, Some(operation_fn()));
    effect::set_value(d, e1, SUMMON_TYPE_SYNCHRO as i64);
    card::register_effect(d, c, e1, false);
    e1
}

/// The filters of one condition/target call (`sub1` with "can be treated as a Tuner").
struct Filters<'a> {
    p: &'a Params,
    sc: CardId,
    tp: u8,
}
impl Filters<'_> {
    /// `sub1` as the procedure extends it: `aux.OR(sub1, function(_c) return _c:IsHasEffect(EFFECT_CAN_BE_TUNER) and (not f1 or f1(_c,c,...)) end)`
    fn sub1(&self, d: &mut Duel, c: CardId) -> bool {
        if self.p.sub1.as_ref().is_some_and(|s| s(d, c, Some(self.sc), ST, self.tp)) {
            return true;
        }
        !card::is_has_effect(d, c, EFFECT_CAN_BE_TUNER, None).is_empty() && self.p.f1.as_ref().is_none_or(|f| f(d, c, Some(self.sc), ST, self.tp))
    }
    fn f1(&self, d: &mut Duel, c: CardId) -> bool {
        self.p.f1.as_ref().is_none_or(|f| f(d, c, Some(self.sc), ST, self.tp))
    }
    /// `Synchro.TunerFilter(c, f1, sub1, sc, tp)`
    fn tuner(&self, d: &mut Duel, c: CardId) -> bool {
        (card::is_type_ctx(d, c, TYPE_TUNER, Some(self.sc), ST, Some(self.tp)) && self.f1(d, c)) || self.sub1(d, c)
    }
    /// `Synchro.NonTunerFilter(c, f2, sub2, sc, tp)`
    fn non_tuner(&self, d: &mut Duel, c: CardId) -> bool {
        self.p.f2.as_ref().is_none_or(|f| f(d, c, Some(self.sc), ST, self.tp)) || self.p.sub2.as_ref().is_some_and(|s| s(d, c, Some(self.sc), ST, self.tp))
    }
    /// `Synchro.CheckFilterChk(c, f1, f2, sub1, sub2, sc, tp)`: a monster with EFFECT_SYNCHRO_CHECK
    /// ("Genomix Fighter") that could be a material
    fn check_filter_chk(&self, d: &mut Duel, c: CardId) -> bool {
        let Some(&te) = card::get_card_effect(d, c, EFFECT_SYNCHRO_CHECK).first() else { return false };
        let reset = call_value(d, te, &[Value::Effect(te), Value::Card(c)]).truthy();
        let res = (card::is_type_ctx(d, c, TYPE_TUNER, Some(self.sc), ST, Some(self.tp)) && self.f1(d, c))
            || self.p.f2.as_ref().is_none_or(|f| f(d, c, Some(self.sc), ST, self.tp))
            || self.sub1(d, c)
            || self.p.sub2.as_ref().is_some_and(|s| s(d, c, Some(self.sc), ST, self.tp));
        if reset {
            duel::assume_reset(d);
        }
        res
    }
    fn req(&self, d: &mut Duel, r: &Option<ReqFn>, g: &CardSet) -> bool {
        r.as_ref().is_none_or(|r| r(d, g, self.sc, self.tp))
    }
}
/// `te:GetValue()(...)` (a number value is not callable)
fn call_value(d: &mut Duel, te: EffectId, args: &[Value]) -> Value {
    match effect::get_value(d, te) {
        Value::Fn(f) => f.call(d, args),
        _ => crate::lua_error("attempt to call a number value (local 'val')"),
    }
}
/// `te:GetTarget()(...)` (nil is not callable)
fn call_target(d: &mut Duel, te: EffectId, args: &[Value]) -> Value {
    match effect::get_target(d, te) {
        Some(f) => f.call(d, args),
        None => crate::lua_error("attempt to call a nil value (local 'tgchk')"),
    }
}
fn multi(v: &Value, i: usize) -> Value {
    match v {
        Value::Multi(m) => m.get(i).cloned().unwrap_or(Value::Nil),
        v if i == 0 => v.clone(),
        _ => Value::Nil,
    }
}
fn group_of(v: &Value) -> CardSet {
    match v {
        Value::Group(g) => g.clone(),
        _ => crate::lua_error("Parameter 2 should be \"Card\" or \"Group\""),
    }
}
/// `aux.HarmonizingMagFilter(c, f, f:GetValue())`
fn harmonizing(d: &mut Duel, c: CardId, f: EffectId) -> bool {
    match effect::get_value(d, f) {
        Value::Fn(v) => !v.call(d, &[Value::Effect(f), Value::Card(c)]).truthy(),
        _ => crate::lua_error("attempt to call a number value (local 'f')"),
    }
}
/// The material restrictions of `c` and of the chosen materials `sg` ("Harmonizing Magician"):
/// `None` if `c` cannot be added, else the cards of `pools` (each) `c` excludes.
fn restrictions(d: &mut Duel, c: CardId, sg: &CardSet, pools: &[&CardSet]) -> Option<Vec<CardSet>> {
    let mut out = vec![CardSet::new(); pools.len()];
    for f in card::get_card_effect(d, c, EFFECT_SYNCHRO_MAT_RESTRICTION) {
        if group::is_exists(d, sg, &|d, x| harmonizing(d, x, f), 1, c.into()) {
            return None;
        }
        for (i, pool) in pools.iter().enumerate() {
            let s = group::filter(d, pool, &|d, x| harmonizing(d, x, f), NO_EX);
            group::add_card(&mut out[i], &s);
        }
    }
    for tc in group::iter(d, sg) {
        for f in card::get_card_effect(d, tc, EFFECT_SYNCHRO_MAT_RESTRICTION) {
            if harmonizing(d, c, f) {
                return None;
            }
        }
    }
    Some(out)
}
/// The hand-Synchro checks (`EFFECT_HAND_SYNCHRO+EFFECT_SYNCHRO_CHECK` targets) when adding `c`:
/// `None` if they fail, else the (Tuner, non-Tuner) groups `c`'s effect excludes.
fn hand_checks(d: &mut Duel, c: CardId, sg: &CardSet, a: &CardSet, b: &CardSet, tsg: &CardSet, ntsg: &CardSet) -> Option<(Option<Value>, Option<Value>)> {
    let mut hanchk = true;
    let mut got = (None, None);
    for te in card::get_card_effect(d, c, EFFECT_HAND_SYNCHRO + EFFECT_SYNCHRO_CHECK) {
        let r = call_target(d, te, &[Value::Effect(te), Value::Card(c), Value::Group(sg.clone()), Value::Group(a.clone()), Value::Group(b.clone()), Value::Group(tsg.clone()), Value::Group(ntsg.clone())]);
        hanchk = multi(&r, 0).truthy();
        if hanchk {
            got = (Some(multi(&r, 1)), Some(multi(&r, 2)));
            break;
        }
    }
    if !hanchk {
        return None;
    }
    for tc in group::iter(d, sg) {
        let mut hanchk = true;
        for te in card::get_card_effect(d, tc, EFFECT_HAND_SYNCHRO + EFFECT_SYNCHRO_CHECK) {
            hanchk = multi(&call_target(d, te, &[Value::Effect(te), Value::Nil, Value::Group(sg.clone()), Value::Group(a.clone()), Value::Group(b.clone()), Value::Group(tsg.clone()), Value::Group(ntsg.clone())]), 0).truthy();
            if hanchk {
                break;
            }
        }
        if !hanchk {
            return None;
        }
    }
    Some(got)
}

/// What the material checks share.
struct Ctx<'a> {
    fl: Filters<'a>,
    lv: u32,
    pg: &'a CardSet,
    mgchk: bool,
    min: Option<usize>,
    max: Option<usize>,
}
fn check_additional(d: &mut Duel, tp: u8, sg: &CardSet, sc: CardId) -> bool {
    match d.script_var(vars::CHECK_ADDITIONAL) {
        Value::Fn(f) => f.call(d, &[Value::Int(tp as i64), Value::Group(sg.clone()), Value::Card(sc)]).truthy(),
        _ => true,
    }
}
/// `Synchro.CheckLabel(c, label)`
fn check_label(d: &mut Duel, c: CardId, label: i64) -> bool {
    card::get_card_effect(d, c, EFFECT_HAND_SYNCHRO).first().is_some_and(|&e| effect::get_label(d, e) == label)
}
/// `Synchro.CheckHand(c, sg)`
fn check_hand(d: &mut Duel, c: CardId, sg: &CardSet) -> bool {
    let teg = card::get_card_effect(d, c, EFFECT_HAND_SYNCHRO + EFFECT_SYNCHRO_CHECK);
    if teg.is_empty() {
        return false;
    }
    for te in teg {
        let label = effect::get_label(d, te);
        if group::is_exists(d, sg, &|d, x| check_label(d, x, label), 1, c.into()) {
            return false;
        }
    }
    true
}
/// `Synchro.CheckP43(tsg, ntsg, sg, lv, sc, tp)`: the chosen materials work (Levels, zones)
fn check_p43(d: &mut Duel, tsg: &CardSet, ntsg: &CardSet, sg: &CardSet, lv: u32, sc: CardId, tp: u8) -> bool {
    if group::is_exists(d, sg, &|d, x| check_hand(d, x, sg), 1, NO_EX) {
        return false;
    }
    let mut lvchk = false;
    for tc in group::iter(d, sg) {
        for te in card::get_card_effect(d, tc, EFFECT_SYNCHRO_MATERIAL_CUSTOM) {
            let Some(op) = effect::get_operation(d, te) else { crate::lua_error("attempt to call a nil value (local 'op')") };
            let r = op.call(d, &[Value::Effect(te), Value::Group(tsg.clone()), Value::Group(ntsg.clone()), Value::Group(sg.clone()), Value::Int(lv as i64), Value::Card(sc), Value::Int(tp as i64)]);
            if !multi(&r, 0).truthy() {
                return false;
            }
            lvchk = lvchk || multi(&r, 1).truthy();
        }
    }
    let n = sg.len() as i32;
    check_additional(d, tp, sg, sc)
        && (lvchk || group::check_with_sum_equal(d, sg, &|d, x| card::get_synchro_level(d, x, sc) as i64, lv, n, n).0)
        && if card::is_location(d, sc, LOCATION_EXTRA) {
            duel::get_location_count_from_ex(d, tp, Some(tp), sg.into(), duel::ExtraCard::Card(sc), None).0 > 0
        } else {
            duel::get_m_zone_count(d, tp, sg.into(), Some(tp), None, None).0 > 0
        }
}
fn reset_check_assume(d: &mut Duel, sg: &CardSet) {
    if !group::is_exists(d, sg, &|d, x| !card::is_has_effect(d, x, EFFECT_SYNCHRO_CHECK, None).is_empty(), 1, NO_EX) {
        duel::assume_reset(d);
    }
}
/// The end condition with the non-Tuners chosen (`P32`/`P42`/the target's "finish").
fn goal(d: &mut Duel, x: &Ctx, tsg: &CardSet, ntsg: &CardSet, sg: &CardSet, with_nt_filter: bool) -> bool {
    let (t, n) = (tsg.len(), ntsg.len());
    x.min.is_none_or(|m| t + n >= m)
        && x.fl.req(d, &x.fl.p.req2, ntsg)
        && x.fl.req(d, &x.fl.p.reqm, sg)
        && (!with_nt_filter || group::is_exists(d, ntsg, &|d, c| x.fl.non_tuner(d, c), n as u16, NO_EX))
        && group::includes(sg, x.pg)
        && check_p43(d, tsg, ntsg, sg, x.lv, x.fl.sc, x.fl.tp)
}

/// `g:IsExists(Synchro.CheckP31, 1, sg, ...)`: the candidates are fixed when it starts
fn exists_p31(d: &mut Duel, x: &Ctx, g: &mut CardSet, tsg: &mut CardSet, ntsg: &mut CardSet, sg: &mut CardSet) -> bool {
    let cands = group::iter(d, &group::difference(g, sg));
    for c in cands {
        if crate::lua::protected(d, false, |d| check_p31(d, x, c, g, tsg, ntsg, sg)) {
            return true;
        }
    }
    false
}
fn exists_p32(d: &mut Duel, x: &Ctx, g: &mut CardSet, tsg: &mut CardSet, ntsg: &mut CardSet, sg: &mut CardSet) -> bool {
    let cands = group::iter(d, &group::difference(g, sg));
    for c in cands {
        if crate::lua::protected(d, false, |d| check_p32(d, x, c, g, tsg, ntsg, sg)) {
            return true;
        }
    }
    false
}
fn tuners_ok(d: &mut Duel, x: &Ctx, tsg: &CardSet) -> bool {
    group::is_exists(d, tsg, &|d, c| x.fl.tuner(d, c), tsg.len() as u16, NO_EX) && x.fl.req(d, &x.fl.p.req1, tsg)
}
/// `Synchro.CheckP31`: adding the Tuner `c` (with EFFECT_SYNCHRO_CHECK monsters around)
fn check_p31(d: &mut Duel, x: &Ctx, c: CardId, g: &mut CardSet, tsg: &mut CardSet, ntsg: &mut CardSet, sg: &mut CardSet) -> bool {
    let mut rg = CardSet::new();
    for te in card::get_card_effect(d, c, EFFECT_SYNCHRO_CHECK) {
        let tg = group::filter(d, g, &|d, mc| call_value(d, te, &[Value::Effect(te), Value::Card(mc)]).truthy(), NO_EX);
        rg = group::filter(d, &tg, &|d, mc| !x.fl.tuner(d, mc) && !x.fl.non_tuner(d, mc), NO_EX);
    }
    let Some(r) = restrictions(d, c, sg, &[g]) else { return false };
    group::add_card(&mut rg, &r[0]);
    if !x.mgchk {
        let Some((trg, _)) = hand_checks(d, c, sg, &g.clone(), &g.clone(), tsg, ntsg) else { return false };
        if let Some(trg) = trg {
            group::add_card(&mut rg, &group_of(&trg));
        }
    }
    group::remove_card(g, &rg);
    tsg.insert(c);
    sg.insert(c);
    let tsg_count = tsg.len();
    let p = x.fl.p;
    let res = if x.max.is_some_and(|m| tsg_count > m || (m as i64 - tsg_count as i64) < p.min2 as i64) {
        false
    } else if x.max.is_some_and(|m| m - tsg_count == p.min2) {
        tuners_ok(d, x, tsg) && exists_p32(d, x, g, tsg, ntsg, sg)
    } else if tsg_count < p.min1 {
        exists_p31(d, x, g, tsg, ntsg, sg)
    } else if tsg_count < p.max1 {
        exists_p31(d, x, g, tsg, ntsg, sg) || (tuners_ok(d, x, tsg) && exists_p32(d, x, g, tsg, ntsg, sg))
    } else {
        tuners_ok(d, x, tsg) && exists_p32(d, x, g, tsg, ntsg, sg)
    };
    group::add_card(g, &rg);
    tsg.remove(c);
    sg.remove(c);
    reset_check_assume(d, sg);
    res
}
/// `Synchro.CheckP32`: adding the non-Tuner `c` (with EFFECT_SYNCHRO_CHECK monsters around)
fn check_p32(d: &mut Duel, x: &Ctx, c: CardId, g: &mut CardSet, tsg: &mut CardSet, ntsg: &mut CardSet, sg: &mut CardSet) -> bool {
    let mut rg = CardSet::new();
    for te in card::get_card_effect(d, c, EFFECT_SYNCHRO_CHECK) {
        let tg = group::filter(d, g, &|d, mc| call_value(d, te, &[Value::Effect(te), Value::Card(mc)]).truthy(), NO_EX);
        rg = group::filter(d, &tg, &|d, mc| !x.fl.non_tuner(d, mc), NO_EX);
    }
    let Some(r) = restrictions(d, c, sg, &[g]) else { return false };
    group::add_card(&mut rg, &r[0]);
    if !x.mgchk {
        let Some((_, ntrg)) = hand_checks(d, c, sg, &CardSet::new(), &g.clone(), tsg, ntsg) else { return false };
        if let Some(ntrg) = ntrg {
            group::add_card(&mut rg, &group_of(&ntrg));
        }
    }
    group::remove_card(g, &rg);
    ntsg.insert(c);
    sg.insert(c);
    let (t, n) = (tsg.len(), ntsg.len());
    let p = x.fl.p;
    let res = if x.max.is_some_and(|m| t + n > m) {
        false
    } else if n < p.min2 {
        exists_p32(d, x, g, tsg, ntsg, sg)
    } else if n < p.max2 {
        exists_p32(d, x, g, tsg, ntsg, sg) || goal(d, x, tsg, ntsg, sg, true)
    } else {
        goal(d, x, tsg, ntsg, sg, true)
    };
    group::add_card(g, &rg);
    ntsg.remove(c);
    sg.remove(c);
    reset_check_assume(d, sg);
    res
}

/// The memo of `CheckP41`/`CheckP42` for one top-level call (`ygoai_overrides.lua`).
type Memo = HashMap<(u8, CardId, Vec<Vec<CardId>>), bool>;
fn key(name: u8, c: CardId, groups: &[&CardSet]) -> (u8, CardId, Vec<Vec<CardId>>) {
    (name, c, groups.iter().map(|g| g.to_vec()).collect())
}
fn exists_p41(d: &mut Duel, x: &Ctx, memo: &mut Memo, tg: &mut CardSet, ntg: &mut CardSet, tsg: &mut CardSet, ntsg: &mut CardSet, sg: &mut CardSet) -> bool {
    let cands = group::iter(d, &group::difference(tg, sg));
    for c in cands {
        if crate::lua::protected(d, false, |d| check_p41(d, x, memo, c, tg, ntg, tsg, ntsg, sg)) {
            return true;
        }
    }
    false
}
fn exists_p42(d: &mut Duel, x: &Ctx, memo: &mut Memo, ntg: &mut CardSet, tsg: &mut CardSet, ntsg: &mut CardSet, sg: &mut CardSet) -> bool {
    let cands = group::iter(d, &group::difference(ntg, sg));
    for c in cands {
        if crate::lua::protected(d, false, |d| check_p42(d, x, memo, c, ntg, tsg, ntsg, sg)) {
            return true;
        }
    }
    false
}
/// `Synchro.CheckP41` (memoized): adding the Tuner `c`
#[allow(clippy::too_many_arguments)]
fn check_p41(d: &mut Duel, x: &Ctx, memo: &mut Memo, c: CardId, tg: &mut CardSet, ntg: &mut CardSet, tsg: &mut CardSet, ntsg: &mut CardSet, sg: &mut CardSet) -> bool {
    let k = key(b't', c, &[tg, ntg, tsg, ntsg, sg]);
    if let Some(&r) = memo.get(&k) {
        return r;
    }
    let r = check_p41_body(d, x, memo, c, tg, ntg, tsg, ntsg, sg);
    memo.insert(k, r);
    r
}
#[allow(clippy::too_many_arguments)]
fn check_p41_body(d: &mut Duel, x: &Ctx, memo: &mut Memo, c: CardId, tg: &mut CardSet, ntg: &mut CardSet, tsg: &mut CardSet, ntsg: &mut CardSet, sg: &mut CardSet) -> bool {
    let Some(r) = restrictions(d, c, sg, &[tg, ntg]) else { return false };
    let (mut trg, mut ntrg) = (r[0].clone(), r[1].clone());
    if !x.mgchk {
        let Some((trg2, ntrg2)) = hand_checks(d, c, sg, &tg.clone(), &ntg.clone(), tsg, ntsg) else { return false };
        if let Some(t) = trg2 {
            group::add_card(&mut trg, &group_of(&t));
            group::add_card(&mut ntrg, &group_of(&ntrg2.unwrap_or(Value::Nil)));
        }
    }
    group::remove_card(tg, &trg);
    group::remove_card(ntg, &ntrg);
    tsg.insert(c);
    sg.insert(c);
    let tsg_count = tsg.len();
    let p = x.fl.p;
    let res = if x.max.is_some_and(|m| tsg_count > m || (m as i64 - tsg_count as i64) < p.min2 as i64) {
        false
    } else if x.max.is_some_and(|m| m - tsg_count == p.min2) {
        x.fl.req(d, &p.req1, tsg) && exists_p42(d, x, memo, ntg, tsg, ntsg, sg)
    } else if tsg_count < p.min1 {
        exists_p41(d, x, memo, tg, ntg, tsg, ntsg, sg)
    } else if tsg_count < p.max1 {
        exists_p41(d, x, memo, tg, ntg, tsg, ntsg, sg) || (x.fl.req(d, &p.req1, tsg) && exists_p42(d, x, memo, ntg, tsg, ntsg, sg))
    } else {
        x.fl.req(d, &p.req1, tsg) && exists_p42(d, x, memo, ntg, tsg, ntsg, sg)
    };
    group::add_card(tg, &trg);
    group::add_card(ntg, &ntrg);
    tsg.remove(c);
    sg.remove(c);
    res
}
/// `Synchro.CheckP42` (memoized): adding the non-Tuner `c`
fn check_p42(d: &mut Duel, x: &Ctx, memo: &mut Memo, c: CardId, ntg: &mut CardSet, tsg: &mut CardSet, ntsg: &mut CardSet, sg: &mut CardSet) -> bool {
    let k = key(b'n', c, &[ntg, tsg, ntsg, sg]);
    if let Some(&r) = memo.get(&k) {
        return r;
    }
    let r = check_p42_body(d, x, memo, c, ntg, tsg, ntsg, sg);
    memo.insert(k, r);
    r
}
fn check_p42_body(d: &mut Duel, x: &Ctx, memo: &mut Memo, c: CardId, ntg: &mut CardSet, tsg: &mut CardSet, ntsg: &mut CardSet, sg: &mut CardSet) -> bool {
    let Some(r) = restrictions(d, c, sg, &[ntg]) else { return false };
    let mut ntrg = r[0].clone();
    if !x.mgchk {
        let Some((_, ntrg2)) = hand_checks(d, c, sg, &CardSet::new(), &ntg.clone(), tsg, ntsg) else { return false };
        if let Some(n) = ntrg2 {
            group::add_card(&mut ntrg, &group_of(&n));
        }
    }
    group::remove_card(ntg, &ntrg);
    ntsg.insert(c);
    sg.insert(c);
    let (t, n) = (tsg.len(), ntsg.len());
    let p = x.fl.p;
    let res = if x.max.is_some_and(|m| t + n > m) {
        false
    } else if n < p.min2 {
        exists_p42(d, x, memo, ntg, tsg, ntsg, sg)
    } else if n < p.max2 {
        exists_p42(d, x, memo, ntg, tsg, ntsg, sg) || goal(d, x, tsg, ntsg, sg, false)
    } else {
        goal(d, x, tsg, ntsg, sg, false)
    };
    group::add_card(ntg, &ntrg);
    ntsg.remove(c);
    sg.remove(c);
    res
}
/// `tg:Filter(Synchro.CheckP41, sg, ...)` (each candidate is a top-level call with its own memo)
fn filter_p41(d: &mut Duel, x: &Ctx, tg: &mut CardSet, ntg: &mut CardSet, tsg: &mut CardSet, ntsg: &mut CardSet, sg: &mut CardSet) -> CardSet {
    let mut out = CardSet::new();
    for c in group::iter(d, &group::difference(tg, sg)) {
        if crate::lua::protected(d, false, |d| check_p41(d, x, &mut Memo::new(), c, tg, ntg, tsg, ntsg, sg)) {
            out.insert(c);
        }
    }
    out
}
fn filter_p42(d: &mut Duel, x: &Ctx, ntg: &mut CardSet, tsg: &mut CardSet, ntsg: &mut CardSet, sg: &mut CardSet) -> CardSet {
    let mut out = CardSet::new();
    for c in group::iter(d, &group::difference(ntg, sg)) {
        if crate::lua::protected(d, false, |d| check_p42(d, x, &mut Memo::new(), c, ntg, tsg, ntsg, sg)) {
            out.insert(c);
        }
    }
    out
}
fn filter_p31(d: &mut Duel, x: &Ctx, g: &mut CardSet, tsg: &mut CardSet, ntsg: &mut CardSet, sg: &mut CardSet) -> CardSet {
    let mut out = CardSet::new();
    for c in group::iter(d, &group::difference(g, sg)) {
        if crate::lua::protected(d, false, |d| check_p31(d, x, c, g, tsg, ntsg, sg)) {
            out.insert(c);
        }
    }
    out
}
fn filter_p32(d: &mut Duel, x: &Ctx, g: &mut CardSet, tsg: &mut CardSet, ntsg: &mut CardSet, sg: &mut CardSet) -> CardSet {
    let mut out = CardSet::new();
    for c in group::iter(d, &group::difference(g, sg)) {
        if crate::lua::protected(d, false, |d| check_p32(d, x, c, g, tsg, ntsg, sg)) {
            out.insert(c);
        }
    }
    out
}

/// The materials both the condition and the target start with: (dg, g, mgchk).
fn materials(d: &mut Duel, c: CardId, tp: u8, mg: Option<&CardSet>) -> (CardSet, CardSet, bool) {
    let (dg, mut g, mgchk) = match mg {
        Some(mg) => {
            let g = group::filter(d, mg, &|d, x| card::is_can_be_synchro_material(d, x, Some(c), None, None), c.into());
            (mg.clone(), g, true)
        }
        None => {
            let synchmatfilter = |d: &mut Duel, mc: CardId| {
                let handmatfilter = card::is_has_effect(d, mc, EFFECT_SYNCHRO_MAT_FROM_HAND, None).first().copied();
                (card::is_location(d, mc, LOCATION_MZONE) && card::is_faceup(d, mc) && (card::is_controler(d, mc, tp) || card::is_can_be_synchro_material(d, mc, Some(c), None, None)))
                    || handmatfilter.is_some_and(|h| effect::check_count_limit(d, h, tp) && call_value(d, h, &[Value::Effect(h), Value::Card(mc), Value::Card(c)]).truthy())
            };
            let dg = duel::get_matching_group(d, Some(&synchmatfilter), tp, LOCATION_MZONE | LOCATION_HAND, LOCATION_MZONE, c.into());
            let g = group::filter(d, &dg, &|d, x| card::is_can_be_synchro_material(d, x, Some(c), None, None), NO_EX);
            (dg, g, false)
        }
    };
    group::remove(d, &mut g, &|d, x| card::get_synchro_level(d, x, c) < 1, NO_EX);
    (dg, g, mgchk)
}
/// The Tuner and non-Tuner candidates (with the hand-Synchro additions): (tg, ntg).
fn tuner_groups(d: &mut Duel, fl: &Filters, g: &CardSet, mgchk: bool) -> (CardSet, CardSet) {
    let mut tg = group::filter(d, g, &|d, x| fl.tuner(d, x), NO_EX);
    let mut ntg = group::filter(d, g, &|d, x| fl.non_tuner(d, x), NO_EX);
    if !mgchk {
        let mut thg = group::filter(d, &tg, &|d, x| !card::is_has_effect(d, x, EFFECT_HAND_SYNCHRO, None).is_empty(), NO_EX);
        let more = group::filter(d, &ntg, &|d, x| !card::is_has_effect(d, x, EFFECT_HAND_SYNCHRO, None).is_empty(), NO_EX);
        group::add_card(&mut thg, &more);
        let sc = fl.sc;
        let hg = duel::get_matching_group(d, Some(&|d, x| card::is_can_be_synchro_material(d, x, Some(sc), None, None)), fl.tp, LOCATION_HAND + LOCATION_GRAVE, 0, sc.into());
        for thc in group::iter(d, &thg) {
            let te = *card::get_card_effect(d, thc, EFFECT_HAND_SYNCHRO).first().nil();
            let thag = group::filter(d, &hg, &|d, mc| fl.tuner(d, mc) && call_value(d, te, &[Value::Effect(te), Value::Card(mc), Value::Card(sc)]).truthy(), NO_EX);
            let nthag = group::filter(d, &hg, &|d, mc| fl.non_tuner(d, mc) && call_value(d, te, &[Value::Effect(te), Value::Card(mc), Value::Card(sc)]).truthy(), NO_EX);
            group::add_card(&mut tg, &thag);
            group::add_card(&mut ntg, &nthag);
        }
    }
    (tg, ntg)
}
fn reset_hand_synchro(d: &mut Duel, tp: u8) {
    let hg = duel::get_matching_group(d, Some(&|d, x| !card::is_has_effect(d, x, EFFECT_HAND_SYNCHRO + EFFECT_SYNCHRO_CHECK, None).is_empty()), tp, LOCATION_HAND + LOCATION_GRAVE, 0, NO_EX);
    aux::reset_effects(d, &hg, EFFECT_HAND_SYNCHRO + EFFECT_SYNCHRO_CHECK);
}
fn opt_count(v: &Value) -> Option<usize> {
    if v.is_nil() { None } else { Some(v.int() as usize) }
}

/// `Synchro.Condition(...)`: `function(e, c, smat, mg, min, max)`
fn condition_fn(p: Params) -> LuaFn {
    lua_fn(move |d, a| {
        let Some(c) = a.card(1) else { return true };
        if card::is_type(d, c, TYPE_PENDULUM) && card::is_faceup(d, c) {
            return false;
        }
        let tp = card::get_controler(d, c);
        let smat = a.v(2).as_group().cloned();
        let mg = a.v(3).as_group().cloned();
        let (min, max) = (opt_count(a.v(4)), opt_count(a.v(5)));
        let lv = card::get_level(d, c);
        let fl = Filters { p: &p, sc: c, tp };
        let (dg, mut g, mgchk) = materials(d, c, tp, mg.as_ref());
        let mut pg = aux::get_must_be_material_group(d, tp, &dg, tp, Some(c), &g, REASON_SYNCHRO);
        if !group::includes(&g, &pg) || group::is_exists(d, &pg, &|d, x| !card::is_can_be_synchro_material(d, x, Some(c), None, None), 1, NO_EX) {
            return false;
        }
        if let Some(smat) = &smat {
            if group::is_exists(d, smat, &|d, x| !card::is_can_be_synchro_material(d, x, Some(c), None, None), 1, NO_EX) {
                return false;
            }
            group::add_card(&mut pg, smat);
            group::add_card(&mut g, smat);
        }
        if group::is_exists(d, &g, &|d, x| fl.check_filter_chk(d, x), 1, NO_EX) {
            // (a monster with EFFECT_SYNCHRO_CHECK: Genomix Fighter/Mono Synchron)
            let mut g2 = g.clone();
            if !mgchk {
                let hg = duel::get_matching_group(d, Some(&|d, x| card::is_can_be_synchro_material(d, x, Some(c), None, None)), tp, LOCATION_HAND + LOCATION_GRAVE, 0, c.into());
                for thc in group::iter(d, &g) {
                    if let Some(&te) = card::get_card_effect(d, thc, EFFECT_HAND_SYNCHRO).first() {
                        let ag = group::filter(d, &hg, &|d, mc| call_value(d, te, &[Value::Effect(te), Value::Card(mc), Value::Card(c)]).truthy(), NO_EX);
                        group::add_card(&mut g2, &ag);
                    }
                }
            }
            let x = Ctx { fl, lv, pg: &pg, mgchk, min, max };
            let mut g2m = g2.clone();
            let res = {
                let cands = group::iter(d, &g2);
                let mut found = false;
                for tc in cands {
                    if crate::lua::protected(d, false, |d| check_p31(d, &x, tc, &mut g2m, &mut CardSet::new(), &mut CardSet::new(), &mut CardSet::new())) {
                        found = true;
                        break;
                    }
                }
                found
            };
            reset_hand_synchro(d, tp);
            duel::assume_reset(d);
            res
        } else {
            // (no race change)
            let (mut tg, mut ntg) = tuner_groups(d, &fl, &g, mgchk);
            let lv = card::get_level(d, c);
            let x = Ctx { fl, lv, pg: &pg, mgchk, min, max };
            let cands = group::iter(d, &tg);
            let mut res = false;
            for tc in cands {
                if crate::lua::protected(d, false, |d| check_p41(d, &x, &mut Memo::new(), tc, &mut tg, &mut ntg, &mut CardSet::new(), &mut CardSet::new(), &mut CardSet::new())) {
                    res = true;
                    break;
                }
            }
            reset_hand_synchro(d, tp);
            res
        }
    })
}
/// `Synchro.Target(...)`: `function(e,tp,eg,ep,ev,re,r,rp,chk,c,smat,mg,min,max)`
fn target_fn(p: Params) -> LuaFn {
    lua_fn(move |d, a| {
        let (e, tp) = (a.e(0), a.p(1));
        let c = a.c(9);
        let smat = a.v(10).as_group().cloned();
        let mg = a.v(11).as_group().cloned();
        let (min, max) = (opt_count(a.v(12)), opt_count(a.v(13)));
        let mut sg = CardSet::new();
        let fl = Filters { p: &p, sc: c, tp };
        let (dg, mut g, mgchk) = materials(d, c, tp, mg.as_ref());
        let mut pg = aux::get_must_be_material_group(d, tp, &dg, tp, Some(c), &g, REASON_SYNCHRO);
        if let Some(smat) = &smat {
            group::add_card(&mut pg, smat);
            group::add_card(&mut g, smat);
        }
        let (mut tg, mut ntg) = tuner_groups(d, &fl, &g, mgchk);
        let lv = card::get_level(d, c);
        let mut tsg = CardSet::new();
        let mut selectedastuner = CardSet::new();
        let x = Ctx { fl, lv, pg: &pg, mgchk, min, max };
        let p = &p;
        if group::is_exists(d, &g, &|d, cc| x.fl.check_filter_chk(d, cc), 1, NO_EX) {
            let mut ntsg = CardSet::new();
            let mut tune = true;
            while ntsg.len() < p.max2 {
                if tune {
                    let cancel = !mgchk && duel::is_summon_cancelable(d) && tsg.is_empty();
                    let mut ntg2 = ntg.clone();
                    let g3 = {
                        // (`ntg:Filter(Synchro.CheckP32, sg, g, ...)`: the candidates are ntg's, the checks use g)
                        let mut out = CardSet::new();
                        for cc in group::iter(d, &group::difference(&ntg2, &sg)) {
                            if crate::lua::protected(d, false, |d| check_p32(d, &x, cc, &mut g, &mut tsg, &mut ntsg, &mut sg)) {
                                out.insert(cc);
                            }
                        }
                        ntg2 = ntg.clone();
                        out
                    };
                    let _ = ntg2;
                    let mut g2 = filter_p31(d, &x, &mut g, &mut tsg, &mut ntsg, &mut sg);
                    if !g3.is_empty() && tsg.len() >= p.min1 && tuners_ok(d, &x, &tsg) {
                        group::add_card(&mut g2, &g3);
                    }
                    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SMATERIAL as u64);
                    let tc = group::select_unselect(d, &g2, Some(&sg), tp, false, cancel, None, None);
                    let Some(tc) = tc else {
                        let ok = tsg.len() >= p.min1 && tuners_ok(d, &x, &tsg) && {
                            let mut out = 0;
                            for cc in group::iter(d, &group::difference(&ntg, &sg)) {
                                if crate::lua::protected(d, false, |d| check_p32(d, &x, cc, &mut g, &mut tsg, &mut ntsg, &mut sg)) {
                                    out += 1;
                                }
                            }
                            out > 0
                        };
                        if ok {
                            // (`tune=false`, then `sg:IsContains(nil)` is false and `tsg:AddCard(nil)` an error)
                            crate::lua_error("Parameter 2 should be \"Card\" or \"Group\"");
                        }
                        return false;
                    };
                    if !sg.contains(tc) {
                        if g3.contains(tc) {
                            ntsg.insert(tc);
                            tune = false;
                        } else {
                            tsg.insert(tc);
                        }
                        selectedastuner.insert(tc);
                        sg.insert(tc);
                        for te in card::get_card_effect(d, tc, EFFECT_SYNCHRO_CHECK) {
                            for mc in group::iter(d, &g) {
                                call_value(d, te, &[Value::Effect(te), Value::Card(mc)]);
                            }
                        }
                    } else {
                        selectedastuner.remove(tc);
                        tsg.remove(tc);
                        sg.remove(tc);
                        reset_check_assume(d, &sg);
                    }
                    if filter_p31(d, &x, &mut g, &mut tsg, &mut ntsg, &mut sg).is_empty() || tsg.len() >= p.max1 {
                        tune = false;
                    }
                } else {
                    let finish = ntsg.len() >= p.min2
                        && x.fl.req(d, &p.req2, &ntsg)
                        && x.fl.req(d, &p.reqm, &sg)
                        && group::is_exists(d, &ntsg, &|d, cc| x.fl.non_tuner(d, cc), ntsg.len() as u16, NO_EX)
                        && group::includes(&sg, &pg)
                        && check_p43(d, &tsg, &ntsg, &sg, lv, c, tp);
                    let cancel = !mgchk && duel::is_summon_cancelable(d) && sg.is_empty();
                    let mut g2 = filter_p32(d, &x, &mut g, &mut tsg, &mut ntsg, &mut sg);
                    if g2.is_empty() {
                        break;
                    }
                    let g3 = filter_p31(d, &x, &mut g, &mut tsg, &mut ntsg, &mut sg);
                    if !g3.is_empty() && group::difference(&ntsg, &selectedastuner).is_empty() && tsg.len() < p.max1 {
                        group::add_card(&mut g2, &g3);
                    }
                    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SMATERIAL as u64);
                    let Some(tc) = group::select_unselect(d, &g2, Some(&sg), tp, finish, cancel, None, None) else {
                        if ntsg.len() >= p.min2 && x.fl.req(d, &p.req2, &ntsg) && x.fl.req(d, &p.reqm, &sg) && group::includes(&sg, &pg) && check_p43(d, &tsg, &ntsg, &sg, lv, c, tp) {
                            break;
                        }
                        return false;
                    };
                    if !selectedastuner.contains(tc) {
                        if !sg.contains(tc) {
                            ntsg.insert(tc);
                            sg.insert(tc);
                            for te in card::get_card_effect(d, tc, EFFECT_SYNCHRO_CHECK) {
                                for mc in group::iter(d, &g) {
                                    call_value(d, te, &[Value::Effect(te), Value::Card(mc)]);
                                }
                            }
                        } else {
                            ntsg.remove(tc);
                            sg.remove(tc);
                            reset_check_assume(d, &sg);
                        }
                    } else if group::difference(&ntsg, &selectedastuner).is_empty() {
                        tune = true;
                        selectedastuner.remove(tc);
                        ntsg.remove(tc);
                        tsg.remove(tc);
                        sg.remove(tc);
                    }
                }
            }
            duel::assume_reset(d);
        } else {
            let mut ntsg = CardSet::new();
            let mut tune = true;
            while ntsg.len() < p.max2 {
                if tune {
                    let cancel = !mgchk && duel::is_summon_cancelable(d) && tsg.is_empty();
                    let g3 = filter_p42(d, &x, &mut ntg, &mut tsg, &mut ntsg, &mut sg);
                    let mut g2 = filter_p41(d, &x, &mut tg, &mut ntg, &mut tsg, &mut ntsg, &mut sg);
                    if !g3.is_empty() && tsg.len() >= p.min1 && x.fl.req(d, &p.req1, &tsg) {
                        group::add_card(&mut g2, &g3);
                    }
                    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SMATERIAL as u64);
                    match group::select_unselect(d, &g2, Some(&sg), tp, false, cancel, None, None) {
                        None => {
                            if tsg.len() >= p.min1 && x.fl.req(d, &p.req1, &tsg) && !filter_p42(d, &x, &mut ntg, &mut tsg, &mut ntsg, &mut sg).is_empty() {
                                tune = false;
                            } else {
                                return false;
                            }
                        }
                        Some(tc) => {
                            if !sg.contains(tc) {
                                if g3.contains(tc) {
                                    ntsg.insert(tc);
                                    tune = false;
                                } else {
                                    tsg.insert(tc);
                                }
                                selectedastuner.insert(tc);
                                sg.insert(tc);
                            } else {
                                selectedastuner.remove(tc);
                                tsg.remove(tc);
                                sg.remove(tc);
                            }
                        }
                    }
                    if filter_p41(d, &x, &mut tg, &mut ntg, &mut tsg, &mut ntsg, &mut sg).is_empty() || tsg.len() >= p.max1 {
                        tune = false;
                    }
                } else {
                    let finish = ntsg.len() >= p.min2 && x.fl.req(d, &p.req2, &ntsg) && x.fl.req(d, &p.reqm, &sg) && group::includes(&sg, &pg) && check_p43(d, &tsg, &ntsg, &sg, lv, c, tp);
                    let cancel = !mgchk && duel::is_summon_cancelable(d) && sg.is_empty();
                    let mut g2 = filter_p42(d, &x, &mut ntg, &mut tsg, &mut ntsg, &mut sg);
                    if g2.is_empty() {
                        break;
                    }
                    let g3 = filter_p41(d, &x, &mut tg, &mut ntg, &mut tsg, &mut ntsg, &mut sg);
                    if !g3.is_empty() && group::difference(&ntsg, &selectedastuner).is_empty() && tsg.len() < p.max1 {
                        group::add_card(&mut g2, &g3);
                    }
                    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SMATERIAL as u64);
                    let Some(tc) = group::select_unselect(d, &g2, Some(&sg), tp, finish, cancel, None, None) else {
                        if ntsg.len() >= p.min2 && x.fl.req(d, &p.req2, &ntsg) && x.fl.req(d, &p.reqm, &sg) && group::includes(&sg, &pg) && check_p43(d, &tsg, &ntsg, &sg, lv, c, tp) {
                            break;
                        }
                        return false;
                    };
                    if !selectedastuner.contains(tc) {
                        if !sg.contains(tc) {
                            ntsg.insert(tc);
                            sg.insert(tc);
                        } else {
                            ntsg.remove(tc);
                            sg.remove(tc);
                        }
                    } else if group::difference(&ntsg, &selectedastuner).is_empty() {
                        tune = true;
                        selectedastuner.remove(tc);
                        ntsg.remove(tc);
                        tsg.remove(tc);
                        sg.remove(tc);
                    }
                }
            }
        }
        reset_hand_synchro(d, tp);
        // (substitute Tuners become Tuners)
        let subtsg = group::filter(d, &tsg, &|d, cc| x.fl.sub1(d, cc) && ((p.f1.is_some() && !x.fl.f1(d, cc)) || !card::is_type(d, cc, TYPE_TUNER)), NO_EX);
        for subc in group::iter(d, &subtsg) {
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_ADD_TYPE);
            effect::set_value(d, e1, TYPE_TUNER as i64);
            effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
            card::register_effect(d, subc, e1, true);
        }
        effect::set_label_object(d, e, Value::Group(sg));
        true
    })
}
/// `Synchro.Operation(e,tp,eg,ep,ev,re,r,rp,c,smat,mg)`
fn operation_fn() -> LuaFn {
    lua_fn(|d, a| {
        let (e, tp) = (a.e(0), a.p(1));
        let c = a.c(8);
        let g = effect::get_label_object(d, e).as_group().cloned().nil();
        card::set_material(d, c, g.clone());
        // (the operation of the Synchro Monster's EFFECT_MATERIAL_CHECK effect: "Cupid Pitch")
        if let Some(&mce) = card::is_has_effect(d, c, EFFECT_MATERIAL_CHECK, None).first() {
            if let Some(op) = effect::get_operation(d, mce) {
                op.call(d, &[Value::Effect(mce), Value::Card(c)]);
            }
        }
        // (the count limit of EFFECT_SYNCHRO_MAT_FROM_HAND effects: "Revolution Synchron"; the
        // Lua tests `GetValue(...)`, which is always truthy)
        for mc in group::iter(d, &g) {
            if let Some(&h) = card::is_has_effect(d, mc, EFFECT_SYNCHRO_MAT_FROM_HAND, None).first() {
                effect::use_count_limit(d, h, tp, 1, false);
            }
        }
        let tg = group::filter(d, &g, &|d, x| card::get_flag_effect(d, x, 3096468) != 0, NO_EX);
        if !tg.is_empty() {
            set_send(d, 2);
            for tc in group::iter(d, &tg) {
                card::reset_flag_effect(d, tc, 3096468);
            }
        }
        let send = d.script_var(vars::SEND).int();
        let r = REASON_MATERIAL + REASON_SYNCHRO;
        match send {
            1 => {
                duel::send_to_grave(d, g, r + REASON_RETURN, None, None);
            }
            2 => {
                duel::remove(d, g, POS_FACEUP as u8, r, None, None);
            }
            3 => {
                duel::remove(d, g, POS_FACEDOWN as u8, r, None, None);
            }
            4 => {
                duel::send_to_hand(d, g, None, r, None);
            }
            5 => {
                duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, r, None);
            }
            6 => {
                duel::destroy(d, g, r, None, None);
            }
            _ => {
                duel::send_to_grave(d, g, r, None, None);
            }
        }
        set_send(d, 0);
        set_check_additional(d, None);
    })
}
