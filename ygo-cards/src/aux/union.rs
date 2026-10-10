//! `proc_union.lua`: Union monsters (`aux.AddUnionProcedure`).

use crate::constants::*;
use crate::lua::{card, duel, effect};
use crate::prelude::*;

/// `aux.AddUnionProcedure(c, f, oldequip, oldprotect)`: equip to a monster matching `f`, and
/// unequip (`oldequip`/`oldprotect`: the old rules)
pub fn add_union_procedure(d: &mut Duel, c: CardId, f: Option<CardFilter>, oldequip: bool, oldprotect: Option<bool>) {
    let oldprotect = oldprotect.unwrap_or(oldequip);
    // equip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, 1068);
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target(d, e1, Some(union_target(f.clone(), oldequip)));
    effect::set_operation(d, e1, Some(union_operation(f.clone())));
    card::register_effect(d, c, e1, false);
    // unequip
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, 2);
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_condition(d, e2, Some(if oldequip { is_union_state() } else { has_equip_target() }));
    effect::set_target(d, e2, Some(union_sum_target(oldequip)));
    effect::set_operation(d, e2, Some(union_sum_operation(oldequip)));
    card::register_effect(d, c, e2, false);
    // destroy sub
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e3, EFFECT_DESTROY_SUBSTITUTE);
    effect::set_condition(d, e3, Some(if oldprotect { is_union_state() } else { has_equip_target() }));
    effect::set_value_fn(d, e3, union_replace(oldprotect));
    card::register_effect(d, c, e3, false);
    // eqlimit
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e4, EFFECT_UNION_LIMIT);
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_value_fn(d, e4, union_limit(f));
    card::register_effect(d, c, e4, false);
    // auxiliary function compatibility
    if oldequip {
        crate::class::set(d, c, crate::class::OLD_UNION, Value::Bool(true));
    }
}
fn has_equip_target() -> LuaFn {
    lua_fn(|d, a| {
        let h = effect::get_handler(d, a.e(0)).nil();
        Value::card(card::get_equip_target(d, h))
    })
}
/// `aux.UnionFilter(c, f, oldrule)`
pub fn union_filter(d: &mut Duel, c: CardId, f: &Option<CardFilter>, oldrule: bool) -> bool {
    let (ct1, ct2) = card::get_union_count(d, c);
    if card::is_faceup(d, c) && f.as_ref().is_none_or(|f| f(d, c)) {
        if oldrule { ct1 == 0 } else { ct2 == 0 }
    } else {
        false
    }
}
fn union_flag_reset() -> u32 {
    RESET_EVENT + (RESETS_STANDARD - RESET_TOFIELD - RESET_LEAVE) + RESET_PHASE + PHASE_END
}
/// `aux.UnionTarget(f, oldrule)`
pub fn union_target(f: Option<CardFilter>, oldrule: bool) -> LuaFn {
    cost_target(move |d, a| {
        let tp = a.tp;
        let c = effect::get_handler(d, a.e).nil();
        let code = card::get_original_code(d, c);
        if let Some(chkc) = a.chkc() {
            // (the Lua checks the Union monster itself here, not chkc)
            return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && union_filter(d, c, &f, oldrule);
        }
        if a.chk == Some(0) {
            return card::get_flag_effect(d, c, code) == 0
                && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
                && duel::is_existing_target(d, Some(&|d, x| union_filter(d, x, &f, oldrule)), tp, LOCATION_MZONE, 0, 1, c.into());
        }
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
        let g = duel::select_target(d, tp, Some(&|d, x| union_filter(d, x, &f, oldrule)), tp, LOCATION_MZONE, 0, 1, 1, false, c.into());
        duel::set_operation_info(d, 0, CATEGORY_EQUIP, g, 1, 0, 0);
        card::register_flag_effect(d, c, code, union_flag_reset(), 0, 1, 0, 0);
        true
    })
}
/// `aux.UnionOperation(f)`
pub fn union_operation(f: Option<CardFilter>) -> LuaFn {
    operation(move |d, a| {
        let c = effect::get_handler(d, a.e).nil();
        let tc = duel::first_target(d);
        if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
            return;
        }
        let tc = tc.nil();
        if !card::is_relate_to_effect(d, tc, a.e) || f.as_ref().is_some_and(|f| !f(d, tc)) {
            duel::send_to_grave(d, c, REASON_EFFECT, None, None);
            return;
        }
        if !duel::equip(d, a.tp, c, tc, false, false) {
            return;
        }
        set_union_state(d, c);
    })
}
/// `aux.UnionSumTarget(oldrule)`
pub fn union_sum_target(oldrule: bool) -> LuaFn {
    cost_target(move |d, a| {
        let tp = a.tp;
        let c = effect::get_handler(d, a.e).nil();
        let code = card::get_original_code(d, c);
        let pos = if oldrule { POS_FACEUP_ATTACK } else { POS_FACEUP } as u8;
        if a.chk == Some(0) {
            return card::get_flag_effect(d, c, code) == 0
                && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && card::is_can_be_special_summoned(d, c, a.e, 0, tp, true, false, Some(pos), None, None);
        }
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
        card::register_flag_effect(d, c, code, union_flag_reset(), 0, 1, 0, 0);
        true
    })
}
/// `aux.UnionSumOperation(oldrule)`
pub fn union_sum_operation(oldrule: bool) -> LuaFn {
    operation(move |d, a| {
        let tp = a.tp;
        let c = effect::get_handler(d, a.e).nil();
        if !card::is_relate_to_effect(d, c, a.e) {
            return;
        }
        let pos = if oldrule { POS_FACEUP_ATTACK } else { POS_FACEUP } as u8;
        if duel::special_summon(d, c, 0, tp, tp, true, false, pos, None) == 0
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, true, false, Some(pos), None, None)
        {
            duel::send_to_grave(d, c, REASON_RULE, None, None);
        }
    })
}
/// `aux.UnionReplace(oldrule)`: `function(e, re, r, rp)`
pub fn union_replace(oldrule: bool) -> LuaFn {
    lua_fn(move |_d, a| {
        let r = a.int(2) as u32;
        if oldrule { r & REASON_BATTLE != 0 } else { r & REASON_BATTLE != 0 || r & REASON_EFFECT != 0 }
    })
}
/// `aux.UnionLimit(f)`
pub fn union_limit(f: Option<CardFilter>) -> LuaFn {
    effect_card_fn(move |d, e, c| {
        f.as_ref().is_none_or(|f| f(d, c)) || {
            let h = effect::get_handler(d, e).nil();
            card::get_equip_target(d, h) == Some(c)
        }
    })
}
/// `aux.IsUnionState(effect)`
pub fn is_union_state() -> LuaFn {
    lua_fn(|d, a| {
        let h = effect::get_handler(d, a.e(0)).nil();
        !card::is_has_effect(d, h, EFFECT_UNION_STATUS, None).is_empty()
    })
}
/// `aux.SetUnionState(c)`
pub fn set_union_state(d: &mut Duel, c: CardId) {
    let eset = card::get_card_effect(d, c, EFFECT_UNION_LIMIT);
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e0, EFFECT_EQUIP_LIMIT);
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    match effect::get_value(d, *eset.first().nil()) {
        Value::Fn(f) => effect::set_value_fn(d, e0, f),
        v => effect::set_value(d, e0, v.int()),
    }
    effect::set_reset(d, e0, RESET_EVENT + RESETS_STANDARD, 1);
    card::register_effect(d, c, e0, false);
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UNION_STATUS);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
    card::register_effect(d, c, e1, false);
    if crate::class::get(d, c, crate::class::OLD_UNION).truthy() {
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_OLDUNION_STATUS);
        card::register_effect(d, c, e2, false);
    }
}
/// `aux.CheckUnionEquip(uc, tc)`
pub fn check_union_equip(d: &mut Duel, uc: CardId, tc: CardId) -> bool {
    let (ct1, ct2) = card::get_union_count(d, tc);
    if crate::class::get(d, uc, crate::class::OLD_UNION).truthy() { ct1 == 0 } else { ct2 == 0 }
}
