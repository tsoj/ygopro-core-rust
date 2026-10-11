//! Operations (`operations.cpp` part 3: control changes, self destruction, trap monsters, equips).

use crate::card::public_order;
use crate::constants::*;
use crate::duel::Duel;
use crate::types::*;
use crate::units::{self, LocationReason};

impl Duel {
    fn select_hint(&mut self, player: u8, hint: u64) {
        let m = self.new_message(MSG_HINT);
        self.msg(m).u8(HINT_SELECTMSG).u8(player).u64(hint);
    }

    pub fn process_get_control(&mut self, arg: &mut units::GetControl) -> bool {
        let (reason_effect, chose_player, playerid, reset_phase, reset_count, zone) = (arg.reason_effect, arg.chose_player, arg.playerid, arg.reset_phase, arg.reset_count, arg.zone);
        let tg = arg.targets.unwrap();
        match arg.step {
            0 => {
                arg.destroy_set.clear();
                let set = self.group(tg).clone();
                for c in public_order(self, &set) {
                    c.filter_disable_related_cards(self);
                    let cc = self.card(c);
                    let mut change = !(cc.overlay_target.is_some()
                        || cc.current.controler == playerid
                        || cc.current.controler == PLAYER_NONE as u8
                        || cc.current.location as u32 != LOCATION_MZONE);
                    if change && !c.is_capable_change_control(self) {
                        change = false;
                    }
                    if change && reason_effect.is_some() && !c.is_affect_by_effect(self, reason_effect) {
                        change = false;
                    }
                    if change
                        && !self.is_flag(DUEL_TRAP_MONSTERS_NOT_USE_ZONE)
                        && c.get_type(self, None, 0, 2) & TYPE_TRAPMONSTER != 0
                        && self.get_useable_count(Some(c), playerid, LOCATION_SZONE as u8, playerid, LocationReason::Control, 0xff, None) <= 0
                    {
                        change = false;
                    }
                    if !change {
                        self.group_mut(tg).remove(c);
                    }
                }
                let fcount = self.get_useable_count(None, playerid, LOCATION_MZONE as u8, playerid, LocationReason::Control, zone, None);
                if fcount <= 0 {
                    let all = std::mem::take(self.group_mut(tg));
                    arg.destroy_set = all;
                    arg.step = 5;
                    return false;
                }
                let n = self.group(tg).len() as i32;
                if n > fcount {
                    let set = self.group(tg).clone();
                    self.core.select_cards = public_order(self, &set);
                    self.select_hint(playerid, 502);
                    let ct = (n - fcount) as u8;
                    self.emplace_process(units::SelectCard { step: 0, playerid, cancelable: false, min: ct, max: ct });
                } else {
                    arg.step = 1;
                }
                false
            }
            1 => {
                for &c in &self.return_cards.list {
                    arg.destroy_set.insert(c);
                }
                for c in self.return_cards.list.clone() {
                    self.group_mut(tg).remove(c);
                }
                false
            }
            2 => {
                let set = self.group(tg).clone();
                for c in public_order(self, &set) {
                    if self.card(c).unique_code != 0 && self.card(c).unique_location & LOCATION_MZONE != 0 {
                        self.remove_unique_card(c);
                    }
                }
                arg.iter_order = public_order(self, &set);
                arg.iter_pos = 0;
                false
            }
            3 => {
                if arg.iter_pos >= arg.iter_order.len() {
                    self.adjust_instant();
                    arg.step = 4;
                    return false;
                }
                let c = arg.iter_order[arg.iter_pos];
                let mp = if chose_player == PLAYER_NONE as u8 { playerid } else { chose_player };
                let pos = self.card(c).current.position as u8;
                self.move_to_field(c, mp, playerid, LOCATION_MZONE as u16, pos, false, 0, zone as u8, false, LocationReason::None, true);
                false
            }
            4 => {
                let c = arg.iter_order[arg.iter_pos];
                self.card_mut(c).set_status(STATUS_ATTACK_CANCELED, true);
                self.set_control(c, playerid, reset_phase, reset_count);
                c.reset(self, RESET_CONTROL, RESET_EVENT);
                c.filter_disable_related_cards(self);
                arg.iter_pos += 1;
                arg.step = 2;
                false
            }
            5 => {
                let reason_player = if chose_player == PLAYER_NONE as u8 { PLAYER_NONE as u8 } else { self.core.reason_player };
                let set = self.group(tg).clone();
                for c in public_order(self, &set) {
                    if self.card(c).current.location as u32 & LOCATION_ONFIELD == 0 {
                        self.group_mut(tg).remove(c);
                        continue;
                    }
                    if self.card(c).unique_code != 0 && self.card(c).unique_location & LOCATION_MZONE != 0 {
                        self.add_unique_card(c);
                    }
                    self.raise_single_event(c, None, EVENT_CONTROL_CHANGED, reason_effect, REASON_EFFECT, reason_player, playerid, 0);
                    self.raise_single_event(c, None, EVENT_MOVE, reason_effect, REASON_EFFECT, reason_player, playerid, 0);
                }
                let set = self.group(tg).clone();
                if !set.is_empty() {
                    self.raise_event(set.clone(), EVENT_CONTROL_CHANGED, reason_effect, REASON_EFFECT, reason_player, playerid, 0);
                    self.raise_event(set, EVENT_MOVE, reason_effect, REASON_EFFECT, reason_player, playerid, 0);
                }
                self.process_single_event();
                self.process_instant_event();
                false
            }
            6 => {
                if !arg.destroy_set.is_empty() {
                    let ds = std::mem::take(&mut arg.destroy_set);
                    self.destroy(ds, None, REASON_RULE, PLAYER_NONE as u8, PLAYER_NONE as u8, 0, 0);
                }
                false
            }
            7 => {
                let set = self.group(tg).clone();
                self.returns.set_i32(0, set.len() as i32);
                self.core.operated_set = set;
                true
            }
            _ => true,
        }
    }

    pub fn process_swap_control(&mut self, arg: &mut units::SwapControl) -> bool {
        let (reason_effect, reason_player, reset_phase, reset_count) = (arg.reason_effect, arg.reason_player, arg.reset_phase, arg.reset_count);
        let (t1, t2) = (arg.targets1.unwrap(), arg.targets2.unwrap());
        match arg.step {
            0 => {
                arg.step = 9;
                let s1 = self.group(t1).clone();
                let s2 = self.group(t2).clone();
                if s1.is_empty() || s2.is_empty() || s1.len() != s2.len() {
                    return false;
                }
                for c in public_order(self, &s1) {
                    c.filter_disable_related_cards(self);
                }
                for c in public_order(self, &s2) {
                    c.filter_disable_related_cards(self);
                }
                let p1 = self.card(s1.first().unwrap()).current.controler;
                let p2 = self.card(s2.first().unwrap()).current.controler;
                if p1 == p2 || p1 == PLAYER_NONE as u8 || p2 == PLAYER_NONE as u8 {
                    return false;
                }
                for (set, p) in [(&s1, p1), (&s2, p2)] {
                    for c in public_order(self, set) {
                        let cc = self.card(c);
                        if cc.overlay_target.is_some() || cc.current.controler != p || cc.current.location as u32 != LOCATION_MZONE {
                            return false;
                        }
                        if !c.is_capable_change_control(self) {
                            return false;
                        }
                        if reason_effect.is_some() && !c.is_affect_by_effect(self, reason_effect) {
                            return false;
                        }
                    }
                }
                for (set, p) in [(&s1, p1), (&s2, p2)] {
                    let mut ct = self.get_useable_count(None, p, LOCATION_MZONE as u8, reason_player, LocationReason::Control, 0xff, None);
                    for c in public_order(self, set) {
                        if self.card(c).current.sequence >= 5 {
                            ct -= 1;
                        }
                    }
                    if ct < 0 {
                        return false;
                    }
                }
                for set in [&s1, &s2] {
                    for c in public_order(self, set) {
                        if self.card(c).unique_code != 0 && self.card(c).unique_location & LOCATION_MZONE != 0 {
                            self.remove_unique_card(c);
                        }
                    }
                }
                arg.iter1 = public_order(self, &s1);
                arg.pos1 = 0;
                arg.iter2 = public_order(self, &s2);
                arg.pos2 = 0;
                arg.step = 0;
                false
            }
            1 => {
                if arg.pos1 >= arg.iter1.len() {
                    arg.step = 3;
                    return false;
                }
                let c1 = arg.iter1[arg.pos1];
                let p1 = self.card(c1).current.controler;
                let s1 = self.card(c1).current.sequence;
                let mut flag = 0u32;
                self.get_useable_count(None, p1, LOCATION_MZONE as u8, reason_player, LocationReason::Control, 0xff, Some(&mut flag));
                flag = (flag & !(1u32 << s1) & 0xff) | !0x1f;
                let c2 = arg.iter2[arg.pos2];
                let code = self.card(c2).data.code;
                self.select_hint(p1, code as u64);
                self.emplace_process(units::SelectPlace { step: 0, playerid: p1, count: 1, flag, disable_field: false });
                false
            }
            2 => {
                arg.self_selected_sequence = self.returns.at_i8(2) as u8;
                let c2 = arg.iter2[arg.pos2];
                let p2 = self.card(c2).current.controler;
                let s2 = self.card(c2).current.sequence;
                let mut flag = 0u32;
                self.get_useable_count(None, p2, LOCATION_MZONE as u8, reason_player, LocationReason::Control, 0xff, Some(&mut flag));
                flag = (flag & !(1u32 << s2) & 0xff) | !0x1f;
                let c1 = arg.iter1[arg.pos1];
                let code = self.card(c1).data.code;
                self.select_hint(p2, code as u64);
                self.emplace_process(units::SelectPlace { step: 0, playerid: p2, count: 1, flag, disable_field: false });
                false
            }
            3 => {
                let c1 = arg.iter1[arg.pos1];
                let c2 = arg.iter2[arg.pos2];
                let p1 = self.card(c1).current.controler;
                let p2 = self.card(c2).current.controler;
                let new_s1 = arg.self_selected_sequence;
                let new_s2 = self.returns.at_i8(2) as u8;
                self.swap_card_seq(c1, c2, new_s1, new_s2);
                c1.reset(self, RESET_CONTROL, RESET_EVENT);
                c2.reset(self, RESET_CONTROL, RESET_EVENT);
                self.set_control(c1, p2, reset_phase, reset_count);
                self.set_control(c2, p1, reset_phase, reset_count);
                self.card_mut(c1).set_status(STATUS_ATTACK_CANCELED, true);
                self.card_mut(c2).set_status(STATUS_ATTACK_CANCELED, true);
                arg.pos1 += 1;
                arg.pos2 += 1;
                arg.step = 0;
                false
            }
            4 => {
                let s2 = self.group(t2).clone();
                self.group_mut(t1).extend(s2.iter());
                let s1 = self.group(t1).clone();
                for c in public_order(self, &s1) {
                    c.filter_disable_related_cards(self);
                    if self.card(c).unique_code != 0 && self.card(c).unique_location & LOCATION_MZONE != 0 {
                        self.add_unique_card(c);
                    }
                    let ctrl = self.card(c).current.controler;
                    self.raise_single_event(c, None, EVENT_CONTROL_CHANGED, reason_effect, REASON_EFFECT, reason_player, ctrl, 0);
                    self.raise_single_event(c, None, EVENT_MOVE, reason_effect, REASON_EFFECT, reason_player, ctrl, 0);
                }
                self.raise_event(s1.clone(), EVENT_CONTROL_CHANGED, reason_effect, REASON_EFFECT, reason_player, 0, 0);
                self.raise_event(s1, EVENT_MOVE, reason_effect, REASON_EFFECT, reason_player, 0, 0);
                self.process_single_event();
                self.process_instant_event();
                false
            }
            5 => {
                self.core.operated_set = self.group(t1).clone();
                self.returns.set_i32(0, 1);
                true
            }
            10 => {
                self.core.operated_set.clear();
                self.returns.set_i32(0, 0);
                true
            }
            _ => true,
        }
    }

    pub fn process_control_adjust(&mut self, arg: &mut units::ControlAdjust) -> bool {
        let tp = self.infos.turn_player;
        match arg.step {
            0 => {
                arg.destroy_set.clear();
                let b0 = self.get_useable_count(None, 0, LOCATION_MZONE as u8, 0, LocationReason::Control, 0xff, None) as u32;
                let b1 = self.get_useable_count(None, 1, LOCATION_MZONE as u8, 1, LocationReason::Control, 0xff, None) as u32;
                for p in 0..2 {
                    for c in self.core.control_adjust_set[p].clone().iter() {
                        c.filter_disable_related_cards(self);
                    }
                }
                let (n0, n1) = (self.core.control_adjust_set[0].len() as u32, self.core.control_adjust_set[1].len() as u32);
                let handle = |d: &mut Duel, arg: &mut units::ControlAdjust, from: usize, nf: u32, no: u32, b: u32, selector: u8| {
                    if nf.wrapping_sub(no) > b {
                        if no == 0 && b == 0 {
                            arg.destroy_set = std::mem::take(&mut d.core.control_adjust_set[from]);
                            arg.step = 4;
                        } else {
                            arg.adjusting_player = from as u8;
                            let count = (nf - no - b) as u8;
                            d.core.select_cards = d.core.control_adjust_set[from].to_vec();
                            d.select_hint(tp, 502);
                            d.emplace_process(units::SelectCard { step: 0, playerid: selector, cancelable: false, min: count, max: count });
                        }
                    } else {
                        arg.step = 1;
                    }
                };
                if n0 > n1 {
                    handle(self, arg, 0, n0, n1, b1, 1);
                } else if n0 < n1 {
                    handle(self, arg, 1, n1, n0, b0, 0);
                } else {
                    arg.step = 1;
                }
                false
            }
            1 => {
                for &c in &self.return_cards.list {
                    arg.destroy_set.insert(c);
                }
                let ap = arg.adjusting_player as usize;
                for c in self.return_cards.list.clone() {
                    self.core.control_adjust_set[ap].remove(c);
                }
                false
            }
            2 => {
                for p in 0..2 {
                    for c in self.core.control_adjust_set[p].clone().iter() {
                        if self.card(c).unique_code != 0 && self.card(c).unique_location & LOCATION_MZONE != 0 {
                            self.remove_unique_card(c);
                        }
                    }
                }
                let order1 = public_order(self, &self.core.control_adjust_set[0].clone());
                let order2 = public_order(self, &self.core.control_adjust_set[1].clone());
                let n = order1.len().min(order2.len());
                for i in 0..n {
                    self.swap_card(order1[i], order2[i]);
                    order1[i].reset(self, RESET_CONTROL, RESET_EVENT);
                    order2[i].reset(self, RESET_CONTROL, RESET_EVENT);
                }
                arg.adjust_set = CardSet::from_iter(order1[n..].iter().chain(order2[n..].iter()).copied());
                false
            }
            3 => {
                if arg.adjust_set.is_empty() {
                    return false;
                }
                let c = public_order(self, &arg.adjust_set)[0];
                arg.adjust_set.remove(c);
                c.reset(self, RESET_CONTROL, RESET_EVENT);
                let other = 1 - self.card(c).current.controler;
                let pos = self.card(c).current.position as u8;
                self.move_to_field(c, other, other, LOCATION_MZONE as u16, pos, false, 0, 0xff, false, LocationReason::None, true);
                arg.step = 2;
                false
            }
            4 => {
                let s1 = self.core.control_adjust_set[1].clone();
                self.core.control_adjust_set[0].extend(s1.iter());
                let s0 = self.core.control_adjust_set[0].clone();
                for c in public_order(self, &s0) {
                    if self.card(c).current.location as u32 & LOCATION_ONFIELD == 0 {
                        self.core.control_adjust_set[0].remove(c);
                        continue;
                    }
                    c.filter_disable_related_cards(self);
                    if self.card(c).unique_code != 0 && self.card(c).unique_location & LOCATION_MZONE != 0 {
                        self.add_unique_card(c);
                    }
                    let ctrl = self.card(c).current.controler;
                    self.raise_single_event(c, None, EVENT_CONTROL_CHANGED, None, REASON_RULE, 0, ctrl, 0);
                    self.raise_single_event(c, None, EVENT_MOVE, None, REASON_RULE, 0, ctrl, 0);
                }
                let s0 = self.core.control_adjust_set[0].clone();
                if !s0.is_empty() {
                    self.raise_event(s0.clone(), EVENT_CONTROL_CHANGED, None, 0, 0, 0, 0);
                    self.raise_event(s0, EVENT_MOVE, None, 0, 0, 0, 0);
                }
                self.process_single_event();
                self.process_instant_event();
                false
            }
            5 => {
                if !arg.destroy_set.is_empty() {
                    let ds = std::mem::take(&mut arg.destroy_set);
                    self.destroy(ds, None, REASON_RULE, PLAYER_NONE as u8, PLAYER_NONE as u8, 0, 0);
                }
                true
            }
            _ => true,
        }
    }

    pub fn process_self_destroy_unique(&mut self, arg: &mut units::SelfDestroyUnique) -> bool {
        let uc = arg.unique_card;
        let mut playerid = arg.playerid;
        match arg.step {
            0 => {
                if !self.core.unique_cards[playerid as usize].contains(&uc) {
                    self.core.unique_destroy_set.remove(uc);
                    return true;
                }
                let mut cset = CardSet::new();
                uc.get_unique_target(self, &mut cset, playerid, None);
                if cset.is_empty() {
                    self.card_mut(uc).unique_fieldid = 0;
                } else if cset.len() == 1 {
                    let f = self.card(cset.first().unwrap()).fieldid;
                    self.card_mut(uc).unique_fieldid = f;
                } else {
                    let ordered = public_order(self, &cset);
                    let pick = |d: &Duel, p: u8, check_fid: bool| -> Vec<CardId> {
                        ordered.iter().copied().filter(|&c| d.card(c).current.controler == p && (!check_fid || d.card(c).unique_fieldid != u32::MAX)).collect()
                    };
                    self.core.select_cards = pick(self, playerid, true);
                    if self.core.select_cards.is_empty() {
                        playerid = 1 - playerid;
                        self.core.select_cards = pick(self, playerid, true);
                    }
                    if self.core.select_cards.is_empty() {
                        playerid = 1 - playerid;
                        self.core.select_cards = pick(self, playerid, false);
                    }
                    if self.core.select_cards.is_empty() {
                        playerid = 1 - playerid;
                        self.core.select_cards = pick(self, playerid, false);
                    }
                    if self.core.select_cards.len() == 1 {
                        self.return_cards.clear();
                        let c = self.core.select_cards[0];
                        self.return_cards.list.push(c);
                    } else {
                        self.select_hint(playerid, 534);
                        self.emplace_process(units::SelectCard { step: 0, playerid, cancelable: false, min: 1, max: 1 });
                    }
                    return false;
                }
                self.core.unique_destroy_set.remove(uc);
                true
            }
            1 => {
                let mut cset = CardSet::new();
                uc.get_unique_target(self, &mut cset, playerid, None);
                let mcard = self.return_cards.list[0];
                let f = self.card(mcard).fieldid;
                self.card_mut(uc).unique_fieldid = f;
                cset.remove(mcard);
                let (ue, uctrl) = (self.card(uc).unique_effect, self.card(uc).current.controler);
                for c in public_order(self, &cset) {
                    let cc = self.card_mut(c);
                    cc.temp.reason_effect = cc.current.reason_effect;
                    cc.temp.reason_player = cc.current.reason_player;
                    cc.current.reason_effect = ue;
                    cc.current.reason_player = uctrl;
                }
                self.destroy(cset, None, REASON_RULE, PLAYER_SELFDES as u8, PLAYER_NONE as u8, 0, 0);
                false
            }
            2 => {
                self.core.unique_destroy_set.remove(uc);
                true
            }
            _ => true,
        }
    }

    pub fn process_self_destroy(&mut self, arg: &mut units::SelfDestroy) -> bool {
        match arg.step {
            0 => {
                if self.core.self_destroy_set.is_empty() {
                    return false;
                }
                let c = public_order(self, &self.core.self_destroy_set)[0];
                if let Some(pe) = c.is_affected_by_effect(self, EFFECT_SELF_DESTROY) {
                    let hp = pe.get_handler_player(self);
                    let cc = self.card_mut(c);
                    cc.temp.reason_effect = cc.current.reason_effect;
                    cc.temp.reason_player = cc.current.reason_player;
                    cc.current.reason_effect = Some(pe);
                    cc.current.reason_player = hp;
                    self.destroy(CardSet::from_card(c), Some(pe), REASON_EFFECT, PLAYER_SELFDES as u8, PLAYER_NONE as u8, 0, 0);
                }
                self.core.self_destroy_set.remove(c);
                arg.step = units::RESTART;
                false
            }
            1 => {
                self.returns.set_i32(0, 0);
                self.core.operated_set.clear();
                true
            }
            _ => true,
        }
    }

    pub fn process_self_to_grave(&mut self, arg: &mut units::SelfToGrave) -> bool {
        match arg.step {
            0 => {
                if self.core.self_tograve_set.is_empty() {
                    return false;
                }
                let c = public_order(self, &self.core.self_tograve_set)[0];
                if let Some(pe) = c.is_affected_by_effect(self, EFFECT_SELF_TOGRAVE) {
                    let hp = pe.get_handler_player(self);
                    let cc = self.card_mut(c);
                    cc.temp.reason_effect = cc.current.reason_effect;
                    cc.temp.reason_player = cc.current.reason_player;
                    cc.current.reason_effect = Some(pe);
                    cc.current.reason_player = hp;
                    self.send_to(CardSet::from_card(c), None, REASON_EFFECT, PLAYER_NONE as u8, PLAYER_NONE as u8, LOCATION_GRAVE as u16, 0, POS_FACEUP as u8, false);
                }
                self.core.self_tograve_set.remove(c);
                arg.step = units::RESTART;
                false
            }
            1 => {
                self.returns.set_i32(0, 0);
                self.core.operated_set.clear();
                true
            }
            _ => true,
        }
    }

    pub fn process_trap_monster_adjust(&mut self, arg: &mut units::TrapMonsterAdjust) -> bool {
        let tp = self.infos.turn_player;
        let check_player = if arg.oppo_selection { 1 - tp } else { tp };
        let mut step = arg.step;
        loop {
            match step {
                0 => {
                    if !self.is_flag(DUEL_TRAP_MONSTERS_NOT_USE_ZONE) {
                        arg.step = 3;
                        return false;
                    }
                    arg.to_grave_set.clear();
                    return false;
                }
                1 => {
                    self.refresh_location_info_instant();
                    let fcount = self.get_useable_count(None, check_player, LOCATION_SZONE as u8, check_player, LocationReason::None, 0xff, None);
                    let cp = check_player as usize;
                    if fcount <= 0 {
                        let s = std::mem::take(&mut self.core.trap_monster_adjust_set[cp]);
                        arg.to_grave_set.extend(s.iter());
                        arg.step = 2;
                    } else if self.core.trap_monster_adjust_set[cp].len() as i32 > fcount {
                        let ct = (self.core.trap_monster_adjust_set[cp].len() as i32 - fcount) as u8;
                        self.core.select_cards = self.core.trap_monster_adjust_set[cp].to_vec();
                        self.select_hint(check_player, 502);
                        self.emplace_process(units::SelectCard { step: 0, playerid: check_player, cancelable: false, min: ct, max: ct });
                    } else {
                        arg.step = 2;
                    }
                    return false;
                }
                2 => {
                    for c in self.return_cards.list.clone() {
                        arg.to_grave_set.insert(c);
                        self.core.trap_monster_adjust_set[check_player as usize].remove(c);
                    }
                    step = 3;
                    arg.step = 3;
                }
                3 => {
                    if !arg.oppo_selection {
                        arg.oppo_selection = true;
                        arg.step = 0;
                    }
                    return false;
                }
                4 => {
                    let mut p = tp;
                    for _ in 0..2 {
                        for c in self.core.trap_monster_adjust_set[p as usize].clone().iter() {
                            c.reset(self, RESET_TURN_SET, RESET_EVENT);
                            if !self.is_flag(DUEL_TRAP_MONSTERS_NOT_USE_ZONE) {
                                self.refresh_location_info_instant();
                            }
                            let pos = self.card(c).current.position as u8;
                            self.move_to_field(c, p, p, LOCATION_SZONE as u16, pos, false, 2, 0xff, false, LocationReason::None, true);
                        }
                        p = 1 - p;
                    }
                    if !arg.to_grave_set.is_empty() {
                        let s = std::mem::take(&mut arg.to_grave_set);
                        self.send_to(s, None, REASON_RULE, PLAYER_NONE as u8, PLAYER_NONE as u8, LOCATION_GRAVE as u16, 0, POS_FACEUP as u8, false);
                    }
                    return true;
                }
                _ => return true,
            }
        }
    }

    pub fn process_equip(&mut self, arg: &mut units::Equip) -> bool {
        let (equip_player, equip_card, target, faceup, is_step) = (arg.equip_player, arg.equip_card, arg.target, arg.faceup, arg.is_step);
        match arg.step {
            0 => {
                self.returns.set_i32(0, 0);
                let re = self.core.reason_effect;
                if !equip_card.is_affect_by_effect(self, re) || equip_card == target {
                    return true;
                }
                let mut to_grave = false;
                let tc = self.card(target);
                if tc.current.location as u32 != LOCATION_MZONE || tc.current.position & POS_FACEDOWN != 0 {
                    if self.is_flag(DUEL_EQUIP_NOT_SENT_IF_MISSING_TARGET) && self.card(equip_card).current.location as u32 == LOCATION_MZONE {
                        return true;
                    }
                    to_grave = true;
                }
                if self.card(equip_card).current.location as u32 != LOCATION_SZONE {
                    self.refresh_location_info_instant();
                    if self.get_useable_count(Some(equip_card), equip_player, LOCATION_SZONE as u8, equip_player, LocationReason::ToField, 0xff, None) <= 0 {
                        to_grave = true;
                    }
                }
                if to_grave {
                    if self.card(equip_card).current.location as u32 != LOCATION_GRAVE {
                        self.send_to(CardSet::from_card(equip_card), None, REASON_RULE, PLAYER_NONE as u8, PLAYER_NONE as u8, LOCATION_GRAVE as u16, 0, POS_FACEUP as u8, false);
                    }
                    arg.step = 2;
                    return false;
                }
                if let Some(et) = self.card(equip_card).equiping_target {
                    self.card_mut(equip_card).effect_target_cards.remove(et);
                    self.card_mut(et).effect_target_owner.remove(equip_card);
                    equip_card.unequip(self);
                    equip_card.enable_field_effect(self, false);
                    return false;
                }
                if self.card(equip_card).current.location as u32 == LOCATION_SZONE {
                    if faceup && self.card(equip_card).is_position(POS_FACEDOWN) != 0 {
                        self.change_position_one(equip_card, None, equip_player, POS_FACEUP as u8, 0, false);
                    }
                    return false;
                }
                equip_card.enable_field_effect(self, false);
                self.card_mut(equip_card).current.reason_player = equip_player;
                let pos = if faceup || self.card(equip_card).is_position(POS_FACEUP) != 0 { POS_FACEUP } else { POS_FACEDOWN } as u8;
                self.move_to_field(equip_card, equip_player, equip_player, LOCATION_SZONE as u16, pos, false, 0, 0xff, false, LocationReason::None, true);
                false
            }
            1 => {
                equip_card.equip(self, target, true);
                if self.card(equip_card).data.type_ & TYPE_EQUIP == 0 {
                    let pe = self.new_effect();
                    let dt = self.card(equip_card).data.type_;
                    let is_trap = equip_card.get_type(self, None, 0, 2) & TYPE_TRAP != 0;
                    let e = self.effect_mut(pe);
                    e.owner = Some(equip_card);
                    e.handler = Some(equip_card);
                    e.type_ = EFFECT_TYPE_SINGLE as u16;
                    if is_trap {
                        e.code = EFFECT_ADD_TYPE;
                        e.value = TYPE_EQUIP as i32;
                    } else if dt & TYPE_UNION != 0 {
                        e.code = EFFECT_CHANGE_TYPE;
                        e.value = (TYPE_EQUIP + TYPE_SPELL + TYPE_UNION) as i32;
                    } else if dt & TYPE_TOKEN != 0 {
                        e.code = EFFECT_CHANGE_TYPE;
                        e.value = (TYPE_EQUIP + TYPE_SPELL + TYPE_TOKEN) as i32;
                    } else {
                        e.code = EFFECT_CHANGE_TYPE;
                        e.value = (TYPE_EQUIP + TYPE_SPELL) as i32;
                    }
                    e.flag[0] = EFFECT_FLAG_CANNOT_DISABLE;
                    e.reset_flag = RESET_EVENT + 0x17e0000;
                    equip_card.add_effect(self, pe);
                }
                self.card_mut(equip_card).effect_target_cards.insert(target);
                self.card_mut(target).effect_target_owner.insert(equip_card);
                if !is_step {
                    if self.card(equip_card).is_position(POS_FACEUP) != 0 {
                        equip_card.enable_field_effect(self, true);
                    }
                    self.adjust_disable_check_list();
                    let cset = CardSet::from_card(equip_card);
                    let (re, rp) = (self.core.reason_effect, self.core.reason_player);
                    self.raise_single_event(target, Some(&cset), EVENT_EQUIP, re, 0, rp, PLAYER_NONE as u8, 0);
                    self.raise_event(cset, EVENT_EQUIP, re, 0, rp, PLAYER_NONE as u8, 0);
                    let ht = match self.card(target).overlay_target {
                        Some(o) => self.card(o).current.controler,
                        None => self.card(target).current.controler,
                    };
                    self.core.hint_timing[ht as usize] |= TIMING_EQUIP;
                    self.process_single_event();
                    self.process_instant_event();
                    false
                } else {
                    self.core.equiping_cards.insert(equip_card);
                    self.returns.set_i32(0, 1);
                    true
                }
            }
            2 => {
                self.returns.set_i32(0, 1);
                true
            }
            3 => {
                self.returns.set_i32(0, 0);
                true
            }
            _ => true,
        }
    }
}
