//! `proc_persistent.lua`: Continuous Traps that target a monster (`aux.AddPersistentProcedure`).

use crate::constants::*;
use crate::lua::{card, duel, effect, group, NO_EX};
use crate::prelude::*;
use std::sync::Arc;

/// `tg(e, tp, eg, ep, ev, re, r, rp, tc, chk)`: more of the target (`chk` 0: can `tc` be targeted)
pub type PersistentTg = Arc<dyn Fn(&mut Duel, &EffArgs, Option<CardId>, i64) -> bool + Send + Sync>;

/// `aux.AddPersistentProcedure(c, p, f, category, property, hint1, hint2, con, cost, tg, op, anypos)`
#[allow(clippy::too_many_arguments)]
pub fn add_persistent_procedure(
    d: &mut Duel,
    c: CardId,
    p: Option<u32>,
    f: Option<super::EquipFilter>,
    category: Option<u64>,
    property: Option<u32>,
    hint1: Option<u32>,
    hint2: Option<u32>,
    con: Option<LuaFn>,
    cost: Option<LuaFn>,
    tg: Option<PersistentTg>,
    op: Option<LuaFn>,
    anypos: bool,
) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, 1068);
    if let Some(cat) = category {
        effect::set_category(d, e1, cat);
    }
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    match (hint1, hint2) {
        (None, None) => {}
        (h1, h2) if h1 == h2 => effect::set_hint_timing(d, e1, h1.unwrap_or(0), None),
        (Some(h1), None) => effect::set_hint_timing(d, e1, h1, Some(0)),
        (None, Some(h2)) => effect::set_hint_timing(d, e1, 0, Some(h2)),
        (Some(h1), Some(h2)) => effect::set_hint_timing(d, e1, h1, Some(h2)),
    }
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + property.unwrap_or(0), 0);
    if con.is_some() {
        effect::set_condition(d, e1, con);
    }
    if cost.is_some() {
        effect::set_cost(d, e1, cost);
    }
    effect::set_target(d, e1, Some(persistent_target(tg, p, f)));
    effect::set_operation(d, e1, op);
    card::register_effect(d, c, e1, false);
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
    effect::set_label_object(d, e2, Value::Effect(e1));
    effect::set_condition(d, e2, Some(persistent_tg_con()));
    effect::set_operation(d, e2, Some(persistent_tg_op(anypos)));
    card::register_effect(d, c, e2, false);
}
/// `aux.PersistentFilter(c, p, f, e, tp, tg, eg, ep, ev, re, r, rp)`
fn persistent_filter(d: &mut Duel, c: CardId, p: u32, f: &Option<super::EquipFilter>, e: EffectId, tp: u8, tg: Option<(&PersistentTg, &EffArgs)>) -> bool {
    (p == PLAYER_ALL || card::is_controler(d, c, p as u8)) && f.as_ref().is_none_or(|f| f(d, c, e, tp)) && tg.is_none_or(|(tg, a)| tg(d, a, Some(c), 0))
}
/// `aux.PersistentTarget(tg, p, f)`
pub fn persistent_target(tg: Option<PersistentTg>, p: Option<u32>, f: Option<super::EquipFilter>) -> LuaFn {
    cost_target(move |d, a| {
        let (e, tp) = (a.e, a.tp);
        let player: Option<u32> = match p {
            Some(0) => Some(tp as u32),
            Some(1) => Some(1 - tp as u32),
            Some(PLAYER_ALL) | None => Some(PLAYER_ALL),
            _ => None,
        };
        if let Some(chkc) = a.chkc() {
            // (`c:IsControler(nil)` is an error)
            let pl = player.unwrap_or_else(|| crate::lua_error("Parameter 2 should be \"Int\" but is \"nil\"."));
            return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc) && persistent_filter(d, chkc, pl, &f, e, tp, None);
        }
        if a.chk == Some(0) {
            // (with no player, every filter call is an error: false)
            let Some(pl) = player else { return false };
            let tgp = tg.as_ref().map(|t| (t, a));
            return duel::is_existing_target(d, Some(&|d, c| persistent_filter(d, c, pl, &f, e, tp, tgp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
        }
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
        let pl = player.unwrap_or_else(|| crate::lua_error("Parameter 2 should be \"Int\" but is \"nil\"."));
        let g = duel::select_target(d, tp, Some(&|d, c| persistent_filter(d, c, pl, &f, e, tp, None)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        if let Some(tg) = &tg {
            let first = group::get_first(d, &g);
            tg(d, a, first, 1);
        }
        true
    })
}
/// `aux.PersistentTgCon`
pub fn persistent_tg_con() -> LuaFn {
    condition(|d, a| a.re.is_some() && effect::get_label_object(d, a.e).as_effect() == a.re)
}
/// `aux.PersistentTgOp(anypos)`: the Trap keeps targeting the monster
pub fn persistent_tg_op(anypos: bool) -> LuaFn {
    operation(move |d, a| {
        let c = effect::get_handler(d, a.e).nil();
        let tcs = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
        let Some(Value::Group(g)) = tcs.into_iter().next() else { crate::lua_error("attempt to index a nil value") };
        let tc = group::get_first(d, &g);
        let re = a.re.nil();
        if card::is_relate_to_effect(d, c, re) {
            if let Some(tc) = tc {
                if (anypos || card::is_faceup(d, tc)) && card::is_relate_to_effect(d, tc, re) {
                    card::set_card_target(d, c, tc);
                    card::create_relation(d, c, tc, RESET_EVENT + RESETS_STANDARD);
                }
            }
        }
    })
}
/// `aux.PersistentTargetFilter(e, c)`
pub fn persistent_target_filter() -> LuaFn {
    effect_card_fn(|d, e, c| {
        let h = effect::get_handler(d, e).nil();
        card::is_has_card_target(d, h, c)
    })
}
