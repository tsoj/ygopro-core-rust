//! `Fusion.*` (`proc_fusion.lua`): Fusion material procedures and contact Fusions.
//! `Fusion.CheckSelectMixRep`/`CheckMixRepSelectedCond` are memoized as in
//! `core_ext/lua/ygoai_overrides.lua`. (`AddProcMixRepUnfix` is not ported: no script uses it.)

use crate::aux;
use crate::constants::*;
use crate::lua::{card, duel, effect, group, NO_EX};
use crate::prelude::*;
// (memo tables: lookups only, so the fast non-random hash)
use rustc_hash::FxHashMap as HashMap;
use std::sync::Arc;

/// What a material function gets after the card: `function(c, fc, sumtype, tp, sub, mg, sg, contact)`
pub struct MatArgs<'a> {
    pub fc: CardId,
    pub sumtype: u64,
    pub tp: u8,
    pub sub: bool,
    pub mg: Option<&'a CardSet>,
    pub sg: Option<&'a CardSet>,
    pub contact: bool,
}
/// A material function of a Fusion procedure
pub type FusMat = Arc<dyn Fn(&mut Duel, CardId, &MatArgs) -> bool + Send + Sync>;
/// A material function of the usual form `function(c, fc, sumtype, tp)` (e.g.
/// `aux.FilterBoolFunctionEx(Card.IsRace, RACE_DRAGON)`: `fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_DRAGON, Some(fc), st, Some(tp)))`)
pub fn fus_mat(f: impl Fn(&mut Duel, CardId, CardId, u64, u8) -> bool + Send + Sync + 'static) -> FusMat {
    Arc::new(move |d, c, a| f(d, c, a.fc, a.sumtype, a.tp))
}
/// A material function that uses all the arguments (`sub`, `mg`, `sg`, `contact`)
pub fn fus_mat_full(f: impl Fn(&mut Duel, CardId, &MatArgs) -> bool + Send + Sync + 'static) -> FusMat {
    Arc::new(f)
}
/// A material of a procedure: a card name, a function, or a table of names and functions (any of
/// them).
#[derive(Clone)]
pub enum Mat {
    Code(u32),
    F(FusMat),
    Any(Vec<Mat>),
}

/// Script state of the Fusion procedures.
pub mod vars {
    use crate::ns::AUX;
    /// `Fusion.SummonEffect`
    pub const SUMMON_EFFECT: (u32, u64) = (AUX, 400);
    /// `ForcedUseZone` (a local of `proc_fusion.lua`)
    pub const FORCED_USE_ZONE: (u32, u64) = (AUX, 401);
    /// `Fusion.CheckExact`, `Fusion.CheckMin`, `Fusion.CheckMax` (numbers), `Fusion.CheckAdditional` (a function)
    pub const CHECK_EXACT: (u32, u64) = (AUX, 402);
    pub const CHECK_MIN: (u32, u64) = (AUX, 403);
    pub const CHECK_MAX: (u32, u64) = (AUX, 404);
    pub const CHECK_ADDITIONAL: (u32, u64) = (AUX, 405);
}
/// `Fusion.CheckExact = n` (`None`: nil)
pub fn set_check_exact(d: &mut Duel, n: Option<i64>) {
    d.set_script_var(vars::CHECK_EXACT, n.map_or(Value::Nil, Value::Int));
}
pub fn set_check_min(d: &mut Duel, n: Option<i64>) {
    d.set_script_var(vars::CHECK_MIN, n.map_or(Value::Nil, Value::Int));
}
pub fn set_check_max(d: &mut Duel, n: Option<i64>) {
    d.set_script_var(vars::CHECK_MAX, n.map_or(Value::Nil, Value::Int));
}
/// `Fusion.CheckAdditional = f` (`f(tp, sg, fc, sumtype, tp)`)
pub fn set_check_additional(d: &mut Duel, f: Option<LuaFn>) {
    d.set_script_var(vars::CHECK_ADDITIONAL, f.map_or(Value::Nil, Value::Fn));
}
/// `Fusion.SummonEffect`
pub fn summon_effect(d: &Duel) -> Option<EffectId> {
    d.script_var(vars::SUMMON_EFFECT).as_effect()
}
fn set_summon_effect(d: &mut Duel, e: Value) {
    d.set_script_var(vars::SUMMON_EFFECT, e);
}
fn opt_int(d: &Duel, k: (u32, u64)) -> Option<i64> {
    match d.script_var(k) {
        Value::Nil => None,
        v => Some(v.int()),
    }
}
fn check_additional(d: &mut Duel, tp: u8, g: &CardSet, fc: CardId, sumtype: u64) -> Option<bool> {
    match d.script_var(vars::CHECK_ADDITIONAL) {
        Value::Fn(f) => Some(f.call(d, &[Value::Int(tp as i64), Value::Group(g.clone()), Value::Card(fc), Value::Int(sumtype as i64), Value::Int(tp as i64)]).truthy()),
        _ => None,
    }
}
fn forced_zone(d: &Duel) -> Option<u32> {
    opt_int(d, vars::FORCED_USE_ZONE).map(|z| z as u32)
}

/// The arguments of the procedure's material functions: `(c, fc, sub, sub2, mg, sg, tp, contact, sumtype)`.
pub struct FunCtx<'a> {
    pub fc: CardId,
    pub sub: bool,
    pub sub2: bool,
    pub mg: Option<&'a CardSet>,
    pub sg: Option<&'a CardSet>,
    pub tp: u8,
    pub contact: bool,
    pub sumtype: u64,
}
pub type Fun = Arc<dyn Fn(&mut Duel, CardId, &FunCtx) -> bool + Send + Sync>;
fn has(d: &mut Duel, c: CardId, code: u32) -> bool {
    !card::is_has_effect(d, c, code, None).is_empty()
}
fn mat_args<'a>(x: &FunCtx<'a>) -> MatArgs<'a> {
    MatArgs { fc: x.fc, sumtype: x.sumtype, tp: x.tp, sub: x.sub, mg: x.mg, sg: x.sg, contact: x.contact }
}
fn name_check(d: &mut Duel, c: CardId, x: &FunCtx, codes: &[u32]) -> bool {
    let fcp = card::get_controler(d, x.fc);
    card::is_summon_code(d, c, Some(x.fc), x.sumtype, fcp, codes) || (x.sub && card::check_fusion_substitute(d, c, x.fc)) || (x.sub2 && has(d, c, 511002961))
}
fn add_mat(mat: &mut Vec<u32>, code: u32) {
    if !mat.contains(&code) {
        mat.push(code);
    }
}
/// `Fusion.ParseMaterialTable(tab, mat)`
fn parse_material_table(tab: &[Mat], mat: &mut Vec<u32>) -> Fun {
    let mut funcs: Vec<FusMat> = Vec::new();
    let mut named: Vec<u32> = Vec::new();
    for m in tab {
        match m {
            Mat::F(f) => funcs.push(f.clone()),
            Mat::Code(code) => {
                named.push(*code);
                add_mat(mat, *code);
            }
            Mat::Any(_) => panic!("nested material tables"),
        }
    }
    Arc::new(move |d, c, x| {
        // (aux.OR(aux.FALSE, aux.OR(function_func(f), tmp_extramat_func), ...))
        let mut any = false;
        for f in &funcs {
            if f(d, c, &mat_args(x)) || (x.sub2 && has(d, c, 511002961)) {
                any = true;
                break;
            }
        }
        (any && !has(d, c, 6205579)) || (!named.is_empty() && name_check(d, c, x, &named))
    })
}
pub fn make_fun(m: &Mat, mat: &mut Vec<u32>) -> Fun {
    match m {
        Mat::F(f) => {
            let f = f.clone();
            Arc::new(move |d, c, x| (f(d, c, &mat_args(x)) || (x.sub2 && has(d, c, 511002961))) && !has(d, c, 6205579))
        }
        Mat::Any(tab) => parse_material_table(tab, mat),
        Mat::Code(code) => {
            add_mat(mat, *code);
            let code = *code;
            Arc::new(move |d, c, x| name_check(d, c, x, &[code]))
        }
    }
}
fn set_material_fields(d: &mut Duel, c: CardId, mat: &[u32], min: usize, max: usize) {
    use crate::class;
    if class::get(d, c, class::MATERIAL_COUNT).is_nil() {
        if !mat.is_empty() {
            class::set(d, c, class::MATERIAL_COUNT, Value::Int(mat.len() as i64));
            class::set(d, c, class::MATERIAL, Value::Multi(mat.iter().map(|&x| Value::Int(x as i64)).collect()));
        }
        class::set(d, c, class::MIN_MATERIAL_COUNT, Value::Int(min as i64));
        class::set(d, c, class::MAX_MATERIAL_COUNT, Value::Int(max as i64));
    }
}

/// The flags of `chkfnf`.
struct Chk {
    chkf: u8,
    contact: bool,
    cancelable: bool,
    sumtype: u64,
    sub: bool,
}
fn parse_chk(d: &mut Duel, chkfnf: u64, sub: bool) -> Chk {
    let chkf = (chkfnf & 0xff) as u8;
    let zone = ((chkfnf >> 40) & 0xff) as u32;
    d.set_script_var(vars::FORCED_USE_ZONE, Value::Int(if zone == 0 { 0xff } else { zone } as i64));
    let notfusion = chkfnf & FUSPROC_NOTFUSION as u64 != 0;
    let contact = chkfnf & FUSPROC_CONTACTFUS as u64 != 0;
    let cancelable = chkfnf & (FUSPROC_CONTACTFUS | FUSPROC_CANCELABLE) as u64 != 0;
    let listedmats = chkfnf & FUSPROC_LISTEDMATS as u64 != 0;
    let sumtype = if listedmats {
        0
    } else if contact || notfusion {
        MATERIAL_FUSION
    } else {
        SUMMON_TYPE_FUSION as u64 | MATERIAL_FUSION
    };
    Chk { chkf, contact, cancelable, sumtype, sub: !listedmats && (sub || notfusion) && !contact }
}
/// `Fusion.ConditionFilterMix(c, fc, sub, sub2, contact, sumtype, matcheck, tp, ...)`
fn condition_filter_mix(d: &mut Duel, c: CardId, fc: CardId, k: &Chk, matcheck: &Value, tp: u8, funs: &[Fun]) -> bool {
    let x = FunCtx { fc, sub: k.sub, sub2: k.sub, mg: None, sg: None, tp, contact: k.contact, sumtype: k.sumtype };
    match matcheck {
        Value::Fn(f) => {
            if !f.call(d, &[Value::Card(c), Value::Card(fc), Value::Bool(k.sub), Value::Bool(k.sub), Value::Nil, Value::Nil, Value::Int(tp as i64), Value::Bool(k.contact), Value::Int(k.sumtype as i64)]).truthy() {
                return false;
            }
        }
        v if v.int() != 0 => crate::lua_error("attempt to call a number value (local 'matcheck')"),
        _ => {}
    }
    if !card::is_can_be_fusion_material(d, c, Some(fc), Some(k.sumtype), None) {
        return false;
    }
    funs.iter().any(|f| f(d, c, &x))
}
/// `aux.HarmonizingMagFilter(c, f, f:GetValue())`
fn harmonizing(d: &mut Duel, c: CardId, f: EffectId) -> bool {
    match effect::get_value(d, f) {
        Value::Fn(v) => !v.call(d, &[Value::Effect(f), Value::Card(c)]).truthy(),
        _ => crate::lua_error("attempt to call a number value (local 'f')"),
    }
}
/// The material restrictions ("Harmonizing Magician") of adding `c` to `sg`: `None` if `c` cannot
/// be added, else `mg2` without the cards `c` excludes.
fn restrict(d: &mut Duel, c: CardId, sg: &CardSet, mg: &CardSet, mustg: &CardSet, contact: bool) -> Option<CardSet> {
    let mut mg2 = mg.clone();
    if !contact && has(d, c, EFFECT_FUSION_MAT_RESTRICTION) {
        for f in card::get_card_effect(d, c, EFFECT_FUSION_MAT_RESTRICTION) {
            if group::is_exists(d, sg, &|d, x| harmonizing(d, x, f), 1, c.into()) {
                return None;
            }
            let sg2 = group::filter(d, &mg2, &|d, x| harmonizing(d, x, f), NO_EX);
            group::remove_card(&mut mg2, &sg2);
            if !mustg.is_empty() && !group::includes(&mg2, mustg) {
                return None;
            }
        }
    }
    if !contact {
        let g2 = group::filter(d, sg, &|d, x| has(d, x, EFFECT_FUSION_MAT_RESTRICTION), NO_EX);
        for tc in group::iter(d, &g2) {
            for f in card::get_card_effect(d, tc, EFFECT_FUSION_MAT_RESTRICTION) {
                if harmonizing(d, c, f) {
                    return None;
                }
            }
        }
    }
    Some(mg2)
}
fn zone_ok(d: &mut Duel, chkf: u8, tp: u8, sg: &CardSet, fc: CardId, extra_only: bool) -> bool {
    if chkf as u32 == PLAYER_NONE {
        return true;
    }
    let zone = forced_zone(d);
    if extra_only || card::is_location(d, fc, LOCATION_EXTRA) {
        duel::get_location_count_from_ex(d, chkf, Some(tp), sg.into(), duel::ExtraCard::Card(fc), zone).0 > 0
    } else {
        duel::get_m_zone_count(d, chkf, sg.into(), Some(tp), None, None).0 > 0
    }
}

// ---------------------------------------------------------------------------------------------
// Fusion.AddProcMix: one material per function

/// What the checks of one procedure call share.
struct MixCtx<'a> {
    funs: &'a [Fun],
    tp: u8,
    fc: CardId,
    sub2: bool,
    contact: bool,
    sumtype: u64,
    chkf: u8,
    /// `check_mix_goal` results by selected set: the result depends only on the set, and
    /// `select_mix` reaches the same set in every order (n! times for n materials)
    goal_memo: std::cell::RefCell<HashMap<(Vec<CardId>, bool), bool>>,
    /// `select_mix` results by selected set (with the new card): without material restrictions
    /// ("Harmonizing Magician", `restrict`) the result depends only on the set, which `select_mix`
    /// reaches in every order. `None` when a card of the material pool has a restriction.
    select_memo: Option<std::cell::RefCell<HashMap<(CardSet, bool), bool>>>,
    /// `exists_mix` results by (pool, cards given to the functions before, sub, functions left):
    /// the result does not depend on which card went to which function before, and
    /// `check_mix_goal` tries every such assignment (n! for n materials)
    exists_memo: std::cell::RefCell<HashMap<(CardSet, CardSet, bool, usize), bool>>,
}
impl<'a> MixCtx<'a> {
    #[allow(clippy::too_many_arguments)]
    fn new(d: &mut Duel, funs: &'a [Fun], tp: u8, fc: CardId, k: &Chk, mg: &CardSet, mustg: &CardSet) -> Self {
        let restricted = !k.contact && group::iter(d, &group::union(mg, mustg)).into_iter().any(|c| has(d, c, EFFECT_FUSION_MAT_RESTRICTION));
        MixCtx {
            funs,
            tp,
            fc,
            sub2: k.sub,
            contact: k.contact,
            sumtype: k.sumtype,
            chkf: k.chkf,
            goal_memo: Default::default(),
            select_memo: (!restricted).then(Default::default),
            exists_memo: Default::default(),
        }
    }
}
/// `Fusion.CheckMix(c, mg, sg, fc, sub, sub2, contact, sumtype, tp, fun1, fun2, ...)`
fn check_mix(d: &mut Duel, x: &MixCtx, c: CardId, mg: &CardSet, sg: &mut CardSet, sub: bool, funs: &[Fun]) -> bool {
    let fun1 = &funs[0];
    if funs.len() > 1 {
        sg.insert(c);
        let mut res = false;
        let sgc = sg.clone();
        let a = FunCtx { fc: x.fc, sub: false, sub2: x.sub2, mg: Some(mg), sg: Some(&sgc), tp: x.tp, contact: x.contact, sumtype: x.sumtype };
        if fun1(d, c, &a) {
            res = exists_mix(d, x, mg, sg, sub, &funs[1..]);
        } else {
            let b = FunCtx { sub: true, ..a };
            if sub && fun1(d, c, &b) {
                res = exists_mix(d, x, mg, sg, false, &funs[1..]);
            }
        }
        sg.remove(c);
        res
    } else {
        let sgc = sg.clone();
        fun1(d, c, &FunCtx { fc: x.fc, sub, sub2: x.sub2, mg: Some(mg), sg: Some(&sgc), tp: x.tp, contact: x.contact, sumtype: x.sumtype })
    }
}
/// `mg:IsExists(Fusion.CheckMix, 1, sg, mg, sg, ...)`
fn exists_mix(d: &mut Duel, x: &MixCtx, mg: &CardSet, sg: &mut CardSet, sub: bool, funs: &[Fun]) -> bool {
    let key = (mg.clone(), sg.clone(), sub, funs.len());
    if let Some(&v) = x.exists_memo.borrow().get(&key) {
        return v;
    }
    let mut res = false;
    for c in group::iter(d, &group::difference(mg, sg)) {
        if ygo_core::api::check_matching_with(d, c, |d| check_mix(d, x, c, mg, sg, sub, funs)) {
            res = true;
            break;
        }
    }
    x.exists_memo.borrow_mut().insert(key, res);
    res
}
/// `Fusion.CheckMixGoal(tp, sg, fc, sub, sub2, contact, sumtype, chkf, ...)`
fn check_mix_goal(d: &mut Duel, x: &MixCtx, sg: &CardSet, sub: bool) -> bool {
    let key = (sg.to_vec(), sub);
    if let Some(&v) = x.goal_memo.borrow().get(&key) {
        return v;
    }
    let v = check_mix_goal_uncached(d, x, sg, sub);
    x.goal_memo.borrow_mut().insert(key, v);
    v
}
fn check_mix_goal_uncached(d: &mut Duel, x: &MixCtx, sg: &CardSet, sub: bool) -> bool {
    let mut g = CardSet::new();
    let mut ok = false;
    for c in group::iter(d, sg) {
        if ygo_core::api::check_matching_with(d, c, |d| check_mix(d, x, c, sg, &mut g, sub, x.funs)) {
            ok = true;
            break;
        }
    }
    ok && zone_ok(d, x.chkf, x.tp, sg, x.fc, false) && check_additional(d, x.tp, sg, x.fc, x.sumtype).unwrap_or(true)
}
/// `Fusion.SelectMix(c, tp, mg, sg, mustg, fc, sub, sub2, contact, sumtype, chkf, ...)`
fn select_mix(d: &mut Duel, x: &MixCtx, c: CardId, mg: &CardSet, sg: &mut CardSet, mustg: &CardSet, sub: bool) -> bool {
    let totalcount = x.funs.len() as i64;
    let mustgcount = mustg.len() as i64;
    if mustgcount > totalcount {
        return false;
    }
    if let Some(e) = opt_int(d, vars::CHECK_EXACT) {
        if e != totalcount || mustgcount > e {
            return false;
        }
    }
    if let Some(m) = opt_int(d, vars::CHECK_MAX) {
        if m < totalcount || mustgcount > m {
            return false;
        }
    }
    if let Some(m) = opt_int(d, vars::CHECK_MIN) {
        if m > totalcount {
            return false;
        }
    }
    let Some(mg2) = restrict(d, c, sg, mg, mustg, x.contact) else { return false };
    sg.insert(c);
    let key = x.select_memo.as_ref().map(|_| (sg.clone(), sub));
    if let (Some(m), Some(k)) = (&x.select_memo, &key) {
        if let Some(&v) = m.borrow().get(k) {
            sg.remove(c);
            return v;
        }
    }
    let res = if (sg.len() as i64) < totalcount {
        let rest = group::difference(mustg, sg);
        let mut found = false;
        for c2 in group::iter(d, &group::difference(&mg2, sg)) {
            if ygo_core::api::check_matching_with(d, c2, |d| select_mix(d, x, c2, &mg2, sg, &rest, sub)) {
                found = true;
                break;
            }
        }
        found
    } else {
        let r = check_mix_goal(d, x, &sg.clone(), sub);
        r && group::includes(sg, mustg)
    };
    if let (Some(m), Some(k)) = (&x.select_memo, key) {
        m.borrow_mut().insert(k, res);
    }
    sg.remove(c);
    res
}

/// `Fusion.AddProcMix(c, sub, insf, ...)`: one material per entry of `mats`
pub fn add_proc_mix(d: &mut Duel, c: CardId, sub: bool, insf: bool, mats: Vec<Mat>) -> Option<EffectId> {
    if card::is_status(d, c, STATUS_COPYING_EFFECT) {
        return None;
    }
    let mut mat = Vec::new();
    let funs: Vec<Fun> = mats.iter().map(|m| make_fun(m, &mut mat)).collect();
    set_material_fields(d, c, &mat, mats.len(), mats.len());
    let funs: Arc<[Fun]> = funs.into();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_FUSION_MATERIAL);
    effect::set_condition(d, e1, Some(condition_mix(insf, sub, funs.clone())));
    effect::set_operation(d, e1, Some(operation_mix(sub, funs)));
    card::register_effect(d, c, e1, false);
    Some(e1)
}
fn group_arg(v: &Value) -> Option<CardSet> {
    match v {
        Value::Group(g) => Some(g.clone()),
        Value::Card(c) => Some(CardSet::from_card(*c)),
        _ => None,
    }
}
/// The start of the conditions: `None` when `g` is nil (Instant Fusion: the result is then decided).
fn must_without_g(d: &mut Duel, tp: u8, c: CardId) -> CardSet {
    aux::get_must_be_material_group_opt(d, tp, None, tp, Some(c), None, REASON_FUSION)
}
/// `Fusion.ConditionMix(insf, sub, ...)`: `function(e, g, gc, chkfnf, summonEff)`
pub fn condition_mix(insf: bool, sub: bool, funs: Arc<[Fun]>) -> LuaFn {
    lua_fn(move |d, a| {
        let e = a.e(0);
        let c = effect::get_handler(d, e).nil();
        let tp = card::get_controler(d, c);
        let Some(g) = group_arg(a.v(1)) else {
            let mustg = must_without_g(d, tp, c);
            return insf && mustg.is_empty();
        };
        let gc = group_arg(a.v(2));
        set_summon_effect(d, a.v(4).clone());
        let k = parse_chk(d, a.int(3) as u64, sub);
        let matcheck = effect::get_value(d, e);
        let mut mg = group::filter(d, &g, &|d, x| condition_filter_mix(d, x, c, &k, &matcheck, tp, &funs), c.into());
        let mut mustg = aux::get_must_be_material_group(d, tp, &g, tp, Some(c), &mg, REASON_FUSION);
        if k.contact {
            mustg.clear();
        }
        let res = (|| {
            if !group::includes(&mg, &mustg) || group::is_exists(d, &mustg, &|d, x| !card::is_can_be_fusion_material(d, x, Some(c), Some(k.sumtype), None), 1, NO_EX) {
                return false;
            }
            if let Some(gc) = &gc {
                if group::is_exists(d, gc, &|d, x| !card::is_can_be_fusion_material(d, x, Some(c), Some(k.sumtype), None), 1, NO_EX)
                    || group::is_exists(d, gc, &|d, x| !condition_filter_mix(d, x, c, &k, &matcheck, tp, &funs), 1, NO_EX)
                {
                    return false;
                }
                group::add_card(&mut mustg, gc);
            }
            let mut sg = CardSet::new();
            group::add_card(&mut mg, &mustg);
            let x = MixCtx::new(d, &funs, tp, c, &k, &mg, &mustg);
            for c2 in group::iter(d, &mg) {
                if ygo_core::api::check_matching_with(d, c2, |d| select_mix(d, &x, c2, &mg, &mut sg, &mustg, k.sub)) {
                    return true;
                }
            }
            false
        })();
        set_summon_effect(d, Value::Nil);
        res
    })
}
/// The opponent chooses the materials ("Shaddoll Fusion"-like effect 511004008): (selecting
/// player, shuffle the hand after).
fn material_chooser(d: &mut Duel, tp: u8, mg: &CardSet, contact: bool) -> (u8, bool) {
    let mut p = tp;
    let mut sfhchk = false;
    if !contact && duel::is_player_affected_by_effect(d, tp, 511004008).is_some() && duel::select_yes_no(d, 1 - tp, 65) {
        p = 1 - tp;
        duel::confirm_cards(d, 1 - tp, mg);
        if group::is_exists(d, mg, &|d, x| card::is_location(d, x, LOCATION_HAND), 1, NO_EX) {
            sfhchk = true;
        }
    }
    (p, sfhchk)
}
/// `Fusion.OperationMix(insf, sub, ...)`: `function(e,tp,eg,ep,ev,re,r,rp,gc,chkfnf,summonEff)`
pub fn operation_mix(sub: bool, funs: Arc<[Fun]>) -> LuaFn {
    lua_fn(move |d, a| {
        let e = a.e(0);
        let eg = a.g(2);
        let gc = group_arg(a.v(8));
        set_summon_effect(d, a.v(10).clone());
        let k = parse_chk(d, a.int(9) as u64, sub);
        let c = effect::get_handler(d, e).nil();
        let tp = card::get_controler(d, c);
        let matcheck = effect::get_value(d, e);
        let mg = group::filter(d, &eg, &|d, x| condition_filter_mix(d, x, c, &k, &matcheck, tp, &funs), c.into());
        let mut mustg = aux::get_must_be_material_group(d, tp, &eg, tp, Some(c), &mg, REASON_FUSION);
        if k.contact {
            mustg.clear();
        }
        let mut sg = CardSet::new();
        if let Some(gc) = &gc {
            group::add_card(&mut mustg, gc);
        }
        for tc in group::iter(d, &mustg) {
            sg.insert(tc);
            if !k.contact && has(d, tc, EFFECT_FUSION_MAT_RESTRICTION) {
                // (`gc:GetCardEffect(...)`: gc is a group, which has no such method)
                crate::lua_error("attempt to call a nil value (method 'GetCardEffect')");
            }
        }
        let (p, sfhchk) = material_chooser(d, tp, &mg, k.contact);
        let x = MixCtx::new(d, &funs, tp, c, &k, &mg, &mustg);
        while sg.len() < funs.len() {
            duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_FMATERIAL as u64);
            let rest = group::difference(&mustg, &sg);
            let mut cg = CardSet::new();
            for c2 in group::iter(d, &group::difference(&mg, &sg)) {
                if ygo_core::api::check_matching_with(d, c2, |d| select_mix(d, &x, c2, &mg, &mut sg, &rest, k.sub)) {
                    cg.insert(c2);
                }
            }
            let n = funs.len() as u16;
            let Some(tc) = group::select_unselect(d, &cg, Some(&sg), p, false, k.cancelable && sg.is_empty(), Some(n), Some(n)) else { break };
            if mustg.is_empty() || !mustg.contains(tc) {
                if !sg.contains(tc) {
                    sg.insert(tc);
                } else {
                    sg.remove(tc);
                }
            }
        }
        if sfhchk {
            duel::shuffle_hand(d, tp);
        }
        duel::set_fusion_material(d, &sg);
        set_summon_effect(d, Value::Nil);
    })
}

// ---------------------------------------------------------------------------------------------
// Fusion.AddProcMixRep: fun1 * minc..maxc + one material per other function

/// What the checks of one `Fusion.SelectMixRep` call share (the material functions: `fun1`, and
/// the other ones by index in `slots`).
struct RepCtx<'a> {
    fun1: &'a Fun,
    funs: &'a [Fun],
    tp: u8,
    fc: CardId,
    sub2: bool,
    contact: bool,
    sumtype: u64,
    chkf: u8,
}
/// The memo of one top-level `Fusion.SelectMixRep` call (`ygoai_overrides.lua`).
type RepMemo = HashMap<(u8, Vec<CardId>, Vec<CardId>, bool, i64, i64, Vec<usize>), bool>;
impl RepCtx<'_> {
    fn call(&self, d: &mut Duel, f: &Fun, c: CardId, sub: bool, mg: &CardSet, sg: &CardSet) -> bool {
        f(d, c, &FunCtx { fc: self.fc, sub, sub2: self.sub2, mg: Some(mg), sg: Some(sg), tp: self.tp, contact: self.contact, sumtype: self.sumtype })
    }
}
/// `Fusion.CheckMixRep(sg, fun2Materials, fc, sub, sub2, contact, sumtype, chkf, tp, fun1, minc, maxc, fun2, ...)`
fn check_mix_rep(d: &mut Duel, x: &RepCtx, sg: &CardSet, g: &mut CardSet, sub: bool, slots: &[usize]) -> bool {
    if let Some((&first, rest)) = slots.split_first() {
        let _ = rest;
        for c in group::iter(d, &group::difference(sg, g)) {
            if ygo_core::api::check_matching_with(d, c, |d| check_mix_rep_filter(d, x, c, sg, g, sub, slots, first)) {
                return true;
            }
        }
        false
    } else {
        let total = group::union(sg, g);
        let rest = group::difference(sg, g);
        let mut ct1 = 0;
        let mut ct2 = 0;
        for c in group::iter(d, &rest) {
            if ygo_core::api::check_matching_with(d, c, |d| x.call(d, x.fun1, c, sub, &total, &total)) {
                ct1 += 1;
            }
        }
        for c in group::iter(d, &rest) {
            if ygo_core::api::check_matching_with(d, c, |d| x.call(d, x.fun1, c, false, &total, &total)) {
                ct2 += 1;
            }
        }
        ct1 == sg.len() as i64 - g.len() as i64 && ct1 - ct2 <= 1
    }
}
/// `Fusion.CheckMixRepFilter(c, sg, g, fc, sub, sub2, contact, sumtype, chkf, tp, fun1, minc, maxc, fun2, ...)`
#[allow(clippy::too_many_arguments)]
fn check_mix_rep_filter(d: &mut Duel, x: &RepCtx, c: CardId, sg: &CardSet, g: &mut CardSet, sub: bool, slots: &[usize], fun2: usize) -> bool {
    let gc = group::union(g, &CardSet::from_card(c));
    if x.call(d, &x.funs[fun2], c, sub, sg, &gc) {
        g.insert(c);
        let gs = g.clone();
        let sub = sub && x.call(d, &x.funs[fun2], c, false, sg, &gs);
        let res = check_mix_rep(d, x, sg, g, sub, &slots[1..]);
        g.remove(c);
        return res;
    }
    false
}
/// `Fusion.CheckMixRepGoal(tp, sg, mustg, fc, sub, sub2, contact, sumtype, chkf, fun1, minc, maxc, ...)`
fn check_mix_rep_goal(d: &mut Duel, x: &RepCtx, sg: &CardSet, sub: bool, minc: i64, maxc: i64, slots: &[usize]) -> bool {
    let n = sg.len() as i64;
    if n < minc + slots.len() as i64 || n > maxc + slots.len() as i64 {
        return false;
    }
    let mut g = CardSet::new();
    check_mix_rep(d, x, sg, &mut g, sub, slots) && zone_ok(d, x.chkf, x.tp, sg, x.fc, true) && check_additional(d, x.tp, sg, x.fc, x.sumtype).unwrap_or(true)
}
/// Which `cond` of `Fusion.CheckMixRepTemplate`
#[derive(Clone, Copy)]
enum Cond {
    /// `Fusion.CheckMixRepSelectedCond`
    Selected,
    /// `Fusion.CheckSelectMixRep`
    SelectRep,
}
/// The arguments the rep checks pass on after `tp`: `mg, sg, mustg, g, ..., sub, ..., minc, maxc, slots`
struct RepArgs<'a> {
    mg: &'a CardSet,
    sg: &'a CardSet,
}
#[allow(clippy::too_many_arguments)]
fn call_cond(d: &mut Duel, x: &RepCtx, memo: &mut RepMemo, cond: Cond, r: &RepArgs, mustg: &CardSet, g: &mut CardSet, sub: bool, minc: i64, maxc: i64, slots: &[usize]) -> bool {
    match cond {
        Cond::Selected => check_mix_rep_selected_cond(d, x, memo, r, mustg, g, sub, minc, maxc, slots),
        Cond::SelectRep => check_select_mix_rep(d, x, memo, r, mustg, g, sub, minc, maxc, slots),
    }
}
/// `Fusion.CheckMixRepTemplate(c, cond, tp, mg, sg, mustg, g, fc, sub, sub2, contact, sumtype, chkf, fun1, minc, maxc, ...)`
#[allow(clippy::too_many_arguments)]
fn check_mix_rep_template(d: &mut Duel, x: &RepCtx, memo: &mut RepMemo, c: CardId, cond: Cond, r: &RepArgs, mustg: &CardSet, g: &mut CardSet, sub: bool, minc: i64, maxc: i64, slots: &[usize]) -> bool {
    let mustg_c = group::difference(mustg, &CardSet::from_card(c));
    for (i, &f) in slots.iter().enumerate() {
        let gc = group::union(g, &CardSet::from_card(c));
        if x.call(d, &x.funs[f], c, sub, r.mg, &gc) {
            g.insert(c);
            let gs = g.clone();
            let sub = sub && x.call(d, &x.funs[f], c, false, r.mg, &gs);
            let mut t = slots.to_vec();
            t.remove(i);
            let res = call_cond(d, x, memo, cond, r, &mustg_c, g, sub, minc, maxc, &t);
            g.remove(c);
            if res {
                return true;
            }
        }
    }
    if maxc > 0 {
        let gc = group::union(g, &CardSet::from_card(c));
        if x.call(d, x.fun1, c, sub, r.mg, &gc) {
            g.insert(c);
            let gs = g.clone();
            let sub = sub && x.call(d, x.fun1, c, false, r.mg, &gs);
            let res = call_cond(d, x, memo, cond, r, &mustg_c, g, sub, minc - 1, maxc - 1, slots);
            g.remove(c);
            if res {
                return true;
            }
        }
    }
    false
}
fn memo_key(name: u8, g: &CardSet, mustg: &CardSet, sub: bool, minc: i64, maxc: i64, slots: &[usize]) -> (u8, Vec<CardId>, Vec<CardId>, bool, i64, i64, Vec<usize>) {
    (name, g.to_vec(), mustg.to_vec(), sub, minc, maxc, slots.to_vec())
}
/// `Fusion.CheckMixRepSelectedCond(tp, mg, sg, mustg, g, ...)` (memoized)
#[allow(clippy::too_many_arguments)]
fn check_mix_rep_selected_cond(d: &mut Duel, x: &RepCtx, memo: &mut RepMemo, r: &RepArgs, mustg: &CardSet, g: &mut CardSet, sub: bool, minc: i64, maxc: i64, slots: &[usize]) -> bool {
    let key = memo_key(b's', g, mustg, sub, minc, maxc, slots);
    if let Some(&v) = memo.get(&key) {
        return v;
    }
    let res = if g.len() < r.sg.len() {
        let mut found = false;
        for c in group::iter(d, &group::difference(r.sg, g)) {
            if ygo_core::api::check_matching_with(d, c, |d| check_mix_rep_template(d, x, memo, c, Cond::Selected, r, mustg, g, sub, minc, maxc, slots)) {
                found = true;
                break;
            }
        }
        found
    } else {
        check_select_mix_rep(d, x, memo, r, mustg, g, sub, minc, maxc, slots)
    };
    memo.insert(key, res);
    res
}
/// `Fusion.CheckSelectMixRep(tp, mg, sg, mustg, g, fc, sub, sub2, contact, sumtype, chkf, fun1, minc, maxc, ...)` (memoized)
#[allow(clippy::too_many_arguments)]
fn check_select_mix_rep(d: &mut Duel, x: &RepCtx, memo: &mut RepMemo, r: &RepArgs, mustg: &CardSet, g: &mut CardSet, sub: bool, minc: i64, maxc: i64, slots: &[usize]) -> bool {
    let key = memo_key(b'e', g, mustg, sub, minc, maxc, slots);
    if let Some(&v) = memo.get(&key) {
        return v;
    }
    let res = (|| {
        if check_additional(d, x.tp, g, x.fc, x.sumtype) == Some(false) {
            return false;
        }
        if zone_ok(d, x.chkf, x.tp, g, x.fc, true) {
            if minc <= 0 && slots.is_empty() && group::includes(g, mustg) {
                return true;
            }
            for c in group::iter(d, &group::difference(r.mg, g)) {
                if ygo_core::api::check_matching_with(d, c, |d| check_select_mix_rep_all(d, x, memo, c, r, mustg, g, sub, minc, maxc, slots)) {
                    return true;
                }
            }
            false
        } else {
            for c in group::iter(d, &group::difference(r.mg, g)) {
                let ok = ygo_core::api::check_matching_with(d, c, |d| card::is_controler(d, c, x.tp) && card::is_location(d, c, LOCATION_MZONE) && check_mix_rep_template(d, x, memo, c, Cond::SelectRep, r, mustg, g, sub, minc, maxc, slots));
                if ok {
                    return true;
                }
            }
            false
        }
    })();
    memo.insert(key, res);
    res
}
/// `Fusion.CheckSelectMixRepAll(c, tp, mg, sg, mustg, g, fc, sub, sub2, contact, sumtype, chkf, fun1, minc, maxc, fun2, ...)`
#[allow(clippy::too_many_arguments)]
fn check_select_mix_rep_all(d: &mut Duel, x: &RepCtx, memo: &mut RepMemo, c: CardId, r: &RepArgs, mustg: &CardSet, g: &mut CardSet, sub: bool, minc: i64, maxc: i64, slots: &[usize]) -> bool {
    let gc = group::union(g, &CardSet::from_card(c));
    if let Some((&fun2, rest)) = slots.split_first() {
        if x.call(d, &x.funs[fun2], c, sub, r.mg, &gc) {
            g.insert(c);
            let gs = g.clone();
            let sub = sub && x.call(d, &x.funs[fun2], c, false, r.mg, &gs);
            let res = check_select_mix_rep(d, x, memo, r, mustg, g, sub, minc, maxc, rest);
            g.remove(c);
            return res;
        }
    } else if maxc > 0 && x.call(d, x.fun1, c, sub, r.mg, &gc) {
        g.insert(c);
        let gs = g.clone();
        let sub = sub && x.call(d, x.fun1, c, false, r.mg, &gs);
        let res = check_select_mix_rep(d, x, memo, r, mustg, g, sub, minc - 1, maxc - 1, &[]);
        g.remove(c);
        return res;
    }
    false
}
/// `Fusion.SelectMixRep(c, tp, mg, sg, mustg, fc, sub, sub2, contact, sumtype, chkf, fun1, minc, maxc, ...)`
/// (a top-level call: its own memo)
#[allow(clippy::too_many_arguments)]
fn select_mix_rep(d: &mut Duel, x: &RepCtx, c: CardId, mg: &CardSet, sg: &mut CardSet, mustg: &CardSet, sub: bool, minc: i64, maxc: i64) -> bool {
    let slots: Vec<usize> = (0..x.funs.len()).collect();
    let totalcount = slots.len() as i64;
    let mustgcount = mustg.len() as i64;
    let (mut minc, mut maxc) = (minc, maxc);
    if let Some(e) = opt_int(d, vars::CHECK_EXACT) {
        if e < minc + totalcount || mustgcount > e {
            return false;
        }
        maxc = e - totalcount;
        minc = e - totalcount;
    }
    if let Some(m) = opt_int(d, vars::CHECK_MAX) {
        if m < minc + totalcount || mustgcount > m {
            return false;
        }
        maxc = maxc.min(m - totalcount);
    }
    if let Some(m) = opt_int(d, vars::CHECK_MIN) {
        if m > maxc + totalcount {
            return false;
        }
        minc = minc.max(m - totalcount);
    }
    let Some(mg2) = restrict(d, c, sg, mg, mustg, x.contact) else { return false };
    sg.insert(c);
    let res = if check_additional(d, x.tp, sg, x.fc, x.sumtype) == Some(false) {
        false
    } else if check_mix_rep_goal(d, x, &sg.clone(), sub, minc, maxc, &slots) {
        true
    } else {
        let mut memo = RepMemo::default();
        let sgc = sg.clone();
        let r = RepArgs { mg: &mg2, sg: &sgc };
        let mut g = CardSet::new();
        let mut found = false;
        for c2 in group::iter(d, &sgc) {
            if ygo_core::api::check_matching_with(d, c2, |d| check_mix_rep_template(d, x, &mut memo, c2, Cond::Selected, &r, mustg, &mut g, sub, minc, maxc, &slots)) {
                found = true;
                break;
            }
        }
        found
    };
    sg.remove(c);
    res
}

/// `Fusion.AddProcMixRep(c, sub, insf, fun1, minc, maxc, ...)`: `fun1` * `minc`..`maxc` + one
/// material per entry of `mats`
#[allow(clippy::too_many_arguments)]
pub fn add_proc_mix_rep(d: &mut Duel, c: CardId, sub: bool, insf: bool, fun1: Mat, minc: usize, maxc: usize, mats: Vec<Mat>) -> Option<EffectId> {
    if card::is_status(d, c, STATUS_COPYING_EFFECT) {
        return None;
    }
    let mut mat = Vec::new();
    let f1 = make_fun(&fun1, &mut mat);
    let funs: Vec<Fun> = mats.iter().map(|m| make_fun(m, &mut mat)).collect();
    set_material_fields(d, c, &mat, minc + mats.len(), maxc + mats.len());
    let funs: Arc<[Fun]> = funs.into();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_FUSION_MATERIAL);
    effect::set_condition(d, e1, Some(condition_mix_rep(insf, sub, f1.clone(), minc as i64, maxc as i64, funs.clone())));
    effect::set_operation(d, e1, Some(operation_mix_rep(sub, f1, minc as i64, maxc as i64, funs)));
    card::register_effect(d, c, e1, false);
    Some(e1)
}
fn all_funs(f1: &Fun, funs: &[Fun]) -> Vec<Fun> {
    let mut v = vec![f1.clone()];
    v.extend(funs.iter().cloned());
    v
}
/// `Fusion.ConditionMixRep(insf, sub, fun1, minc, maxc, ...)`
fn condition_mix_rep(insf: bool, sub: bool, f1: Fun, minc: i64, maxc: i64, funs: Arc<[Fun]>) -> LuaFn {
    lua_fn(move |d, a| {
        let e = a.e(0);
        let c = effect::get_handler(d, e).nil();
        let tp = card::get_controler(d, c);
        let Some(g) = group_arg(a.v(1)) else {
            if !insf {
                return false;
            }
            return must_without_g(d, tp, c).is_empty();
        };
        let gc = group_arg(a.v(2));
        set_summon_effect(d, a.v(4).clone());
        let k = parse_chk(d, a.int(3) as u64, sub);
        let matcheck = effect::get_value(d, e);
        // (`GetMustBeMaterialGroup(tp, eg, tp, c, mg, ...)`: eg and mg are not defined there)
        let mut mustg = must_without_g(d, tp, c);
        if k.contact {
            mustg.clear();
        }
        let every = all_funs(&f1, &funs);
        let mut mg = group::filter(d, &g, &|d, x| condition_filter_mix(d, x, c, &k, &matcheck, tp, &every), c.into());
        let res = (|| {
            if !group::includes(&mg, &mustg) || group::is_exists(d, &mustg, &|d, x| !card::is_can_be_fusion_material(d, x, Some(c), Some(k.sumtype), None), 1, NO_EX) {
                return false;
            }
            if let Some(gc) = &gc {
                if group::is_exists(d, gc, &|d, x| !card::is_can_be_fusion_material(d, x, Some(c), Some(k.sumtype), None), 1, NO_EX)
                    || group::is_exists(d, gc, &|d, x| !condition_filter_mix(d, x, c, &k, &matcheck, tp, &every), 1, NO_EX)
                {
                    return false;
                }
                group::add_card(&mut mustg, gc);
            }
            let mut sg = CardSet::new();
            group::add_card(&mut mg, &mustg);
            let x = RepCtx { fun1: &f1, funs: &funs, tp, fc: c, sub2: k.sub, contact: k.contact, sumtype: k.sumtype, chkf: k.chkf };
            for c2 in group::iter(d, &mg) {
                if ygo_core::api::check_matching_with(d, c2, |d| select_mix_rep(d, &x, c2, &mg, &mut sg, &mustg, k.sub, minc, maxc)) {
                    return true;
                }
            }
            false
        })();
        set_summon_effect(d, Value::Nil);
        res
    })
}
/// `Fusion.OperationMixRep(insf, sub, fun1, minc, maxc, ...)`
fn operation_mix_rep(sub: bool, f1: Fun, minc: i64, maxc: i64, funs: Arc<[Fun]>) -> LuaFn {
    lua_fn(move |d, a| {
        let e = a.e(0);
        let eg = a.g(2);
        let gc = group_arg(a.v(8));
        set_summon_effect(d, a.v(10).clone());
        let k = parse_chk(d, a.int(9) as u64, sub);
        let c = effect::get_handler(d, e).nil();
        let tp = card::get_controler(d, c);
        let matcheck = effect::get_value(d, e);
        let mut sg = CardSet::new();
        let every = all_funs(&f1, &funs);
        let mg = group::filter(d, &eg, &|d, x| condition_filter_mix(d, x, c, &k, &matcheck, tp, &every), c.into());
        let mut mustg = aux::get_must_be_material_group(d, tp, &eg, tp, Some(c), &mg, REASON_FUSION);
        if k.contact {
            mustg.clear();
        }
        if !group::includes(&mg, &mustg) || group::is_exists(d, &mustg, &|d, x| !card::is_can_be_fusion_material(d, x, Some(c), Some(k.sumtype), None), 1, NO_EX) {
            set_summon_effect(d, Value::Nil);
            return;
        }
        if let Some(gc) = &gc {
            group::add_card(&mut mustg, gc);
        }
        group::add_card(&mut sg, &mustg);
        let (p, sfhchk) = material_chooser(d, tp, &mg, k.contact);
        let x = RepCtx { fun1: &f1, funs: &funs, tp, fc: c, sub2: k.sub, contact: k.contact, sumtype: k.sumtype, chkf: k.chkf };
        while (sg.len() as i64) < maxc + funs.len() as i64 {
            let mut cg = CardSet::new();
            for c2 in group::iter(d, &group::difference(&mg, &sg)) {
                if ygo_core::api::check_matching_with(d, c2, |d| select_mix_rep(d, &x, c2, &mg, &mut sg, &mustg, k.sub, minc, maxc)) {
                    cg.insert(c2);
                }
            }
            if cg.is_empty() {
                break;
            }
            let slots: Vec<usize> = (0..funs.len()).collect();
            let finish = check_mix_rep_goal(d, &x, &sg.clone(), k.sub, minc, maxc, &slots)
                && opt_int(d, vars::CHECK_EXACT).is_none()
                && !opt_int(d, vars::CHECK_MIN).is_some_and(|m| (sg.len() as i64) < m);
            let cancel = k.cancelable && sg.is_empty();
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FMATERIAL as u64);
            let Some(tc) = group::select_unselect(d, &cg, Some(&sg), p, finish, cancel, None, None) else { break };
            if mustg.is_empty() || !mustg.contains(tc) {
                if !sg.contains(tc) {
                    sg.insert(tc);
                } else {
                    sg.remove(tc);
                }
            }
        }
        if sfhchk {
            duel::shuffle_hand(d, tp);
        }
        duel::set_fusion_material(d, &sg);
        set_summon_effect(d, Value::Nil);
    })
}

// ---------------------------------------------------------------------------------------------
// The other forms (all `AddProcMix`/`AddProcMixRep`)

/// `Fusion.AddProcMixN(c, sub, insf, mat1, n1, mat2, n2, ...)`: `n` materials of each
pub fn add_proc_mix_n(d: &mut Duel, c: CardId, sub: bool, insf: bool, mats: Vec<(Mat, usize)>) -> Option<EffectId> {
    if card::is_status(d, c, STATUS_COPYING_EFFECT) {
        return None;
    }
    let mut fun = Vec::new();
    for (m, n) in mats {
        for _ in 0..n {
            fun.push(m.clone());
        }
    }
    add_proc_mix(d, c, sub, insf, fun)
}
/// `Fusion.AddProcCode2(c, code1, code2, sub, insf)`
pub fn add_proc_code2(d: &mut Duel, c: CardId, code1: u32, code2: u32, sub: bool, insf: bool) -> Option<EffectId> {
    add_proc_mix(d, c, sub, insf, vec![Mat::Code(code1), Mat::Code(code2)])
}
pub fn add_proc_code3(d: &mut Duel, c: CardId, code1: u32, code2: u32, code3: u32, sub: bool, insf: bool) -> Option<EffectId> {
    add_proc_mix(d, c, sub, insf, vec![Mat::Code(code1), Mat::Code(code2), Mat::Code(code3)])
}
pub fn add_proc_code4(d: &mut Duel, c: CardId, code1: u32, code2: u32, code3: u32, code4: u32, sub: bool, insf: bool) -> Option<EffectId> {
    add_proc_mix(d, c, sub, insf, vec![Mat::Code(code1), Mat::Code(code2), Mat::Code(code3), Mat::Code(code4)])
}
/// `Fusion.AddProcCodeRep(c, code1, cc, sub, insf)`
pub fn add_proc_code_rep(d: &mut Duel, c: CardId, code1: u32, cc: usize, sub: bool, insf: bool) -> Option<EffectId> {
    add_proc_mix(d, c, sub, insf, vec![Mat::Code(code1); cc])
}
/// `Fusion.AddProcCodeRep2(c, code1, minc, maxc, sub, insf)`
pub fn add_proc_code_rep2(d: &mut Duel, c: CardId, code1: u32, minc: usize, maxc: usize, sub: bool, insf: bool) -> Option<EffectId> {
    add_proc_mix_rep(d, c, sub, insf, Mat::Code(code1), minc, maxc, vec![])
}
/// `Fusion.AddProcCodeFun(c, code1, f, cc, sub, insf)`
pub fn add_proc_code_fun(d: &mut Duel, c: CardId, code1: u32, f: FusMat, cc: usize, sub: bool, insf: bool) -> Option<EffectId> {
    let mut v = vec![Mat::Code(code1)];
    v.extend(std::iter::repeat_n(Mat::F(f), cc));
    add_proc_mix(d, c, sub, insf, v)
}
/// `Fusion.AddProcFun2(c, f1, f2, insf)`
pub fn add_proc_fun2(d: &mut Duel, c: CardId, f1: FusMat, f2: FusMat, insf: bool) -> Option<EffectId> {
    add_proc_mix(d, c, false, insf, vec![Mat::F(f1), Mat::F(f2)])
}
/// `Fusion.AddProcFunRep(c, f, cc, insf)`
pub fn add_proc_fun_rep(d: &mut Duel, c: CardId, f: FusMat, cc: usize, insf: bool) -> Option<EffectId> {
    add_proc_mix(d, c, false, insf, vec![Mat::F(f); cc])
}
/// `Fusion.AddProcFunRep2(c, f, minc, maxc, insf)`
pub fn add_proc_fun_rep2(d: &mut Duel, c: CardId, f: FusMat, minc: usize, maxc: usize, insf: bool) -> Option<EffectId> {
    add_proc_mix_rep(d, c, false, insf, Mat::F(f), minc, maxc, vec![])
}
/// `Fusion.AddProcFunFun(c, f1, f2, cc, insf)`
pub fn add_proc_fun_fun(d: &mut Duel, c: CardId, f1: FusMat, f2: FusMat, cc: usize, insf: bool) -> Option<EffectId> {
    let mut v = vec![Mat::F(f1)];
    v.extend(std::iter::repeat_n(Mat::F(f2), cc));
    add_proc_mix(d, c, false, insf, v)
}
/// `Fusion.AddProcFunFunRep(c, f1, f2, minc, maxc, insf)`
pub fn add_proc_fun_fun_rep(d: &mut Duel, c: CardId, f1: FusMat, f2: FusMat, minc: usize, maxc: usize, insf: bool) -> Option<EffectId> {
    add_proc_mix_rep(d, c, false, insf, Mat::F(f2), minc, maxc, vec![Mat::F(f1)])
}
/// `Fusion.AddProcCodeFunRep(c, code1, f, minc, maxc, sub, insf)`
#[allow(clippy::too_many_arguments)]
pub fn add_proc_code_fun_rep(d: &mut Duel, c: CardId, code1: u32, f: FusMat, minc: usize, maxc: usize, sub: bool, insf: bool) -> Option<EffectId> {
    add_proc_mix_rep(d, c, sub, insf, Mat::F(f), minc, maxc, vec![Mat::Code(code1)])
}
/// `Fusion.AddProcCode2FunRep(c, code1, code2, f, minc, maxc, sub, insf)`
#[allow(clippy::too_many_arguments)]
pub fn add_proc_code2_fun_rep(d: &mut Duel, c: CardId, code1: u32, code2: u32, f: FusMat, minc: usize, maxc: usize, sub: bool, insf: bool) -> Option<EffectId> {
    add_proc_mix_rep(d, c, sub, insf, Mat::F(f), minc, maxc, vec![Mat::Code(code1), Mat::Code(code2)])
}

// ---------------------------------------------------------------------------------------------
// Contact Fusion

/// `group` of a contact Fusion: `function(tp)` returning the materials
pub type ContactGroup = Arc<dyn Fn(&mut Duel, u8) -> CardSet + Send + Sync>;
/// `op` of a contact Fusion: `function(g, tp, c)`
pub type ContactOp = Arc<dyn Fn(&mut Duel, &CardSet, u8, CardId) + Send + Sync>;
/// `condition` of a contact Fusion: `function(tp)`
pub type ContactCond = Arc<dyn Fn(&mut Duel, u8) -> bool + Send + Sync>;
/// The arguments of `Fusion.AddContactProc` (named or positional in Lua).
#[derive(Clone, Default)]
pub struct Contact {
    pub material_filter: Option<ContactGroup>,
    pub material_operation: Option<ContactOp>,
    /// `sumcon`: `Some(None)` for `true` (no value), `Some(Some(f))` for a function
    pub sumcon: Option<Option<LuaFn>>,
    pub condition: Option<ContactCond>,
    pub sumtype: Option<u32>,
    pub desc: Option<u64>,
    /// `cannotBeLizard` (`None`: as `true`)
    pub cannot_be_lizard: Option<bool>,
    pub summon_to_player: Option<u8>,
    pub summon_to_zones: Option<u32>,
}
/// `Fusion.CreateContactProc(c, group, op, sumcon, condition, sumtype, desc, cannotBeLizard, summonToPlayer, summonToZones)`
pub fn create_contact_proc(d: &mut Duel, c: CardId, p: Contact) -> Option<(EffectId, Option<EffectId>)> {
    if card::is_status(d, c, STATUS_COPYING_EFFECT) {
        return None;
    }
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_description(d, e1, p.desc.unwrap_or(2));
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    let mut flag = EFFECT_FLAG_UNCOPYABLE;
    effect::set_range(d, e1, LOCATION_EXTRA);
    if let Some(stp) = p.summon_to_player {
        effect::set_target_range(d, e1, POS_FACEUP, stp as u32);
        flag |= EFFECT_FLAG_SPSUM_PARAM;
    }
    effect::set_property(d, e1, flag, 0);
    if let Some(zones) = p.summon_to_zones {
        let sumtype = p.sumtype.unwrap_or(0);
        effect::set_value_fn(d, e1, LuaFn::new(move |_, _| Value::Multi(vec![Value::Int(sumtype as i64), Value::Int(zones as i64), Value::Bool(true)])));
    } else if let Some(st) = p.sumtype {
        effect::set_value(d, e1, st as i64);
    }
    let group = p.material_filter.clone().unwrap_or_else(|| crate::lua_error("attempt to call a nil value (upvalue 'f')"));
    effect::set_condition(d, e1, Some(contact_con(group.clone(), p.condition.clone(), p.summon_to_player, p.summon_to_zones)));
    effect::set_target(d, e1, Some(contact_tg(group, p.summon_to_player, p.summon_to_zones)));
    effect::set_operation(d, e1, Some(contact_op(p.material_operation.clone())));
    let mut e2 = None;
    if let Some(sumcon) = &p.sumcon {
        // spsummon condition
        let e = effect::create_effect(d, c);
        effect::set_type(d, e, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
        effect::set_code(d, e, EFFECT_SPSUMMON_CONDITION);
        if let Some(f) = sumcon {
            effect::set_value_fn(d, e, f.clone());
        }
        e2 = Some(e);
    }
    // lizard check
    if p.cannot_be_lizard != Some(false) {
        aux::add_lizard_check(d, c);
    }
    Some((e1, e2))
}
/// `Fusion.AddContactProc(c, ...)`
pub fn add_contact_proc(d: &mut Duel, c: CardId, p: Contact) {
    if let Some((e1, e2)) = create_contact_proc(d, c, p) {
        card::register_effect(d, c, e1, false);
        if let Some(e2) = e2 {
            card::register_effect(d, c, e2, false);
        }
    }
}
fn contact_chkf(chkp: u8, zones: Option<u32>) -> u64 {
    chkp as u64 | FUSPROC_CONTACTFUS as u64 | ((zones.unwrap_or(0) as u64) << 40)
}
/// `Fusion.ContactCon(f, fcon, summonToPlayer, summonToZones)`: `function(e, c)`
fn contact_con(f: ContactGroup, fcon: Option<ContactCond>, to_player: Option<u8>, zones: Option<u32>) -> LuaFn {
    lua_fn(move |d, a| {
        let e = a.e(0);
        let Some(c) = a.card(1) else { return true };
        let hp = effect::get_handler_player(d, e);
        let m = f(d, hp);
        let cp = card::get_controler(d, c);
        let chkp = if to_player == Some(1) { 1 - cp } else { cp };
        card::is_facedown(d, c) && card::check_fusion_material(d, c, Some(&m), None, Some(contact_chkf(chkp, zones))) && fcon.as_ref().is_none_or(|fc| fc(d, hp))
    })
}
/// `Fusion.ContactTg(f, summonToPlayer, summonToZones)`
fn contact_tg(f: ContactGroup, to_player: Option<u8>, zones: Option<u32>) -> LuaFn {
    lua_fn(move |d, a| {
        let (e, tp) = (a.e(0), a.p(1));
        let m = f(d, tp);
        let chkp = if to_player == Some(1) { 1 - tp } else { tp };
        let h = effect::get_handler(d, e).nil();
        let sg = duel::select_fusion_material(d, tp, h, &m, None, Some(contact_chkf(chkp, zones)));
        if !sg.is_empty() {
            effect::set_label_object(d, e, Value::Group(sg));
            true
        } else {
            false
        }
    })
}
/// `Fusion.ContactOp(f)`: `function(e,tp,eg,ep,ev,re,r,rp,c)`
fn contact_op(f: Option<ContactOp>) -> LuaFn {
    lua_fn(move |d, a| {
        let (e, tp) = (a.e(0), a.p(1));
        let c = a.c(8);
        let g = effect::get_label_object(d, e).as_group().cloned().unwrap_or_default();
        card::set_material(d, c, g.clone());
        match &f {
            Some(f) => f(d, &g, tp, c),
            None => crate::lua_error("attempt to call a nil value (upvalue 'f')"),
        }
    })
}
/// `Fusion.AddUnionFusionProc(c)`
pub fn add_union_fusion_proc(d: &mut Duel, c: CardId) {
    add_contact_proc(
        d,
        c,
        Contact {
            material_filter: Some(Arc::new(union_fus_filter)),
            material_operation: Some(Arc::new(union_fus_op)),
            sumtype: Some(SUMMON_TYPE_FUSION),
            cannot_be_lizard: Some(false),
            ..Default::default()
        },
    );
}
/// `Fusion.UnionFusFilter(tp)`
pub fn union_fus_filter(d: &mut Duel, tp: u8) -> CardSet {
    duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_able_to_deck_or_extra_as_cost(d, c))), tp, LOCATION_ONFIELD, 0, NO_EX)
}
/// `Fusion.UnionFusOp(g, tp)`
pub fn union_fus_op(d: &mut Duel, g: &CardSet, tp: u8, _c: CardId) {
    duel::send_to_deck(d, g, None, SEQ_DECKBOTTOM as i32, REASON_COST + REASON_MATERIAL, None);
    let og = duel::get_operated_group(d);
    let og = group::filter(d, &og, &|d, x| card::is_location(d, x, LOCATION_DECK), NO_EX);
    if og.len() > 1 {
        duel::sort_deckbottom(d, tp, tp, og.len() as u16);
    }
}

pub use super::fusion_spell::*;
