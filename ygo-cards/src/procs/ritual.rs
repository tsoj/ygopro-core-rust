//! `Ritual.*` (`proc_ritual.lua`): Ritual Summons.

use crate::aux;
use crate::constants::*;
use crate::lua::{card, duel, effect, group, NO_EX};
use crate::prelude::*;
use std::sync::Arc;

/// `lv`: a fixed Level, or `function(c)` (default: the monster's Level)
#[derive(Clone)]
pub enum Lv {
    N(i64),
    F(Arc<dyn Fn(&mut Duel, CardId) -> i64 + Send + Sync>),
}
/// `extrafil(e, tp, eg, ep, ev, re, r, rp, chk)`: more materials
pub type ExtraFil = Arc<dyn Fn(&mut Duel, &EffArgs) -> Option<CardSet> + Send + Sync>;
/// `extraop`/`stage2`/`customoperation`: `function(mat, e, tp, eg, ep, ev, re, r, rp, tc)`
pub type MatOp = Arc<dyn Fn(&mut Duel, &CardSet, &EffArgs, CardId) + Send + Sync>;
/// `matfilter`: `function(c, e, tp)`, or a group of the allowed materials
#[derive(Clone)]
pub enum MatFilt {
    F(Arc<dyn Fn(&mut Duel, CardId, EffectId, u8) -> bool + Send + Sync>),
    G(CardSet),
}
/// `forcedselection(e, tp, g, c)`: (the materials are valid, stop adding materials)
pub type Forced = Arc<dyn Fn(&mut Duel, EffectId, u8, &CardSet, CardId) -> (bool, bool) + Send + Sync>;
/// `specificmatfilter(c, rc, mg, tp)`
pub type SpecificMatFilter = Arc<dyn Fn(&mut Duel, CardId, CardId, &CardSet, u8) -> bool + Send + Sync>;
/// `requirementfunc(c, rc)`: the Level a material counts for (default `Card.GetRitualLevel`)
pub type ReqFn = Arc<dyn Fn(&mut Duel, CardId, CardId) -> i64 + Send + Sync>;

/// The parameters of `Ritual.CreateProc` (named or positional in Lua).
#[derive(Clone, Default)]
pub struct RitualProc {
    /// `_type`/`lvtype`: RITPROC_GREATER or RITPROC_EQUAL
    pub lvtype: u32,
    pub filter: Option<CardFilter>,
    pub lv: Option<Lv>,
    pub desc: Option<u64>,
    pub extrafil: Option<ExtraFil>,
    pub extraop: Option<MatOp>,
    pub matfilter: Option<MatFilt>,
    pub stage2: Option<MatOp>,
    pub location: Option<u32>,
    pub forcedselection: Option<Forced>,
    pub customoperation: Option<MatOp>,
    pub specificmatfilter: Option<SpecificMatFilter>,
    pub requirementfunc: Option<ReqFn>,
    pub sumpos: Option<u8>,
    pub extratg: Option<LuaFn>,
    /// `self`: the effect Summons its own card
    pub self_: bool,
}

/// `Ritual.SummoningLevel` (the Level the monster being Summoned needs)
pub const SUMMONING_LEVEL: (u32, u64) = (crate::ns::AUX, 500);
fn set_summoning_level(d: &mut Duel, lv: Option<i64>) {
    d.set_script_var(SUMMONING_LEVEL, lv.map_or(Value::Nil, Value::Int));
}

/// `Ritual.GetMatchingFilterFunction(c)`
pub fn get_matching_filter_function(d: &Duel, c: CardId) -> Option<LuaFn> {
    crate::card_var::get(d, c, crate::card_var::RITUAL_MATCHING_FUNCTION).as_fn().cloned()
}
/// `Ritual.CheckMatFilter(matfilter, e, tp, mg, mg2)`
fn check_mat_filter(d: &mut Duel, matfilter: &Option<MatFilt>, e: EffectId, tp: u8, mg: &mut CardSet, mg2: &mut CardSet) {
    match matfilter {
        Some(MatFilt::F(f)) => {
            group::remove(d, mg, &|d, c| !f(d, c, e, tp), NO_EX);
            group::remove(d, mg2, &|d, c| !f(d, c, e, tp), NO_EX);
        }
        Some(MatFilt::G(g)) => {
            group::remove(d, mg, &|_, c| !g.contains(c), NO_EX);
            group::remove(d, mg2, &|_, c| !g.contains(c), NO_EX);
        }
        None => {}
    }
}
/// `Ritual.AddWholeLevelTribute(c, condition)`: the card counts for the whole Level (`cond(c, e)`)
pub fn add_whole_level_tribute(d: &mut Duel, c: CardId, cond: Arc<dyn Fn(&mut Duel, CardId, EffectId) -> bool + Send + Sync>) -> EffectId {
    let e = effect::create_effect(d, c);
    effect::set_type(d, e, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e, EFFECT_RITUAL_LEVEL);
    effect::set_value_fn(d, e, whole_level_tribute_value(cond));
    card::register_effect(d, c, e, false);
    e
}
/// `Ritual.WholeLevelTributeValue(cond)`
pub fn whole_level_tribute_value(cond: Arc<dyn Fn(&mut Duel, CardId, EffectId) -> bool + Send + Sync>) -> LuaFn {
    effect_card_value(move |d, e, c| {
        let h = effect::get_handler(d, e).nil();
        let lv = card::get_level(d, h) as i64;
        if cond(d, c, e) {
            let clv = match d.script_var(SUMMONING_LEVEL) {
                Value::Nil => card::get_level(d, c) as i64,
                v => v.int(),
            };
            (lv << 16) | clv
        } else {
            lv
        }
    })
}
/// `Ritual.CreateProc(c, ...)`: an Activate effect that Ritual Summons
pub fn create_proc(d: &mut Duel, c: CardId, p: RitualProc) -> EffectId {
    if let Some(f) = &p.filter {
        let f = f.clone();
        crate::card_var::set(d, c, crate::card_var::RITUAL_MATCHING_FUNCTION, Value::Fn(LuaFn::new(move |d, a| Value::Bool(f(d, a[0].as_card().nil())))));
    }
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, p.desc.unwrap_or(1171));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    let tg = target(d, p.clone());
    effect::set_target(d, e1, Some(tg));
    let op = operation(d, p);
    effect::set_operation(d, e1, Some(op));
    e1
}
/// `Ritual.AddProc(c, ...)`
pub fn add_proc(d: &mut Duel, c: CardId, p: RitualProc) -> EffectId {
    let e1 = create_proc(d, c, p);
    card::register_effect(d, c, e1, false);
    e1
}
/// `Ritual.AddProcGreater(c, ...)`
pub fn add_proc_greater(d: &mut Duel, c: CardId, p: RitualProc) -> EffectId {
    add_proc(d, c, RitualProc { lvtype: RITPROC_GREATER, ..p })
}
/// `Ritual.AddProcEqual(c, ...)`
pub fn add_proc_equal(d: &mut Duel, c: CardId, p: RitualProc) -> EffectId {
    add_proc(d, c, RitualProc { lvtype: RITPROC_EQUAL, ..p })
}
/// `Ritual.AddProcCode(c, _type, lv, desc, codes...)`: for the monsters `codes`
pub fn add_proc_code(d: &mut Duel, c: CardId, lvtype: u32, lv: Option<Lv>, desc: Option<u64>, codes: &'static [u32]) -> EffectId {
    if !card::is_status(d, c, STATUS_COPYING_EFFECT) && crate::class::get(d, c, crate::class::FIT_MONSTER).is_nil() && crate::card_info(d, c).fit_monster.is_empty() {
        crate::class::set(d, c, crate::class::FIT_MONSTER, Value::Multi(codes.iter().map(|&x| Value::Int(x as i64)).collect()));
    }
    add_proc(d, c, RitualProc { lvtype, filter: Some(Arc::new(move |d, x| card::is_code(d, x, codes))), lv, desc, ..Default::default() })
}
/// `Ritual.AddProcGreaterCode(c, lv, desc, codes...)`
pub fn add_proc_greater_code(d: &mut Duel, c: CardId, lv: Option<Lv>, desc: Option<u64>, codes: &'static [u32]) -> EffectId {
    add_proc_code(d, c, RITPROC_GREATER, lv, desc, codes)
}
/// `Ritual.AddProcEqualCode(c, lv, desc, codes...)`
pub fn add_proc_equal_code(d: &mut Duel, c: CardId, lv: Option<Lv>, desc: Option<u64>, codes: &'static [u32]) -> EffectId {
    add_proc_code(d, c, RITPROC_EQUAL, lv, desc, codes)
}

/// `MergeForcedSelection(f1, f2)`
fn merge_forced(f1: Option<Forced>, f2: Option<Forced>) -> Option<Forced> {
    match (f1, f2) {
        (None, f) | (f, None) => f,
        (Some(a), Some(b)) => Some(Arc::new(move |d, e, tp, g, c| {
            let (r1, r2) = a(d, e, tp, g, c);
            let (p1, p2) = b(d, e, tp, g, c);
            (r1 && p1, r2 || p2)
        })),
    }
}
fn custom_check(d: &Duel, c: CardId) -> Option<Forced> {
    crate::card_info(d, c).ritual_custom_check.map(|f| Arc::new(f) as Forced)
}
/// `aux.RitualCheckAdditionalLevel(c, rc)`
pub fn ritual_check_additional_level(d: &mut Duel, c: CardId, rc: CardId) -> i64 {
    let raw = card::get_ritual_level(d, c, rc) as i64;
    let (lv1, lv2) = (raw & 0xffff, raw >> 16);
    if lv2 > 0 { lv1.min(lv2) } else { lv1 }
}
fn level_of(d: &mut Duel, lv: &Option<Lv>, c: CardId) -> i64 {
    match lv {
        Some(Lv::N(n)) => *n,
        Some(Lv::F(f)) => f(d, c),
        None => card::get_level(d, c) as i64,
    }
}
fn req_or(d: &mut Duel, req: &Option<ReqFn>, c: CardId, sc: CardId, additional: bool) -> i64 {
    match req {
        Some(r) => r(d, c, sc),
        None if additional => ritual_check_additional_level(d, c, sc),
        None => card::get_ritual_level(d, c, sc) as i64,
    }
}
fn sum_check(d: &mut Duel, sg: &CardSet, req: &Option<ReqFn>, lv: i64, sc: CardId, lvtype: u32) -> bool {
    let f = |d: &mut Duel, c: CardId| req_or(d, req, c, sc, false);
    if lvtype == RITPROC_EQUAL {
        let n = sg.len() as i32;
        group::check_with_sum_equal(d, sg, &f, lv as u32, n, n).0
    } else {
        duel::set_selected_card(d, sg);
        group::check_with_sum_greater(d, sg, &f, lv as u32).0
    }
}
/// `Ritual.Check(sc, lv, forcedselection, _type, requirementfunc)` as a rescon
fn check(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, c: Option<CardId>, sc: CardId, lv: i64, forced: &Option<Forced>, lvtype: u32, req: &Option<ReqFn>) -> (bool, bool) {
    let sum = group::get_sum(d, sg, &|d, x| req_or(d, req, x, sc, true));
    let res = if lvtype == RITPROC_EQUAL {
        sum <= lv
    } else {
        let cc = c.nil();
        sum - req_or(d, req, cc, sc, true) <= lv
    };
    if !res {
        return (false, true);
    }
    let mut res = res;
    let mut stop = false;
    if let Some(f) = forced {
        let (r, s) = f(d, e.nil(), tp, sg, sc);
        res = r;
        stop = s || stop;
    }
    if res && !stop {
        res = sum_check(d, sg, req, lv, sc, lvtype);
        res = res
            && if card::is_location(d, sc, LOCATION_EXTRA) {
                duel::get_location_count_from_ex(d, tp, Some(tp), sg.into(), duel::ExtraCard::Card(sc), None).0 > 0
            } else {
                duel::get_m_zone_count(d, tp, sg.into(), Some(tp), None, None).0 > 0
            };
    }
    (res, stop)
}
/// `mg:Match(specificmatfilter, nil, c, mg, tp)`: the core removes the cards one by one (in card
/// id order), so the filter sees `mg` without the cards removed before
fn match_specific(d: &mut Duel, mg: &mut CardSet, f: &SpecificMatFilter, c: CardId, tp: u8) {
    for x in mg.to_vec() {
        let snapshot = mg.clone();
        if !crate::lua::protected(d, false, |d| f(d, x, c, &snapshot, tp)) {
            mg.remove(x);
        }
    }
}
/// `Ritual.Filter(c, filter, _type, e, tp, m, m2, forcedselection, specificmatfilter, lv, requirementfunc, sumpos)`
#[allow(clippy::too_many_arguments)]
fn ritual_filter(d: &mut Duel, c: CardId, p: &RitualProc, e: EffectId, tp: u8, m: &CardSet, m2: &CardSet, forced: &Option<Forced>, sumpos: u8) -> bool {
    if !(card::is_original_type(d, c, TYPE_RITUAL) && card::is_original_type(d, c, TYPE_MONSTER))
        || p.filter.as_ref().is_some_and(|f| !f(d, c))
        || !card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_RITUAL, tp, false, true, Some(sumpos), None, None)
    {
        return false;
    }
    let lv = level_of(d, &p.lv, c).max(1);
    set_summoning_level(d, Some(lv));
    let mut mg = group::filter(d, m, &|d, x| card::is_can_be_ritual_material(d, x, Some(c), None), c.into());
    group::add_card(&mut mg, &group::difference(m2, &CardSet::from_card(c)));
    if let Some(mf) = crate::card_info(d, c).mat_filter {
        group::match_(d, &mut mg, &|d, x| mf(d, x, tp), c.into());
    }
    if let Some(sf) = &p.specificmatfilter {
        match_specific(d, &mut mg, sf, c, tp);
    }
    let forced = merge_forced(custom_check(d, c), forced.clone());
    let (lvtype, req) = (p.lvtype, p.requirementfunc.clone());
    let rescon = |d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, cc: Option<CardId>| check(d, sg, e, tp, cc, c, lv, &forced, lvtype, &req);
    let res = matches!(aux::select_unselect_group(d, &mg, Some(e), tp, Some(1), Some(lv as usize), Some(&rescon), 0, tp, None, None, None, false), Err(true));
    set_summoning_level(d, None);
    res
}
fn default_location(d: &Duel) -> u32 {
    if duel::is_duel_type(d, DUEL_EXTRA_DECK_RITUAL as u64) { LOCATION_EXTRA } else { LOCATION_HAND }
}
/// The forced selections of EFFECT_EXTRA_RITUAL_MATERIAL effects (their label object `{f}`), added
/// to `func` (each function once in a row)
fn merge_extra_ritual_forced(d: &mut Duel, mg: &CardSet, mut func: Option<Forced>) -> Option<Forced> {
    let extra_eff_g = group::filter(d, mg, &|d, c| !card::is_has_effect(d, c, EFFECT_EXTRA_RITUAL_MATERIAL, None).is_empty(), NO_EX);
    let mut prev: Option<LuaFn> = None;
    for tmp_c in group::iter(d, &extra_eff_g) {
        for eff in card::is_has_effect(d, tmp_c, EFFECT_EXTRA_RITUAL_MATERIAL, None) {
            if let Value::Multi(t) = effect::get_label_object(d, eff) {
                if let Some(Value::Fn(f)) = t.first() {
                    if !prev.as_ref().is_some_and(|p| p.ptr_eq(f)) {
                        prev = Some(f.clone());
                        let f = f.clone();
                        let forced: Forced = Arc::new(move |d, e, tp, g, c| {
                            let r = f.call(d, &[Value::Effect(e), Value::Int(tp as i64), Value::Group(g.clone()), Value::Card(c)]);
                            match r {
                                Value::Multi(v) => (v.first().is_some_and(|x| x.truthy()), v.get(1).is_some_and(|x| x.truthy())),
                                v => (v.truthy(), false),
                            }
                        });
                        func = merge_forced(func, Some(forced));
                    }
                }
            }
        }
    }
    func
}
fn force_extra_release(d: &mut Duel, mg: &CardSet, tp: u8, func: Option<Forced>) -> Option<Forced> {
    let extra_mat_g = group::filter(d, mg, &|d, c| card::is_controler(d, c, 1 - tp) && !card::is_has_effect(d, c, EFFECT_EXTRA_RELEASE, None).is_empty(), NO_EX);
    if extra_mat_g.is_empty() {
        return func;
    }
    merge_forced(Some(Arc::new(move |_d, _e, _tp, g, _c| (group::includes(g, &extra_mat_g), false))), func)
}
/// `Ritual.Target(...)`: the target of a Ritual Summon effect (`p.lvtype` etc. as for `Ritual.CreateProc`)
pub fn target(d: &mut Duel, p: RitualProc) -> LuaFn {
    let location = p.location.unwrap_or_else(|| default_location(d));
    let sumpos = p.sumpos.unwrap_or(POS_FACEUP as u8);
    cost_target(move |d, a| {
        let (e, tp) = (a.e, a.tp);
        if a.chk == Some(0) {
            let mut mg = duel::get_ritual_material(d, tp, p.requirementfunc.is_none());
            let mut mg2 = p.extrafil.as_ref().and_then(|f| f(d, a)).unwrap_or_default();
            let func = force_extra_release(d, &mg, tp, p.forcedselection.clone());
            let func = merge_extra_ritual_forced(d, &mg, func);
            check_mat_filter(d, &p.matfilter, e, tp, &mut mg, &mut mg2);
            return duel::is_existing_matching_card(d, Some(&|d, c| ritual_filter(d, c, &p, e, tp, &mg, &mg2, &func, sumpos)), tp, location, 0, 1, NO_EX);
        }
        if let Some(et) = &p.extratg {
            et.call(d, &[Value::Effect(e), Value::Int(tp as i64), a.eg.clone().map_or(Value::Nil, Value::Group), Value::Int(a.ep as i64), Value::Int(a.ev as i64), Value::effect(a.re), Value::Int(a.r as i64), Value::Int(a.rp as i64), a.chk.map_or(Value::Nil, Value::Int)]);
        }
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, location as i32);
        true
    })
}
/// `Ritual.Operation(...)`: the operation of a Ritual Summon effect
pub fn operation(d: &mut Duel, p: RitualProc) -> LuaFn {
    let location = p.location.unwrap_or_else(|| default_location(d));
    let sumpos = p.sumpos.unwrap_or(POS_FACEUP as u8);
    crate::prelude::operation(move |d, a| {
        let (e, tp) = (a.e, a.tp);
        let c = effect::get_handler(d, e).nil();
        if p.self_ && !card::is_relate_to_effect(d, c, e) {
            return;
        }
        let mut mg = duel::get_ritual_material(d, tp, p.requirementfunc.is_none());
        let mut mg2 = p.extrafil.as_ref().and_then(|f| f(d, a)).unwrap_or_default();
        let func = merge_extra_ritual_forced(d, &mg, p.forcedselection.clone());
        let mut func = force_extra_release(d, &mg, tp, func);
        check_mat_filter(d, &p.matfilter, e, tp, &mut mg, &mut mg2);
        let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
        let mut tg = CardSet::new();
        if !p.self_ {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            let (mgr, mg2r, funcr) = (&mg, &mg2, &func);
            let f = aux::necro_valley_filter(|d, x| ritual_filter(d, x, &p, e, tp, mgr, mg2r, funcr, sumpos));
            tg = duel::select_matching_card(d, tp, Some(&f), tp, location, 0, 1, 1, false, NO_EX).nil();
        } else if ritual_filter(d, c, &p, e, tp, &mg, &mg2, &func, sumpos) && card::is_has_effect(d, c, EFFECT_NECRO_VALLEY, None).is_empty() {
            tg.insert(c);
        }
        if tg.is_empty() {
            return;
        }
        let tc = group::get_first(d, &tg).nil();
        let lv = level_of(d, &p.lv, tc).max(1);
        set_summoning_level(d, Some(lv));
        group::match_(d, &mut mg, &|d, x| card::is_can_be_ritual_material(d, x, Some(tc), None), tc.into());
        group::add_card(&mut mg, &group::difference(&mg2, &CardSet::from_card(tc)));
        if let Some(sf) = &p.specificmatfilter {
            match_specific(d, &mut mg, sf, tc, tp);
        }
        func = merge_forced(custom_check(d, tc), func);
        if let Some(mf) = crate::card_info(d, tc).mat_filter {
            group::match_(d, &mut mg, &|d, x| mf(d, x, tp), tc.into());
        }
        let req = p.requirementfunc.clone();
        let mat = if ft > 0 && func.is_none() {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
            let f = |d: &mut Duel, x: CardId| req_or(d, &req, x, tc, false);
            if p.lvtype == RITPROC_EQUAL {
                group::select_with_sum_equal(d, &mg, tp, &f, lv as u32, 1, mg.len() as i32)
            } else {
                group::select_with_sum_greater(d, &mg, tp, &f, lv as u32)
            }
        } else {
            let lvtype = p.lvtype;
            let rescon = |d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, cc: Option<CardId>| check(d, sg, e, tp, cc, tc, lv, &func, lvtype, &req);
            let finishcon = |d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>| {
                if let Some(f) = &func {
                    if !f(d, e.nil(), tp, sg, tc).0 {
                        return (false, false);
                    }
                }
                (sum_check(d, sg, &req, lv, tc, lvtype), false)
            };
            aux::select_unselect_group(d, &mg, Some(e), tp, Some(1), Some(lv as usize), Some(&rescon), 1, tp, Some(HINTMSG_RELEASE as u64), Some(&finishcon), None, false).ok()
        };
        let mat = mat.nil();
        // (a material of a once-per-turn EFFECT_EXTRA_RITUAL_MATERIAL effect)
        let extra_eff_g = group::filter(d, &mat, &|d, x| !card::is_has_effect(d, x, EFFECT_EXTRA_RITUAL_MATERIAL, None).is_empty(), NO_EX);
        for tmp_c in group::iter(d, &extra_eff_g) {
            for eff in card::is_has_effect(d, tmp_c, EFFECT_EXTRA_RITUAL_MATERIAL, None) {
                let max_count_limit = effect::get_count_limit(d, eff).1;
                if max_count_limit > 0 && !mg2.contains(tmp_c) {
                    effect::use_count_limit(d, eff, tp, 1, false);
                    let h = effect::get_handler(d, eff).nil();
                    let code = card::get_code(d, h)[0];
                    duel::register_flag_effect(d, tp, code, RESET_PHASE + PHASE_END, 0, 1, 0);
                }
            }
        }
        if let Some(co) = &p.customoperation {
            co(d, &mat, a, tc);
        } else {
            card::set_material(d, tc, mat.clone());
            match &p.extraop {
                Some(op) => op(d, &mat, a, tc),
                None => duel::release_ritual_material(d, &mat, false),
            }
            duel::break_effect(d);
            duel::special_summon(d, tc, SUMMON_TYPE_RITUAL, tp, tp, false, true, sumpos, None);
            card::complete_procedure(d, tc);
            if card::is_facedown(d, tc) {
                duel::confirm_cards(d, 1 - tp, tc);
            }
            if let Some(s2) = &p.stage2 {
                s2(d, &mat, a, tc);
            }
        }
        set_summoning_level(d, None);
    })
}
