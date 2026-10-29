//! The processor (`processor.cpp` part 2: the idle command of the main phases).

use crate::constants::*;
use crate::duel::{Chain, Duel};
use crate::types::*;
use crate::units;

impl Duel {
    pub fn process_idle_command(&mut self, arg: &mut units::IdleCommand) -> bool {
        let tp = self.infos.turn_player;
        match arg.step {
            0 => {
                let mut must_attack = false;
                self.core.select_chains.clear();
                self.nil_event.event_code = EVENT_FREE_CHAIN;
                if self.core.set_forced_attack {
                    self.core.set_forced_attack = false;
                    arg.step = units::RESTART;
                    self.emplace_process(units::ForcedBattle { step: 0, backup_phase: 0, backup_attacker: None, backup_attack_target: None });
                    return false;
                }
                self.core.to_bp = true;
                self.core.to_ep = true;
                if (!self.is_flag(DUEL_ATTACK_FIRST_TURN) && self.infos.turn_id == 1 && self.is_player_affected_by_effect(tp, EFFECT_BP_FIRST_TURN).is_none())
                    || self.infos.phase as u32 == PHASE_MAIN2
                    || self.is_player_affected_by_effect(tp, EFFECT_CANNOT_BP).is_some()
                    || self.core.force_turn_end
                {
                    self.core.to_bp = false;
                }
                if self.infos.phase as u32 == PHASE_MAIN1 {
                    for c in self.player[tp as usize].list_mzone.clone().into_iter().flatten() {
                        if c.is_capable_attack(self) && c.is_affected_by_effect(self, EFFECT_MUST_ATTACK).is_some() {
                            must_attack = true;
                            break;
                        }
                    }
                    if self.core.to_bp && (must_attack || self.is_player_affected_by_effect(tp, EFFECT_CANNOT_EP).is_some()) {
                        self.core.to_ep = false;
                    }
                }
                let ph = self.infos.phase as u32;
                if (ph == PHASE_MAIN1 && self.is_player_affected_by_effect(tp, EFFECT_SKIP_M1).is_some())
                    || (ph == PHASE_MAIN2 && self.is_player_affected_by_effect(tp, EFFECT_SKIP_M2).is_some())
                    || self.core.force_turn_end
                {
                    if self.core.to_bp && self.core.to_ep {
                        self.core.select_options = vec![80, 81];
                        self.emplace_process(units::SelectOption { step: 0, playerid: tp });
                        arg.step = 11;
                    } else if self.core.to_bp {
                        arg.phase_to_change_to = 6;
                        arg.step = 10;
                        self.reset_phase(ph);
                        self.adjust_all();
                    } else {
                        arg.phase_to_change_to = 7;
                        arg.step = 10;
                        self.reset_phase(ph);
                        self.adjust_all();
                    }
                    return false;
                }
                if ph == PHASE_MAIN2 && self.core.skip_m2 {
                    self.core.skip_m2 = false;
                    self.returns.set_i32(0, 7);
                    return false;
                }
                let ne = self.nil_event.clone();
                for pe in self.field_effects.activate_effect.range(EVENT_FREE_CHAIN) {
                    pe.set_activate_location(self);
                    if pe.is_activateable(self, tp, &ne, false, false, false, false, false) {
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
                for pe in self.field_effects.ignition_effect.all() {
                    pe.set_activate_location(self);
                    if pe.is_activateable(self, tp, &ne, false, false, false, false, false) {
                        self.core.select_chains.push_back(Chain { triggering_effect: Some(pe), ..Default::default() });
                    }
                }
                self.core.summonable_cards.clear();
                for c in self.player[tp as usize].list_hand.clone() {
                    if c.is_can_be_summoned(self, tp, false, None, 0, 0x1f) {
                        self.core.summonable_cards.push(c);
                    }
                }
                for c in self.player[tp as usize].list_mzone.clone().into_iter().flatten() {
                    if c.is_can_be_summoned(self, tp, false, None, 0, 0x1f) {
                        self.core.summonable_cards.push(c);
                    }
                }
                self.core.spsummonable_cards.clear();
                let mut eset = Vec::new();
                self.filter_field_effect(EFFECT_SPSUMMON_PROC, &mut eset, true);
                for peff in eset {
                    let c = peff.get_handler(self);
                    let ctrl = self.card(c).current.controler;
                    if !peff.check_count_limit(self, ctrl) {
                        continue;
                    }
                    if ctrl == tp && c.is_special_summonable(self, tp, 0) != 0 {
                        self.core.spsummonable_cards.push(c);
                    }
                }
                let mut eset = Vec::new();
                self.filter_field_effect(EFFECT_SPSUMMON_PROC_G, &mut eset, true);
                for peff in eset {
                    let c = peff.get_handler(self);
                    if !peff.check_count_limit(self, tp) {
                        continue;
                    }
                    if self.card(c).current.controler != tp && !self.effect(peff).is_flag(EFFECT_FLAG_BOTH_SIDE) {
                        continue;
                    }
                    let oreason = self.core.reason_effect;
                    let op = self.core.reason_player;
                    self.core.reason_effect = Some(peff);
                    self.core.reason_player = self.card(c).current.controler;
                    let cond = self.effect(peff).condition().clone();
                    self.add_param(Value::Effect(peff));
                    self.add_param(Value::Card(c));
                    if self.check_condition(cond.as_ref(), 2) {
                        self.core.spsummonable_cards.push(c);
                    }
                    self.core.reason_effect = oreason;
                    self.core.reason_player = op;
                }
                self.core.repositionable_cards.clear();
                for c in self.player[tp as usize].list_mzone.clone().into_iter().flatten() {
                    if (self.card(c).is_position(POS_FACEUP | POS_FACEDOWN_ATTACK) != 0 && c.is_capable_change_position(self, tp))
                        || (self.card(c).is_position(POS_FACEDOWN) != 0 && c.is_can_be_flip_summoned(self, tp))
                    {
                        self.core.repositionable_cards.push(c);
                    }
                }
                self.core.msetable_cards.clear();
                self.core.ssetable_cards.clear();
                for c in self.player[tp as usize].list_hand.clone() {
                    if c.is_setable_mzone(self, tp, false, None, 0, 0x1f) {
                        self.core.msetable_cards.push(c);
                    }
                    if c.is_setable_szone(self, tp, false) {
                        self.core.ssetable_cards.push(c);
                    }
                }
                self.emplace_process(units::SelectIdleCmd { step: 0, playerid: tp });
                false
            }
            1 => {
                let r = self.returns.at_i32(0);
                let ctype = (r & 0xffff) as u32;
                let sel = (r >> 16) as usize;
                match ctype {
                    5 => {
                        let pe = self.core.select_chains[sel].triggering_effect.unwrap();
                        if self.effect(pe).type_ as u32 & EFFECT_TYPE_CONTINUOUS != 0 {
                            self.core.select_chains.clear();
                            let hp = pe.get_handler_player(self);
                            let ne = self.nil_event.clone();
                            self.solve_continuous(hp, pe, &ne);
                            arg.step = 2;
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
                    }
                    0 => {
                        arg.step = 4;
                        false
                    }
                    1 => {
                        arg.step = 5;
                        false
                    }
                    2 => {
                        arg.step = 6;
                        false
                    }
                    3 => {
                        arg.step = 7;
                        false
                    }
                    4 => {
                        arg.step = 8;
                        false
                    }
                    8 => {
                        arg.step = units::RESTART;
                        self.shuffle(tp, LOCATION_HAND as u8);
                        self.infos.can_shuffle = false;
                        false
                    }
                    _ => {
                        arg.step = 9;
                        let m = self.new_message(MSG_HINT);
                        self.msg(m).u8(HINT_EVENT).u8(1 - tp).u64(23u64);
                        self.core.select_chains.clear();
                        self.core.hint_timing[tp as usize] = TIMING_MAIN_END;
                        self.emplace_process(units::QuickEffect { step: 0, skip_freechain: false, is_opponent: false, priority_player: 1 - tp });
                        self.infos.priorities[tp as usize] = 1;
                        self.infos.priorities[1 - tp as usize] = 0;
                        arg.phase_to_change_to = ctype as u8;
                        false
                    }
                }
            }
            2 => {
                self.core.chain_limit.clear();
                for ch in self.core.current_chain.clone() {
                    let h = ch.triggering_effect.unwrap().get_handler(self);
                    self.card_mut(h).set_status(STATUS_CHAINING, false);
                }
                self.emplace_process(units::SolveChain { step: 0, skip_trigger: false, skip_freechain: false, skip_new: false, backed_up_operation: None });
                arg.step = units::RESTART;
                false
            }
            3 => {
                self.adjust_instant();
                self.emplace_process(units::PointEvent { step: 0, skip_trigger: false, skip_freechain: false, skip_new: false });
                arg.step = units::RESTART;
                false
            }
            5 => {
                let target = self.core.summonable_cards[(self.returns.at_i32(0) >> 16) as usize];
                self.core.summon_cancelable = 1;
                self.summon(tp, target, None, false, 0, 0x1f);
                arg.step = units::RESTART;
                false
            }
            6 => {
                let target = self.core.spsummonable_cards[(self.returns.at_i32(0) >> 16) as usize];
                self.core.summon_cancelable = 1;
                self.special_summon_rule(tp, target, 0);
                arg.step = units::RESTART;
                false
            }
            7 => {
                let target = self.core.repositionable_cards[(self.returns.at_i32(0) >> 16) as usize];
                let pos = self.card(target).current.position;
                if pos & POS_FACEUP_ATTACK != 0 {
                    self.core.phase_action = true;
                    self.change_position_one(target, None, tp, POS_FACEUP_DEFENSE as u8, 0, false);
                    self.adjust_all();
                    self.emplace_process(units::PointEvent { step: 0, skip_trigger: false, skip_freechain: false, skip_new: false });
                } else if pos & POS_FACEUP_DEFENSE != 0 {
                    self.core.phase_action = true;
                    self.change_position_one(target, None, tp, POS_FACEUP_ATTACK as u8, 0, false);
                    self.adjust_all();
                    self.emplace_process(units::PointEvent { step: 0, skip_trigger: false, skip_freechain: false, skip_new: false });
                } else if pos & POS_FACEDOWN_ATTACK != 0 {
                    arg.card_to_reposition = Some(target);
                    let mut positions = 0u8;
                    if target.is_capable_change_position(self, tp) {
                        positions |= POS_FACEDOWN_DEFENSE as u8;
                    }
                    if target.is_can_be_flip_summoned(self, tp) {
                        positions |= POS_FACEUP_ATTACK as u8;
                    }
                    let code = self.card(target).data.code;
                    self.emplace_process(units::SelectPosition { step: 0, playerid: tp, positions, code });
                    arg.step = 12;
                    return false;
                } else {
                    let ctrl = self.card(target).current.controler;
                    self.emplace_process(units::FlipSummon { step: 0, sumplayer: ctrl, target, flip_summon_cost_effects: Vec::new() });
                }
                self.card_mut(target).set_status(STATUS_FORM_CHANGED, true);
                arg.step = units::RESTART;
                false
            }
            8 => {
                let target = self.core.msetable_cards[(self.returns.at_i32(0) >> 16) as usize];
                self.core.summon_cancelable = 1;
                let ctrl = self.card(target).current.controler;
                self.mset(ctrl, target, None, false, 0, 0x1f);
                arg.step = units::RESTART;
                false
            }
            9 => {
                let target = self.core.ssetable_cards[(self.returns.at_i32(0) >> 16) as usize];
                let ctrl = self.card(target).current.controler;
                self.emplace_process(units::SpellSet { step: 0, setplayer: ctrl, toplayer: ctrl, target, reason_effect: None });
                arg.step = units::RESTART;
                false
            }
            10 => {
                self.core.chain_limit.clear();
                if !self.core.current_chain.is_empty() {
                    for ch in self.core.current_chain.clone() {
                        let h = ch.triggering_effect.unwrap().get_handler(self);
                        self.card_mut(h).set_status(STATUS_CHAINING, false);
                    }
                    self.emplace_process(units::SolveChain { step: 0, skip_trigger: false, skip_freechain: false, skip_new: false, backed_up_operation: None });
                    arg.step = units::RESTART;
                    return false;
                }
                let ph = self.infos.phase as u32;
                self.reset_phase(ph);
                self.adjust_all();
                false
            }
            11 => {
                self.returns.set_i32(0, arg.phase_to_change_to as i32);
                self.infos.can_shuffle = true;
                true
            }
            12 => {
                arg.phase_to_change_to = if self.returns.at_i32(0) == 0 { 6 } else { 7 };
                let ph = self.infos.phase as u32;
                self.reset_phase(ph);
                self.adjust_all();
                arg.step = 10;
                false
            }
            13 => {
                let target = arg.card_to_reposition.unwrap();
                if self.returns.at_i32(0) == POS_FACEUP_ATTACK as i32 {
                    let ctrl = self.card(target).current.controler;
                    self.emplace_process(units::FlipSummon { step: 0, sumplayer: ctrl, target, flip_summon_cost_effects: Vec::new() });
                } else {
                    self.core.phase_action = true;
                    self.change_position_one(target, None, tp, POS_FACEDOWN_DEFENSE as u8, 0, false);
                    self.adjust_all();
                    self.emplace_process(units::PointEvent { step: 0, skip_trigger: false, skip_freechain: false, skip_new: false });
                }
                self.card_mut(target).set_status(STATUS_FORM_CHANGED, true);
                arg.step = units::RESTART;
                false
            }
            _ => true,
        }
    }
}
