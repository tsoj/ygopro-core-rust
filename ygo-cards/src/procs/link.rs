//! `Link.*` (`proc_link.lua`): Link Summons.

use super::{extra_check, extra_from_value, extra_to_value, use_extra_materials, GroupCheck, MatFilter};
use crate::aux::{self, ExtraMaterial};
use crate::constants::*;
use crate::lua::{card, duel, effect, group, NO_EX};
use crate::prelude::*;

/// `Link.AddProcedure(c, f, min, max, specialchk, desc, spcon)` (`max`: the Link Rating if `None`;
/// `spcon(e, e, tp, SUMMON_TYPE_LINK)`)
#[allow(clippy::too_many_arguments)]
pub fn add_procedure(d: &mut Duel, c: CardId, f: Option<MatFilter>, min: usize, max: Option<usize>, specialchk: Option<GroupCheck>, desc: Option<u64>, spcon: Option<LuaFn>) {
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_description(d, e1, desc.unwrap_or(1174));
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_range(d, e1, LOCATION_EXTRA);
    let max = max.unwrap_or_else(|| card::get_link(d, c) as usize);
    let p = Params { f, minc: min, maxc: max, specialchk, spcon };
    effect::set_condition(d, e1, Some(condition_fn(p.clone())));
    effect::set_target(d, e1, Some(target_fn(p.clone())));
    effect::set_operation(d, e1, Some(operation_fn(p)));
    effect::set_value(d, e1, SUMMON_TYPE_LINK as i64);
    card::register_effect(d, c, e1, false);
}

#[derive(Clone)]
struct Params {
    f: Option<MatFilter>,
    minc: usize,
    maxc: usize,
    specialchk: Option<GroupCheck>,
    spcon: Option<LuaFn>,
}

fn call_f(d: &mut Duel, f: &Option<MatFilter>, c: CardId, lc: CardId, tp: u8) -> bool {
    f.as_ref().is_none_or(|f| f(d, c, Some(lc), SUMMON_TYPE_LINK as u64 | MATERIAL_LINK, tp))
}
/// `Link.ConditionFilter(c, f, lc, tp)` (with "Formud Skipper"'s assumed properties)
pub fn condition_filter(d: &mut Duel, c: CardId, f: &Option<MatFilter>, lc: CardId, tp: u8) -> bool {
    let res1 = card::is_can_be_link_material(d, c, Some(lc), Some(tp)) && call_f(d, f, c, lc, tp);
    let mut res2 = false;
    if let Some(&formud_eff) = card::is_has_effect(d, c, EFFECT_FORMUD_SKIPPER, None).first() {
        let label = effect::get_labels(d, formud_eff);
        let mut i = 0;
        while i + 1 < label.len() {
            card::assume_property(d, c, label[i] as u32, label[i + 1] as u64);
            i += 2;
        }
        res2 = card::is_can_be_link_material(d, c, Some(lc), Some(tp)) && call_f(d, f, c, lc, tp);
    }
    res1 || res2
}
/// `Link.GetLinkCount(c)`
pub fn get_link_count(d: &mut Duel, c: CardId) -> i64 {
    if card::is_link_monster(d, c) && card::get_link(d, c) > 1 { 1 + 0x10000 * card::get_link(d, c) as i64 } else { 1 }
}

/// The arguments the recursive checks share.
struct Ctx<'a> {
    tp: u8,
    mg: &'a CardSet,
    lc: CardId,
    minc: usize,
    maxc: usize,
    specialchk: &'a Option<GroupCheck>,
    og: &'a CardSet,
    emt: &'a [ExtraMaterial],
}
/// `mg:IsExists(Link.CheckRecursive, 1, sg, ..., {table.unpack(filt)})`: one copy of `filt`
/// shared by all the candidates (each may add entries to it)
fn exists_recursive(d: &mut Duel, x: &Ctx, mg: &CardSet, sg: &mut CardSet, filt: &[ExtraMaterial]) -> bool {
    let mut shared = filt.to_vec();
    for c in group::iter(d, mg) {
        if sg.contains(c) {
            continue;
        }
        if ygo_core::api::check_matching_with(d, c, |d| check_recursive(d, x, c, sg, &mut shared)) {
            return true;
        }
    }
    false
}
/// `Link.CheckRecursive(c, tp, sg, mg, lc, minc, maxc, f, specialchk, og, emt, filt)`
fn check_recursive(d: &mut Duel, x: &Ctx, c: CardId, sg: &mut CardSet, filt: &mut Vec<ExtraMaterial>) -> bool {
    if sg.len() > x.maxc {
        return false;
    }
    sg.insert(c);
    for ex in filt.clone() {
        if !extra_check(d, &ex, c, x.tp, sg, x.mg, x.lc, 1) {
            sg.remove(c);
            return false;
        }
    }
    if !x.og.contains(c) && !aux::check_valid_extra(d, c, x.tp, sg, x.mg, x.lc, x.emt, filt) {
        sg.remove(c);
        return false;
    }
    let res = check_goal(d, x, sg, filt) || (sg.len() < x.maxc && exists_recursive(d, x, x.mg, sg, filt));
    sg.remove(c);
    res
}
/// `Link.CheckRecursive2(c, tp, sg, sg2, secondg, mg, lc, minc, maxc, f, specialchk, og, emt, filt)`:
/// goes through the chosen materials `sg2` (adding the entries they use to `filt`), then checks
/// the rest with `secondg`
fn check_recursive2(d: &mut Duel, x: &Ctx, c: CardId, sg: &mut CardSet, sg2: &CardSet, secondg: Option<&CardSet>, filt: &mut Vec<ExtraMaterial>) -> bool {
    if sg.len() > x.maxc {
        return false;
    }
    sg.insert(c);
    for ex in filt.clone() {
        if !extra_check(d, &ex, c, x.tp, sg, x.mg, x.lc, 1) {
            sg.remove(c);
            return false;
        }
    }
    if !x.og.contains(c) && !aux::check_valid_extra(d, c, x.tp, sg, x.mg, x.lc, x.emt, filt) {
        sg.remove(c);
        return false;
    }
    let rest = group::difference(sg2, sg);
    if rest.is_empty() {
        let res = match secondg {
            Some(sec) if !sec.is_empty() => exists_recursive(d, x, sec, sg, filt),
            _ => check_goal(d, x, sg, &filt.clone()),
        };
        sg.remove(c);
        return res;
    }
    let next = group::get_first(d, &rest).nil();
    let res = check_recursive2(d, x, next, sg, sg2, secondg, filt);
    sg.remove(c);
    res
}
/// `Link.CheckGoal(tp, sg, lc, minc, f, specialchk, filt)`
fn check_goal(d: &mut Duel, x: &Ctx, sg: &CardSet, filt: &[ExtraMaterial]) -> bool {
    for ex in filt {
        let empty = CardSet::new();
        if !group::is_exists(d, sg, &|d, c| extra_check(d, ex, c, x.tp, sg, &empty, x.lc, 1), 1, NO_EX) {
            return false;
        }
    }
    let n = sg.len() as i32;
    let link = card::get_link(d, x.lc);
    sg.len() >= x.minc
        && group::check_with_sum_equal(d, sg, &get_link_count, link, n, n).0
        && x.specialchk.as_ref().is_none_or(|s| s(d, sg, x.lc, SUMMON_TYPE_LINK as u64 | MATERIAL_LINK, x.tp))
        && duel::get_location_count_from_ex(d, x.tp, Some(x.tp), sg.into(), duel::ExtraCard::Card(x.lc), None).0 > 0
}

fn spcon_ok(d: &mut Duel, p: &Params, e: EffectId, tp: u8) -> bool {
    p.spcon.as_ref().is_none_or(|f| f.call(d, &[Value::Effect(e), Value::Effect(e), Value::Int(tp as i64), Value::Int(SUMMON_TYPE_LINK as i64)]).truthy())
}
/// `Link.Condition(f, minc, maxc, specialchk, spcon)`: `function(e, c, must, g, min, max)`
fn condition_fn(p: Params) -> LuaFn {
    lua_fn(move |d, a| {
        let e = a.e(0);
        let Some(c) = a.card(1) else { return true };
        if card::is_type(d, c, TYPE_PENDULUM) && card::is_faceup(d, c) {
            return false;
        }
        let tp = card::get_controler(d, c);
        if !spcon_ok(d, &p, e, tp) {
            return false;
        }
        let must = a.v(2).as_group().cloned();
        let g = match a.v(3) {
            Value::Group(g) => g.clone(),
            _ => duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), tp, LOCATION_MZONE, 0, NO_EX),
        };
        let mg = group::filter(d, &g, &|d, x| condition_filter(d, x, &p.f, c, tp), NO_EX);
        let mut mustg = aux::get_must_be_material_group(d, tp, &g, tp, Some(c), &mg, REASON_LINK);
        if let Some(m) = &must {
            group::add_card(&mut mustg, m);
        }
        let (min, max) = (a.v(4), a.v(5));
        if !min.is_nil() && (min.int() as usize) < p.minc {
            return false;
        }
        if !max.is_nil() && (max.int() as usize) > p.maxc {
            return false;
        }
        let min = if min.is_nil() { p.minc } else { min.int() as usize };
        let max = if max.is_nil() { p.maxc } else { max.int() as usize };
        if group::is_exists(d, &mustg, &|d, x| !condition_filter(d, x, &p.f, c, tp), 1, NO_EX) || mustg.len() > max {
            return false;
        }
        let (emt, mut tg) = aux::get_extra_materials(d, tp, Some(&group::union(&mustg, &mg)), Some(c), SUMMON_TYPE_LINK);
        group::match_(d, &mut tg, &|d, x| condition_filter(d, x, &p.f, c, tp), NO_EX);
        let mg_tg = group::union(&mg, &tg);
        let mut res = group::includes(&mg_tg, &mustg) && mustg.len() <= max;
        if res {
            let x = Ctx { tp, mg: &mg_tg, lc: c, minc: min, maxc: max, specialchk: &p.specialchk, og: &mg, emt: &emt };
            if mustg.len() == max {
                let mut sg = CardSet::new();
                res = exists_recursive_fresh(d, &x, &mustg, &mut sg);
            } else if mustg.len() < max {
                let mut sg = mustg.clone();
                res = exists_recursive_fresh(d, &x, &mg_tg, &mut sg);
            }
        }
        aux::delete_extra_material_groups(d, &emt);
        res
    })
}
/// `g:IsExists(Link.CheckRecursive, 1, sg, ...)` without `filt`: a new table per candidate
fn exists_recursive_fresh(d: &mut Duel, x: &Ctx, g: &CardSet, sg: &mut CardSet) -> bool {
    for c in group::iter(d, g) {
        if sg.contains(c) {
            continue;
        }
        if ygo_core::api::check_matching_with(d, c, |d| check_recursive(d, x, c, sg, &mut Vec::new())) {
            return true;
        }
    }
    false
}
/// `Link.Target(f, minc, maxc, specialchk, spcon)`: `function(e,tp,eg,ep,ev,re,r,rp,chk,c,must,g,min,max)`
fn target_fn(p: Params) -> LuaFn {
    lua_fn(move |d, a| {
        let (e, tp) = (a.e(0), a.p(1));
        let c = a.c(9);
        if !spcon_ok(d, &p, e, tp) {
            return false;
        }
        let must = a.v(10).as_group().cloned();
        let g = match a.v(11) {
            Value::Group(g) => g.clone(),
            _ => duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), tp, LOCATION_MZONE, 0, NO_EX),
        };
        let (min, max) = (a.v(12), a.v(13));
        if !min.is_nil() && (min.int() as usize) < p.minc {
            return false;
        }
        if !max.is_nil() && (max.int() as usize) > p.maxc {
            return false;
        }
        let min = if min.is_nil() { p.minc } else { min.int() as usize };
        let max = if max.is_nil() { p.maxc } else { max.int() as usize };
        let mg = group::filter(d, &g, &|d, x| condition_filter(d, x, &p.f, c, tp), NO_EX);
        let mut mustg = aux::get_must_be_material_group(d, tp, &g, tp, Some(c), &mg, REASON_LINK);
        if let Some(m) = &must {
            group::add_card(&mut mustg, m);
        }
        let (emt, mut tg) = aux::get_extra_materials(d, tp, Some(&group::union(&mustg, &mg)), Some(c), SUMMON_TYPE_LINK);
        group::match_(d, &mut tg, &|d, x| condition_filter(d, x, &p.f, c, tp), NO_EX);
        let mut sg = CardSet::new();
        group::add_card(&mut sg, &mustg);
        let mg_tg = group::union(&mg, &tg);
        let x = Ctx { tp, mg: &mg_tg, lc: c, minc: min, maxc: max, specialchk: &p.specialchk, og: &mg, emt: &emt };
        while sg.len() < max {
            let mut filters = Vec::new();
            if !sg.is_empty() {
                let first = group::get_first(d, &sg).nil();
                check_recursive2(d, &x, first, &mut CardSet::new(), &sg.clone(), Some(&mg_tg), &mut filters);
            }
            // (`mg_tg:Filter(Link.CheckRecursive, sg, ..., {table.unpack(filters)})`: one shared copy)
            let mut shared = filters.clone();
            let mut cg = CardSet::new();
            for tc in group::iter(d, &mg_tg) {
                if sg.contains(tc) {
                    continue;
                }
                if ygo_core::api::check_matching_with(d, tc, |d| check_recursive(d, &x, tc, &mut sg, &mut shared)) {
                    cg.insert(tc);
                }
            }
            if cg.is_empty() {
                break;
            }
            let finish = sg.len() >= min && sg.len() <= max && check_goal(d, &x, &sg, &filters);
            // (`not og`: `og` is an undefined global there)
            let cancel = duel::is_summon_cancelable(d) && sg.is_empty();
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_LMATERIAL as u64);
            let Some(tc) = group::select_unselect(d, &cg, Some(&sg), tp, finish, cancel, Some(1), Some(1)) else { break };
            if mustg.is_empty() || !mustg.contains(tc) {
                if !sg.contains(tc) {
                    sg.insert(tc);
                } else {
                    sg.remove(tc);
                }
            }
        }
        if !sg.is_empty() {
            let mut filters = Vec::new();
            let first = group::get_first(d, &sg).nil();
            check_recursive2(d, &x, first, &mut CardSet::new(), &sg.clone(), Some(&mg_tg), &mut filters);
            effect::set_label_object(d, e, Value::Multi(vec![Value::Group(sg), extra_to_value(&filters), extra_to_value(&emt)]));
            true
        } else {
            aux::delete_extra_material_groups(d, &emt);
            false
        }
    })
}
/// `Link.Operation(f, minc, maxc, specialchk)`: `function(e,tp,eg,ep,ev,re,r,rp,c,must,g,min,max)`
fn operation_fn(p: Params) -> LuaFn {
    lua_fn(move |d, a| {
        let (e, tp) = (a.e(0), a.p(1));
        let c = a.c(8);
        let Value::Multi(lo) = effect::get_label_object(d, e) else { crate::lua_error("bad argument #1 to 'unpack' (table expected)") };
        let g = lo[0].as_group().cloned().unwrap_or_default();
        let filt = extra_from_value(&lo[1]);
        let emt = extra_from_value(&lo[2]);
        use_extra_materials(d, &filt, &g, SUMMON_TYPE_LINK, c, tp);
        for tc in group::iter(d, &g) {
            if let Some(&formud_eff) = card::is_has_effect(d, tc, EFFECT_FORMUD_SKIPPER, None).first() {
                let res1 = card::is_can_be_link_material(d, tc, Some(c), Some(tp)) && call_f(d, &p.f, tc, c, tp);
                let label = effect::get_labels(d, formud_eff);
                let mut i = 0;
                while i + 1 < label.len() {
                    card::assume_property(d, tc, label[i] as u32, label[i + 1] as u64);
                    i += 2;
                }
                let res2 = card::is_can_be_link_material(d, tc, Some(c), Some(tp)) && call_f(d, &p.f, tc, c, tp);
                if !res2 || (res1 && res2 && !duel::select_effect_yes_no(d, tp, tc, None)) {
                    duel::assume_reset(d);
                }
            }
        }
        card::set_material(d, c, g.clone());
        duel::send_to_grave(d, g, REASON_MATERIAL + REASON_LINK, None, None);
        aux::delete_extra_material_groups(d, &emt);
    })
}
