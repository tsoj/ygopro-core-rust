//! The processor (`processor_visit.cpp`, `processor.cpp` part 1): the unit loop, card functions
//! run in replay scopes, events, phase events and the chain windows.

use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use crate::constants::*;
use crate::duel::{Chain, Duel, Event};
use crate::scope::{Interrupt, Scope, StopHere, StopPoint};
use crate::types::*;
use crate::units::{self, Unit};

/// `OCG_DuelStatus`
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Status {
    End = 0,
    Awaiting = 1,
    Continue = 2,
}

/// Which function of an effect an `Execute*` unit runs.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ExecKind {
    Cost,
    Target,
    Operation,
}

pub fn events_equal(a: &Event, b: &Event) -> bool {
    a.trigger_card == b.trigger_card
        && a.event_cards == b.event_cards
        && a.reason_effect == b.reason_effect
        && a.event_code == b.event_code
        && a.event_value == b.event_value
        && a.reason == b.reason
        && a.event_player == b.event_player
        && a.reason_player == b.reason_player
        && a.global_id == b.global_id
}

impl Duel {
    fn splice_subunits(&mut self) {
        while let Some(u) = self.core.subunits.pop_back() {
            self.core.units.push_front(u);
        }
    }

    /// `field::process`: runs one step of the first unit.
    pub fn process(&mut self) -> Status {
        if self.scope_waiting() {
            // a card function waits for the response given now: run it again from the start
            return self.restart_scope();
        }
        self.splice_subunits();
        self.collect_groups();
        let Some(mut unit) = self.core.units.pop_front() else {
            return Status::End;
        };
        let opens_scope = self.scope.is_none() && matches!(unit, Unit::ExecuteCost(_) | Unit::ExecuteTarget(_) | Unit::ExecuteOperation(_));
        let result = if opens_scope {
            let mut snapshot = self.clone();
            snapshot.core.units.push_front(unit.clone());
            self.scope = Some(Box::new(Scope {
                snapshot: std::rc::Rc::new(snapshot),
                log: Vec::new(),
                cursor: 0,
                delivered_messages: 0,
                skip_messages: 0,
                delivered_logs: 0,
                skip_logs: 0,
                running: true,
                waiting_at: StopPoint::Question(0),
                ops: Vec::new(),
                stops_done: 0,
                stop_cursor: 0,
            }));
            self.run_scoped(&mut unit)
        } else {
            Some(self.process_unit(&mut unit))
        };
        self.finish_step(unit, result)
    }

    fn finish_step(&mut self, mut unit: Unit, result: Option<bool>) -> Status {
        match result {
            // (the scope waits: for a response, or at a stop for the caller to go on)
            None if self.scope.as_ref().is_some_and(|s| matches!(s.waiting_at, StopPoint::Stop(_))) => Status::Continue,
            None => Status::Awaiting,
            Some(true) => Status::Continue,
            Some(false) => {
                let s = unit.step_mut();
                *s = s.wrapping_add(1);
                let na = unit.needs_answer();
                self.core.units.push_front(unit);
                if na { Status::Awaiting } else { Status::Continue }
            }
        }
    }

    fn run_scoped(&mut self, unit: &mut Unit) -> Option<bool> {
        let result = catch_unwind(AssertUnwindSafe(|| self.process_unit(unit)));
        match result {
            Ok(done) => {
                // (the messages of this run go out while the scope can still drop those an
                // earlier run sent)
                self.generate_buffer();
                self.scope = None;
                Some(done)
            }
            Err(payload) => {
                if payload.is::<Interrupt>() {
                    if let Some(s) = self.scope.as_mut() {
                        s.running = false;
                        s.waiting_at = StopPoint::Question(s.log.len());
                    }
                    None
                } else if payload.is::<StopHere>() {
                    if let Some(s) = self.scope.as_mut() {
                        s.running = false;
                    }
                    None
                } else {
                    resume_unwind(payload)
                }
            }
        }
    }

    /// The response arrived: back to the state before the card function, which runs again.
    fn restart_scope(&mut self) -> Status {
        let mut scope = self.scope.take().unwrap();
        let buff = std::mem::take(&mut self.buff);
        let logs = std::mem::take(&mut self.logs);
        let mut restored = (*scope.snapshot).clone();
        restored.buff = buff;
        restored.logs = logs;
        *self = restored;
        scope.cursor = 0;
        scope.stop_cursor = 0;
        scope.skip_messages = scope.delivered_messages;
        scope.skip_logs = scope.delivered_logs;
        scope.running = true;
        self.splice_subunits();
        let mut unit = self.core.units.pop_front().expect("scope unit");
        self.scope = Some(scope);
        let result = self.run_scoped(&mut unit);
        self.finish_step(unit, result)
    }

    /// Whether `process` should report a question now (a scope waiting for a response).
    pub fn scope_waiting(&self) -> bool {
        self.scope.as_ref().is_some_and(|s| !s.running)
    }

    /// A response for a question asked inside a running scope: from the log, or interrupt.
    pub fn scope_take_response(&mut self) {
        let scope = self.scope.as_mut().expect("a question outside of a card function's scope");
        if scope.cursor < scope.log.len() {
            let k = scope.cursor;
            let r = scope.log[k].clone();
            scope.cursor += 1;
            // (the changes made from outside while the scope waited at this question)
            let ops: Vec<_> = scope.ops.iter().filter(|(at, _)| *at == StopPoint::Question(k)).map(|(_, op)| op.clone()).collect();
            for op in ops {
                op(self);
            }
            self.returns.data = r;
        } else {
            resume_unwind(Box::new(Interrupt));
        }
    }

    /// Processes the units pushed by a card function until they are done (the Lua API functions
    /// that yield to the core); questions inside take their responses from the scope.
    pub fn run_pushed_units(&mut self) {
        // The C++ yields here, and on resume `Execute*` sets reason_effect/reason_player back to the
        // triggering effect (processor.cpp), even if a nested Execute cleared them when it ended.
        let (reason_effect, reason_player) = (self.core.reason_effect, self.core.reason_player);
        // (the C++ step that ran the card function's code up to here ends)
        self.scope_stop_point();
        self.run_pushed_units_inner();
        self.core.reason_effect = reason_effect;
        self.core.reason_player = reason_player;
    }
    fn run_pushed_units_inner(&mut self) {
        let base = self.core.units.len();
        loop {
            self.splice_subunits();
            if self.core.units.len() <= base {
                break;
            }
            let mut unit = self.core.units.pop_front().unwrap();
            let done = self.process_unit(&mut unit);
            if !done {
                let s = unit.step_mut();
                *s = s.wrapping_add(1);
                let na = unit.needs_answer();
                self.core.units.push_front(unit);
                if na {
                    self.scope_take_response();
                    continue;
                }
            }
            self.scope_stop_point();
        }
    }

    /// With `fine_steps`: the end of a step of a running card function, where the C++ core returns
    /// from `OCG_DuelProcess` if the step sent messages (its Lua code yields to the core between
    /// steps). A stop passed in an earlier run of the scope is passed again (its messages were sent,
    /// the changes made from outside there are made again); at a new one the card function is
    /// interrupted, its messages are sent and `process` says "continue": the caller can change the
    /// duel there, as between the C++ core's steps.
    pub fn scope_stop_point(&mut self) {
        if !self.fine_steps {
            return;
        }
        let Some(s) = self.scope.as_mut() else { return };
        if !s.running || self.messages.iter().all(|m| m.data.is_empty()) {
            return;
        }
        let k = s.stop_cursor;
        s.stop_cursor += 1;
        if k < s.stops_done {
            let ops: Vec<_> = s.ops.iter().filter(|(at, _)| *at == StopPoint::Stop(k)).map(|(_, op)| op.clone()).collect();
            self.generate_buffer(); // (sent by an earlier run: dropped)
            for op in ops {
                op(self);
            }
            return;
        }
        self.generate_buffer();
        let s = self.scope.as_mut().unwrap();
        s.stops_done = k + 1;
        s.waiting_at = StopPoint::Stop(k);
        resume_unwind(Box::new(StopHere));
    }

    /// `field::process(Processors::Execute*)`
    pub fn process_execute(&mut self, kind: ExecKind, triggering_effect: EffectId, triggering_player: u8, shuffle_check_was_disabled: &mut bool) -> bool {
        let f = {
            let e = self.effect(triggering_effect);
            match kind {
                ExecKind::Cost => e.cost.clone(),
                ExecKind::Target => e.target.clone(),
                ExecKind::Operation => e.operation.clone(),
            }
        };
        while let Some(ev) = self.core.sub_solving_event.pop_back() {
            self.core.solving_event.push_front(ev);
        }
        let Some(f) = f else {
            self.params.clear();
            self.core.solving_event.pop_front();
            return true;
        };
        let e = self.core.solving_event.front().cloned().unwrap_or_default();
        if kind != ExecKind::Operation {
            self.add_param_front(Value::Int(1));
        }
        self.add_param_front(Value::Int(e.reason_player as i64));
        self.add_param_front(Value::Int(e.reason as i64));
        self.add_param_front(Value::effect(e.reason_effect));
        self.add_param_front(Value::Int(e.event_value as i64));
        self.add_param_front(Value::Int(e.event_player as i64));
        self.add_param_front(Value::from_group(&e.event_cards));
        self.add_param_front(Value::Int(triggering_player as i64));
        self.add_param_front(Value::Effect(triggering_effect));
        if self.core.check_level == 0 {
            self.core.shuffle_deck_check = [false, false];
            self.core.shuffle_hand_check = [false, false];
        }
        *shuffle_check_was_disabled = self.core.shuffle_check_disabled;
        self.core.shuffle_check_disabled = false;
        self.core.check_level += 1;
        self.core.reason_effect = Some(triggering_effect);
        self.core.reason_player = triggering_player;
        let args = std::mem::take(&mut self.params);
        self.call_depth += 1;
        let ret = self.protected_call(&f, &args).unwrap_or(Value::Nil);
        self.call_depth -= 1;
        if self.call_depth == 0 {
            self.restore_assumes();
        }
        self.returns.set_i32(0, ret.int() as i32);
        self.core.reason_effect = None;
        self.core.reason_player = PLAYER_NONE as u8;
        self.core.check_level -= 1;
        if self.core.check_level == 0 {
            if self.core.shuffle_hand_check[0] {
                self.shuffle(0, LOCATION_HAND as u8);
            }
            if self.core.shuffle_hand_check[1] {
                self.shuffle(1, LOCATION_HAND as u8);
            }
            if self.core.shuffle_deck_check[0] {
                self.shuffle(0, LOCATION_DECK as u8);
            }
            if self.core.shuffle_deck_check[1] {
                self.shuffle(1, LOCATION_DECK as u8);
            }
        }
        self.core.shuffle_check_disabled = *shuffle_check_was_disabled;
        self.core.solving_event.pop_front();
        true
    }

    #[allow(clippy::too_many_arguments)]
    pub fn raise_event_card(&mut self, event_card: Option<CardId>, event_code: u32, reason_effect: Option<EffectId>, reason: u32, reason_player: u8, event_player: u8, event_value: u32) {
        let ev = Event {
            trigger_card: None,
            event_cards: event_card.map(CardSet::from_card),
            event_code,
            reason_effect,
            reason,
            reason_player,
            event_player,
            event_value,
            global_id: self.infos.event_id,
        };
        self.core.queue_event.push_back(ev);
    }
    #[allow(clippy::too_many_arguments)]
    pub fn raise_event(&mut self, event_cards: CardSet, event_code: u32, reason_effect: Option<EffectId>, reason: u32, reason_player: u8, event_player: u8, event_value: u32) {
        let ev = Event {
            trigger_card: None,
            event_cards: Some(event_cards),
            event_code,
            reason_effect,
            reason,
            reason_player,
            event_player,
            event_value,
            global_id: self.infos.event_id,
        };
        self.core.queue_event.push_back(ev);
    }
    #[allow(clippy::too_many_arguments)]
    pub fn raise_single_event(
        &mut self,
        trigger_card: CardId,
        event_cards: Option<&CardSet>,
        event_code: u32,
        reason_effect: Option<EffectId>,
        reason: u32,
        reason_player: u8,
        event_player: u8,
        event_value: u32,
    ) {
        let ev = Event {
            trigger_card: Some(trigger_card),
            event_cards: event_cards.cloned(),
            event_code,
            reason_effect,
            reason,
            reason_player,
            event_player,
            event_value,
            global_id: self.infos.event_id,
        };
        self.core.single_event.push_back(ev);
    }
    pub fn check_event(&self, code: u32, pe: Option<&mut Event>) -> bool {
        for ev in self.core.point_event.iter().chain(self.core.instant_event.iter()) {
            if ev.event_code == code {
                if let Some(p) = pe {
                    *p = ev.clone();
                }
                return true;
            }
        }
        false
    }
    pub fn check_event_c(&mut self, peffect: EffectId, playerid: u8, neglect_con: bool, neglect_cost: bool, copy_info: bool, mut pe: Option<&mut Event>) -> bool {
        let code = self.effect(peffect).code;
        let re = self.core.reason_effect.unwrap_or(peffect);
        if code == EVENT_FREE_CHAIN {
            let ne = self.nil_event.clone();
            return peffect.is_activate_ready_by(self, re, playerid, &ne, neglect_con, neglect_cost, false);
        }
        let events: Vec<Event> = self.core.point_event.iter().chain(self.core.instant_event.iter()).cloned().collect();
        for ev in events {
            if ev.event_code == code && peffect.is_activate_ready_by(self, re, playerid, &ev, neglect_con, neglect_cost, false) {
                if copy_info && self.no_action == 0 && !self.core.current_chain.is_empty() {
                    self.core.current_chain.last_mut().unwrap().evt = ev.clone();
                }
                if let Some(p) = pe.as_mut() {
                    **p = ev;
                }
                return true;
            }
        }
        false
    }
    pub fn check_hint_timing(&mut self, peffect: EffectId) -> bool {
        let p = peffect.get_handler_player(self);
        let ht = self.effect(peffect).hint_timing;
        if p == 0 {
            (ht[0] & self.core.hint_timing[0]) != 0 || (ht[1] & self.core.hint_timing[1]) != 0
        } else {
            (ht[0] & self.core.hint_timing[1]) != 0 || (ht[1] & self.core.hint_timing[0]) != 0
        }
    }
    /// `chain::set_triggering_state`
    pub fn set_triggering_state(&mut self, ch: &mut Chain, pcard: CardId) {
        let cur = self.card(pcard).current;
        ch.triggering_controler = cur.controler;
        ch.triggering_location = cur.location as u16;
        if cur.location as u32 & (LOCATION_MZONE | LOCATION_SZONE) != 0 {
            let extra = if cur.is_location(LOCATION_FZONE) {
                LOCATION_FZONE
            } else if cur.is_location(LOCATION_PZONE) {
                LOCATION_PZONE
            } else if cur.is_location(LOCATION_STZONE) {
                LOCATION_STZONE
            } else if cur.is_location(LOCATION_EMZONE) {
                LOCATION_EMZONE
            } else if cur.is_location(LOCATION_MMZONE) {
                LOCATION_MMZONE
            } else {
                0
            };
            ch.triggering_location |= extra as u16;
        }
        let c = self.card(pcard);
        ch.triggering_sequence = cur.sequence;
        ch.triggering_position = cur.position as u8;
        ch.triggering_status = c.status;
        ch.triggering_summon_type = c.summon.type_ & 0xff00ffff;
        ch.triggering_summon_location = c.summon.location;
        ch.triggering_summon_proc_complete = c.is_status(STATUS_PROC_COMPLETE);
        ch.triggering_state.code = pcard.get_code(self);
        ch.triggering_state.code2 = pcard.get_another_code(self);
        ch.triggering_state.level = pcard.get_level(self);
        ch.triggering_state.rank = pcard.get_rank(self);
        ch.triggering_state.link = pcard.get_link(self);
        ch.triggering_state.lscale = pcard.get_lscale(self);
        ch.triggering_state.rscale = pcard.get_rscale(self);
        ch.triggering_state.attribute = pcard.get_attribute(self, None, 0, 2);
        ch.triggering_state.type_ = pcard.get_type(self, None, 0, 2);
        ch.triggering_state.race = pcard.get_race(self, None, 0, 2);
        ch.triggering_state.attack = pcard.get_attack(self);
        ch.triggering_state.defense = pcard.get_defense(self);
        let mut sc = std::collections::BTreeSet::new();
        pcard.get_set_card(self, &mut sc);
        ch.triggering_state.setcodes = SetCodes::default();
        for s in sc {
            ch.triggering_state.setcodes.insert(s);
        }
    }

    fn new_chain_for(&mut self, peffect: EffectId, ev: &Event, triggering_player: u8) -> Chain {
        let phandler = peffect.get_handler(self);
        let mut ch = Chain { flag: 0, evt: ev.clone(), triggering_effect: Some(peffect), ..Default::default() };
        ch.chain_id = self.infos.field_id as u16;
        self.infos.field_id += 1;
        self.set_triggering_state(&mut ch, phandler);
        ch.triggering_player = triggering_player;
        ch
    }

    /// core.spe_effect[p]: # of optional trigger effects, activate/quick effect with hints
    pub fn process_phase_event(&mut self, arg: &mut units::PhaseEvent) -> bool {
        let phase = arg.phase as u32;
        let tp = self.infos.turn_player;
        match arg.step {
            0 => {
                let skip = |d: &mut Duel, code| d.core.force_turn_end || d.is_player_affected_by_effect(tp, code).is_some();
                if (phase == PHASE_DRAW && skip(self, EFFECT_SKIP_DP))
                    || (phase == PHASE_STANDBY && skip(self, EFFECT_SKIP_SP))
                    || (phase == PHASE_BATTLE_START && skip(self, EFFECT_SKIP_BP))
                    || (phase == PHASE_BATTLE && skip(self, EFFECT_SKIP_BP))
                    || (phase == PHASE_END && skip(self, EFFECT_SKIP_EP))
                {
                    arg.step = 24;
                    return false;
                }
                let phase_event = EVENT_PHASE + phase;
                self.nil_event.event_code = phase_event;
                self.nil_event.event_player = tp;
                let check_player = if arg.is_opponent { 1 - tp } else { tp };
                self.core.select_chains.clear();
                let (mut tf_count, mut to_count, mut fc_count, mut cn_count) = (0, 0, 0, 0);
                let ne = self.nil_event.clone();
                for pe in self.field_effects.trigger_f_effect.range(phase_event) {
                    pe.set_activate_location(self);
                    if !pe.is_activateable(self, check_player, &ne, false, false, false, false, false) {
                        continue;
                    }
                    let id = self.infos.field_id;
                    self.infos.field_id += 1;
                    self.effect_mut(pe).id = id;
                    self.core.select_chains.push_back(Chain { triggering_effect: Some(pe), ..Default::default() });
                    tf_count += 1;
                }
                for pe in self.field_effects.continuous_effect.range(phase_event) {
                    if pe.get_handler_player(self) != check_player || !pe.is_activateable(self, check_player, &ne, false, false, false, false, false) {
                        continue;
                    }
                    let id = self.infos.field_id;
                    self.infos.field_id += 1;
                    self.effect_mut(pe).id = id;
                    self.core.select_chains.push_back(Chain { triggering_effect: Some(pe), ..Default::default() });
                    cn_count += 1;
                }
                // all effects taking control non-permanently are only until End Phase, not until Turn end
                let mut pheff = self.field_effects.pheff.clone();
                crate::effect::sort_effects_by_id(self, &mut pheff);
                for pe in pheff {
                    let e = self.effect(pe);
                    if e.code != EFFECT_SET_CONTROL || e.reset_flag & phase == 0 {
                        continue;
                    }
                    let pid = pe.get_handler_player(self);
                    if pid != check_player {
                        continue;
                    }
                    let rf = self.effect(pe).reset_flag;
                    if !((rf & RESET_SELF_TURN != 0 && pid == tp) || (rf & RESET_OPPO_TURN != 0 && pid != tp)) {
                        continue;
                    }
                    if self.effect(pe).reset_count != 1 {
                        continue;
                    }
                    let ph = pe.get_handler(self);
                    if pe.get_value_card(self, Some(ph), 0) != self.card(ph).current.controler as i64 {
                        continue;
                    }
                    self.core.select_chains.push_back(Chain { triggering_effect: Some(pe), ..Default::default() });
                    cn_count += 1;
                }
                self.core.spe_effect[check_player as usize] = 0;
                if !self.core.hand_adjusted {
                    for pe in self.field_effects.trigger_o_effect.range(phase_event) {
                        pe.set_activate_location(self);
                        if !pe.is_activateable(self, check_player, &ne, false, false, false, false, false) {
                            continue;
                        }
                        let id = self.infos.field_id;
                        self.infos.field_id += 1;
                        self.effect_mut(pe).id = id;
                        self.core.select_chains.push_back(Chain { triggering_effect: Some(pe), ..Default::default() });
                        to_count += 1;
                        self.core.spe_effect[check_player as usize] += 1;
                    }
                    self.core.hint_timing[tp as usize] = match phase {
                        PHASE_DRAW => TIMING_DRAW_PHASE,
                        PHASE_STANDBY => TIMING_STANDBY_PHASE,
                        PHASE_BATTLE_START => TIMING_BATTLE_START,
                        PHASE_BATTLE => TIMING_BATTLE_END,
                        _ => TIMING_END_PHASE,
                    };
                    for pe in self.field_effects.activate_effect.range(EVENT_FREE_CHAIN) {
                        pe.set_activate_location(self);
                        if !pe.is_chainable(self, check_player) || !pe.is_activateable(self, check_player, &ne, false, false, false, false, false) {
                            continue;
                        }
                        let id = self.infos.field_id;
                        self.infos.field_id += 1;
                        self.effect_mut(pe).id = id;
                        self.core.select_chains.push_back(Chain { triggering_effect: Some(pe), ..Default::default() });
                        if self.check_hint_timing(pe) || self.check_cteffect_hint(pe, check_player) {
                            self.core.spe_effect[check_player as usize] += 1;
                        }
                        fc_count += 1;
                    }
                    for pe in self.field_effects.quick_o_effect.range(EVENT_FREE_CHAIN) {
                        pe.set_activate_location(self);
                        if !pe.is_chainable(self, check_player) || !pe.is_activateable(self, check_player, &ne, false, false, false, false, false) {
                            continue;
                        }
                        let id = self.infos.field_id;
                        self.infos.field_id += 1;
                        self.effect_mut(pe).id = id;
                        self.core.select_chains.push_back(Chain { triggering_effect: Some(pe), ..Default::default() });
                        if self.check_hint_timing(pe) {
                            self.core.spe_effect[check_player as usize] += 1;
                        }
                        fc_count += 1;
                    }
                    for pe in self.field_effects.continuous_effect.range(EVENT_FREE_CHAIN) {
                        if pe.get_handler_player(self) != check_player || !pe.is_activateable(self, check_player, &ne, false, false, false, false, false) {
                            continue;
                        }
                        let id = self.infos.field_id;
                        self.infos.field_id += 1;
                        self.effect_mut(pe).id = id;
                        self.core.select_chains.push_back(Chain { triggering_effect: Some(pe), ..Default::default() });
                        fc_count += 1;
                    }
                }
                if self.core.select_chains.is_empty() {
                    self.returns.set_i32(0, -1);
                    arg.step = 1;
                    false
                } else if tf_count == 0 && cn_count == 1 && to_count == 0 && fc_count == 0 {
                    self.returns.set_i32(0, 0);
                    arg.step = 1;
                    false
                } else {
                    let hint: u64 = match self.infos.phase as u32 {
                        PHASE_DRAW => 20,
                        PHASE_STANDBY => 21,
                        PHASE_BATTLE_START => 28,
                        PHASE_BATTLE => 25,
                        _ => 26,
                    };
                    let m = self.new_message(MSG_HINT);
                    self.msg(m).u8(HINT_EVENT).u8(check_player).u64(hint);
                    if tf_count == 0 && to_count == 1 && fc_count == 0 && cn_count == 0 {
                        let h = self.core.select_chains.front().unwrap().triggering_effect.unwrap().get_handler(self);
                        self.emplace_process(units::SelectEffectYesNo { step: 0, playerid: check_player, pcard: Some(h), description: 0 });
                        false
                    } else {
                        let spe = self.core.spe_effect[check_player as usize] as u8;
                        self.emplace_process(units::SelectChain { step: 0, playerid: check_player, spe_count: spe, forced: (tf_count + cn_count) > 0 });
                        arg.step = 1;
                        false
                    }
                }
            }
            1 => {
                let v = self.returns.at_i32(0);
                self.returns.set_i32(0, v - 1);
                false
            }
            2 => {
                if self.returns.at_i32(0) == -1 {
                    if arg.priority_passed {
                        arg.step = 19;
                    } else {
                        arg.priority_passed = true;
                        arg.is_opponent = !arg.is_opponent;
                        arg.step = units::RESTART;
                    }
                    return false;
                }
                arg.priority_passed = false;
                let idx = self.returns.at_i32(0) as usize;
                let pe = self.core.select_chains[idx].triggering_effect.unwrap();
                let phandler = pe.get_handler(self);
                let typ = self.effect(pe).type_ as u32;
                if typ & EFFECT_TYPE_ACTIONS == 0 {
                    if self.effect(pe).is_flag(EFFECT_FLAG_FIELD_ONLY) {
                        self.remove_effect(pe);
                    } else {
                        let h = self.effect(pe).handler.unwrap();
                        h.remove_effect(self, pe);
                    }
                    self.adjust_all();
                    arg.step = 3;
                } else if typ & EFFECT_TYPE_CONTINUOUS == 0 {
                    let check_player = if arg.is_opponent { 1 - tp } else { tp };
                    let mut ch = self.core.select_chains.remove(idx).unwrap();
                    ch.flag = 0;
                    ch.chain_id = self.infos.field_id as u16;
                    self.infos.field_id += 1;
                    ch.evt = self.nil_event.clone();
                    self.set_triggering_state(&mut ch, phandler);
                    ch.triggering_player = check_player;
                    self.core.new_chains.push_back(ch);
                    self.card_mut(phandler).set_status(STATUS_CHAINING, true);
                    pe.dec_count(self, check_player as u32);
                    self.core.select_chains.clear();
                    self.emplace_process(units::AddChain { step: 0, is_activated_effect: false });
                    let pp = if self.is_flag(DUEL_INVERTED_QUICK_PRIORITY) { check_player } else { 1 - check_player };
                    self.emplace_process(units::QuickEffect { step: 0, skip_freechain: false, is_opponent: false, priority_player: pp });
                    self.infos.priorities = [0, 0];
                } else {
                    self.core.select_chains.clear();
                    let hp = pe.get_handler_player(self);
                    let ne = self.nil_event.clone();
                    self.solve_continuous(hp, pe, &ne);
                    arg.step = 3;
                }
                false
            }
            3 => {
                self.core.chain_limit.clear();
                for ch in self.core.current_chain.clone() {
                    let h = ch.triggering_effect.unwrap().get_handler(self);
                    self.card_mut(h).set_status(STATUS_CHAINING, false);
                }
                self.emplace_process(units::SolveChain { step: 0, skip_trigger: false, skip_freechain: false, skip_new: false, backed_up_operation: None });
                arg.step = units::RESTART;
                false
            }
            4 => {
                self.adjust_instant();
                self.emplace_process(units::PointEvent { step: 0, skip_trigger: false, skip_freechain: false, skip_new: false });
                arg.step = units::RESTART;
                false
            }
            20 => {
                if phase != PHASE_END {
                    arg.step = 24;
                    return false;
                }
                let mut limit = 6;
                let mut eset = Vec::new();
                self.filter_player_effect(tp, EFFECT_HAND_LIMIT, &mut eset, true);
                if let Some(&last) = eset.last() {
                    limit = last.get_value(self, 0) as i32;
                }
                let hd = self.player[tp as usize].list_hand.len() as i32;
                if hd <= limit || self.is_flag(DUEL_NO_HAND_LIMIT) {
                    arg.step = 24;
                    return false;
                }
                self.core.select_cards = self.player[tp as usize].list_hand.clone();
                let m = self.new_message(MSG_HINT);
                self.msg(m).u8(HINT_SELECTMSG).u8(tp).u64(501u64);
                let to_discard = (hd - limit) as u8;
                self.emplace_process(units::SelectCard { step: 0, playerid: tp, cancelable: false, min: to_discard, max: to_discard });
                false
            }
            21 => {
                if !self.return_cards.list.is_empty() {
                    let set = CardSet::from_iter(self.return_cards.list.iter().copied());
                    self.send_to(set, None, REASON_RULE + REASON_DISCARD + REASON_ADJUST, tp, PLAYER_NONE as u8, LOCATION_GRAVE as u16, 0, POS_FACEUP as u8, false);
                }
                false
            }
            22 => {
                self.core.hand_adjusted = true;
                self.emplace_process(units::PointEvent { step: 0, skip_trigger: false, skip_freechain: false, skip_new: false });
                arg.step = units::RESTART;
                arg.is_opponent = false;
                arg.priority_passed = false;
                false
            }
            25 => {
                self.core.hint_timing[tp as usize] = 0;
                self.reset_phase(phase);
                self.adjust_all();
                false
            }
            26 => {
                self.core.quick_f_chain.clear();
                self.core.instant_event.clear();
                self.core.point_event.clear();
                self.core.delayed_activate_event.clear();
                self.core.full_event.clear();
                true
            }
            _ => true,
        }
    }

    fn update_trigger_chain(&mut self, ch: &mut Chain, include_hand_relation: bool) {
        let pe = ch.triggering_effect.unwrap();
        let phandler = pe.get_handler(self);
        let mut updated = false;
        if phandler.is_has_relation_chain(self, pe, ch.chain_id) {
            self.set_triggering_state(ch, phandler);
            updated = true;
        }
        if include_hand_relation {
            let e = self.effect(pe);
            if !e.is_flag(EFFECT_FLAG_FIELD_ONLY)
                && e.type_ as u32 & EFFECT_TYPE_FIELD != 0
                && e.range as u32 & LOCATION_HAND != 0
                && self.card(phandler).current.location as u32 == LOCATION_HAND
            {
                let ctrl = self.card(phandler).current.controler;
                if !phandler.is_has_relation_chain(self, pe, ch.chain_id) && pe.is_condition_check(self, ctrl, &ch.evt.clone()) {
                    phandler.create_relation_chain(self, pe, ch.chain_id);
                }
                pe.set_activate_location(self);
                ch.triggering_player = self.card(phandler).current.controler;
                if !updated {
                    self.set_triggering_state(ch, phandler);
                    updated = true;
                }
            }
        }
        let ctrl = self.card(phandler).current.controler;
        if ch.triggering_player != ctrl && !self.effect(pe).is_flag(EFFECT_FLAG_EVENT_PLAYER) {
            ch.triggering_player = ctrl;
            if !updated {
                self.set_triggering_state(ch, phandler);
            }
        }
    }

    fn tcg_segoc_first(&mut self) {
        if self.is_flag(DUEL_TCG_SEGOC_FIRSTTRIGGER) {
            self.core.select_chains.make_contiguous().sort_by_key(|c| c.event_id);
            let id = self.core.select_chains.front().unwrap().event_id;
            let end = self.core.select_chains.iter().position(|c| c.event_id != id).unwrap_or(self.core.select_chains.len());
            self.core.select_chains.truncate(end);
        }
    }

    /// core.ignition_priority_chains: used in step 8 (obsolete ignition effect ruling)
    pub fn process_point_event(&mut self, arg: &mut units::PointEvent) -> bool {
        let (skip_trigger, skip_freechain, skip_new) = (arg.skip_trigger, arg.skip_freechain, arg.skip_new);
        let tp = self.infos.turn_player;
        let done = match arg.step {
            0 => {
                self.core.select_chains.clear();
                let ie = std::mem::take(&mut self.core.instant_event);
                self.core.point_event.extend(ie);
                if skip_trigger {
                    arg.step = 7;
                    return false;
                }
                let mut nf = std::mem::take(&mut self.core.new_fchain);
                nf.extend(std::mem::take(&mut self.core.new_fchain_s));
                self.core.new_fchain_s = nf;
                let mut no = std::mem::take(&mut self.core.new_ochain);
                no.extend(std::mem::take(&mut self.core.new_ochain_s));
                self.core.new_ochain_s = no;
                let dae = std::mem::take(&mut self.core.delayed_activate_event);
                self.core.full_event.extend(dae);
                self.core.delayed_quick.clear();
                std::mem::swap(&mut self.core.delayed_quick_tmp, &mut self.core.delayed_quick);
                self.core.current_player = tp;
                arg.step = 1;
                return false;
            }
            1 => return false,
            2 => {
                // forced trigger
                self.core.select_chains.clear();
                let mut i = 0;
                while i < self.core.new_fchain_s.len() {
                    let mut ch = self.core.new_fchain_s[i].clone();
                    self.update_trigger_chain(&mut ch, false);
                    let pe = ch.triggering_effect.unwrap();
                    let phandler = pe.get_handler(self);
                    let ctp = ch.triggering_player;
                    let pk = self.is_flag(DUEL_TRIGGER_WHEN_PRIVATE_KNOWLEDGE);
                    let evt = ch.evt.clone();
                    self.core.new_fchain_s[i] = ch.clone();
                    if (!ch.was_just_sent || self.card(phandler).current.location as u32 == LOCATION_HAND)
                        && self.check_trigger_effect(&ch)
                        && pe.is_chainable(self, ctp)
                        && pe.is_activateable(self, ctp, &evt, true, false, false, pk, pk)
                    {
                        if ctp == self.core.current_player {
                            self.core.select_chains.push_back(ch);
                        }
                    } else {
                        self.effect_mut(pe).active_type = 0;
                        self.core.new_fchain_s.remove(i);
                        continue;
                    }
                    i += 1;
                }
                if self.core.select_chains.is_empty() {
                    self.returns.set_i32(0, -1);
                } else {
                    self.tcg_segoc_first();
                    if self.core.select_chains.len() == 1 {
                        self.returns.set_i32(0, 0);
                    } else {
                        let cp = self.core.current_player;
                        self.emplace_process(units::SelectChain { step: 0, playerid: cp, spe_count: 0x7f, forced: true });
                    }
                }
                return false;
            }
            3 => {
                if self.returns.at_i32(0) == -1 {
                    if !self.core.new_fchain_s.is_empty() {
                        self.core.current_player = 1 - tp;
                        arg.step = 1;
                    } else {
                        self.core.current_player = tp;
                    }
                    return false;
                }
                let idx = self.returns.at_i32(0) as usize;
                let ch = self.core.select_chains.remove(idx).unwrap();
                let pe = ch.triggering_effect.unwrap();
                let ctp = ch.triggering_player;
                let h = pe.get_handler(self);
                self.card_mut(h).set_status(STATUS_CHAINING, true);
                pe.dec_count(self, ctp as u32);
                let chain_id = ch.chain_id;
                self.core.new_chains.push_back(ch);
                self.emplace_process(units::AddChain { step: 0, is_activated_effect: false });
                self.core.new_fchain_s.retain(|c| c.chain_id != chain_id);
                arg.step = 1;
                return false;
            }
            4 => {
                // optional trigger
                self.core.select_chains.clear();
                let mut i = 0;
                while i < self.core.new_ochain_s.len() {
                    let mut ch = self.core.new_ochain_s[i].clone();
                    self.update_trigger_chain(&mut ch, true);
                    let pe = ch.triggering_effect.unwrap();
                    let phandler = pe.get_handler(self);
                    let ctp = ch.triggering_player;
                    let pk = self.is_flag(DUEL_TRIGGER_WHEN_PRIVATE_KNOWLEDGE);
                    let evt = ch.evt.clone();
                    let ok = (!ch.was_just_sent || self.card(phandler).current.location as u32 == LOCATION_HAND)
                        && (pk || self.check_nonpublic_trigger(&mut ch))
                        && self.check_trigger_effect(&ch)
                        && pe.is_chainable(self, ctp)
                        && pe.is_activateable(self, ctp, &evt, true, false, false, pk, pk)
                        && self.check_spself_from_hand_trigger(&ch);
                    self.core.new_ochain_s[i] = ch.clone();
                    if ok {
                        if ctp == self.core.current_player {
                            self.core.select_chains.push_back(ch);
                        }
                    } else {
                        self.effect_mut(pe).active_type = 0;
                        self.core.new_ochain_s.remove(i);
                        continue;
                    }
                    i += 1;
                }
                if self.core.select_chains.is_empty() {
                    self.returns.set_i32(0, -2);
                    arg.step = 5;
                    return false;
                }
                if self.is_flag(DUEL_TCG_SEGOC_NONPUBLIC) {
                    self.core.new_ochain_h.clear();
                }
                self.tcg_segoc_first();
                if self.core.select_chains.len() == 1 && self.core.current_chain.is_empty() {
                    let h = self.core.select_chains.front().unwrap().triggering_effect.unwrap().get_handler(self);
                    let cp = self.core.current_player;
                    self.emplace_process(units::SelectEffectYesNo { step: 0, playerid: cp, pcard: Some(h), description: 221 });
                    return false;
                }
                let cp = self.core.current_player;
                self.emplace_process(units::SelectChain { step: 0, playerid: cp, spe_count: 0x7f, forced: false });
                arg.step = 5;
                return false;
            }
            5 => {
                let v = self.returns.at_i32(0);
                self.returns.set_i32(0, v - 1);
                return false;
            }
            6 => {
                let ret = self.returns.at_i32(0);
                if ret == -2 || (ret == -1 && !self.is_flag(DUEL_TCG_SEGOC_FIRSTTRIGGER)) {
                    for ch in self.core.select_chains.clone() {
                        let pe = ch.triggering_effect.unwrap();
                        self.effect_mut(pe).active_type = 0;
                        self.core.new_ochain_s.retain(|c| c.chain_id != ch.chain_id);
                    }
                    if !self.core.new_ochain_s.is_empty() {
                        self.core.current_player = 1 - tp;
                        arg.step = 3;
                    } else {
                        self.core.current_player = tp;
                        arg.step = 6;
                    }
                    return false;
                } else if ret == -1 {
                    let discardedid = self.core.select_chains.front().unwrap().event_id;
                    let cp = self.core.current_player;
                    self.core.new_ochain_s.retain(|c| !(c.event_id == discardedid && c.triggering_player == cp));
                    arg.step = 3;
                    return false;
                }
                let ch = self.core.select_chains.remove(ret as usize).unwrap();
                let pe = ch.triggering_effect.unwrap();
                let ctp = ch.triggering_player;
                let h = pe.get_handler(self);
                self.card_mut(h).set_status(STATUS_CHAINING, true);
                pe.dec_count(self, ctp as u32);
                let chain_id = ch.chain_id;
                self.core.new_chains.push_back(ch);
                self.emplace_process(units::AddChain { step: 0, is_activated_effect: false });
                self.core.new_ochain_s.retain(|c| c.chain_id != chain_id);
                self.core.new_ochain_h.retain(|c| c.chain_id != chain_id);
                arg.step = 3;
                return false;
            }
            7 => {
                self.core.select_chains.clear();
                return false;
            }
            8 => {
                let ph = self.infos.phase as u32;
                if skip_freechain
                    || !(self.is_flag(DUEL_OCG_OBSOLETE_IGNITION) || self.is_flag(DUEL_TCG_FAST_EFFECT_IGNITION))
                    || (ph != PHASE_MAIN1 && ph != PHASE_MAIN2)
                {
                    return false;
                }
                let check_events_ocg = |d: &Duel| {
                    if d.check_event(EVENT_CHAIN_END, None) {
                        return true;
                    }
                    let mut e = Event::default();
                    if !d.check_event(EVENT_SUMMON_SUCCESS, Some(&mut e))
                        && !d.check_event(EVENT_SPSUMMON_SUCCESS, Some(&mut e))
                        && !d.check_event(EVENT_FLIP_SUMMON_SUCCESS, Some(&mut e))
                    {
                        return false;
                    }
                    e.reason_player == d.infos.turn_player
                };
                if self.core.current_chain.is_empty() && (self.is_flag(DUEL_TCG_FAST_EFFECT_IGNITION) || check_events_ocg(self)) {
                    let mut newchain = Chain::default();
                    newchain.evt.event_player = PLAYER_NONE as u8;
                    newchain.evt.reason_player = PLAYER_NONE as u8;
                    newchain.flag = 0;
                    newchain.triggering_player = tp;
                    for pe in self.field_effects.ignition_effect.all() {
                        let phandler = pe.get_handler(self);
                        newchain.evt.event_code = self.effect(pe).code;
                        let evt = newchain.evt.clone();
                        if (self.is_flag(DUEL_TCG_FAST_EFFECT_IGNITION) || self.card(phandler).current.location as u32 == LOCATION_MZONE)
                            && pe.is_chainable(self, tp)
                            && pe.is_activateable(self, tp, &evt, false, false, false, false, false)
                        {
                            newchain.chain_id = self.infos.field_id as u16;
                            self.infos.field_id += 1;
                            newchain.triggering_effect = Some(pe);
                            let mut nc = newchain.clone();
                            self.set_triggering_state(&mut nc, phandler);
                            newchain = nc.clone();
                            self.core.ignition_priority_chains.push_back(nc);
                        }
                    }
                }
                return false;
            }
            9 => {
                self.infos.priorities = [0, 0];
                if self.core.current_chain.is_empty() {
                    if !self.core.hand_adjusted {
                        let pp = if self.is_flag(DUEL_INVERTED_QUICK_PRIORITY) { 1 - tp } else { tp };
                        self.emplace_process(units::QuickEffect { step: 0, skip_freechain, is_opponent: false, priority_player: pp });
                    }
                } else {
                    let pp = 1 - self.core.current_chain.last().unwrap().triggering_player;
                    self.emplace_process(units::QuickEffect { step: 0, skip_freechain, is_opponent: false, priority_player: pp });
                }
                return false;
            }
            10 => {
                self.core.new_ochain_h.clear();
                self.core.full_event.clear();
                self.core.delayed_quick.clear();
                self.core.chain_limit.clear();
                if !self.core.current_chain.is_empty() {
                    for ch in self.core.current_chain.clone() {
                        let h = ch.triggering_effect.unwrap().get_handler(self);
                        self.card_mut(h).set_status(STATUS_CHAINING, false);
                    }
                    self.emplace_process(units::SolveChain { step: 0, skip_trigger, skip_freechain, skip_new, backed_up_operation: None });
                } else {
                    // (the core keeps the events in used_event for Lua's garbage collector: nothing here)
                    self.core.point_event.clear();
                    self.reset_chain();
                    self.returns.set_i32(0, 0);
                }
                true
            }
            30 | 32 => {
                let check_player = if arg.step == 30 { tp } else { 1 - tp };
                self.nil_event.event_code = EVENT_FREE_CHAIN;
                self.core.select_chains.clear();
                self.core.spe_effect[check_player as usize] = 0;
                let ne = self.nil_event.clone();
                for pe in self.field_effects.continuous_effect.range(EVENT_FREE_CHAIN) {
                    if pe.get_handler_player(self) == check_player && pe.is_activateable(self, check_player, &ne, false, false, false, false, false) {
                        self.core.select_chains.push_back(Chain { triggering_effect: Some(pe), ..Default::default() });
                        self.core.spe_effect[check_player as usize] += 1;
                    }
                }
                if !self.core.select_chains.is_empty() {
                    let spe = self.core.spe_effect[check_player as usize] as u8;
                    self.emplace_process(units::SelectChain { step: 0, playerid: check_player, spe_count: spe, forced: false });
                } else if arg.step == 30 {
                    arg.step = 31;
                } else {
                    arg.step = units::RESTART;
                }
                return false;
            }
            31 | 33 => {
                if self.returns.at_i32(0) == -1 {
                    if arg.step == 33 {
                        arg.step = units::RESTART;
                    }
                    return false;
                }
                let idx = self.returns.at_i32(0) as usize;
                let pe = self.core.select_chains[idx].triggering_effect.unwrap();
                self.core.select_chains.clear();
                let hp = pe.get_handler_player(self);
                let ne = self.nil_event.clone();
                self.solve_continuous(hp, pe, &ne);
                arg.step = if arg.step == 31 { 29 } else { 31 };
                return false;
            }
            _ => true,
        };
        if self.core.set_forced_attack {
            self.core.set_forced_attack = false;
            self.emplace_process(units::ForcedBattle { step: 0, backup_phase: 0 });
        }
        done
    }

    pub fn process_quick_effect(&mut self, arg: &mut units::QuickEffect) -> bool {
        let skip_freechain = arg.skip_freechain;
        let priority = arg.priority_player;
        let tp = self.infos.turn_player;
        match arg.step {
            0 => {
                let check_player = if arg.is_opponent { 1 - tp } else { tp };
                self.core.select_chains.clear();
                let mut i = 0;
                while i < self.core.quick_f_chain.len() {
                    let (pe, ch) = self.core.quick_f_chain[i].clone();
                    let phandler = pe.get_handler(self);
                    if pe.is_chainable(self, ch.triggering_player)
                        && pe.check_count_limit(self, ch.triggering_player)
                        && phandler.is_has_relation_chain(self, pe, ch.chain_id)
                    {
                        if ch.triggering_player == check_player {
                            self.core.select_chains.push_back(ch);
                        }
                    } else {
                        self.core.quick_f_chain.remove(i);
                        continue;
                    }
                    i += 1;
                }
                {
                    let d = &*self;
                    let mut v: Vec<Chain> = d.core.select_chains.iter().cloned().collect();
                    v.sort_by(|a, b| {
                        let (ea, eb) = (d.effect(a.triggering_effect.unwrap()), d.effect(b.triggering_effect.unwrap()));
                        (ea.initial_id, ea.id).cmp(&(eb.initial_id, eb.id))
                    });
                    self.core.select_chains = v.into();
                }
                if self.core.select_chains.is_empty() {
                    self.returns.set_i32(0, -1);
                } else if self.core.select_chains.len() == 1 {
                    self.returns.set_i32(0, 0);
                } else {
                    self.emplace_process(units::SelectChain { step: 0, playerid: check_player, spe_count: 0, forced: true });
                }
                false
            }
            1 => {
                if self.returns.at_i32(0) == -1 {
                    if !self.core.quick_f_chain.is_empty() {
                        arg.is_opponent = true;
                        arg.step = units::RESTART;
                    } else if !self.core.new_chains.is_empty() {
                        self.emplace_process(units::AddChain { step: 0, is_activated_effect: false });
                        let pp = 1 - self.core.new_chains.back().unwrap().triggering_player;
                        self.emplace_process(units::QuickEffect { step: 0, skip_freechain: false, is_opponent: false, priority_player: pp });
                        self.infos.priorities = [0, 0];
                        return true;
                    }
                    return false;
                }
                let idx = self.returns.at_i32(0) as usize;
                let ch = self.core.select_chains.remove(idx).unwrap();
                let pe = ch.triggering_effect.unwrap();
                let ctp = ch.triggering_player;
                let h = pe.get_handler(self);
                self.card_mut(h).set_status(STATUS_CHAINING, true);
                pe.dec_count(self, ctp as u32);
                self.core.new_chains.push_back(ch);
                self.core.quick_f_chain.retain(|x| x.0 != pe);
                arg.step = units::RESTART;
                false
            }
            2 => {
                if !self.core.ignition_priority_chains.is_empty() {
                    std::mem::swap(&mut self.core.select_chains, &mut self.core.ignition_priority_chains);
                }
                let events: Vec<Event> = self.core.point_event.iter().chain(self.core.instant_event.iter()).cloned().collect();
                for ev in &events {
                    for cont in 0..2 {
                        let list = if cont == 0 {
                            self.field_effects.activate_effect.range(ev.event_code)
                        } else {
                            self.field_effects.quick_o_effect.range(ev.event_code)
                        };
                        for pe in list {
                            pe.set_activate_location(self);
                            if !self.effect(pe).is_flag(EFFECT_FLAG_DELAY)
                                && pe.is_chainable(self, priority)
                                && pe.is_activateable(self, priority, ev, false, false, false, false, false)
                            {
                                let ch = self.new_chain_for(pe, ev, priority);
                                self.core.select_chains.push_back(ch);
                            }
                        }
                    }
                }
                let mut j = 0;
                while j < self.core.new_ochain_h.len() {
                    let mut ch = self.core.new_ochain_h[j].clone();
                    let pe = ch.triggering_effect.unwrap();
                    let phandler = pe.get_handler(self);
                    let e = self.effect(pe);
                    if !e.is_flag(EFFECT_FLAG_FIELD_ONLY)
                        && e.type_ as u32 & EFFECT_TYPE_FIELD != 0
                        && e.range as u32 & LOCATION_HAND != 0
                        && self.card(phandler).current.location as u32 == LOCATION_HAND
                    {
                        let ctrl = self.card(phandler).current.controler;
                        if !phandler.is_has_relation_chain(self, pe, ch.chain_id) && pe.is_condition_check(self, ctrl, &ch.evt.clone()) {
                            phandler.create_relation_chain(self, pe, ch.chain_id);
                        }
                        pe.set_activate_location(self);
                        ch.triggering_player = self.card(phandler).current.controler;
                        self.set_triggering_state(&mut ch, phandler);
                    }
                    self.core.new_ochain_h[j] = ch.clone();
                    let evt = ch.evt.clone();
                    if ch.triggering_player == priority
                        && !self.card(phandler).is_status(STATUS_CHAINING)
                        && ((ch.triggering_location as u32 == LOCATION_HAND && self.card(phandler).is_position(POS_FACEDOWN) != 0)
                            || ch.triggering_location as u32 == LOCATION_DECK)
                        && phandler.is_has_relation_chain(self, pe, ch.chain_id)
                        && pe.is_chainable(self, priority)
                        && pe.is_activateable(self, priority, &evt, true, false, false, false, false)
                        && self.check_spself_from_hand_trigger(&ch)
                    {
                        self.core.select_chains.push_back(ch);
                    }
                    j += 1;
                }
                // delayed activate
                for ev in self.core.full_event.clone() {
                    for pe in self.field_effects.activate_effect.range(ev.event_code) {
                        pe.set_activate_location(self);
                        if self.effect(pe).is_flag(EFFECT_FLAG_DELAY)
                            && pe.is_chainable(self, priority)
                            && pe.is_activateable(self, priority, &ev, false, false, false, false, false)
                        {
                            let ch = self.new_chain_for(pe, &ev, priority);
                            self.core.select_chains.push_back(ch);
                        }
                    }
                }
                // delayed quick
                for (pe, evt) in self.core.delayed_quick.clone() {
                    pe.set_activate_location(self);
                    if pe.is_chainable(self, priority) && pe.is_activateable(self, priority, &evt, true, false, false, false, false) {
                        let ch = self.new_chain_for(pe, &evt, priority);
                        self.core.select_chains.push_back(ch);
                    }
                }
                self.core.spe_effect[priority as usize] = self.core.select_chains.len() as i32;
                if !skip_freechain {
                    self.nil_event.event_code = EVENT_FREE_CHAIN;
                    let ne = self.nil_event.clone();
                    for pe in self.field_effects.activate_effect.range(EVENT_FREE_CHAIN) {
                        pe.set_activate_location(self);
                        if pe.is_chainable(self, priority) && pe.is_activateable(self, priority, &ne, false, false, false, false, false) {
                            let ch = self.new_chain_for(pe, &ne, priority);
                            self.core.select_chains.push_back(ch);
                            if self.check_hint_timing(pe) || self.check_cteffect_hint(pe, priority) {
                                self.core.spe_effect[priority as usize] += 1;
                            }
                        }
                    }
                    for pe in self.field_effects.quick_o_effect.range(EVENT_FREE_CHAIN) {
                        pe.set_activate_location(self);
                        if pe.is_chainable(self, priority) && pe.is_activateable(self, priority, &ne, false, false, false, false, false) {
                            let ch = self.new_chain_for(pe, &ne, priority);
                            self.core.select_chains.push_back(ch);
                            if self.check_hint_timing(pe) {
                                self.core.spe_effect[priority as usize] += 1;
                            }
                        }
                    }
                }
                if !self.core.current_chain.is_empty() || self.core.hint_timing[0] & TIMING_ATTACK != 0 || self.core.hint_timing[1] & TIMING_ATTACK != 0 {
                    self.core.spe_effect[priority as usize] = self.core.select_chains.len() as i32;
                }
                let spe = self.core.spe_effect[priority as usize] as u8;
                self.emplace_process(units::SelectChain { step: 0, playerid: priority, spe_count: spe, forced: false });
                false
            }
            3 => {
                if !self.core.select_chains.is_empty() && self.returns.at_i32(0) != -1 {
                    let idx = self.returns.at_i32(0) as usize;
                    let ch = self.core.select_chains.remove(idx).unwrap();
                    let pe = ch.triggering_effect.unwrap();
                    if let Some(k) = self.core.delayed_quick.iter().position(|(e, ev)| *e == pe && events_equal(ev, &ch.evt)) {
                        self.core.delayed_quick.remove(k);
                    }
                    self.core.new_chains.push_back(ch);
                    let h = pe.get_handler(self);
                    self.card_mut(h).set_status(STATUS_CHAINING, true);
                    pe.dec_count(self, priority as u32);
                    self.emplace_process(units::AddChain { step: 0, is_activated_effect: false });
                    self.emplace_process(units::QuickEffect { step: 0, skip_freechain: false, is_opponent: false, priority_player: 1 - priority });
                    self.infos.priorities = [0, 0];
                } else {
                    self.infos.priorities[priority as usize] = 1;
                    if self.infos.priorities[0] == 0 || self.infos.priorities[1] == 0 {
                        self.emplace_process(units::QuickEffect { step: 1, skip_freechain, is_opponent: false, priority_player: 1 - priority });
                    } else {
                        self.core.hint_timing[0] &= TIMING_DAMAGE_STEP | TIMING_DAMAGE_CAL;
                        self.core.hint_timing[1] &= TIMING_DAMAGE_STEP | TIMING_DAMAGE_CAL;
                    }
                }
                self.core.select_chains.clear();
                true
            }
            _ => true,
        }
    }

    fn continuous_chain(&self, peffect: EffectId, owner_player: u8, ev: &Event) -> Chain {
        Chain {
            chain_id: 0,
            chain_count: 0,
            triggering_effect: Some(peffect),
            triggering_player: owner_player,
            evt: ev.clone(),
            target_cards: None,
            target_player: PLAYER_NONE as u8,
            target_param: 0,
            disable_player: PLAYER_NONE as u8,
            disable_reason: None,
            flag: 0,
            ..Default::default()
        }
    }

    fn push_continuous(&mut self, peffect: EffectId, owner_player: u8, ev: &Event, tpl: &mut Vec<Chain>, ntpl: &mut Vec<Chain>) {
        let ch = self.continuous_chain(peffect, owner_player, ev);
        if self.effect(peffect).is_flag(EFFECT_FLAG_DELAY) && (self.core.chain_solving || self.core.conti_solving) {
            if owner_player == self.infos.turn_player {
                self.core.delayed_continuous_tp.push_back(ch);
            } else {
                self.core.delayed_continuous_ntp.push_back(ch);
            }
        } else if owner_player == self.infos.turn_player {
            tpl.push(ch);
        } else {
            ntpl.push(ch);
        }
    }

    pub fn process_instant_event(&mut self) -> i32 {
        if self.core.queue_event.is_empty() {
            return 1;
        }
        let mut tpl: Vec<Chain> = Vec::new();
        let mut ntpl: Vec<Chain> = Vec::new();
        let queue: Vec<Event> = self.core.queue_event.iter().cloned().collect();
        for ev in &queue {
            // continuous events
            for pe in self.field_effects.continuous_effect.range(ev.event_code) {
                let mut owner_player = pe.get_handler_player(self);
                if self.effect(pe).is_flag(EFFECT_FLAG_EVENT_PLAYER) && (ev.event_player == 0 || ev.event_player == 1) {
                    owner_player = ev.event_player;
                }
                if pe.is_activateable(self, owner_player, ev, false, false, false, false, false) {
                    self.push_continuous(pe, owner_player, ev, &mut tpl, &mut ntpl);
                }
            }
            if ev.event_code == EVENT_ADJUST || ev.event_code == EVENT_BREAK_EFFECT || (ev.event_code & 0xfffff000) == EVENT_PHASE_START {
                continue;
            }
            let check_simul = |d: &Duel, pe: EffectId, ph: CardId| d.effect(pe).flag[1] & EFFECT_FLAG2_CHECK_SIMULTANEOUS != 0 && d.core.just_sent_cards.contains(ph);
            // triggers
            for pe in self.field_effects.trigger_f_effect.range(ev.event_code) {
                let phandler = pe.get_handler(self);
                let ctrl = self.card(phandler).current.controler;
                if !self.card(phandler).is_status(STATUS_EFFECT_ENABLED) || !pe.is_condition_check(self, ctrl, ev) {
                    continue;
                }
                let was_just_sent = check_simul(self, pe, phandler);
                if was_just_sent && self.effect(pe).range as u32 & LOCATION_HAND == 0 {
                    continue;
                }
                pe.set_activate_location(self);
                let mut ch = Chain { was_just_sent, flag: 0, event_id: ev.global_id, evt: ev.clone(), triggering_effect: Some(pe), ..Default::default() };
                ch.chain_id = self.infos.field_id as u16;
                self.infos.field_id += 1;
                self.set_triggering_state(&mut ch, phandler);
                ch.triggering_player = if self.effect(pe).is_flag(EFFECT_FLAG_EVENT_PLAYER) && (ev.event_player == 0 || ev.event_player == 1) {
                    ev.event_player
                } else {
                    self.card(phandler).current.controler
                };
                phandler.create_relation_chain(self, pe, ch.chain_id);
                self.core.new_fchain.push_back(ch);
            }
            for pe in self.field_effects.trigger_o_effect.range(ev.event_code) {
                let phandler = pe.get_handler(self);
                let ctrl = self.card(phandler).current.controler;
                let act = self.card(phandler).is_status(STATUS_EFFECT_ENABLED) && pe.is_condition_check(self, ctrl, ev);
                let was_just_sent = check_simul(self, pe, phandler);
                let range = self.effect(pe).range as u32;
                if was_just_sent && range & LOCATION_HAND == 0 {
                    continue;
                }
                if range & LOCATION_HAND == 0 && !act {
                    continue;
                }
                pe.set_activate_location(self);
                let mut ch = Chain { was_just_sent, flag: 0, event_id: ev.global_id, evt: ev.clone(), triggering_effect: Some(pe), ..Default::default() };
                ch.chain_id = self.infos.field_id as u16;
                self.infos.field_id += 1;
                self.set_triggering_state(&mut ch, phandler);
                ch.triggering_player = if self.effect(pe).is_flag(EFFECT_FLAG_EVENT_PLAYER) && (ev.event_player == 0 || ev.event_player == 1) {
                    ev.event_player
                } else {
                    self.card(phandler).current.controler
                };
                if self.effect(pe).is_flag(EFFECT_FLAG_FIELD_ONLY) || range & LOCATION_HAND == 0 || (range & self.card(phandler).current.location as u32 != 0 && act) {
                    phandler.create_relation_chain(self, pe, ch.chain_id);
                }
                self.core.new_ochain.push_back(ch);
            }
            // instant_f
            for pe in self.field_effects.quick_f_effect.range(ev.event_code) {
                let phandler = pe.get_handler(self);
                pe.set_activate_location(self);
                let ctrl = self.card(phandler).current.controler;
                if pe.is_activateable(self, ctrl, ev, false, false, false, false, false) {
                    let mut ch = Chain { flag: 0, evt: ev.clone(), triggering_effect: Some(pe), ..Default::default() };
                    ch.chain_id = self.infos.field_id as u16;
                    self.infos.field_id += 1;
                    self.set_triggering_state(&mut ch, phandler);
                    ch.triggering_player = if self.effect(pe).is_flag(EFFECT_FLAG_EVENT_PLAYER) && (ev.event_player == 0 || ev.event_player == 1) {
                        ev.event_player
                    } else {
                        self.card(phandler).current.controler
                    };
                    phandler.create_relation_chain(self, pe, ch.chain_id);
                    // (core.quick_f_chain[peffect]: an existing entry is overwritten field by field)
                    match self.core.quick_f_chain.iter_mut().find(|x| x.0 == pe) {
                        Some(x) => {
                            let old = std::mem::take(&mut x.1);
                            x.1 = Chain { ..ch };
                            x.1.target_cards = old.target_cards;
                            x.1.opinfos = old.opinfos;
                            x.1.possibleopinfos = old.possibleopinfos;
                        }
                        None => {
                            let pos = self.core.quick_f_chain.len();
                            self.core.quick_f_chain.insert(pos, (pe, ch));
                        }
                    }
                }
            }
            // delayed activate effect
            self.core.delayed_activate_event.push_back(ev.clone());
            // delayed quick effect
            for pe in self.field_effects.quick_o_effect.range(ev.event_code) {
                let ctrl = self.card(pe.get_handler(self)).current.controler;
                if self.effect(pe).is_flag(EFFECT_FLAG_DELAY) && pe.is_condition_check(self, ctrl, ev) {
                    if !self.core.delayed_quick_tmp.iter().any(|(e, x)| *e == pe && events_equal(x, ev)) {
                        self.core.delayed_quick_tmp.push((pe, ev.clone()));
                    }
                }
            }
        }
        for ch in tpl.into_iter().chain(ntpl) {
            self.core.sub_solving_continuous.push_back(ch);
            self.emplace_process(units::SolveContinuous { step: 0, reason_player: 0, reason_effect: None });
        }
        let qe = std::mem::take(&mut self.core.queue_event);
        self.core.instant_event.extend(qe);
        1
    }

    pub fn process_single_event(&mut self) -> i32 {
        if self.core.single_event.is_empty() {
            return 1;
        }
        let mut tpl: Vec<Chain> = Vec::new();
        let mut ntpl: Vec<Chain> = Vec::new();
        let events: Vec<Event> = self.core.single_event.iter().cloned().collect();
        for ev in &events {
            let starget = ev.trigger_card.unwrap();
            for pe in self.card(starget).single_effect().range(ev.event_code) {
                self.process_single_event_effect(pe, ev, &mut tpl, &mut ntpl);
            }
            for mat in self.card(starget).xyz_materials.clone() {
                for pe in self.card(mat).xmaterial_effect().range(ev.event_code) {
                    if self.effect(pe).type_ as u32 & EFFECT_TYPE_FIELD != 0 {
                        continue;
                    }
                    self.process_single_event_effect(pe, ev, &mut tpl, &mut ntpl);
                }
            }
        }
        for ch in tpl.into_iter().chain(ntpl) {
            self.core.sub_solving_continuous.push_back(ch);
            self.emplace_process(units::SolveContinuous { step: 0, reason_player: 0, reason_effect: None });
        }
        self.core.single_event.clear();
        1
    }

    fn process_single_event_effect(&mut self, peffect: EffectId, e: &Event, tpl: &mut Vec<Chain>, ntpl: &mut Vec<Chain>) -> bool {
        let typ = self.effect(peffect).type_ as u32;
        if typ & EFFECT_TYPE_ACTIONS == 0 {
            return false;
        }
        if typ & EFFECT_TYPE_FLIP != 0 && e.event_value & (NO_FLIP_EFFECT >> 16) != 0 {
            return false;
        }
        if typ & EFFECT_TYPE_CONTINUOUS != 0 {
            let mut owner_player = peffect.get_handler_player(self);
            if self.effect(peffect).is_flag(EFFECT_FLAG_EVENT_PLAYER) && (e.event_player == 0 || e.event_player == 1) {
                owner_player = e.event_player;
            }
            if peffect.is_activateable(self, owner_player, e, false, false, false, false, false) {
                self.push_continuous(peffect, owner_player, e, tpl, ntpl);
            }
        } else {
            let phandler = peffect.get_handler(self);
            let ctrl = self.card(phandler).current.controler;
            if !peffect.is_condition_check(self, ctrl, e) {
                return false;
            }
            peffect.set_activate_location(self);
            let mut ch = Chain { flag: 0, event_id: e.global_id, evt: e.clone(), triggering_effect: Some(peffect), ..Default::default() };
            ch.chain_id = self.infos.field_id as u16;
            self.infos.field_id += 1;
            self.set_triggering_state(&mut ch, phandler);
            if self.effect(peffect).is_flag(EFFECT_FLAG_EVENT_PLAYER) && (e.event_player == 0 || e.event_player == 1) {
                ch.triggering_player = e.event_player;
            } else if self.card(phandler).current.reason & REASON_TEMPORARY != 0 {
                ch.triggering_player = self.card(phandler).previous.controler;
            } else {
                ch.triggering_player = ch.triggering_controler;
            }
            peffect.set_active_type(self);
            phandler.create_relation_chain(self, peffect, ch.chain_id);
            let chain_id = ch.chain_id;
            let trig_o = typ & EFFECT_TYPE_TRIGGER_O != 0;
            if self.core.flip_delayed && e.event_code == EVENT_FLIP {
                if trig_o {
                    self.core.new_ochain_b.push_back(ch);
                } else {
                    self.core.new_fchain_b.push_back(ch);
                }
            } else if trig_o {
                self.core.new_ochain.push_back(ch);
            } else {
                self.core.new_fchain.push_back(ch);
            }
            if let Some(deffect) = phandler.is_affected_by_effect(self, EFFECT_DISABLE_EFFECT) {
                let negeff = self.new_effect();
                let dv = deffect.get_value(self, 0) as u32;
                let downer = self.effect(deffect).owner;
                let ne = self.effect_mut(negeff);
                ne.owner = downer;
                ne.type_ = EFFECT_TYPE_SINGLE as u16;
                ne.code = EFFECT_DISABLE_CHAIN;
                ne.value = chain_id as i32;
                ne.reset_flag = RESET_EVENT | dv;
                phandler.add_effect(self, negeff);
            }
        }
        true
    }
}
