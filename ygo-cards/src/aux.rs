//! `Auxiliary` (`aux.*`, `utility.lua`). The function combinators of `utility.lua`
//! (`aux.AND`, `aux.FilterBoolFunction`, ...) are closures in Rust and not ported as functions.

use crate::constants::*;
use crate::lua::{card, duel, effect};
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
    let mut sg = CardSet::new();
    for te in duel::get_player_effect(d, tp, EFFECT_MUST_BE_MATERIAL) {
        let val = match effect::get_value(d, te) {
            Value::Fn(f) => f.call(d, &[Value::Effect(te), Value::Group(eg.clone()), Value::Int(sump as i64), Value::card(sc), Value::Group(g.clone())]).int(),
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
