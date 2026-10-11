//! `Card.*` as scripts see it: the core's functions, the ones the global scripts redefine, and
//! the ones they add (`utility.lua`, `proc_workaround.lua`, ...).

use crate::constants::*;
pub use ygo_core::api::card::*;
use ygo_core::api::card as raw;
use ygo_core::duel::Duel;
use ygo_core::types::*;

/// `c:RegisterEffect(e, forced)`. (`utility.lua` adds marker effects for extra codes, but
/// `proc_workaround.lua` wraps it again without passing them on: they are never registered.)
pub fn register_effect(d: &mut Duel, c: CardId, e: EffectId, forced: bool) -> Option<i32> {
    let reg = raw::register_effect(d, c, e, forced);
    match reg {
        Some(r) if r > 0 => {}
        _ => return reg,
    }
    super::wrap_cannot_summon_target(d, e);
    reg
}
/// `c:IsRelateToEffect(e)`: while an activated effect resolves, the relation to its chain link.
pub fn is_relate_to_effect(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    if !(super::duel::is_chain_solving(d) && ygo_core::api::effect::is_activated(d, e)) {
        return raw::is_relate_to_effect(d, c, e);
    }
    if super::duel::get_reason_effect(d) == Some(e) {
        return raw::is_relate_to_chain(d, c, 0);
    }
    for link in 0..super::duel::get_current_chain(d, false) {
        let te = super::duel::get_chain_info(d, link as u8, &[CHAININFO_TRIGGERING_EFFECT]);
        if te.first().and_then(|v| v.as_effect()) == Some(e) {
            return raw::is_relate_to_chain(d, c, link as u8);
        }
    }
    raw::is_relate_to_effect(d, c, e)
}
pub fn is_able_to_hand(d: &mut Duel, c: CardId, p: Option<u8>) -> bool {
    !raw::is_location(d, c, LOCATION_HAND) && raw::is_able_to_hand(d, c, p)
}
pub fn is_able_to_deck(d: &mut Duel, c: CardId) -> bool {
    !raw::is_location(d, c, LOCATION_DECK) && raw::is_able_to_deck(d, c)
}
pub fn is_able_to_extra(d: &mut Duel, c: CardId) -> bool {
    !raw::is_location(d, c, LOCATION_EXTRA) && raw::is_able_to_extra(d, c)
}
pub fn is_able_to_grave(d: &mut Duel, c: CardId, reason: Option<u32>) -> bool {
    !raw::is_location(d, c, LOCATION_GRAVE) && raw::is_able_to_grave(d, c, reason)
}
pub fn is_able_to_remove(d: &mut Duel, c: CardId, p: Option<u8>, pos: Option<u8>, reason: Option<u32>) -> bool {
    !raw::is_location(d, c, LOCATION_REMOVED) && raw::is_able_to_remove(d, c, p, pos, reason)
}
/// `c:IsCanBeXyzMaterial(xyz, player, reason)`
pub fn is_can_be_xyz_material(d: &mut Duel, c: CardId, xyz: Option<CardId>, p: Option<u8>, reason: Option<u32>) -> bool {
    if xyz == Some(c) {
        return false;
    }
    let p = p.unwrap_or(d.core.reason_player);
    raw::is_can_be_xyz_material(d, c, xyz, Some(p), Some(reason.unwrap_or(REASON_XYZ | REASON_MATERIAL)))
}
/// `c:IsExactType(typ, ...)` (`utility.lua`: the type equals `typ`)
pub fn is_exact_type(d: &mut Duel, c: CardId, typ: u32) -> bool {
    raw::get_type(d, c) == typ
}
pub fn is_exact_type_ctx(d: &mut Duel, c: CardId, typ: u32, scard: Option<CardId>, sumtype: u64, p: Option<u8>) -> bool {
    raw::get_type_ctx(d, c, scard, sumtype, p) == typ
}
/// `c:HasFlagEffect(id, ct = 1)` (`utility.lua`)
pub fn has_flag_effect(d: &Duel, c: CardId, id: u32, ct: Option<usize>) -> bool {
    raw::get_flag_effect(d, c, id) >= ct.unwrap_or(1)
}

// ---- utility.lua ----

pub fn is_monster(d: &mut Duel, c: CardId) -> bool {
    raw::is_type(d, c, TYPE_MONSTER)
}
pub fn is_spell(d: &mut Duel, c: CardId) -> bool {
    raw::is_type(d, c, TYPE_SPELL)
}
pub fn is_trap(d: &mut Duel, c: CardId) -> bool {
    raw::is_type(d, c, TYPE_TRAP)
}
pub fn is_spell_trap(d: &mut Duel, c: CardId) -> bool {
    raw::is_type(d, c, TYPE_SPELL | TYPE_TRAP)
}
/// `c:IsCompositeType(t)`: all bits of `t` (the core's `IsExactType`).
pub fn is_composite_type(d: &mut Duel, c: CardId, t: u32) -> bool {
    raw::is_exact_type(d, c, t)
}
pub fn is_composite_type_ctx(d: &mut Duel, c: CardId, t: u32, scard: Option<CardId>, sumtype: u64, p: Option<u8>) -> bool {
    raw::is_exact_type_ctx(d, c, t, scard, sumtype, p)
}
pub fn is_quick_play_spell(d: &mut Duel, c: CardId) -> bool {
    is_composite_type(d, c, TYPE_SPELL | TYPE_QUICKPLAY)
}
pub fn is_continuous_spell(d: &mut Duel, c: CardId) -> bool {
    is_composite_type(d, c, TYPE_SPELL | TYPE_CONTINUOUS)
}
pub fn is_equip_spell(d: &mut Duel, c: CardId) -> bool {
    is_composite_type(d, c, TYPE_SPELL | TYPE_EQUIP)
}
pub fn is_field_spell(d: &mut Duel, c: CardId) -> bool {
    is_composite_type(d, c, TYPE_SPELL | TYPE_FIELD)
}
pub fn is_ritual_spell(d: &mut Duel, c: CardId) -> bool {
    is_composite_type(d, c, TYPE_SPELL | TYPE_RITUAL)
}
pub fn is_link_spell(d: &mut Duel, c: CardId) -> bool {
    is_composite_type(d, c, TYPE_SPELL | TYPE_LINK)
}
pub fn is_continuous_trap(d: &mut Duel, c: CardId) -> bool {
    is_composite_type(d, c, TYPE_TRAP | TYPE_CONTINUOUS)
}
pub fn is_counter_trap(d: &mut Duel, c: CardId) -> bool {
    is_composite_type(d, c, TYPE_TRAP | TYPE_COUNTER)
}
pub fn is_fusion_monster(d: &mut Duel, c: CardId) -> bool {
    is_composite_type(d, c, TYPE_MONSTER | TYPE_FUSION)
}
pub fn is_ritual_monster(d: &mut Duel, c: CardId) -> bool {
    is_composite_type(d, c, TYPE_MONSTER | TYPE_RITUAL)
}
pub fn is_synchro_monster(d: &mut Duel, c: CardId) -> bool {
    is_composite_type(d, c, TYPE_MONSTER | TYPE_SYNCHRO)
}
pub fn is_xyz_monster(d: &mut Duel, c: CardId) -> bool {
    is_composite_type(d, c, TYPE_MONSTER | TYPE_XYZ)
}
pub fn is_pendulum_monster(d: &mut Duel, c: CardId) -> bool {
    is_composite_type(d, c, TYPE_MONSTER | TYPE_PENDULUM)
}
pub fn is_link_monster(d: &mut Duel, c: CardId) -> bool {
    is_composite_type(d, c, TYPE_MONSTER | TYPE_LINK)
}
pub fn is_normal_spell(d: &mut Duel, c: CardId) -> bool {
    raw::get_type(d, c) == TYPE_SPELL
}
pub fn is_normal_trap(d: &mut Duel, c: CardId) -> bool {
    raw::get_type(d, c) == TYPE_TRAP
}
pub fn is_normal_spell_trap(d: &mut Duel, c: CardId) -> bool {
    is_normal_spell(d, c) || is_normal_trap(d, c)
}
pub fn is_continuous_spell_trap(d: &mut Duel, c: CardId) -> bool {
    is_continuous_spell(d, c) || is_continuous_trap(d, c)
}
pub fn is_equip_card(d: &mut Duel, c: CardId) -> bool {
    raw::is_type(d, c, TYPE_EQUIP)
}
pub fn is_monster_card(d: &Duel, c: CardId) -> bool {
    raw::is_original_type(d, c, TYPE_MONSTER)
}
pub fn is_spell_card(d: &Duel, c: CardId) -> bool {
    raw::is_original_type(d, c, TYPE_SPELL)
}
pub fn is_trap_card(d: &Duel, c: CardId) -> bool {
    raw::is_original_type(d, c, TYPE_TRAP)
}
pub fn is_spell_trap_card(d: &Duel, c: CardId) -> bool {
    raw::is_original_type(d, c, TYPE_SPELL | TYPE_TRAP)
}
pub fn is_trap_monster(d: &Duel, c: CardId) -> bool {
    is_trap_card(d, c) && (raw::get_original_level(d, c) > 0 || raw::get_original_attribute(d, c) > 0 || raw::get_original_race(d, c) > 0)
}
/// `c:IsEquipTrap()`: a Trap that equips itself (`self_equip_trap`) or whose activation stays on
/// the field (`HasRemainFieldCost`).
pub fn is_equip_trap(d: &mut Duel, c: CardId) -> bool {
    if !is_trap(d, c) {
        return false;
    }
    if crate::card_info(d, c).self_equip_trap {
        return true;
    }
    match raw::get_activate_effect(d, c).first() {
        Some(&ae) => super::effect::has_remain_field_cost(d, ae),
        None => false,
    }
}
pub fn get_main_card_type(d: &mut Duel, c: CardId) -> u32 {
    raw::get_type(d, c) & (TYPE_MONSTER | TYPE_SPELL | TYPE_TRAP)
}
pub fn is_effect_monster(d: &mut Duel, c: CardId) -> bool {
    is_monster(d, c) && raw::is_type(d, c, TYPE_EFFECT)
}
pub fn is_non_effect_monster(d: &mut Duel, c: CardId) -> bool {
    is_monster(d, c) && !raw::is_type(d, c, TYPE_EFFECT)
}
pub fn is_base_attack(d: &mut Duel, c: CardId, atk: i32) -> bool {
    raw::get_base_attack(d, c) == atk
}
pub fn is_text_attack(d: &Duel, c: CardId, atk: i32) -> bool {
    raw::get_text_attack(d, c) == atk
}
pub fn is_previous_attack_on_field(d: &Duel, c: CardId, atk: i32) -> bool {
    raw::get_previous_attack_on_field(d, c) == atk
}
pub fn is_base_defense(d: &mut Duel, c: CardId, def: i32) -> bool {
    has_defense(d, c) && raw::get_base_defense(d, c) == def
}
pub fn is_text_defense(d: &mut Duel, c: CardId, def: i32) -> bool {
    has_defense(d, c) && raw::get_text_defense(d, c) == def
}
pub fn is_previous_defense_on_field(d: &mut Duel, c: CardId, def: i32) -> bool {
    has_defense(d, c) && raw::get_previous_defense_on_field(d, c) == def
}
/// `c:IsLevelBetween(a, b)` (inclusive, either order)
pub fn is_level_between(d: &mut Duel, c: CardId, first: u32, second: u32) -> bool {
    let (a, b) = if first > second { (second, first) } else { (first, second) };
    raw::is_level_above(d, c, a) && raw::is_level_below(d, c, b)
}
pub fn is_original_level(d: &mut Duel, c: CardId, lvl: u32) -> bool {
    has_level(d, c) && raw::get_original_level(d, c) == lvl
}
pub fn is_previous_level_on_field(d: &mut Duel, c: CardId, lvl: u32) -> bool {
    has_level(d, c) && raw::get_previous_level_on_field(d, c) == lvl
}
pub fn is_original_rank(d: &mut Duel, c: CardId, rank: u32) -> bool {
    has_rank(d, c) && raw::get_original_rank(d, c) == rank
}
pub fn is_previous_rank_on_field(d: &mut Duel, c: CardId, rank: u32) -> bool {
    has_rank(d, c) && raw::get_previous_rank_on_field(d, c) == rank
}
pub fn is_scale(d: &mut Duel, c: CardId, scales: &[u32]) -> bool {
    let s = get_scale(d, c);
    scales.contains(&s)
}
fn summoned(d: &Duel, c: CardId, t: u32) -> bool {
    raw::is_summon_type(d, c, &[t])
}
pub fn is_normal_summoned(d: &Duel, c: CardId) -> bool {
    summoned(d, c, SUMMON_TYPE_NORMAL)
}
pub fn is_tribute_summoned(d: &Duel, c: CardId) -> bool {
    summoned(d, c, SUMMON_TYPE_TRIBUTE)
}
pub fn is_flip_summoned(d: &Duel, c: CardId) -> bool {
    summoned(d, c, SUMMON_TYPE_FLIP)
}
pub fn is_gemini_summoned(d: &Duel, c: CardId) -> bool {
    summoned(d, c, SUMMON_TYPE_GEMINI)
}
pub fn is_special_summoned(d: &Duel, c: CardId) -> bool {
    summoned(d, c, SUMMON_TYPE_SPECIAL)
}
pub fn is_ritual_summoned(d: &Duel, c: CardId) -> bool {
    summoned(d, c, SUMMON_TYPE_RITUAL)
}
pub fn is_fusion_summoned(d: &Duel, c: CardId) -> bool {
    summoned(d, c, SUMMON_TYPE_FUSION)
}
pub fn is_synchro_summoned(d: &Duel, c: CardId) -> bool {
    summoned(d, c, SUMMON_TYPE_SYNCHRO)
}
pub fn is_xyz_summoned(d: &Duel, c: CardId) -> bool {
    summoned(d, c, SUMMON_TYPE_XYZ)
}
pub fn is_pendulum_summoned(d: &Duel, c: CardId) -> bool {
    summoned(d, c, SUMMON_TYPE_PENDULUM)
}
pub fn is_link_summoned(d: &Duel, c: CardId) -> bool {
    summoned(d, c, SUMMON_TYPE_LINK)
}
pub fn is_previous_attribute_on_field(d: &Duel, c: CardId, attr: u32) -> bool {
    raw::get_previous_attribute_on_field(d, c) & attr > 0
}
pub fn is_previous_race_on_field(d: &Duel, c: CardId, race: u64) -> bool {
    raw::get_previous_race_on_field(d, c) & race > 0
}
pub fn is_previous_type_on_field(d: &Duel, c: CardId, t: u32) -> bool {
    raw::get_previous_type_on_field(d, c) & t > 0
}
pub fn is_previous_code_on_field(d: &Duel, c: CardId, codes: &[u32]) -> bool {
    let pc = raw::get_previous_code_on_field(d, c);
    codes.iter().any(|&code| pc.contains(&code))
}
pub fn is_previous_sequence(d: &Duel, c: CardId, seq: u32) -> bool {
    raw::get_previous_sequence(d, c) == seq
}
pub fn is_owner(d: &Duel, c: CardId, p: u8) -> bool {
    raw::get_owner(d, c) == p
}
pub fn is_battle_position(d: &Duel, c: CardId, pos: u32) -> bool {
    raw::get_battle_position(d, c) == pos
}
pub fn has_counter(d: &Duel, c: CardId, counter: u16) -> bool {
    raw::get_counter(d, c, counter) > 0
}
pub fn has_equip_card(d: &Duel, c: CardId) -> bool {
    raw::get_equip_count(d, c) > 0
}
pub fn is_co_linked(d: &mut Duel, c: CardId, count: Option<usize>) -> bool {
    raw::get_mutual_linked_group_count(d, c) >= count.unwrap_or(1)
}
pub fn is_destination(d: &Duel, c: CardId, dest: u32) -> bool {
    raw::get_destination(d, c) == dest
}
pub fn is_leave_field_dest(d: &mut Duel, c: CardId, dest: u32) -> bool {
    raw::get_leave_field_dest(d, c) == dest
}
pub fn is_field_id(d: &Duel, c: CardId, fid: u32) -> bool {
    raw::get_field_id(d, c) == fid
}
pub fn is_real_field_id(d: &Duel, c: CardId, fid: u32) -> bool {
    raw::get_real_field_id(d, c) == fid
}
/// `Card.IsReasonCard(rc, c)`: the reason card of `c` is `rc` (note the order).
pub fn is_reason_card(d: &Duel, rc: CardId, c: CardId) -> bool {
    raw::get_reason_card(d, c) == Some(rc)
}
pub fn is_reason_effect(d: &Duel, c: CardId, e: EffectId) -> bool {
    raw::get_reason_effect(d, c) == Some(e)
}
pub fn is_reason_player(d: &Duel, c: CardId, p: u8) -> bool {
    raw::get_reason_player(d, c) == p
}
/// `c:AddSetcodesRule(code, copyable, setcodes...)`: the effects created.
pub fn add_setcodes_rule(d: &mut Duel, c: CardId, code: u32, copyable: bool, setcodes: &[u16]) -> Vec<EffectId> {
    use ygo_core::api::effect as eff;
    let prop = if copyable { 0 } else { EFFECT_FLAG_UNCOPYABLE };
    let mut t = Vec::new();
    for &sc in setcodes {
        let e = eff::create_effect(d, c);
        eff::set_type(d, e, EFFECT_TYPE_SINGLE);
        eff::set_property(d, e, EFFECT_FLAG_CANNOT_DISABLE + prop, 0);
        eff::set_code(d, e, EFFECT_ADD_SETCODE);
        eff::set_value(d, e, sc as i64);
        eff::set_label(d, e, &[code as i64]);
        eff::set_condition(
            d,
            e,
            Some(LuaFn::new(|d, a| {
                let e = a[0].as_effect().unwrap();
                let c = eff::get_handler(d, e).unwrap();
                let label = eff::get_label(d, e);
                let ocr = raw::get_original_code_rule(d, c)[0];
                Value::Bool(if label > 0 && ocr as i64 == label { raw::is_code(d, c, &[ocr]) } else { true })
            })),
        );
        register_effect(d, c, e, false);
        t.push(e);
    }
    t
}
pub fn check_adjacent(d: &Duel, c: CardId) -> bool {
    let p = raw::get_controler(d, c);
    let seq = raw::get_sequence(d, c);
    if seq > 4 {
        return false;
    }
    (seq > 0 && ygo_core::api::duel::check_location(d, p, LOCATION_MZONE, seq - 1)) || (seq < 4 && ygo_core::api::duel::check_location(d, p, LOCATION_MZONE, seq + 1))
}
fn adjacent_flag(d: &Duel, c: CardId) -> Option<(u32, u8)> {
    let seq = raw::get_sequence(d, c);
    if seq > 4 {
        return None;
    }
    let tp = raw::get_controler(d, c);
    let mut flag = 0u32;
    if seq > 0 && ygo_core::api::duel::check_location(d, tp, LOCATION_MZONE, seq - 1) {
        flag |= 1 << (seq - 1);
    }
    if seq < 4 && ygo_core::api::duel::check_location(d, tp, LOCATION_MZONE, seq + 1) {
        flag |= 1 << (seq + 1);
    }
    if flag == 0 { None } else { Some((flag, tp)) }
}
/// `c:SelectAdjacent(sel_player)`: the zone chosen (`None` if there is none).
pub fn select_adjacent(d: &mut Duel, c: CardId, sel_player: Option<u8>) -> Option<u32> {
    let (flag, tp) = adjacent_flag(d, c)?;
    let sp = sel_player.unwrap_or(tp);
    let loc1 = if tp == sp { LOCATION_MZONE } else { 0 };
    let loc2 = if loc1 == 0 { LOCATION_MZONE } else { 0 };
    let shift = loc2 * 4;
    ygo_core::api::duel::hint(d, HINT_SELECTMSG as u8, sp, HINTMSG_TOZONE as u64);
    let zone = ygo_core::api::duel::select_disable_field(d, sp, 1, loc1, loc2, Some(!(flag << shift)), false) >> shift;
    ygo_core::api::duel::hint(d, HINT_ZONE as u8, tp, zone as u64);
    Some(zone.trailing_zeros())
}
pub fn move_adjacent(d: &mut Duel, c: CardId, sel_player: Option<u8>) {
    let Some((flag, tp)) = adjacent_flag(d, c) else { return };
    let sp = sel_player.unwrap_or(tp);
    let loc1 = if tp == sp { LOCATION_MZONE } else { 0 };
    let loc2 = if loc1 == 0 { LOCATION_MZONE } else { 0 };
    let shift = loc2 * 4;
    ygo_core::api::duel::hint(d, HINT_SELECTMSG as u8, sp, HINTMSG_TOZONE as u64);
    let zone = ygo_core::api::duel::select_disable_field(d, sp, 1, loc1, loc2, Some(!(flag << shift)), false);
    ygo_core::api::duel::move_sequence(d, c, (zone >> shift).trailing_zeros() as u8, None);
}
/// `c:IsColumn(seq, tp, loc)`
pub fn is_column(d: &Duel, c: CardId, seq: u32, tp: Option<u8>, loc: Option<u32>) -> bool {
    if !raw::is_on_field(d, c) {
        return false;
    }
    let mut cseq = raw::get_sequence(d, c);
    let mut seq = seq;
    let loc = loc.unwrap_or(raw::get_location(d, c));
    let tp = tp.unwrap_or(raw::get_controler(d, c));
    if raw::is_location(d, c, LOCATION_MZONE) {
        if cseq == 5 {
            cseq = 1;
        }
        if cseq == 6 {
            cseq = 3;
        }
    } else if cseq == 6 {
        cseq = 5;
    }
    if loc == LOCATION_MZONE {
        if seq == 5 {
            seq = 1;
        }
        if seq == 6 {
            seq = 3;
        }
    } else if seq == 6 {
        seq = 5;
    }
    if raw::is_controler(d, c, tp) { cseq == seq } else { cseq as i64 == 4 - seq as i64 }
}
fn update_stat(d: &mut Duel, c: CardId, code: u32, amt: i64, reset: Option<u32>, rc: Option<CardId>, reset_count: Option<u16>) {
    use ygo_core::api::effect as eff;
    let rc = rc.unwrap_or(c);
    let r = if c == rc { RESETS_STANDARD_DISABLE } else { RESETS_STANDARD };
    let reset = reset.unwrap_or(RESET_EVENT + r);
    let e1 = eff::create_effect(d, rc);
    eff::set_type(d, e1, EFFECT_TYPE_SINGLE);
    eff::set_code(d, e1, code);
    eff::set_value(d, e1, amt);
    eff::set_reset(d, e1, reset, reset_count.unwrap_or(1));
    register_effect(d, c, e1, false);
}
/// `c:UpdateAttack(amt, reset, rc, reset_count)`: the change.
pub fn update_attack(d: &mut Duel, c: CardId, amt: i64, reset: Option<u32>, rc: Option<CardId>, reset_count: Option<u16>) -> i32 {
    let atk = raw::get_attack(d, c);
    update_stat(d, c, EFFECT_UPDATE_ATTACK, amt, reset, rc, reset_count);
    raw::get_attack(d, c) - atk
}
pub fn update_defense(d: &mut Duel, c: CardId, amt: i64, reset: Option<u32>, rc: Option<CardId>, reset_count: Option<u16>) -> i32 {
    let def = raw::get_defense(d, c);
    update_stat(d, c, EFFECT_UPDATE_DEFENSE, amt, reset, rc, reset_count);
    raw::get_defense(d, c) - def
}
fn update_lvl(d: &mut Duel, c: CardId, code: u32, cur: i64, has: bool, amt: i64, reset: Option<u32>, rc: Option<CardId>) -> bool {
    if !has {
        return false;
    }
    // (unlike ATK, a too large decrease reduces as much as possible)
    let amt = if cur + amt <= 0 { -(cur - 1) } else { amt };
    use ygo_core::api::effect as eff;
    let rc = rc.unwrap_or(c);
    let r = if c == rc { RESETS_STANDARD_DISABLE } else { RESETS_STANDARD };
    let reset = reset.unwrap_or(RESET_EVENT + r);
    let e1 = eff::create_effect(d, rc);
    eff::set_type(d, e1, EFFECT_TYPE_SINGLE);
    eff::set_code(d, e1, code);
    eff::set_value(d, e1, amt);
    eff::set_reset(d, e1, reset, 0);
    register_effect(d, c, e1, false);
    true
}
pub fn update_level(d: &mut Duel, c: CardId, amt: i64, reset: Option<u32>, rc: Option<CardId>) -> i64 {
    let lv = raw::get_level(d, c) as i64;
    let has = raw::is_level_below(d, c, 2147483647);
    if update_lvl(d, c, EFFECT_UPDATE_LEVEL, lv, has, amt, reset, rc) { raw::get_level(d, c) as i64 - lv } else { 0 }
}
pub fn update_rank(d: &mut Duel, c: CardId, amt: i64, reset: Option<u32>, rc: Option<CardId>) -> i64 {
    let rk = raw::get_rank(d, c) as i64;
    let has = raw::is_rank_below(d, c, 2147483647);
    if update_lvl(d, c, EFFECT_UPDATE_RANK, rk, has, amt, reset, rc) { raw::get_rank(d, c) as i64 - rk } else { 0 }
}
pub fn update_link(d: &mut Duel, c: CardId, amt: i64, reset: Option<u32>, rc: Option<CardId>) -> i64 {
    let lk = raw::get_link(d, c) as i64;
    let has = raw::is_link_below(d, c, 2147483647);
    if update_lvl(d, c, EFFECT_UPDATE_LINK, lk, has, amt, reset, rc) { raw::get_link(d, c) as i64 - lk } else { 0 }
}
pub fn update_scale(d: &mut Duel, c: CardId, amt: i64, reset: Option<u32>, rc: Option<CardId>) -> i64 {
    use ygo_core::api::effect as eff;
    let rc = rc.unwrap_or(c);
    let r = if c == rc { RESETS_STANDARD_DISABLE } else { RESETS_STANDARD };
    let reset = reset.unwrap_or(RESET_EVENT + r);
    // (GetLeftScale always returns a number: the branch is always taken)
    let scl = raw::get_left_scale(d, c) as i64;
    let amt = if scl + amt <= 0 { -(scl - 1) } else { amt };
    let e1 = eff::create_effect(d, rc);
    eff::set_type(d, e1, EFFECT_TYPE_SINGLE);
    eff::set_code(d, e1, EFFECT_UPDATE_LSCALE);
    eff::set_value(d, e1, amt);
    eff::set_reset(d, e1, reset, 0);
    register_effect(d, c, e1, false);
    let e2 = eff::clone(d, e1);
    eff::set_code(d, e2, EFFECT_UPDATE_RSCALE);
    register_effect(d, c, e2, false);
    raw::get_left_scale(d, c) as i64 - scl
}
/// `Card.GetToBeLinkedZone(tc, c, tp, clink, emz)`: the zones `c` would point to `tc` from.
pub fn get_to_be_linked_zone(d: &mut Duel, tc: CardId, c: CardId, tp: u8, clink: bool, emz: bool) -> u32 {
    let mut zone: i64 = 0;
    let seq = raw::get_sequence(d, tc) as i64;
    let lm = |d: &mut Duel, x: CardId, m: u32| raw::is_link_marker(d, x, m);
    let sh = |n: f64| -> i64 { 1i64 << (n as i64) };
    if raw::is_location(d, tc, LOCATION_MZONE) && raw::is_controler(d, tc, tp) {
        if lm(d, c, LINK_MARKER_LEFT) && seq < 4 && (!clink || lm(d, tc, LINK_MARKER_RIGHT)) {
            zone |= 1 << (seq + 1);
        }
        if lm(d, c, LINK_MARKER_RIGHT) && seq > 0 && seq <= 4 && (!clink || lm(d, tc, LINK_MARKER_LEFT)) {
            zone |= 1 << (seq - 1);
        }
        if lm(d, c, LINK_MARKER_TOP_RIGHT) && (seq == 5 || seq == 6) && (!clink || lm(d, tc, LINK_MARKER_BOTTOM_LEFT)) {
            zone |= 1 << (2 * (seq - 5));
        }
        if lm(d, c, LINK_MARKER_TOP) && (seq == 5 || seq == 6) && (!clink || lm(d, tc, LINK_MARKER_BOTTOM)) {
            zone |= 1 << (2 * (seq - 5) + 1);
        }
        if lm(d, c, LINK_MARKER_TOP_LEFT) && (seq == 5 || seq == 6) && (!clink || lm(d, tc, LINK_MARKER_BOTTOM_RIGHT)) {
            zone |= 1 << (2 * (seq - 5) + 2);
        }
        if emz && lm(d, c, LINK_MARKER_BOTTOM_LEFT) && (seq == 0 || seq == 2) && (!clink || lm(d, tc, LINK_MARKER_TOP_RIGHT)) {
            zone |= sh(5.0 + seq as f64 / 2.0);
        }
        if emz && lm(d, c, LINK_MARKER_BOTTOM) && (seq == 1 || seq == 3) && (!clink || lm(d, tc, LINK_MARKER_TOP)) {
            zone |= sh(5.0 + (seq - 1) as f64 / 2.0);
        }
        if emz && lm(d, c, LINK_MARKER_BOTTOM_RIGHT) && (seq == 2 || seq == 4) && (!clink || lm(d, tc, LINK_MARKER_TOP_LEFT)) {
            zone |= sh(5.0 + (seq - 2) as f64 / 2.0);
        }
    } else if raw::is_location(d, tc, LOCATION_MZONE) {
        if lm(d, c, LINK_MARKER_TOP_RIGHT) && (seq == 5 || seq == 6 || (emz && (seq == 0 || seq == 2))) && (!clink || lm(d, tc, LINK_MARKER_TOP_RIGHT)) {
            if seq == 5 || seq == 6 {
                zone |= 1 << (-2 * (seq - 5) + 2);
            } else {
                zone |= sh(-(seq as f64) / 2.0 + 6.0);
            }
        }
        if lm(d, c, LINK_MARKER_TOP) && (seq == 5 || seq == 6 || (emz && (seq == 1 || seq == 3))) && (!clink || lm(d, tc, LINK_MARKER_TOP)) {
            if seq == 5 || seq == 6 {
                zone |= 1 << (-2 * (seq - 5) + 3);
            } else {
                zone |= sh(-((seq - 1) as f64) / 2.0 + 6.0);
            }
        }
        if lm(d, c, LINK_MARKER_TOP_LEFT) && (seq == 2 || seq == 4 || (emz && (seq == 2 || seq == 4))) && (!clink || lm(d, tc, LINK_MARKER_TOP_LEFT)) {
            if seq == 5 || seq == 6 {
                zone |= 1 << (-2 * (seq - 5) + 4);
            } else {
                zone |= sh(-((seq - 2) as f64) / 2.0 + 6.0);
            }
        }
    } else if raw::is_location(d, tc, LOCATION_SZONE) && raw::is_controler(d, tc, tp) {
        if lm(d, c, LINK_MARKER_BOTTOM_LEFT) && seq < 4 && (!clink || lm(d, tc, LINK_MARKER_TOP_RIGHT)) {
            zone |= 1 << (seq + 1);
        }
        if lm(d, c, LINK_MARKER_BOTTOM) && seq <= 4 && (!clink || lm(d, tc, LINK_MARKER_TOP)) {
            zone |= 1 << seq;
        }
        if lm(d, c, LINK_MARKER_BOTTOM_RIGHT) && seq > 0 && seq <= 4 && (!clink || lm(d, tc, LINK_MARKER_TOP_LEFT)) {
            // (the script calls the zone number here: an error)
            crate::lua_error("attempt to call a number value (local 'zone')");
        }
    }
    zone as u32
}
/// `c:GetScale()`: the scale it uses in its Pendulum Zone (left or right), else the left scale.
pub fn get_scale(d: &mut Duel, c: CardId) -> u32 {
    if !raw::is_type(d, c, TYPE_PENDULUM) {
        return 0;
    }
    if raw::is_location(d, c, LOCATION_PZONE) {
        let seq = raw::get_sequence(d, c);
        if seq == 0 || seq == 1 || seq == 6 { raw::get_left_scale(d, c) } else { raw::get_right_scale(d, c) }
    } else {
        raw::get_left_scale(d, c)
    }
}
pub fn is_odd_scale(d: &mut Duel, c: CardId) -> bool {
    raw::is_type(d, c, TYPE_PENDULUM) && get_scale(d, c) % 2 != 0
}
pub fn is_even_scale(d: &mut Duel, c: CardId) -> bool {
    raw::is_type(d, c, TYPE_PENDULUM) && get_scale(d, c) % 2 == 0
}
/// `c:CanSummonOrSet(ignore_count, e, minc, zone)`
pub fn can_summon_or_set(d: &mut Duel, c: CardId, ign: bool, e: Option<EffectId>, minc: u16, zone: Option<u32>) -> bool {
    raw::is_summonable(d, c, ign, e, minc, zone.map(|z| z as u16)) || raw::is_m_setable(d, c, ign, e, minc, zone)
}
pub fn is_battle_destroyed(d: &Duel, c: CardId) -> bool {
    raw::is_status(d, c, STATUS_BATTLE_DESTROYED) && raw::is_reason(d, c, REASON_BATTLE)
}
pub fn is_in_main_m_zone(d: &Duel, c: CardId, tp: Option<u8>) -> bool {
    raw::is_location(d, c, LOCATION_MMZONE) && tp.is_none_or(|tp| raw::is_controler(d, c, tp))
}
pub fn is_in_extra_m_zone(d: &Duel, c: CardId, tp: Option<u8>) -> bool {
    raw::is_location(d, c, LOCATION_EMZONE) && tp.is_none_or(|tp| raw::is_controler(d, c, tp))
}
pub fn announce_another_attribute(d: &mut Duel, c: CardId, tp: u8) -> u32 {
    let att = raw::get_attribute(d, c);
    ygo_core::api::duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATTRIBUTE as u64);
    let avail = if att & att.wrapping_sub(1) == 0 { !att & ATTRIBUTE_ALL } else { ATTRIBUTE_ALL };
    ygo_core::api::duel::announce_attribute(d, tp, 1, avail)
}
/// `c:IsAttributeExcept(att, scard, sumtype, player)`
pub fn is_attribute_except(d: &mut Duel, c: CardId, att: u32, scard: Option<CardId>, sumtype: u64, p: Option<u8>) -> bool {
    let a = raw::get_attribute_ctx(d, c, scard, sumtype, Some(p.unwrap_or(PLAYER_NONE as u8)));
    (a & att) != a
}
pub fn announce_another_race(d: &mut Duel, c: CardId, tp: u8) -> u64 {
    let race = raw::get_race(d, c);
    ygo_core::api::duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RACE as u64);
    let avail = if race & race.wrapping_sub(1) == 0 { !race & RACE_ALL } else { RACE_ALL };
    ygo_core::api::duel::announce_race(d, tp, 1, avail)
}
pub fn is_race_except(d: &mut Duel, c: CardId, race: u64, scard: Option<CardId>, sumtype: u64, p: Option<u8>) -> bool {
    let r = raw::get_race_ctx(d, c, scard, sumtype, Some(p.unwrap_or(PLAYER_NONE as u8)));
    (r & race) != r
}
pub fn is_sequence(d: &Duel, c: CardId, seqs: &[u32]) -> bool {
    let s = raw::get_sequence(d, c);
    seqs.contains(&s)
}
/// `c:HasLevel()`: Links have none, Xyz only with EFFECT_RANK_LEVEL(_S); Dark Synchros and
/// Level 0 do.
pub fn has_level(d: &mut Duel, c: CardId) -> bool {
    if is_monster(d, c) {
        let t = raw::get_type(d, c);
        t & TYPE_LINK != TYPE_LINK
            && (t & TYPE_XYZ != TYPE_XYZ && !(!raw::is_has_effect(d, c, EFFECT_RANK_LEVEL, None).is_empty() || !raw::is_has_effect(d, c, EFFECT_RANK_LEVEL_S, None).is_empty()))
            && !raw::is_status(d, c, STATUS_NO_LEVEL)
    } else if raw::is_original_type(d, c, TYPE_MONSTER) {
        !(raw::is_original_type(d, c, TYPE_XYZ + TYPE_LINK) || raw::is_status(d, c, STATUS_NO_LEVEL))
    } else {
        false
    }
}
pub fn has_rank(d: &mut Duel, c: CardId) -> bool {
    if is_monster(d, c) {
        raw::is_type(d, c, TYPE_XYZ) || (has_level(d, c) && (!raw::is_has_effect(d, c, EFFECT_LEVEL_RANK, None).is_empty() || !raw::is_has_effect(d, c, EFFECT_LEVEL_RANK_S, None).is_empty()))
    } else if raw::is_original_type(d, c, TYPE_MONSTER) {
        raw::is_original_type(d, c, TYPE_XYZ)
    } else {
        false
    }
}
pub fn has_defense(d: &mut Duel, c: CardId) -> bool {
    if is_monster(d, c) {
        !(raw::is_type(d, c, TYPE_LINK) || is_maximum_mode(d, c))
    } else if raw::is_original_type(d, c, TYPE_MONSTER) {
        !raw::is_original_type(d, c, TYPE_LINK)
    } else {
        false
    }
}
/// `c:IsMaximumMode()` (proc_maximum.lua)
pub fn is_maximum_mode(d: &Duel, c: CardId) -> bool {
    has_flag_effect(d, c, crate::global::FLAG_MAXIMUM_CENTER, None) || has_flag_effect(d, c, crate::global::FLAG_MAXIMUM_SIDE, None)
}
/// `tc:NegateEffects(c, reset, negates_cards, ct)`: "its effects are negated".
pub fn negate_effects(d: &mut Duel, tc: CardId, c: CardId, reset: Option<u32>, negates_cards: bool, ct: Option<u16>) {
    use ygo_core::api::effect as eff;
    let mut reset = reset.unwrap_or(RESET_EVENT | RESETS_STANDARD);
    reset |= RESET_EVENT | RESETS_STANDARD;
    let trap_monster_chk = negates_cards && raw::is_type(d, tc, TYPE_TRAPMONSTER);
    if trap_monster_chk {
        reset &= !(RESET_TOFIELD | RESET_LEAVE | RESET_TURN_SET);
    }
    let ct = ct.unwrap_or(1);
    ygo_core::api::duel::negate_related_chain(d, tc, RESET_TURN_SET);
    let e1 = eff::create_effect(d, c);
    eff::set_type(d, e1, EFFECT_TYPE_SINGLE);
    eff::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    eff::set_code(d, e1, EFFECT_DISABLE);
    eff::set_reset(d, e1, reset, ct);
    register_effect(d, tc, e1, false);
    let e2 = eff::clone(d, e1);
    eff::set_code(d, e2, EFFECT_DISABLE_EFFECT);
    eff::set_value(d, e2, RESET_TURN_SET as i64);
    register_effect(d, tc, e2, false);
    if trap_monster_chk {
        let e3 = eff::clone(d, e1);
        eff::set_code(d, e3, EFFECT_DISABLE_TRAPMONSTER);
        register_effect(d, tc, e3, false);
    }
}

fn match_setcode(set_code: u32, to_match: u32) -> bool {
    (set_code & 0xfff) == (to_match & 0xfff) && (set_code & to_match) == set_code
}
/// `c:ListsArchetypeAsMaterial(setcodes...)`
pub fn lists_archetype_as_material(d: &Duel, c: CardId, setcodes: &[u32]) -> bool {
    let m = crate::card_info(d, c).material_setcode;
    setcodes.iter().any(|&sc| m.iter().any(|&v| match_setcode(sc, v)))
}
/// `c:ListsCode(codes...)`: in `listed_names`, or (without those) in `fit_monster`.
pub fn lists_code(d: &Duel, c: CardId, codes: &[u32]) -> bool {
    let info = crate::card_info(d, c);
    // (Lua: a table, even an empty one, is used when it exists; an empty slice means none here)
    if !info.listed_names.is_empty() {
        codes.iter().any(|w| info.listed_names.contains(w))
    } else {
        codes.iter().any(|w| crate::class::fit_monster(d, c).contains(w))
    }
}
pub fn lists_code_with_archetype(d: &Duel, c: CardId, setcodes: &[u32]) -> bool {
    let info = crate::card_info(d, c);
    for &code in info.listed_names {
        let ms: Vec<u16> = ygo_core::api::duel::get_card_data_from_code(d, code).map_or(Vec::new(), |cd| cd.setcodes.iter().collect());
        if setcodes.iter().any(|&sc| ms.iter().any(|&m| match_setcode(sc, m as u32))) {
            return true;
        }
    }
    false
}
pub fn lists_card_type(d: &Duel, c: CardId, types: &[u32]) -> bool {
    let info = crate::card_info(d, c);
    types.iter().any(|&t| info.listed_card_types.iter().any(|&tt| t & tt != 0))
}
pub fn lists_archetype(d: &Duel, c: CardId, setcodes: &[u32]) -> bool {
    let info = crate::card_info(d, c);
    setcodes.iter().any(|w| info.listed_series.contains(w))
}
/// `c:ListsCodeAsMaterial(codes...)`
pub fn lists_code_as_material(d: &Duel, c: CardId, codes: &[u32]) -> bool {
    let material = crate::class::material(d, c);
    codes.iter().any(|w| material.contains(w))
}
pub fn is_negatable_monster(d: &mut Duel, c: CardId) -> bool {
    raw::is_faceup(d, c) && !raw::is_disabled(d, c) && (!is_non_effect_monster(d, c) || raw::get_original_type(d, c) & TYPE_EFFECT != 0)
}
pub fn is_negatable_spell_trap(d: &mut Duel, c: CardId) -> bool {
    raw::is_faceup(d, c) && !raw::is_disabled(d, c) && is_spell_trap(d, c)
}
pub fn is_negatable(d: &mut Duel, c: CardId) -> bool {
    is_negatable_monster(d, c) || is_negatable_spell_trap(d, c)
}
pub fn has_non_zero_attack(d: &mut Duel, c: CardId) -> bool {
    raw::is_faceup(d, c) && raw::get_attack(d, c) > 0
}
pub fn has_non_zero_defense(d: &mut Duel, c: CardId) -> bool {
    raw::is_faceup(d, c) && raw::get_defense(d, c) > 0
}
fn add_summon_condition(d: &mut Duel, c: CardId, code: u32, value: Option<LuaFn>) -> EffectId {
    use ygo_core::api::effect as eff;
    let e0 = eff::create_effect(d, c);
    eff::set_type(d, e0, EFFECT_TYPE_SINGLE);
    eff::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    eff::set_code(d, e0, code);
    if let Some(v) = value {
        eff::set_value_fn(d, e0, v);
    }
    register_effect(d, c, e0, false);
    e0
}
pub fn add_cannot_be_normal_summoned(d: &mut Duel, c: CardId) -> EffectId {
    add_summon_condition(d, c, EFFECT_CANNOT_SUMMON, None)
}
pub fn add_cannot_be_flip_summoned(d: &mut Duel, c: CardId) -> EffectId {
    add_summon_condition(d, c, EFFECT_CANNOT_FLIP_SUMMON, None)
}
pub fn add_cannot_be_special_summoned(d: &mut Duel, c: CardId) -> EffectId {
    add_summon_condition(d, c, EFFECT_SPSUMMON_CONDITION, None)
}
/// `c:AddMustBeSpecialSummoned()` (= `AddCannotBeSpecialSummoned`)
pub fn add_must_be_special_summoned(d: &mut Duel, c: CardId) -> EffectId {
    add_cannot_be_special_summoned(d, c)
}
pub fn add_must_be_special_summoned_by_card_effect(d: &mut Duel, c: CardId) -> EffectId {
    let v = LuaFn::new(|d, a| {
        let se = a.get(1).and_then(|v| v.as_effect());
        Value::Bool(se.is_some_and(|se| ygo_core::api::effect::is_has_type(d, se, EFFECT_TYPE_ACTIONS)))
    });
    add_summon_condition(d, c, EFFECT_SPSUMMON_CONDITION, Some(v))
}
pub fn add_must_be_ritual_summoned(d: &mut Duel, c: CardId) -> EffectId {
    add_summon_condition(d, c, EFFECT_SPSUMMON_CONDITION, Some(crate::aux::sumlimit(SUMMON_TYPE_RITUAL)))
}
pub fn add_must_be_fusion_summoned(d: &mut Duel, c: CardId) -> EffectId {
    add_summon_condition(d, c, EFFECT_SPSUMMON_CONDITION, Some(crate::aux::fuslimit()))
}
pub fn add_must_be_synchro_summoned(d: &mut Duel, c: CardId) -> EffectId {
    add_summon_condition(d, c, EFFECT_SPSUMMON_CONDITION, Some(crate::aux::synlimit()))
}
pub fn add_must_be_xyz_summoned(d: &mut Duel, c: CardId) -> EffectId {
    add_summon_condition(d, c, EFFECT_SPSUMMON_CONDITION, Some(crate::aux::xyzlimit()))
}
pub fn add_must_be_pendulum_summoned(d: &mut Duel, c: CardId) -> EffectId {
    add_summon_condition(d, c, EFFECT_SPSUMMON_CONDITION, Some(crate::aux::penlimit()))
}
pub fn add_must_be_link_summoned(d: &mut Duel, c: CardId) -> EffectId {
    add_summon_condition(d, c, EFFECT_SPSUMMON_CONDITION, Some(crate::aux::lnklimit()))
}
/// "Must first be X Summoned": (handler, sum_type) -> allowed
fn must_first(d: &mut Duel, c: CardId, f: fn(&mut Duel, CardId, u32) -> bool) -> EffectId {
    let v = LuaFn::new(move |d, a| {
        let e = a[0].as_effect().unwrap();
        let h = ygo_core::api::effect::get_handler(d, e).unwrap();
        let st = a.get(3).map_or(0, |v| v.int()) as u32;
        Value::Bool(f(d, h, st))
    });
    add_summon_condition(d, c, EFFECT_SPSUMMON_CONDITION, Some(v))
}
pub fn add_must_first_be_ritual_summoned(d: &mut Duel, c: CardId) -> EffectId {
    must_first(d, c, |d, h, st| raw::is_status(d, h, STATUS_PROC_COMPLETE) || st & SUMMON_TYPE_RITUAL == SUMMON_TYPE_RITUAL)
}
pub fn add_must_first_be_fusion_summoned(d: &mut Duel, c: CardId) -> EffectId {
    must_first(d, c, |d, h, st| !raw::is_location(d, h, LOCATION_EXTRA) || st & SUMMON_TYPE_FUSION == SUMMON_TYPE_FUSION)
}
pub fn add_must_first_be_synchro_summoned(d: &mut Duel, c: CardId) -> EffectId {
    must_first(d, c, |d, h, st| !raw::is_location(d, h, LOCATION_EXTRA) || st & SUMMON_TYPE_SYNCHRO == SUMMON_TYPE_SYNCHRO)
}
pub fn add_must_first_be_xyz_summoned(d: &mut Duel, c: CardId) -> EffectId {
    must_first(d, c, |d, h, st| !raw::is_location(d, h, LOCATION_EXTRA) || st & SUMMON_TYPE_XYZ == SUMMON_TYPE_XYZ)
}
pub fn add_must_first_be_pendulum_summoned(d: &mut Duel, c: CardId) -> EffectId {
    must_first(d, c, |d, h, st| raw::is_status(d, h, STATUS_PROC_COMPLETE) || st & SUMMON_TYPE_PENDULUM == SUMMON_TYPE_PENDULUM)
}
pub fn add_must_first_be_link_summoned(d: &mut Duel, c: CardId) -> EffectId {
    must_first(d, c, |d, h, st| !raw::is_location(d, h, LOCATION_EXTRA) || st & SUMMON_TYPE_LINK == SUMMON_TYPE_LINK)
}
/// `c:ListsCounter(counter_type)`: in `counter_list`, or (without one) `counter_place_list`.
pub fn lists_counter(d: &Duel, c: CardId, counter_type: u32) -> bool {
    let info = crate::card_info(d, c);
    if !info.counter_list.is_empty() { info.counter_list.contains(&counter_type) } else { info.counter_place_list.contains(&counter_type) }
}
pub fn places_counter(d: &Duel, c: CardId, counter_type: u32) -> bool {
    crate::card_info(d, c).counter_place_list.contains(&counter_type)
}
/// `Card.EquipByEffectLimit(e, c)`: equip limit "only to its owner, by the effect in the label"
pub fn equip_by_effect_limit() -> LuaFn {
    LuaFn::new(|d, a| {
        let e = a[0].as_effect().unwrap();
        let c = a.get(1).and_then(|v| v.as_card());
        if ygo_core::api::effect::get_owner(d, e) != c {
            return Value::Bool(false);
        }
        let Some(c) = c else { return Value::Bool(false) };
        let lo = ygo_core::api::effect::get_label_object(d, e);
        let eff = raw::get_card_effect(d, c, 89785779 + EFFECT_EQUIP_LIMIT);
        Value::Bool(eff.into_iter().any(|te| matches!(lo, Value::Effect(x) if x == te)))
    })
}
/// `c:EquipByEffectAndLimitRegister(e, tp, tc, code, mustbefaceup)`: equip `tc` to `c` with the
/// equip limit of `Card.EquipByEffectLimit` (and the flag `code`, if given).
pub fn equip_by_effect_and_limit_register(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId, code: Option<u32>, mustbefaceup: Option<bool>) -> bool {
    // (`false or mustbefaceup` is an explicit nil without it, and a boolean argument that is
    // present but nil is false in the C++ (`lua_get<bool, true>` only defaults when it is absent))
    if !ygo_core::api::duel::equip(d, tp, tc, c, mustbefaceup.unwrap_or(false), false) {
        return false;
    }
    if let Some(code) = code {
        raw::register_flag_effect(d, tc, code, RESET_EVENT + RESETS_STANDARD, 0, 0, 0, 0);
    }
    let te = ygo_core::api::effect::get_label_object(d, e);
    let e1 = ygo_core::api::effect::create_effect(d, c);
    ygo_core::api::effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    ygo_core::api::effect::set_property(d, e1, EFFECT_FLAG_OWNER_RELATE, 0);
    ygo_core::api::effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
    ygo_core::api::effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
    ygo_core::api::effect::set_value_fn(d, e1, equip_by_effect_limit());
    ygo_core::api::effect::set_label_object(d, e1, te);
    register_effect(d, tc, e1, false);
    true
}
/// `c:GetTributeRequirement()`: the script's `min_tribute_req`/`max_tribute_req` if it declares them
pub fn get_tribute_requirement(d: &mut Duel, c: CardId) -> (i32, i32) {
    let (lo, hi) = (crate::class::get(d, c, crate::class::MIN_TRIBUTE_REQ), crate::class::get(d, c, crate::class::MAX_TRIBUTE_REQ));
    if !lo.is_nil() && !hi.is_nil() {
        return (lo.int() as i32, hi.int() as i32);
    }
    match crate::card_info(d, c).tribute_req {
        Some((lo, hi)) => (lo as i32, hi as i32),
        None => raw::get_tribute_requirement(d, c),
    }
}
/// `c:GetSummonPhase()`: the phase the monster was Normal/Special Summoned in (0 if not recorded)
pub fn get_summon_phase(d: &mut Duel, c: CardId) -> i64 {
    if has_flag_effect(d, c, 160214042, None) { raw::get_flag_effect_label(d, c, 160214042).first().copied().unwrap_or(0) } else { 0 }
}
pub fn is_summon_phase(d: &mut Duel, c: CardId, phase: u32) -> bool {
    get_summon_phase(d, c) == phase as i64
}
pub fn is_summon_phase_main(d: &mut Duel, c: CardId) -> bool {
    let p = get_summon_phase(d, c);
    p == PHASE_MAIN1 as i64 || p == PHASE_MAIN2 as i64
}
pub fn is_summon_phase_battle(d: &mut Duel, c: CardId) -> bool {
    let p = get_summon_phase(d, c);
    p >= PHASE_BATTLE_START as i64 && p <= PHASE_BATTLE as i64
}
/// `c:IsReincarnationSummoned()` (cards_specific_functions.lua): Summoned by "Salamangreat"
/// Reincarnation (the flag of `aux.EnableCheckReincarnation`)
pub fn is_reincarnation_summoned(d: &mut Duel, c: CardId) -> bool {
    let label = raw::get_flag_effect_label(d, c, CARD_SALAMANGREAT_SANCTUARY).into_iter().fold(0, |l, x| l | x);
    label & (raw::get_summon_player(d, c) as i64 + 1) != 0
}
/// `c:IsNouvellesSummoned()` (cards_specific_functions.lua): Special Summoned by the effect of a
/// "Nouvelles" monster (a flag with the card's original code)
pub fn is_nouvelles_summoned(d: &mut Duel, c: CardId) -> bool {
    let code = get_original_code(d, c);
    has_flag_effect(d, c, code, None)
}
/// `c:IsRikkaReleasable(tp)` (cards_specific_functions.lua)
pub use crate::aux::is_rikka_releasable;
/// `Card.IsGeminiStatus`/`Card.EnableGeminiStatus` (proc_gemini.lua aliases)
pub use raw::{enable_gemini_state as enable_gemini_status, is_gemini_state as is_gemini_status};

macro_rules! card_info {
    ($(#[$m:meta] $name:ident, $field:ident, $t:ty;)*) => {$(
        #[$m]
        pub fn $name(d: &mut Duel, c: CardId, v: Option<$t>) -> $t {
            if let Some(v) = v {
                d.card_mut(c).data.$field = v;
            }
            d.card(c).data.$field
        }
    )*};
}
card_info! {
    /// `c:Code()` (the printed data) / `c:Code(v)` (sets it)
    info_code, code, u32;
    /// `c:Alias()` / `c:Alias(v)`
    info_alias, alias, u32;
    /// `c:Type()` / `c:Type(v)`
    info_type, type_, u32;
    /// `c:Level()` / `c:Level(v)`
    info_level, level, u32;
    /// `c:Attribute()` / `c:Attribute(v)`
    info_attribute, attribute, u32;
    /// `c:Race()` / `c:Race(v)`
    info_race, race, u64;
    /// `c:Attack()` / `c:Attack(v)`
    info_attack, attack, i32;
    /// `c:Defense()` / `c:Defense(v)`
    info_defense, defense, i32;
    /// `c:Rscale()` / `c:Rscale(v)`
    info_rscale, rscale, u32;
    /// `c:Lscale()` / `c:Lscale(v)`
    info_lscale, lscale, u32;
    /// `c:LinkMarker()` / `c:LinkMarker(v)`
    info_link_marker, link_marker, u32;
}
