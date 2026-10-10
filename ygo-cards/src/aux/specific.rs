//! `cards_specific_functions.lua`: helpers of single archetypes and mechanics (`aux.*` there;
//! the archetype namespaces `Witchcrafter`, `Cyberdark`, ... are submodules).

use crate::constants::*;
use crate::lua::{card, duel, effect, group, NO_EX};
use crate::prelude::*;

/// `aux.qlifilter(e, te)`: immunity of "Qli" monsters (activated monster effects of monsters
/// with a lower original Level/Rank)
pub fn qlifilter() -> LuaFn {
    lua_fn(|d, a| {
        let (e, te) = (a.e(0), a.e(1));
        if effect::is_active_type(d, te, TYPE_MONSTER) && effect::is_activated(d, te) {
            let lv = card::get_level(d, effect::get_handler(d, e).nil());
            let ec = effect::get_owner(d, te).nil();
            if card::is_type(d, ec, TYPE_LINK) {
                false
            } else if card::is_type(d, ec, TYPE_XYZ) {
                card::get_original_rank(d, ec) < lv
            } else {
                card::get_original_level(d, ec) < lv
            }
        } else {
            false
        }
    })
}
/// `aux.nvfilter(c)`
pub fn nvfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_has_effect(d, c, EFFECT_NECRO_VALLEY, None).is_empty()
}
/// `aux.NecroValleyFilter(f)`: `f`, except cards "Necro Valley" protects
pub fn necro_valley_filter(f: impl Fn(&mut Duel, CardId) -> bool) -> impl Fn(&mut Duel, CardId) -> bool {
    move |d, c| f(d, c) && !(!card::is_has_effect(d, c, EFFECT_NECRO_VALLEY, None).is_empty() && duel::is_chain_disablable(d, 0))
}
fn summon_type_between(lo: u32, hi: u32) -> LuaFn {
    condition(move |d, a| {
        let st = card::get_summon_type(d, effect::get_handler(d, a.e).nil());
        st >= SUMMON_TYPE_SPECIAL + lo && st < SUMMON_TYPE_SPECIAL + hi
    })
}
/// `aux.gbspcon`: Special Summoned by a "Gladiator Beast" effect
pub fn gbspcon() -> LuaFn {
    summon_type_between(100, 150)
}
/// `aux.evospcon`: Special Summoned by an "Evolsaur" effect
pub fn evospcon() -> LuaFn {
    summon_type_between(150, 180)
}
/// `aux.SpElimFilter(c, mustbefaceup, includemzone)`: "Spirit Elimination" (banish from the
/// field instead of the GY)
pub fn sp_elim_filter(d: &mut Duel, c: CardId, mustbefaceup: bool, includemzone: bool) -> bool {
    if card::is_monster(d, c) {
        if mustbefaceup && card::is_location(d, c, LOCATION_MZONE) && card::is_facedown(d, c) {
            return false;
        }
        let p = card::get_controler(d, c);
        if includemzone {
            return card::is_location(d, c, LOCATION_MZONE) || duel::is_player_affected_by_effect(d, p, 69832741).is_none();
        }
        if duel::is_player_affected_by_effect(d, p, 69832741).is_some() {
            card::is_location(d, c, LOCATION_MZONE)
        } else {
            card::is_location(d, c, LOCATION_GRAVE)
        }
    } else {
        includemzone || card::is_location(d, c, LOCATION_GRAVE)
    }
}
fn set_reset_opt(d: &mut Duel, e: EffectId, resetflag: Option<u32>, resetcount: Option<u16>) {
    match (resetflag, resetcount) {
        (Some(f), Some(n)) => effect::set_reset(d, e, f, n),
        (Some(f), None) => effect::set_reset(d, e, f, 1),
        _ => {}
    }
}
/// `aux.AddEREquipLimit(c, con, equipval, equipop, linkedeff, prop, resetflag, resetcount)`:
/// "Eyes Restrict"-like equipping (`equipval(ec, c, tp)`, `equipop(c, e, tp, tc)`)
#[allow(clippy::too_many_arguments)]
pub fn add_er_equip_limit(d: &mut Duel, c: CardId, con: Option<LuaFn>, equipval: LuaFn, equipop: LuaFn, linkedeff: EffectId, prop: Option<u32>, resetflag: Option<u32>, resetcount: Option<u16>) {
    let finalprop = EFFECT_FLAG_CANNOT_DISABLE | prop.unwrap_or(0);
    let e1 = effect::create_effect(d, c);
    if con.is_some() {
        effect::set_condition(d, e1, con);
    }
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, finalprop, EFFECT_FLAG2_MAJESTIC_MUST_COPY);
    effect::set_code(d, e1, 89785779);
    effect::set_label_object(d, e1, Value::Effect(linkedeff));
    set_reset_opt(d, e1, resetflag, resetcount);
    effect::set_value_fn(d, e1, LuaFn::new(move |d, a| equipval.call(d, &a[..a.len().min(3)])));
    effect::set_operation(d, e1, Some(LuaFn::new(move |d, a| {
        equipop.call(d, &a[..a.len().min(4)]);
        Value::Nil
    })));
    card::register_effect(d, c, e1, false);
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, finalprop & !EFFECT_FLAG_CANNOT_DISABLE, EFFECT_FLAG2_MAJESTIC_MUST_COPY);
    effect::set_code(d, e2, 89785779 + EFFECT_EQUIP_LIMIT);
    set_reset_opt(d, e2, resetflag, resetcount);
    card::register_effect(d, c, e2, false);
    effect::set_label_object(d, linkedeff, Value::Effect(e2));
}

/// `aux.EnableNeosReturn(c, extracat, extrainfo, extraop, returneff)`: "Neos" Fusion monsters
/// return to the Extra Deck in the End Phase
pub fn enable_neos_return(d: &mut Duel, c: CardId, extracat: Option<u64>, extrainfo: Option<LuaFn>, extraop: Option<LuaFn>, returneff: Option<EffectId>) {
    let extracat = extracat.unwrap_or(0);
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TODECK | extracat);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(|d, a| !neos_has_return_substitute(d, a.e))));
    effect::set_target(d, e1, Some(neos_return_target(extrainfo)));
    effect::set_operation(d, e1, Some(neos_return_operation(extraop)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, 0, 0);
    effect::set_condition(d, e2, Some(condition(|d, a| neos_has_return_substitute(d, a.e))));
    card::register_effect(d, c, e2, false);
    if let Some(re) = returneff {
        effect::set_label_object(d, e1, Value::Effect(re));
        effect::set_label_object(d, e2, Value::Effect(re));
    }
}
fn neos_has_return_substitute(d: &mut Duel, e: EffectId) -> bool {
    let h = effect::get_handler(d, e).nil();
    !card::is_has_effect(d, h, 42015635, None).is_empty()
}
fn neos_return_target(extrainfo: Option<LuaFn>) -> LuaFn {
    LuaFn::new(move |d, args| {
        let a = EffArgs::with_chk(args);
        if a.chk == Some(0) {
            return Value::Bool(true);
        }
        let h = effect::get_handler(d, a.e).nil();
        duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(h)), 1, 0, 0);
        if let Some(f) = &extrainfo {
            f.call(d, &args[..args.len().min(9)]);
        }
        Value::Nil
    })
}
/// `aux.NeosReturnSubstituteFilter(c)`
pub fn neos_return_substitute_filter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[14088859]) && card::is_able_to_remove_as_cost(d, c, None)
}
fn neos_return_operation(extraop: Option<LuaFn>) -> LuaFn {
    LuaFn::new(move |d, args| {
        let a = EffArgs::new(args);
        let tp = a.tp;
        let c = effect::get_handler(d, a.e).nil();
        if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
            return Value::Nil;
        }
        let sc = duel::get_first_matching_card(d, Some(&super::necro_valley_filter(neos_return_substitute_filter)), tp, LOCATION_GRAVE, 0, NO_EX);
        if sc.is_some() && duel::select_yes_no(d, tp, super::stringid(14088859, 0)) {
            duel::remove(d, sc.unwrap(), POS_FACEUP as u8, REASON_COST, None, None);
        } else {
            duel::send_to_deck(d, c, None, 2, REASON_EFFECT, None);
        }
        if card::is_location(d, c, LOCATION_EXTRA) {
            if let Some(f) = &extraop {
                f.call(d, &args[..args.len().min(8)]);
            }
        }
        Value::Nil
    })
}

/// `aux.*` state of this file (`Duel::script_var` keys).
mod vars {
    use crate::ns::AUX;
    /// `ReincarnationChecked`
    pub const REINCARNATION_CHECKED: (u32, u64) = (AUX, 100);
}
/// `aux.EnableCheckReincarnation(c)`: (once per duel) record "Salamangreat" Reincarnation
/// Summons
pub fn enable_check_reincarnation(d: &mut Duel, _c: CardId) {
    if !d.script_var(vars::REINCARNATION_CHECKED).truthy() {
        d.set_script_var(vars::REINCARNATION_CHECKED, Value::Bool(true));
        let e1 = effect::global_effect(d);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_MATERIAL_CHECK);
        effect::set_value_fn(d, e1, effect_card_value(|d, e, c| {
            reincarnation_check_value(d, e, c);
            0
        }));
        let ge1 = effect::global_effect(d);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_GRANT);
        effect::set_label_object(d, ge1, Value::Effect(e1));
        effect::set_target_range(d, ge1, 0xff, 0xff);
        effect::set_target(d, ge1, Some(effect_card_fn(|d, _e, c| card::is_type(d, c, TYPE_FUSION + TYPE_RITUAL + TYPE_SYNCHRO + TYPE_XYZ + TYPE_LINK))));
        duel::register_effect(d, ge1, 0);
    }
}
/// `aux.ReincarnationRitualFilter(c, rc, id, tp)`
pub fn reincarnation_ritual_filter(d: &mut Duel, c: CardId, rc: CardId, id: u32, tp: u8) -> bool {
    card::is_summon_code(d, c, Some(rc), SUMMON_TYPE_RITUAL as u64, tp, &[id]) && card::is_controler(d, c, tp) && card::is_location(d, c, LOCATION_MZONE)
}
/// `aux.ReincarnationCheckValue(e, c)`
fn reincarnation_check_value(d: &mut Duel, _e: EffectId, c: CardId) {
    let g = card::get_material(d, c);
    let id = card::get_code(d, c)[0];
    let tp = card::get_controler(d, c);
    let mut label = 3i64;
    let by = |d: &mut Duel, st: u32| group::is_exists(d, &g, &|d, m| card::is_summon_code(d, m, Some(c), st as u64, tp, &[id]), 1, NO_EX);
    let rc = if card::is_link_monster(d, c) {
        by(d, SUMMON_TYPE_LINK)
    } else if card::is_type(d, c, TYPE_FUSION) {
        by(d, SUMMON_TYPE_FUSION)
    } else if card::is_type(d, c, TYPE_RITUAL) {
        label = 0;
        if group::is_exists(d, &g, &|d, m| reincarnation_ritual_filter(d, m, c, id, 0), 1, NO_EX) {
            label = 1;
        }
        if group::is_exists(d, &g, &|d, m| reincarnation_ritual_filter(d, m, c, id, 1), 1, NO_EX) {
            label += 2;
        }
        label != 0
    } else if card::is_type(d, c, TYPE_SYNCHRO) {
        by(d, SUMMON_TYPE_SYNCHRO)
    } else if card::is_type(d, c, TYPE_XYZ) {
        by(d, SUMMON_TYPE_XYZ)
    } else {
        false
    };
    if rc {
        card::register_flag_effect(d, c, CARD_SALAMANGREAT_SANCTUARY, RESET_EVENT + RESETS_STANDARD - RESET_TOFIELD, 0, 1, label, 0);
    }
}

/// `aux.MaleficUniqueFilter(cc)`: the unique-on-field filter of "Malefic" monsters (the
/// `has_malefic_unique` table it also fills is never read)
pub fn malefic_unique_filter() -> LuaFn {
    lua_fn(|d, a| {
        let c = a.c(0);
        let p = card::get_controler(d, c);
        duel::is_player_affected_by_effect(d, p, 75223115).is_none() && card::is_set_card(d, c, &[SET_MALEFIC as u16])
    })
}
/// `aux.AddMaleficSummonProcedure(c, code, loc, excon)`: Special Summon from the hand by
/// banishing `code` (a "Malefic" card without one) from `loc`
pub fn add_malefic_summon_procedure(d: &mut Duel, c: CardId, code: Option<u32>, loc: u32, excon: Option<LuaFn>) {
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(malefic_summon_condition(code, loc, excon)));
    effect::set_target(d, e1, Some(malefic_summon_target(code, loc)));
    effect::set_operation(d, e1, Some(malefic_summon_operation()));
    card::register_effect(d, c, e1, false);
}
/// `aux.MaleficSummonFilter(c, cd)`
pub fn malefic_summon_filter(d: &mut Duel, c: CardId, cd: Option<u32>) -> bool {
    (match cd {
        Some(cd) => card::is_code(d, c, &[cd]),
        None => card::is_set_card(d, c, &[SET_MALEFIC as u16]),
    }) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `aux.MaleficSummonSubstitute(c, cd, tp)`
pub fn malefic_summon_substitute(d: &mut Duel, c: CardId, tp: u8) -> bool {
    !card::is_has_effect(d, c, 48829461, Some(tp)).is_empty() && card::is_able_to_remove_as_cost(d, c, None)
}
fn malefic_summon_condition(cd: Option<u32>, loc: u32, excon: Option<LuaFn>) -> LuaFn {
    lua_fn(move |d, a| {
        let e = a.e(0);
        let c = a.card(1);
        if let Some(f) = &excon {
            if !f.call(d, &[Value::Effect(e), Value::card(c)]).truthy() {
                return false;
            }
        }
        let Some(c) = c else { return true };
        let p = card::get_controler(d, c);
        // (the substitute filter gets `cd` as its `tp`: its argument list is (c, cd, tp))
        duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0
            && (duel::is_existing_matching_card(d, Some(&|d, x| malefic_summon_filter(d, x, cd)), p, loc, 0, 1, NO_EX)
                || duel::is_existing_matching_card(d, Some(&|d, x| malefic_summon_substitute(d, x, cd.map_or(0, |v| v as u8))), p, LOCATION_ONFIELD + LOCATION_GRAVE, 0, 1, NO_EX))
    })
}
fn malefic_summon_target(cd: Option<u32>, loc: u32) -> LuaFn {
    lua_fn(move |d, a| {
        let (e, tp, c) = (a.e(0), a.p(1), a.c(9));
        let mut g = duel::get_matching_group(d, Some(&|d, x| malefic_summon_filter(d, x, cd)), tp, loc, 0, NO_EX);
        let cp = card::get_controler(d, c);
        // (the substitute filter gets the controler as `cd` and no `tp`)
        let g2 = duel::get_matching_group(d, Some(&|d, x| malefic_summon_substitute_no_tp(d, x, cp)), tp, LOCATION_ONFIELD + LOCATION_GRAVE, 0, NO_EX);
        group::add_card(&mut g, &g2);
        let rescon = super::chkf_mmz(1);
        let sg = super::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(1), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true).unwrap_or_default();
        if !sg.is_empty() {
            effect::set_label_object(d, e, Value::Group(sg));
            return true;
        }
        false
    })
}
/// `MaleficSummonSubstitute(c, cd)` called without `tp`: `IsHasEffect(48829461, nil)`
fn malefic_summon_substitute_no_tp(d: &mut Duel, c: CardId, _cd: u8) -> bool {
    !card::is_has_effect(d, c, 48829461, None).is_empty() && card::is_able_to_remove_as_cost(d, c, None)
}
fn malefic_summon_operation() -> LuaFn {
    lua_fn(|d, a| {
        let (e, tp) = (a.e(0), a.p(1));
        let Value::Group(g) = effect::get_label_object(d, e) else { return };
        let tc = group::get_first(d, &g).nil();
        if let Some(&te) = card::is_has_effect(d, tc, 48829461, Some(tp)).first() {
            effect::use_count_limit(d, te, tp, 1, false);
        }
        duel::remove(d, tc, POS_FACEUP as u8, REASON_COST, None, None);
    })
}

/// `Witchcrafter.*`
pub mod witchcrafter {
    use super::*;
    use std::sync::{Arc, LazyLock};
    static DISCARD_COST: LazyLock<LuaFn> = LazyLock::new(|| crate::cost::replaceable(crate::cost::discard(Some(Arc::new(|d, c, _e, _tp| card::is_spell(d, c))), false, None, None, None), None));
    static TRIBUTE_AND_DISCARD_COST: LazyLock<LuaFn> = LazyLock::new(|| crate::cost::and(vec![crate::cost::self_tribute(), discard_cost()]));
    /// `Witchcrafter.DiscardCost`
    pub fn discard_cost() -> LuaFn {
        DISCARD_COST.clone()
    }
    /// `Witchcrafter.TributeAndDiscardCost`
    pub fn tribute_and_discard_cost() -> LuaFn {
        TRIBUTE_AND_DISCARD_COST.clone()
    }
    /// `Witchcrafter.CreateCostReplaceEffect(c)`
    pub fn create_cost_replace_effect(d: &mut Duel, c: CardId) -> EffectId {
        let e = effect::create_effect(d, c);
        effect::set_type(d, e, EFFECT_TYPE_FIELD);
        effect::set_property(d, e, EFFECT_FLAG_PLAYER_TARGET, 0);
        effect::set_code(d, e, EFFECT_COST_REPLACE);
        effect::set_target_range(d, e, 1, 0);
        // function(base, extracon, e, tp, eg, ep, ev, re, r, rp, chk)
        effect::set_value_fn(d, e, lua_fn(|d, a| {
            let (e, tp, chk) = (a.e(2), a.p(3), a.int(10));
            if chk == 0 {
                let c = effect::get_handler(d, e).nil();
                return card::is_set_card(d, c, &[SET_WITCHCRAFTER as u16]) && card::is_controler(d, c, tp) && card::is_location(d, c, LOCATION_MZONE);
            }
            crate::chain::is_triggering(d, 0, crate::chain::Prop::SetCard, &[SET_WITCHCRAFTER as i64]).truthy()
                && crate::chain::is_triggering(d, 0, crate::chain::Prop::Controler, &[tp as i64]).truthy() && is_triggering_player(d, tp)
                && crate::chain::is_triggering(d, 0, crate::chain::Prop::Location, &[LOCATION_MZONE as i64]).truthy()
        }));
        e
    }
    fn is_triggering_player(d: &Duel, tp: u8) -> bool {
        duel::get_chain_info(d, 0, &[CHAININFO_TRIGGERING_PLAYER]).first().is_some_and(|v| v.int() == tp as i64)
    }
}

/// `aux.EvilHeroLimit(e, se, sp, st)`: "Evil HERO" Fusion monsters' Special Summon condition
pub fn evil_hero_limit() -> LuaFn {
    lua_fn(|d, a| {
        let (e, se, st) = (a.e(0), a.e(1), a.int(3) as u32);
        let hp = effect::get_handler_player(d, e);
        let sh = effect::get_handler(d, se).nil();
        card::is_code(d, sh, &[CARD_DARK_FUSION])
            || (duel::is_player_affected_by_effect(d, hp, SKILL_DARK_UNITY).is_some() && card::is_code(d, sh, &[CARD_SUPER_POLYMERIZATION]))
            || (duel::is_player_affected_by_effect(d, hp, EFFECT_SUPREME_CASTLE).is_some() && st & SUMMON_TYPE_FUSION == SUMMON_TYPE_FUSION)
    })
}
/// `c:AddMustBeSpecialSummonedByDarkFusion()`. (The card's `INFO` must also declare
/// `dark_calling: true`, which the Lua sets here.)
pub fn add_must_be_special_summoned_by_dark_fusion(d: &mut Duel, c: CardId) -> EffectId {
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    effect::set_value_fn(d, e0, evil_hero_limit());
    card::register_effect(d, c, e0, false);
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, CARD_CLOCK_LIZARD);
    effect::set_condition(d, e1, Some(lua_fn(|d, a| {
        let hp = effect::get_handler_player(d, a.e(0));
        duel::is_player_affected_by_effect(d, hp, EFFECT_SUPREME_CASTLE).is_none()
    })));
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
    e0
}
/// `aux.FossilLimit(e, se, sp, st)`
pub fn fossil_limit() -> LuaFn {
    lua_fn(|d, a| {
        let (e, se) = (a.e(0), a.e(1));
        let h = effect::get_handler(d, e).nil();
        let sh = effect::get_handler(d, se).nil();
        !card::is_location(d, h, LOCATION_EXTRA) || card::is_code(d, sh, &[CARD_FOSSIL_FUSION])
    })
}

/// `aux.AddLavaProcedure(c, required, position, filter, value, description)`: "Lava Golem"-like
/// Special Summon to the opponent's field by Tributing their monsters
pub fn add_lava_procedure(d: &mut Duel, c: CardId, required: Option<usize>, position: u32, filter: Option<CardFilter>, value: Option<i64>, description: Option<u64>) -> EffectId {
    let required = required.filter(|&r| r >= 1).unwrap_or(1);
    let filter: CardFilter = filter.unwrap_or_else(|| std::sync::Arc::new(card::is_monster));
    let value = value.unwrap_or(0);
    let e1 = effect::create_effect(d, c);
    if let Some(desc) = description {
        effect::set_description(d, e1, desc);
    }
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_SPSUM_PARAM, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_target_range(d, e1, position, 1);
    effect::set_value(d, e1, value);
    effect::set_condition(d, e1, Some(lava_condition(required, filter.clone())));
    effect::set_target(d, e1, Some(lava_target(required, filter)));
    effect::set_operation(d, e1, Some(lava_operation()));
    card::register_effect(d, c, e1, false);
    e1
}
/// `aux.LavaCheck(sg, e, tp, mg)`
pub fn lava_check(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    (duel::get_m_zone_count(d, 1 - tp, sg.into(), Some(tp), None, None).0 > 0, false)
}
fn lava_group(d: &mut Duel, tp: u8, filter: &CardFilter) -> CardSet {
    duel::get_matching_group(d, Some(&|d, c| card::is_releasable(d, c, None, None) && filter(d, c)), tp, 0, LOCATION_MZONE, NO_EX)
}
fn lava_condition(required: usize, filter: CardFilter) -> LuaFn {
    lua_fn(move |d, a| {
        let e = a.e(0);
        let Some(c) = a.card(1) else { return true };
        let tp = card::get_controler(d, c);
        let mg = lava_group(d, tp, &filter);
        matches!(super::select_unselect_group(d, &mg, Some(e), tp, Some(required), Some(required), Some(&lava_check), 0, tp, None, None, None, false), Err(true))
    })
}
fn lava_target(required: usize, filter: CardFilter) -> LuaFn {
    lua_fn(move |d, a| {
        let (e, tp) = (a.e(0), a.p(1));
        let mg = lava_group(d, tp, &filter);
        let g = super::select_unselect_group(d, &mg, Some(e), tp, Some(required), Some(required), Some(&lava_check), 1, tp, Some(HINTMSG_RELEASE as u64), None, None, true).unwrap_or_default();
        if !g.is_empty() {
            effect::set_label_object(d, e, Value::Group(g));
            true
        } else {
            false
        }
    })
}
fn lava_operation() -> LuaFn {
    lua_fn(|d, a| {
        let g = effect::get_label_object(d, a.e(0));
        duel::release(d, g.as_group().cloned().unwrap_or_default(), REASON_COST, None);
    })
}
/// `aux.AddKaijuProcedure(c)`: the Special Summon procedures of "Kaiju" monsters
pub fn add_kaiju_procedure(d: &mut Duel, c: CardId) -> (EffectId, EffectId) {
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Function(lua_fn(|d, a| card::is_set_card(d, a.c(0), &[SET_KAIJU as u16]))), Some(LOCATION_MZONE));
    let e1 = add_lava_procedure(d, c, Some(1), POS_FACEUP_ATTACK, None, None, None);
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_SPSUM_PARAM, 0);
    effect::set_target_range(d, e2, POS_FACEUP_ATTACK, 0);
    effect::set_condition(d, e2, Some(kaiju_condition()));
    card::register_effect(d, c, e2, false);
    (e1, e2)
}
/// `aux.KaijuCondition(e, c)`
pub fn kaiju_condition() -> LuaFn {
    lua_fn(|d, a| {
        let Some(c) = a.card(1) else { return true };
        let tp = card::get_controler(d, c);
        duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&super::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_KAIJU as u16]))), tp, 0, LOCATION_MZONE, 1, NO_EX)
    })
}
/// `aux.CheckStealEquip(c, e, tp)`: `c` can be taken by equipping `e`'s card to it
pub fn check_steal_equip(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    if card::is_facedown(d, c) || !card::is_controler_can_be_changed(d, c, false, None) || !card::is_controler(d, c, 1 - tp) {
        return false;
    }
    if card::is_location(d, effect::get_handler(d, e).nil(), LOCATION_SZONE) {
        return true;
    }
    if !duel::is_duel_type(d, DUEL_TRAP_MONSTERS_NOT_USE_ZONE as u64) && card::is_type(d, c, TYPE_TRAPMONSTER) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, Some(tp), Some(LOCATION_REASON_CONTROL), None).0 > 0
            && duel::get_location_count(d, tp, LOCATION_SZONE as u8, Some(tp), Some(0), None).0 >= 2;
    }
    true
}
/// `aux.CheckSummonGate(tp)`: the least number of monsters "Summon Gate"-like effects allow from
/// the Extra Deck (`None` without such effects)
pub fn check_summon_gate_limit(d: &mut Duel, tp: u8) -> Option<i64> {
    let mut tot: Option<i64> = None;
    for eff in duel::get_player_effect(d, tp, CARD_SUMMON_GATE) {
        let val = match effect::get_value(d, eff) {
            Value::Fn(f) => f.call(d, &[Value::Int(tp as i64)]),
            v => v,
        };
        if val.truthy() {
            let v = val.int();
            tot = Some(tot.map_or(v, |t| t.min(v)));
        }
    }
    tot
}
/// `aux.CheckSummonGate(tp, count)`: `count` more monsters can be summoned from the Extra Deck
pub fn check_summon_gate(d: &mut Duel, tp: u8, count: i64) -> bool {
    check_summon_gate_limit(d, tp).is_none_or(|t| t >= count)
}

/// `aux.addLizardCheck(c)`: "Clock Lizard" check (the card counts as summoned from the Extra Deck)
pub fn add_lizard_check(d: &mut Duel, c: CardId) -> EffectId {
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, CARD_CLOCK_LIZARD);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
    e1
}
/// `aux.createTempLizardCheck(c, filter, reset, tRange, tRange2, resetcount)` (`filter`: a
/// target function `(e, c)`)
pub fn create_temp_lizard_check(d: &mut Duel, c: CardId, filter: Option<LuaFn>, reset: Option<u32>, t_range: Option<u32>, t_range2: Option<u32>, resetcount: Option<u16>) -> EffectId {
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, CARD_CLOCK_LIZARD);
    effect::set_target_range(d, e1, t_range.unwrap_or(0xff), t_range2.unwrap_or(0));
    effect::set_reset(d, e1, reset.unwrap_or(RESET_PHASE | PHASE_END), resetcount.unwrap_or(1));
    effect::set_target(d, e1, Some(filter.unwrap_or_else(aux_true)));
    effect::set_value(d, e1, 1);
    e1
}
/// `aux.addTempLizardCheck(c, tp, filter, reset, tRange, tRange2, resetcount)`
#[allow(clippy::too_many_arguments)]
pub fn add_temp_lizard_check(d: &mut Duel, c: CardId, tp: u8, filter: Option<LuaFn>, reset: Option<u32>, t_range: Option<u32>, t_range2: Option<u32>, resetcount: Option<u16>) -> EffectId {
    let e1 = create_temp_lizard_check(d, c, filter, reset, t_range, t_range2, resetcount);
    duel::register_effect(d, e1, tp);
    e1
}
/// `aux.createContinuousLizardCheck(c, location, filter, tRange, tRange2)`
pub fn create_continuous_lizard_check(d: &mut Duel, c: CardId, location: u32, filter: Option<LuaFn>, t_range: Option<u32>, t_range2: Option<u32>) -> EffectId {
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, CARD_CLOCK_LIZARD);
    effect::set_target_range(d, e1, t_range.unwrap_or(0xff), t_range2.unwrap_or(0));
    effect::set_range(d, e1, location);
    effect::set_target(d, e1, Some(filter.unwrap_or_else(aux_true)));
    effect::set_value(d, e1, 1);
    e1
}
/// `aux.addContinuousLizardCheck(c, location, filter, tRange, tRange2)`
pub fn add_continuous_lizard_check(d: &mut Duel, c: CardId, location: u32, filter: Option<LuaFn>, t_range: Option<u32>, t_range2: Option<u32>) -> EffectId {
    let e1 = create_continuous_lizard_check(d, c, location, filter, t_range, t_range2);
    card::register_effect(d, c, e1, false);
    e1
}
