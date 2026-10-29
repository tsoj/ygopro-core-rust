//! The processor (`processor.cpp` part 6: location refresh, adjustments, startup, small units).

use crate::card::public_order;
use crate::constants::*;
use crate::duel::{Duel, Event};
use crate::field1::field_used_count;
use crate::types::*;
use crate::units;

impl Duel {
    pub fn process_refresh_loc(&mut self, arg: &mut units::RefreshLoc) -> bool {
        match arg.step {
            0 => {
                if self.is_flag(DUEL_3_COLUMNS_FIELD) {
                    self.player[0].used_location |= 0x1111;
                    self.player[1].used_location |= 0x1111;
                }
                arg.previously_disabled_locations = (self.player[0].disabled_location & 0xffff) | (self.player[1].disabled_location << 16);
                self.player[0].disabled_location = 0;
                self.player[1].disabled_location = 0;
                self.core.disfield_effects.clear();
                self.core.extra_mzone_effects.clear();
                self.core.extra_szone_effects.clear();
                let mut eset = Vec::new();
                self.filter_field_effect(EFFECT_DISABLE_FIELD, &mut eset, true);
                for pe in eset {
                    let value = pe.get_value(self, 0) as u32;
                    if value != 0 && !self.effect(pe).is_flag(EFFECT_FLAG_REPEAT) {
                        self.player[0].disabled_location |= value & 0xff7f;
                        self.player[1].disabled_location |= (value >> 16) & 0xff7f;
                    } else {
                        self.core.disfield_effects.push(pe);
                    }
                }
                let mut eset = Vec::new();
                self.filter_field_effect(EFFECT_USE_EXTRA_MZONE, &mut eset, true);
                for pe in eset {
                    let p = pe.get_handler_player(self) as usize;
                    let value = pe.get_value(self, 0) as u32;
                    self.player[p].disabled_location |= (value >> 16) & 0x1f;
                    if (field_used_count((value >> 16) & 0x1f) as u32) < (value & 0xffff) {
                        self.core.extra_mzone_effects.push(pe);
                    }
                }
                let mut eset = Vec::new();
                self.filter_field_effect(EFFECT_USE_EXTRA_SZONE, &mut eset, true);
                for pe in eset {
                    let p = pe.get_handler_player(self) as usize;
                    let value = pe.get_value(self, 0) as u32;
                    self.player[p].disabled_location |= (value >> 8) & 0x1f00;
                    if (field_used_count((value >> 16) & 0x1f) as u32) < (value & 0xffff) {
                        self.core.extra_szone_effects.push(pe);
                    }
                }
                false
            }
            1 => {
                if self.core.disfield_effects.is_empty() {
                    arg.step = 2;
                    return false;
                }
                let pe = self.core.disfield_effects.remove(0);
                arg.current_disable_field_effect = Some(pe);
                if self.effect(pe).operation.is_none() {
                    self.effect_mut(pe).value = 0x80;
                    arg.step = 0;
                    return false;
                }
                let ne = self.nil_event.clone();
                self.core.sub_solving_event.push_back(ne);
                let hp = pe.get_handler_player(self);
                self.emplace_process(units::ExecuteOperation { step: 0, triggering_player: hp, shuffle_check_was_disabled: false, triggering_effect: pe });
                false
            }
            2 => {
                let mut disabled = self.returns.at_u32(0);
                disabled &= 0xff7fff7f;
                if disabled == 0 {
                    disabled = 0x80;
                }
                let pe = arg.current_disable_field_effect.unwrap();
                if pe.get_handler_player(self) == 0 {
                    self.effect_mut(pe).value = disabled as i32;
                    self.player[0].disabled_location |= disabled & 0xff7f;
                    self.player[1].disabled_location |= (disabled >> 16) & 0xff7f;
                } else {
                    self.effect_mut(pe).value = ((disabled << 16) | (disabled >> 16)) as i32;
                    self.player[1].disabled_location |= disabled & 0xff7f;
                    self.player[0].disabled_location |= (disabled >> 16) & 0xff7f;
                }
                self.returns.set_u32(0, disabled);
                arg.step = 0;
                false
            }
            3 | 5 => {
                let mzone = arg.step == 3;
                let list = if mzone { &mut self.core.extra_mzone_effects } else { &mut self.core.extra_szone_effects };
                if list.is_empty() {
                    arg.step = if mzone { 4 } else { 6 };
                    return false;
                }
                let pe = list.remove(0);
                arg.current_disable_field_effect = Some(pe);
                let p = pe.get_handler_player(self) as usize;
                let used = self.player[p].disabled_location | self.player[p].used_location;
                let zone_flag = if mzone { used & 0x1f } else { (used >> 8) & 0x1f };
                if zone_flag == 0x1f {
                    arg.step = if mzone { 4 } else { 6 };
                    return false;
                }
                let val = pe.get_value(self, 0) as i32;
                let mut dis_count = (val & 0xffff) - field_used_count(((val >> 16) & 0x1f) as u32);
                let empty_count = 5 - field_used_count(zone_flag);
                let flag = if mzone { zone_flag | 0xffffffe0 } else { (zone_flag << 8) | 0xffffe0ff };
                if mzone {
                    if dis_count > empty_count {
                        dis_count = empty_count;
                    }
                } else if (dis_count as u32) > (empty_count as u32) {
                    dis_count = empty_count;
                }
                arg.dis_count = dis_count as u8;
                self.emplace_process(units::SelectPlace { step: 0, playerid: p as u8, count: dis_count as u8, flag, disable_field: true });
                false
            }
            4 | 6 => {
                let mzone = arg.step == 4;
                let mut zone_flag = 0u32;
                let mut pt = 0usize;
                for _ in 0..arg.dis_count {
                    let s = self.returns.at_i8(pt + 2) as u8;
                    zone_flag |= 1u32 << s;
                    pt += 3;
                }
                let pe = arg.current_disable_field_effect.unwrap();
                let hp = pe.get_handler_player(self) as usize;
                if mzone {
                    self.player[hp].disabled_location |= zone_flag;
                } else {
                    self.player[hp].disabled_location |= zone_flag << 8;
                }
                let v = self.effect(pe).value;
                self.effect_mut(pe).value = (v as u32 | (zone_flag << 16)) as i32;
                arg.step = if mzone { 2 } else { 4 };
                false
            }
            7 => {
                let d1 = self.player[1].disabled_location;
                self.player[0].disabled_location |= (((d1 >> 5) & 1) << 6) | (((d1 >> 6) & 1) << 5);
                let d0 = self.player[0].disabled_location;
                self.player[1].disabled_location |= (((d0 >> 5) & 1) << 6) | (((d0 >> 6) & 1) << 5);
                let dis = self.player[0].disabled_location | (self.player[1].disabled_location << 16);
                if dis != arg.previously_disabled_locations {
                    let m = self.new_message(MSG_FIELD_DISABLED);
                    self.msg(m).u32(dis);
                }
                true
            }
            _ => true,
        }
    }

    pub fn process_adjust(&mut self, arg: &mut units::Adjust) -> bool {
        let tp = self.infos.turn_player;
        match arg.step {
            0 => {
                self.core.re_adjust = false;
                self.refresh_effect_conditions();
                false
            }
            1 => {
                // win check (deck = 0 or LP = 0)
                if !self.core.force_turn_end {
                    let (mut winp, rea) = self.win_check_pub();
                    if self.is_flag(DUEL_RELAY) {
                        if winp == PLAYER_NONE {
                            let p1 = self.relay_check(0);
                            let p2 = self.relay_check(1);
                            if p1 && !p2 {
                                winp = 0;
                            } else if !p1 && p2 {
                                winp = 1;
                            } else if p1 && p2 {
                                winp = 5;
                                self.core.overdraw = [false, false];
                            }
                        } else if winp < PLAYER_NONE && self.relay_check(1 - winp as u8) {
                            winp = 5;
                            self.core.overdraw = [false, false];
                        }
                    }
                    self.emit_win_pub(winp, rea);
                }
                false
            }
            2 => {
                // disable check
                let mut p = tp;
                for _ in 0..2 {
                    let cards: Vec<CardId> = self.player[p as usize].list_mzone.iter().chain(self.player[p as usize].list_szone.iter()).flatten().copied().collect();
                    for c in cards {
                        self.add_to_disable_check_list(c);
                    }
                    p = 1 - p;
                }
                self.adjust_disable_check_list();
                self.emplace_process(units::RefreshLoc { step: 0, dis_count: 0, previously_disabled_locations: 0, current_disable_field_effect: None });
                false
            }
            3 => {
                // trap monster
                self.core.trap_monster_adjust_set = [CardSet::new(), CardSet::new()];
                for p in 0..2 {
                    for c in self.player[p].list_mzone.clone().into_iter().flatten() {
                        if c.get_type(self, None, 0, 2) & TYPE_TRAPMONSTER != 0 && c.is_affected_by_effect(self, EFFECT_DISABLE_TRAPMONSTER).is_some() {
                            self.core.trap_monster_adjust_set[p].insert(c);
                        }
                    }
                }
                if !self.core.trap_monster_adjust_set[0].is_empty() || !self.core.trap_monster_adjust_set[1].is_empty() {
                    self.core.re_adjust = true;
                    self.emplace_process(units::TrapMonsterAdjust { step: 0, oppo_selection: false, to_grave_set: CardSet::new() });
                }
                false
            }
            4 => {
                // control
                self.core.control_adjust_set = [CardSet::new(), CardSet::new()];
                let mut reason_cards = CardSet::new();
                for p in 0..2 {
                    for c in self.player[p].list_mzone.clone().into_iter().flatten() {
                        let cur = self.card(c).current.controler;
                        let (r, pe) = c.refresh_control_status(self);
                        if cur != r && c.is_capable_change_control(self) {
                            self.core.control_adjust_set[p].insert(c);
                            if let Some(pe) = pe {
                                if self.effect(pe).type_ as u32 & EFFECT_TYPE_SINGLE == 0 || self.effect(pe).condition().is_some() {
                                    let h = pe.get_handler(self);
                                    reason_cards.insert(h);
                                }
                            }
                        }
                    }
                }
                if !self.core.control_adjust_set[0].is_empty() || !self.core.control_adjust_set[1].is_empty() {
                    self.core.re_adjust = true;
                    let s1 = self.core.control_adjust_set[1 - tp as usize].clone();
                    self.get_control(s1, None, PLAYER_NONE as u8, tp, 0, 0, 0xff);
                    let s0 = self.core.control_adjust_set[tp as usize].clone();
                    self.get_control(s0, None, PLAYER_NONE as u8, 1 - tp, 0, 0, 0xff);
                    for rc in public_order(self, &reason_cards) {
                        let cnt = match self.core.readjust_map.iter_mut().find(|x| x.0 == rc) {
                            Some(x) => {
                                x.1 += 1;
                                x.1
                            }
                            None => {
                                self.core.readjust_map.push((rc, 1));
                                1
                            }
                        };
                        if cnt > 3 {
                            self.destroy(CardSet::from_card(rc), None, REASON_RULE, PLAYER_NONE as u8, PLAYER_NONE as u8, 0, 0);
                        }
                    }
                }
                self.core.last_control_changed_id = self.infos.field_id;
                false
            }
            5 => {
                // remove brainwashing
                if self.core.global_flag & GLOBALFLAG_BRAINWASHING_CHECK != 0 {
                    self.core.control_adjust_set = [CardSet::new(), CardSet::new()];
                    let mut eset = Vec::new();
                    self.filter_field_effect(EFFECT_REMOVE_BRAINWASHING, &mut eset, false);
                    self.core.remove_brainwashing = !eset.is_empty();
                    if self.core.remove_brainwashing {
                        for p in 0..2 {
                            for c in self.player[p].list_mzone.clone().into_iter().flatten() {
                                if c.is_affected_by_effect(self, EFFECT_REMOVE_BRAINWASHING).is_none() {
                                    continue;
                                }
                                for pe in self.card(c).single_effect().range(EFFECT_SET_CONTROL) {
                                    if self.effect(pe).condition().is_none() {
                                        let h = self.effect(pe).handler.unwrap();
                                        h.remove_effect(self, pe);
                                    }
                                }
                                if p as u8 != self.card(c).owner && c.is_capable_change_control(self) {
                                    self.core.control_adjust_set[p].insert(c);
                                }
                            }
                        }
                    }
                    if !self.core.control_adjust_set[0].is_empty() || !self.core.control_adjust_set[1].is_empty() {
                        self.core.re_adjust = true;
                        let s1 = self.core.control_adjust_set[1 - tp as usize].clone();
                        self.get_control(s1, None, PLAYER_NONE as u8, tp, 0, 0, 0xff);
                        let s0 = self.core.control_adjust_set[tp as usize].clone();
                        self.get_control(s0, None, PLAYER_NONE as u8, 1 - tp, 0, 0, 0xff);
                    }
                }
                arg.step = 7;
                false
            }
            8 => {
                if self.adjust_grant_effect() {
                    self.core.re_adjust = true;
                }
                false
            }
            9 => {
                if self.core.selfdes_disabled {
                    arg.step = 10;
                    return false;
                }
                self.adjust_self_destroy_set();
                false
            }
            10 => {
                // equip check
                let mut p = tp;
                let mut destroy_set = CardSet::new();
                for _ in 0..2 {
                    for i in 0..5 {
                        if let Some(c) = self.player[p as usize].list_szone[i] {
                            if let Some(et) = self.card(c).equiping_target() {
                                if c.is_affected_by_effect_t(self, EFFECT_EQUIP_LIMIT, Some(et)).is_none() {
                                    destroy_set.insert(c);
                                }
                            }
                        }
                    }
                    p = 1 - p;
                }
                if !destroy_set.is_empty() {
                    self.core.re_adjust = true;
                    self.destroy(destroy_set, None, REASON_RULE, PLAYER_NONE as u8, PLAYER_NONE as u8, 0, 0);
                }
                false
            }
            11 => {
                // position
                let mut p = tp;
                let mut pos_adjust = CardSet::new();
                for _ in 0..2 {
                    for c in self.player[p as usize].list_mzone.clone().into_iter().flatten() {
                        let dt = self.card(c).data.type_;
                        if (dt & TYPE_LINK != 0 && dt & TYPE_MONSTER != 0) || c.is_affected_by_effect(self, EFFECT_CANNOT_CHANGE_POS_E).is_some() {
                            continue;
                        }
                        let mut eset = Vec::new();
                        c.filter_effect(self, EFFECT_SET_POSITION, &mut eset, true);
                        if let Some(&last) = eset.last() {
                            let pos = last.get_value(self, 0) as u32;
                            let cc = self.card_mut(c);
                            if (pos & 0xff) != cc.current.position {
                                pos_adjust.insert(c);
                                cc.position_param = pos;
                                let jp = cc.is_status(STATUS_JUST_POS);
                                cc.set_status(STATUS_CONTINUOUS_POS, jp);
                            } else {
                                cc.set_status(STATUS_CONTINUOUS_POS, false);
                            }
                            cc.set_status(STATUS_JUST_POS, false);
                        }
                    }
                    p = 1 - p;
                }
                if !pos_adjust.is_empty() {
                    self.core.re_adjust = true;
                    let ng = self.new_group(pos_adjust);
                    self.emplace_process(units::ChangePos { step: 0, reason_player: PLAYER_NONE as u8, enable: true, oppo_selection: false, reason_effect: None, targets: Some(ng), to_grave_set: CardSet::new() });
                }
                false
            }
            12 => {
                // shuffle check
                for p in 0..2usize {
                    for c in self.player[p].list_hand.clone() {
                        let public = c.is_affected_by_effect(self, EFFECT_PUBLIC).is_some();
                        if !public && self.card(c).is_position(POS_FACEUP) != 0 {
                            self.core.shuffle_hand_check[p] = true;
                        }
                        let pos = if public { POS_FACEUP } else { POS_FACEDOWN } as u8;
                        let prev = self.card(c).current.position as u8;
                        self.card_mut(c).current.position = pos as u32;
                        if (prev as u32 & POS_FACEUP) != (pos as u32 & POS_FACEUP) {
                            let cc = self.card(c);
                            let (code, ctrl, loc, seq) = (cc.data.code, cc.current.controler, cc.current.location, cc.current.sequence);
                            let m = self.new_message(MSG_POS_CHANGE);
                            self.msg(m).u32(code).u8(ctrl).u8(loc).u8(seq).u8(prev).u8(pos);
                        }
                    }
                }
                if self.core.check_level > 0 {
                    return false;
                }
                if self.core.shuffle_hand_check[tp as usize] {
                    self.shuffle(tp, LOCATION_HAND as u8);
                }
                if self.core.shuffle_hand_check[1 - tp as usize] {
                    self.shuffle(1 - tp, LOCATION_HAND as u8);
                }
                false
            }
            13 => {
                // reverse_deck
                if self.core.global_flag & GLOBALFLAG_DECK_REVERSE_CHECK != 0 {
                    let mut eset = Vec::new();
                    self.filter_field_effect(EFFECT_REVERSE_DECK, &mut eset, false);
                    let reversed = !eset.is_empty();
                    if std::mem::replace(&mut self.core.deck_reversed, reversed) != reversed {
                        self.reverse_deck(0);
                        self.reverse_deck(1);
                        self.new_message(MSG_REVERSE_DECK);
                        if reversed {
                            for p in 0..2u8 {
                                if let Some(&ptop) = self.player[p as usize].list_main.last() {
                                    let (code, pos) = (self.card(ptop).data.code, self.card(ptop).current.position);
                                    let m = self.new_message(MSG_DECK_TOP);
                                    self.msg(m).u8(p).u32(0u32).u32(code).u32(pos);
                                }
                            }
                        }
                    }
                }
                false
            }
            14 => {
                // attack cancel
                let Some(attacker) = self.core.attacker else { return false };
                if attacker.is_affected_by_effect(self, EFFECT_UNSTOPPABLE_ATTACK).is_none() && self.card(attacker).is_status(STATUS_ATTACK_CANCELED) {
                    return false;
                }
                let ph = self.infos.phase as u32;
                if ph != PHASE_DAMAGE && ph != PHASE_DAMAGE_CAL {
                    if !attacker.is_capable_attack(self)
                        || self.card(attacker).current.controler != self.card(attacker).attack_controler
                        || self.card(attacker).fieldid_r != self.core.pre_field[0]
                    {
                        self.card_mut(attacker).set_status(STATUS_ATTACK_CANCELED, true);
                        return false;
                    }
                    if self.core.attack_rollback {
                        return false;
                    }
                    let mut fidset: Vec<u32> = self.player[1 - tp as usize].list_mzone.iter().flatten().map(|&c| self.card(c).fieldid_r).collect();
                    fidset.sort();
                    fidset.dedup();
                    if fidset != self.core.opp_mzone || !self.confirm_attack_target() {
                        self.core.attack_rollback = true;
                    }
                } else {
                    let ac = self.card(attacker);
                    let mut cancel = ac.current.location as u32 != LOCATION_MZONE
                        || ac.fieldid_r != self.core.pre_field[0]
                        || ac.current.controler != ac.attack_controler;
                    if !cancel && ac.current.position & POS_DEFENSE != 0 && attacker.is_affected_by_effect(self, EFFECT_DEFENSE_ATTACK).is_none() {
                        cancel = true;
                    }
                    if !cancel {
                        if let Some(t) = self.core.attack_target {
                            let tc = self.card(t);
                            if tc.current.location as u32 != LOCATION_MZONE || tc.current.controler != tc.attack_controler || tc.fieldid_r != self.core.pre_field[1] {
                                cancel = true;
                            }
                        }
                    }
                    if cancel {
                        self.card_mut(attacker).set_status(STATUS_ATTACK_CANCELED, true);
                    }
                }
                false
            }
            15 => {
                self.raise_event_card(None, EVENT_ADJUST, None, 0, PLAYER_NONE as u8, PLAYER_NONE as u8, 0);
                self.process_instant_event();
                false
            }
            16 => {
                if self.core.re_adjust {
                    arg.step = units::RESTART;
                    return false;
                }
                if self.core.check_level > 0 {
                    return true;
                }
                for p in 0..2u8 {
                    if self.core.shuffle_hand_check[p as usize] {
                        self.shuffle(p, LOCATION_HAND as u8);
                    }
                }
                for p in 0..2u8 {
                    if self.core.shuffle_deck_check[p as usize] {
                        self.shuffle(p, LOCATION_DECK as u8);
                    }
                }
                true
            }
            _ => true,
        }
    }

    pub fn process_startup(&mut self, arg: &mut units::Startup) -> bool {
        match arg.step {
            0 => {
                self.core.shuffle_hand_check = [false, false];
                self.core.shuffle_deck_check = [false, false];
                self.raise_event_card(None, EVENT_STARTUP, None, 0, 0, 0, 0);
                self.process_instant_event();
                false
            }
            1 => {
                for p in 0..2usize {
                    self.core.shuffle_hand_check[p] = false;
                    self.core.shuffle_deck_check[p] = false;
                    let sc = self.player[p].start_count;
                    if sc > 0 {
                        self.draw(None, REASON_RULE, PLAYER_NONE as u8, p as u8, sc as u16);
                    }
                    for l in 0..self.player[p].extra_lists_main.len() {
                        for _ in 0..sc {
                            let Some(c) = self.player[p].extra_lists_main[l].pop() else { break };
                            self.player[p].extra_lists_hand[l].push(c);
                            let n = self.player[p].extra_lists_hand[l].len() as u32;
                            let cc = self.card_mut(c);
                            cc.current.controler = p as u8;
                            cc.current.location = LOCATION_HAND as u8;
                            cc.current.sequence = n - 1;
                            cc.current.position = POS_FACEDOWN;
                        }
                    }
                }
                self.emplace_process(units::Turn { step: 0, turn_player: 0, has_performed_second_battle_phase: false });
                true
            }
            _ => true,
        }
    }

    pub fn process_sort_deck(&mut self, arg: &mut units::SortDeck) -> bool {
        let tpl = arg.target_player as usize;
        let mut count = arg.count as usize;
        if count > self.player[tpl].list_main.len() {
            count = self.player[tpl].list_main.len();
        }
        match arg.step {
            0 => {
                let list = &self.player[tpl].list_main;
                self.core.select_cards = if arg.bottom { list[..count].to_vec() } else { list.iter().rev().take(count).copied().collect() };
                self.emplace_process(units::SortCard { step: 0, playerid: arg.sort_player, is_chain: false });
                false
            }
            1 => {
                if self.returns.at_i8(0) == -1 || count == 0 {
                    return true;
                }
                let mut tc: Vec<Option<CardId>> = vec![None; self.core.select_cards.len()];
                for i in 0..count {
                    tc[self.returns.at_u8(i) as usize] = Some(self.core.select_cards[i]);
                }
                for i in 0..count {
                    let pc = if arg.bottom { tc[i] } else { tc[count - i - 1] }.unwrap();
                    let loc = pc.get_info_location(self);
                    let m = self.new_message(MSG_MOVE);
                    self.msg(m).u32(0u32).loc(loc);
                    let seq = self.card(pc).current.sequence as usize;
                    self.player[tpl].list_main.remove(seq);
                    if arg.bottom {
                        self.player[tpl].list_main.insert(0, pc);
                    } else {
                        self.player[tpl].list_main.push(pc);
                    }
                    self.reset_sequence(tpl as u8, LOCATION_DECK as u8);
                    let loc = pc.get_info_location(self);
                    let reason = self.card(pc).current.reason;
                    self.msg(m).loc(loc).u32(reason);
                }
                if self.core.global_flag & GLOBALFLAG_DECK_REVERSE_CHECK != 0 {
                    let ptop = *self.player[tpl].list_main.last().unwrap();
                    if self.core.deck_reversed || self.card(ptop).current.position == POS_FACEUP_DEFENSE {
                        let (code, pos) = (self.card(ptop).data.code, self.card(ptop).current.position);
                        let m = self.new_message(MSG_DECK_TOP);
                        self.msg(m).u8(tpl as u8).u32(0u32).u32(code).u32(pos);
                    }
                }
                true
            }
            _ => true,
        }
    }

    pub fn process_discard_hand(&mut self, arg: &mut units::DiscardHand) -> bool {
        match arg.step {
            0 => {
                let m = self.new_message(MSG_HINT);
                let hint: u64 = if arg.reason & REASON_DISCARD != 0 { 501 } else { 504 };
                self.msg(m).u8(HINT_SELECTMSG).u8(arg.playerid).u64(hint);
                self.emplace_process(units::SelectCard { step: 0, playerid: arg.playerid, cancelable: false, min: arg.min, max: arg.max });
                false
            }
            1 => {
                if self.return_cards.list.is_empty() {
                    self.returns.set_i32(0, 0);
                } else {
                    let set = CardSet::from_iter(self.return_cards.list.iter().copied());
                    let (re, rp) = (self.core.reason_effect, self.core.reason_player);
                    self.send_to(set, re, arg.reason, rp, PLAYER_NONE as u8, LOCATION_GRAVE as u16, 0, POS_FACEUP as u8, false);
                }
                true
            }
            _ => true,
        }
    }

    pub fn process_attack_disable(&mut self, arg: &mut units::AttackDisable) -> bool {
        match arg.step {
            0 => {
                let Some(attacker) = self.core.attacker else {
                    self.returns.set_i32(0, 0);
                    return true;
                };
                let re = self.core.reason_effect;
                if self.core.effect_damage_step != 0
                    || self.card(attacker).fieldid_r != self.core.pre_field[0]
                    || self.card(attacker).current.location as u32 != LOCATION_MZONE
                    || !attacker.is_capable_attack(self)
                    || !attacker.is_affect_by_effect(self, re)
                    || attacker.is_affected_by_effect(self, EFFECT_UNSTOPPABLE_ATTACK).is_some()
                {
                    self.returns.set_i32(0, 0);
                    return true;
                }
                let pe = self.new_effect();
                {
                    let e = self.effect_mut(pe);
                    e.code = EFFECT_ATTACK_DISABLED;
                    e.type_ = EFFECT_TYPE_SINGLE as u16;
                }
                attacker.add_effect(self, pe);
                self.card_mut(attacker).set_status(STATUS_ATTACK_CANCELED, true);
                let rp = self.core.reason_player;
                self.raise_event_card(Some(attacker), EVENT_ATTACK_DISABLED, re, 0, rp, PLAYER_NONE as u8, 0);
                self.process_instant_event();
                false
            }
            1 => {
                self.returns.set_i32(0, 1);
                true
            }
            _ => true,
        }
    }

    pub fn process_select_fusion(&mut self, arg: &mut units::SelectFusion) -> bool {
        let pcard = arg.pcard.unwrap();
        match arg.step {
            0 => {
                let mut eset = Vec::new();
                let fm = arg.fusion_materials.map(|g| self.group(g).clone());
                let forced = arg.forced_materials.map(|g| self.group(g).clone());
                pcard.fusion_filter_valid(self, fm.as_ref(), forced.as_ref(), arg.chkf, &mut eset);
                self.core.select_effects.clear();
                self.core.select_options.clear();
                if eset.is_empty() {
                    return true;
                }
                for pe in eset {
                    self.core.select_effects.push(Some(pe));
                    let d = self.effect(pe).description;
                    self.core.select_options.push(d);
                }
                if self.core.select_options.len() == 1 {
                    self.returns.set_i32(0, 0);
                } else {
                    self.emplace_process(units::SelectOption { step: 0, playerid: arg.playerid });
                }
                false
            }
            1 => {
                self.core.fusion_materials.clear();
                let idx = self.returns.at_i32(0) as usize;
                let Some(sel) = self.core.select_effects[idx] else { return true };
                let ev = Event {
                    event_cards: arg.fusion_materials.map(|g| self.group(g).clone()),
                    reason_effect: Some(sel),
                    reason_player: arg.playerid,
                    ..Default::default()
                };
                self.core.sub_solving_event.push_back(ev);
                let forced = arg.forced_materials.map(|g| self.group(g).clone());
                self.add_param(Value::from_group(&forced));
                self.add_param(Value::Int(arg.chkf as i64));
                self.add_param(Value::effect(self.core.reason_effect));
                self.emplace_process(units::ExecuteOperation { step: 0, triggering_player: arg.playerid, shuffle_check_was_disabled: false, triggering_effect: sel });
                false
            }
            _ => true,
        }
    }

    pub fn process_refresh_relay(&mut self, arg: &mut units::RefreshRelay) -> bool {
        match arg.step {
            0 | 1 => {
                let p = arg.step as u8;
                if self.player[p as usize].recharge {
                    self.next_player(p);
                }
                false
            }
            _ => true,
        }
    }

    pub fn win_check_pub(&mut self) -> (u32, u32) {
        self.win_check_impl()
    }
    pub fn emit_win_pub(&mut self, winp: u32, rea: u32) {
        self.emit_win_impl(winp, rea)
    }
}
