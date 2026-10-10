//! `proc_equip.lua`: Equip Spells that equip by rule (`aux.AddEquipProcedure`).

use crate::constants::*;
use crate::lua::{card, duel, effect, group, NO_EX};
use crate::prelude::*;
use std::sync::Arc;

/// `f(c, e, tp)`: the monsters an Equip Spell can equip to
pub type EquipFilter = Arc<dyn Fn(&mut Duel, CardId, EffectId, u8) -> bool + Send + Sync>;
pub fn equip_filter_fn(f: impl Fn(&mut Duel, CardId, EffectId, u8) -> bool + Send + Sync + 'static) -> EquipFilter {
    Arc::new(f)
}
/// `tg(e, tp, eg, ep, ev, re, r, rp, tc)`: more of the target (with the chosen monster)
pub type EquipTg = Arc<dyn Fn(&mut Duel, &EffArgs, Option<CardId>) + Send + Sync>;

/// `aux.AddEquipProcedure(c, p, f, eqlimit, cost, tg, op, con, prop)`: activate by equipping to a
/// face-up monster (`p`: 0 yours, 1 your opponent's, `None`/PLAYER_ALL both players') matching `f`
#[allow(clippy::too_many_arguments)]
pub fn add_equip_procedure(
    d: &mut Duel,
    c: CardId,
    p: Option<u32>,
    f: Option<EquipFilter>,
    eqlimit: Option<LuaFn>,
    cost: Option<LuaFn>,
    tg: Option<EquipTg>,
    op: Option<LuaFn>,
    con: Option<LuaFn>,
    prop: Option<u32>,
) -> EffectId {
    let property = prop.unwrap_or(0);
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, 1068);
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_CONTINUOUS_TARGET + property, 0);
    if con.is_some() {
        effect::set_condition(d, e1, con);
    }
    if cost.is_some() {
        effect::set_cost(d, e1, cost);
    }
    effect::set_target(d, e1, Some(equip_target(tg, p, f.clone())));
    effect::set_operation(d, e1, Some(equip_operation(op)));
    card::register_effect(d, c, e1, false);
    // Equip limit
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_EQUIP_LIMIT);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_value_fn(d, e2, eqlimit.unwrap_or_else(|| equip_limit(f)));
    card::register_effect(d, c, e2, false);
    e1
}
/// `aux.EquipLimit(f)`
pub fn equip_limit(f: Option<EquipFilter>) -> LuaFn {
    effect_card_fn(move |d, e, c| {
        let hp = effect::get_handler_player(d, e);
        f.as_ref().is_none_or(|f| f(d, c, e, hp))
    })
}
/// `aux.EquipFilter(c, p, f, e, tp)`
pub fn equip_filter(d: &mut Duel, c: CardId, p: u32, f: &Option<EquipFilter>, e: EffectId, tp: u8) -> bool {
    (p == PLAYER_ALL || card::is_controler(d, c, p as u8)) && card::is_faceup(d, c) && f.as_ref().is_none_or(|f| f(d, c, e, tp))
}
/// `aux.EquipTarget(tg, p, f)`
pub fn equip_target(tg: Option<EquipTg>, p: Option<u32>, f: Option<EquipFilter>) -> LuaFn {
    cost_target(move |d, a| {
        let (e, tp) = (a.e, a.tp);
        let player: Option<u32> = match p {
            Some(0) => Some(tp as u32),
            Some(1) => Some(1 - tp as u32),
            Some(PLAYER_ALL) | None => Some(PLAYER_ALL),
            _ => None,
        };
        if let Some(chkc) = a.chkc() {
            return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc) && player.is_some_and(|pl| equip_filter(d, chkc, pl, &f, e, tp));
        }
        if a.chk == Some(0) {
            return player.is_some_and(|pl| duel::is_existing_target(d, Some(&|d, c| equip_filter(d, c, pl, &f, e, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX));
        }
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
        let pl = player.nil();
        let g = duel::select_target(d, tp, Some(&|d, c| equip_filter(d, c, pl, &f, e, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        if let Some(tg) = &tg {
            let first = group::get_first(d, &g);
            tg(d, a, first);
        }
        let h = effect::get_handler(d, e).nil();
        duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(h)), 1, 0, 0);
        true
    })
}
/// `aux.EquipOperation(op)`
pub fn equip_operation(op: Option<LuaFn>) -> LuaFn {
    LuaFn::new(move |d, args| {
        let a = EffArgs::new(args);
        let tc = duel::first_target(d).nil();
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            let h = effect::get_handler(d, a.e).nil();
            duel::equip(d, a.tp, h, tc, true, false);
        }
        if let Some(op) = &op {
            op.call(d, &args[..args.len().min(8)]);
        }
        Value::Nil
    })
}
