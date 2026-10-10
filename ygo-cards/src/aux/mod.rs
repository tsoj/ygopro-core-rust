//! `Auxiliary` (`aux.*`, `utility.lua`). The function combinators of `utility.lua`
//! (`aux.AND`, `aux.FilterBoolFunction`, ...) are closures in Rust and not ported as functions.

use crate::constants::*;
use crate::lua::{card, duel, effect, group, CardOrGroup, Filter, NO_EX};
use crate::prelude::*;

/// `aux.Stringid(code, id)`: a description from the card's strings.
pub fn stringid(code: u32, id: u32) -> u64 {
    ((id & 0xfffff) as u64) | ((code as u64) << 20)
}

/// One entry of `aux.GetExtraMaterials`: the extra materials, the check function (the effect's
/// operation, `aux.TRUE` if it has none) and the effect.
#[derive(Clone)]
pub struct ExtraMaterial {
    pub group: CardSet,
    pub check: Option<LuaFn>,
    pub effect: EffectId,
}

/// `aux.GetExtraMaterials(tp, mustg, sc, summon_type)`: the EFFECT_EXTRA_MATERIAL entries and all
/// their cards.
pub fn get_extra_materials(d: &mut Duel, tp: u8, mustg: Option<&CardSet>, sc: Option<CardId>, summon_type: u32) -> (Vec<ExtraMaterial>, CardSet) {
    let mut tg = CardSet::new();
    let mut t = Vec::new();
    for te in duel::get_player_effect(d, tp, EFFECT_EXTRA_MATERIAL) {
        if effect::check_count_limit(d, te, tp) {
            let Value::Fn(f) = effect::get_value(d, te) else {
                crate::lua_error("attempt to call a number value");
            };
            let r = f.call(d, &[Value::Int(0), Value::Int(summon_type as i64), Value::Effect(te), Value::Int(tp as i64), Value::card(sc)]);
            let mut eg = r.as_group().cloned().unwrap_or_default();
            if let Some(m) = mustg {
                eg = ygo_core::api::group::difference(&eg, m);
            }
            tg = ygo_core::api::group::union(&tg, &eg);
            let check = effect::get_operation(d, te);
            t.push(ExtraMaterial { group: eg, check, effect: te });
        }
    }
    (t, tg)
}

/// `aux.CheckValidExtra(c, tp, sg, mg, lc, emt, filt)`: whether an extra-material entry allows
/// `c`; the entries that do are added to `filt`.
#[allow(clippy::too_many_arguments)]
pub fn check_valid_extra(d: &mut Duel, c: CardId, tp: u8, sg: &CardSet, mg: &CardSet, lc: CardId, emt: &[ExtraMaterial], filt: &mut Vec<ExtraMaterial>) -> bool {
    let mut res = false;
    for ex in emt {
        if !ex.group.contains(c) {
            continue;
        }
        let ok = match &ex.check {
            None => true,
            Some(f) => f
                .call(
                    d,
                    &[
                        Value::Card(c),
                        Value::Effect(ex.effect),
                        Value::Int(tp as i64),
                        Value::Group(sg.clone()),
                        Value::Group(mg.clone()),
                        Value::Card(lc),
                        Value::Group(ex.group.clone()),
                        Value::Int(0),
                    ],
                )
                .truthy(),
        };
        if ok {
            res = true;
            filt.push(ex.clone());
        }
    }
    res
}

/// `aux.DeleteExtraMaterialGroups(emt)`: calls each value function with mode 2 (the groups
/// themselves are values here).
pub fn delete_extra_material_groups(d: &mut Duel, emt: &[ExtraMaterial]) {
    for ex in emt {
        if let Value::Fn(f) = effect::get_value(d, ex.effect) {
            f.call(d, &[Value::Int(2), Value::Nil, Value::Effect(ex.effect), Value::Group(ex.group.clone())]);
        }
    }
}

/// `aux.GetMustBeMaterialGroup(tp, eg, sump, sc, g, r)`
pub fn get_must_be_material_group(d: &mut Duel, tp: u8, eg: &CardSet, sump: u8, sc: Option<CardId>, g: &CardSet, r: u32) -> CardSet {
    get_must_be_material_group_opt(d, tp, Some(eg), sump, sc, Some(g), r)
}
/// `aux.GetMustBeMaterialGroup` where the Lua passes `nil` groups
pub fn get_must_be_material_group_opt(d: &mut Duel, tp: u8, eg: Option<&CardSet>, sump: u8, sc: Option<CardId>, g: Option<&CardSet>, r: u32) -> CardSet {
    let mut sg = CardSet::new();
    let grp = |g: Option<&CardSet>| g.map_or(Value::Nil, |g| Value::Group(g.clone()));
    for te in duel::get_player_effect(d, tp, EFFECT_MUST_BE_MATERIAL) {
        let val = match effect::get_value(d, te) {
            Value::Fn(f) => f.call(d, &[Value::Effect(te), grp(eg), Value::Int(sump as i64), Value::card(sc), grp(g)]).int(),
            v => v.int(),
        };
        if val & r as i64 > 0 {
            if let Some(h) = effect::get_handler(d, te) {
                sg.insert(h);
            }
        }
    }
    sg
}

/// `aux.bdcon`: condition of EVENT_BATTLE_DESTROYING
pub fn bdcon() -> LuaFn {
    condition(|d, a| {
        let c = effect::get_handler(d, a.e).unwrap();
        card::is_relate_to_battle(d, c)
    })
}
/// `aux.bdocon`: EVENT_BATTLE_DESTROYING + opponent's monster
pub fn bdocon() -> LuaFn {
    condition(|d, a| {
        let c = effect::get_handler(d, a.e).unwrap();
        card::is_relate_to_battle(d, c) && card::is_status(d, c, STATUS_OPPO_BATTLE)
    })
}
fn battle_target_in_grave(d: &mut Duel, c: CardId) -> bool {
    match card::get_battle_target(d, c) {
        Some(bc) => card::is_location(d, bc, LOCATION_GRAVE) && card::is_monster(d, bc),
        // (Lua: indexing nil is an error)
        None => crate::lua_error("attempt to index a nil value (local 'bc')"),
    }
}
/// `aux.bdgcon`: EVENT_BATTLE_DESTROYING + to the GY
pub fn bdgcon() -> LuaFn {
    condition(|d, a| {
        let c = effect::get_handler(d, a.e).unwrap();
        card::is_relate_to_battle(d, c) && battle_target_in_grave(d, c)
    })
}
/// `aux.bdogcon`: EVENT_BATTLE_DESTROYING + opponent's monster + to the GY
pub fn bdogcon() -> LuaFn {
    condition(|d, a| {
        let c = effect::get_handler(d, a.e).unwrap();
        card::is_relate_to_battle(d, c) && card::is_status(d, c, STATUS_OPPO_BATTLE) && battle_target_in_grave(d, c)
    })
}
/// `aux.dogcon`: EVENT_TO_GRAVE + destroyed by the opponent from the field
pub fn dogcon() -> LuaFn {
    condition(|d, a| {
        let c = effect::get_handler(d, a.e).unwrap();
        card::is_previous_controler(d, c, a.tp) && card::is_reason(d, c, REASON_DESTROY) && a.rp == 1 - a.tp
    })
}
/// `aux.exccon`: "except the turn this card was sent to the GY"
pub fn exccon() -> LuaFn {
    LuaFn::new(|d, a| {
        let e = a[0].as_effect().unwrap();
        let c = effect::get_handler(d, e).unwrap();
        Value::Bool(duel::get_turn_count(d, None) as u32 != card::get_turn_id(d, c) || card::is_reason(d, c, REASON_RETURN))
    })
}
/// `aux.chainreg`: flag effect for Spell Counters
pub fn chainreg() -> LuaFn {
    operation(|d, a| {
        let c = effect::get_handler(d, a.e).unwrap();
        if card::get_flag_effect(d, c, 1) == 0 {
            card::register_flag_effect(d, c, 1, RESET_EVENT + RESETS_STANDARD - RESET_TURN_SET + RESET_CHAIN, 0, 1, 0, 0);
        }
    })
}
/// `aux.imval1`: default filter for EFFECT_CANNOT_BE_BATTLE_TARGET
pub fn imval1() -> LuaFn {
    effect_card_fn(|d, e, c| !card::is_immune_to_effect(d, c, e))
}
/// `aux.imval2`: EFFECT_CANNOT_BE_BATTLE_TARGET + opponent's
pub fn imval2() -> LuaFn {
    effect_card_fn(|d, e, c| !card::is_immune_to_effect(d, c, e) && card::get_controler(d, c) != effect::get_handler_player(d, e))
}
fn re_rp_fn(f: fn(&mut Duel, EffectId, u8) -> bool) -> LuaFn {
    LuaFn::new(move |d, a| {
        let e = a[0].as_effect().unwrap();
        let rp = a.get(2).map_or(0, |v| v.int()) as u8;
        Value::Bool(f(d, e, rp))
    })
}
/// `aux.tgoval`: EFFECT_CANNOT_BE_EFFECT_TARGET + opponent
pub fn tgoval() -> LuaFn {
    re_rp_fn(|d, e, rp| rp != effect::get_handler_player(d, e))
}
/// `aux.indsval`: EFFECT_INDESTRUCTABLE_EFFECT + self
pub fn indsval() -> LuaFn {
    re_rp_fn(|d, e, rp| rp == effect::get_handler_player(d, e))
}
/// `aux.indoval`: EFFECT_INDESTRUCTABLE_EFFECT + opponent
pub fn indoval() -> LuaFn {
    re_rp_fn(|d, e, rp| rp as i32 == 1 - effect::get_handler_player(d, e) as i32)
}
/// `aux.sumreg`: flag effect for the summon turn (code = the effect's label)
pub fn sumreg() -> LuaFn {
    operation(|d, a| {
        let code = effect::get_label(d, a.e) as u32;
        for tc in crate::lua::group::iter(d, &a.eg()) {
            if card::get_original_code(d, tc) == code {
                card::register_flag_effect(d, tc, code, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
            }
        }
    })
}
/// `aux.sumlimit(sumtype)`: a summon condition (value `function(e, se, sp, st)`)
pub fn sumlimit(sumtype: u32) -> LuaFn {
    LuaFn::new(move |_d, a| {
        let st = a.get(3).map_or(0, |v| v.int()) as u32;
        Value::Bool(st & sumtype == sumtype)
    })
}
pub fn fuslimit() -> LuaFn {
    sumlimit(SUMMON_TYPE_FUSION)
}
pub fn ritlimit() -> LuaFn {
    sumlimit(SUMMON_TYPE_RITUAL)
}
pub fn synlimit() -> LuaFn {
    sumlimit(SUMMON_TYPE_SYNCHRO)
}
pub fn xyzlimit() -> LuaFn {
    sumlimit(SUMMON_TYPE_XYZ)
}
pub fn penlimit() -> LuaFn {
    sumlimit(SUMMON_TYPE_PENDULUM)
}
pub fn lnklimit() -> LuaFn {
    sumlimit(SUMMON_TYPE_LINK)
}
/// `aux.cannotmatfilter(types...)`: value for EFFECT_CANNOT_BE_MATERIAL, `function(e, c, sumtype, tp)`
pub fn cannotmatfilter(allowed: &[u32]) -> LuaFn {
    let allowed = allowed.to_vec();
    let tot = allowed.iter().fold(0u32, |a, &v| a | v);
    LuaFn::new(move |_d, a| {
        let sumtype = a.get(2).map_or(0, |v| v.int()) as u32;
        let sum = tot & sumtype;
        Value::Bool(allowed.contains(&sum))
    })
}
/// `aux.damcon1`: the effect would inflict damage to `tp`
pub fn damcon1() -> LuaFn {
    condition(|d, a| {
        let e1 = duel::is_player_affected_by_effect(d, a.tp, EFFECT_REVERSE_DAMAGE).is_some();
        let e2 = duel::is_player_affected_by_effect(d, a.tp, EFFECT_REVERSE_RECOVER).is_some();
        let rd = e1 && !e2;
        let rr = !e1 && e2;
        let hits = |o: &Option<ygo_core::duel::OpTarget>| o.as_ref().is_some_and(|o| o.op_player == a.tp || o.op_player as u32 == PLAYER_ALL);
        let ex = duel::get_operation_info(d, a.ev, CATEGORY_DAMAGE);
        if hits(&ex) && !rd && duel::is_player_affected_by_effect(d, a.tp, EFFECT_NO_EFFECT_DAMAGE).is_none() {
            return true;
        }
        let ex = duel::get_operation_info(d, a.ev, CATEGORY_RECOVER);
        hits(&ex) && rr && duel::is_player_affected_by_effect(d, a.tp, EFFECT_NO_EFFECT_DAMAGE).is_none()
    })
}
/// `aux.StatChangeDamageStepCondition()`
pub fn stat_change_damage_step_condition(d: &Duel) -> bool {
    !(duel::is_phase(d, PHASE_DAMAGE) && duel::is_damage_calculated(d))
}

/// The `rescon` of `aux.SelectUnselectGroup`: `function(sg, e, tp, mg, c)` returning
/// (the group is valid, stop extending it).
pub type Rescon<'a> = &'a dyn Fn(&mut Duel, &CardSet, Option<EffectId>, u8, &CardSet, Option<CardId>) -> (bool, bool);

struct LoopCtx<'a> {
    mg: &'a CardSet,
    e: Option<EffectId>,
    tp: u8,
    minc: usize,
    maxc: usize,
    rescon: Option<Rescon<'a>>,
    /// (ygoai_overrides.lua) whether a group can be completed, per group, for one top-level call
    memo: std::collections::HashMap<Vec<CardId>, bool>,
}

fn select_unselect_loop_inner(d: &mut Duel, x: &mut LoopCtx, c: CardId, sg: &mut CardSet) -> bool {
    let mut res = x.rescon.is_none();
    if sg.len() >= x.maxc {
        return false;
    }
    sg.insert(c);
    if let Some(rescon) = x.rescon {
        let (r, stop) = rescon(d, sg, x.e, x.tp, x.mg, Some(c));
        res = r;
        if stop {
            sg.remove(c);
            return false;
        }
    }
    if sg.len() < x.minc || (sg.len() < x.maxc && !res) {
        res = extends(d, x, sg);
    }
    sg.remove(c);
    res
}

/// `mg:IsExists(aux.SelectUnselectLoop, 1, sg, sg, mg, ...)`, remembered per group.
fn extends(d: &mut Duel, x: &mut LoopCtx, sg: &mut CardSet) -> bool {
    let key = sg.to_vec();
    if let Some(&r) = x.memo.get(&key) {
        return r;
    }
    let mut res = false;
    for c in crate::lua::group::iter(d, x.mg) {
        if sg.contains(c) {
            continue;
        }
        if crate::lua::protected(d, false, |d| select_unselect_loop_inner(d, x, c, sg)) {
            res = true;
            break;
        }
    }
    x.memo.insert(key, res);
    res
}

/// `aux.SelectUnselectLoop(c, sg, mg, e, tp, minc, maxc, rescon)`: whether adding `c` to `sg`
/// can lead to a valid group.
#[allow(clippy::too_many_arguments)]
pub fn select_unselect_loop(d: &mut Duel, c: CardId, sg: &mut CardSet, mg: &CardSet, e: Option<EffectId>, tp: u8, minc: usize, maxc: usize, rescon: Option<Rescon>) -> bool {
    let mut x = LoopCtx { mg, e, tp, minc, maxc, rescon, memo: Default::default() };
    select_unselect_loop_inner(d, &mut x, c, sg)
}

/// `aux.SelectUnselectGroup(g, e, tp, minc = 1, maxc = #g, rescon, chk, seltp, hintmsg, finishcon, breakcon, cancelable)`:
/// with `chk == 0` whether a valid group exists (`Err`), else the group chosen card by card (`Ok`).
#[allow(clippy::too_many_arguments)]
pub fn select_unselect_group(
    d: &mut Duel,
    g: &CardSet,
    e: Option<EffectId>,
    tp: u8,
    minc: Option<usize>,
    maxc: Option<usize>,
    rescon: Option<Rescon>,
    chk: i64,
    seltp: u8,
    hintmsg: Option<u64>,
    finishcon: Option<Rescon>,
    breakcon: Option<Rescon>,
    cancelable: bool,
) -> Result<CardSet, bool> {
    let minc = minc.unwrap_or(1);
    let maxc = maxc.unwrap_or(g.len());
    if chk == 0 {
        if g.len() < minc {
            return Err(false);
        }
        let mut eg = g.clone();
        for c in crate::lua::group::iter(d, g) {
            let mut sg = CardSet::new();
            if select_unselect_loop(d, c, &mut sg, &eg, e, tp, minc, maxc, rescon) {
                return Err(true);
            }
            eg.remove(c);
        }
        return Err(false);
    }
    let hintmsg = hintmsg.unwrap_or(0);
    let mut sg = CardSet::new();
    loop {
        let finishable = sg.len() >= minc && finishcon.is_none_or(|f| f(d, &sg, e, tp, g, None).0);
        let mut mg = CardSet::new();
        for c in crate::lua::group::iter(d, g) {
            if sg.contains(c) {
                continue;
            }
            let mut sg2 = sg.clone();
            if crate::lua::protected(d, false, |d| select_unselect_loop(d, c, &mut sg2, g, e, tp, minc, maxc, rescon)) {
                mg.insert(c);
            }
        }
        if breakcon.is_some_and(|b| b(d, &sg, e, tp, &mg, None).0) || mg.is_empty() || sg.len() >= maxc {
            break;
        }
        duel::hint(d, HINT_SELECTMSG as u8, seltp, hintmsg);
        let tc = crate::lua::group::select_unselect(d, &mg, Some(&sg), seltp, finishable, finishable || (cancelable && sg.is_empty()), Some(minc as u16), Some(maxc as u16));
        let Some(tc) = tc else { break };
        if sg.contains(tc) {
            sg.remove(tc);
        } else {
            sg.insert(tc);
        }
    }
    Ok(sg)
}

/// `aux.ChkfMMZ(sumcount)`: rescon "enough free Monster Zones after the group leaves"
pub fn chkf_mmz(sumcount: i32) -> impl Fn(&mut Duel, &CardSet, Option<EffectId>, u8, &CardSet, Option<CardId>) -> (bool, bool) {
    move |d, sg, _e, tp, _mg, _c| (duel::get_m_zone_count(d, tp, sg.into(), None, None, None).0 >= sumcount, false)
}

/// `aux.ResetEffects(g, code)`: resets the effects with that code of the cards.
pub fn reset_effects(d: &mut Duel, g: &CardSet, code: u32) {
    for c in crate::lua::group::iter(d, g) {
        for v in card::get_card_effect(d, c, code) {
            effect::reset(d, v);
        }
    }
}
/// `aux.IsZone(c, zone, tp)`
pub fn is_zone(d: &Duel, c: CardId, zone: u32, tp: u8) -> bool {
    let seq = card::get_sequence(d, c);
    let mine = card::is_controler(d, c, tp);
    let mut rzone = if mine { 1u32 << seq } else { 1u32 << (16 + seq) };
    if seq == 5 || seq == 6 {
        rzone |= if mine { 1 << (16 + 11 - seq) } else { 1 << (11 - seq) };
    }
    rzone & zone > 0
}
/// `aux.BitSplit(v)`: the set bits as values.
pub fn bit_split(v: u64) -> Vec<u64> {
    (0..64).map(|i| 1u64 << i).filter(|p| *p <= v && v & p != 0).collect()
}
pub fn get_coin_count_from_ev(ev: u32) -> u32 {
    ev & 0xff
}
pub fn get_coin_heads_from_ev(ev: u32) -> u32 {
    (ev >> 8) & 0xff
}
pub fn get_coin_tails_from_ev(ev: u32) -> u32 {
    (ev >> 16) & 0xff
}
pub fn get_dice_count_self_from_ev(ev: u32) -> u32 {
    ev & 0xffff
}
pub fn get_dice_count_oppo_from_ev(ev: u32) -> u32 {
    (ev >> 16) & 0xffff
}
/// `aux.GetCoinEffectHintString(coin)`
pub fn get_coin_effect_hint_string(coin: i32) -> Option<u64> {
    match coin as u32 {
        COIN_HEADS => Some(62),
        COIN_TAILS => Some(63),
        _ => None,
    }
}
/// `aux.dpcheck(fun)`: rescon "all cards differ in fun" (fun: `function(c)` returning a number)
pub fn dpcheck(fun: fn(&mut Duel, CardId) -> i64) -> impl Fn(&mut Duel, &CardSet, Option<EffectId>, u8, &CardSet, Option<CardId>) -> (bool, bool) {
    move |d, sg, _e, _tp, _mg, _c| {
        let c1 = crate::lua::group::get_class_count(d, sg, &fun);
        let c2 = sg.len();
        (c1 == c2, c1 != c2)
    }
}
/// `aux.dncheck`: rescon "all cards have different names" (by their first code)
pub fn dncheck() -> impl Fn(&mut Duel, &CardSet, Option<EffectId>, u8, &CardSet, Option<CardId>) -> (bool, bool) {
    dpcheck(|d, c| card::get_code(d, c)[0] as i64)
}
// `aux.MainAndExtraSpSummonLoop` is not ported: no card uses it, and it calls the undefined
// `Duel.GetMasterRule`.

/// `aux.seqmovcon`: the handler can move to an adjacent zone
pub fn seqmovcon() -> LuaFn {
    condition(|d, a| {
        let c = effect::get_handler(d, a.e).unwrap();
        card::check_adjacent(d, c)
    })
}
/// `aux.seqmovop`: move to an adjacent zone chosen at resolution
pub fn seqmovop() -> LuaFn {
    operation(|d, a| {
        let c = effect::get_handler(d, a.e).unwrap();
        if !card::is_relate_to_effect(d, c, a.e) || card::is_controler(d, c, 1 - a.tp) {
            return;
        }
        card::move_adjacent(d, c, None);
    })
}
/// `aux.seqmovtg`: choose the adjacent zone at target time (in the label)
pub fn seqmovtg() -> LuaFn {
    cost_target(|d, a| {
        if a.chk == Some(0) {
            return true;
        }
        let c = effect::get_handler(d, a.e).unwrap();
        // (Lua: `SelectAdjacent` returning nothing leaves `SetLabel` without its argument)
        let seq = card::select_adjacent(d, c, None).unwrap_or_else(|| crate::lua_error("bad argument #2 to 'SetLabel'"));
        effect::set_label(d, a.e, &[seq as i64]);
        true
    })
}
/// `aux.seqmovtgop`: move to the zone chosen by `aux.seqmovtg`
pub fn seqmovtgop() -> LuaFn {
    operation(|d, a| {
        let c = effect::get_handler(d, a.e).unwrap();
        let seq = effect::get_label(d, a.e);
        if !card::is_relate_to_effect(d, c, a.e) || card::is_controler(d, c, 1 - a.tp) || !duel::check_location(d, a.tp, LOCATION_MZONE, seq as u32) {
            return;
        }
        duel::move_sequence(d, c, seq as u8, None);
    })
}

/// `aux.GetAttributeStrings(v)`: the hint strings of the attributes in `v`
pub fn get_attribute_strings(v: u64) -> Vec<u64> {
    let t = [ATTRIBUTE_EARTH, ATTRIBUTE_WATER, ATTRIBUTE_FIRE, ATTRIBUTE_WIND, ATTRIBUTE_LIGHT, ATTRIBUTE_DARK, ATTRIBUTE_DIVINE];
    bit_split(v).into_iter().filter_map(|a| t.iter().position(|x| *x as u64 == a).map(|i| 1010 + i as u64)).collect()
}
/// `aux.GetRaceStrings(v)`
pub fn get_race_strings(v: u64) -> Vec<u64> {
    let t = [
        RACE_WARRIOR, RACE_SPELLCASTER, RACE_FAIRY, RACE_FIEND, RACE_ZOMBIE, RACE_MACHINE, RACE_AQUA, RACE_PYRO, RACE_ROCK, RACE_WINGEDBEAST, RACE_PLANT, RACE_INSECT, RACE_THUNDER,
        RACE_DRAGON, RACE_BEAST, RACE_BEASTWARRIOR, RACE_DINOSAUR, RACE_FISH, RACE_SEASERPENT, RACE_REPTILE, RACE_PSYCHIC, RACE_DIVINE, RACE_CREATORGOD, RACE_WYRM, RACE_CYBERSE, RACE_ILLUSION,
    ];
    bit_split(v).into_iter().filter_map(|r| t.iter().position(|x| *x as u64 == r).map(|i| 1020 + i as u64)).collect()
}
/// `aux.FaceupFilter(f, ...)`: `f` on face-up cards only (the arguments go into the closure)
pub fn faceup_filter(f: impl Fn(&mut Duel, CardId) -> bool) -> impl Fn(&mut Duel, CardId) -> bool {
    move |d, c| card::is_faceup(d, c) && f(d, c)
}
/// `aux.zptfilter(c, ec)`: `c` left a zone `ec` points to
pub fn zptfilter(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    let pc = card::get_previous_controler(d, c);
    !card::is_location(d, c, LOCATION_MZONE) && card::get_linked_zone(d, ec, Some(pc)) & (1 << card::get_previous_sequence(d, c)) != 0
}
fn zpt_filtered(d: &mut Duel, eg: &CardSet, filter: Option<&dyn Fn(&mut Duel, CardId, u8) -> bool>, tp: u8) -> CardSet {
    group::filter(d, eg, &|d, cc| filter.map_or(true, |f| f(d, cc, tp)), NO_EX)
}
/// `aux.zptgroup(eg, filter, c, tp)`: the summoned `filter` monsters in zones `c` points to
pub fn zptgroup(d: &mut Duel, eg: &CardSet, filter: Option<&dyn Fn(&mut Duel, CardId, u8) -> bool>, c: CardId, tp: u8) -> CardSet {
    let fil = zpt_filtered(d, eg, filter, tp);
    let lg = card::get_linked_group(d, c);
    let gone = group::filter(d, eg, &|d, cc| zptfilter(d, cc, c), NO_EX);
    group::union(&group::intersection(&fil, &lg), &gone)
}
pub fn zptgroupcon(d: &mut Duel, eg: &CardSet, filter: Option<&dyn Fn(&mut Duel, CardId, u8) -> bool>, c: CardId, tp: u8) -> bool {
    let fil = zpt_filtered(d, eg, filter, tp);
    let lg = card::get_linked_group(d, c);
    !group::intersection(&fil, &lg).is_empty() || group::is_exists(d, eg, &|d, cc| zptfilter(d, cc, c), 1, NO_EX)
}
/// `aux.zptcon(filter)`
pub fn zptcon(filter: Option<fn(&mut Duel, CardId, u8) -> bool>) -> LuaFn {
    condition(move |d, a| {
        let c = effect::get_handler(d, a.e).unwrap();
        let f = filter.map(|f| move |d: &mut Duel, c: CardId, tp: u8| f(d, c, tp));
        zptgroupcon(d, &a.eg(), f.as_ref().map(|f| f as &dyn Fn(&mut Duel, CardId, u8) -> bool), c, a.tp)
    })
}

/// `aux.*` state (`Duel::script_var` keys).
pub mod vars {
    use crate::ns::AUX;
    /// `Auxiliary.ToResetFuncTable` (the functions of `aux.AddValuesReset`)
    pub const TO_RESET_FUNC_TABLE: (u32, u64) = (AUX, 1);
    /// `delayed_operation_id` (the last id of `aux.DelayedOperation`)
    pub const DELAYED_OPERATION_ID: (u32, u64) = (AUX, 2);
}

/// `aux.ToHandOrElse(card, player, check, oper, str, ...)`: add `targets` to the hand or (the
/// player's choice) do `oper` (default: send to the GY, if `check` (default: `IsAbleToGrave`)
/// holds for all of them). The arguments `...` go into the closures.
pub fn to_hand_or_else(
    d: &mut Duel,
    targets: impl Into<CardOrGroup>,
    player: u8,
    check: Option<&dyn Fn(&mut Duel, CardId) -> bool>,
    oper: Option<&dyn Fn(&mut Duel, &CardSet) -> i32>,
    str: Option<u64>,
) -> i32 {
    let targets = targets.into().cards();
    let str = str.unwrap_or(574);
    let (mut b1, mut b2) = (true, true);
    for ctg in group::iter(d, &targets) {
        if !card::is_able_to_hand(d, ctg, None) {
            b1 = false;
        }
        let ok = match check {
            Some(f) => f(d, ctg),
            None => card::is_able_to_grave(d, ctg, None),
        };
        if !ok {
            b2 = false;
        }
    }
    let opt = if b1 && b2 {
        duel::select_option(d, player, true, &[573, str])
    } else if b1 {
        duel::select_option(d, player, true, &[573])
    } else {
        duel::select_option(d, player, true, &[str]) + 1
    };
    if opt == 0 {
        let res = duel::send_to_hand(d, &targets, None, REASON_EFFECT, None);
        if res > 0 {
            let og = duel::get_operated_group(d);
            let og = group::filter(d, &og, &|d, c| card::is_previous_location(d, c, LOCATION_DECK), NO_EX);
            if !og.is_empty() {
                duel::confirm_cards(d, 1 - player, &og);
            }
        }
        res
    } else {
        match oper {
            Some(f) => f(d, &targets),
            None => thoe_send(d, &targets),
        }
    }
}
/// `aux.thoeSend(card)`
pub fn thoe_send(d: &mut Duel, targets: &CardSet) -> i32 {
    duel::send_to_grave(d, targets, REASON_EFFECT, None, None)
}
/// `aux.RegisterClientHint(card, property, tp, player1, player2, str, reset, ct)`: a player
/// effect that only shows a hint until the End Phase.
#[allow(clippy::too_many_arguments)]
pub fn register_client_hint(d: &mut Duel, c: CardId, property: u32, tp: u8, player1: u32, player2: u32, str: Option<u64>, reset: u32, ct: Option<u16>) -> EffectId {
    let eff = effect::create_effect(d, c);
    effect::set_property(d, eff, (EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT) | property, 0);
    effect::set_target_range(d, eff, player1, player2);
    let desc = str.unwrap_or_else(|| stringid(card::get_original_code(d, c), 1));
    effect::set_description(d, eff, desc);
    effect::set_reset(d, eff, (RESET_PHASE + PHASE_END) | reset, ct.unwrap_or(1));
    duel::register_effect(d, eff, tp);
    eff
}
/// `aux.FieldSummonProcTg(fun1, fun2)`: target of a summon procedure field effect: `fun1(e, tp)`
/// without a card, `fun2(e, tp, eg, ..., chk, c, ...)` with one.
pub fn field_summon_proc_tg(fun1: Option<LuaFn>, fun2: Option<LuaFn>) -> LuaFn {
    LuaFn::new(move |d, a| {
        if a.get(9).and_then(|v| v.as_card()).is_none() {
            return fun1.as_ref().map_or(Value::Bool(true), |f| f.call(d, &a[..2.min(a.len())]));
        }
        fun2.as_ref().map_or(Value::Bool(true), |f| f.call(d, a))
    })
}
/// `aux.AddValuesReset(resetfunc)`: `resetfunc` runs at the end of each turn.
pub fn add_values_reset(d: &mut Duel, resetfunc: fn(&mut Duel)) {
    let f = Value::Fn(LuaFn::new(move |d, _| {
        resetfunc(d);
        Value::Nil
    }));
    match d.script_var(vars::TO_RESET_FUNC_TABLE) {
        Value::Multi(mut t) => {
            t.push(f);
            d.set_script_var(vars::TO_RESET_FUNC_TABLE, Value::Multi(t));
        }
        _ => {
            d.set_script_var(vars::TO_RESET_FUNC_TABLE, Value::Multi(vec![f]));
            let ge = effect::global_effect(d);
            effect::set_type(d, ge, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            effect::set_code(d, ge, EVENT_TURN_END);
            effect::set_count_limit(d, ge, 1, 0, 0, 0);
            effect::set_condition(d, ge, Some(values_reset()));
            duel::register_effect(d, ge, 0);
        }
    }
}
/// `aux.ValuesReset`
fn values_reset() -> LuaFn {
    LuaFn::new(|d, _| {
        if let Value::Multi(t) = d.script_var(vars::TO_RESET_FUNC_TABLE) {
            for f in t {
                if let Value::Fn(f) = f {
                    f.call(d, &[]);
                }
            }
        }
        Value::Bool(false)
    })
}
/// `aux.GlobalCheck(s, func)`: `func` once per duel for the script `code`
pub fn global_check(d: &mut Duel, code: u32, func: impl FnOnce(&mut Duel)) {
    let k = (crate::ns::GLOBAL_CHECK, code as u64);
    if !d.script_var(k).truthy() {
        d.set_script_var(k, Value::Bool(true));
        func(d);
    }
}
/// `aux.HarmonizingMagFilter(c, e, f)`
pub fn harmonizing_mag_filter(d: &mut Duel, c: CardId, e: EffectId, f: Option<&LuaFn>) -> bool {
    f.is_some_and(|f| !f.call(d, &[Value::Effect(e), Value::Card(c)]).truthy())
}
/// `aux.ChangeBattleDamage(player, value)`: `value` as the battle damage to the effect's owner
/// (`player == 0`) or their opponent (`player == 1`)
pub fn change_battle_damage(player: u8, value: i64) -> LuaFn {
    LuaFn::new(move |d, a| {
        let e = a[0].as_effect().unwrap();
        let damp = a.get(1).map_or(0, |v| v.int()) as u8;
        let op = effect::get_owner_player(d, e);
        match player {
            0 => Value::Int(if op == damp { value } else { -1 }),
            1 => Value::Int(if op == 1 - damp { value } else { -1 }),
            _ => Value::Nil,
        }
    })
}
/// `aux.GetMMZonesPointedTo(player, by_filter, player_location, oppo_location, target_player, ...)`:
/// the Main Monster Zones of `target_player` that face-up Link cards (matching `by_filter`) point to
pub fn get_mm_zones_pointed_to(d: &mut Duel, player: u8, by_filter: Option<Filter>, player_location: Option<u32>, oppo_location: Option<u32>, target_player: Option<u8>) -> u32 {
    let loc1 = player_location.unwrap_or(LOCATION_MZONE);
    let loc2 = oppo_location.unwrap_or(loc1);
    let target_player = target_player.unwrap_or(player);
    let f = |d: &mut Duel, c: CardId| card::is_faceup(d, c) && card::is_type(d, c, TYPE_LINK) && by_filter.map_or(true, |f| f(d, c));
    let g = duel::get_matching_group(d, Some(&f), player, loc1, loc2, NO_EX);
    group::get_linked_zone(d, &g, target_player) & ZONES_MMZ
}

/// `oper`/`cond` of `aux.DelayedOperation`: `(ag, e, tp, eg, ep, ev, re, r, rp)`
pub type DelayedFn = std::sync::Arc<dyn Fn(&mut Duel, &CardSet, &EffArgs) -> bool + Send + Sync>;

/// `aux.DelayedOperation(card_or_group, phase, flag, e, tp, oper, cond, reset, reset_count, hint,
/// effect_desc)`: `oper` on the cards (still flagged with `flag`) when `phase` is entered.
/// `None` for no cards.
#[allow(clippy::too_many_arguments)]
pub fn delayed_operation(
    d: &mut Duel,
    targets: impl Into<CardOrGroup>,
    phase: u32,
    flag: u32,
    e: EffectId,
    tp: u8,
    oper: Option<DelayedFn>,
    cond: Option<DelayedFn>,
    reset: Option<u32>,
    reset_count: Option<u16>,
    hint: Option<u64>,
    effect_desc: Option<u64>,
) -> Option<EffectId> {
    let g = targets.into().cards();
    if g.is_empty() {
        return None;
    }
    let reset = reset.unwrap_or(RESET_PHASE | phase);
    let reset_count = reset_count.unwrap_or(1);
    let fid = d.script_var(vars::DELAYED_OPERATION_ID).int() + 1;
    d.set_script_var(vars::DELAYED_OPERATION_ID, Value::Int(fid));

    fn get_affected_group(d: &mut Duel, e: EffectId, flag: u32) -> CardSet {
        let lbl = effect::get_label(d, e);
        let Value::Group(g) = effect::get_label_object(d, e) else { return CardSet::new() };
        group::filter(d, &g, &|d, c| card::get_flag_effect_label(d, c, flag).first() == Some(&lbl), NO_EX)
    }
    let c = effect::get_handler(d, e).unwrap();
    let e1 = effect::create_effect(d, c);
    if let Some(desc) = effect_desc {
        effect::set_description(d, e1, desc);
    }
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e1, EVENT_PHASE | phase);
    effect::set_reset(d, e1, reset, reset_count);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_label(d, e1, &[fid]);
    effect::set_label_object(d, e1, Value::Group(g.clone()));
    effect::set_condition(
        d,
        e1,
        Some(condition(move |d, a| {
            let ag = get_affected_group(d, a.e, flag);
            !ag.is_empty() && cond.as_ref().map_or(true, |f| f(d, &ag, a))
        })),
    );
    effect::set_operation(
        d,
        e1,
        Some(operation(move |d, a| {
            if let Some(f) = &oper {
                let ag = get_affected_group(d, a.e, flag);
                f(d, &ag, a);
            }
        })),
    );
    duel::register_effect(d, e1, tp);

    let flagprop = if hint.is_some() { EFFECT_FLAG_CLIENT_HINT } else { 0 };
    for tc in group::iter(d, &g) {
        let fe = card::register_flag_effect(d, tc, flag, RESET_EVENT + RESETS_STANDARD, flagprop, 1, fid, hint.unwrap_or(0));
        effect::set_condition(d, fe, Some(LuaFn::new(move |d, _| Value::Bool(!effect::is_deleted(d, e1)))));
    }
    Some(e1)
}
/// `aux.RemoveUntil(card_or_group, pos, reason, phase, flag, e, tp, oper, cond, reset,
/// reset_count, hint, effect_desc)`: banish the cards, then `oper` on them when `phase` is
/// entered. (Lua filters a passed group in place to the banished cards.)
#[allow(clippy::too_many_arguments)]
pub fn remove_until(
    d: &mut Duel,
    targets: impl Into<CardOrGroup>,
    pos: Option<u8>,
    reason: u32,
    phase: u32,
    flag: u32,
    e: EffectId,
    tp: u8,
    oper: Option<DelayedFn>,
    cond: Option<DelayedFn>,
    reset: Option<u32>,
    reset_count: Option<u16>,
    hint: Option<u64>,
    effect_desc: Option<u64>,
) -> Option<EffectId> {
    let g = targets.into().cards();
    if g.is_empty() || duel::remove(d, &g, pos.unwrap_or(0), reason | REASON_TEMPORARY, None, None) == 0 {
        return None;
    }
    let g = group::filter(d, &g, &|d, c| card::is_location(d, c, LOCATION_REMOVED), NO_EX);
    if g.is_empty() {
        return None;
    }
    delayed_operation(d, g, phase, flag, e, tp, oper, cond, reset, reset_count, hint, effect_desc)
}
fn select_field_return_cards(d: &mut Duel, p: u8, mut g: CardSet) {
    if g.is_empty() {
        return;
    }
    let ft = duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0;
    if ft > 0 && g.len() as i32 > ft {
        duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_TOFIELD as u64);
        let tg = group::select(d, &g, p, ft as u16, ft as u16, false, NO_EX).unwrap_or_default();
        for tc in group::iter(d, &tg) {
            duel::return_to_field(d, tc, None, None);
        }
        g = group::difference(&g, &tg);
    }
    for tc in group::iter(d, &g) {
        duel::return_to_field(d, tc, None, None);
    }
}
/// `aux.DefaultFieldReturnOp(rg)`: return the banished cards to the field, turn player first
/// (each player chooses if they have fewer zones than cards)
pub fn default_field_return_op(d: &mut Duel, rg: &CardSet) {
    if rg.is_empty() {
        return;
    }
    let turn_p = duel::get_turn_player(d);
    let (g0, g1) = group::split(d, rg, &|d, c| card::is_previous_controler(d, c, turn_p), NO_EX);
    select_field_return_cards(d, turn_p, g0);
    select_field_return_cards(d, 1 - turn_p, g1);
}
/// `aux.DefaultFieldReturnOp` as the `oper` of `aux.RemoveUntil`
pub fn default_field_return() -> DelayedFn {
    std::sync::Arc::new(|d, rg, _| {
        default_field_return_op(d, rg);
        true
    })
}

mod specific;
pub use specific::*;
mod workaround;
pub use workaround::*;
/// `aux.RemainFieldCost`, `aux.RemainFieldDisabled`, `aux.bfgcost` (= `Cost.SelfBanish`)
pub use crate::cost::{remain_field_cost, remain_field_disabled, self_banish as bfgcost};
mod equip;
pub use equip::*;
mod persistent;
pub use persistent::*;
mod union;
pub use union::*;
mod normal;
pub use normal::*;

/// `aux.FilterSummonCode(codes...)`: a material filter `c:IsSummonCode(scard, sumtype, tp, codes...)`
pub fn filter_summon_code(codes: &'static [u32]) -> crate::procs::MatFilter {
    std::sync::Arc::new(move |d, c, sc, st, tp| card::is_summon_code(d, c, sc, st, tp, codes))
}
/// `aux.PropertyTableFilter(f, allowed...)`: the values of `f(c)` that are in `allowed` (the Lua
/// remembers them per card; `f` is pure, so this is the same)
pub fn property_table_filter(f: impl Fn(&mut Duel, CardId) -> Vec<i64>, allowed: &'static [i64]) -> impl Fn(&mut Duel, CardId) -> Vec<i64> {
    move |d, c| f(d, c).into_iter().filter(|v| allowed.contains(v)).collect()
}
