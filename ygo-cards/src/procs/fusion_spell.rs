//! `proc_fusion_spell.lua`: Fusion Summon effects (`Fusion.CreateSummonEff`, `Fusion.SummonEffTG`,
//! `Fusion.SummonEffOP`, ...). (Exported as `fusion::*`.)

use crate::constants::*;
use crate::global::vars::{FUSION_EXTRA_GROUP, FUSION_GEFF};
use crate::lua::{card, duel, effect, group, NO_EX};
use crate::prelude::*;
use std::sync::{Arc, LazyLock};

use super::fusion::{set_check_additional, set_check_exact, set_check_max, set_check_min};

/// `fusfilter(c, tp)`: the Fusion Monsters the effect can Summon
pub type FusFilter = Arc<dyn Fn(&mut Duel, CardId, u8) -> bool + Send + Sync>;
/// `matfilter(c, e, tp, chk)`: the materials it can use (default `Card.IsAbleToGrave`)
pub type MatFilterFn = Arc<dyn Fn(&mut Duel, CardId, EffectId, u8, i64) -> bool + Send + Sync>;
/// `extrafil(e, tp, mg)`: more materials, and a check of all the materials (`Fusion.CheckAdditional`)
pub type ExtraFil = Arc<dyn Fn(&mut Duel, EffectId, u8, &CardSet) -> (Option<CardSet>, Option<LuaFn>) + Send + Sync>;
/// `stage2(e, tc, tp, mat, stage)`
pub type Stage2 = Arc<dyn Fn(&mut Duel, EffectId, Option<CardId>, u8, Option<&CardSet>, i64) + Send + Sync>;
/// `preselect(e, tc)`: `false` stops the effect
pub type Preselect = Arc<dyn Fn(&mut Duel, EffectId, CardId) -> bool + Send + Sync>;
/// `gc`: the material the effect must use (`function(e, tp, eg, ep, ev, re, r, rp, chk)` returning a
/// card or a group)
pub type Gc = Arc<dyn Fn(&mut Duel, &EffArgs) -> Value + Send + Sync>;

pub fn fus_filter(f: impl Fn(&mut Duel, CardId, u8) -> bool + Send + Sync + 'static) -> FusFilter {
    Arc::new(f)
}
pub fn mat_filter_fn(f: impl Fn(&mut Duel, CardId, EffectId, u8, i64) -> bool + Send + Sync + 'static) -> MatFilterFn {
    Arc::new(f)
}
/// An `extraop` (or the operation of an EFFECT_EXTRA_FUSION_MATERIAL effect): `function(e, tc, tp, sg)`.
/// It may change `sg` (e.g. `sg:Clear()`); it returns `false` to stop the Summon (`true` for a Lua
/// function that returns nothing).
pub fn extra_op(f: impl Fn(&mut Duel, EffectId, Option<CardId>, u8, &mut CardSet) -> bool + Send + Sync + 'static) -> LuaFn {
    LuaFn::new(move |d, a| {
        let e = a[0].as_effect().nil();
        let tc = a.get(1).and_then(|v| v.as_card());
        let tp = a.get(2).map_or(0, |v| v.int()) as u8;
        match a.get(3) {
            Some(Value::SharedGroup(r)) => {
                let r = *r;
                let mut g = d.group(r).clone();
                let res = f(d, e, tc, tp, &mut g);
                *d.group_mut(r) = g;
                Value::Bool(res)
            }
            other => {
                let mut g = other.and_then(|v| v.as_group().cloned()).unwrap_or_default();
                Value::Bool(f(d, e, tc, tp, &mut g))
            }
        }
    })
}
/// Calls an `extraop`-like function on `sg` (which it may change)
fn call_extra_op(d: &mut Duel, f: &LuaFn, e: EffectId, tc: CardId, tp: u8, sg: &mut CardSet) -> Value {
    let r = d.new_group(sg.clone());
    let res = f.call(d, &[Value::Effect(e), Value::Card(tc), Value::Int(tp as i64), Value::SharedGroup(r)]);
    *sg = d.group(r).clone();
    res
}
static BANISH_MATERIAL: LazyLock<LuaFn> = LazyLock::new(|| {
    extra_op(|d, _e, _tc, _tp, sg| {
        duel::remove(d, &*sg, POS_FACEUP as u8, REASON_EFFECT + REASON_MATERIAL + REASON_FUSION, None, None);
        sg.clear();
        true
    })
});
static SHUFFLE_MATERIAL: LazyLock<LuaFn> = LazyLock::new(|| {
    extra_op(|d, _e, _tc, tp, sg| {
        let rg = group::filter(d, sg, &|d, c| card::is_facedown(d, c), NO_EX);
        let hg = group::filter(d, sg, &|d, c| card::is_location(d, c, LOCATION_GRAVE | LOCATION_REMOVED), NO_EX);
        if !rg.is_empty() {
            duel::confirm_cards(d, 1 - tp, &rg);
        }
        if !hg.is_empty() {
            duel::hint_selection(d, &hg, true);
        }
        let tg = add_maximum_check(d, sg);
        duel::send_to_deck(d, tg, None, 2, REASON_EFFECT | REASON_MATERIAL | REASON_FUSION, None);
        sg.clear();
        true
    })
});
/// `Fusion.BanishMaterial` (an `extraop`: banish the materials)
pub fn banish_material() -> LuaFn {
    BANISH_MATERIAL.clone()
}
/// `Fusion.ShuffleMaterial` (an `extraop`: shuffle the materials into the Deck)
pub fn shuffle_material() -> LuaFn {
    SHUFFLE_MATERIAL.clone()
}
/// `Group.AddMaximumCheck(g)` (proc_maximum.lua): with all the parts of Maximum Mode monsters
pub fn add_maximum_check(d: &mut Duel, g: &CardSet) -> CardSet {
    let mut out = g.clone();
    for tc in group::iter(d, g) {
        if card::is_maximum_mode(d, tc) {
            let p = card::get_controler(d, tc);
            let g2 = duel::get_matching_group(d, Some(&|d, c| card::is_maximum_mode(d, c)), p, LOCATION_MZONE, 0, tc.into());
            group::add_card(&mut out, &g2);
        }
    }
    out
}
/// `Fusion.OnFieldMat` used directly as the `matfilter` (`c:IsOnField() and c:IsAbleToGrave()`)
pub fn on_field_mat() -> MatFilterFn {
    Arc::new(|d, c, _e, _tp, _chk| card::is_on_field(d, c) && card::is_able_to_grave(d, c, None))
}
/// `Fusion.OnFieldMat(filter, ...)`: on the field and matching `filter`
pub fn on_field_mat_with(f: MatFilterFn) -> MatFilterFn {
    Arc::new(move |d, c, e, tp, chk| card::is_on_field(d, c) && f(d, c, e, tp, chk))
}
/// `Fusion.InHandMat`
pub fn in_hand_mat() -> MatFilterFn {
    Arc::new(|d, c, _e, _tp, _chk| card::is_location(d, c, LOCATION_HAND) && card::is_able_to_grave(d, c, None))
}
pub fn in_hand_mat_with(f: MatFilterFn) -> MatFilterFn {
    Arc::new(move |d, c, e, tp, chk| card::is_location(d, c, LOCATION_HAND) && f(d, c, e, tp, chk))
}
/// `Fusion.IsMonsterFilter(f)` as a `fusfilter`
pub fn is_monster_filter(f: FusFilter) -> FusFilter {
    Arc::new(move |d, c, tp| card::is_monster(d, c) && f(d, c, tp))
}
/// `Fusion.ForcedHandler` (a `gc`: the effect's own card)
pub fn forced_handler() -> Gc {
    Arc::new(|d, a| Value::card(effect::get_handler(d, a.e)))
}
/// `Fusion.CheckWithHandler(fun)` as a `matfilter`: the effect's own card, or `fun`
pub fn check_with_handler(f: MatFilterFn) -> MatFilterFn {
    Arc::new(move |d, c, e, tp, chk| effect::get_handler(d, e) == Some(c) || f(d, c, e, tp, chk))
}

/// The parameters of a Fusion Summon effect (`Fusion.CreateSummonEff`, named or positional in Lua).
#[derive(Clone, Default)]
pub struct SummonEff {
    pub fusfilter: Option<FusFilter>,
    pub matfilter: Option<MatFilterFn>,
    pub extrafil: Option<ExtraFil>,
    pub extraop: Option<LuaFn>,
    pub gc: Option<Gc>,
    pub stage2: Option<Stage2>,
    pub exactcount: Option<i64>,
    pub value: Option<u64>,
    pub location: Option<u32>,
    pub chkf: Option<u64>,
    pub desc: Option<u64>,
    pub preselect: Option<Preselect>,
    pub nosummoncheck: bool,
    pub extratg: Option<LuaFn>,
    pub mincount: Option<i64>,
    pub maxcount: Option<i64>,
    pub sumpos: Option<u8>,
}
/// `Fusion.CreateSummonEff(c, ...)`: an Activate effect that Fusion Summons
pub fn create_summon_eff(d: &mut Duel, c: CardId, p: SummonEff) -> EffectId {
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, p.desc.unwrap_or(1170));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    let tg = summon_eff_tg(d, p.clone());
    effect::set_target(d, e1, Some(tg));
    let op = summon_eff_op(d, p);
    effect::set_operation(d, e1, Some(op));
    e1
}
/// `Fusion.RegisterSummonEff(c, ...)`
pub fn register_summon_eff(d: &mut Duel, c: CardId, p: SummonEff) -> EffectId {
    let e1 = create_summon_eff(d, c, p);
    card::register_effect(d, c, e1, false);
    e1
}

/// `GetExtraMatEff(c, summon_card)`: the first EFFECT_EXTRA_FUSION_MATERIAL effect on `c` (not
/// the global one), that applies to `summon_card` if given
fn get_extra_mat_eff(d: &mut Duel, c: CardId, summon_card: Option<CardId>) -> Option<EffectId> {
    let geff = d.script_var(FUSION_GEFF).as_effect();
    for eff in card::is_has_effect(d, c, EFFECT_EXTRA_FUSION_MATERIAL, None) {
        if Some(eff) == geff {
            continue;
        }
        let Some(sc) = summon_card else { return Some(eff) };
        let ok = match effect::get_value(d, eff) {
            Value::Fn(f) => f.call(d, &[Value::Effect(eff), Value::Card(sc)]).truthy(),
            v => v.int() == 1,
        };
        if ok {
            return Some(eff);
        }
    }
    None
}
/// `ExtraMatOPTCheck(mg1, e, tp, extrafil, efmg)`: without the extra materials of a once-per-turn
/// EFFECT_EXTRA_FUSION_MATERIAL effect already used
fn extra_mat_opt_check(d: &mut Duel, mg1: &mut CardSet, e: EffectId, tp: u8, extrafil: &Option<ExtraFil>, efmg: &mut CardSet) {
    let extra_feff_mg = group::filter(d, mg1, &|d, c| get_extra_mat_eff(d, c, None).is_some(), NO_EX);
    if !extra_feff_mg.is_empty() {
        let first = group::get_first(d, &extra_feff_mg).nil();
        let Some(extra_feff) = get_extra_mat_eff(d, first, None) else { return };
        if !effect::check_count_limit(d, extra_feff, tp) {
            let loc = effect::get_target_range(d, extra_feff).0;
            let remove = |d: &mut Duel, mg1: &mut CardSet, efmg: &mut CardSet| {
                let g = group::filter(d, &extra_feff_mg, &|d, c| card::is_location(d, c, loc), NO_EX);
                group::remove_card(mg1, &g);
                efmg.clear();
            };
            match extrafil {
                Some(ef) => match ef(d, e, tp, mg1).0 {
                    Some(g) if !g.is_empty() && !group::is_exists(d, &g, &|d, c| card::is_location(d, c, loc), 1, NO_EX) => remove(d, mg1, efmg),
                    None => remove(d, mg1, efmg),
                    _ => {}
                },
                None => remove(d, mg1, efmg),
            }
        }
    } else if !efmg.is_empty() {
        let first = group::get_first(d, efmg).nil();
        if let Some(extra_feff) = get_extra_mat_eff(d, first, None) {
            if !effect::check_count_limit(d, extra_feff, tp) {
                efmg.clear();
            }
        }
    }
}
/// `Duel.GetFusionMaterial(tp)` (proc_fusion_spell.lua): also the Extra Deck cards with an
/// EFFECT_EXTRA_FUSION_MATERIAL effect
pub fn get_fusion_material(d: &mut Duel, tp: u8) -> CardSet {
    let mut res = duel::get_fusion_material_raw(d, tp);
    let g = duel::get_matching_group(d, Some(&|d, c| !card::is_has_effect(d, c, EFFECT_EXTRA_FUSION_MATERIAL, None).is_empty()), tp, LOCATION_EXTRA, 0, NO_EX);
    if !g.is_empty() {
        group::add_card(&mut res, &g);
    }
    res
}
/// `Fusion.SummonEffFilter(c, fusfilter, e, tp, mg, gc, chkf, value, sumlimit, nosummoncheck, sumpos, efmg)`
/// (`mg` gains the extra materials that apply to `c`, for the next cards too)
#[allow(clippy::too_many_arguments)]
fn summon_eff_filter(d: &mut Duel, c: CardId, fusfilter: &dyn Fn(&mut Duel, CardId, u8) -> bool, e: EffectId, tp: u8, mg: &mut CardSet, gc: Option<&CardSet>, chkf: u64, value: u64, sumlimit: bool, nosummoncheck: bool, sumpos: u8, efmg: Option<&CardSet>) -> bool {
    if let Some(efmg) = efmg {
        let add = group::filter(d, efmg, &|d, x| get_extra_mat_eff(d, x, Some(c)).is_some(), NO_EX);
        group::add_card(mg, &add);
    }
    card::is_type(d, c, TYPE_FUSION)
        && fusfilter(d, c, tp)
        && (nosummoncheck || card::is_can_be_special_summoned(d, c, e, value as u32, tp, sumlimit, false, Some(sumpos), None, None))
        && card::check_fusion_material(d, c, Some(&*mg), gc, Some(chkf))
}
/// `Fusion.ForcedMatValidity(c, e)`
fn forced_mat_validity(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    if effect::get_handler(d, e) == Some(c) {
        return !card::is_relate_to_effect(d, c, e);
    }
    card::is_immune_to_effect(d, c, e)
}
/// `aux.AND(f, g)` of two optional `Fusion.CheckAdditional` functions (`nil` ends the list)
fn and_check(a: Option<LuaFn>, b: Option<LuaFn>) -> Option<LuaFn> {
    match (a, b) {
        (Some(a), Some(b)) => Some(LuaFn::new(move |d, args| Value::Bool(a.call(d, args).truthy() && b.call(d, args).truthy()))),
        (Some(a), None) => Some(LuaFn::new(move |d, args| Value::Bool(a.call(d, args).truthy()))),
        (None, _) => None,
    }
}
/// The settings both the target and the operation derive: (chkf, sumlimit, notfusion).
fn chkf_of(d: &mut Duel, cell: u64, param: Option<u64>, tp: u8) -> u64 {
    // (`if not chkf or ((chkf&PLAYER_NONE)~=PLAYER_NONE) then chkf=chkf and chkf|tp or tp end`:
    // the closure keeps the new value)
    let cur = match crate::cell::get(d, cell) {
        Value::Int(v) => Some(v as u64),
        _ => param,
    };
    let chkf = match cur {
        None => tp as u64,
        Some(c) if c & PLAYER_NONE as u64 != PLAYER_NONE as u64 => c | tp as u64,
        Some(c) => c,
    };
    crate::cell::set_int(d, cell, chkf as i64);
    chkf
}
/// `gc` evaluated (a card becomes a group)
fn eval_gc(d: &mut Duel, gc: &Option<Gc>, a: &EffArgs) -> Option<CardSet> {
    let gcf = gc.as_ref()?;
    match gcf(d, a) {
        Value::Card(c) => Some(CardSet::from_card(c)),
        Value::Group(g) => Some(g),
        // (`gc(...) or gc`: the function itself, which `Includes` cannot take)
        _ => crate::lua_error("Parameter 2 should be \"Group\" but is \"function\"."),
    }
}
fn matfilter_or_default(p: &SummonEff) -> MatFilterFn {
    p.matfilter.clone().unwrap_or_else(|| Arc::new(|d, c, _e, _tp, _chk| card::is_able_to_grave(d, c, None)))
}
fn fusfilter_of(p: &SummonEff) -> impl Fn(&mut Duel, CardId, u8) -> bool + '_ {
    move |d, c, tp| p.fusfilter.as_ref().is_none_or(|f| f(d, c, tp))
}
/// `repl_function` of an EFFECT_EXTRA_FUSION_MATERIAL effect (its label object `{f1, f2}`)
fn repl_functions(v: &Value) -> Option<(LuaFn, Option<LuaFn>)> {
    match v {
        Value::Multi(t) => Some((t.first()?.as_fn()?.clone(), t.get(1).and_then(|x| x.as_fn()).cloned())),
        _ => None,
    }
}
fn call_extrafil(d: &mut Duel, f: &LuaFn, e: EffectId, tp: u8, mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    let r = f.call(d, &[Value::Effect(e), Value::Int(tp as i64), Value::Group(mg.clone())]);
    match r {
        Value::Multi(v) => (v.first().and_then(|x| x.as_group().cloned()), v.get(1).and_then(|x| x.as_fn()).cloned()),
        Value::Group(g) => (Some(g), None),
        _ => (None, None),
    }
}
fn set_extra_group(d: &mut Duel, g: Option<CardSet>) {
    d.set_script_var(FUSION_EXTRA_GROUP, g.map_or(Value::Nil, Value::Group));
}
/// The chain-material effects' summonable monsters: `None` to check only.
#[allow(clippy::too_many_arguments)]
fn chain_material_target(d: &mut Duel, ce: EffectId, e: EffectId, tp: u8, value: u64) -> CardSet {
    let Some(fgroup) = effect::get_target(d, ce) else { crate::lua_error("attempt to call a nil value (local 'fgroup')") };
    fgroup.call(d, &[Value::Effect(ce), Value::Effect(e), Value::Int(tp as i64), Value::Int(value as i64)]).as_group().cloned().unwrap_or_default()
}
fn chain_material_fcheck(d: &mut Duel, ce: EffectId, check_addition: &Option<LuaFn>) -> Option<LuaFn> {
    let fcheck = effect::get_label_object(d, ce).as_effect().and_then(|lo| effect::get_operation(d, lo));
    match fcheck {
        Some(f) => match check_addition {
            Some(ca) => and_check(Some(ca.clone()), Some(f)),
            None => Some(f),
        },
        None => check_addition.clone(),
    }
}
fn mf_and(mf: Value, fusfilter: &Option<FusFilter>) -> impl Fn(&mut Duel, CardId, u8) -> bool + use<> {
    let fusfilter = fusfilter.clone();
    move |d, c, tp| {
        let ok = match &mf {
            Value::Fn(f) => f.call(d, &[Value::Card(c), Value::Int(tp as i64)]).truthy(),
            _ => crate::lua_error("attempt to call a number value"),
        };
        ok && fusfilter.as_ref().is_none_or(|f| f(d, c, tp))
    }
}

/// `Fusion.SummonEffTG(...)` (the target of a Fusion Summon effect)
pub fn summon_eff_tg(d: &mut Duel, p: SummonEff) -> LuaFn {
    let cell = crate::cell::new(d);
    cost_target(move |d, a| summon_eff_tg_body(d, &p, cell, a))
}
/// `Fusion.SummonEffTG(...)(e, tp, ..., chk)` called directly (a new closure each time)
pub fn summon_eff_tg_call(d: &mut Duel, p: &SummonEff, a: &EffArgs) -> bool {
    // (a fresh closure: its `chkf` starts as the parameter; cell 0 is never written)
    summon_eff_tg_body(d, p, u64::MAX, a)
}
fn chkf_fresh_or_cell(d: &mut Duel, cell: u64, param: Option<u64>, tp: u8) -> u64 {
    if cell == u64::MAX {
        match param {
            None => tp as u64,
            Some(c) if c & PLAYER_NONE as u64 != PLAYER_NONE as u64 => c | tp as u64,
            Some(c) => c,
        }
    } else {
        chkf_of(d, cell, param, tp)
    }
}
fn summon_eff_tg_body(d: &mut Duel, p: &SummonEff, cell: u64, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let sumpos = p.sumpos.unwrap_or(POS_FACEUP as u8);
    let location = p.location.unwrap_or(LOCATION_EXTRA);
    let chkf = chkf_fresh_or_cell(d, cell, p.chkf, tp);
    let sumlimit = chkf & (FUSPROC_NOTFUSION | FUSPROC_NOLIMIT) as u64 != 0;
    let notfusion = chkf & FUSPROC_NOTFUSION as u64 != 0;
    let mut value = p.value.unwrap_or(0) | MATERIAL_FUSION;
    if !notfusion {
        value |= SUMMON_TYPE_FUSION as u64;
    }
    let gc = eval_gc(d, &p.gc, a);
    let matfilter = matfilter_or_default(p);
    if a.chk == Some(0) {
        // (the materials of matfilter and the ones with an EFFECT_EXTRA_FUSION_MATERIAL effect)
        let fmg_all = get_fusion_material(d, tp);
        let mut mg1 = group::filter(d, &fmg_all, &|d, c| matfilter(d, c, e, tp, 0), NO_EX);
        let mut efmg = group::filter(d, &fmg_all, &|d, c| get_extra_mat_eff(d, c, None).is_some(), NO_EX);
        let mut check_addition: Option<LuaFn> = None;
        let mut repl_flag = false;
        let fusmat_ok = |d: &mut Duel, g: &CardSet| {
            let g = group::filter(d, g, &|d, c| card::is_can_be_fusion_material(d, c, None, Some(value), None), NO_EX);
            group::filter(d, &g, &|d, c| !card::is_immune_to_effect(d, c, e), NO_EX)
        };
        if !efmg.is_empty() {
            let first = group::get_first(d, &efmg).nil();
            if let Some(extra_feff) = get_extra_mat_eff(d, first, None) {
                if let Some((r1, r2)) = repl_functions(&effect::get_label_object(d, extra_feff)) {
                    repl_flag = true;
                    match &p.extrafil {
                        // no extrafil (Polymerization)
                        None => {
                            let (g, ca) = call_extrafil(d, &r1, e, tp, &mg1);
                            if let Some(mut g) = g {
                                group::match_(d, &mut g, &|d, c| matfilter(d, c, e, tp, 0), NO_EX);
                                if let Some(r2) = &r2 {
                                    group::match_(d, &mut g, &|d, c| r2.call(d, &[Value::Card(c), Value::Effect(e), Value::Int(tp as i64)]).truthy(), NO_EX);
                                    group::match_(d, &mut efmg, &|d, c| r2.call(d, &[Value::Card(c), Value::Effect(e), Value::Int(tp as i64)]).truthy(), NO_EX);
                                }
                                let xg = fusmat_ok(d, &g);
                                set_extra_group(d, Some(xg));
                                group::add_card(&mut mg1, &g);
                            }
                            check_addition = ca;
                        }
                        // extrafil (Shaddoll Fusion; with an fcheck: Cynet Fusion)
                        Some(ef) => {
                            let (ret1, ret2) = ef(d, e, tp, &mg1);
                            let (repl1, repl2) = call_extrafil(d, &r1, e, tp, &mg1);
                            if let Some(mut g) = ret1 {
                                let mut rg = repl1.unwrap_or_else(|| crate::lua_error("attempt to index a nil value"));
                                group::match_(d, &mut rg, &|d, c| matfilter(d, c, e, tp, 0), NO_EX);
                                group::add_card(&mut g, &rg);
                                let xg = fusmat_ok(d, &g);
                                set_extra_group(d, Some(xg));
                                group::add_card(&mut mg1, &g);
                            }
                            check_addition = if ret2.is_some() { and_check(ret2, repl2) } else { repl2 };
                        }
                    }
                }
            }
        }
        if !repl_flag {
            if let Some(ef) = &p.extrafil {
                let (g, ca) = ef(d, e, tp, &mg1);
                if let Some(g) = g {
                    let xg = fusmat_ok(d, &g);
                    set_extra_group(d, Some(xg));
                    group::add_card(&mut mg1, &g);
                }
                check_addition = ca;
            }
        }
        if let Some(gc) = &gc {
            if !group::includes(&mg1, gc) {
                set_extra_group(d, None);
                return false;
            }
        }
        set_check_additional(d, check_addition.clone());
        let mut mg1 = fusmat_ok(d, &mg1);
        set_check_exact(d, p.exactcount);
        set_check_min(d, p.mincount);
        set_check_max(d, p.maxcount);
        // (without the extra materials of a once-per-turn effect already used)
        extra_mat_opt_check(d, &mut mg1, e, tp, &p.extrafil, &mut efmg);
        let ff = fusfilter_of(p);
        let mut res = false;
        let fg = duel::get_field_group(d, tp, location, 0);
        for c in group::iter(d, &fg) {
            if crate::lua::protected(d, false, |d| summon_eff_filter(d, c, &ff, e, tp, &mut mg1, gc.as_ref(), chkf, value & 0xffffffff, sumlimit, p.nosummoncheck, sumpos, Some(&efmg))) {
                res = true;
                break;
            }
        }
        set_check_additional(d, None);
        set_extra_group(d, None);
        if !res && !notfusion {
            for ce in duel::get_player_effect(d, tp, EFFECT_CHAIN_MATERIAL) {
                let mut mg = chain_material_target(d, ce, e, tp, value);
                let exact = d.script_var(super::fusion::vars::CHECK_EXACT);
                let min = d.script_var(super::fusion::vars::CHECK_MIN);
                if !mg.is_empty() && (exact.is_nil() || mg.len() as i64 == exact.int()) && (min.is_nil() || mg.len() as i64 >= min.int()) {
                    let f = mf_and(effect::get_value(d, ce), &p.fusfilter);
                    let ca = chain_material_fcheck(d, ce, &check_addition);
                    set_check_additional(d, ca);
                    set_extra_group(d, Some(mg.clone()));
                    let mut found = false;
                    let fg = duel::get_field_group(d, tp, location, 0);
        for c in group::iter(d, &fg) {
                        if crate::lua::protected(d, false, |d| summon_eff_filter(d, c, &f, e, tp, &mut mg, gc.as_ref(), chkf, value, sumlimit, p.nosummoncheck, sumpos, None)) {
                            found = true;
                            break;
                        }
                    }
                    set_check_additional(d, None);
                    set_extra_group(d, None);
                    if found {
                        res = true;
                        break;
                    }
                }
            }
        }
        set_check_exact(d, None);
        set_check_min(d, None);
        set_check_max(d, None);
        return res;
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, location as i32);
    if let Some(et) = &p.extratg {
        et.call(d, &eff_args_chk(a));
    }
    true
}
fn eff_args_chk(a: &EffArgs) -> Vec<Value> {
    vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
        a.chk.map_or(Value::Nil, Value::Int),
    ]
}
/// `Fusion.ChainMaterialPrompt(effswithgroup, cardID, tp, e)`: which effect Summons `tc`
fn chain_material_prompt(d: &mut Duel, effs: &[(EffectId, CardSet)], tc: CardId, tp: u8) -> usize {
    let idx: Vec<usize> = effs.iter().enumerate().filter(|(_, (_, g))| g.contains(tc)).map(|(i, _)| i).collect();
    if idx.len() == 1 {
        return idx[0];
    }
    let descs: Vec<u64> = idx.iter().map(|&i| effect::get_description(d, effs[i].0)).collect();
    let op = duel::select_option(d, tp, false, &descs);
    *idx.get(op as usize).nil()
}

/// `Fusion.SummonEffOP(...)` (the operation of a Fusion Summon effect)
pub fn summon_eff_op(d: &mut Duel, p: SummonEff) -> LuaFn {
    let cell = crate::cell::new(d);
    operation(move |d, a| summon_eff_op_body(d, &p, cell, a))
}
/// `Fusion.SummonEffOP(...)(e, tp, ...)` called directly (a new closure each time)
pub fn summon_eff_op_call(d: &mut Duel, p: &SummonEff, a: &EffArgs) {
    summon_eff_op_body(d, p, u64::MAX, a)
}
fn summon_eff_op_body(d: &mut Duel, p: &SummonEff, cell: u64, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let sumpos = p.sumpos.unwrap_or(POS_FACEUP as u8);
    let location = p.location.unwrap_or(LOCATION_EXTRA);
    let mut chkf = chkf_fresh_or_cell(d, cell, p.chkf, tp);
    if p.preselect.is_none() {
        chkf |= FUSPROC_CANCELABLE as u64;
        if cell != u64::MAX {
            crate::cell::set_int(d, cell, chkf as i64);
        }
    }
    let sumlimit = chkf & (FUSPROC_NOTFUSION | FUSPROC_NOLIMIT) as u64 != 0;
    let notfusion = chkf & FUSPROC_NOTFUSION as u64 != 0;
    let mut value = p.value.unwrap_or(0);
    if !notfusion {
        value |= SUMMON_TYPE_FUSION as u64 | MATERIAL_FUSION;
    }
    // (`gc(e,tp,eg,ep,ev,re,r,rp,chk)`: `chk` is not defined in the operation)
    let a_nochk = EffArgs { chk: None, ..a.clone() };
    let gc = eval_gc(d, &p.gc, &a_nochk);
    let matfilter = matfilter_or_default(p);
    let mut check_addition: Option<LuaFn> = None;
    let fmg_all = get_fusion_material(d, tp);
    let mut mg1 = group::filter(d, &fmg_all, &|d, c| matfilter(d, c, e, tp, 1), NO_EX);
    let mut efmg = group::filter(d, &fmg_all, &|d, c| get_extra_mat_eff(d, c, None).is_some(), NO_EX);
    let mut extragroup: Option<CardSet> = None;
    let mut repl_flag = false;
    let fmin = |d: &mut Duel, card_: CardId| card::is_can_be_fusion_material(d, card_, None, Some(value), None) && !card::is_immune_to_effect(d, card_, e) && crate::aux::nvfilter(d, card_);
    if !efmg.is_empty() {
        let first = group::get_first(d, &efmg).nil();
        if let Some(extra_feff) = get_extra_mat_eff(d, first, None) {
            if let Some((r1, r2)) = repl_functions(&effect::get_label_object(d, extra_feff)) {
                repl_flag = true;
                match &p.extrafil {
                    None => {
                        let (g, ca) = call_extrafil(d, &r1, e, tp, &mg1);
                        if let Some(mut g) = g {
                            group::match_(d, &mut g, &|d, c| matfilter(d, c, e, tp, 1), NO_EX);
                            if let Some(r2) = &r2 {
                                group::match_(d, &mut g, &|d, c| r2.call(d, &[Value::Card(c), Value::Effect(e), Value::Int(tp as i64)]).truthy(), NO_EX);
                                group::match_(d, &mut efmg, &|d, c| r2.call(d, &[Value::Card(c), Value::Effect(e), Value::Int(tp as i64)]).truthy(), NO_EX);
                            }
                            let xg = group::filter(d, &g, &|d, c| fmin(d, c), NO_EX);
                            set_extra_group(d, Some(xg));
                            group::add_card(&mut mg1, &g);
                        }
                        check_addition = ca;
                    }
                    Some(ef) => {
                        let (ret1, ret2) = ef(d, e, tp, &mg1);
                        let (repl1, repl2) = call_extrafil(d, &r1, e, tp, &mg1);
                        if let Some(mut g) = ret1 {
                            let mut rg = repl1.unwrap_or_else(|| crate::lua_error("attempt to index a nil value"));
                            group::match_(d, &mut rg, &|d, c| matfilter(d, c, e, tp, 1), NO_EX);
                            group::add_card(&mut g, &rg);
                            let xg = group::filter(d, &g, &|d, c| fmin(d, c), NO_EX);
                            set_extra_group(d, Some(xg));
                            group::add_card(&mut mg1, &g);
                        }
                        check_addition = if ret2.is_some() { and_check(ret2, repl2) } else { repl2 };
                    }
                }
            }
        }
    }
    if !repl_flag {
        if let Some(ef) = &p.extrafil {
            let (g, ca) = ef(d, e, tp, &mg1);
            if let Some(g) = g {
                let xg = group::filter(d, &g, &|d, c| fmin(d, c), NO_EX);
                set_extra_group(d, Some(xg));
                extragroup = Some(g.clone());
                group::add_card(&mut mg1, &g);
            }
            check_addition = ca;
        }
    }
    group::match_(d, &mut mg1, &|d, c| fmin(d, c), NO_EX);
    if let Some(gc) = &gc {
        if !group::includes(&mg1, gc) || group::is_exists(d, gc, &|d, c| forced_mat_validity(d, c, e), 1, NO_EX) {
            set_extra_group(d, None);
            return;
        }
    }
    set_check_exact(d, p.exactcount);
    set_check_min(d, p.mincount);
    set_check_max(d, p.maxcount);
    set_check_additional(d, check_addition.clone());
    let mut effswithgroup: Vec<(EffectId, CardSet)> = Vec::new();
    extra_mat_opt_check(d, &mut mg1, e, tp, &p.extrafil, &mut efmg);
    let ff = fusfilter_of(p);
    let mut sg1 = CardSet::new();
    let fg = duel::get_field_group(d, tp, location, 0);
        for c in group::iter(d, &fg) {
        if crate::lua::protected(d, false, |d| summon_eff_filter(d, c, &ff, e, tp, &mut mg1, gc.as_ref(), chkf, value & 0xffffffff, sumlimit, p.nosummoncheck, sumpos, Some(&efmg))) {
            sg1.insert(c);
        }
    }
    if !sg1.is_empty() {
        effswithgroup.push((e, sg1.clone()));
    }
    set_extra_group(d, None);
    set_check_additional(d, None);
    if !notfusion {
        for ce in duel::get_player_effect(d, tp, EFFECT_CHAIN_MATERIAL) {
            let mut mg2 = chain_material_target(d, ce, e, tp, value);
            let exact = d.script_var(super::fusion::vars::CHECK_EXACT);
            let min = d.script_var(super::fusion::vars::CHECK_MIN);
            if !mg2.is_empty() && (exact.is_nil() || mg2.len() as i64 == exact.int()) && (min.is_nil() || mg2.len() as i64 >= min.int()) {
                let f = mf_and(effect::get_value(d, ce), &p.fusfilter);
                let ca = chain_material_fcheck(d, ce, &check_addition);
                set_check_additional(d, ca);
                set_extra_group(d, Some(mg2.clone()));
                let mut sg2 = CardSet::new();
                let fg = duel::get_field_group(d, tp, location, 0);
        for c in group::iter(d, &fg) {
                    if crate::lua::protected(d, false, |d| summon_eff_filter(d, c, &f, e, tp, &mut mg2, gc.as_ref(), chkf, value, sumlimit, p.nosummoncheck, sumpos, None)) {
                        sg2.insert(c);
                    }
                }
                if !sg2.is_empty() {
                    effswithgroup.push((ce, sg2.clone()));
                    group::add_card(&mut sg1, &sg2);
                }
                set_check_additional(d, None);
                set_extra_group(d, None);
            }
        }
    }
    let stage2 = |d: &mut Duel, tc: Option<CardId>, mat: Option<&CardSet>, stage: i64| {
        if let Some(s) = &p.stage2 {
            s(d, e, tc, tp, mat, stage);
        }
    };
    if !sg1.is_empty() {
        let sg = sg1.clone();
        let mut mat1 = CardSet::new();
        let mut sel: Option<usize> = None;
        let mut tc: Option<CardId> = None;
        while mat1.is_empty() {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            let picked = group::select(d, &sg, tp, 1, 1, false, NO_EX).nil();
            let t = group::get_first(d, &picked).nil();
            tc = Some(t);
            if p.preselect.as_ref().is_some_and(|f| !f(d, e, t)) {
                return;
            }
            let i = chain_material_prompt(d, &effswithgroup, t, tp);
            sel = Some(i);
            if effswithgroup[i].0 == e {
                set_check_additional(d, check_addition.clone());
                set_extra_group(d, extragroup.clone());
                mat1 = duel::select_fusion_material(d, tp, t, &mg1, gc.clone(), Some(chkf));
            } else {
                let ce = effswithgroup[i].0;
                let ca = chain_material_fcheck(d, ce, &check_addition);
                set_check_additional(d, ca);
                let xg = chain_material_target(d, ce, e, tp, value);
                set_extra_group(d, Some(xg.clone()));
                mat1 = duel::select_fusion_material(d, tp, t, &xg, gc.clone(), Some(chkf));
            }
        }
        let tc = tc.nil();
        let backupmat;
        let sel_eff = effswithgroup[sel.nil()].0;
        if sel_eff == e {
            set_extra_group(d, None);
            backupmat = mat1.clone();
            if !notfusion {
                card::set_material(d, tc, mat1.clone());
            }
            // (an "extraop", and an EFFECT_EXTRA_FUSION_MATERIAL effect with its own operation)
            let extra_feff_mg = group::filter(d, &mat1, &|d, c| get_extra_mat_eff(d, c, Some(tc)).is_some(), NO_EX);
            if !extra_feff_mg.is_empty() {
                if let Some(extraop) = &p.extraop {
                    let first = group::get_first(d, &extra_feff_mg).nil();
                    if let Some(extra_feff) = get_extra_mat_eff(d, first, Some(tc)) {
                        let extra_feff_op = effect::get_operation(d, extra_feff);
                        if let Some(op) = extra_feff_op.filter(|op| !op.ptr_eq(extraop)) {
                            if effect::check_count_limit(d, extra_feff, tp) {
                                let mut flag = false;
                                if let Some(ef) = &p.extrafil {
                                    let extrafil_g = ef(d, e, tp, &mg1).0.unwrap_or_else(|| crate::lua_error("attempt to get length of a nil value"));
                                    let loc = effect::get_target_range(d, extra_feff).0;
                                    if !group::is_exists(d, &extrafil_g, &|d, c| card::is_location(d, c, loc), 1, NO_EX) {
                                        group::remove_card(&mut mat1, &extra_feff_mg);
                                        let mut g = extra_feff_mg.clone();
                                        call_extra_op(d, &op, e, tc, tp, &mut g);
                                        flag = true;
                                    } else {
                                        let h = effect::get_handler(d, extra_feff).nil();
                                        if duel::select_effect_yes_no(d, tp, h, None) {
                                            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RESOLVECARD as u64);
                                            let mut g = group::select(d, &extra_feff_mg, tp, 1, extra_feff_mg.len() as u16, false, NO_EX).nil();
                                            if !g.is_empty() {
                                                group::remove_card(&mut mat1, &g);
                                                call_extra_op(d, &op, e, tc, tp, &mut g);
                                                flag = true;
                                            }
                                        }
                                    }
                                } else {
                                    group::remove_card(&mut mat1, &extra_feff_mg);
                                    let mut g = extra_feff_mg.clone();
                                    call_extra_op(d, &op, e, tc, tp, &mut g);
                                    flag = true;
                                }
                                if flag && effect::check_count_limit(d, extra_feff, tp) {
                                    effect::use_count_limit(d, extra_feff, tp, 1, false);
                                }
                            }
                        }
                    }
                }
            }
            if let Some(extraop) = &p.extraop {
                if matches!(call_extra_op(d, extraop, e, tc, tp, &mut mat1), Value::Bool(false)) {
                    return;
                }
            }
            if !mat1.is_empty() {
                let (extra_feff_mg, normal_mg) = group::split(d, &mat1, &|d, c| get_extra_mat_eff(d, c, Some(tc)).is_some(), NO_EX);
                let extra_feff = if !extra_feff_mg.is_empty() {
                    let first = group::get_first(d, &extra_feff_mg).nil();
                    get_extra_mat_eff(d, first, Some(tc))
                } else {
                    None
                };
                if !normal_mg.is_empty() && extra_feff.is_none_or(|x| effect::get_label(d, x) != 160018042) {
                    let normal_mg = add_maximum_check(d, &normal_mg);
                    duel::send_to_grave(d, normal_mg, REASON_EFFECT + REASON_MATERIAL + REASON_FUSION, None, None);
                }
                if let Some(extra_feff) = extra_feff {
                    match effect::get_operation(d, extra_feff) {
                        Some(op) => {
                            let mut g = extra_feff_mg.clone();
                            call_extra_op(d, &op, e, tc, tp, &mut g);
                        }
                        None => {
                            let g = add_maximum_check(d, &extra_feff_mg);
                            duel::send_to_grave(d, g, REASON_EFFECT + REASON_MATERIAL + REASON_FUSION, None, None);
                        }
                    }
                    if effect::check_count_limit(d, extra_feff, tp) {
                        effect::use_count_limit(d, extra_feff, tp, 1, false);
                    }
                }
            }
            duel::break_effect(d);
            duel::special_summon_step(d, tc, value as u32, tp, tp, sumlimit, false, sumpos, None);
        } else {
            set_check_additional(d, None);
            set_extra_group(d, None);
            let Some(op) = effect::get_operation(d, sel_eff) else { crate::lua_error("attempt to call a nil value") };
            op.call(d, &[Value::Effect(sel_eff), Value::Effect(e), Value::Int(tp as i64), Value::Card(tc), Value::Group(mat1.clone()), Value::Int(value as i64), Value::Nil, Value::Int(sumpos as i64)]);
            backupmat = card::get_material(d, tc);
        }
        stage2(d, Some(tc), Some(&backupmat), 0);
        duel::special_summon_complete(d);
        stage2(d, Some(tc), Some(&backupmat), 3);
        if chkf & FUSPROC_NOTFUSION as u64 == 0 {
            card::complete_procedure(d, tc);
        }
        stage2(d, Some(tc), Some(&backupmat), 1);
    }
    stage2(d, None, None, 2);
    set_check_min(d, None);
    set_check_max(d, None);
    set_check_exact(d, None);
    set_check_additional(d, None);
}
