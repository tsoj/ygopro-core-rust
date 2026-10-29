//! `Debug.*` (`libdebug.cpp`): the functions duels use (puzzle setup, `ReloadFieldBegin`, is not
//! ported).

use crate::constants::*;
use crate::duel::Duel;
use crate::types::*;

/// `Debug.Message(...)`
pub fn message(d: &mut Duel, text: &str) {
    d.handle_message(text, 1);
}
/// `Debug.SetPlayerInfo(player, lp, start_count, draw_count)`
pub fn set_player_info(d: &mut Duel, p: u8, lp: u32, startcount: u32, drawcount: u32) {
    if p > 1 {
        return;
    }
    let pl = &mut d.player[p as usize];
    pl.lp = lp as i32;
    pl.start_lp = lp as i32;
    pl.start_count = startcount as i32;
    pl.draw_count = drawcount as i32;
}
/// `Debug.PreSummon(c, sumtype, location = 0, sequence = 0, pzone = false)`
pub fn pre_summon(d: &mut Duel, c: CardId, summon_type: u32, location: u8, sequence: u8, pzone: bool) {
    let cc = d.card_mut(c);
    cc.summon.location = location;
    cc.summon.type_ = summon_type;
    cc.summon.sequence = sequence;
    cc.summon.pzone = pzone;
}
pub fn pre_equip(d: &mut Duel, equip_card: CardId, target: CardId) -> bool {
    let (e, t) = (d.card(equip_card).current, d.card(target).current);
    if e.location as u32 != LOCATION_SZONE || t.location as u32 != LOCATION_MZONE || t.position & POS_FACEDOWN != 0 {
        return false;
    }
    equip_card.equip(d, target, false);
    equip_card.insert_effect_target(d, target);
    d.card_mut(target).effect_target_owner.insert(equip_card);
    true
}
pub fn pre_set_target(d: &mut Duel, c: CardId, target: CardId) {
    c.add_card_target(d, target);
}
pub fn pre_add_counter(d: &mut Duel, c: CardId, countertype: u16, count: u16) {
    let cttype = countertype & !(COUNTER_NEED_ENABLE as u16);
    let cc = d.card_mut(c);
    let i = match cc.counters.binary_search_by_key(&cttype, |x| x.0) {
        Ok(i) => i,
        Err(i) => {
            cc.counters.insert(i, (cttype, [0, 0]));
            i
        }
    };
    if countertype as u32 & COUNTER_WITHOUT_PERMIT != 0 && countertype as u32 & COUNTER_NEED_ENABLE == 0 {
        cc.counters[i].1[0] = cc.counters[i].1[0].wrapping_add(count);
    } else {
        cc.counters[i].1[1] = cc.counters[i].1[1].wrapping_add(count);
    }
}
/// `Debug.AddCard(code, owner, player, location, sequence, position, proc = false)`
#[allow(clippy::too_many_arguments)]
pub fn add_card(d: &mut Duel, code: u32, owner: u8, p: u8, location: u32, sequence: u32, position: u8, proc_: bool) -> Option<CardId> {
    if owner > 1 || p > 1 {
        return None;
    }
    let mut location = location & 0xffff;
    let mut sequence = sequence & 0xffff;
    let mut position = position;
    if !d.is_field_location_valid(location, sequence) {
        crate::lua::lua_error("Passed invalid location");
    }
    if d.is_location_useable(p as u32, location, sequence) {
        let c = d.new_card(code);
        d.card_mut(c).owner = owner;
        if location == LOCATION_EXTRA && (position == 0 || d.card(c).data.type_ & TYPE_PENDULUM == 0) {
            position = POS_FACEDOWN_DEFENSE as u8;
        }
        d.card_mut(c).sendto_param.position = position;
        let three = d.is_flag(DUEL_3_COLUMNS_FIELD) as u32;
        let mut pzone = false;
        match location {
            LOCATION_PZONE => {
                location = LOCATION_SZONE;
                sequence = d.get_pzone_index(sequence as u8, p) as u32;
                pzone = true;
            }
            LOCATION_FZONE => {
                location = LOCATION_SZONE;
                sequence = 5;
            }
            LOCATION_STZONE => {
                location = LOCATION_SZONE;
                sequence += three;
            }
            LOCATION_MMZONE => {
                location = LOCATION_MZONE;
                sequence += three;
            }
            LOCATION_EMZONE => {
                location = LOCATION_MZONE;
                sequence += 5;
            }
            _ => {}
        }
        d.add_card(p, c, location as u8, sequence as u8, pzone);
        d.card_mut(c).current.position = position as u32;
        if location & LOCATION_ONFIELD == 0 || position as u32 & POS_FACEUP != 0 {
            c.enable_field_effect(d, true);
            d.adjust_instant();
        }
        if proc_ {
            d.card_mut(c).set_status(STATUS_PROC_COMPLETE, true);
        }
        Some(c)
    } else if location & LOCATION_ONFIELD != 0 {
        let c = d.new_card(code);
        d.card_mut(c).owner = owner;
        let f = d.get_field_card(p as u32, location, sequence)?;
        f.xyz_add(d, c);
        if proc_ {
            d.card_mut(c).set_status(STATUS_PROC_COMPLETE, true);
        }
        Some(c)
    } else {
        None
    }
}
fn write_string_message(d: &mut Duel, code: u32, s: &str, max_len: usize) {
    let b = s.as_bytes();
    let len = b.len().min(max_len);
    let m = d.new_message(code);
    d.msg(m).u16(len as u16).bytes(&b[..len]).u8(0u8);
}
pub fn set_ai_name(d: &mut Duel, name: &str) {
    write_string_message(d, MSG_AI_NAME, name, 100);
}
pub fn show_hint(d: &mut Duel, text: &str) {
    write_string_message(d, MSG_SHOW_HINT, text, 1024);
}
