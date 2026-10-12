//! The processor (`processor.cpp` part 4: the turn).

use crate::constants::*;
use crate::duel::Duel;
use crate::units;

impl Duel {
    fn clear_new_chains(&mut self) {
        self.core.new_fchain.clear();
        self.core.new_ochain.clear();
        self.core.quick_f_chain.clear();
        self.core.delayed_quick_tmp.clear();
    }
    fn new_phase_message(&mut self) {
        let ph = self.infos.phase;
        let m = self.new_message(MSG_NEW_PHASE);
        self.msg(m).u16(ph);
    }
    fn phase_event(&mut self, phase: u32) {
        self.emplace_process(units::PhaseEvent { step: 0, phase: phase as u16, is_opponent: false, priority_passed: false });
    }
    fn point_event(&mut self, skip_trigger: bool, skip_freechain: bool, skip_new: bool) {
        self.emplace_process(units::PointEvent { step: 0, skip_trigger, skip_freechain, skip_new });
    }
    fn idle_command(&mut self) {
        self.emplace_process(units::IdleCommand { step: 0, phase_to_change_to: 0, card_to_reposition: None });
    }

    pub fn process_turn(&mut self, arg: &mut units::Turn) -> bool {
        let turn_player = arg.turn_player;
        match arg.step {
            0 => {
                // Pre Draw
                for e in std::mem::take(&mut self.core.reseted_effects) {
                    self.delete_effect(e);
                }
                self.core.effect_count_code.clear();
                for p in 0..2 {
                    for c in self.player[p].list_mzone.clone().into_iter().flatten() {
                        let cc = self.card_mut(c);
                        cc.set_status(STATUS_SUMMON_TURN, false);
                        cc.set_status(STATUS_FLIP_SUMMON_TURN, false);
                        cc.set_status(STATUS_SPSUMMON_TURN, false);
                        cc.set_status(STATUS_SET_TURN, false);
                        cc.set_status(STATUS_FORM_CHANGED, false);
                        cc.indestructable_effects.clear();
                        cc.attack_announce_count = 0;
                        cc.announce_count = 0;
                        cc.attacked_count = 0;
                        cc.announced_cards.clear();
                        cc.attacked_cards.clear();
                        cc.battled_cards.clear();
                        cc.attack_all_target = 1;
                    }
                    for c in self.player[p].list_szone.clone().into_iter().flatten() {
                        let cc = self.card_mut(c);
                        cc.set_status(STATUS_SET_TURN, false);
                        cc.indestructable_effects.clear();
                    }
                    let core = &mut self.core;
                    core.summon_state_count[p] = 0;
                    core.normalsummon_state_count[p] = 0;
                    core.flipsummon_state_count[p] = 0;
                    core.spsummon_state_count[p] = 0;
                    core.spsummon_state_count_rst[p] = 0;
                    core.attack_state_count[p] = 0;
                    core.battle_phase_count[p] = 0;
                    core.battled_count[p] = 0;
                    core.summon_count[p] = 0;
                    core.extra_summon[p] = 0;
                    core.spsummon_once_map[p].clear();
                    core.spsummon_once_map_rst[p].clear();
                }
                self.emplace_process(units::RefreshRelay { step: 0 });
                false
            }
            1 => {
                self.core.force_turn_end = false;
                self.core.spsummon_rst = false;
                for e in self.field_effects.rechargeable.clone() {
                    if !self.effect(e).is_flag(EFFECT_FLAG_NO_TURN_RESET) {
                        e.recharge(self);
                    }
                }
                for counter in [
                    &mut self.core.summon_counter,
                    &mut self.core.normalsummon_counter,
                    &mut self.core.spsummon_counter,
                    &mut self.core.flipsummon_counter,
                    &mut self.core.attack_counter,
                    &mut self.core.chain_counter,
                ] {
                    for x in counter.iter_mut() {
                        x.1.player_amount = [0, 0];
                    }
                }
                for e in self.field_effects.spsummon_count_eff.clone() {
                    let c = e.get_handler(self);
                    if !self.effect(e).is_flag(EFFECT_FLAG_NO_TURN_RESET) {
                        let cc = self.card_mut(c);
                        cc.spsummon_counter = [0, 0];
                        cc.spsummon_counter_rst = [0, 0];
                    }
                }
                self.infos.turn_id = self.infos.turn_id.wrapping_add(1);
                self.infos.turn_id_by_player[turn_player as usize] = self.infos.turn_id_by_player[turn_player as usize].wrapping_add(1);
                self.infos.turn_player = turn_player;
                let m = self.new_message(MSG_NEW_TURN);
                self.msg(m).u8(turn_player);
                if !self.is_flag(DUEL_RELAY) && self.infos.turn_id != 1 {
                    self.tag_swap(turn_player);
                }
                if self.is_player_affected_by_effect(turn_player, EFFECT_SKIP_TURN).is_some() {
                    arg.step = 18;
                    self.reset_phase(PHASE_DRAW);
                    self.reset_phase(PHASE_STANDBY);
                    self.reset_phase(PHASE_END);
                    self.adjust_all();
                    return false;
                }
                self.infos.phase = PHASE_DRAW as u16;
                self.core.phase_action = false;
                self.core.hand_adjusted = false;
                self.raise_event_card(None, EVENT_PHASE_START + PHASE_DRAW, None, 0, 0, turn_player, 0);
                self.process_instant_event();
                self.adjust_all();
                false
            }
            2 => {
                self.clear_new_chains();
                if self.is_player_affected_by_effect(turn_player, EFFECT_SKIP_DP).is_some() || self.core.force_turn_end {
                    arg.step = 3;
                    self.reset_phase(PHASE_DRAW);
                    self.adjust_all();
                    return false;
                }
                self.new_phase_message();
                self.raise_event_card(None, EVENT_PREDRAW, None, 0, 0, turn_player, 0);
                self.process_instant_event();
                let m = self.new_message(MSG_HINT);
                self.msg(m).u8(HINT_EVENT).u8(turn_player).u64(27u64);
                if !self.core.new_fchain.is_empty() || !self.core.new_ochain.is_empty() {
                    self.point_event(false, true, false);
                }
                false
            }
            3 => {
                // Draw, new ruling
                if self.is_flag(DUEL_1ST_TURN_DRAW) || self.infos.turn_id > 1 {
                    let mut count = self.get_draw_count(turn_player);
                    if count > 0 && self.is_flag(DUEL_DRAW_UNTIL_5) {
                        count = count.max(5 - self.player[turn_player as usize].list_hand.len() as i32);
                    }
                    if count > 0 {
                        self.draw(None, REASON_RULE, turn_player, turn_player, count as u16);
                        self.point_event(false, false, false);
                    }
                }
                self.phase_event(PHASE_DRAW);
                false
            }
            4 => false,
            5 => {
                // Standby Phase
                self.infos.phase = PHASE_STANDBY as u16;
                self.core.phase_action = false;
                self.clear_new_chains();
                if self.is_flag(DUEL_NO_STANDBY_PHASE) || self.is_player_affected_by_effect(turn_player, EFFECT_SKIP_SP).is_some() || self.core.force_turn_end {
                    arg.step = 6;
                    self.reset_phase(PHASE_STANDBY);
                    self.adjust_all();
                    return false;
                }
                self.new_phase_message();
                self.raise_event_card(None, EVENT_PHASE_START + PHASE_STANDBY, None, 0, 0, turn_player, 0);
                self.process_instant_event();
                false
            }
            6 => {
                // EVENT_PHASE_START + PHASE_STANDBY is a special case (c89642993)
                if !self.core.new_fchain.is_empty()
                    || !self.core.new_ochain.is_empty()
                    || self.core.instant_event.back().map(|e| e.event_code) != Some(EVENT_PHASE_START + PHASE_STANDBY)
                {
                    self.point_event(false, false, false);
                }
                self.phase_event(PHASE_STANDBY);
                false
            }
            7 => {
                // Main1
                self.infos.phase = PHASE_MAIN1 as u16;
                self.core.phase_action = false;
                self.raise_event_card(None, EVENT_PHASE_START + PHASE_MAIN1, None, 0, 0, turn_player, 0);
                self.process_instant_event();
                self.adjust_all();
                false
            }
            8 => false,
            9 => {
                self.clear_new_chains();
                self.new_phase_message();
                self.idle_command();
                false
            }
            10 => {
                if self.returns.at_i32(0) == 7 {
                    arg.step = 15;
                    return false;
                }
                self.infos.phase = PHASE_BATTLE_START as u16;
                self.clear_new_chains();
                self.core.phase_action = false;
                self.core.battle_phase_count[turn_player as usize] = self.core.battle_phase_count[turn_player as usize].wrapping_add(1);
                self.new_phase_message();
                if self.is_player_affected_by_effect(turn_player, EFFECT_SKIP_BP).is_some() || self.core.force_turn_end {
                    arg.step = 15;
                    self.reset_phase(PHASE_BATTLE_START);
                    self.reset_phase(PHASE_BATTLE_STEP);
                    self.reset_phase(PHASE_BATTLE);
                    self.adjust_all();
                    return false;
                }
                self.raise_event_card(None, EVENT_PHASE_START + PHASE_BATTLE_START, None, 0, 0, turn_player, 0);
                self.process_instant_event();
                self.adjust_all();
                false
            }
            11 => {
                if !self.core.new_fchain.is_empty() || !self.core.new_ochain.is_empty() {
                    self.point_event(false, false, false);
                }
                self.phase_event(PHASE_BATTLE_START);
                false
            }
            12 => {
                self.infos.phase = PHASE_BATTLE_STEP as u16;
                self.clear_new_chains();
                self.core.phase_action = false;
                self.core.chain_attack = false;
                self.emplace_process(crate::processor3::new_battle_command(0, None, false));
                false
            }
            13 => {
                if !arg.has_performed_second_battle_phase && self.returns.at_i32(1) != 0 {
                    arg.has_performed_second_battle_phase = true;
                    arg.step = 9;
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
                    return false;
                }
                arg.has_performed_second_battle_phase = false;
                if self.is_flag(DUEL_NO_MAIN_PHASE_2) {
                    arg.step = 15;
                    self.adjust_all();
                    return false;
                }
                self.core.skip_m2 = false;
                if self.returns.at_i32(0) == 3 || self.core.force_turn_end {
                    self.core.skip_m2 = true;
                }
                // Main2
                self.infos.phase = PHASE_MAIN2 as u16;
                self.core.phase_action = false;
                self.raise_event_card(None, EVENT_PHASE_START + PHASE_MAIN2, None, 0, 0, turn_player, 0);
                self.process_instant_event();
                self.adjust_all();
                false
            }
            14 => {
                if !self.core.new_fchain.is_empty() || !self.core.new_ochain.is_empty() {
                    self.point_event(false, false, false);
                }
                false
            }
            15 => {
                self.clear_new_chains();
                self.new_phase_message();
                self.infos.can_shuffle = true;
                self.idle_command();
                false
            }
            16 => {
                // End Phase
                self.infos.phase = PHASE_END as u16;
                self.core.phase_action = false;
                if self.is_player_affected_by_effect(turn_player, EFFECT_SKIP_EP).is_some() {
                    arg.step = 18;
                    self.reset_phase(PHASE_END);
                    self.adjust_all();
                    return false;
                }
                self.new_phase_message();
                self.raise_event_card(None, EVENT_PHASE_START + PHASE_END, None, 0, 0, turn_player, 0);
                self.process_instant_event();
                self.adjust_all();
                false
            }
            17 => {
                if !self.core.new_fchain.is_empty() || !self.core.new_ochain.is_empty() {
                    self.point_event(false, false, false);
                }
                false
            }
            18 => {
                self.clear_new_chains();
                self.phase_event(PHASE_END);
                false
            }
            19 => {
                self.raise_event_card(None, EVENT_TURN_END, None, 0, 0, turn_player, 0);
                self.process_instant_event();
                self.adjust_all();
                false
            }
            20 => {
                self.clear_new_chains();
                arg.step = units::RESTART;
                arg.turn_player = 1 - arg.turn_player;
                false
            }
            _ => true,
        }
    }
}
