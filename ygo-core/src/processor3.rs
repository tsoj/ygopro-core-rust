//! The processor (`processor.cpp` part 3: the battle phase and battle damage).

use crate::card::public_order;
use crate::constants::*;
use crate::duel::{Chain, Duel};
use crate::types::*;
use crate::units;

const NOLOC: LocInfo = LocInfo { controler: 0, location: 0, sequence: 0, position: 0 };

impl Duel {
    fn hint_event_both(&mut self, code: u64) {
        let m = self.new_message(MSG_HINT);
        self.msg(m).u8(HINT_EVENT).u8(0u8).u64(code);
        let m = self.new_message(MSG_HINT);
        self.msg(m).u8(HINT_EVENT).u8(1u8).u64(code);
    }
    fn point_event_unit(&mut self, skip_new: bool) {
        self.emplace_process(units::PointEvent { step: 0, skip_trigger: false, skip_freechain: false, skip_new });
    }
    fn clear_chaining_status(&mut self) {
        for ch in self.core.current_chain.clone() {
            let h = ch.triggering_effect.unwrap().get_handler(self);
            self.card_mut(h).set_status(STATUS_CHAINING, false);
        }
    }

    pub fn process_battle_command(&mut self, arg: &mut units::BattleCommand) -> bool {
        let tp = self.infos.turn_player;
        match arg.step {
            0 => {
                self.core.select_chains.clear();
                self.nil_event.event_code = EVENT_FREE_CHAIN;
                if !self.core.chain_attack {
                    self.core.chain_attacker_id = 0;
                    self.core.chain_attack_target = None;
                }
                self.core.attack_player = 0;
                self.core.attacker = None;
                self.core.attack_target = None;
                let skip = self.is_player_affected_by_effect(tp, EFFECT_SKIP_BP);
                if skip.is_some() || self.core.force_turn_end {
                    arg.step = 41;
                    arg.phase_to_change_to = 2;
                    arg.repeat_battle_phase = self.is_player_affected_by_effect(tp, EFFECT_BP_TWICE).is_some();
                    if self.core.force_turn_end || !self.effect(skip.unwrap()).has_value() {
                        self.reset_phase(PHASE_BATTLE_STEP);
                        self.adjust_all();
                        self.infos.phase = PHASE_BATTLE as u16;
                        self.emplace_process(units::PhaseEvent { step: 0, phase: PHASE_BATTLE as u16, is_opponent: false, priority_passed: false });
                    } else {
                        self.core.hint_timing[tp as usize] = 0;
                        self.reset_phase(PHASE_BATTLE);
                        self.adjust_all();
                    }
                    return false;
                }
                let ne = self.nil_event.clone();
                for pe in self.field_effects.activate_effect.range(EVENT_FREE_CHAIN) {
                    pe.set_activate_location(self);
                    if pe.is_activateable(self, tp, &ne, false, false, false, false, false) && pe.get_speed(self) > 1 {
                        self.core.select_chains.push_back(Chain { triggering_effect: Some(pe), ..Default::default() });
                    }
                }
                for pe in self.field_effects.quick_o_effect.range(EVENT_FREE_CHAIN) {
                    pe.set_activate_location(self);
                    if pe.is_activateable(self, tp, &ne, false, false, false, false, false) {
                        self.core.select_chains.push_back(Chain { triggering_effect: Some(pe), ..Default::default() });
                    }
                }
                for pe in self.field_effects.continuous_effect.range(EVENT_FREE_CHAIN) {
                    if pe.get_handler_player(self) == tp && pe.is_activateable(self, tp, &ne, false, false, false, false, false) {
                        self.core.select_chains.push_back(Chain { triggering_effect: Some(pe), ..Default::default() });
                    }
                }
                self.core.attackable_cards.clear();
                let mut first_attack = Vec::new();
                let mut must_attack = Vec::new();
                if self.is_player_affected_by_effect(tp, EFFECT_CANNOT_ATTACK_ANNOUNCE).is_none() {
                    for c in self.player[tp as usize].list_mzone.clone().into_iter().flatten() {
                        if !c.is_capable_attack_announce(self, tp) {
                            continue;
                        }
                        let chain_attack = self.core.chain_attack && self.core.chain_attacker_id == self.card(c).fieldid;
                        let mut cv = Vec::new();
                        self.get_attack_target(c, &mut cv, chain_attack, true, None);
                        if cv.is_empty() && self.card(c).direct_attackable == 0 {
                            continue;
                        }
                        self.core.attackable_cards.push(c);
                        if c.is_affected_by_effect(self, EFFECT_FIRST_ATTACK).is_some() {
                            first_attack.push(c);
                        }
                        if c.is_affected_by_effect(self, EFFECT_MUST_ATTACK).is_some() {
                            must_attack.push(c);
                        }
                    }
                    if !first_attack.is_empty() {
                        self.core.attackable_cards = first_attack;
                    }
                }
                self.core.to_m2 = !self.is_flag(DUEL_NO_MAIN_PHASE_2);
                self.core.to_ep = true;
                if !must_attack.is_empty() || self.is_player_affected_by_effect(tp, EFFECT_CANNOT_M2).is_some() || self.core.force_turn_end {
                    self.core.to_m2 = false;
                }
                if !must_attack.is_empty() {
                    self.core.to_ep = false;
                }
                self.core.attack_cancelable = true;
                self.core.attack_cost_paid = 0;
                self.emplace_process(units::SelectBattleCmd { step: 0, playerid: tp });
                false
            }
            1 => {
                let r = self.returns.at_i32(0);
                let ctype = r & 0xffff;
                let sel = (r >> 16) as usize;
                if arg.forced_attack_done || (ctype != 0 && ctype != 1) {
                    arg.step = 39;
                    arg.phase_to_change_to = ctype as u16;
                    let m = self.new_message(MSG_HINT);
                    self.msg(m).u8(HINT_EVENT).u8(1 - tp).u64(29u64);
                    self.core.select_chains.clear();
                    self.core.hint_timing[tp as usize] = TIMING_BATTLE_STEP_END;
                    self.emplace_process(units::QuickEffect { step: 0, skip_freechain: false, is_opponent: false, priority_player: 1 - tp });
                    self.infos.priorities[tp as usize] = 1;
                    self.infos.priorities[1 - tp as usize] = 0;
                    false
                } else if ctype == 0 {
                    let pe = self.core.select_chains[sel].triggering_effect.unwrap();
                    if self.effect(pe).type_ as u32 & EFFECT_TYPE_CONTINUOUS != 0 {
                        self.core.select_chains.clear();
                        let hp = pe.get_handler_player(self);
                        let ne = self.nil_event.clone();
                        self.solve_continuous(hp, pe, &ne);
                        arg.step = 14;
                        return false;
                    }
                    let phandler = pe.get_handler(self);
                    let mut ch = self.core.select_chains.remove(sel).unwrap();
                    ch.flag = 0;
                    ch.chain_id = self.infos.field_id as u16;
                    self.infos.field_id += 1;
                    ch.evt.event_code = self.effect(pe).code;
                    ch.evt.event_player = PLAYER_NONE as u8;
                    ch.evt.event_value = 0;
                    ch.evt.event_cards = None;
                    ch.evt.reason = 0;
                    ch.evt.reason_effect = None;
                    ch.evt.reason_player = PLAYER_NONE as u8;
                    self.set_triggering_state(&mut ch, phandler);
                    ch.triggering_player = tp;
                    self.core.new_chains.push_back(ch);
                    self.card_mut(phandler).set_status(STATUS_CHAINING, true);
                    pe.dec_count(self, tp as u32);
                    self.core.select_chains.clear();
                    self.emplace_process(units::AddChain { step: 0, is_activated_effect: false });
                    self.emplace_process(units::QuickEffect { step: 0, skip_freechain: false, is_opponent: false, priority_player: 1 - tp });
                    self.infos.priorities = [0, 0];
                    self.core.select_chains.clear();
                    false
                } else {
                    arg.step = 2;
                    arg.attack_announce_failed = false;
                    if !arg.forced_attack {
                        self.core.attacker = Some(self.core.attackable_cards[sel]);
                    }
                    let attacker = self.core.attacker.unwrap();
                    self.card_mut(attacker).set_status(STATUS_ATTACK_CANCELED, false);
                    let ctrl = self.card(attacker).current.controler;
                    self.card_mut(attacker).attack_controler = ctrl;
                    self.core.pre_field[0] = self.card(attacker).fieldid_r;
                    if self.core.chain_attack && self.core.chain_attacker_id != self.card(attacker).fieldid {
                        self.core.chain_attack = false;
                        self.core.chain_attacker_id = 0;
                    }
                    let mut eset = Vec::new();
                    self.core.tpchain.clear();
                    self.filter_player_effect(tp, EFFECT_ATTACK_COST, &mut eset, false);
                    attacker.filter_effect(self, EFFECT_ATTACK_COST, &mut eset, true);
                    for pe in eset {
                        if self.effect(pe).operation.is_some() {
                            self.core.tpchain.push_back(Chain { triggering_effect: Some(pe), ..Default::default() });
                            self.core.attack_cancelable = false;
                        }
                    }
                    if self.core.tpchain.len() > 1 {
                        self.emplace_process(units::SortChain { step: 0, playerid: tp });
                        arg.step = 13;
                    } else if self.core.tpchain.len() == 1 {
                        let ne = self.nil_event.clone();
                        self.core.sub_solving_event.push_back(ne);
                        let te = self.core.tpchain.front().unwrap().triggering_effect.unwrap();
                        self.emplace_process(units::ExecuteOperation { step: 0, triggering_player: tp, shuffle_check_was_disabled: false, triggering_effect: te });
                        self.adjust_all();
                    }
                    false
                }
            }
            2 => {
                self.core.chain_limit.clear();
                self.clear_chaining_status();
                self.emplace_process(units::SolveChain { step: 0, skip_trigger: false, skip_freechain: false, skip_new: false, backed_up_operation: None });
                arg.step = units::RESTART;
                false
            }
            3 => {
                arg.is_replaying_attack = false;
                let attacker = self.core.attacker.unwrap();
                if self.card(attacker).is_status(STATUS_ATTACK_CANCELED) {
                    arg.attack_announce_failed = true;
                    arg.step = 6;
                    return false;
                }
                // Cancel the attack cost
                if self.core.attack_cost_paid != 1 && !self.core.attack_cancelable {
                    if arg.forced_attack {
                        self.returns.set_i32(0, 2);
                        arg.step = 0;
                    } else {
                        arg.step = units::RESTART;
                    }
                    return false;
                }
                if arg.forced_attack {
                    arg.step = 6;
                }
                false
            }
            4 => {
                // select attack target (replay start point)
                self.core.attack_player = 0;
                self.core.select_cards.clear();
                self.return_cards.clear();
                arg.must_attack_map.clear();
                let attacker = self.core.attacker.unwrap();
                let mut sc = Vec::new();
                let ca = self.core.chain_attack;
                let atype = self.get_attack_target(attacker, &mut sc, ca, true, Some(&mut arg.must_attack_map));
                self.core.select_cards = sc;
                if self.card(attacker).direct_attackable != 0 {
                    if self.core.select_cards.is_empty() {
                        self.returns.set_i32(0, -2);
                        arg.step = 5;
                        return false;
                    }
                    if self.is_player_affected_by_effect(tp, EFFECT_PATRICIAN_OF_DARKNESS).is_some() {
                        self.emplace_process(units::SelectEffectYesNo { step: 0, playerid: 1 - tp, pcard: Some(attacker), description: 31 });
                    } else {
                        self.emplace_process(units::SelectYesNo { step: 0, playerid: tp, description: 31 });
                    }
                    return false;
                }
                if self.core.select_cards.is_empty() {
                    arg.attack_announce_failed = true;
                    arg.step = 6;
                    return false;
                }
                let mut distinct: Vec<EffectId> = arg.must_attack_map.iter().map(|x| x.0).collect();
                distinct.sort();
                distinct.dedup();
                let different = distinct.len();
                let patrician = self.is_player_affected_by_effect(tp, EFFECT_PATRICIAN_OF_DARKNESS).is_some();
                if patrician || (atype == 3 && different != 1) {
                    if self.core.select_cards.len() == 1 {
                        let c = self.core.select_cards[0];
                        self.return_cards.list.push(c);
                    } else {
                        let loc = attacker.get_info_location(self);
                        let m = self.new_message(MSG_CARD_SELECTED);
                        self.msg(m).u32(1u32).loc(loc);
                        let m = self.new_message(MSG_HINT);
                        self.msg(m).u8(HINT_SELECTMSG).u8(1 - tp).u64(549u64);
                        self.emplace_process(units::SelectCard { step: 0, playerid: 1 - tp, cancelable: false, min: 1, max: 1 });
                        if !patrician && atype == 3 && arg.must_attack_map.len() != different {
                            arg.step = 15;
                            return false;
                        }
                    }
                } else {
                    let m = self.new_message(MSG_HINT);
                    self.msg(m).u8(HINT_SELECTMSG).u8(tp).u64(549u64);
                    let cancelable = self.core.attack_cancelable;
                    self.emplace_process(units::SelectCard { step: 0, playerid: tp, cancelable, min: 1, max: 1 });
                }
                arg.step = 5;
                false
            }
            5 => {
                // the answer of "direct attack or not"
                if self.returns.at_i32(0) != 0 {
                    self.returns.set_i32(0, -2);
                } else if !self.core.select_cards.is_empty() {
                    let opposel = self.is_player_affected_by_effect(tp, EFFECT_PATRICIAN_OF_DARKNESS).is_some();
                    let sel_player = if opposel { 1 - tp } else { tp };
                    let cancelable = self.core.attack_cancelable && !opposel;
                    let m = self.new_message(MSG_HINT);
                    self.msg(m).u8(HINT_SELECTMSG).u8(sel_player).u64(549u64);
                    self.emplace_process(units::SelectCard { step: 0, playerid: sel_player, cancelable, min: 1, max: 1 });
                } else {
                    arg.attack_announce_failed = true;
                    arg.step = 6;
                }
                false
            }
            6 => {
                if self.return_cards.canceled {
                    if arg.is_replaying_attack {
                        arg.step = 12;
                        return false;
                    }
                    if arg.forced_attack {
                        self.returns.set_i32(0, 2);
                        arg.step = 0;
                    } else {
                        arg.step = units::RESTART;
                    }
                    return false;
                }
                self.core.attack_target = if self.returns.at_i32(0) == -2 { None } else { Some(self.return_cards.list[0]) };
                self.core.pre_field[1] = match self.core.attack_target {
                    Some(t) => self.card(t).fieldid_r,
                    None => 0,
                };
                false
            }
            7 => {
                let attacker = self.core.attacker.unwrap();
                if !arg.is_replaying_attack {
                    self.core.phase_action = true;
                    self.core.attack_state_count[tp as usize] = self.core.attack_state_count[tp as usize].wrapping_add(1);
                    self.check_card_counter(attacker, ACTIVITY_ATTACK, tp as i32);
                    let c = self.card_mut(attacker);
                    c.attack_announce_count = c.attack_announce_count.wrapping_add(1);
                }
                if arg.attack_announce_failed {
                    let c = self.card_mut(attacker);
                    c.announce_count = c.announce_count.wrapping_add(1);
                    self.core.chain_attack = false;
                    if arg.forced_attack {
                        self.returns.set_i32(0, 2);
                        arg.step = 0;
                    } else {
                        arg.step = units::RESTART;
                    }
                }
                false
            }
            8 => {
                self.core.attack_cancelable = true;
                let attacker = self.core.attacker.unwrap();
                let aloc = attacker.get_info_location(self);
                let m = self.new_message(MSG_ATTACK);
                self.msg(m).loc(aloc);
                if let Some(t) = self.core.attack_target {
                    self.raise_single_event(t, None, EVENT_BE_BATTLE_TARGET, None, 0, 0, 1 - tp, 0);
                    self.raise_event_card(Some(t), EVENT_BE_BATTLE_TARGET, None, 0, 0, 1 - tp, 0);
                    let tloc = t.get_info_location(self);
                    self.msg(m).loc(tloc);
                } else {
                    self.msg(m).loc(NOLOC);
                }
                self.core.attack_rollback = false;
                self.core.opp_mzone.clear();
                for c in self.player[1 - tp as usize].list_mzone.clone().into_iter().flatten() {
                    let f = self.card(c).fieldid_r;
                    if !self.core.opp_mzone.contains(&f) {
                        let pos = self.core.opp_mzone.partition_point(|&x| x < f);
                        self.core.opp_mzone.insert(pos, f);
                    }
                }
                if !arg.is_replaying_attack {
                    self.raise_single_event(attacker, None, EVENT_ATTACK_ANNOUNCE, None, 0, 0, tp, 0);
                    self.raise_event_card(Some(attacker), EVENT_ATTACK_ANNOUNCE, None, 0, 0, tp, 0);
                }
                let ctrl = self.card(attacker).current.controler;
                self.card_mut(attacker).attack_controler = ctrl;
                self.core.pre_field[0] = self.card(attacker).fieldid_r;
                self.process_single_event();
                self.process_instant_event();
                self.core.hint_timing[tp as usize] = TIMING_ATTACK;
                self.point_event_unit(false);
                false
            }
            9 => {
                let attacker = self.core.attacker.unwrap();
                if self.is_player_affected_by_effect(tp, EFFECT_SKIP_BP).is_some() || self.card(attacker).is_status(STATUS_ATTACK_CANCELED) || self.core.attack_rollback {
                    arg.step = 10;
                    return false;
                }
                self.hint_event_both(24);
                self.core.hint_timing = [TIMING_BATTLE_PHASE, TIMING_BATTLE_PHASE];
                self.emplace_process(units::PointEvent { step: 30, skip_trigger: false, skip_freechain: false, skip_new: false });
                false
            }
            10 => {
                if self.returns.at_i32(0) != 0 {
                    arg.step = 8;
                } else {
                    self.adjust_all();
                }
                false
            }
            11 => {
                let attacker = self.core.attacker.unwrap();
                if attacker.is_affected_by_effect(self, EFFECT_ATTACK_DISABLED).is_some() {
                    attacker.reset(self, EFFECT_ATTACK_DISABLED, RESET_CODE);
                    self.new_message(MSG_ATTACK_DISABLED);
                    self.card_mut(attacker).set_status(STATUS_ATTACK_CANCELED, true);
                }
                if self.is_player_affected_by_effect(tp, EFFECT_SKIP_BP).is_some() || self.card(attacker).is_status(STATUS_ATTACK_CANCELED) {
                    arg.step = 12;
                    return false;
                }
                // go to damage step
                if !self.core.attack_rollback {
                    let c = self.card_mut(attacker);
                    c.announce_count = c.announce_count.wrapping_add(1);
                    let at = self.core.attack_target.map(|t| (t, self.card(t).fieldid_r));
                    self.card_mut(attacker).announced_cards.addcard(at);
                    self.attack_all_target_check();
                    arg.step = 18;
                    return false;
                }
                let mut cv = Vec::new();
                let ca = self.core.chain_attack;
                self.get_attack_target(attacker, &mut cv, ca, true, None);
                if cv.is_empty() && self.card(attacker).direct_attackable == 0 {
                    arg.step = 12;
                    return false;
                }
                // replay
                if self.is_flag(DUEL_STORE_ATTACK_REPLAYS) && !self.core.chain_attack {
                    self.returns.set_i32(0, 0);
                } else if attacker.is_affected_by_effect(self, EFFECT_MUST_ATTACK).is_none() {
                    self.emplace_process(units::SelectYesNo { step: 0, playerid: tp, description: 30 });
                } else {
                    self.returns.set_i32(0, 1);
                    self.core.attack_cancelable = false;
                }
                false
            }
            12 => {
                // answer of "replay or not"
                if self.returns.at_i32(0) != 0 {
                    arg.is_replaying_attack = true;
                    arg.attack_announce_failed = false;
                    arg.step = 3;
                }
                false
            }
            13 => {
                let attacker = self.core.attacker.unwrap();
                if self.card(attacker).fieldid_r == self.core.pre_field[0]
                    && (!(self.is_flag(DUEL_STORE_ATTACK_REPLAYS) && !self.core.chain_attack) || self.card(attacker).is_status(STATUS_ATTACK_CANCELED))
                {
                    let c = self.card_mut(attacker);
                    c.announce_count = c.announce_count.wrapping_add(1);
                    let at = self.core.attack_target.map(|t| (t, self.card(t).fieldid_r));
                    self.card_mut(attacker).announced_cards.addcard(at);
                    self.attack_all_target_check();
                }
                self.core.chain_attack = false;
                if arg.forced_attack {
                    self.returns.set_i32(0, 2);
                    arg.step = 0;
                } else {
                    arg.step = units::RESTART;
                }
                self.reset_phase(PHASE_DAMAGE);
                self.adjust_all();
                false
            }
            14 => {
                for ch in self.core.tpchain.clone() {
                    let ne = self.nil_event.clone();
                    self.core.sub_solving_event.push_back(ne);
                    self.emplace_process(units::ExecuteOperation { step: 0, triggering_player: tp, shuffle_check_was_disabled: false, triggering_effect: ch.triggering_effect.unwrap() });
                    self.adjust_all();
                }
                self.core.tpchain.clear();
                arg.step = 2;
                false
            }
            15 => {
                self.adjust_instant();
                self.point_event_unit(false);
                arg.step = units::RESTART;
                false
            }
            16 => {
                // EFFECT_MUST_ATTACK_MONSTER where an effect affects more than 1 monster
                let selected = self.return_cards.list[0];
                let key = arg.must_attack_map.iter().find(|x| x.1 == selected).map(|x| x.0);
                let range: Vec<CardId> = arg.must_attack_map.iter().filter(|x| Some(x.0) == key).map(|x| x.1).collect();
                if range.len() < 2 {
                    self.core.attack_target = Some(selected);
                    self.core.pre_field[1] = self.card(selected).fieldid_r;
                    arg.step = 5;
                    return false;
                }
                self.core.select_cards = range;
                let m = self.new_message(MSG_HINT);
                self.msg(m).u8(HINT_SELECTMSG).u8(tp).u64(549u64);
                let cancelable = self.core.attack_cancelable;
                self.emplace_process(units::SelectCard { step: 0, playerid: tp, cancelable, min: 1, max: 1 });
                arg.step = 5;
                false
            }
            19 => {
                self.infos.phase = PHASE_DAMAGE as u16;
                self.core.chain_attack = false;
                arg.is_replaying_attack = false;
                self.core.damage_calculated = false;
                self.core.selfdes_disabled = true;
                self.core.flip_delayed = true;
                let attacker = self.core.attacker.unwrap();
                let ctrl = self.card(attacker).current.controler;
                self.card_mut(attacker).attack_controler = ctrl;
                self.core.pre_field[0] = self.card(attacker).fieldid_r;
                if let Some(t) = self.core.attack_target {
                    let tc = self.card(t).current.controler;
                    self.card_mut(t).attack_controler = tc;
                    self.core.pre_field[1] = self.card(t).fieldid_r;
                } else {
                    self.core.pre_field[1] = 0;
                }
                let c = self.card_mut(attacker);
                c.attacked_count = c.attacked_count.wrapping_add(1);
                let at = self.core.attack_target.map(|t| (t, self.card(t).fieldid_r));
                self.card_mut(attacker).attacked_cards.addcard(at);
                self.core.battled_count[tp as usize] = self.core.battled_count[tp as usize].wrapping_add(1);
                self.adjust_all();
                false
            }
            20 => {
                // start of PHASE_DAMAGE
                self.new_message(MSG_DAMAGE_STEP_START);
                let attacker = self.core.attacker.unwrap();
                self.raise_single_event(attacker, None, EVENT_BATTLE_START, None, 0, 0, 0, 0);
                if let Some(t) = self.core.attack_target {
                    self.raise_single_event(t, None, EVENT_BATTLE_START, None, 0, 0, 0, 1);
                }
                self.raise_event_card(None, EVENT_BATTLE_START, None, 0, 0, 0, 0);
                self.process_single_event();
                self.process_instant_event();
                arg.previous_point_event_had_any_trigger_to_resolve = false;
                let any = !self.core.new_fchain.is_empty() || !self.core.new_ochain.is_empty();
                if !self.is_flag(DUEL_6_STEP_BATLLE_STEP) || any {
                    arg.previous_point_event_had_any_trigger_to_resolve = any;
                    self.hint_event_both(40);
                    let sn = self.is_flag(DUEL_SINGLE_CHAIN_IN_DAMAGE_SUBSTEP);
                    self.point_event_unit(sn);
                }
                false
            }
            21 => {
                let attacker = self.core.attacker.unwrap();
                if self.card(attacker).is_status(STATUS_ATTACK_CANCELED) {
                    arg.step = 33;
                    return false;
                }
                let Some(t) = self.core.attack_target else { return false };
                let ap = self.card(attacker).current.position;
                self.card_mut(attacker).temp.position = ap;
                let tpos = self.card(t).current.position;
                self.card_mut(t).temp.position = tpos;
                if self.card(t).is_position(POS_FACEDOWN) != 0 {
                    self.change_position_one(t, None, PLAYER_NONE as u8, (tpos >> 1) as u8, 0, true);
                    self.adjust_all();
                }
                false
            }
            22 => {
                let attacker = self.core.attacker.unwrap();
                self.raise_single_event(attacker, None, EVENT_BATTLE_CONFIRM, None, 0, 0, 0, 0);
                if let Some(t) = self.core.attack_target {
                    if self.card(t).temp.position & POS_FACEDOWN != 0 {
                        self.core.pre_field[1] = self.card(t).fieldid_r;
                    }
                    self.raise_single_event(t, None, EVENT_BATTLE_CONFIRM, None, 0, 0, 0, 1);
                }
                self.raise_event_card(None, EVENT_BATTLE_CONFIRM, None, 0, 0, 0, 0);
                self.process_single_event();
                self.process_instant_event();
                if !self.is_flag(DUEL_6_STEP_BATLLE_STEP)
                    || !arg.previous_point_event_had_any_trigger_to_resolve
                    || !self.core.new_fchain.is_empty()
                    || !self.core.new_ochain.is_empty()
                {
                    self.hint_event_both(41);
                    self.core.hint_timing[tp as usize] = TIMING_DAMAGE_STEP;
                    let sn = self.is_flag(DUEL_SINGLE_CHAIN_IN_DAMAGE_SUBSTEP);
                    self.point_event_unit(sn);
                }
                false
            }
            23 => {
                let attacker = self.core.attacker.unwrap();
                if self.card(attacker).is_status(STATUS_ATTACK_CANCELED) {
                    arg.step = 33;
                    return false;
                }
                self.infos.phase = PHASE_DAMAGE_CAL as u16;
                self.adjust_all();
                false
            }
            24 => {
                self.calculate_battle_damage(None, None, None);
                let attacker = self.core.attacker.unwrap();
                self.raise_single_event(attacker, None, EVENT_PRE_DAMAGE_CALCULATE, None, 0, 0, 0, 0);
                if let Some(t) = self.core.attack_target {
                    self.raise_single_event(t, None, EVENT_PRE_DAMAGE_CALCULATE, None, 0, 0, 0, 1);
                }
                self.raise_event_card(None, EVENT_PRE_DAMAGE_CALCULATE, None, 0, 0, 0, 0);
                self.process_single_event();
                self.process_instant_event();
                self.hint_event_both(42);
                self.core.hint_timing[tp as usize] = TIMING_DAMAGE_CAL;
                let sn = self.is_flag(DUEL_SINGLE_CHAIN_IN_DAMAGE_SUBSTEP);
                self.point_event_unit(sn);
                false
            }
            25 => {
                let attacker = self.core.attacker.unwrap();
                if self.card(attacker).is_status(STATUS_ATTACK_CANCELED) {
                    self.reset_phase(PHASE_DAMAGE_CAL);
                    self.adjust_all();
                    self.infos.phase = PHASE_DAMAGE as u16;
                    arg.step = 33;
                }
                false
            }
            26 => {
                // Duel.CalculateDamage() goes here
                let attacker = self.core.attacker.unwrap();
                let aa = attacker.get_attack(self) as u32;
                let ad = attacker.get_defense(self) as u32;
                let (mut da, mut dd) = (0u32, 0u32);
                let pa = self.card(attacker).current.controler;
                self.card_mut(attacker).set_status(STATUS_BATTLE_RESULT, false);
                self.card_mut(attacker).set_status(STATUS_BATTLE_DESTROYED, false);
                if let Some(t) = self.core.attack_target {
                    da = t.get_attack(self) as u32;
                    dd = t.get_defense(self) as u32;
                    self.card_mut(t).set_status(STATUS_BATTLE_RESULT, false);
                    self.card_mut(t).set_status(STATUS_BATTLE_DESTROYED, false);
                    let pd = self.card(t).current.controler;
                    if pa != pd {
                        self.card_mut(attacker).set_status(STATUS_OPPO_BATTLE, true);
                        self.card_mut(t).set_status(STATUS_OPPO_BATTLE, true);
                    }
                }
                let mut damchange = None;
                let mut reason_card = None;
                let mut bd = [false, false];
                self.calculate_battle_damage(Some(&mut damchange), Some(&mut reason_card), Some(&mut bd));
                if bd[0] {
                    let at = self.core.attack_target;
                    if let Some(ie) = attacker.is_affected_by_effect_t(self, EFFECT_INDESTRUCTABLE_BATTLE, at) {
                        let code = self.card(self.effect(ie).owner.unwrap()).data.code;
                        let m = self.new_message(MSG_HINT);
                        self.msg(m).u8(HINT_CARD).u8(0u8).u64(code);
                        bd[0] = false;
                    } else {
                        self.card_mut(attacker).set_status(STATUS_BATTLE_RESULT, true);
                    }
                }
                if bd[1] {
                    let t = self.core.attack_target.unwrap();
                    if let Some(ie) = t.is_affected_by_effect_t(self, EFFECT_INDESTRUCTABLE_BATTLE, Some(attacker)) {
                        let code = self.card(self.effect(ie).owner.unwrap()).data.code;
                        let m = self.new_message(MSG_HINT);
                        self.msg(m).u8(HINT_CARD).u8(0u8).u64(code);
                        bd[1] = false;
                    } else {
                        self.card_mut(t).set_status(STATUS_BATTLE_RESULT, true);
                    }
                }
                let aloc = attacker.get_info_location(self);
                let m = self.new_message(MSG_BATTLE);
                self.msg(m).loc(aloc).u32(aa).u32(ad).u8(bd[0]);
                if let Some(t) = self.core.attack_target {
                    let tloc = t.get_info_location(self);
                    self.msg(m).loc(tloc).u32(da).u32(dd).u8(bd[1]);
                } else {
                    self.msg(m).loc(NOLOC).u32(0u32).u32(0u32).u8(0u8);
                }
                arg.damage_change_effect = damchange;
                arg.reason_card = reason_card;
                if let Some(rc) = reason_card {
                    arg.reason_player = self.card(rc).current.controler;
                }
                if damchange.is_none() {
                    for p in [tp, 1 - tp] {
                        let dmg = self.core.battle_damage[p as usize];
                        if dmg != 0 {
                            let rc = reason_card.unwrap();
                            let rp = self.card(rc).current.controler;
                            self.raise_single_event(attacker, None, EVENT_PRE_BATTLE_DAMAGE, None, 0, rp, p, dmg as u32);
                            if let Some(t) = self.core.attack_target {
                                self.raise_single_event(t, None, EVENT_PRE_BATTLE_DAMAGE, None, 0, rp, p, dmg as u32);
                            }
                            self.raise_event_card(Some(rc), EVENT_PRE_BATTLE_DAMAGE, None, 0, rp, p, dmg as u32);
                        }
                    }
                }
                self.process_single_event();
                self.process_instant_event();
                self.core.damage_calculated = true;
                false
            }
            27 => {
                self.infos.phase = PHASE_DAMAGE as u16;
                self.core.hint_timing[tp as usize] = 0;
                self.core.chain_attack = false;
                let attacker = self.core.attacker.unwrap();
                let at = self.core.attack_target.map(|t| (t, self.card(t).fieldid_r));
                self.card_mut(attacker).battled_cards.addcard(at);
                if let Some(t) = self.core.attack_target {
                    let a = Some((attacker, self.card(attacker).fieldid_r));
                    self.card_mut(t).battled_cards.addcard(a);
                }
                let reason_card = arg.reason_card;
                let damchange = arg.damage_change_effect.take();
                let rp = arg.reason_player;
                for p in 0..2u8 {
                    let dmg = self.core.battle_damage[p as usize];
                    if dmg != 0 {
                        if damchange.is_none() {
                            self.damage(None, REASON_BATTLE, rp, reason_card, p, dmg as u32, false);
                        } else {
                            self.damage(damchange, REASON_EFFECT, rp, reason_card, p, dmg as u32, false);
                        }
                    }
                }
                self.reset_phase(PHASE_DAMAGE_CAL);
                self.adjust_all();
                false
            }
            28 => {
                let mut des = CardSet::new();
                let attacker = self.core.attacker.unwrap();
                let target = self.core.attack_target;
                for (c, other, pf) in [(Some(attacker), target, self.core.pre_field[0]), (target, Some(attacker), self.core.pre_field[1])] {
                    let Some(c) = c else { continue };
                    let cc = self.card(c);
                    if !(cc.is_status(STATUS_BATTLE_RESULT) && cc.current.location as u32 == LOCATION_MZONE && cc.fieldid_r == pf) {
                        continue;
                    }
                    des.insert(c);
                    let other = other.unwrap();
                    let octrl = self.card(other).current.controler;
                    {
                        let cm = self.card_mut(c);
                        cm.temp.reason = cm.current.reason;
                        cm.temp.reason_card = cm.current.reason_card;
                        cm.temp.reason_effect = cm.current.reason_effect;
                        cm.temp.reason_player = cm.current.reason_player;
                        cm.current.reason_effect = None;
                        cm.current.reason = REASON_BATTLE;
                        cm.current.reason_card = Some(other);
                        cm.current.reason_player = octrl;
                    }
                    let mut dest = LOCATION_GRAVE;
                    let mut seq = 0;
                    if let Some(pe) = other.is_affected_by_effect(self, EFFECT_BATTLE_DESTROY_REDIRECT) {
                        if self.card(c).data.type_ & TYPE_MONSTER != 0 {
                            dest = pe.get_value_card(self, Some(c), 0) as u32;
                            seq = dest >> 16;
                            dest &= 0xffff;
                        }
                    }
                    let owner = self.card(c).owner;
                    self.card_mut(c).sendto_param.set(owner, POS_FACEUP as u8, dest as u8, seq as u8);
                    self.card_mut(c).set_status(STATUS_DESTROY_CONFIRMED, true);
                }
                self.card_mut(attacker).set_status(STATUS_BATTLE_RESULT, false);
                if let Some(t) = target {
                    self.card_mut(t).set_status(STATUS_BATTLE_RESULT, false);
                }
                self.core.battle_destroy_rep.clear();
                self.core.desrep_chain.clear();
                if !des.is_empty() {
                    let ng = self.new_group(des);
                    self.emplace_process(units::Destroy { step: 10, reason_player: PLAYER_NONE as u8, reason: REASON_BATTLE, targets: Some(ng), reason_effect: None });
                    arg.cards_destroyed_by_battle = Some(ng);
                }
                false
            }
            29 => {
                if !self.core.battle_destroy_rep.is_empty() {
                    let rep = self.core.battle_destroy_rep.clone();
                    self.destroy(rep, None, REASON_EFFECT | REASON_REPLACE, PLAYER_NONE as u8, PLAYER_NONE as u8, 0, 0);
                }
                if !self.core.desrep_chain.is_empty() {
                    self.emplace_process(units::OperationReplace { step: 15, is_destroy: false, replace_effect: None, targets: None, target: None });
                }
                self.adjust_all();
                false
            }
            30 => {
                if let Some(des) = arg.cards_destroyed_by_battle {
                    let set = self.group(des).clone();
                    for c in public_order(self, &set) {
                        self.card_mut(c).set_status(STATUS_BATTLE_DESTROYED, true);
                        self.card_mut(c).set_status(STATUS_DESTROY_CONFIRMED, false);
                        c.filter_disable_related_cards(self);
                    }
                }
                self.core.selfdes_disabled = false;
                self.adjust_all();
                if self.is_flag(DUEL_6_STEP_BATLLE_STEP) {
                    if self.core.effect_damage_step == 0 {
                        self.hint_event_both(45);
                        self.core.hint_timing[tp as usize] = TIMING_DAMAGE_CAL;
                        let sn = self.is_flag(DUEL_SINGLE_CHAIN_IN_DAMAGE_SUBSTEP);
                        self.point_event_unit(sn);
                    } else {
                        self.break_effect(true);
                    }
                }
                false
            }
            31 | 32 => {
                if arg.step == 31 {
                    self.core.flip_delayed = false;
                    let mut fb = std::mem::take(&mut self.core.new_fchain_b);
                    fb.extend(std::mem::take(&mut self.core.new_fchain));
                    self.core.new_fchain = fb;
                    let mut ob = std::mem::take(&mut self.core.new_ochain_b);
                    ob.extend(std::mem::take(&mut self.core.new_ochain));
                    self.core.new_ochain = ob;
                    let attacker = self.core.attacker.unwrap();
                    self.raise_single_event(attacker, None, EVENT_BATTLED, None, 0, PLAYER_NONE as u8, 0, 0);
                    let mut battled = CardSet::from_card(attacker);
                    if let Some(t) = self.core.attack_target {
                        battled.insert(t);
                        self.raise_single_event(t, None, EVENT_BATTLED, None, 0, PLAYER_NONE as u8, 0, 1);
                    }
                    self.raise_event(battled, EVENT_BATTLED, None, 0, PLAYER_NONE as u8, 0, 0);
                    self.process_single_event();
                    self.process_instant_event();
                    if self.core.effect_damage_step != 0 {
                        if let Some(units::Unit::DamageStep(ds)) = self.core.reserved.as_mut() {
                            ds.cards_destroyed_by_battle = arg.cards_destroyed_by_battle;
                        }
                        return true;
                    }
                    arg.step = 32;
                }
                self.hint_event_both(43);
                self.core.hint_timing[0] |= TIMING_BATTLED;
                self.core.hint_timing[1] |= TIMING_BATTLED;
                let sn = self.is_flag(DUEL_SINGLE_CHAIN_IN_DAMAGE_SUBSTEP);
                self.point_event_unit(sn);
                false
            }
            33 => {
                if let Some(des) = arg.cards_destroyed_by_battle {
                    let set = self.group(des).clone();
                    for c in public_order(self, &set) {
                        let cc = self.card(c);
                        if cc.current.location as u32 != LOCATION_MZONE || (cc.fieldid_r != self.core.pre_field[0] && cc.fieldid_r != self.core.pre_field[1]) {
                            self.group_mut(des).remove(c);
                        }
                    }
                    self.emplace_process(units::Destroy { step: 3, reason_player: PLAYER_NONE as u8, reason: REASON_BATTLE, targets: Some(des), reason_effect: None });
                }
                self.adjust_all();
                false
            }
            34 => {
                arg.cards_destroyed_by_battle = None;
                self.core.damage_calculated = true;
                self.core.selfdes_disabled = false;
                self.core.flip_delayed = false;
                let mut fb = std::mem::take(&mut self.core.new_fchain_b);
                fb.extend(std::mem::take(&mut self.core.new_fchain));
                self.core.new_fchain = fb;
                let mut ob = std::mem::take(&mut self.core.new_ochain_b);
                ob.extend(std::mem::take(&mut self.core.new_ochain));
                self.core.new_ochain = ob;
                let mut ing = CardSet::new();
                let mut ed = CardSet::new();
                let attacker = self.core.attacker.unwrap();
                let target = self.core.attack_target;
                if self.card(attacker).is_status(STATUS_BATTLE_DESTROYED) && self.card(attacker).current.reason & REASON_BATTLE != 0 {
                    let t = target.unwrap();
                    let r = self.card(attacker).current.reason;
                    let tc = self.card(t).current.controler;
                    self.raise_single_event(t, None, EVENT_BATTLE_DESTROYING, None, r, tc, 0, 1);
                    self.raise_single_event(attacker, None, EVENT_BATTLE_DESTROYED, None, r, tc, 0, 0);
                    self.raise_single_event(attacker, None, EVENT_DESTROYED, None, r, tc, 0, 0);
                    ing.insert(t);
                    ed.insert(attacker);
                }
                if let Some(t) = target {
                    if self.card(t).is_status(STATUS_BATTLE_DESTROYED) && self.card(t).current.reason & REASON_BATTLE != 0 {
                        let r = self.card(t).current.reason;
                        let ac = self.card(attacker).current.controler;
                        self.raise_single_event(attacker, None, EVENT_BATTLE_DESTROYING, None, r, ac, 0, 0);
                        self.raise_single_event(t, None, EVENT_BATTLE_DESTROYED, None, r, ac, 0, 1);
                        self.raise_single_event(t, None, EVENT_DESTROYED, None, r, ac, 0, 1);
                        ing.insert(attacker);
                        ed.insert(t);
                    }
                }
                if !ing.is_empty() {
                    self.raise_event(ing, EVENT_BATTLE_DESTROYING, None, 0, 0, 0, 0);
                }
                if !ed.is_empty() {
                    self.raise_event(ed.clone(), EVENT_BATTLE_DESTROYED, None, 0, 0, 0, 0);
                    self.raise_event(ed, EVENT_DESTROYED, None, 0, 0, 0, 0);
                }
                self.raise_single_event(attacker, None, EVENT_DAMAGE_STEP_END, None, 0, 0, 0, 0);
                if let Some(t) = target {
                    self.raise_single_event(t, None, EVENT_DAMAGE_STEP_END, None, 0, 0, 0, 1);
                }
                self.raise_event_card(None, EVENT_DAMAGE_STEP_END, None, 0, 0, 0, 0);
                self.card_mut(attacker).set_status(STATUS_BATTLE_DESTROYED, false);
                if let Some(t) = target {
                    self.card_mut(t).set_status(STATUS_BATTLE_DESTROYED, false);
                }
                self.process_single_event();
                self.process_instant_event();
                self.hint_event_both(44);
                self.point_event_unit(true);
                arg.step = 38;
                false
            }
            38 => false,
            39 => {
                let attacker = self.core.attacker.unwrap();
                self.card_mut(attacker).set_status(STATUS_OPPO_BATTLE, false);
                if let Some(t) = self.core.attack_target {
                    self.card_mut(t).set_status(STATUS_OPPO_BATTLE, false);
                }
                if arg.forced_attack {
                    arg.forced_attack_done = true;
                    arg.step = 0;
                } else {
                    arg.step = units::RESTART;
                }
                self.infos.phase = PHASE_BATTLE_STEP as u16;
                self.new_message(MSG_DAMAGE_STEP_END);
                self.reset_phase(PHASE_DAMAGE);
                self.adjust_all();
                self.core.effect_damage_step != 0
            }
            40 => {
                self.core.chain_limit.clear();
                if !self.core.current_chain.is_empty() {
                    self.clear_chaining_status();
                    self.emplace_process(units::SolveChain { step: 0, skip_trigger: false, skip_freechain: false, skip_new: false, backed_up_operation: None });
                    if !arg.forced_attack {
                        arg.step = units::RESTART;
                    }
                    return false;
                }
                self.reset_phase(PHASE_BATTLE_STEP);
                self.adjust_all();
                false
            }
            41 => {
                // normal end of battle step
                arg.second_battle_phase_is_optional = true;
                let mut eset = Vec::new();
                self.filter_player_effect(tp, EFFECT_BP_TWICE, &mut eset, false);
                for &pe in &eset {
                    if !self.effect(pe).has_value() || pe.get_value(self, 0) != 1 {
                        arg.second_battle_phase_is_optional = false;
                        break;
                    }
                }
                arg.repeat_battle_phase = !eset.is_empty();
                self.infos.phase = PHASE_BATTLE as u16;
                self.emplace_process(units::PhaseEvent { step: 0, phase: PHASE_BATTLE as u16, is_opponent: false, priority_passed: false });
                self.adjust_all();
                false
            }
            42 => {
                self.core.attacker = None;
                self.core.attack_target = None;
                if arg.repeat_battle_phase && arg.second_battle_phase_is_optional {
                    self.emplace_process(units::SelectYesNo { step: 0, playerid: tp, description: 32 });
                    return false;
                }
                self.returns.set_i32(0, arg.phase_to_change_to as i32);
                self.returns.set_i32(1, arg.repeat_battle_phase as i32);
                true
            }
            43 => {
                let bp_twice = self.returns.at_i32(0);
                self.returns.set_i32(0, arg.phase_to_change_to as i32);
                self.returns.set_i32(1, bp_twice);
                true
            }
            _ => true,
        }
    }

    fn reset_attack_counts_on_field(&mut self) {
        for p in 0..2 {
            for c in self.player[p].list_mzone.clone().into_iter().flatten() {
                let cc = self.card_mut(c);
                cc.attack_announce_count = 0;
                cc.announce_count = 0;
                cc.attacked_count = 0;
                cc.announced_cards.clear();
                cc.attacked_cards.clear();
                cc.battled_cards.clear();
            }
        }
    }

    pub fn process_forced_battle(&mut self, arg: &mut units::ForcedBattle) -> bool {
        let tp = self.infos.turn_player;
        match arg.step {
            0 => {
                if self.is_player_affected_by_effect(tp, EFFECT_CANNOT_BP).is_some() {
                    return true;
                }
                self.core.battle_phase_count[tp as usize] = self.core.battle_phase_count[tp as usize].wrapping_add(1);
                if self.is_player_affected_by_effect(tp, EFFECT_SKIP_BP).is_some() || self.core.force_turn_end {
                    let m = self.new_message(MSG_NEW_PHASE);
                    self.msg(m).u16(PHASE_BATTLE_START);
                    self.reset_phase(PHASE_BATTLE_START);
                    self.reset_phase(PHASE_BATTLE_STEP);
                    self.reset_phase(PHASE_BATTLE);
                    self.adjust_all();
                    let ph = self.infos.phase;
                    let m = self.new_message(MSG_NEW_PHASE);
                    self.msg(m).u16(ph);
                    return true;
                }
                arg.backup_phase = self.infos.phase;
                let tmp_attacker = self.core.forced_attacker.unwrap();
                let tmp_attack_target = self.core.forced_attack_target;
                if !tmp_attacker.is_capable_attack_announce(self, tp) {
                    return true;
                }
                let mut cv = Vec::new();
                self.get_attack_target(tmp_attacker, &mut cv, false, true, None);
                if (cv.is_empty() && self.card(tmp_attacker).direct_attackable == 0) || tmp_attack_target.is_some_and(|t| !cv.contains(&t)) {
                    return true;
                }
                self.core.attacker = Some(tmp_attacker);
                self.core.attack_target = tmp_attack_target;
                self.reset_attack_counts_on_field();
                self.core.attack_cancelable = true;
                self.core.attack_cost_paid = 0;
                self.core.chain_attacker_id = 0;
                self.core.chain_attack_target = None;
                self.returns.set_i32(0, 1);
                let ph = self.infos.phase as u32;
                self.reset_phase(ph);
                self.reset_phase(PHASE_BATTLE_START);
                self.infos.phase = PHASE_BATTLE_STEP as u16;
                self.core.new_fchain.clear();
                self.core.new_ochain.clear();
                self.core.quick_f_chain.clear();
                self.core.delayed_quick_tmp.clear();
                let m = self.new_message(MSG_NEW_PHASE);
                self.msg(m).u16(PHASE_BATTLE_START);
                self.emplace_process(new_battle_command(1, None, true));
                false
            }
            1 => {
                let ph = self.infos.phase as u32;
                self.reset_phase(ph);
                self.infos.phase = arg.backup_phase;
                self.core.new_fchain.clear();
                self.core.new_ochain.clear();
                self.core.quick_f_chain.clear();
                self.core.delayed_quick_tmp.clear();
                self.reset_attack_counts_on_field();
                self.core.attacker = None;
                self.core.attack_target = None;
                let ph = self.infos.phase;
                let m = self.new_message(MSG_NEW_PHASE);
                self.msg(m).u16(ph);
                true
            }
            _ => true,
        }
    }

    pub fn process_damage_step(&mut self, arg: &mut units::DamageStep) -> bool {
        let tp = self.infos.turn_player;
        match arg.step {
            0 => {
                if self.core.effect_damage_step != 0 && !arg.new_attack {
                    return true;
                }
                self.core.effect_damage_step = 1;
                std::mem::swap(&mut self.core.attacker, &mut arg.attacker);
                std::mem::swap(&mut self.core.attack_target, &mut arg.attack_target);
                arg.backup_phase = self.infos.phase;
                let attacker = self.core.attacker.unwrap();
                if self.card(attacker).current.location as u32 != LOCATION_MZONE
                    || self.core.attack_target.is_some_and(|t| self.card(t).current.location as u32 != LOCATION_MZONE)
                {
                    arg.step = 2;
                    return false;
                }
                if arg.new_attack {
                    self.core.attack_state_count[tp as usize] = self.core.attack_state_count[tp as usize].wrapping_add(1);
                    self.core.battled_count[tp as usize] = self.core.battled_count[tp as usize].wrapping_add(1);
                    self.check_card_counter(attacker, ACTIVITY_ATTACK, tp as i32);
                }
                let at = self.core.attack_target.map(|t| (t, self.card(t).fieldid_r));
                self.card_mut(attacker).announced_cards.addcard(at);
                self.attack_all_target_check();
                let aloc = attacker.get_info_location(self);
                let m = self.new_message(MSG_ATTACK);
                self.msg(m).loc(aloc);
                match self.core.attack_target {
                    Some(t) => {
                        let tloc = t.get_info_location(self);
                        self.msg(m).loc(tloc);
                    }
                    None => {
                        self.msg(m).loc(NOLOC);
                    }
                }
                self.infos.phase = PHASE_DAMAGE as u16;
                self.new_message(MSG_DAMAGE_STEP_START);
                self.core.pre_field[0] = self.card(attacker).fieldid_r;
                let c = self.card_mut(attacker);
                c.attacked_count = c.attacked_count.wrapping_add(1);
                if let Some(t) = self.core.attack_target {
                    self.core.pre_field[1] = self.card(t).fieldid_r;
                    if self.card(t).is_position(POS_FACEDOWN) != 0 {
                        let pos = self.card(t).current.position;
                        self.change_position_one(t, None, PLAYER_NONE as u8, (pos >> 1) as u8, 0, true);
                        self.adjust_all();
                    }
                } else {
                    self.core.pre_field[1] = 0;
                }
                false
            }
            1 => {
                self.infos.phase = PHASE_DAMAGE_CAL as u16;
                self.emplace_process(new_battle_command(26, None, false));
                arg.step = 2;
                self.core.reserved = Some(units::Unit::DamageStep(arg.clone()));
                true
            }
            2 => {
                self.core.effect_damage_step = 2;
                let g = arg.cards_destroyed_by_battle;
                self.emplace_process(new_battle_command(32, g, false));
                false
            }
            3 => {
                std::mem::swap(&mut self.core.attacker, &mut arg.attacker);
                std::mem::swap(&mut self.core.attack_target, &mut arg.attack_target);
                if let Some(a) = self.core.attacker {
                    self.card_mut(a).set_status(STATUS_ATTACK_CANCELED, true);
                }
                if let Some(t) = self.core.attack_target {
                    self.card_mut(t).set_status(STATUS_ATTACK_CANCELED, true);
                }
                self.core.effect_damage_step = 0;
                self.infos.phase = arg.backup_phase;
                true
            }
            _ => true,
        }
    }

    fn apply_reflect_also(&mut self, both: bool, pa: u8, damp: u8, reflect: [Option<EffectId>; 2], also: [bool; 2]) {
        fn bdm(d: &mut Duel, i: u8) -> &mut i32 {
            &mut d.core.battle_damage[i as usize]
        }
        let pa_u = pa as usize;
        let hp = |d: &Duel, e: Option<EffectId>| e.map(|e| e.get_handler_player(d));
        if both {
            // turn player's effect applies first
            if reflect[pa_u].is_some() && hp(self, reflect[pa_u]) == Some(pa) {
                let v = *bdm(self, pa);
                *bdm(self, 1 - pa) += v;
                *bdm(self, pa) = 0;
            } else if reflect[1 - pa_u].is_some() && hp(self, reflect[1 - pa_u]) == Some(pa) {
                let v = *bdm(self, 1 - pa);
                *bdm(self, pa) += v;
                *bdm(self, 1 - pa) = 0;
            } else if reflect[pa_u].is_some() && hp(self, reflect[pa_u]) == Some(1 - pa) {
                let v = *bdm(self, pa);
                *bdm(self, 1 - pa) += v;
                *bdm(self, pa) = 0;
            } else if reflect[1 - pa_u].is_some() && hp(self, reflect[1 - pa_u]) == Some(1 - pa) {
                let v = *bdm(self, 1 - pa);
                *bdm(self, pa) += v;
                *bdm(self, 1 - pa) = 0;
            }
        } else {
            let d_u = damp as usize;
            if reflect[d_u].is_some() {
                if !also[1 - d_u] {
                    let v = *bdm(self, damp);
                    *bdm(self, 1 - damp) += v;
                    *bdm(self, damp) = 0;
                } else {
                    let v = *bdm(self, damp);
                    *bdm(self, 1 - damp) += v;
                    let w = *bdm(self, 1 - damp);
                    *bdm(self, damp) = w;
                }
            } else if also[d_u] {
                if reflect[1 - d_u].is_none() {
                    let v = *bdm(self, damp);
                    *bdm(self, 1 - damp) += v;
                } else {
                    let v = *bdm(self, damp);
                    *bdm(self, 1 - damp) += v;
                    let w = *bdm(self, 1 - damp);
                    *bdm(self, damp) += w;
                    *bdm(self, 1 - damp) = 0;
                }
            }
        }
    }

    fn apply_change_battle_damage(&mut self, eset: &[EffectId], card_param: Option<CardId>) {
        for p in 0..2u8 {
            let mut double_dam = false;
            let mut half_dam = false;
            let mut dam_value: i32 = -1;
            for &pe in eset {
                let mut val: i64 = -1;
                if !self.effect(pe).is_flag(EFFECT_FLAG_PLAYER_TARGET) {
                    self.add_param(Value::Int(p as i64));
                    self.add_param(Value::card(card_param));
                    val = pe.get_value(self, 2);
                } else if pe.is_target_player(self, p) {
                    self.add_param(Value::card(card_param));
                    val = pe.get_value(self, 1);
                }
                if val == DOUBLE_DAMAGE as i64 {
                    double_dam = true;
                } else if val == HALF_DAMAGE as i64 {
                    half_dam = true;
                } else if val > 0 {
                    dam_value = val as i32;
                } else if val == 0 {
                    dam_value = 0;
                    break;
                }
            }
            if double_dam && half_dam {
                double_dam = false;
                half_dam = false;
            }
            let bd = &mut self.core.battle_damage[p as usize];
            if double_dam {
                *bd = bd.wrapping_mul(2);
            }
            if half_dam {
                *bd /= 2;
            }
            if dam_value >= 0 && *bd > 0 {
                *bd = dam_value;
            }
        }
    }

    pub fn calculate_battle_damage(&mut self, pdamchange: Option<&mut Option<EffectId>>, preason_card: Option<&mut Option<CardId>>, battle_destroyed: Option<&mut [bool; 2]>) {
        let attacker = self.core.attacker.unwrap();
        let aa = attacker.get_attack(self) as u32;
        let ad = attacker.get_defense(self) as u32;
        let mut a = aa;
        let pa = self.card(attacker).current.controler;
        let mut damp: u8 = 0;
        let mut damchange = None;
        let mut reason_card: Option<CardId> = None;
        let mut bd = [false, false];
        let mut pierce = false;
        self.core.battle_damage = [0, 0];
        if self.card(attacker).is_position(POS_FACEUP_DEFENSE) != 0 {
            if let Some(defattack) = attacker.is_affected_by_effect(self, EFFECT_DEFENSE_ATTACK) {
                if defattack.get_value_card(self, Some(attacker), 0) != 0 {
                    a = ad;
                }
            }
        }
        if let Some(battstat) = attacker.is_affected_by_effect(self, EFFECT_CHANGE_BATTLE_STAT) {
            a = battstat.get_value_card(self, Some(attacker), 0) as u32;
        }
        if let Some(target) = self.core.attack_target {
            let da = target.get_attack(self) as u32;
            let dd = target.get_defense(self) as u32;
            let pd = self.card(target).current.controler;
            let d = if let Some(battstat) = target.is_affected_by_effect(self, EFFECT_CHANGE_BATTLE_STAT) {
                battstat.get_value_card(self, Some(target), 0) as u32
            } else if self.card(target).is_position(POS_ATTACK) != 0 {
                da
            } else {
                dd
            };
            if self.card(target).is_position(POS_ATTACK) != 0 {
                if a > d {
                    damp = pd;
                    self.core.battle_damage[damp as usize] = a.wrapping_sub(d) as i32;
                    reason_card = Some(attacker);
                    bd[1] = true;
                } else if a < d {
                    damp = pa;
                    self.core.battle_damage[damp as usize] = d.wrapping_sub(a) as i32;
                    reason_card = Some(target);
                    bd[0] = true;
                } else if a != 0 || self.is_flag(DUEL_0_ATK_DESTROYED) {
                    bd = [true, true];
                }
            } else if a > d {
                let mut eset = Vec::new();
                attacker.filter_effect(self, EFFECT_PIERCE, &mut eset, true);
                if !eset.is_empty() {
                    pierce = true;
                    let mut dp = [false, false];
                    for &pe in &eset {
                        dp[1 - pe.get_handler_player(self) as usize] = true;
                    }
                    if dp[0] {
                        self.core.battle_damage[0] = a.wrapping_sub(d) as i32;
                    }
                    if dp[1] {
                        self.core.battle_damage[1] = a.wrapping_sub(d) as i32;
                    }
                    let mut double_damage = false;
                    for &pe in &eset {
                        if pe.get_value(self, 0) == DOUBLE_DAMAGE as i64 {
                            double_damage = true;
                        }
                    }
                    if double_damage {
                        if dp[0] {
                            self.core.battle_damage[0] = self.core.battle_damage[0].wrapping_mul(2);
                        }
                        if dp[1] {
                            self.core.battle_damage[1] = self.core.battle_damage[1].wrapping_mul(2);
                        }
                    }
                    let mut both = dp[0] && dp[1];
                    if !both {
                        damp = if dp[0] { 0 } else { 1 };
                        if attacker.is_affected_by_effect(self, EFFECT_BOTH_BATTLE_DAMAGE).is_some() || target.is_affected_by_effect(self, EFFECT_BOTH_BATTLE_DAMAGE).is_some() {
                            self.core.battle_damage[1 - damp as usize] = self.core.battle_damage[damp as usize];
                            both = true;
                        }
                    }
                    let mut reflect = [None, None];
                    reflect[pd as usize] = target.is_affected_by_effect_t(self, EFFECT_REFLECT_BATTLE_DAMAGE, Some(attacker));
                    if reflect[pd as usize].is_none() {
                        reflect[pd as usize] = self.is_player_affected_by_effect(pd, EFFECT_REFLECT_BATTLE_DAMAGE);
                    }
                    reflect[1 - pd as usize] = attacker.is_affected_by_effect_t(self, EFFECT_REFLECT_BATTLE_DAMAGE, Some(target));
                    if reflect[1 - pd as usize].is_none() {
                        reflect[1 - pd as usize] = self.is_player_affected_by_effect(1 - pd, EFFECT_REFLECT_BATTLE_DAMAGE);
                    }
                    let mut also = [false, false];
                    if !both && (target.is_affected_by_effect(self, EFFECT_ALSO_BATTLE_DAMAGE).is_some() || self.is_player_affected_by_effect(pd, EFFECT_ALSO_BATTLE_DAMAGE).is_some()) {
                        also[pd as usize] = true;
                    }
                    if !both
                        && (attacker.is_affected_by_effect(self, EFFECT_ALSO_BATTLE_DAMAGE).is_some() || self.is_player_affected_by_effect(1 - pd, EFFECT_ALSO_BATTLE_DAMAGE).is_some())
                    {
                        also[1 - pd as usize] = true;
                    }
                    self.apply_reflect_also(both, pa, damp, reflect, also);
                    let mut eset = Vec::new();
                    attacker.filter_effect(self, EFFECT_CHANGE_BATTLE_DAMAGE, &mut eset, false);
                    target.filter_effect(self, EFFECT_CHANGE_BATTLE_DAMAGE, &mut eset, false);
                    self.filter_player_effect(pa, EFFECT_CHANGE_BATTLE_DAMAGE, &mut eset, false);
                    self.filter_player_effect(1 - pa, EFFECT_CHANGE_BATTLE_DAMAGE, &mut eset, false);
                    crate::effect::sort_effects_by_id(self, &mut eset);
                    self.apply_change_battle_damage(&eset, Some(attacker));
                    if attacker.is_affected_by_effect(self, EFFECT_NO_BATTLE_DAMAGE).is_some()
                        || target.is_affected_by_effect_t(self, EFFECT_AVOID_BATTLE_DAMAGE, Some(attacker)).is_some()
                        || self.is_player_affected_by_effect(pd, EFFECT_AVOID_BATTLE_DAMAGE).is_some()
                    {
                        self.core.battle_damage[pd as usize] = 0;
                    }
                    if target.is_affected_by_effect(self, EFFECT_NO_BATTLE_DAMAGE).is_some()
                        || attacker.is_affected_by_effect_t(self, EFFECT_AVOID_BATTLE_DAMAGE, Some(target)).is_some()
                        || self.is_player_affected_by_effect(1 - pd, EFFECT_AVOID_BATTLE_DAMAGE).is_some()
                    {
                        self.core.battle_damage[1 - pd as usize] = 0;
                    }
                    reason_card = Some(attacker);
                }
                bd[1] = true;
            } else if a < d {
                damp = pa;
                self.core.battle_damage[damp as usize] = d.wrapping_sub(a) as i32;
                reason_card = Some(target);
            }
        } else if a != 0 {
            damp = 1 - pa;
            self.core.battle_damage[damp as usize] = a as i32;
            reason_card = Some(attacker);
        }
        if let Some(rc) = reason_card {
            if !pierce {
                damchange = rc.is_affected_by_effect(self, EFFECT_BATTLE_DAMAGE_TO_EFFECT);
                if damchange.is_none() {
                    let dam_card = if rc == attacker { self.core.attack_target } else { Some(attacker) };
                    let mut both = false;
                    if rc.is_affected_by_effect(self, EFFECT_BOTH_BATTLE_DAMAGE).is_some()
                        || dam_card.is_some_and(|dc| dc.is_affected_by_effect(self, EFFECT_BOTH_BATTLE_DAMAGE).is_some())
                    {
                        self.core.battle_damage[1 - damp as usize] = self.core.battle_damage[damp as usize];
                        both = true;
                    }
                    let mut reflect = [None, None];
                    if let Some(dc) = dam_card {
                        reflect[damp as usize] = dc.is_affected_by_effect_t(self, EFFECT_REFLECT_BATTLE_DAMAGE, Some(rc));
                    }
                    if reflect[damp as usize].is_none() {
                        reflect[damp as usize] = self.is_player_affected_by_effect(damp, EFFECT_REFLECT_BATTLE_DAMAGE);
                    }
                    reflect[1 - damp as usize] = rc.is_affected_by_effect_t(self, EFFECT_REFLECT_BATTLE_DAMAGE, dam_card);
                    if reflect[1 - damp as usize].is_none() {
                        reflect[1 - damp as usize] = self.is_player_affected_by_effect(1 - damp, EFFECT_REFLECT_BATTLE_DAMAGE);
                    }
                    let mut also = [false, false];
                    if !both
                        && (dam_card.is_some_and(|dc| dc.is_affected_by_effect(self, EFFECT_ALSO_BATTLE_DAMAGE).is_some())
                            || self.is_player_affected_by_effect(damp, EFFECT_ALSO_BATTLE_DAMAGE).is_some())
                    {
                        also[damp as usize] = true;
                    }
                    if !both
                        && (rc.is_affected_by_effect(self, EFFECT_ALSO_BATTLE_DAMAGE).is_some() || self.is_player_affected_by_effect(1 - damp, EFFECT_ALSO_BATTLE_DAMAGE).is_some())
                    {
                        also[1 - damp as usize] = true;
                    }
                    self.apply_reflect_also(both, pa, damp, reflect, also);
                    let mut eset = Vec::new();
                    rc.filter_effect(self, EFFECT_CHANGE_BATTLE_DAMAGE, &mut eset, false);
                    if let Some(dc) = dam_card {
                        dc.filter_effect(self, EFFECT_CHANGE_BATTLE_DAMAGE, &mut eset, false);
                    }
                    self.filter_player_effect(damp, EFFECT_CHANGE_BATTLE_DAMAGE, &mut eset, false);
                    self.filter_player_effect(1 - damp, EFFECT_CHANGE_BATTLE_DAMAGE, &mut eset, false);
                    crate::effect::sort_effects_by_id(self, &mut eset);
                    self.apply_change_battle_damage(&eset, Some(rc));
                    if rc.is_affected_by_effect(self, EFFECT_NO_BATTLE_DAMAGE).is_some()
                        || dam_card.is_some_and(|dc| dc.is_affected_by_effect_t(self, EFFECT_AVOID_BATTLE_DAMAGE, Some(rc)).is_some())
                        || self.is_player_affected_by_effect(damp, EFFECT_AVOID_BATTLE_DAMAGE).is_some()
                    {
                        self.core.battle_damage[damp as usize] = 0;
                    }
                    if dam_card.is_some_and(|dc| dc.is_affected_by_effect(self, EFFECT_NO_BATTLE_DAMAGE).is_some())
                        || rc.is_affected_by_effect_t(self, EFFECT_AVOID_BATTLE_DAMAGE, dam_card).is_some()
                        || self.is_player_affected_by_effect(1 - damp, EFFECT_AVOID_BATTLE_DAMAGE).is_some()
                    {
                        self.core.battle_damage[1 - damp as usize] = 0;
                    }
                }
            }
        }
        if self.core.battle_damage[damp as usize] == 0 && self.core.battle_damage[1 - damp as usize] == 0 {
            reason_card = None;
        }
        if let Some(p) = pdamchange {
            *p = damchange;
        }
        if let Some(p) = preason_card {
            *p = reason_card;
        }
        if let Some(p) = battle_destroyed {
            *p = bd;
        }
    }
}

/// `Processors::BattleCommand(step, cards_destroyed_by_battle, forced_attack)`
pub fn new_battle_command(step: u16, cards_destroyed_by_battle: Option<GroupRef>, forced_attack: bool) -> units::BattleCommand {
    units::BattleCommand {
        step,
        phase_to_change_to: 0,
        forced_attack,
        forced_attack_done: false,
        is_replaying_attack: false,
        attack_announce_failed: false,
        repeat_battle_phase: false,
        second_battle_phase_is_optional: false,
        previous_point_event_had_any_trigger_to_resolve: false,
        reason_player: PLAYER_NONE as u8,
        damage_change_effect: None,
        cards_destroyed_by_battle,
        reason_card: None,
        must_attack_map: Vec::new(),
    }
}
