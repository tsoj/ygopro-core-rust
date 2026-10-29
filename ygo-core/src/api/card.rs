//! `Card.*` (`libcard.cpp`). A Lua call with optional "summon context" arguments
//! (`c:GetCode(scard, sumtype, playerid)`) is a separate function with an `_ctx` suffix whose
//! `playerid` is `None` when Lua leaves it out.

use std::collections::BTreeSet;

use crate::constants::*;
use crate::duel::Duel;
use crate::types::*;

/// The player of a summon context: given, or the reason player for Fusion Summons.
fn ctx_player(d: &Duel, sumtype: u64, playerid: Option<u8>) -> u8 {
    match playerid {
        Some(p) => p,
        None if sumtype == SUMMON_TYPE_FUSION as u64 => d.core.reason_player,
        None => PLAYER_NONE as u8,
    }
}

/// Swaps the two players' halves of a zone mask when asked from the other side.
fn zone_for(d: &Duel, c: CardId, zone: u32, cp: Option<u8>) -> u32 {
    let ctrl = d.card(c).current.controler;
    let cp = cp.unwrap_or(ctrl);
    if cp == 1u8.wrapping_sub(ctrl) {
        ((zone & 0xffff) << 16) | (zone >> 16)
    } else {
        zone
    }
}

/// `c:GetCode()`: the code, and the second code if the card has one.
pub fn get_code(d: &mut Duel, c: CardId) -> Vec<u32> {
    let code = c.get_code(d);
    let ot = c.get_another_code(d);
    if ot != 0 { vec![code, ot] } else { vec![code] }
}
/// `c:GetCode(scard, sumtype, playerid)`: the codes as a material (empty: Lua nil).
pub fn get_code_ctx(d: &mut Duel, c: CardId, scard: Option<CardId>, sumtype: u64, playerid: Option<u8>) -> Vec<u32> {
    let p = ctx_player(d, sumtype, playerid);
    let mut codes = BTreeSet::new();
    c.get_summon_code(d, &mut codes, scard, sumtype, p);
    codes.into_iter().collect()
}
pub fn get_original_code(d: &Duel, c: CardId) -> u32 {
    let data = &d.card(c).data;
    if data.alias != 0 {
        let dif = data.code as i64 - data.alias as i64;
        if dif > -10 && dif < 10 {
            return data.alias;
        }
    }
    data.code
}
pub fn get_original_code_rule(d: &mut Duel, c: CardId) -> Vec<u32> {
    let mut eset = Vec::new();
    c.filter_effect(d, EFFECT_ADD_CODE, &mut eset, true);
    let data = d.card(c).data;
    if data.alias != 0 && eset.is_empty() {
        vec![data.alias]
    } else if let Some(&e) = eset.last() {
        let ot = e.get_value_card(d, Some(c), 0) as u32;
        vec![data.code, ot]
    } else {
        vec![data.code]
    }
}
/// `c:GetSetCard()` (empty: Lua nil)
pub fn get_set_card(d: &mut Duel, c: CardId) -> Vec<u16> {
    let mut s = BTreeSet::new();
    c.get_set_card(d, &mut s);
    s.into_iter().collect()
}
pub fn get_set_card_ctx(d: &mut Duel, c: CardId, scard: Option<CardId>, sumtype: u64, playerid: Option<u8>) -> Vec<u16> {
    let p = ctx_player(d, sumtype, playerid);
    let mut s = BTreeSet::new();
    c.get_summon_set_card(d, &mut s, scard, sumtype, p);
    s.into_iter().collect()
}
pub fn get_original_set_card(d: &Duel, c: CardId) -> Vec<u16> {
    c.get_origin_set_card(d).iter().collect()
}
pub fn get_previous_set_card(d: &Duel, c: CardId) -> Vec<u16> {
    d.note_read(c, 16384);
    let mut s = BTreeSet::new();
    c.get_pre_set_card(d, &mut s);
    s.into_iter().collect()
}
pub fn get_type(d: &mut Duel, c: CardId) -> u32 {
    c.get_type(d, None, 0, PLAYER_NONE as u8)
}
pub fn get_type_ctx(d: &mut Duel, c: CardId, scard: Option<CardId>, sumtype: u64, playerid: Option<u8>) -> u32 {
    let p = ctx_player(d, sumtype, playerid);
    c.get_type(d, scard, sumtype, p)
}
pub fn get_original_type(d: &Duel, c: CardId) -> u32 {
    d.card(c).data.type_
}
pub fn get_level(d: &mut Duel, c: CardId) -> u32 {
    c.get_level(d)
}
pub fn get_rank(d: &mut Duel, c: CardId) -> u32 {
    c.get_rank(d)
}
pub fn get_link(d: &mut Duel, c: CardId) -> u32 {
    c.get_link(d)
}
pub fn get_synchro_level(d: &mut Duel, c: CardId, scard: CardId) -> u32 {
    c.get_synchro_level(d, Some(scard))
}
pub fn get_ritual_level(d: &mut Duel, c: CardId, scard: CardId) -> u32 {
    c.get_ritual_level(d, Some(scard))
}
pub fn get_original_level(d: &Duel, c: CardId) -> u32 {
    let cc = d.card(c);
    if cc.data.type_ & (TYPE_XYZ | TYPE_LINK) != 0 || cc.status & STATUS_NO_LEVEL != 0 { 0 } else { cc.data.level }
}
pub fn get_original_rank(d: &Duel, c: CardId) -> u32 {
    let cc = d.card(c);
    if cc.data.type_ & TYPE_XYZ == 0 { 0 } else { cc.data.level }
}
pub fn is_xyz_level(d: &mut Duel, c: CardId, xyzcard: CardId, lv: u32) -> bool {
    c.check_xyz_level(d, xyzcard, lv)
}
pub fn get_left_scale(d: &mut Duel, c: CardId) -> u32 {
    c.get_lscale(d)
}
pub fn get_original_left_scale(d: &Duel, c: CardId) -> u32 {
    d.card(c).data.lscale
}
pub fn get_right_scale(d: &mut Duel, c: CardId) -> u32 {
    c.get_rscale(d)
}
pub fn get_original_right_scale(d: &Duel, c: CardId) -> u32 {
    d.card(c).data.rscale
}
pub fn get_link_marker(d: &mut Duel, c: CardId) -> u32 {
    c.get_link_marker(d)
}
pub fn is_link_marker(d: &mut Duel, c: CardId, dir: u32) -> bool {
    c.is_link_marker(d, dir)
}
pub fn get_linked_group(d: &mut Duel, c: CardId) -> Group {
    let mut cset = CardSet::new();
    let z = c.get_linked_zone(d, false);
    c.get_linked_cards(d, &mut cset, z);
    cset
}
pub fn get_linked_group_count(d: &mut Duel, c: CardId) -> usize {
    get_linked_group(d, c).len()
}
pub fn get_linked_zone(d: &mut Duel, c: CardId, cp: Option<u8>) -> u32 {
    let z = c.get_linked_zone(d, false);
    zone_for(d, c, z, cp)
}
pub fn get_free_linked_zone(d: &mut Duel, c: CardId, cp: Option<u8>) -> u32 {
    let z = c.get_linked_zone(d, true);
    zone_for(d, c, z, cp)
}
pub fn get_mutual_linked_group(d: &mut Duel, c: CardId) -> Group {
    let mut cset = CardSet::new();
    c.get_mutual_linked_cards(d, &mut cset);
    cset
}
pub fn get_mutual_linked_group_count(d: &mut Duel, c: CardId) -> usize {
    get_mutual_linked_group(d, c).len()
}
pub fn get_mutual_linked_zone(d: &mut Duel, c: CardId, cp: Option<u8>) -> u32 {
    let z = c.get_mutual_linked_zone(d);
    zone_for(d, c, z, cp)
}
pub fn is_linked(d: &mut Duel, c: CardId) -> bool {
    c.is_link_state(d)
}
pub fn is_extra_linked(d: &mut Duel, c: CardId) -> bool {
    c.is_extra_link_state(d)
}
pub fn get_column_group(d: &mut Duel, c: CardId, left: u8, right: u8) -> Group {
    let mut cset = CardSet::new();
    c.get_column_cards(d, &mut cset, left as i32, right as i32);
    cset
}
pub fn get_column_group_count(d: &mut Duel, c: CardId, left: u8, right: u8) -> usize {
    get_column_group(d, c, left, right).len()
}
pub fn get_column_zone(d: &mut Duel, c: CardId, loc: u32, left: u8, right: u8, cp: Option<u8>) -> u32 {
    let z = c.get_column_zone(d, loc as u16 as i32, left as i32, right as i32);
    zone_for(d, c, z, cp)
}
pub fn is_all_column(d: &mut Duel, c: CardId) -> bool {
    c.is_all_column(d)
}
pub fn get_attribute(d: &mut Duel, c: CardId) -> u32 {
    c.get_attribute(d, None, 0, PLAYER_NONE as u8)
}
pub fn get_attribute_ctx(d: &mut Duel, c: CardId, scard: Option<CardId>, sumtype: u64, playerid: Option<u8>) -> u32 {
    let p = ctx_player(d, sumtype, playerid);
    c.get_attribute(d, scard, sumtype, p)
}
pub fn get_original_attribute(d: &Duel, c: CardId) -> u32 {
    let cc = d.card(c);
    if cc.status & STATUS_NO_LEVEL != 0 { 0 } else { cc.data.attribute }
}
pub fn get_race(d: &mut Duel, c: CardId) -> u64 {
    c.get_race(d, None, 0, PLAYER_NONE as u8)
}
pub fn get_race_ctx(d: &mut Duel, c: CardId, scard: Option<CardId>, sumtype: u64, playerid: Option<u8>) -> u64 {
    let p = ctx_player(d, sumtype, playerid);
    c.get_race(d, scard, sumtype, p)
}
pub fn get_original_race(d: &Duel, c: CardId) -> u64 {
    let cc = d.card(c);
    if cc.status & STATUS_NO_LEVEL != 0 { 0 } else { cc.data.race }
}
pub fn get_attack(d: &mut Duel, c: CardId) -> i32 {
    c.get_attack(d).max(0)
}
pub fn get_base_attack(d: &mut Duel, c: CardId) -> i32 {
    c.get_base_attack(d).max(0)
}
pub fn get_text_attack(d: &Duel, c: CardId) -> i32 {
    let cc = d.card(c);
    if cc.status & STATUS_NO_LEVEL != 0 { 0 } else { cc.data.attack }
}
pub fn get_defense(d: &mut Duel, c: CardId) -> i32 {
    c.get_defense(d).max(0)
}
pub fn get_base_defense(d: &mut Duel, c: CardId) -> i32 {
    c.get_base_defense(d).max(0)
}
pub fn get_text_defense(d: &Duel, c: CardId) -> i32 {
    let cc = d.card(c);
    if cc.status & STATUS_NO_LEVEL != 0 { 0 } else { cc.data.defense }
}
pub fn get_previous_code_on_field(d: &Duel, c: CardId) -> Vec<u32> {
    d.note_read(c, 16384);
    let p = &d.card(c).previous;
    if p.code2 != 0 { vec![p.code, p.code2] } else { vec![p.code] }
}
pub fn get_previous_type_on_field(d: &Duel, c: CardId) -> u32 {
    d.note_read(c, 16384);
    d.card(c).previous.type_
}
pub fn get_previous_level_on_field(d: &Duel, c: CardId) -> u32 {
    d.note_read(c, 16384);
    d.card(c).previous.level
}
pub fn get_previous_rank_on_field(d: &Duel, c: CardId) -> u32 {
    d.note_read(c, 16384);
    d.card(c).previous.rank
}
pub fn get_previous_attribute_on_field(d: &Duel, c: CardId) -> u32 {
    d.note_read(c, 16384);
    d.card(c).previous.attribute
}
pub fn get_previous_race_on_field(d: &Duel, c: CardId) -> u64 {
    d.note_read(c, 16384);
    d.card(c).previous.race
}
pub fn get_previous_attack_on_field(d: &Duel, c: CardId) -> i32 {
    d.note_read(c, 16384);
    d.card(c).previous.attack
}
pub fn get_previous_defense_on_field(d: &Duel, c: CardId) -> i32 {
    d.note_read(c, 16384);
    d.card(c).previous.defense
}
pub fn get_owner(d: &Duel, c: CardId) -> u8 {
    d.card(c).owner
}
pub fn get_controler(d: &Duel, c: CardId) -> u8 {
    d.card(c).current.controler
}
pub fn get_previous_controler(d: &Duel, c: CardId) -> u8 {
    d.note_read(c, 16384);
    d.card(c).previous.controler
}
pub fn get_reason(d: &Duel, c: CardId) -> u32 {
    d.note_read(c, 16384);
    d.card(c).current.reason
}
pub fn get_reason_card(d: &Duel, c: CardId) -> Option<CardId> {
    d.note_read(c, 16384);
    d.card(c).current.reason_card
}
pub fn get_reason_player(d: &Duel, c: CardId) -> u8 {
    d.note_read(c, 16384);
    d.card(c).current.reason_player
}
pub fn get_reason_effect(d: &Duel, c: CardId) -> Option<EffectId> {
    d.note_read(c, 16384);
    d.card(c).current.reason_effect
}
/// `c:SetReason(reason, add)`
pub fn set_reason(d: &mut Duel, c: CardId, reason: u32, add: bool) {
    let cc = d.card_mut(c);
    if add {
        cc.current.reason |= reason;
    } else {
        cc.current.reason = reason;
    }
}
pub fn set_reason_card(d: &mut Duel, c: CardId, rcard: CardId) {
    d.card_mut(c).current.reason_card = Some(rcard);
}
pub fn set_reason_player(d: &mut Duel, c: CardId, rp: u8) {
    if rp as u32 <= PLAYER_ALL {
        d.card_mut(c).current.reason_player = rp;
    }
}
pub fn set_reason_effect(d: &mut Duel, c: CardId, re: EffectId) {
    d.card_mut(c).current.reason_effect = Some(re);
}
pub fn get_position(d: &Duel, c: CardId) -> u32 {
    d.card(c).current.position
}
pub fn get_previous_position(d: &Duel, c: CardId) -> u32 {
    d.note_read(c, 16384);
    d.card(c).previous.position
}
pub fn get_battle_position(d: &Duel, c: CardId) -> u32 {
    d.card(c).temp.position
}
pub fn get_location(d: &Duel, c: CardId) -> u32 {
    let cc = d.card(c);
    if cc.get_status(STATUS_SUMMONING | STATUS_SUMMON_DISABLED | STATUS_ACTIVATE_DISABLED | STATUS_SPSUMMON_STEP) != 0 { 0 } else { cc.current.location as u32 }
}
pub fn get_previous_location(d: &Duel, c: CardId) -> u32 {
    d.note_read(c, 16384);
    d.card(c).previous.location as u32
}
pub fn get_sequence(d: &Duel, c: CardId) -> u32 {
    d.card(c).current.sequence
}
pub fn get_previous_sequence(d: &Duel, c: CardId) -> u32 {
    d.note_read(c, 16384);
    d.card(c).previous.sequence
}
pub fn get_summon_type(d: &Duel, c: CardId) -> u32 {
    d.card(c).summon.type_ & 0xff00ffff
}
pub fn get_summon_location(d: &Duel, c: CardId) -> u32 {
    d.card(c).summon.location as u32
}
pub fn get_summon_player(d: &Duel, c: CardId) -> u8 {
    d.card(c).summon.player
}
pub fn get_destination(d: &Duel, c: CardId) -> u32 {
    d.card(c).sendto_param.location as u32
}
pub fn get_leave_field_dest(d: &mut Duel, c: CardId) -> u32 {
    c.leave_field_redirect(d, REASON_EFFECT)
}
pub fn get_turn_id(d: &Duel, c: CardId) -> u32 {
    d.note_read(c, 16384);
    d.card(c).turnid as u32
}
pub fn get_field_id(d: &Duel, c: CardId) -> u32 {
    d.note_read(c, 16384);
    d.card(c).fieldid
}
pub fn get_real_field_id(d: &Duel, c: CardId) -> u32 {
    d.card(c).fieldid_r
}
pub fn get_card_id(d: &Duel, c: CardId) -> u32 {
    d.card(c).cardid
}
pub fn is_original_code_rule(d: &mut Duel, c: CardId, codes: &[u32]) -> bool {
    let mut eset = Vec::new();
    c.filter_effect(d, EFFECT_ADD_CODE, &mut eset, true);
    let data = d.card(c).data;
    let (code1, code2) = if data.alias != 0 && eset.is_empty() {
        (data.alias, 0)
    } else {
        let c2 = match eset.last() {
            Some(&e) => e.get_value_card(d, Some(c), 0) as u32,
            None => 0,
        };
        (data.code, c2)
    };
    codes.iter().any(|&t| code1 == t || (code2 != 0 && code2 == t))
}
pub fn is_original_code(d: &Duel, c: CardId, codes: &[u32]) -> bool {
    let oc = get_original_code(d, c);
    codes.contains(&oc)
}
pub fn is_code(d: &mut Duel, c: CardId, codes: &[u32]) -> bool {
    let code1 = c.get_code(d);
    let code2 = c.get_another_code(d);
    codes.iter().any(|&t| code1 == t || (code2 != 0 && code2 == t))
}
pub fn is_summon_code(d: &mut Duel, c: CardId, scard: Option<CardId>, sumtype: u64, playerid: u8, codes: &[u32]) -> bool {
    let mut eset = Vec::new();
    c.filter_effect(d, EFFECT_ADD_CODE, &mut eset, false);
    c.filter_effect(d, EFFECT_REMOVE_CODE, &mut eset, false);
    c.filter_effect(d, EFFECT_CHANGE_CODE, &mut eset, true);
    let mut set = BTreeSet::new();
    set.insert(c.get_code(d));
    let code2 = c.get_another_code(d);
    if code2 != 0 {
        set.insert(code2);
    }
    for peff in eset {
        let Some(op) = d.effect(peff).operation.clone() else { continue };
        d.add_param(Value::card(scard));
        d.add_param(Value::Int(sumtype as i64));
        d.add_param(Value::Int(playerid as i64));
        if !d.check_condition(Some(&op), 3) {
            continue;
        }
        let v = peff.get_value_card(d, Some(c), 0) as u32;
        match d.effect(peff).code {
            EFFECT_ADD_CODE => {
                set.insert(v);
            }
            EFFECT_REMOVE_CODE => {
                set.remove(&v);
            }
            _ => {
                set.clear();
                set.insert(v);
            }
        }
    }
    codes.iter().any(|t| set.contains(t))
}
fn contains_setcode(setcodes: &BTreeSet<u16>, sc: u16) -> bool {
    setcodes.iter().any(|&s| crate::card::Card::match_setcode(sc, s))
}
/// `c:IsSetCard(setcode)` (a table of set codes is `any`)
pub fn is_set_card(d: &mut Duel, c: CardId, setcodes: &[u16]) -> bool {
    let mut s = BTreeSet::new();
    c.get_set_card(d, &mut s);
    setcodes.iter().any(|&sc| contains_setcode(&s, sc))
}
pub fn is_set_card_ctx(d: &mut Duel, c: CardId, setcodes: &[u16], scard: Option<CardId>, sumtype: u64, playerid: Option<u8>) -> bool {
    let mut s = BTreeSet::new();
    c.get_summon_set_card(d, &mut s, scard, sumtype, playerid.unwrap_or(PLAYER_NONE as u8));
    setcodes.iter().any(|&sc| contains_setcode(&s, sc))
}
pub fn is_original_set_card(d: &Duel, c: CardId, setcodes: &[u16]) -> bool {
    let s: BTreeSet<u16> = c.get_origin_set_card(d).iter().collect();
    setcodes.iter().any(|&sc| contains_setcode(&s, sc))
}
pub fn is_previous_set_card(d: &Duel, c: CardId, setcodes: &[u16]) -> bool {
    d.note_read(c, 16384);
    let mut s = BTreeSet::new();
    c.get_pre_set_card(d, &mut s);
    setcodes.iter().any(|&sc| contains_setcode(&s, sc))
}
pub fn is_type(d: &mut Duel, c: CardId, ttype: u32) -> bool {
    c.get_type(d, None, 0, PLAYER_NONE as u8) & ttype != 0
}
pub fn is_type_ctx(d: &mut Duel, c: CardId, ttype: u32, scard: Option<CardId>, sumtype: u64, playerid: Option<u8>) -> bool {
    let p = ctx_player(d, sumtype, playerid);
    c.get_type(d, scard, sumtype, p) & ttype != 0
}
pub fn is_exact_type(d: &mut Duel, c: CardId, ttype: u32) -> bool {
    c.get_type(d, None, 0, PLAYER_NONE as u8) & ttype == ttype
}
pub fn is_exact_type_ctx(d: &mut Duel, c: CardId, ttype: u32, scard: Option<CardId>, sumtype: u64, playerid: Option<u8>) -> bool {
    let p = ctx_player(d, sumtype, playerid);
    c.get_type(d, scard, sumtype, p) & ttype == ttype
}
pub fn is_original_type(d: &Duel, c: CardId, ttype: u32) -> bool {
    d.card(c).data.type_ & ttype != 0
}
pub fn is_level(d: &mut Duel, c: CardId, vals: &[u32]) -> bool {
    let v = c.get_level(d);
    vals.contains(&v)
}
pub fn is_rank(d: &mut Duel, c: CardId, vals: &[u32]) -> bool {
    let v = c.get_rank(d);
    vals.contains(&v)
}
pub fn is_link(d: &mut Duel, c: CardId, vals: &[u32]) -> bool {
    let v = c.get_link(d);
    vals.contains(&v)
}
fn is_monster_now(d: &mut Duel, c: CardId) -> bool {
    d.card(c).data.type_ & TYPE_MONSTER != 0 || c.get_type(d, None, 0, PLAYER_NONE as u8) & TYPE_MONSTER != 0 || c.is_affected_by_effect(d, EFFECT_PRE_MONSTER).is_some()
}
pub fn is_attack(d: &mut Duel, c: CardId, vals: &[u32]) -> bool {
    if !is_monster_now(d, c) {
        return false;
    }
    let v = c.get_attack(d) as u32;
    vals.contains(&v)
}
pub fn is_defense(d: &mut Duel, c: CardId, vals: &[u32]) -> bool {
    if d.card(c).data.type_ & TYPE_LINK != 0 || !is_monster_now(d, c) {
        return false;
    }
    let v = c.get_defense(d) as u32;
    vals.contains(&v)
}
pub fn is_race(d: &mut Duel, c: CardId, trace: u64) -> bool {
    c.get_race(d, None, 0, PLAYER_NONE as u8) & trace != 0
}
pub fn is_race_ctx(d: &mut Duel, c: CardId, trace: u64, scard: Option<CardId>, sumtype: u64, playerid: Option<u8>) -> bool {
    let p = ctx_player(d, sumtype, playerid);
    c.get_race(d, scard, sumtype, p) & trace != 0
}
pub fn is_original_race(d: &Duel, c: CardId, trace: u64) -> bool {
    let cc = d.card(c);
    cc.status & STATUS_NO_LEVEL == 0 && cc.data.race & trace != 0
}
pub fn is_attribute(d: &mut Duel, c: CardId, tattrib: u32) -> bool {
    c.get_attribute(d, None, 0, PLAYER_NONE as u8) & tattrib != 0
}
pub fn is_attribute_ctx(d: &mut Duel, c: CardId, tattrib: u32, scard: Option<CardId>, sumtype: u64, playerid: Option<u8>) -> bool {
    let p = ctx_player(d, sumtype, playerid);
    c.get_attribute(d, scard, sumtype, p) & tattrib != 0
}
pub fn is_original_attribute(d: &Duel, c: CardId, tattrib: u32) -> bool {
    let cc = d.card(c);
    cc.status & STATUS_NO_LEVEL == 0 && cc.data.attribute & tattrib != 0
}
pub fn is_reason(d: &Duel, c: CardId, treason: u32) -> bool {
    d.note_read(c, 16384);
    d.card(c).current.reason & treason != 0
}
/// `c:IsSummonType(t1, t2, ...)`: any of the types matches completely.
pub fn is_summon_type(d: &Duel, c: CardId, types: &[u32]) -> bool {
    let info = d.card(c).summon.type_ & 0xff00ffff;
    types.iter().any(|&t| info & t == t)
}
pub fn is_summon_location(d: &Duel, c: CardId, loc: u32) -> bool {
    let s = &d.card(c).summon;
    crate::card::LocState { location: s.location, sequence: s.sequence as u32, pzone: s.pzone }.is_location(loc as u16 as u32)
}
pub fn is_summon_player(d: &Duel, c: CardId, player: u8) -> bool {
    d.card(c).summon.player == player
}
pub fn is_status(d: &Duel, c: CardId, tstatus: u32) -> bool {
    d.note_read(c, 16384);
    d.card(c).status & tstatus != 0
}
pub fn is_not_tuner(d: &mut Duel, c: CardId, scard: CardId, playerid: u8) -> bool {
    c.is_not_tuner(d, Some(scard), playerid)
}
pub fn set_status(d: &mut Duel, c: CardId, tstatus: u32, enable: bool) {
    if d.card(c).status & STATUS_COPYING_EFFECT != 0 {
        return;
    }
    d.card_mut(c).set_status(tstatus, enable);
}
pub fn is_gemini_state(d: &mut Duel, c: CardId) -> bool {
    c.is_affected_by_effect(d, EFFECT_GEMINI_STATUS).is_some()
}
pub fn enable_gemini_state(d: &mut Duel, c: CardId) {
    let de = d.new_effect();
    let e = d.effect_mut(de);
    e.owner = Some(c);
    e.code = EFFECT_GEMINI_STATUS;
    e.type_ = EFFECT_TYPE_SINGLE as u16;
    e.flag[0] = EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_CLIENT_HINT;
    e.description = 64;
    e.reset_flag = RESET_EVENT + 0x1fe0000;
    c.add_effect(d, de);
}
pub fn set_turn_counter(d: &mut Duel, c: CardId, ct: u16) {
    c.count_turn(d, ct);
}
pub fn get_turn_counter(d: &Duel, c: CardId) -> u32 {
    d.note_read(c, 16384);
    d.card(c).turn_counter as u32
}
/// `c:SetMaterial(c_or_g_or_nil)`
pub fn set_material(d: &mut Duel, c: CardId, mats: CardSet) {
    c.set_material(d, mats);
}
pub fn get_material(d: &Duel, c: CardId) -> Group {
    d.card(c).material_cards.clone()
}
pub fn get_material_count(d: &Duel, c: CardId) -> usize {
    d.card(c).material_cards.len()
}
pub fn get_equip_group(d: &Duel, c: CardId) -> Group {
    d.card(c).equiping_cards.clone()
}
pub fn get_equip_count(d: &Duel, c: CardId) -> usize {
    d.card(c).equiping_cards.len()
}
pub fn get_equip_target(d: &Duel, c: CardId) -> Option<CardId> {
    d.card(c).equiping_target()
}
pub fn get_previous_equip_target(d: &Duel, c: CardId) -> Option<CardId> {
    d.note_read(c, 16384);
    d.card(c).pre_equip_target
}
fn check_union_like(d: &mut Duel, c: CardId, target: CardId, code: u32) -> bool {
    c.is_affected_by_effect_t(d, code, Some(target)).is_some()
        && ((c.is_affected_by_effect(d, EFFECT_OLDUNION_STATUS).is_none() || target.get_union_count(d) == 0)
            && (c.is_affected_by_effect(d, EFFECT_UNION_STATUS).is_none() || target.get_old_union_count(d) == 0))
}
pub fn check_equip_target(d: &mut Duel, c: CardId, target: CardId) -> bool {
    check_union_like(d, c, target, EFFECT_EQUIP_LIMIT)
}
pub fn check_union_target(d: &mut Duel, c: CardId, target: CardId) -> bool {
    check_union_like(d, c, target, EFFECT_UNION_LIMIT)
}
pub fn get_union_count(d: &mut Duel, c: CardId) -> (i32, i32) {
    (c.get_union_count(d), c.get_old_union_count(d))
}
pub fn get_overlay_group(d: &Duel, c: CardId) -> Group {
    CardSet::from_iter(d.card(c).xyz_materials.iter().copied())
}
pub fn get_overlay_count(d: &Duel, c: CardId) -> usize {
    d.card(c).xyz_materials.len()
}
pub fn get_overlay_target(d: &Duel, c: CardId) -> Option<CardId> {
    d.card(c).overlay_target()
}
pub fn check_remove_overlay_card(d: &mut Duel, c: CardId, playerid: u8, count: u16, reason: u32) -> bool {
    if playerid > 1 {
        return false;
    }
    let g = CardSet::from_card(c);
    d.is_player_can_remove_overlay_card(playerid, Some(&g), 0, 0, count, reason)
}
/// `c:RemoveOverlayCard(player, min, max, reason)`: the number of cards detached.
pub fn remove_overlay_card(d: &mut Duel, c: CardId, playerid: u8, min: u16, max: u16, reason: u32) -> i32 {
    crate::lua::check_action_permission(d);
    if playerid > 1 {
        return 0;
    }
    let g = CardSet::from_card(c);
    d.remove_overlay_card(reason, Some(&g), playerid as u32, 0, 0, min, max);
    d.run_pushed_units();
    d.returns.at_i32(0)
}
pub fn get_attacked_group(d: &Duel, c: CardId) -> Group {
    CardSet::from_iter(d.card(c).attacked_cards.items.iter().filter_map(|x| x.1))
}
pub fn get_attacked_group_count(d: &Duel, c: CardId) -> usize {
    d.card(c).attacked_cards.len()
}
pub fn get_attacked_count(d: &Duel, c: CardId) -> u32 {
    d.card(c).attacked_count as u32
}
pub fn get_battled_group(d: &Duel, c: CardId) -> Group {
    CardSet::from_iter(d.card(c).battled_cards.items.iter().filter_map(|x| x.1))
}
pub fn get_battled_group_count(d: &Duel, c: CardId) -> usize {
    d.card(c).battled_cards.len()
}
pub fn get_attack_announced_count(d: &Duel, c: CardId) -> u32 {
    d.card(c).attack_announce_count as u32
}
pub fn is_direct_attacked(d: &Duel, c: CardId) -> bool {
    d.card(c).attacked_cards.findcard(0) != 0
}
pub fn set_card_target(d: &mut Duel, c: CardId, ocard: CardId) {
    c.add_card_target(d, ocard);
}
pub fn get_card_target(d: &Duel, c: CardId) -> Group {
    d.card(c).effect_target_cards().clone()
}
pub fn get_first_card_target(d: &Duel, c: CardId) -> Option<CardId> {
    let t = &d.card(c).effect_target_cards();
    if t.is_empty() { None } else { crate::card::public_order(d, t).first().copied() }
}
pub fn get_card_target_count(d: &Duel, c: CardId) -> usize {
    d.card(c).effect_target_cards().len()
}
pub fn is_has_card_target(d: &Duel, c: CardId, rcard: CardId) -> bool {
    d.card(c).effect_target_cards().contains(rcard)
}
pub fn cancel_card_target(d: &mut Duel, c: CardId, rcard: CardId) {
    c.cancel_card_target(d, rcard);
}
pub fn get_owner_target(d: &Duel, c: CardId) -> Group {
    d.card(c).effect_target_owner.clone()
}
pub fn get_owner_target_count(d: &Duel, c: CardId) -> usize {
    d.card(c).effect_target_owner.len()
}
pub fn get_activate_effect(d: &Duel, c: CardId) -> Vec<EffectId> {
    d.card(c).field_effect().all().into_iter().filter(|&e| d.effect(e).type_ as u32 & EFFECT_TYPE_ACTIVATE != 0).collect()
}
/// `c:CheckActivateEffect(neglect_con, neglect_cost, copy_info)`: the effect, and with
/// `copy_info` (unless it is a free chain effect) the event it would respond to.
pub fn check_activate_effect(d: &mut Duel, c: CardId, neglect_con: bool, neglect_cost: bool, copy_info: bool) -> Option<(EffectId, Option<crate::duel::Event>)> {
    for peffect in d.card(c).field_effect().all() {
        if d.effect(peffect).type_ as u32 & EFFECT_TYPE_ACTIVATE == 0 {
            continue;
        }
        let mut pe = crate::duel::Event::default();
        let rp = d.core.reason_player;
        if d.check_event_c(peffect, rp, neglect_con, neglect_cost, copy_info, Some(&mut pe)) {
            if !copy_info || d.effect(peffect).code == EVENT_FREE_CHAIN {
                return Some((peffect, None));
            }
            return Some((peffect, Some(pe)));
        }
    }
    None
}
/// `c:RegisterEffect(e, forced)`: the effect's index on the card (-1), or `None` (Lua: nothing).
pub fn register_effect(d: &mut Duel, c: CardId, e: EffectId, forced: bool) -> Option<i32> {
    if d.effect(e).owner == Some(d.temp_card) {
        return None;
    }
    if !forced {
        if let Some(re) = d.core.reason_effect {
            if !c.is_affect_by_effect(d, Some(re)) {
                if !d.core.reseted_effects.contains(&e) {
                    d.core.reseted_effects.push(e);
                }
                return None;
            }
        }
    }
    let mut id = -1;
    if d.effect(e).handler.is_none() {
        id = c.add_effect(d, e) as i32;
    }
    Some(id)
}
/// `c:IsHasEffect(code, check_player)`: the effects (empty: Lua nil).
pub fn is_has_effect(d: &mut Duel, c: CardId, code: u32, check_player: Option<u8>) -> Vec<EffectId> {
    d.note_read(c, 16384);
    let mut eset = Vec::new();
    c.filter_effect(d, code, &mut eset, true);
    let p = check_player.unwrap_or(PLAYER_NONE as u8);
    let mut out = Vec::new();
    for e in eset {
        if p as u32 == PLAYER_NONE || e.check_count_limit(d, p) {
            out.push(e);
        }
    }
    out
}
/// `c:GetCardEffect(code)` (empty: Lua nil)
pub fn get_card_effect(d: &mut Duel, c: CardId, code: u32) -> Vec<EffectId> {
    let mut eset = Vec::new();
    c.get_card_effect(d, code, &mut eset);
    eset
}
pub fn get_own_effects(d: &Duel, c: CardId) -> Vec<EffectId> {
    let mut eset = Vec::new();
    c.get_own_effects(d, &mut eset);
    eset
}
pub fn reset_effect(d: &mut Duel, c: CardId, code: u32, typ: u32) {
    c.reset(d, code, typ);
}
pub fn get_effect_count(d: &mut Duel, c: CardId, code: u32) -> usize {
    let mut eset = Vec::new();
    c.filter_effect(d, code, &mut eset, true);
    eset.len()
}
fn flag_code(code: u32) -> u32 {
    (code & 0xfffffff) | 0x10000000
}
/// `c:RegisterFlagEffect(code, reset, flag, count, label, desc)`
#[allow(clippy::too_many_arguments)]
pub fn register_flag_effect(d: &mut Duel, c: CardId, code: u32, reset: u32, flag: u32, count: u16, label: i64, desc: u64) -> EffectId {
    let code = flag_code(code);
    let count = if count == 0 { 1 } else { count };
    let mut reset = reset;
    if reset & RESET_PHASE != 0 && reset & (RESET_SELF_TURN | RESET_OPPO_TURN) == 0 {
        reset |= RESET_SELF_TURN | RESET_OPPO_TURN;
    }
    let pe = d.new_effect();
    let e = d.effect_mut(pe);
    e.owner = Some(c);
    e.handler = None;
    e.type_ = EFFECT_TYPE_SINGLE as u16;
    e.code = code;
    e.reset_flag = reset;
    e.flag[0] = flag | EFFECT_FLAG_CANNOT_DISABLE;
    e.reset_count = count;
    e.label = vec![label];
    e.description = desc;
    c.add_effect(d, pe);
    pe
}
pub fn get_flag_effect(d: &Duel, c: CardId, code: u32) -> usize {
    d.note_read(c, 1024);
    d.card(c).single_effect().range(flag_code(code)).len()
}
pub fn reset_flag_effect(d: &mut Duel, c: CardId, code: u32) {
    c.reset(d, flag_code(code), RESET_CODE);
}
pub fn set_flag_effect_label(d: &mut Duel, c: CardId, code: u32, lab: u32) -> bool {
    match d.card(c).single_effect().range(flag_code(code)).first() {
        None => false,
        Some(&e) => {
            d.effect_mut(e).label = vec![lab as i64];
            true
        }
    }
}
/// `c:GetFlagEffectLabel(code)`: one label per flag effect (empty: Lua nil).
pub fn get_flag_effect_label(d: &Duel, c: CardId, code: u32) -> Vec<i64> {
    d.note_read(c, 1024);
    d.card(c).single_effect().range(flag_code(code)).into_iter().map(|e| d.effect(e).label.first().copied().unwrap_or(0)).collect()
}
pub fn create_relation(d: &mut Duel, c: CardId, rcard: CardId, reset: u32) {
    c.create_relation(d, rcard, reset);
}
pub fn release_relation(d: &mut Duel, c: CardId, rcard: CardId) {
    c.release_relation(d, rcard);
}
pub fn create_effect_relation(d: &mut Duel, c: CardId, e: EffectId) {
    c.create_relation_effect(d, e);
}
pub fn release_effect_relation(d: &mut Duel, c: CardId, e: EffectId) {
    c.release_relation_effect(d, e);
}
pub fn clear_effect_relation(d: &mut Duel, c: CardId) {
    c.clear_relate_effect(d);
}
pub fn is_relate_to_effect(d: &Duel, c: CardId, e: EffectId) -> bool {
    d.note_read(c, 16384);
    c.is_has_relation_effect(d, e)
}
pub fn is_relate_to_chain(d: &Duel, c: CardId, chain_count: u8) -> bool {
    d.note_read(c, 16384);
    let n = d.core.current_chain.len();
    let ct = if chain_count as usize > n || chain_count < 1 { n } else { chain_count as usize };
    let ch = &d.core.current_chain[ct - 1];
    c.is_has_relation_chain(d, ch.triggering_effect.unwrap(), ch.chain_id)
}
pub fn is_relate_to_card(d: &Duel, c: CardId, rcard: CardId) -> bool {
    c.is_has_relation(d, rcard)
}
pub fn is_relate_to_battle(d: &Duel, c: CardId) -> bool {
    let f = d.card(c).fieldid_r;
    f == d.core.pre_field[0] || f == d.core.pre_field[1]
}
fn turn_reset(reset: u32) -> u32 {
    if reset & RESET_PHASE != 0 && reset & (RESET_SELF_TURN | RESET_OPPO_TURN) == 0 { reset | RESET_SELF_TURN | RESET_OPPO_TURN } else { reset }
}
/// `c:CopyEffect(code, reset, count = 1)`
pub fn copy_effect(d: &mut Duel, c: CardId, code: u32, reset: u32, count: u8) -> i32 {
    let count = if count == 0 { 1 } else { count };
    c.copy_effect(d, code, turn_reset(reset), count as u32)
}
/// `c:ReplaceEffect(code, reset, count = 1)`
pub fn replace_effect(d: &mut Duel, c: CardId, code: u32, reset: u32, count: u8) -> i32 {
    let count = if count == 0 { 1 } else { count };
    c.replace_effect(d, code, turn_reset(reset), count as u32, false)
}
fn add_rule_effect(d: &mut Duel, c: CardId, code: u32) {
    let pe = d.new_effect();
    let e = d.effect_mut(pe);
    e.owner = Some(c);
    e.code = code;
    e.type_ = EFFECT_TYPE_SINGLE as u16;
    e.flag[0] = EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_UNCOPYABLE;
    c.add_effect(d, pe);
}
pub fn enable_unsummonable(d: &mut Duel, c: CardId) {
    if !d.card(c).is_status(STATUS_COPYING_EFFECT) {
        add_rule_effect(d, c, EFFECT_UNSUMMONABLE_CARD);
    }
}
pub fn enable_revive_limit(d: &mut Duel, c: CardId) {
    if !d.card(c).is_status(STATUS_COPYING_EFFECT) {
        add_rule_effect(d, c, EFFECT_UNSUMMONABLE_CARD);
        add_rule_effect(d, c, EFFECT_REVIVE_LIMIT);
    }
}
pub fn complete_procedure(d: &mut Duel, c: CardId) {
    d.card_mut(c).set_status(STATUS_PROC_COMPLETE, true);
}
pub fn is_disabled(d: &Duel, c: CardId) -> bool {
    d.card(c).is_status(STATUS_DISABLED)
}
/// `c:IsDestructable(e)`
pub fn is_destructable(d: &mut Duel, c: CardId, e: Option<EffectId>) -> bool {
    match e {
        Some(e) => {
            let rp = d.core.reason_player;
            c.is_destructable_by_effect(d, Some(e), rp)
        }
        None => c.is_destructable(d),
    }
}
pub fn is_summonable_card(d: &mut Duel, c: CardId) -> bool {
    c.is_summonable_card(d)
}
pub fn is_special_summonable(d: &mut Duel, c: CardId, sumtype: u32) -> bool {
    let rp = d.core.reason_player;
    c.is_special_summonable(d, rp, sumtype) != 0
}
pub fn is_fusion_summonable_card(d: &mut Duel, c: CardId, summon_type: u32) -> bool {
    c.is_fusion_summonable_card(d, summon_type)
}
/// `spsummonable_rule`: whether `c` can be Special Summoned by its procedure of `sumtype`
/// (with `must`/`materials` as the forced materials).
#[allow(clippy::too_many_arguments)]
fn spsummonable_rule(d: &mut Duel, c: CardId, cardtype: u32, sumtype: u32, must: Option<CardSet>, materials: Option<CardSet>, minc: u16, maxc: u16) -> bool {
    if d.card(c).data.type_ & cardtype == 0 {
        return false;
    }
    let p = d.core.reason_player;
    d.core.must_use_mats = must;
    d.core.only_use_mats = materials;
    d.core.forced_summon_minc = minc as i32;
    d.core.forced_summon_maxc = maxc as i32;
    let res = c.is_special_summonable(d, p, sumtype) != 0;
    d.core.must_use_mats = None;
    d.core.only_use_mats = None;
    d.core.forced_summon_minc = 0;
    d.core.forced_summon_maxc = 0;
    res
}
pub fn is_synchro_summonable(d: &mut Duel, c: CardId, must: Option<CardSet>, materials: Option<CardSet>, minc: u16, maxc: u16) -> bool {
    spsummonable_rule(d, c, TYPE_SYNCHRO, SUMMON_TYPE_SYNCHRO, must, materials, minc, maxc)
}
pub fn is_xyz_summonable(d: &mut Duel, c: CardId, must: Option<CardSet>, materials: Option<CardSet>, minc: u16, maxc: u16) -> bool {
    spsummonable_rule(d, c, TYPE_XYZ, SUMMON_TYPE_XYZ, must, materials, minc, maxc)
}
pub fn is_link_summonable(d: &mut Duel, c: CardId, must: Option<CardSet>, materials: Option<CardSet>, minc: u16, maxc: u16) -> bool {
    spsummonable_rule(d, c, TYPE_LINK, SUMMON_TYPE_LINK, must, materials, minc, maxc)
}
#[allow(clippy::too_many_arguments)]
pub fn is_procedure_summonable(d: &mut Duel, c: CardId, cardtype: u32, sumtype: u32, must: Option<CardSet>, materials: Option<CardSet>, minc: u16, maxc: u16) -> bool {
    spsummonable_rule(d, c, cardtype, sumtype, must, materials, minc, maxc)
}
/// `c:IsSummonable(ignore_count, e, minc = 0, zone = 0x1f)`
pub fn is_summonable(d: &mut Duel, c: CardId, ign: bool, e: Option<EffectId>, minc: u16, zone: Option<u16>) -> bool {
    let rp = d.core.reason_player;
    c.is_can_be_summoned(d, rp, ign, e, minc as u8, zone.unwrap_or(0x1f) as u32)
}
/// `c:IsMSetable(ignore_count, e, minc = 0, zone = 0xff)`
pub fn is_m_setable(d: &mut Duel, c: CardId, ign: bool, e: Option<EffectId>, minc: u16, zone: Option<u32>) -> bool {
    let rp = d.core.reason_player;
    c.is_setable_mzone(d, rp, ign, e, minc as u8, zone.unwrap_or(0xff))
}
/// `c:IsSSetable(ignore_field, toplayer)`
pub fn is_s_setable(d: &mut Duel, c: CardId, ign: bool, toplayer: Option<u8>) -> bool {
    let tp = toplayer.unwrap_or(d.core.reason_player);
    c.is_setable_szone(d, tp, ign)
}
/// `c:IsCanBeSpecialSummoned(e, sumtype, sumplayer, nocheck, nolimit, sumpos = POS_FACEUP, toplayer = sumplayer, zone = 0xff)`
#[allow(clippy::too_many_arguments)]
pub fn is_can_be_special_summoned(d: &mut Duel, c: CardId, e: EffectId, sumtype: u32, sumplayer: u8, nocheck: bool, nolimit: bool, sumpos: Option<u8>, toplayer: Option<u8>, zone: Option<u32>) -> bool {
    let sumpos = sumpos.unwrap_or(POS_FACEUP as u8);
    let toplayer = toplayer.unwrap_or(sumplayer);
    c.is_can_be_special_summoned(d, e, sumtype, sumpos, sumplayer, toplayer, nocheck, nolimit, zone.unwrap_or(0xff))
}
pub fn is_able_to_hand(d: &mut Duel, c: CardId, p: Option<u8>) -> bool {
    let p = p.unwrap_or(d.core.reason_player);
    c.is_capable_send_to_hand(d, p)
}
pub fn is_able_to_deck(d: &mut Duel, c: CardId) -> bool {
    let p = d.core.reason_player;
    c.is_capable_send_to_deck(d, p)
}
pub fn is_able_to_extra(d: &mut Duel, c: CardId) -> bool {
    let p = d.core.reason_player;
    c.is_capable_send_to_extra(d, p)
}
/// `c:IsAbleToGrave(reason = REASON_EFFECT)`
pub fn is_able_to_grave(d: &mut Duel, c: CardId, reason: Option<u32>) -> bool {
    let p = d.core.reason_player;
    c.is_capable_send_to_grave(d, p, reason.unwrap_or(REASON_EFFECT))
}
/// `c:IsAbleToRemove(player, pos = POS_FACEUP, reason = REASON_EFFECT)`
pub fn is_able_to_remove(d: &mut Duel, c: CardId, p: Option<u8>, pos: Option<u8>, reason: Option<u32>) -> bool {
    let p = p.unwrap_or(d.core.reason_player);
    c.is_removeable(d, p, pos.unwrap_or(POS_FACEUP as u8), reason.unwrap_or(REASON_EFFECT))
}
pub fn is_able_to_hand_as_cost(d: &mut Duel, c: CardId) -> bool {
    let p = d.core.reason_player;
    c.is_capable_cost_to_hand(d, p)
}
pub fn is_able_to_deck_as_cost(d: &mut Duel, c: CardId) -> bool {
    let p = d.core.reason_player;
    c.is_capable_cost_to_deck(d, p)
}
pub fn is_able_to_extra_as_cost(d: &mut Duel, c: CardId) -> bool {
    let p = d.core.reason_player;
    c.is_capable_cost_to_extra(d, p)
}
pub fn is_able_to_deck_or_extra_as_cost(d: &mut Duel, c: CardId) -> bool {
    let p = d.core.reason_player;
    if c.is_extra_deck_monster(d) { c.is_capable_cost_to_extra(d, p) } else { c.is_capable_cost_to_deck(d, p) }
}
pub fn is_able_to_grave_as_cost(d: &mut Duel, c: CardId) -> bool {
    let p = d.core.reason_player;
    c.is_capable_cost_to_grave(d, p)
}
/// `c:IsAbleToRemoveAsCost(pos = POS_FACEUP)`
pub fn is_able_to_remove_as_cost(d: &mut Duel, c: CardId, pos: Option<u8>) -> bool {
    let p = d.core.reason_player;
    c.is_removeable_as_cost(d, p, pos.unwrap_or(POS_FACEUP as u8))
}
/// `c:IsReleasable(reason = REASON_COST, player)`
pub fn is_releasable(d: &mut Duel, c: CardId, reason: Option<u32>, player: Option<u8>) -> bool {
    let p = player.unwrap_or(d.core.reason_player);
    c.is_releasable_by_nonsummon(d, p, reason.unwrap_or(REASON_COST))
}
/// `c:IsReleasableByEffect(e, player)`
pub fn is_releasable_by_effect(d: &mut Duel, c: CardId, e: Option<EffectId>, player: Option<u8>) -> bool {
    let re = e.or(d.core.reason_effect);
    let p = player.unwrap_or(d.core.reason_player);
    c.is_releasable_by_nonsummon(d, p, REASON_EFFECT) && c.is_releasable_by_effect(d, p, re)
}
/// `c:IsDiscardable(reason = REASON_COST, player)`
pub fn is_discardable(d: &mut Duel, c: CardId, reason: Option<u32>, player: Option<u8>) -> bool {
    let pe = d.core.reason_effect;
    let reason = reason.unwrap_or(REASON_COST);
    let p = player.unwrap_or(d.core.reason_player);
    (reason != REASON_COST || c.is_affected_by_effect(d, EFFECT_CANNOT_USE_AS_COST).is_none()) && d.is_player_can_discard_hand(p, c, pe, reason)
}
pub fn can_attack(d: &mut Duel, c: CardId) -> bool {
    c.is_capable_attack(d)
}
/// `c:CanChainAttack(ac = 2, monsteronly = false)`
pub fn can_chain_attack(d: &mut Duel, c: CardId, ac: Option<u8>, monsteronly: bool) -> bool {
    if Some(c) != d.core.attacker {
        return false;
    }
    let ac = ac.unwrap_or(2);
    let tp = d.infos.turn_player;
    let cc = d.card(c);
    if cc.is_status(STATUS_BATTLE_DESTROYED) || cc.current.controler != tp || cc.fieldid_r != d.core.pre_field[0] {
        return false;
    }
    let announce_count = cc.announce_count;
    if !c.is_capable_attack_announce(d, tp) || (ac != 0 && announce_count >= ac) || (ac == 2 && c.is_affected_by_effect(d, EFFECT_EXTRA_ATTACK).is_some()) {
        return false;
    }
    let mut v = Vec::new();
    d.get_attack_target(c, &mut v, true, true, None);
    d.core.select_cards = v;
    !(d.core.select_cards.is_empty() && (monsteronly || d.card(c).direct_attackable == 0))
}
pub fn is_faceup(d: &Duel, c: CardId) -> bool {
    d.card(c).is_position(POS_FACEUP) != 0
}
pub fn is_attack_pos(d: &Duel, c: CardId) -> bool {
    d.card(c).is_position(POS_ATTACK) != 0
}
pub fn is_facedown(d: &Duel, c: CardId) -> bool {
    d.card(c).is_position(POS_FACEDOWN) != 0
}
pub fn is_defense_pos(d: &Duel, c: CardId) -> bool {
    d.card(c).is_position(POS_DEFENSE) != 0
}
pub fn is_position(d: &Duel, c: CardId, pos: u32) -> bool {
    d.card(c).is_position(pos & 0xff) != 0
}
pub fn is_previous_position(d: &Duel, c: CardId, pos: u32) -> bool {
    d.note_read(c, 16384);
    d.card(c).previous.position & (pos & 0xff) != 0
}
pub fn is_controler(d: &Duel, c: CardId, con: u8) -> bool {
    d.card(c).current.controler == con
}
pub fn is_previous_controler(d: &Duel, c: CardId, con: u8) -> bool {
    d.note_read(c, 16384);
    d.card(c).previous.controler == con
}
pub fn is_on_field(d: &Duel, c: CardId) -> bool {
    let cc = d.card(c);
    cc.current.location as u32 & LOCATION_ONFIELD != 0 && cc.get_status(STATUS_SUMMONING | STATUS_SUMMON_DISABLED | STATUS_ACTIVATE_DISABLED | STATUS_SPSUMMON_STEP) == 0
}
pub fn is_location(d: &Duel, c: CardId, loc: u32) -> bool {
    let loc = loc & 0xffff;
    let cc = d.card(c);
    match cc.current.location as u32 {
        LOCATION_MZONE => cc.current.is_location(loc) && cc.get_status(STATUS_SUMMONING | STATUS_SUMMON_DISABLED | STATUS_SPSUMMON_STEP) == 0,
        LOCATION_SZONE => cc.current.is_location(loc) && !cc.is_status(STATUS_ACTIVATE_DISABLED),
        l => l & loc != 0,
    }
}
pub fn is_previous_location(d: &Duel, c: CardId, loc: u32) -> bool {
    d.note_read(c, 16384);
    d.card(c).previous.is_location(loc & 0xffff)
}
pub fn is_level_below(d: &mut Duel, c: CardId, lvl: u32) -> bool {
    let p = c.get_level(d);
    p > 0 && p <= lvl
}
pub fn is_level_above(d: &mut Duel, c: CardId, lvl: u32) -> bool {
    let p = c.get_level(d);
    p > 0 && p >= lvl
}
pub fn is_rank_below(d: &mut Duel, c: CardId, rnk: u32) -> bool {
    let p = c.get_rank(d);
    p > 0 && p <= rnk
}
pub fn is_rank_above(d: &mut Duel, c: CardId, rnk: u32) -> bool {
    let p = c.get_rank(d);
    p > 0 && p >= rnk
}
pub fn is_link_below(d: &mut Duel, c: CardId, lnk: u32) -> bool {
    let p = c.get_link(d);
    p > 0 && p <= lnk
}
pub fn is_link_above(d: &mut Duel, c: CardId, lnk: u32) -> bool {
    let p = c.get_link(d);
    p > 0 && p >= lnk
}
pub fn is_attack_below(d: &mut Duel, c: CardId, atk: i32) -> bool {
    if !is_monster_now(d, c) {
        return false;
    }
    let a = c.get_attack(d);
    a >= 0 && a <= atk
}
pub fn is_attack_above(d: &mut Duel, c: CardId, atk: i32) -> bool {
    if !is_monster_now(d, c) {
        return false;
    }
    c.get_attack(d) >= atk
}
pub fn is_defense_below(d: &mut Duel, c: CardId, def: i32) -> bool {
    let cc = d.card(c);
    if cc.data.type_ & TYPE_LINK != 0 {
        return false;
    }
    let in_mzone = cc.current.location as u32 & LOCATION_MZONE != 0;
    if cc.data.type_ & TYPE_MONSTER == 0 && c.get_type(d, None, 0, PLAYER_NONE as u8) & TYPE_MONSTER == 0 && !in_mzone {
        return false;
    }
    let v = c.get_defense(d);
    v >= 0 && v <= def
}
pub fn is_defense_above(d: &mut Duel, c: CardId, def: i32) -> bool {
    if d.card(c).data.type_ & TYPE_LINK != 0 || !is_monster_now(d, c) {
        return false;
    }
    c.get_defense(d) >= def
}
pub fn is_public(d: &Duel, c: CardId) -> bool {
    d.card(c).is_position(POS_FACEUP) != 0
}
pub fn is_forbidden(d: &Duel, c: CardId) -> bool {
    d.card(c).is_status(STATUS_FORBIDDEN)
}
pub fn is_able_to_change_controler(d: &mut Duel, c: CardId) -> bool {
    c.is_capable_change_control(d)
}
/// `c:IsControlerCanBeChanged(ignore_mzone, zone = 0xff)`
pub fn is_controler_can_be_changed(d: &mut Duel, c: CardId, ign: bool, zone: Option<u32>) -> bool {
    c.is_control_can_be_changed(d, ign, zone.unwrap_or(0xff))
}
pub fn add_counter(d: &mut Duel, c: CardId, countertype: u16, count: u16, singly: bool) -> bool {
    let re = d.core.reason_effect;
    if c.is_affect_by_effect(d, re) {
        let rp = d.core.reason_player;
        c.add_counter(d, rp, countertype, count, singly)
    } else {
        false
    }
}
/// `c:RemoveCounter(player, countertype, count, reason)`
pub fn remove_counter(d: &mut Duel, c: CardId, rplayer: u8, countertype: u16, count: u16, reason: u32) -> bool {
    crate::lua::check_action_permission(d);
    if countertype == 0 {
        crate::lua::lua_error("Counter type cannot be 0, use Card.RemoveAllCounters instead");
    }
    d.remove_counter_op(reason, Some(c), rplayer as u32, 0, 0, countertype as u32, count as u32);
    d.run_pushed_units();
    d.returns.at_i32(0) != 0
}
pub fn remove_all_counters(d: &mut Duel, c: CardId) -> u32 {
    crate::lua::check_action_permission(d);
    let mut total = 0u32;
    let counters = d.card(c).counters.clone();
    for (ct, v) in counters {
        let cc = d.card(c);
        let (ctrl, loc, seq) = (cc.current.controler, cc.current.location, cc.current.sequence);
        let m = d.new_message(MSG_REMOVE_COUNTER);
        d.msg(m).u16(ct).u8(ctrl).u8(loc).u8(seq as u8).u16(v[0].wrapping_add(v[1]));
        total += v[0] as u32 + v[1] as u32;
    }
    d.card_mut(c).counters.clear();
    total
}
pub fn get_counter(d: &Duel, c: CardId, countertype: u16) -> i32 {
    d.note_read(c, 16384);
    if countertype == 0 {
        crate::lua::lua_error("Counter type cannot be 0, use Card.GetAllCounters instead");
    }
    c.get_counter(d, countertype)
}
/// `c:GetAllCounters()`: (counter type, count)
pub fn get_all_counters(d: &Duel, c: CardId) -> Vec<(u16, u32)> {
    d.card(c).counters.iter().map(|&(t, v)| (t, v[0] as u32 + v[1] as u32)).collect()
}
pub fn has_counters(d: &Duel, c: CardId) -> bool {
    !d.card(c).counters.is_empty()
}
/// `c:EnableCounterPermit(countertype, location, condition)`
pub fn enable_counter_permit(d: &mut Duel, c: CardId, countertype: u16, location: Option<u32>, condition: Option<LuaFn>) {
    let prange = match location {
        Some(l) => l as u16 as u32,
        None if d.card(c).data.type_ & TYPE_MONSTER != 0 => LOCATION_MZONE,
        None => LOCATION_SZONE | LOCATION_FZONE,
    };
    let pe = d.new_effect();
    let e = d.effect_mut(pe);
    e.owner = Some(c);
    e.type_ = EFFECT_TYPE_SINGLE as u16;
    e.code = EFFECT_COUNTER_PERMIT | countertype as u32;
    e.value = prange as i32;
    e.target = condition;
    c.add_effect(d, pe);
}
pub fn set_counter_limit(d: &mut Duel, c: CardId, countertype: u16, limit: u32) {
    let pe = d.new_effect();
    let e = d.effect_mut(pe);
    e.owner = Some(c);
    e.type_ = EFFECT_TYPE_SINGLE as u16;
    e.code = EFFECT_COUNTER_LIMIT | countertype as u32;
    e.value = limit as i32;
    c.add_effect(d, pe);
}
pub fn is_can_change_position(d: &Duel, c: CardId) -> bool {
    c.is_capable_change_position_by_effect(d, d.core.reason_player)
}
pub fn is_can_turn_set(d: &mut Duel, c: CardId) -> bool {
    let rp = d.core.reason_player;
    c.is_capable_turn_set(d, rp)
}
/// `c:IsCanAddCounter(countertype, count = 0, singly = false, location = 0)`
pub fn is_can_add_counter(d: &mut Duel, c: CardId, countertype: u16, count: u16, singly: bool, loc: u32) -> bool {
    let rp = d.core.reason_player;
    c.is_can_add_counter(d, rp, countertype, count, singly, loc & 0xffff)
}
pub fn is_can_remove_counter(d: &mut Duel, c: CardId, playerid: u8, countertype: u16, count: u16, reason: u32) -> bool {
    if playerid > 1 {
        return false;
    }
    d.is_player_can_remove_counter(playerid, Some(c), 0, 0, countertype, count, reason)
}
/// `c:IsCanBeFusionMaterial(fc, summon_type = SUMMON_TYPE_FUSION, player)`
pub fn is_can_be_fusion_material(d: &mut Duel, c: CardId, fcard: Option<CardId>, summon_type: Option<u64>, playerid: Option<u8>) -> bool {
    let p = playerid.unwrap_or(d.core.reason_player);
    c.is_can_be_fusion_material(d, fcard, summon_type.unwrap_or(SUMMON_TYPE_FUSION as u64), p)
}
/// `c:IsCanBeSynchroMaterial(sc, tuner, player)` (the tuner is not used)
pub fn is_can_be_synchro_material(d: &mut Duel, c: CardId, scard: Option<CardId>, _tuner: Option<CardId>, playerid: Option<u8>) -> bool {
    let p = playerid.unwrap_or(d.core.reason_player);
    c.is_can_be_synchro_material(d, scard, p)
}
pub fn is_can_be_ritual_material(d: &mut Duel, c: CardId, scard: Option<CardId>, playerid: Option<u8>) -> bool {
    let p = playerid.unwrap_or(d.core.reason_player);
    c.is_can_be_ritual_material(d, scard, p)
}
/// `c:IsCanBeXyzMaterial(sc, player, reason = REASON_XYZ|REASON_MATERIAL)`
pub fn is_can_be_xyz_material(d: &mut Duel, c: CardId, scard: Option<CardId>, playerid: Option<u8>, reason: Option<u32>) -> bool {
    let p = playerid.unwrap_or(d.core.reason_player);
    c.is_can_be_xyz_material(d, scard, p, reason.unwrap_or(REASON_XYZ | REASON_MATERIAL))
}
pub fn is_can_be_link_material(d: &mut Duel, c: CardId, scard: Option<CardId>, playerid: Option<u8>) -> bool {
    let p = playerid.unwrap_or(d.core.reason_player);
    c.is_can_be_link_material(d, scard, p)
}
pub fn is_can_be_material(d: &mut Duel, c: CardId, sumtype: u64, scard: Option<CardId>, playerid: Option<u8>) -> bool {
    let p = playerid.unwrap_or(d.core.reason_player);
    c.is_can_be_material(d, scard, sumtype, p)
}
/// `c:CheckFusionMaterial(g, gc, chkf = PLAYER_NONE)`
pub fn check_fusion_material(d: &mut Duel, c: CardId, g: Option<&Group>, gc: Option<&Group>, chkf: Option<u64>) -> bool {
    c.fusion_check(d, g, gc, chkf.unwrap_or(PLAYER_NONE as u64))
}
pub fn check_fusion_substitute(d: &mut Duel, c: CardId, fcard: CardId) -> bool {
    c.check_fusion_substitute(d, Some(fcard))
}
pub fn is_immune_to_effect(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    !c.is_affect_by_effect(d, Some(e))
}
/// `c:IsCanBeDisabledByEffect(e, is_monster_effect = true)`
pub fn is_can_be_disabled_by_effect(d: &mut Duel, c: CardId, e: EffectId, is_monster_effect: bool) -> bool {
    c.is_can_be_disabled_by_effect(d, Some(e), is_monster_effect)
}
/// `c:IsCanBeEffectTarget(e)` (default: the reason effect)
pub fn is_can_be_effect_target(d: &mut Duel, c: CardId, e: Option<EffectId>) -> bool {
    let pe = e.or(d.core.reason_effect).expect("IsCanBeEffectTarget without an effect");
    let rp = d.core.reason_player;
    c.is_capable_be_effect_target(d, pe, rp)
}
pub fn is_can_be_battle_target(d: &mut Duel, c: CardId, bcard: CardId) -> bool {
    c.is_capable_be_battle_target(d, bcard)
}
/// `c:AddMonsterAttribute(type, attribute, race, level, atk, def)`
#[allow(clippy::too_many_arguments)]
pub fn add_monster_attribute(d: &mut Duel, c: CardId, typ: u32, attribute: u32, race: u64, level: u32, atk: i32, def: i32) {
    d.card_mut(c).set_status(STATUS_NO_LEVEL, false);
    let resets = RESET_TURN_SET | RESET_TOGRAVE | RESET_REMOVE | RESET_TEMP_REMOVE | RESET_TOHAND | RESET_TODECK | RESET_OVERLAY;
    let reg = |d: &mut Duel, value: i32, code: u32, reset: u32| {
        let pe = d.new_effect();
        let e = d.effect_mut(pe);
        e.owner = Some(c);
        e.type_ = EFFECT_TYPE_SINGLE as u16;
        e.code = code;
        e.flag[0] = EFFECT_FLAG_CANNOT_DISABLE;
        e.reset_flag = reset;
        e.value = value;
        c.add_effect(d, pe);
    };
    // pre-monster
    reg(d, typ as i32, EFFECT_PRE_MONSTER, RESET_CHAIN | RESET_EVENT | resets);
    if attribute != 0 {
        reg(d, attribute as i32, EFFECT_ADD_ATTRIBUTE, RESET_EVENT | resets);
    }
    if race != 0 {
        reg(d, race as i32, EFFECT_ADD_RACE, RESET_EVENT | resets);
    }
    if level != 0 {
        reg(d, level as i32, EFFECT_CHANGE_LEVEL, RESET_EVENT | resets);
    }
    if atk != 0 {
        reg(d, atk, EFFECT_SET_BASE_ATTACK, RESET_EVENT | resets);
    }
    if def != 0 {
        reg(d, def, EFFECT_SET_BASE_DEFENSE, RESET_EVENT | resets);
    }
}
pub fn add_monster_attribute_complete(_d: &mut Duel, _c: CardId) {}
/// `c:CancelToGrave(cancel = true)`
pub fn cancel_to_grave(d: &mut Duel, c: CardId, cancel: bool) {
    if cancel {
        d.card_mut(c).set_status(STATUS_LEAVE_CONFIRMED, false);
    } else {
        d.core.leave_confirmed.insert(c);
        d.card_mut(c).set_status(STATUS_LEAVE_CONFIRMED, true);
    }
}
pub fn get_tribute_requirement(d: &mut Duel, c: CardId) -> (i32, i32) {
    let r = c.get_summon_tribute_count(d);
    (r & 0xffff, (r >> 16) & 0xffff)
}
pub fn get_battle_target(d: &Duel, c: CardId) -> Option<CardId> {
    if d.core.attacker == Some(c) {
        d.core.attack_target
    } else if d.core.attack_target == Some(c) {
        d.core.attacker
    } else {
        None
    }
}
/// `c:GetAttackableTarget()`: the targets, and whether it can attack directly.
pub fn get_attackable_target(d: &mut Duel, c: CardId) -> (Group, bool) {
    let chain_attack = d.core.chain_attacker_id == d.card(c).fieldid;
    let mut v = Vec::new();
    d.get_attack_target(c, &mut v, chain_attack, true, None);
    (CardSet::from_iter(v), d.card(c).direct_attackable != 0)
}
pub fn set_hint(d: &mut Duel, c: CardId, typ: u8, value: u64) {
    if typ as u32 >= CHINT_DESC_ADD {
        return;
    }
    let loc = c.get_info_location(d);
    let m = d.new_message(MSG_CARD_HINT);
    d.msg(m).loc(loc).u8(typ).u64(value);
}
pub fn reverse_in_deck(d: &mut Duel, c: CardId) {
    if d.card(c).current.location as u32 != LOCATION_DECK {
        return;
    }
    d.core.global_flag |= GLOBALFLAG_DECK_REVERSE_CHECK;
    d.card_mut(c).current.position = POS_FACEUP_DEFENSE;
    let cc = d.card(c);
    let (ctrl, seq, code, pos) = (cc.current.controler, cc.current.sequence, cc.data.code, cc.current.position);
    if seq as usize == d.player[ctrl as usize].list_main.len().wrapping_sub(1) {
        let m = d.new_message(MSG_DECK_TOP);
        d.msg(m).u8(ctrl).u32(0u32).u32(code).u32(pos);
    }
}
/// The unique condition of `c:SetUniqueOnField`: a code, or a filter function.
pub enum UniqueCode {
    Code(u32),
    Function(LuaFn),
}
/// `c:SetUniqueOnField(s, o, code_or_filter, location = LOCATION_ONFIELD)`
pub fn set_unique_on_field(d: &mut Duel, c: CardId, s: u8, o: u8, unique: UniqueCode, location: Option<u32>) {
    {
        let cc = d.card_mut(c);
        cc.unique_pos = [s, o];
        match unique {
            UniqueCode::Function(f) => {
                cc.unique_code = 1;
                cc.unique_function = Some(f);
            }
            UniqueCode::Code(code) => cc.unique_code = code,
        }
    }
    let location = (location.unwrap_or(LOCATION_ONFIELD) as u8 as u32) & LOCATION_ONFIELD;
    d.card_mut(c).unique_location = location;
    let pe = d.new_effect();
    let e = d.effect_mut(pe);
    e.owner = Some(c);
    e.type_ = EFFECT_TYPE_SINGLE as u16;
    e.code = EFFECT_UNIQUE_CHECK;
    e.flag[0] = EFFECT_FLAG_COPY_INHERIT;
    c.add_effect(d, pe);
    d.card_mut(c).unique_effect = Some(pe);
    if d.card(c).current.location as u32 & location != 0 {
        d.add_unique_card(c);
    }
}
/// `c:CheckUniqueOnField(player, location = LOCATION_ONFIELD, icard)`
pub fn check_unique_on_field(d: &mut Duel, c: CardId, check_player: u8, location: Option<u32>, icard: Option<CardId>) -> bool {
    let loc = (location.unwrap_or(LOCATION_ONFIELD) as u8 as u32) & LOCATION_ONFIELD;
    d.check_unique_onfield(c, check_player, loc as u8, icard).is_none()
}
pub fn reset_negate_effect(d: &mut Duel, c: CardId, codes: &[u32]) {
    for &code in codes {
        c.reset(d, code, RESET_CARD);
    }
}
pub fn assume_property(d: &mut Duel, c: CardId, assume: u32, value: u64) {
    if !(ASSUME_CODE..=ASSUME_LINKMARKER).contains(&assume) {
        crate::lua::lua_error("Invalid ASSUME value");
    }
    let cc = d.card_mut(c);
    match cc.assume.iter_mut().find(|a| a.0 == assume) {
        Some(a) => a.1 = value,
        None => {
            let pos = cc.assume.partition_point(|a| a.0 < assume);
            cc.assume.insert(pos, (assume, value));
        }
    }
    if !d.assumes.contains(&c) {
        d.assumes.push(c);
    }
}
pub fn set_spsummon_once(d: &mut Duel, c: CardId, code: u32) {
    if d.card(c).status & STATUS_COPYING_EFFECT != 0 {
        return;
    }
    d.card_mut(c).spsummon_code = code;
    d.core.global_flag |= GLOBALFLAG_SPSUMMON_ONCE;
}
/// `c:Code()`, `c:Code(v)` etc. (the printed data of the card)
pub fn data(d: &Duel, c: CardId) -> CardData {
    d.card(c).data
}
pub fn set_data(d: &mut Duel, c: CardId, data: CardData) {
    d.card_mut(c).data = data;
}
/// `c:Recreate(code, alias, setcodes, type, level, attribute, race, atk, def, lscale, rscale, link_marker, replace_effect)`
pub fn recreate(d: &mut Duel, c: CardId, code: u32, overrides: impl FnOnce(&mut CardData), replace: bool) {
    if c.recreate(d, code) {
        overrides(&mut d.card_mut(c).data);
        if replace {
            c.replace_effect(d, code, 0, 0, true);
        }
    }
}
pub fn cover(d: &Duel, c: CardId) -> u32 {
    d.card(c).cover
}
pub fn set_cover(d: &mut Duel, c: CardId, cover: u32) {
    d.card_mut(c).cover = cover;
}
