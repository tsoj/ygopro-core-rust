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
        duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0
            && (duel::is_existing_matching_card(d, Some(&|d, x| malefic_summon_filter(d, x, cd)), p, loc, 0, 1, NO_EX)
                || duel::is_existing_matching_card(d, Some(&|d, x| malefic_summon_substitute(d, x, p)), p, LOCATION_ONFIELD + LOCATION_GRAVE, 0, 1, NO_EX))
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
pub fn lava_condition(required: usize, filter: CardFilter) -> LuaFn {
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

/// `aux.IceBarrierDiscardFilter(c, tp)`: "Mirror Master of the Ice Barrier" can replace the cost
pub fn ice_barrier_discard_filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    !card::is_has_effect(d, c, EFFECT_ICEBARRIER_REPLACE, Some(tp)).is_empty() && card::is_able_to_remove_as_cost(d, c, None)
}
/// `aux.IceBarrierDiscardCost(f, discard, minc, maxc)`: discard (or send from the hand to the GY)
/// `minc`..`maxc` cards matching `f`, or banish a replacing card from the GY
pub fn ice_barrier_discard_cost(f: Option<CardFilter>, discard: bool, minc: Option<usize>, maxc: Option<usize>) -> LuaFn {
    let minc = minc.unwrap_or(1);
    let maxc = maxc.unwrap_or(1);
    let fliter = move |d: &mut Duel, c: CardId| f.as_ref().is_none_or(|f| f(d, c)) && if discard { card::is_discardable(d, c, None, None) } else { card::is_able_to_grave_as_cost(d, c) };
    cost_target_value(move |d, a| {
        let (e, tp) = (a.e, a.tp);
        if a.chk == Some(0) {
            return Value::Bool(
                duel::is_existing_matching_card(d, Some(&fliter), tp, LOCATION_HAND, 0, minc as u32, NO_EX)
                    || duel::is_existing_matching_card(d, Some(&|d, c| ice_barrier_discard_filter(d, c, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX),
            );
        }
        let mut g = duel::get_matching_group(d, Some(&fliter), tp, LOCATION_HAND, 0, NO_EX);
        let g2 = duel::get_matching_group(d, Some(&|d, c| ice_barrier_discard_filter(d, c, tp)), tp, LOCATION_GRAVE, 0, NO_EX);
        group::add_card(&mut g, &g2);
        let rescon = |d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>| {
            (group::filter_count(d, sg, &|d, c| ice_barrier_discard_filter(d, c, tp), NO_EX) <= 1 && sg.len() >= minc, false)
        };
        let mut sg = super::select_unselect_group(d, &g, Some(e), tp, Some(minc), Some(maxc), Some(&rescon), 1, tp, Some(super::stringid(CARD_REVEALER_ICEBARRIER, 1)), None, None, false).unwrap_or_default();
        let mut rm = 0;
        if group::is_exists(d, &sg, &|d, c| !card::is_has_effect(d, c, EFFECT_ICEBARRIER_REPLACE, Some(tp)).is_empty(), 1, NO_EX) {
            let te = group::filter(d, &sg, &|d, c| !card::is_has_effect(d, c, EFFECT_ICEBARRIER_REPLACE, None).is_empty(), NO_EX);
            let first = group::get_first(d, &te).nil();
            let eff = *card::get_card_effect(d, first, EFFECT_ICEBARRIER_REPLACE).first().nil();
            effect::use_count_limit(d, eff, tp, 1, false);
            rm = duel::remove(d, &te, POS_FACEUP as u8, REASON_COST, None, None);
            group::remove_card(&mut sg, &te);
        }
        Value::Int(if !sg.is_empty() {
            let r = if discard { REASON_COST + REASON_DISCARD } else { REASON_COST };
            (duel::send_to_grave(d, &sg, r, None, None) + rm) as i64
        } else {
            rm as i64
        })
    })
}
/// A cost or target function that returns a value (`function(e,tp,eg,ep,ev,re,r,rp,chk)`).
fn cost_target_value(f: impl Fn(&mut Duel, &EffArgs) -> Value + Send + Sync + 'static) -> LuaFn {
    LuaFn::new(move |d, a| f(d, &EffArgs::with_chk(a)))
}

/// `aux.ZWEquipLimit(tc, te)`: equip limit "only to `tc`, while the equip effect `te` applies"
pub fn zw_equip_limit(tc: CardId, te: Value) -> LuaFn {
    effect_card_fn(move |d, e, c| {
        if c != tc {
            return false;
        }
        let h = effect::get_handler(d, e).nil();
        card::get_card_effect(d, h, 75402014 + EFFECT_EQUIP_LIMIT).into_iter().any(|eff| matches!(te, Value::Effect(x) if x == eff))
    })
}
/// `aux.EquipAndLimitRegister(c, e, tp, tc, code, previousPos)`: equip `c` to `tc` by its own
/// effect `e` (with `aux.ZWEquipLimit`)
pub fn equip_and_limit_register(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId, code: Option<u32>, previous_pos: Option<bool>) -> bool {
    if !duel::equip(d, tp, c, tc, previous_pos.unwrap_or(true), false) {
        return false;
    }
    if let Some(code) = code {
        card::register_flag_effect(d, tc, code, RESET_EVENT + RESETS_STANDARD, 0, 0, 0, 0);
    }
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
    effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
    let te = effect::get_label_object(d, e);
    effect::set_value_fn(d, e1, zw_equip_limit(tc, te));
    card::register_effect(d, c, e1, false);
    true
}
/// `aux.AddZWEquipLimit(c, con, equipval, equipop, linkedeff, prop, resetflag, resetcount)`:
/// equip limit for cards that equip themselves (`equipval(tc, c, tp)`, `equipop(c, e, tp, tc)`)
#[allow(clippy::too_many_arguments)]
pub fn add_zw_equip_limit(d: &mut Duel, c: CardId, con: Option<LuaFn>, equipval: LuaFn, equipop: LuaFn, linkedeff: EffectId, prop: Option<u32>, resetflag: Option<u32>, resetcount: Option<u16>) {
    let finalprop = prop.map_or(EFFECT_FLAG_CANNOT_DISABLE, |p| p | EFFECT_FLAG_CANNOT_DISABLE);
    let e1 = effect::create_effect(d, c);
    if con.is_some() {
        effect::set_condition(d, e1, con);
    }
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, finalprop, EFFECT_FLAG2_MAJESTIC_MUST_COPY);
    effect::set_code(d, e1, 75402014);
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
    effect::set_code(d, e2, 75402014 + EFFECT_EQUIP_LIMIT);
    set_reset_opt(d, e2, resetflag, resetcount);
    card::register_effect(d, c, e2, false);
    effect::set_label_object(d, linkedeff, Value::Effect(e2));
}

/// `AA.*`: "Amazement" and "Ɐttraction" helpers
pub mod aa {
    use super::*;
    /// `AA.eqtgfilter(c, tp)`
    pub fn eqtgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
        card::is_faceup(d, c) && (card::is_set_card(d, c, &[SET_AMAZEMENT as u16]) || !card::is_controler(d, c, tp))
    }
    /// `AA.eqtg`
    pub fn eqtg() -> LuaFn {
        cost_target(|d, a| {
            let tp = a.tp;
            if let Some(chkc) = a.chkc() {
                return card::is_location(d, chkc, LOCATION_MZONE) && eqtgfilter(d, chkc, tp);
            }
            if a.chk == Some(0) {
                return effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) && duel::is_existing_target(d, Some(&|d, c| eqtgfilter(d, c, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
            }
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
            duel::select_target(d, tp, Some(&|d, c| eqtgfilter(d, c, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
            let h = effect::get_handler(d, a.e).nil();
            duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(h)), 1, 0, 0);
            true
        })
    }
    /// `AA.eqlim(e, c)`
    pub fn eqlim() -> LuaFn {
        effect_card_fn(|d, e, c| {
            let h = effect::get_handler(d, e).nil();
            card::get_controler(d, c) == effect::get_handler_player(d, e) || card::get_equip_target(d, h) == Some(c)
        })
    }
    /// `AA.eqop`
    pub fn eqop() -> LuaFn {
        operation(|d, a| {
            let c = effect::get_handler(d, a.e).nil();
            if !card::is_location(d, c, LOCATION_SZONE) || !card::is_relate_to_effect(d, c, a.e) || card::is_status(d, c, STATUS_LEAVE_CONFIRMED) {
                return;
            }
            let tc = duel::first_target(d);
            if let Some(tc) = tc.filter(|&tc| card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc)) {
                duel::equip(d, a.tp, c, tc, true, false);
                let e1 = effect::create_effect(d, c);
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
                effect::set_value_fn(d, e1, eqlim());
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                card::register_effect(d, c, e1, false);
            } else {
                card::cancel_to_grave(d, c, false);
            }
        })
    }
    /// `AA.eqsfilter(c, tp)`
    pub fn eqsfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
        card::is_set_card(d, c, &[SET_ATTRACTION as u16])
            && card::is_trap(d, c)
            && card::get_equip_target(d, c).is_some_and(|et| duel::is_existing_matching_card(d, Some(&|d, x| eqmfilter(d, x, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, et.into()))
    }
    /// `AA.eqmfilter(c, tp)`
    pub fn eqmfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
        card::is_faceup(d, c) && (card::is_set_card(d, c, &[SET_AMAZEMENT as u16]) || !card::is_controler(d, c, tp))
    }
    /// `AA.qeqetg`
    pub fn qeqetg() -> LuaFn {
        cost_target(|d, a| {
            let tp = a.tp;
            if a.chkc().is_some() {
                return false;
            }
            if a.chk == Some(0) {
                return duel::is_existing_target(d, Some(&|d, c| eqsfilter(d, c, tp)), tp, LOCATION_SZONE, 0, 1, NO_EX);
            }
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
            duel::select_target(d, tp, Some(&|d, c| eqsfilter(d, c, tp)), tp, LOCATION_SZONE, 0, 1, 1, false, NO_EX);
            true
        })
    }
    /// `AA.qeqeop`
    pub fn qeqeop() -> LuaFn {
        operation(|d, a| {
            let tp = a.tp;
            let tg = duel::get_target_cards(d, a.e).nil();
            let Some(tc) = group::get_first(d, &tg) else { return };
            let et = card::get_equip_target(d, tc);
            let mg = duel::get_matching_group(d, Some(&|d, x| eqmfilter(d, x, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, et.into());
            if mg.is_empty() {
                return;
            }
            let sel = group::select(d, &mg, tp, 1, 1, false, NO_EX).nil();
            let mc = group::get_first(d, &sel).nil();
            if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, mc) {
                duel::equip(d, tp, tc, mc, true, false);
            }
        })
    }
}
/// `aux.AddAttractionEquipProc(c)`: "Ɐttraction" Traps: equip this card to an "Amazement"
/// monster you control or a face-up monster your opponent controls
pub fn add_attraction_equip_proc(d: &mut Duel, c: CardId) {
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_cost(d, e1, Some(super::remain_field_cost()));
    effect::set_target(d, e1, Some(aa::eqtg()));
    effect::set_operation(d, e1, Some(aa::eqop()));
    card::register_effect(d, c, e1, false);
}
/// `aux.AttractionEquipCon(self)`: equipped to a monster controlled by the card's controller
/// (`self`) or the opponent
pub fn attraction_equip_con(self_: bool) -> LuaFn {
    lua_fn(move |d, a| {
        let e = a.e(0);
        let h = effect::get_handler(d, e).nil();
        match card::get_equip_target(d, h) {
            Some(et) => Value::Bool((card::get_controler(d, et) == effect::get_handler_player(d, e)) == self_),
            None => Value::Nil,
        }
    })
}
/// `aux.AddAmazementQuickEquipEffect(c, id)`
pub fn add_amazement_quick_equip_effect(d: &mut Duel, c: CardId, id: u32) {
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, super::stringid(id, 1));
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e2, 1, id, 0, 0);
    effect::set_target(d, e2, Some(aa::qeqetg()));
    effect::set_operation(d, e2, Some(aa::qeqeop()));
    card::register_effect(d, c, e2, false);
}

/// `SForce.*`
pub mod sforce {
    use super::*;
    use std::sync::LazyLock;
    fn cost_filter(d: &mut Duel, c: CardId) -> bool {
        card::is_set_card(d, c, &[SET_S_FORCE as u16]) && card::is_able_to_remove_as_cost(d, c, None)
    }
    static BANISH_COST: LazyLock<LuaFn> = LazyLock::new(|| {
        crate::cost::replaceable(
            cost_target(|d, a| {
                let tp = a.tp;
                if a.chk == Some(0) {
                    return duel::is_existing_matching_card(d, Some(&cost_filter), tp, LOCATION_HAND, 0, 1, NO_EX);
                }
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
                let rg = duel::select_matching_card(d, tp, Some(&cost_filter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
                duel::remove(d, rg, POS_FACEUP as u8, REASON_COST, None, None);
                true
            }),
            None,
        )
    });
    /// `SForce.BanishCost`
    pub fn banish_cost() -> LuaFn {
        BANISH_COST.clone()
    }
    fn column_filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
        card::is_set_card(d, c, &[SET_S_FORCE as u16]) && card::is_monster(d, c) && card::is_controler(d, c, tp) && card::is_faceup(d, c)
    }
    /// `SForce.ColumnTarget(e, cc)`
    pub fn column_target() -> LuaFn {
        effect_card_fn(|d, e, cc| {
            let hp = effect::get_handler_player(d, e);
            let cg = card::get_column_group(d, cc, 0, 0);
            group::is_exists(d, &cg, &|d, c| column_filter(d, c, hp), 1, cc.into())
        })
    }
}

/// `Ursarctic.*`
pub mod ursarctic {
    use super::*;
    fn spcostfilter(d: &mut Duel, c: CardId) -> bool {
        card::is_level_above(d, c, 7) && card::is_location(d, c, LOCATION_HAND)
    }
    fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
        let c = effect::get_handler(d, a.e).nil();
        if a.chk == Some(0) {
            return duel::check_release_group_cost(d, a.tp, Some(&spcostfilter), 1, 1, true, None, c.into());
        }
        let g = duel::select_release_group_cost(d, a.tp, Some(&spcostfilter), 1, 1, true, None, c.into());
        duel::release(d, g, REASON_COST, None);
        true
    }
    fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
        let c = effect::get_handler(d, a.e).nil();
        if a.chk == Some(0) {
            return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
        }
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
        true
    }
    fn spop(id: u32) -> LuaFn {
        operation(move |d, a| {
            let c = effect::get_handler(d, a.e).nil();
            if card::is_relate_to_effect(d, c, a.e) {
                duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
            }
            // You cannot Special Summon for the rest of this turn, except monsters with a Level
            let e1 = effect::create_effect(d, c);
            effect::set_description(d, e1, super::super::stringid(id, 1));
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
            effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
            effect::set_target_range(d, e1, 1, 0);
            effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_level_above(d, c, 0))));
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
            duel::register_effect(d, e1, a.tp);
        })
    }
    /// `Ursarctic.CreateSpSummonQuickEffect(c, id)`
    pub fn create_sp_summon_quick_effect(d: &mut Duel, c: CardId, id: u32) -> EffectId {
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, super::super::stringid(id, 0));
        effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
        effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
        effect::set_code(d, e1, EVENT_FREE_CHAIN);
        effect::set_range(d, e1, LOCATION_HAND);
        effect::set_count_limit(d, e1, 1, id, 0, 0);
        effect::set_condition(d, e1, Some(lua_fn(|d, _a| duel::is_main_phase(d, None))));
        effect::set_cost(d, e1, Some(crate::cost::replaceable(cost_target(spcost), None)));
        effect::set_target(d, e1, Some(cost_target(sptg)));
        effect::set_operation(d, e1, Some(spop(id)));
        effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
        e1
    }
    /// `Ursarctic.AddSpSummonQuickEffect(c, id)`
    pub fn add_sp_summon_quick_effect(d: &mut Duel, c: CardId, id: u32) {
        let e = create_sp_summon_quick_effect(d, c, id);
        card::register_effect(d, c, e, false);
    }
}

/// `aux.DoubleSnareValidity(c, range, property)`: the card's negation effect counts for "Double
/// Snare"
pub fn double_snare_validity(d: &mut Duel, c: Option<CardId>, range: u32, property: Option<u32>) {
    if let Some(c) = c {
        let eff = effect::create_effect(d, c);
        effect::set_type(d, eff, EFFECT_TYPE_SINGLE);
        effect::set_property(d, eff, EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_SINGLE_RANGE | property.unwrap_or(0), 0);
        effect::set_range(d, eff, range);
        effect::set_code(d, eff, 3682106);
        card::register_effect(d, c, eff, false);
    }
}

/// The effect arguments as a function takes them (`e, tp, eg, ep, ev, re, r, rp`), plus `chk`
/// (and `chkc`) when given.
fn eff_args(a: &EffArgs, chk: Option<i64>, chkc: Option<CardId>) -> Vec<Value> {
    let mut v = vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ];
    if let Some(chk) = chk {
        v.push(Value::Int(chk));
        if let Some(c) = chkc {
            v.push(Value::Card(c));
        }
    }
    v
}

/// `Cyberdark.*`: the equip effects of the "Cyberdark" monsters
pub mod cyberdark {
    use super::*;
    /// `Cyberdark.EquipFilter(f)`: a monster in the GY that can be equipped (`f(c)`, `(c, tp)`)
    pub fn equip_filter(f: Option<CardFilter>) -> std::sync::Arc<dyn Fn(&mut Duel, CardId, u8) -> bool + Send + Sync> {
        std::sync::Arc::new(move |d, c, tp| card::check_unique_on_field(d, c, tp, None, None) && !card::is_forbidden(d, c) && f.as_ref().is_none_or(|f| f(d, c)))
    }
    type EqFilter = std::sync::Arc<dyn Fn(&mut Duel, CardId, u8) -> bool + Send + Sync>;
    /// `Cyberdark.EquipTarget(f, targets, mandatory)`
    pub fn equip_target(f: Option<CardFilter>, targets: bool, mandatory: bool) -> LuaFn {
        let f = equip_filter(f);
        if targets { equip_target_tg(f, mandatory) } else { equip_target_ntg(f, mandatory) }
    }
    /// `Cyberdark.EquipOperation(f, op, targets)` (`op(c, e, tp, tc)`)
    pub fn equip_operation(f: Option<CardFilter>, op: LuaFn, targets: bool) -> LuaFn {
        let f = equip_filter(f);
        if targets { equip_operation_tg(f, op) } else { equip_operation_ntg(f, op) }
    }
    /// `Cyberdark.EquipTarget_TG(f, mandatory)`
    pub fn equip_target_tg(f: EqFilter, mandatory: bool) -> LuaFn {
        cost_target(move |d, a| {
            let tp = a.tp;
            let wc = duel::is_player_affected_by_effect(d, tp, EFFECT_CYBERDARK_WORLD).is_some();
            if let Some(chkc) = a.chkc() {
                return card::is_location(d, chkc, LOCATION_GRAVE) && (wc || card::is_controler(d, chkc, tp)) && f(d, chkc, tp);
            }
            let loc = if wc { LOCATION_GRAVE } else { 0 };
            if a.chk == Some(0) {
                return mandatory || (duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0 && duel::is_existing_target(d, Some(&|d, c| f(d, c, tp)), tp, LOCATION_GRAVE, loc, 1, NO_EX));
            }
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
            let g = duel::select_target(d, tp, Some(&|d, c| f(d, c, tp)), tp, LOCATION_GRAVE, loc, 1, 1, false, NO_EX);
            duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, g.clone(), 1, 0, 0);
            duel::set_operation_info(d, 0, CATEGORY_EQUIP, g, 1, 0, 0);
            true
        })
    }
    /// `Cyberdark.EquipTarget_NTG(f, mandatory)`
    pub fn equip_target_ntg(f: EqFilter, mandatory: bool) -> LuaFn {
        cost_target(move |d, a| {
            let tp = a.tp;
            let wc = duel::is_player_affected_by_effect(d, tp, EFFECT_CYBERDARK_WORLD).is_some();
            let (loc, player) = if wc { (LOCATION_GRAVE, PLAYER_ALL as u8) } else { (0, tp) };
            if a.chk == Some(0) {
                return mandatory || (duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0 && duel::is_existing_matching_card(d, Some(&|d, c| f(d, c, tp)), tp, LOCATION_GRAVE, loc, 1, NO_EX));
            }
            duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, player, 0);
            duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, player, LOCATION_GRAVE as i32);
            true
        })
    }
    /// `Cyberdark.EquipOperation_TG(f, op)`
    pub fn equip_operation_tg(f: EqFilter, op: LuaFn) -> LuaFn {
        operation(move |d, a| {
            let tp = a.tp;
            if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
                return;
            }
            let c = effect::get_handler(d, a.e).nil();
            if card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, a.e) {
                return;
            }
            if let Some(tc) = duel::first_target(d) {
                if card::is_relate_to_effect(d, tc, a.e) && f(d, tc, tp) {
                    op.call(d, &[Value::Card(c), Value::Effect(a.e), Value::Int(tp as i64), Value::Card(tc)]);
                }
            }
        })
    }
    /// `Cyberdark.EquipOperation_NTG(f, op)`
    pub fn equip_operation_ntg(f: EqFilter, op: LuaFn) -> LuaFn {
        operation(move |d, a| {
            let tp = a.tp;
            if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
                return;
            }
            let c = effect::get_handler(d, a.e).nil();
            if card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, a.e) {
                return;
            }
            let wc = duel::is_player_affected_by_effect(d, tp, EFFECT_CYBERDARK_WORLD).is_some();
            let loc = if wc { LOCATION_GRAVE } else { 0 };
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
            let g = duel::select_matching_card(d, tp, Some(&super::super::necro_valley_filter(|d, c| f(d, c, tp))), tp, LOCATION_GRAVE, loc, 1, 1, false, NO_EX).nil();
            if let Some(tc) = group::get_first(d, &g) {
                op.call(d, &[Value::Card(c), Value::Effect(a.e), Value::Int(tp as i64), Value::Card(tc)]);
            }
        })
    }
}

/// `Drytron.*`
pub mod drytron {
    use super::*;
    use std::sync::LazyLock;
    fn tribute_cost_filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
        ((card::is_set_card(d, c, &[SET_DRYTRON as u16]) && card::is_monster(d, c)) || card::is_ritual_monster(d, c))
            && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
            && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
    }
    fn tribute_base_cost(d: &mut Duel, a: &EffArgs) -> bool {
        let (c, tp) = (effect::get_handler(d, a.e).nil(), a.tp);
        if a.chk == Some(0) {
            return duel::check_release_group_cost(d, tp, Some(&|d, x| tribute_cost_filter(d, x, tp)), 1, 1, true, None, c.into());
        }
        let sg = duel::select_release_group_cost(d, tp, Some(&|d, x| tribute_cost_filter(d, x, tp)), 1, 1, true, None, c.into());
        duel::release(d, sg, REASON_COST, None);
        true
    }
    fn tribute_extra_cost(d: &mut Duel, a: &EffArgs) -> bool {
        let c = effect::get_handler(d, a.e).nil();
        let id = card::get_original_code(d, c);
        if a.chk == Some(0) {
            return duel::get_custom_activity_count(d, id, a.tp, ACTIVITY_SPSUMMON) == 0;
        }
        // You cannot Special Summon monsters, except monsters that cannot be Normal Summoned/Set, the turn you activate this effect
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, super::super::stringid(id, 1));
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
        effect::set_target_range(d, e1, 1, 0);
        effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_summonable_card(d, c))));
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        duel::register_effect(d, e1, a.tp);
        true
    }
    static TRIBUTE_COST: LazyLock<LuaFn> = LazyLock::new(|| {
        // extracon(base, e, tp, ...)
        let extracon = lua_fn(|d, a| duel::get_location_count(d, a.p(2), LOCATION_MZONE as u8, None, None, None).0 > 0);
        crate::cost::and(vec![crate::cost::replaceable(cost_target(tribute_base_cost), Some(extracon)), cost_target(tribute_extra_cost)])
    });
    /// `Drytron.TributeCost`
    pub fn tribute_cost() -> LuaFn {
        TRIBUTE_COST.clone()
    }
}

/// `rmcount` of `Effect.CreateMysteruneQPEffect`: a number or `function(e,tp,eg,ep,ev,re,r,rp)`
#[derive(Clone)]
pub enum RmCount {
    N(i64),
    F(LuaFn),
}
/// `Effect.CreateMysteruneQPEffect(c, id, uniquecat, uniquetg, uniqueop, rmcount, uniqueprop,
/// uniquecode)`: the activation of the "Mysterune" Quick-Play Spells (with `uniquecode`: two
/// effects, the unique one first)
pub mod mysterune {
    use super::*;
    fn skipop(d: &mut Duel, a: &EffArgs) {
        if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
            return;
        }
        // Skip next Battle Phase
        let e1 = effect::create_effect(d, effect::get_handler(d, a.e).nil());
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        effect::set_code(d, e1, EFFECT_SKIP_BP);
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        effect::set_target_range(d, e1, 1, 0);
        if duel::is_turn_player(d, a.tp) && duel::is_battle_phase(d, None) {
            let turn = duel::get_turn_count(d, None) as i64;
            effect::set_label(d, e1, &[turn]);
            effect::set_condition(d, e1, Some(lua_fn(|d, a| {
                let e = a.e(0);
                duel::get_turn_count(d, None) as i64 != effect::get_label(d, e)
            })));
            effect::set_reset(d, e1, RESET_PHASE + PHASE_BATTLE + RESET_SELF_TURN, 2);
        } else {
            effect::set_reset(d, e1, RESET_PHASE + PHASE_BATTLE + RESET_SELF_TURN, 1);
        }
        duel::register_effect(d, e1, a.tp);
    }
    fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
        card::is_set_card(d, c, &[SET_RUNICK as u16])
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
            && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), Some(ZONES_EMZ)).0 > 0
    }
    fn sptg(d: &mut Duel, a: &EffArgs, chk: i64) -> bool {
        let (e, tp) = (a.e, a.tp);
        if chk == 0 {
            return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
        }
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
        true
    }
    fn spop(d: &mut Duel, a: &EffArgs) {
        let (e, tp) = (a.e, a.tp);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        if !g.is_empty() {
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, Some(ZONES_EMZ));
        }
        skipop(d, a);
    }
    fn count(d: &mut Duel, a: &EffArgs, rmcount: &RmCount) -> i64 {
        match rmcount {
            RmCount::N(n) => *n,
            RmCount::F(f) => f.call(d, &eff_args(a, None, None)).int(),
        }
    }
    fn rmtg(d: &mut Duel, a: &EffArgs, chk: i64, uniquetg: &Option<LuaFn>, rmcount: &RmCount) -> bool {
        let tp = a.tp;
        let ct = count(d, a, rmcount);
        if chk == 0 {
            return uniquetg.as_ref().is_none_or(|f| f.call(d, &eff_args(a, Some(0), None)).truthy())
                && ct > 0
                && duel::is_player_can_remove(d, tp, None)
                && duel::get_field_group_count(d, tp, 0, LOCATION_DECK) as i64 >= ct;
        }
        if let Some(f) = uniquetg {
            f.call(d, &eff_args(a, Some(1), None));
        }
        if ct > 0 {
            duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, ct as u8, 1 - tp, LOCATION_DECK as i32);
        }
        true
    }
    fn rmop(d: &mut Duel, a: &EffArgs, uniqueop: &Option<LuaFn>, rmcount: &RmCount) {
        let tp = a.tp;
        if uniqueop.as_ref().is_none_or(|f| f.call(d, &eff_args(a, None, None)).truthy()) && duel::is_player_can_remove(d, tp, None) {
            let ct = count(d, a, rmcount);
            let rg = duel::get_decktop_group(d, 1 - tp, ct as u32);
            if !rg.is_empty() {
                duel::disable_shuffle_check(d, true);
                if uniqueop.is_some() {
                    duel::break_effect(d); // don't break effect if we're only banishing
                }
                duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None);
            }
        }
        skipop(d, a);
    }
    fn rmop_fn(uniqueop: Option<LuaFn>, rmcount: RmCount) -> LuaFn {
        operation(move |d, a| rmop(d, a, &uniqueop, &rmcount))
    }
    fn choicetg(id: u32, uniquecat: u64, uniquetg: Option<LuaFn>, uniqueop: Option<LuaFn>, rmcount: RmCount, uniqueprop: Option<u32>) -> LuaFn {
        LuaFn::new(move |d, args| {
            let a = EffArgs::with_chk(args);
            if let Some(chkc) = a.chkc() {
                return match &uniquetg {
                    Some(f) => f.call(d, &eff_args(&a, a.chk, Some(chkc))),
                    None => Value::Nil,
                };
            }
            let b1 = rmtg(d, &a, 0, &uniquetg, &rmcount);
            let b2 = sptg(d, &a, 0);
            if a.chk == Some(0) {
                return Value::Bool(b1 || b2);
            }
            let sel = duel::select_effect(d, a.tp, &[(b1, super::super::stringid(id, 0)), (b2, super::super::stringid(id, 1))]);
            if sel == Some(1) {
                if let Some(p) = uniqueprop {
                    effect::set_property(d, a.e, p, 0);
                }
                effect::set_category(d, a.e, uniquecat | CATEGORY_REMOVE);
                rmtg(d, &a, 1, &uniquetg, &rmcount);
                effect::set_operation(d, a.e, Some(rmop_fn(uniqueop.clone(), rmcount.clone())));
            } else if sel == Some(2) {
                effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
                effect::set_property(d, a.e, 0, 0);
                sptg(d, &a, 1);
                effect::set_operation(d, a.e, Some(operation(spop)));
            } else {
                effect::set_category(d, a.e, 0);
                effect::set_property(d, a.e, 0, 0);
                effect::set_operation(d, a.e, None);
            }
            Value::Nil
        })
    }
    /// The effects of `Effect.CreateMysteruneQPEffect`: `(None, e1)`, or `(Some(e0), e1)` with
    /// `uniquecode`.
    #[allow(clippy::too_many_arguments)]
    pub fn create_mysterune_qp_effect(
        d: &mut Duel,
        c: CardId,
        id: u32,
        uniquecat: Option<u64>,
        uniquetg: Option<LuaFn>,
        uniqueop: Option<LuaFn>,
        rmcount: Option<RmCount>,
        uniqueprop: Option<u32>,
        uniquecode: Option<u32>,
    ) -> (Option<EffectId>, EffectId) {
        let uniquecat = uniquecat.unwrap_or(0);
        let rmcount = rmcount.unwrap_or(RmCount::N(0));
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
        effect::set_code(d, e1, EVENT_FREE_CHAIN);
        effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER + TIMING_END_PHASE));
        effect::set_count_limit(d, e1, 1, id, 0, EFFECT_COUNT_CODE_OATH as u8);
        if let Some(code) = uniquecode {
            // have the unique effect as separate
            let e0 = effect::create_effect(d, c);
            effect::set_description(d, e0, super::super::stringid(id, 0));
            effect::set_category(d, e0, uniquecat | CATEGORY_REMOVE);
            effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
            effect::set_code(d, e0, code);
            effect::set_count_limit(d, e0, 1, id, 0, EFFECT_COUNT_CODE_OATH as u8);
            if let Some(p) = uniqueprop {
                effect::set_property(d, e0, p, 0);
            }
            let (tg, rc) = (uniquetg.clone(), rmcount.clone());
            effect::set_target(d, e0, Some(LuaFn::new(move |d, args| {
                let a = EffArgs::with_chk(args);
                if let Some(chkc) = a.chkc() {
                    return match &tg {
                        Some(f) => f.call(d, &eff_args(&a, a.chk, Some(chkc))),
                        None => Value::Nil,
                    };
                }
                if a.chk == Some(0) {
                    return Value::Bool(rmtg(d, &a, 0, &tg, &rc));
                }
                rmtg(d, &a, 1, &tg, &rc);
                Value::Nil
            })));
            effect::set_operation(d, e0, Some(rmop_fn(uniqueop, rmcount)));
            // complete the Special Summon effect
            effect::set_description(d, e1, super::super::stringid(id, 1));
            effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
            effect::set_target(d, e1, Some(cost_target(|d, a| {
                let chk = a.chk.unwrap_or(0);
                if chk == 0 {
                    return sptg(d, a, chk);
                }
                let desc = effect::get_description(d, a.e);
                duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
                sptg(d, a, chk)
            })));
            effect::set_operation(d, e1, Some(operation(spop)));
            (Some(e0), e1)
        } else {
            effect::set_target(d, e1, Some(choicetg(id, uniquecat, uniquetg, uniqueop, rmcount, uniqueprop)));
            (None, e1)
        }
    }
}

/// `Vernusylph.*`
pub mod vernusylph {
    use super::*;
    /// "Flourishing Hils": its strings are used for all of them
    const STRINGBASE: u32 = 9350312;
    fn spcostfilter(d: &mut Duel, c: CardId) -> bool {
        (card::is_monster(d, c) || card::is_set_card(d, c, &[SET_VERNUSYLPH as u16])) && card::is_discardable(d, c, None, None)
    }
    fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
        let c = effect::get_handler(d, a.e).nil();
        let tp = a.tp;
        if a.chk == Some(0) {
            return card::is_discardable(d, c, None, None) && duel::is_existing_matching_card(d, Some(&spcostfilter), tp, LOCATION_HAND, 0, 1, c.into());
        }
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
        let mut g = duel::select_matching_card(d, tp, Some(&spcostfilter), tp, LOCATION_HAND, 0, 1, 1, false, c.into()).nil();
        g.insert(c);
        duel::send_to_grave(d, g, REASON_COST + REASON_DISCARD, None, None);
        true
    }
    fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: Option<u32>) -> bool {
        card::is_attribute(d, c, ATTRIBUTE_EARTH) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None) && !code.is_some_and(|code| card::is_code(d, c, &[code]))
    }
    fn spop(d: &mut Duel, a: &EffArgs, uniqueop: &LuaFn) {
        let (e, tp) = (a.e, a.tp);
        let r = uniqueop.call(d, &eff_args(a, None, None));
        let (proceed, exempt_id) = match &r {
            Value::Multi(v) => (v.first().is_some_and(|x| x.truthy()), v.get(1).filter(|x| x.truthy()).map(|x| x.int() as u32)),
            v => (v.truthy(), None),
        };
        let f = super::super::necro_valley_filter(|d, c| spfilter(d, c, e, tp, exempt_id));
        if proceed
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&f), tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::select_yes_no(d, tp, super::super::stringid(STRINGBASE, 1))
        {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            let sg = duel::select_matching_card(d, tp, Some(&f), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
            if !sg.is_empty() {
                duel::break_effect(d);
                duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
        // Cannot activate monster effects, except EARTH monsters'
        let e1 = effect::create_effect(d, effect::get_handler(d, e).nil());
        effect::set_description(d, e1, super::super::stringid(STRINGBASE, 2));
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
        effect::set_target_range(d, e1, 1, 0);
        effect::set_value_fn(d, e1, lua_fn(|d, a| {
            let re = a.e(1);
            let h = effect::get_handler(d, re).nil();
            effect::is_monster_effect(d, re) && card::is_attribute_except(d, h, ATTRIBUTE_EARTH, None, 0, None)
        }));
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        duel::register_effect(d, e1, tp);
    }
    /// `Vernusylph.CreateSpSummonEffect(c, id, desc, uniquecat, uniquetg, uniqueop)`
    pub fn create_sp_summon_effect(d: &mut Duel, c: CardId, id: u32, desc: u32, uniquecat: u64, uniquetg: LuaFn, uniqueop: LuaFn) -> EffectId {
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, super::super::stringid(id, desc));
        effect::set_category(d, e1, uniquecat | CATEGORY_SPECIAL_SUMMON);
        effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
        effect::set_range(d, e1, LOCATION_HAND);
        effect::set_count_limit(d, e1, 1, id, desc as u8, 0);
        effect::set_cost(d, e1, Some(crate::cost::replaceable(cost_target(spcost), None)));
        effect::set_target(d, e1, Some(LuaFn::new(move |d, args| {
            let a = EffArgs::with_chk(args);
            if a.chk == Some(0) {
                return uniquetg.call(d, &eff_args(&a, a.chk, None));
            }
            uniquetg.call(d, &eff_args(&a, a.chk, None));
            duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
            Value::Nil
        })));
        effect::set_operation(d, e1, Some(operation(move |d, a| spop(d, a, &uniqueop))));
        e1
    }
    /// `Vernusylph.AddSpSummonEffect(c, id, desc, uniquecat, uniquetg, uniqueop)`
    pub fn add_sp_summon_effect(d: &mut Duel, c: CardId, id: u32, desc: u32, uniquecat: u64, uniquetg: LuaFn, uniqueop: LuaFn) {
        let e = create_sp_summon_effect(d, c, id, desc, uniquecat, uniquetg, uniqueop);
        card::register_effect(d, c, e, false);
    }
}

/// `aux.WelcomeLabrynthTrapDestroyOperation(e, tp)`: the additional destruction "Labrynth
/// Labyrinth" gives "Welcome Labrynth" Normal Traps
pub fn welcome_labrynth_trap_destroy_operation(d: &mut Duel, e: EffectId, tp: u8) {
    let c = effect::get_handler(d, e).nil();
    let addeff = duel::is_player_affected_by_effect(d, tp, CARD_LABRYNTH_LABYRINTH);
    let ok = addeff.is_some_and(|ae| effect::check_count_limit(d, ae, tp))
        && effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE)
        && effect::get_active_type(d, e) == TYPE_TRAP
        && card::is_set_card(d, c, &[SET_WELCOME_LABRYNTH as u16])
        && !card::is_status(d, c, STATUS_ACT_FROM_HAND)
        && duel::is_existing_matching_card(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into())
        && duel::select_yes_no(d, tp, super::stringid(CARD_LABRYNTH_LABYRINTH, 1));
    if !ok {
        return;
    }
    effect::use_count_limit(d, addeff.unwrap(), tp, 1, false);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let g = duel::select_matching_card(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, c.into()).nil();
    if !g.is_empty() {
        duel::hint_selection(d, &g, true);
        duel::break_effect(d);
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
/// `c:IsRikkaReleasable(tp)`: a Plant (or a monster of the opponent "Rikka Konkon" allows)
pub fn is_rikka_releasable(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_race(d, c, RACE_PLANT) || (card::is_controler(d, c, 1 - tp) && !card::is_has_effect(d, c, CARD_RIKKA_KONKON, None).is_empty())
}

/// `Arcana.*`: the coin tosses of "Arcana Force" monsters
pub mod arcana {
    use super::*;
    /// `Arcana.TossCoin(c, tp)`: a coin toss, or the player's choice under "Light Barrier"
    pub fn toss_coin(d: &mut Duel, _c: CardId, tp: u8) -> i32 {
        if duel::is_player_affected_by_effect(d, tp, CARD_LIGHT_BARRIER).is_none() {
            return duel::toss_coin(d, tp, 1).first().copied().unwrap_or(0);
        }
        let h = super::super::get_coin_effect_hint_string(COIN_HEADS as i32).unwrap_or(0);
        let t = super::super::get_coin_effect_hint_string(COIN_TAILS as i32).unwrap_or(0);
        let op = duel::select_option(d, tp, true, &[h, t]);
        if op == 0 { COIN_HEADS as i32 } else { COIN_TAILS as i32 }
    }
    /// `Arcana.RegisterCoinResult(c, coin)`
    pub fn register_coin_result(d: &mut Duel, c: CardId, coin: i32) {
        let desc = super::super::get_coin_effect_hint_string(coin).unwrap_or(0);
        card::register_flag_effect(d, c, CARD_REVERSAL_OF_FATE, RESET_EVENT + RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, coin as i64, desc);
    }
    /// `Arcana.GetCoinResult(c)`
    pub fn get_coin_result(d: &mut Duel, c: CardId) -> Option<i64> {
        card::get_flag_effect_label(d, c, CARD_REVERSAL_OF_FATE).first().copied()
    }
    /// `Arcana.SetCoinResult(c, coin)`
    pub fn set_coin_result(d: &mut Duel, c: CardId, coin: i64) -> bool {
        card::set_flag_effect_label(d, c, CARD_REVERSAL_OF_FATE, coin as u32)
    }
}

/// `Infernoid.*`
pub mod infernoid {
    use super::*;
    fn spfilter(d: &mut Duel, c: CardId) -> bool {
        card::is_set_card(d, c, &[SET_INFERNOID as u16]) && card::is_monster(d, c) && card::is_able_to_remove_as_cost(d, c, None)
    }
    fn get_locations(d: &mut Duel, c: CardId, tp: u8) -> u32 {
        let mut locations = LOCATION_HAND;
        if duel::is_player_affected_by_effect(d, tp, 69832741).is_some() {
            locations |= LOCATION_MZONE;
        } else if !card::is_has_effect(d, c, 34822850, None).is_empty() {
            locations |= LOCATION_GRAVE | LOCATION_MZONE;
        } else {
            locations |= LOCATION_GRAVE;
        }
        locations
    }
    fn summon_condition(n: usize) -> LuaFn {
        lua_fn(move |d, a| {
            let e = a.e(0);
            let Some(c) = a.card(1) else { return true };
            let tp = card::get_controler(d, c);
            let g = duel::get_matching_group(d, Some(&super::super::faceup_filter(|d, c| card::is_type(d, c, TYPE_EFFECT))), tp, LOCATION_MZONE, 0, NO_EX);
            let sum = group::get_sum(d, &g, &|d, c| if card::is_type(d, c, TYPE_XYZ) { card::get_rank(d, c) as i64 } else { card::get_level(d, c) as i64 });
            if sum > 8 {
                return false;
            }
            let locations = get_locations(d, c, tp);
            if locations & LOCATION_MZONE == 0 {
                let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
                ft > 0 && duel::is_existing_matching_card(d, Some(&spfilter), tp, locations, 0, n as u32, c.into())
            } else {
                let mg = duel::get_matching_group(d, Some(&spfilter), tp, locations, 0, c.into());
                let rescon = super::super::chkf_mmz(1);
                matches!(super::super::select_unselect_group(d, &mg, Some(e), tp, Some(n), Some(n), Some(&rescon), 0, tp, None, None, None, false), Err(true))
            }
        })
    }
    fn summon_target(n: usize) -> LuaFn {
        lua_fn(move |d, a| {
            let (e, tp) = (a.e(0), a.p(1));
            let c = effect::get_handler(d, e).nil();
            let locations = get_locations(d, c, tp);
            let mg = duel::get_matching_group(d, Some(&spfilter), tp, locations, 0, c.into());
            let rescon = super::super::chkf_mmz(1);
            let g = super::super::select_unselect_group(d, &mg, Some(e), tp, Some(n), Some(n), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true).unwrap_or_default();
            if !g.is_empty() {
                effect::set_label_object(d, e, Value::Group(g));
                return true;
            }
            false
        })
    }
    fn summon_operation() -> LuaFn {
        lua_fn(|d, a| {
            let Value::Group(g) = effect::get_label_object(d, a.e(0)) else { return };
            duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
        })
    }
    /// `Infernoid.RegisterSummonProcedure(c, monstersToBanish)`
    pub fn register_summon_procedure(d: &mut Duel, c: CardId, monsters_to_banish: usize) {
        // special summon condition
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
        effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
        effect::set_value_fn(d, e1, aux_false());
        card::register_effect(d, c, e1, false);
        // special summon
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_FIELD);
        effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
        effect::set_range(d, e2, if monsters_to_banish > 1 { LOCATION_HAND | LOCATION_GRAVE } else { LOCATION_HAND });
        effect::set_condition(d, e2, Some(summon_condition(monsters_to_banish)));
        effect::set_target(d, e2, Some(summon_target(monsters_to_banish)));
        effect::set_operation(d, e2, Some(summon_operation()));
        card::register_effect(d, c, e2, false);
    }
}

/// `DragonRuler.*`: the discard costs of the "Dragon Ruler" monsters
pub mod dragon_ruler {
    use super::*;
    fn discard_with_other_cost(filter: fn(&mut Duel, CardId, u32) -> bool, attr: u32) -> LuaFn {
        LuaFn::new(move |d, args| {
            let a = EffArgs::with_chk(args);
            let c = effect::get_handler(d, a.e).nil();
            let tp = a.tp;
            if a.chk == Some(0) {
                return Value::Bool(duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, attr)), tp, LOCATION_HAND, 0, 1, c.into()));
            }
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
            let mut g = duel::select_matching_card(d, tp, Some(&|d, x| filter(d, x, attr)), tp, LOCATION_HAND, 0, 1, 1, false, c.into()).nil();
            g.insert(c);
            duel::send_to_grave(d, g, REASON_COST | REASON_DISCARD, None, None);
            // (false: `Cost.AND` stops here, before the self-discard)
            Value::Bool(false)
        })
    }
    fn other_discard_to_grave_filter(d: &mut Duel, c: CardId, attr: u32) -> bool {
        card::is_attribute(d, c, attr) && card::is_discardable(d, c, None, None) && card::is_able_to_grave_as_cost(d, c)
    }
    fn other_discard_filter(d: &mut Duel, c: CardId, attr: u32) -> bool {
        (card::is_attribute(d, c, attr) || card::is_race(d, c, RACE_DRAGON)) && card::is_discardable(d, c, None, None)
    }
    /// `DragonRuler.SelfDiscardToGraveCost(attr)` (Level 7: to the GY, no Dragons)
    pub fn self_discard_to_grave_cost(attr: u32) -> LuaFn {
        crate::cost::and(vec![discard_with_other_cost(other_discard_to_grave_filter, attr), crate::cost::self_discard_to_grave()])
    }
    /// `DragonRuler.SelfDiscardCost(attr)` (lower Levels: any discard, Dragons allowed)
    pub fn self_discard_cost(attr: u32) -> LuaFn {
        crate::cost::and(vec![discard_with_other_cost(other_discard_filter, attr), crate::cost::self_discard()])
    }
}
