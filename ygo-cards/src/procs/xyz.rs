//! `Xyz.*` (`proc_xyz.lua`): Xyz Summons.

use super::MatFilter;
use crate::aux;
use crate::constants::*;
use crate::lua::{card, duel, effect, group, NO_EX};
use crate::prelude::*;
use std::sync::Arc;

/// The maximum number of materials (`maxct`): a number or `Xyz.InfiniteMats`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaxCt {
    N(usize),
    Infinite,
}
/// `Xyz.InfiniteMats`
pub const INFINITE_MATS: MaxCt = MaxCt::Infinite;

/// `alterf(c, tp, xyzc)`: the monster an Xyz Monster can be Summoned on top of
pub type AlterFilter = Arc<dyn Fn(&mut Duel, CardId, u8, CardId) -> bool + Send + Sync>;
pub fn alter_filter(f: impl Fn(&mut Duel, CardId, u8, CardId) -> bool + Send + Sync + 'static) -> AlterFilter {
    Arc::new(f)
}
/// `exchk(matg, tp, xyz)`: a check of all the materials
pub type ExChk = Arc<dyn Fn(&mut Duel, &CardSet, u8, CardId) -> bool + Send + Sync>;
pub fn ex_chk(f: impl Fn(&mut Duel, &CardSet, u8, CardId) -> bool + Send + Sync + 'static) -> ExChk {
    Arc::new(f)
}

/// `Xyz.ProcCancellable` (set by the alternative procedure's target)
pub const PROC_CANCELLABLE: (u32, u64) = (crate::ns::AUX, 200);
/// `Xyz.ProcCancellable`
pub fn proc_cancellable(d: &Duel) -> bool {
    d.script_var(PROC_CANCELLABLE).truthy()
}

/// `Xyz.EffectXyzMaterialChk(c, xyz, tp)`: an opponent's monster that can be used as material
pub fn effect_xyz_material_chk(d: &mut Duel, c: CardId, xyz: CardId, tp: u8) -> bool {
    for eff in card::get_card_effect(d, c, EFFECT_XYZ_MATERIAL) {
        match effect::get_value(d, eff) {
            Value::Fn(f) => {
                if f.call(d, &[Value::Effect(eff), Value::Card(c), Value::Card(xyz), Value::Int(tp as i64)]).truthy() {
                    return true;
                }
            }
            v if v.int() == 0 => return true,
            _ => crate::lua_error("attempt to call a number value (local 'val')"),
        }
    }
    false
}
/// `Xyz.AlterFilter(c, alterf, xyzc, e, tp, op)`
pub fn alter_filter_check(d: &mut Duel, c: CardId, alterf: &AlterFilter, xyzc: CardId, e: EffectId, tp: u8, op: &Option<LuaFn>) -> bool {
    if !alterf(d, c, tp, xyzc)
        || !card::is_can_be_xyz_material(d, c, Some(xyzc), Some(tp), None)
        || (card::is_controler(d, c, 1 - tp) && !effect_xyz_material_chk(d, c, xyzc, tp))
        || op.as_ref().is_some_and(|op| !op.call(d, &[Value::Effect(e), Value::Int(tp as i64), Value::Int(0), Value::Card(c)]).truthy())
    {
        return false;
    }
    if card::is_location(d, xyzc, LOCATION_EXTRA) {
        duel::get_location_count_from_ex(d, tp, Some(tp), c.into(), duel::ExtraCard::Card(xyzc), None).0 > 0
    } else {
        duel::get_m_zone_count(d, tp, c.into(), Some(tp), None, None).0 > 0
    }
}

/// `Xyz.AddProcedure(c, f, lv, ct, alterf, desc, maxct, op, mustbemat, exchk)`: Xyz Summon with
/// `ct`..`maxct` (default `ct`) Level `lv` monsters matching `f`, or (with `alterf`) on top of 1
/// monster (`op(e, tp, chk, c)`: what that also needs)
#[allow(clippy::too_many_arguments)]
pub fn add_procedure(
    d: &mut Duel,
    c: CardId,
    f: Option<MatFilter>,
    lv: Option<u32>,
    ct: usize,
    alterf: Option<AlterFilter>,
    desc: Option<u64>,
    maxct: Option<MaxCt>,
    op: Option<LuaFn>,
    mustbemat: bool,
    exchk: Option<ExChk>,
) {
    let maxct = match maxct {
        None => MaxCt::N(ct),
        // (99 is the deprecated way of writing `Xyz.InfiniteMats`)
        Some(MaxCt::N(99)) => MaxCt::Infinite,
        Some(m) => m,
    };
    if crate::class::get(d, c, crate::class::MIN_XYZ_CT).is_nil() {
        crate::class::set(d, c, crate::class::MIN_XYZ_CT, Value::Int(ct as i64));
        crate::class::set(d, c, crate::class::MAX_XYZ_CT, match maxct {
            MaxCt::N(n) => Value::Int(n as i64),
            MaxCt::Infinite => Value::Int(i64::MAX),
        });
    }
    let p = Params { f, lv, minc: ct, maxc: match maxct { MaxCt::N(n) => Some(n), MaxCt::Infinite => None }, mustbemat, exchk };
    let chk1 = effect::create_effect(d, c);
    effect::set_type(d, chk1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, chk1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_code(d, chk1, 946);
    effect::set_condition(d, chk1, Some(condition_fn(p.clone())));
    effect::set_target(d, chk1, Some(target_fn(p.clone())));
    effect::set_operation(d, chk1, Some(operation_fn()));
    card::register_effect(d, c, chk1, false);
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_description(d, e1, 1173);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e1, LOCATION_EXTRA);
    effect::set_condition(d, e1, Some(condition_fn(p.clone())));
    effect::set_target(d, e1, Some(target_fn(p)));
    effect::set_operation(d, e1, Some(operation_fn()));
    effect::set_value(d, e1, SUMMON_TYPE_XYZ as i64);
    effect::set_label_object(d, e1, Value::Effect(chk1));
    card::register_effect(d, c, e1, false);
    if let Some(alterf) = alterf {
        let chk2 = effect::clone(d, chk1);
        effect::set_description(d, chk2, desc.unwrap_or(0));
        effect::set_condition(d, chk2, Some(condition2_fn(alterf.clone(), op.clone())));
        effect::set_target(d, chk2, Some(target2_fn(alterf.clone(), op.clone())));
        effect::set_operation(d, chk2, Some(operation2_fn()));
        card::register_effect(d, c, chk2, false);
        let e2 = effect::clone(d, e1);
        effect::set_description(d, e2, desc.unwrap_or(0));
        effect::set_condition(d, e2, Some(condition2_fn(alterf.clone(), op.clone())));
        effect::set_target(d, e2, Some(target2_fn(alterf, op)));
        effect::set_operation(d, e2, Some(operation2_fn()));
        card::register_effect(d, c, e2, false);
    }
}

#[derive(Clone)]
struct Params {
    f: Option<MatFilter>,
    lv: Option<u32>,
    minc: usize,
    /// `None`: `Xyz.InfiniteMats`
    maxc: Option<usize>,
    mustbemat: bool,
    exchk: Option<ExChk>,
}

/// `Xyz.MatFilter2(c, f, lv, xyz, tp)`
pub fn mat_filter2(d: &mut Duel, c: CardId, f: &Option<MatFilter>, lv: Option<u32>, xyz: CardId, tp: u8) -> bool {
    if f.as_ref().is_some_and(|f| !f(d, c, Some(xyz), SUMMON_TYPE_XYZ as u64 | MATERIAL_XYZ, tp)) {
        return false;
    }
    if lv.is_some_and(|lv| !card::is_xyz_level(d, c, xyz, lv)) {
        return false;
    }
    card::is_can_be_xyz_material(d, c, Some(xyz), Some(tp), None)
}
/// `Xyz.GetMaterials(tp, xyz)`
pub fn get_materials(d: &mut Duel, tp: u8, xyz: CardId) -> CardSet {
    duel::get_matching_group(
        d,
        Some(&|d, c| {
            if card::is_location(d, c, LOCATION_GRAVE) && card::is_has_effect(d, c, EFFECT_XYZ_MAT_FROM_GRAVE, None).is_empty() {
                return false;
            }
            if card::is_location(d, c, LOCATION_MZONE) && card::is_facedown(d, c) {
                return false;
            }
            card::is_controler(d, c, tp) || effect_xyz_material_chk(d, c, xyz, tp)
        }),
        tp,
        LOCATION_MZONE | LOCATION_GRAVE,
        LOCATION_MZONE,
        NO_EX,
    )
}
/// `Xyz.MatFilter(c, f, lv, xyz, tp)`
pub fn mat_filter(d: &mut Duel, c: CardId, f: &Option<MatFilter>, lv: Option<u32>, xyz: CardId, tp: u8) -> bool {
    (card::is_controler(d, c, tp) || effect_xyz_material_chk(d, c, xyz, tp)) && mat_filter2(d, c, f, lv, xyz, tp)
}
/// `Xyz.SubMatFilter(c, lv, xyz, tp)`: a Spell that can be a material of Level `lv`
pub fn sub_mat_filter(d: &mut Duel, c: CardId, lv: Option<u32>) -> bool {
    let Some(lv) = lv else { return false };
    let Some(&te) = card::get_card_effect(d, c, EFFECT_SPELL_XYZ_MAT).first() else { return false };
    let v = match effect::get_value(d, te) {
        Value::Fn(f) => f.call(d, &[Value::Effect(te)]),
        v => v,
    };
    v.int() == lv as i64
}
/// `Xyz.CheckValidMultiXyzMaterial(effs, xyz, matg)`
pub fn check_valid_multi_xyz_material(d: &mut Duel, effs: &[EffectId], xyz: CardId, matg: &CardSet) -> bool {
    for &te in effs {
        match effect::get_operation(d, te) {
            None => return true,
            Some(tgf) => {
                if tgf.call(d, &[Value::Effect(te), Value::Card(xyz), Value::Group(matg.clone())]).truthy() {
                    return true;
                }
            }
        }
    }
    false
}
/// `Xyz.MatNumChk(matct, ct, comp)`
pub fn mat_num_chk(matct: i64, ct: i64, comp: i64) -> bool {
    (comp & 0x1 == 0x1 && matct > ct) || (comp & 0x2 == 0x2 && matct == ct) || (comp & 0x4 == 0x4 && matct < ct)
}
/// `Xyz.MatNumChkF(tg)`: "Star Seraph Sovereignty"-like material counts
pub fn mat_num_chk_f(d: &mut Duel, tg: &CardSet) -> bool {
    for chkc in group::iter(d, tg) {
        for te in card::get_card_effect(d, chkc, EFFECT_STAR_SERAPH_SOVEREIGNTY) {
            let v = effect::get_value(d, te).int();
            let (rct, comp) = (v & 0xffff, (v >> 16) & 0xffff);
            let n = group::filter_count(d, tg, &|d, c| card::is_monster(d, c), NO_EX) as i64;
            if !mat_num_chk(n, rct, comp) {
                return false;
            }
        }
    }
    true
}
/// `Xyz.MatNumChkF2(tg, lv, xyz)`: "Satellarknight Capella"-like material counts (the Level
/// change does not apply while the count does not fit)
pub fn mat_num_chk_f2(d: &mut Duel, tg: &CardSet, lv: u32, xyz: CardId) -> bool {
    for chkc in group::iter(d, tg) {
        let mut rev: Vec<(EffectId, LuaFn)> = Vec::new();
        for te in card::get_card_effect(d, chkc, EFFECT_SATELLARKNIGHT_CAPELLA) {
            let v = effect::get_value(d, te).int();
            let (rct, comp) = (v & 0xffff, (v >> 16) & 0xffff);
            let n = group::filter_count(d, tg, &|d, c| card::is_monster(d, c), NO_EX) as i64;
            if !mat_num_chk(n, rct, comp) {
                let lo = effect::get_label_object(d, te).as_effect().nil();
                let con = effect::get_condition(d, lo).unwrap_or_else(aux_true);
                if !rev.iter().any(|(x, _)| *x == te) {
                    rev.push((te, con));
                    effect::set_condition(d, lo, Some(aux_false()));
                }
            }
        }
        if !rev.is_empty() {
            let islv = card::is_xyz_level(d, chkc, xyz, lv);
            for (te, con) in rev {
                let lo = effect::get_label_object(d, te).as_effect().nil();
                effect::set_condition(d, lo, Some(con));
            }
            if !islv {
                return false;
            }
        }
    }
    true
}
/// `Xyz.CheckMaterialSet(matg, xyz, tp, exchk, mustg, lv)`
fn check_material_set(d: &mut Duel, matg: &CardSet, xyz: CardId, tp: u8, exchk: &Option<ExChk>, mustg: &CardSet, lv: Option<u32>) -> bool {
    if !group::includes(matg, mustg) {
        return false;
    }
    if !mat_num_chk_f(d, matg) {
        return false;
    }
    if lv.is_some_and(|lv| !mat_num_chk_f2(d, matg, lv, xyz)) {
        return false;
    }
    if exchk.as_ref().is_some_and(|ex| !matg.is_empty() && !ex(d, matg, tp, xyz)) {
        return false;
    }
    if card::is_location(d, xyz, LOCATION_EXTRA) {
        return duel::get_location_count_from_ex(d, tp, Some(tp), matg.into(), duel::ExtraCard::Card(xyz), None).0 > 0;
    }
    duel::get_m_zone_count(d, tp, matg.into(), Some(tp), None, None).0 > 0
}

/// What `Xyz.RecursionChk` gets besides the groups it changes.
struct Ctx<'a> {
    xyz: CardId,
    tp: u8,
    min: usize,
    max: Option<usize>,
    minc: usize,
    maxc: Option<usize>,
    mustbemat: bool,
    exchk: &'a Option<ExChk>,
    mustg: &'a CardSet,
    lv: Option<u32>,
    eqmg: &'a Option<CardSet>,
    equips_inverse: &'a [(CardId, CardSet)],
}
/// `aux.HarmonizingMagFilter(c, e, f)` with `f = e:GetValue()`
fn harmonizing(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    match effect::get_value(d, e) {
        Value::Fn(f) => !f.call(d, &[Value::Effect(e), Value::Card(c)]).truthy(),
        _ => crate::lua_error("attempt to call a number value (local 'f')"),
    }
}
/// `mg:IsExists(Xyz.RecursionChk, 1, sg, ...)`: the candidates are fixed when it starts, `mg` itself
/// changes during the checks
fn exists_recursion(d: &mut Duel, x: &Ctx, mg: &mut CardSet, sg: &mut CardSet, matg: &mut CardSet, ct: usize, matct: usize) -> bool {
    let cands: Vec<CardId> = group::iter(d, &group::difference(mg, sg));
    for c in cands {
        if crate::lua::protected(d, false, |d| recursion_chk(d, x, c, mg, sg, matg, ct, matct)) {
            return true;
        }
    }
    false
}
/// `Xyz.RecursionChk(c, mg, xyz, tp, min, max, minc, maxc, sg, matg, ct, matct, mustbemat, exchk, f, mustg, lv, eqmg, equips_inverse)`
#[allow(clippy::too_many_arguments)]
fn recursion_chk(d: &mut Duel, x: &Ctx, c: CardId, mg: &mut CardSet, sg: &mut CardSet, matg: &mut CardSet, ct: usize, matct: usize) -> bool {
    let mut add_to_matg = true;
    if x.eqmg.as_ref().is_some_and(|eqmg| eqmg.contains(c)) {
        match card::get_equip_target(d, c) {
            Some(et) if sg.contains(et) => {}
            _ => return false,
        }
        add_to_matg = false;
    }
    let mut xct = ct;
    let mut rg = CardSet::new();
    if card::is_has_effect(d, c, EFFECT_ORICHALCUM_CHAIN, None).is_empty() {
        xct += 1;
    } else {
        add_to_matg = true;
    }
    let xmatct = matct + 1;
    if x.max.is_some_and(|m| xct > m) || x.maxc.is_some_and(|m| xmatct > m) {
        group::add_card(mg, &rg);
        return false;
    }
    for f in card::get_card_effect(d, c, EFFECT_XYZ_MAT_RESTRICTION) {
        if group::is_exists(d, matg, &|d, m| harmonizing(d, m, f), 1, c.into()) {
            group::add_card(mg, &rg);
            return false;
        }
        let sg2 = group::filter(d, mg, &|d, m| harmonizing(d, m, f), NO_EX);
        group::add_card(&mut rg, &sg2);
        group::remove_card(mg, &sg2);
    }
    for tc in group::iter(d, sg) {
        for f in card::get_card_effect(d, tc, EFFECT_XYZ_MAT_RESTRICTION) {
            if harmonizing(d, c, f) {
                group::add_card(mg, &rg);
                return false;
            }
        }
    }
    if add_to_matg {
        matg.insert(c);
    }
    sg.insert(c);
    let mut eqg: Option<CardSet> = None;
    let res = (|| {
        if xct >= x.min && xmatct >= x.minc && check_material_set(d, matg, x.xyz, x.tp, x.exchk, x.mustg, x.lv) {
            return true;
        }
        if !x.equips_inverse.is_empty() {
            eqg = x.equips_inverse.iter().find(|(k, _)| *k == c).map(|(_, g)| g.clone());
            if let Some(g) = &eqg {
                group::add_card(mg, g);
            }
        }
        if exists_recursion(d, x, mg, sg, matg, xct, xmatct) {
            return true;
        }
        if !x.mustbemat {
            let mut retchknum: Vec<i64> = Vec::new();
            for te in card::is_has_effect(d, c, EFFECT_DOUBLE_XYZ_MATERIAL, Some(x.tp)) {
                let tgf = effect::get_operation(d, te);
                let val = effect::get_value(d, te).int();
                if val > 0
                    && !retchknum.contains(&val)
                    && x.maxc.is_none_or(|m| xmatct + val as usize <= m)
                    && tgf.as_ref().is_none_or(|t| t.call(d, &[Value::Effect(te), Value::Card(x.xyz), Value::Group(matg.clone())]).truthy())
                {
                    retchknum.push(val);
                    effect::use_count_limit(d, te, x.tp, 1, false);
                    let v = val as usize;
                    let chk = (xct + v >= x.min && xmatct + v >= x.minc && check_material_set(d, matg, x.xyz, x.tp, x.exchk, x.mustg, x.lv))
                        || exists_recursion(d, x, mg, sg, matg, xct, xmatct + v);
                    effect::restore_count_limit(d, te, x.tp, 1, false);
                    if chk {
                        return true;
                    }
                }
            }
        }
        false
    })();
    if add_to_matg {
        matg.remove(c);
    }
    sg.remove(c);
    if let Some(g) = &eqg {
        group::remove_card(mg, g);
    }
    group::add_card(mg, &rg);
    res
}

/// The materials both the condition and the target start with: (g, mg, eqmg, equips_inverse).
fn materials(d: &mut Duel, p: &Params, c: CardId, tp: u8, og: Option<&CardSet>) -> (CardSet, CardSet, Option<CardSet>, Vec<(CardId, CardSet)>) {
    let mut eqmg: Option<CardSet> = None;
    let mut equips_inverse = Vec::new();
    if let Some(og) = og {
        let mg = group::filter(d, og, &|d, x| mat_filter(d, x, &p.f, p.lv, c, tp), NO_EX);
        return (og.clone(), mg, None, equips_inverse);
    }
    let g = get_materials(d, tp, c);
    let mut mg = group::filter(d, &g, &|d, x| mat_filter2(d, x, &p.f, p.lv, c, tp), NO_EX);
    if !p.mustbemat {
        for tc in group::iter(d, &mg) {
            let eqgr = card::get_equip_group(d, tc);
            let eq = group::filter(d, &eqgr, &|d, x| !card::is_has_effect(d, x, EFFECT_EQUIP_SPELL_XYZ_MAT, None).is_empty(), NO_EX);
            if !eq.is_empty() {
                let m = eqmg.get_or_insert_with(CardSet::new);
                group::add_card(m, &eq);
                equips_inverse.push((tc, eq));
            }
        }
        if let Some(m) = &eqmg {
            group::add_card(&mut mg, m);
        }
        if p.f.is_none() {
            let lv = p.lv;
            let sub = duel::get_matching_group(d, Some(&|d, x| sub_mat_filter(d, x, lv)), tp, LOCATION_ONFIELD + LOCATION_GRAVE, 0, NO_EX);
            group::add_card(&mut mg, &sub);
        }
    }
    (g, mg, eqmg, equips_inverse)
}
fn orichalcum_group(d: &mut Duel, tp: u8) -> CardSet {
    duel::get_matching_group(d, Some(&|d, x| !card::is_has_effect(d, x, EFFECT_ORICHALCUM_CHAIN, None).is_empty()), tp, LOCATION_HAND + LOCATION_ONFIELD + LOCATION_GRAVE + LOCATION_REMOVED, 0, NO_EX)
}
fn opt_count(v: &Value) -> Option<usize> {
    if v.is_nil() { None } else { Some(v.int() as usize) }
}

/// `Xyz.Condition(f, lv, minc, maxc, mustbemat, exchk)`: `function(e, c, must, og, min, max)`
fn condition_fn(p: Params) -> LuaFn {
    lua_fn(move |d, a| {
        let Some(c) = a.card(1) else { return true };
        let must = a.v(2).as_group().cloned();
        let og = a.v(3).as_group().cloned();
        let (min, max) = (opt_count(a.v(4)), opt_count(a.v(5)));
        if p.maxc.is_some_and(|m| min.is_some_and(|mn| mn > m)) {
            return false;
        }
        if card::is_type(d, c, TYPE_PENDULUM) && card::is_faceup(d, c) {
            return false;
        }
        let tp = card::get_controler(d, c);
        let (g, mut mg, eqmg, equips_inverse) = materials(d, &p, c, tp, og.as_ref());
        let mut mustg = aux::get_must_be_material_group(d, tp, &g, tp, Some(c), &mg, REASON_XYZ);
        if let Some(m) = &must {
            group::add_card(&mut mustg, m);
        }
        if !group::includes(&mg, &mustg) {
            return false;
        }
        if !p.mustbemat {
            let o = orichalcum_group(d, tp);
            group::add_card(&mut mg, &o);
        }
        let x = Ctx { xyz: c, tp, min: min.unwrap_or(0), max, minc: p.minc, maxc: p.maxc, mustbemat: p.mustbemat, exchk: &p.exchk, mustg: &mustg, lv: p.lv, eqmg: &eqmg, equips_inverse: &equips_inverse };
        exists_recursion(d, &x, &mut mg, &mut CardSet::new(), &mut CardSet::new(), 0, 0)
    })
}
/// `Xyz.Target(f, lv, minc, maxc, mustbemat, exchk)`: `function(e,tp,eg,ep,ev,re,r,rp,chk,c,must,og,min,max)`
fn target_fn(p: Params) -> LuaFn {
    lua_fn(move |d, a| {
        let (e, tp) = (a.e(0), a.p(1));
        let c = a.c(9);
        let must = a.v(10).as_group().cloned();
        let og = a.v(11).as_group().cloned();
        let (min, max) = (opt_count(a.v(12)), opt_count(a.v(13)));
        let _cancel = og.is_none() && duel::is_summon_cancelable(d);
        let (g, mut mg, eqmg, equips_inverse) = materials(d, &p, c, tp, og.as_ref());
        let mut mustg = aux::get_must_be_material_group(d, tp, &g, tp, Some(c), &mg, REASON_XYZ);
        if let Some(m) = &must {
            group::add_card(&mut mustg, m);
        }
        let mut orichalcum: Option<CardSet> = None;
        if !p.mustbemat {
            let o = orichalcum_group(d, tp);
            group::add_card(&mut mg, &o);
            orichalcum = Some(o);
        }
        if orichalcum.as_ref().is_none_or(|o| o.is_empty()) && must.as_ref().is_some_and(|m| Some(m.len()) == min && Some(m.len()) == max) {
            effect::set_label_object(d, e, Value::Group(mustg.clone()));
            return true;
        }
        let mut extra_mats = 0usize;
        let min0 = min.unwrap_or(0);
        let mut matg = CardSet::new();
        let mut sg = CardSet::new();
        let mut multi_selected: Vec<(CardId, EffectId, usize)> = Vec::new();
        let mut finishable = false;
        let x = Ctx { xyz: c, tp, min: min0, max, minc: p.minc, maxc: p.maxc, mustbemat: p.mustbemat, exchk: &p.exchk, mustg: &mustg, lv: p.lv, eqmg: &eqmg, equips_inverse: &equips_inverse };
        loop {
            let ct = matg.len();
            let matct = ct + extra_mats;
            if !(max.is_none_or(|m| matg.len() < m) && p.maxc.is_none_or(|m| matct < m)) {
                break;
            }
            // (`mg:Filter(Xyz.RecursionChk, sg, ...)`: the candidates are fixed when it starts)
            let cands: Vec<CardId> = group::iter(d, &group::difference(&mg, &sg));
            let mut selg = CardSet::new();
            for tc in cands {
                if crate::lua::protected(d, false, |d| recursion_chk(d, &x, tc, &mut mg, &mut sg, &mut matg, ct, matct)) {
                    selg.insert(tc);
                }
            }
            if selg.is_empty() {
                break;
            }
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_XMATERIAL as u64);
            let cancelable = og.is_none() && duel::is_summon_cancelable(d) && sg.is_empty();
            let Some(sc) = group::select_unselect(d, &selg, Some(&sg), tp, finishable, cancelable, None, None) else {
                if !finishable {
                    return false;
                }
                break;
            };
            if !sg.contains(sc) {
                sg.insert(sc);
                if let Some((_, eq)) = equips_inverse.iter().find(|(k, _)| *k == sc) {
                    group::add_card(&mut mg, eq);
                }
                let multi_xyz = card::is_has_effect(d, sc, EFFECT_DOUBLE_XYZ_MATERIAL, Some(tp));
                if !multi_xyz.is_empty() && check_valid_multi_xyz_material(d, &multi_xyz, c, &matg) && ct < p.minc {
                    matg.insert(sc);
                    // (the counts that can be chosen: 1 = as one material, 1+val = as 1+val)
                    let mut multi: Vec<(usize, Vec<EffectId>)> = Vec::new();
                    if exists_recursion(d, &x, &mut mg, &mut sg, &mut matg, ct + 1, matct + 1) {
                        multi.push((1, Vec::new()));
                    }
                    for &te in &multi_xyz {
                        let tgf = effect::get_operation(d, te);
                        let val = effect::get_value(d, te).int();
                        if val > 0 && tgf.as_ref().is_none_or(|t| t.call(d, &[Value::Effect(te), Value::Card(c), Value::Group(matg.clone())]).truthy()) {
                            let new_count = matct + 1 + val as usize;
                            effect::use_count_limit(d, te, tp, 1, false);
                            let chk = (p.minc <= new_count && p.maxc.is_none_or(|m| new_count <= m) && group::includes(&sg, &mustg))
                                || exists_recursion(d, &x, &mut mg, &mut sg, &mut matg, ct + 1, new_count);
                            if chk {
                                let k = 1 + val as usize;
                                match multi.iter_mut().find(|(n, _)| *n == k) {
                                    Some((_, v)) => v.push(te),
                                    None => multi.push((k, vec![te])),
                                }
                            }
                            effect::restore_count_limit(d, te, tp, 1, false);
                        }
                    }
                    if !multi.is_empty() {
                        let mut available: Vec<usize> = multi.iter().map(|(n, _)| *n).collect();
                        let mut chosen = available[0];
                        if available.len() != 1 {
                            available.sort();
                            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_XMATERIAL as u64);
                            let nums: Vec<u64> = available.iter().map(|&n| n as u64).collect();
                            chosen = duel::announce_number(d, tp, &nums).map_or(0, |r| r.0 as usize);
                        }
                        if chosen > 1 {
                            let eff = multi.iter().find(|(n, _)| *n == chosen).nil().1[0];
                            extra_mats += chosen - 1;
                            effect::use_count_limit(d, eff, tp, 1, false);
                            multi_selected.retain(|(k, _, _)| *k != sc);
                            multi_selected.push((sc, eff, chosen));
                        }
                    }
                } else if !card::is_has_effect(d, sc, EFFECT_ORICHALCUM_CHAIN, None).is_empty() {
                    extra_mats += 1;
                } else {
                    matg.insert(sc);
                }
            } else {
                if let Some((_, equips)) = equips_inverse.iter().find(|(k, _)| *k == sc) {
                    if group::includes(&sg, equips) {
                        continue;
                    }
                    group::remove_card(&mut mg, equips);
                }
                sg.remove(sc);
                if !card::is_has_effect(d, sc, EFFECT_ORICHALCUM_CHAIN, None).is_empty() {
                    extra_mats -= 1;
                } else {
                    matg.remove(sc);
                    if let Some(i) = multi_selected.iter().position(|(k, _, _)| *k == sc) {
                        let (_, eff, num) = multi_selected.remove(i);
                        effect::restore_count_limit(d, eff, tp, 1, false);
                        extra_mats -= num - 1;
                    }
                }
            }
            finishable = matg.len() + extra_mats >= p.minc && check_material_set(d, &matg, c, tp, &p.exchk, &mustg, p.lv);
        }
        effect::set_label_object(d, e, Value::Group(sg));
        true
    })
}
/// `Xyz.Operation(...)`: `function(e,tp,eg,ep,ev,re,r,rp,c,must,og,min,max)`
fn operation_fn() -> LuaFn {
    lua_fn(|d, a| {
        let e = a.e(0);
        let c = a.c(8);
        let Value::Group(mut g) = effect::get_label_object(d, e) else { return };
        let remg = group::filter(d, &g, &|d, x| !card::is_has_effect(d, x, EFFECT_ORICHALCUM_CHAIN, None).is_empty(), NO_EX);
        for tc in group::iter(d, &remg) {
            card::register_flag_effect(d, tc, 511002115, RESET_EVENT + RESETS_STANDARD, 0, 0, 0, 0);
        }
        group::remove(d, &mut g, &|d, x| !card::is_has_effect(d, x, EFFECT_ORICHALCUM_CHAIN, None).is_empty(), NO_EX);
        group::remove(d, &mut g, &|d, x| !card::is_has_effect(d, x, 511002115, None).is_empty(), NO_EX);
        card::set_material(d, c, g.clone());
        duel::overlay(d, c, g, true);
    })
}
/// `Xyz.Condition2(alterf, op)`: `function(e, c, must, og, min, max)`
fn condition2_fn(alterf: AlterFilter, op: Option<LuaFn>) -> LuaFn {
    lua_fn(move |d, a| {
        let e = a.e(0);
        let Some(c) = a.card(1) else { return true };
        if card::is_type(d, c, TYPE_PENDULUM) && card::is_faceup(d, c) {
            return false;
        }
        let tp = card::get_controler(d, c);
        let must = a.v(2).as_group().cloned();
        let og = a.v(3).as_group().cloned();
        let min = opt_count(a.v(4));
        let mg = match &og {
            Some(og) => og.clone(),
            None => duel::get_field_group(d, tp, LOCATION_MZONE, LOCATION_MZONE),
        };
        // (`GetMustBeMaterialGroup(tp, og, ...)`: `og` may be nil there)
        let mut mustg = aux::get_must_be_material_group_opt(d, tp, og.as_ref(), tp, Some(c), Some(&mg), REASON_XYZ);
        if let Some(m) = &must {
            group::add_card(&mut mustg, m);
        }
        if mustg.len() > 1 || min.is_some_and(|m| m > 1) || !group::includes(&mg, &mustg) {
            return false;
        }
        match group::get_first(d, &mustg) {
            Some(mustc) => alter_filter_check(d, mustc, &alterf, c, e, tp, &op),
            None => group::is_exists(d, &mg, &|d, x| alter_filter_check(d, x, &alterf, c, e, tp, &op), 1, NO_EX),
        }
    })
}
/// `Xyz.Target2(alterf, op)`: `function(e,tp,eg,ep,ev,re,r,rp,chk,c,must,og,min,max)`
fn target2_fn(alterf: AlterFilter, op: Option<LuaFn>) -> LuaFn {
    lua_fn(move |d, a| {
        let (e, tp) = (a.e(0), a.p(1));
        let c = a.c(9);
        let must = a.v(10).as_group().cloned();
        let og = a.v(11).as_group().cloned();
        let (min, max) = (opt_count(a.v(12)), opt_count(a.v(13)));
        let cancelable = og.is_none() && duel::is_summon_cancelable(d);
        d.set_script_var(PROC_CANCELLABLE, Value::Bool(cancelable));
        let mg = match &og {
            Some(og) => og.clone(),
            None => duel::get_field_group(d, tp, LOCATION_MZONE, LOCATION_MZONE),
        };
        let mut mustg = aux::get_must_be_material_group_opt(d, tp, og.as_ref(), tp, Some(c), Some(&mg), REASON_XYZ);
        if let Some(m) = &must {
            group::add_card(&mut mustg, m);
        }
        let oc = if must.as_ref().is_some_and(|m| Some(m.len()) == min && Some(m.len()) == max) {
            group::get_first(d, &mustg)
        } else if !mustg.is_empty() {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_XMATERIAL as u64);
            group::select(d, &mustg, tp, 1, 1, cancelable, NO_EX).and_then(|g| group::get_first(d, &g))
        } else {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_XMATERIAL as u64);
            group::filter_select(d, &mg, tp, &|d, x| alter_filter_check(d, x, &alterf, c, e, tp, &op), 1, 1, cancelable, NO_EX).and_then(|g| group::get_first(d, &g))
        };
        let Some(oc) = oc else { return false };
        if op.as_ref().is_some_and(|op| !op.call(d, &[Value::Effect(e), Value::Int(tp as i64), Value::Int(1), Value::Card(oc)]).truthy()) {
            return false;
        }
        effect::set_label_object(d, e, Value::Card(oc));
        true
    })
}
/// `Xyz.Operation2(alterf, op)`
fn operation2_fn() -> LuaFn {
    lua_fn(|d, a| {
        let e = a.e(0);
        let c = a.c(8);
        let oc = effect::get_label_object(d, e).as_card().nil();
        card::set_material(d, c, CardSet::from_card(oc));
        duel::overlay(d, c, oc, false);
    })
}
