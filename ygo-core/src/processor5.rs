//! The processor (`processor.cpp` part 5: building and solving chains, continuous effects,
//! breaking effects and win checks, adjustments).

use crate::card::public_order;
use crate::constants::*;
use crate::duel::{Chain, Duel, Event};
use crate::types::*;
use crate::units;

impl Duel {
    pub fn process_add_chain(&mut self, arg: &mut units::AddChain) -> bool {
        match arg.step {
            0 => {
                let Some(clit) = self.core.new_chains.front() else { return true };
                let pe = clit.triggering_effect.unwrap();
                arg.is_activated_effect = false;
                if self.effect(pe).type_ as u32 & EFFECT_TYPE_ACTIVATE != 0 {
                    arg.step = 9;
                }
                false
            }
            1 => {
                let clit = self.core.new_chains.front().unwrap().clone();
                let tpl = clit.triggering_player;
                let mut eset = Vec::new();
                self.filter_player_effect(tpl, EFFECT_ACTIVATE_COST, &mut eset, true);
                for peff in eset {
                    let t = self.effect(peff).target.clone();
                    self.add_param(Value::Effect(peff));
                    self.add_param(Value::effect(clit.triggering_effect));
                    self.add_param(Value::Int(tpl as i64));
                    if !self.check_condition(t.as_ref(), 3) {
                        continue;
                    }
                    if self.effect(peff).operation.is_some() {
                        self.core.sub_solving_event.push_back(clit.evt.clone());
                        self.emplace_process(units::ExecuteOperation { step: 0, triggering_player: tpl, shuffle_check_was_disabled: false, triggering_effect: peff });
                    }
                }
                if !arg.is_activated_effect {
                    return false;
                }
                let pe = clit.triggering_effect.unwrap();
                let phandler = pe.get_handler(self);
                let in_hand = self.card(phandler).current.location as u32 == LOCATION_HAND;
                self.card_mut(phandler).set_status(STATUS_ACT_FROM_HAND, in_hand);
                let ctrl = self.card(phandler).current.controler;
                if self.card(phandler).current.location as u32 == LOCATION_SZONE {
                    self.change_position_one(phandler, None, ctrl, POS_FACEUP as u8, 0, false);
                } else {
                    let mut zone: u32 = 0xff;
                    let htype = self.card(phandler).data.type_;
                    if htype & (TYPE_FIELD | TYPE_PENDULUM) == 0 && self.effect(pe).is_flag(EFFECT_FLAG_LIMIT_ZONE) {
                        self.add_param(Value::Int(tpl as i64));
                        self.add_param(Value::from_group(&clit.evt.event_cards));
                        self.add_param(Value::Int(clit.evt.event_player as i64));
                        self.add_param(Value::Int(clit.evt.event_value as i64));
                        self.add_param(Value::effect(clit.evt.reason_effect));
                        self.add_param(Value::Int(clit.evt.reason as i64));
                        self.add_param(Value::Int(clit.evt.reason_player as i64));
                        zone = pe.get_value(self, 7) as u32;
                        if zone == 0 {
                            return true;
                        }
                    }
                    let mut loc = LOCATION_SZONE;
                    if self.effect(pe).is_flag2(EFFECT_FLAG2_FORCE_ACTIVATE_LOCATION) {
                        loc = pe.get_value(self, 0) as u32;
                        if loc == 0 {
                            return true;
                        }
                    } else if in_hand {
                        if htype & TYPE_PENDULUM != 0 {
                            loc = LOCATION_PZONE;
                        } else if htype & TYPE_FIELD != 0 {
                            loc = LOCATION_FZONE;
                        }
                    }
                    phandler.enable_field_effect(self, false);
                    let pos = if loc == LOCATION_MZONE { POS_FACEUP_ATTACK } else { POS_FACEUP };
                    self.move_to_field(phandler, ctrl, ctrl, loc as u16, pos as u8, false, 0, zone as u8, false, units::LocationReason::None, true);
                }
                false
            }
            2 => {
                let mut clit = self.core.new_chains.front().unwrap().clone();
                let pe = clit.triggering_effect.unwrap();
                let phandler = pe.get_handler(self);
                if self.effect(pe).type_ as u32 & EFFECT_TYPE_ACTIVATE != 0 {
                    self.set_triggering_state(&mut clit, phandler);
                }
                let code = self.card(phandler).data.code;
                let loc = phandler.get_info_location(self);
                let desc = self.effect(pe).description;
                let n = self.core.current_chain.len() as u32 + 1;
                let m = self.new_message(MSG_CHAINING);
                self.msg(m).u32(code).loc(loc).u8(clit.triggering_controler).u8(clit.triggering_location as u8).u32(clit.triggering_sequence).u64(desc).u32(n);
                self.core.chain_limit.clear();
                let mut ct = phandler.get_type(self, None, 0, 2);
                if ct & (TYPE_TRAP | TYPE_MONSTER) == (TYPE_TRAP | TYPE_MONSTER) {
                    ct -= TYPE_TRAP;
                }
                self.effect_mut(pe).card_type = ct;
                let etype = self.effect(pe).type_ as u32;
                if self.is_flag(DUEL_TRIGGER_WHEN_PRIVATE_KNOWLEDGE) {
                    if !(etype & EFFECT_TYPE_CONTINUOUS == 0 && etype & EFFECT_TYPE_SINGLE != 0) {
                        pe.set_active_type(self);
                    }
                } else {
                    pe.set_active_type(self);
                }
                let h = self.effect(pe).handler.unwrap();
                let ot = self.card(h).overlay_target;
                self.effect_mut(pe).active_handler = ot;
                clit.chain_count = self.core.current_chain.len() as u8 + 1;
                clit.target_cards = None;
                clit.target_player = PLAYER_NONE as u8;
                clit.target_param = 0;
                clit.disable_reason = None;
                clit.disable_player = PLAYER_NONE as u8;
                clit.replace_op = None;
                if self.card(phandler).current.location as u32 == LOCATION_HAND {
                    clit.flag |= CHAIN_HAND_EFFECT;
                }
                self.core.current_chain.push(clit.clone());
                let counters = self.check_chain_counter(pe, clit.triggering_player as i32, clit.chain_count as i32);
                self.core.current_chain.last_mut().unwrap().applied_chain_counters = counters;
                // triggered events which are not caused by RaiseEvent create relation with the handler
                let ecode = self.effect(pe).code;
                if !self.effect(pe).is_flag(EFFECT_FLAG_FIELD_ONLY) && (etype & 0x2a0 == 0 || (ecode & 0xfffff000) == EVENT_PHASE) {
                    phandler.create_relation_chain(self, pe, clit.chain_id);
                }
                self.effect_mut(pe).effect_owner = clit.triggering_player;
                // DISABLE_CHAIN should be check before cost
                if !self.effect(pe).is_flag(EFFECT_FLAG_FIELD_ONLY) && phandler.is_has_relation_chain(self, pe, clit.chain_id) {
                    if let Some(deffect) = phandler.is_affected_by_effect(self, EFFECT_DISABLE_EFFECT) {
                        let ne = self.new_effect();
                        let dv = deffect.get_value(self, 0) as u32;
                        let downer = self.effect(deffect).owner;
                        let e = self.effect_mut(ne);
                        e.owner = downer;
                        e.type_ = EFFECT_TYPE_SINGLE as u16;
                        e.code = EFFECT_DISABLE_CHAIN;
                        e.value = clit.chain_id as i32;
                        e.reset_flag = RESET_CHAIN | RESET_EVENT | dv;
                        phandler.add_effect(self, ne);
                    }
                }
                self.core.new_chains.pop_front();
                false
            }
            3 => {
                let clit = self.core.current_chain.last().unwrap().clone();
                let playerid = clit.triggering_player as i32;
                let pe = clit.triggering_effect.unwrap();
                if !self.is_flag(DUEL_USE_TRAPS_IN_NEW_CHAIN) && self.get_cteffect(pe, playerid, true) != 0 {
                    let ph = self.infos.phase as u32;
                    let damage_step = ph == PHASE_DAMAGE && !self.effect(pe).is_flag(EFFECT_FLAG_DAMAGE_STEP);
                    let damage_cal = ph == PHASE_DAMAGE_CAL && !self.effect(pe).is_flag(EFFECT_FLAG_DAMAGE_CAL);
                    if damage_step || damage_cal {
                        self.returns.set_i32(0, 1);
                        return false;
                    }
                    let h = pe.get_handler(self);
                    self.emplace_process(units::SelectEffectYesNo { step: 0, playerid: playerid as u8, pcard: Some(h), description: 94 });
                } else {
                    self.returns.set_i32(0, 0);
                }
                false
            }
            4 => {
                if self.returns.at_i32(0) == 0 {
                    self.core.select_chains.clear();
                    self.core.select_options.clear();
                    arg.step = 5;
                    return false;
                }
                if self.core.select_chains.len() > 1 {
                    let p = self.core.current_chain.last().unwrap().triggering_player;
                    self.emplace_process(units::SelectOption { step: 0, playerid: p });
                } else {
                    self.returns.set_i32(0, 0);
                }
                false
            }
            5 => {
                let idx = self.returns.at_i32(0);
                let ch = self.core.select_chains[idx as usize].clone();
                let playerid = self.core.current_chain.last().unwrap().triggering_player;
                let pe = ch.triggering_effect.unwrap();
                let phandler = pe.get_handler(self);
                let opt = if idx as usize >= self.core.select_options.len() { 65 } else { 65 };
                let m = self.new_message(MSG_HINT);
                self.msg(m).u8(HINT_OPSELECTED).u8(playerid).u64(opt as u64);
                {
                    let clit = self.core.current_chain.last_mut().unwrap();
                    clit.triggering_effect = Some(pe);
                    clit.evt = ch.evt.clone();
                }
                let cid = self.core.current_chain.last().unwrap().chain_id;
                phandler.create_relation_chain(self, pe, cid);
                pe.dec_count(self, playerid as u32);
                if self.effect(pe).type_ as u32 & EFFECT_TYPE_ACTIVATE == 0 {
                    self.effect_mut(pe).type_ |= EFFECT_TYPE_ACTIVATE as u16;
                    self.core.current_chain.last_mut().unwrap().flag |= CHAIN_ACTIVATING;
                }
                self.core.select_chains.clear();
                self.core.select_options.clear();
                let de = self.new_effect();
                {
                    let e = self.effect_mut(de);
                    e.owner = Some(phandler);
                    e.code = 0;
                    e.type_ = EFFECT_TYPE_SINGLE as u16;
                    e.flag[0] = EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_CLIENT_HINT;
                    e.description = 65;
                    e.reset_flag = RESET_CHAIN;
                }
                phandler.add_effect(self, de);
                false
            }
            6 => {
                let clit = self.core.current_chain.last().unwrap().clone();
                let pe = clit.triggering_effect.unwrap();
                if self.effect(pe).cost.is_some() {
                    self.core.sub_solving_event.push_back(clit.evt.clone());
                    self.emplace_process(units::ExecuteCost { step: 0, triggering_player: clit.triggering_player, shuffle_check_was_disabled: false, triggering_effect: pe });
                }
                false
            }
            7 => {
                let clit = self.core.current_chain.last().unwrap().clone();
                let pe = clit.triggering_effect.unwrap();
                if self.effect(pe).target.is_some() {
                    self.core.sub_solving_event.push_back(clit.evt.clone());
                    self.emplace_process(units::ExecuteTarget { step: 0, triggering_player: clit.triggering_player, shuffle_check_was_disabled: false, triggering_effect: pe });
                }
                false
            }
            8 => {
                self.break_effect(false);
                let clit = self.core.current_chain.last().unwrap().clone();
                let pe = clit.triggering_effect.unwrap();
                let phandler = pe.get_handler(self);
                if let Some(tc) = clit.target_cards.as_ref() {
                    if !tc.is_empty() && self.effect(pe).is_flag(EFFECT_FLAG_CARD_TARGET) {
                        for c in public_order(self, tc) {
                            self.raise_single_event(c, None, EVENT_BECOME_TARGET, Some(pe), 0, clit.triggering_player, 0, clit.chain_count as u32);
                        }
                        self.process_single_event();
                        let tc2 = self.core.current_chain.last().unwrap().target_cards.clone().unwrap_or_default();
                        if !tc2.is_empty() {
                            self.raise_event(tc2, EVENT_BECOME_TARGET, Some(pe), 0, clit.triggering_player, clit.triggering_player, clit.chain_count as u32);
                        }
                    }
                }
                if self.effect(pe).type_ as u32 & EFFECT_TYPE_ACTIVATE != 0 {
                    self.core.leave_confirmed.insert(phandler);
                    if self.card(phandler).data.type_ & (TYPE_CONTINUOUS | TYPE_FIELD | TYPE_EQUIP | TYPE_PENDULUM | TYPE_LINK) == 0
                        && phandler.is_affected_by_effect(self, EFFECT_REMAIN_FIELD).is_none()
                    {
                        self.card_mut(phandler).set_status(STATUS_LEAVE_CONFIRMED, true);
                    }
                }
                let ht = phandler.get_type(self, None, 0, 2);
                if ht & (TYPE_SPELL | TYPE_TRAP) != 0
                    && ht & (TYPE_CONTINUOUS | TYPE_FIELD | TYPE_EQUIP | TYPE_PENDULUM | TYPE_LINK) != 0
                    && phandler.is_has_relation_chain(self, pe, clit.chain_id)
                    && self.card(phandler).current.location as u32 == LOCATION_SZONE
                    && !self.effect(pe).is_flag(EFFECT_FLAG_FIELD_ONLY)
                {
                    self.core.current_chain.last_mut().unwrap().flag |= CHAIN_CONTINUOUS_CARD;
                }
                self.core.phase_action = true;
                let clit = self.core.current_chain.last().unwrap().clone();
                if let Some((_, op)) = clit.opinfos.iter().find(|x| x.0 == 0x200) {
                    if op.op_count != 0 {
                        if self.is_flag(DUEL_CANNOT_SUMMON_OATH_OLD) {
                            self.core.spsummon_rst = true;
                            self.set_spsummon_counter(clit.triggering_player, true, true);
                            if op.op_player as u32 == PLAYER_ALL {
                                self.set_spsummon_counter(1 - clit.triggering_player, true, true);
                            }
                        }
                        if self.is_flag(DUEL_SPSUMMON_ONCE_OLD_NEGATE) && self.core.global_flag & GLOBALFLAG_SPSUMMON_ONCE != 0 {
                            if let Some(cards) = op.op_cards.clone() {
                                if op.op_player as u32 == PLAYER_ALL {
                                    let sumplayer = op.op_param as usize;
                                    let v = cards.to_vec();
                                    for (k, &c) in v.iter().take(2).enumerate() {
                                        let code = self.card(c).spsummon_code;
                                        if code != 0 {
                                            let p = if k == 0 { sumplayer } else { 1 - sumplayer };
                                            inc_map(&mut self.core.spsummon_once_map[p], code);
                                            inc_map(&mut self.core.spsummon_once_map_rst[p], code);
                                        }
                                    }
                                } else {
                                    let mut sumplayer = clit.triggering_player as usize;
                                    if self.effect(pe).is_flag(EFFECT_FLAG_CARD_TARGET) && op.op_player == 0x10 {
                                        sumplayer = 1 - sumplayer;
                                    }
                                    for c in public_order(self, &cards) {
                                        let code = self.card(c).spsummon_code;
                                        if code != 0 {
                                            inc_map(&mut self.core.spsummon_once_map[sumplayer], code);
                                            inc_map(&mut self.core.spsummon_once_map_rst[sumplayer], code);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                let m = self.new_message(MSG_CHAINED);
                self.msg(m).u8(clit.chain_count);
                self.raise_event_card(Some(phandler), EVENT_CHAINING, Some(pe), 0, clit.triggering_player, clit.triggering_player, clit.chain_count as u32);
                self.process_instant_event();
                self.core.just_sent_cards.clear();
                self.core.real_chain_count += 1;
                if !self.core.new_chains.is_empty() {
                    self.emplace_process(units::AddChain { step: 0, is_activated_effect: false });
                }
                self.adjust_all();
                true
            }
            10 => {
                arg.is_activated_effect = true;
                let clit = self.core.new_chains.front().unwrap().clone();
                let pe = clit.triggering_effect.unwrap();
                let phandler = pe.get_handler(self);
                let mut ecode = 0u32;
                let hc = self.card(phandler);
                let (loc, htype, ctrl) = (hc.current.location as u32, hc.data.type_, hc.current.controler);
                if loc == LOCATION_HAND {
                    if htype & TYPE_TRAP != 0 {
                        ecode = EFFECT_TRAP_ACT_IN_HAND;
                    } else if htype & TYPE_SPELL != 0
                        && (htype & TYPE_QUICKPLAY != 0 || phandler.is_affected_by_effect(self, EFFECT_BECOME_QUICK).is_some())
                        && self.infos.turn_player != ctrl
                    {
                        ecode = EFFECT_QP_ACT_IN_NTPHAND;
                    }
                } else if loc == LOCATION_SZONE {
                    if htype & TYPE_TRAP != 0 && self.card(phandler).get_status(STATUS_SET_TURN) != 0 {
                        ecode = EFFECT_TRAP_ACT_IN_SET_TURN;
                    }
                    if htype & TYPE_SPELL != 0
                        && (htype & TYPE_QUICKPLAY != 0 || phandler.is_affected_by_effect(self, EFFECT_BECOME_QUICK).is_some())
                        && self.card(phandler).get_status(STATUS_SET_TURN) != 0
                    {
                        ecode = EFFECT_QP_ACT_IN_SET_TURN;
                    }
                }
                if ecode != 0 {
                    self.core.select_effects.clear();
                    self.core.select_options.clear();
                    let mut eset = Vec::new();
                    phandler.filter_effect(self, ecode, &mut eset, true);
                    let mut has_no_side_effect_effs = false;
                    if !eset.is_empty() {
                        for peff in eset {
                            if self.effect(peff).has_function_value() || self.effect(peff).has_count_limit() {
                                if peff.check_count_limit(self, ctrl) {
                                    self.core.select_effects.push(Some(peff));
                                    let d = self.effect(peff).description;
                                    self.core.select_options.push(d);
                                }
                            } else {
                                has_no_side_effect_effs = true;
                            }
                        }
                        if self.core.select_options.is_empty() {
                            arg.step = 0;
                            return false;
                        }
                        if has_no_side_effect_effs {
                            self.core.select_options.insert(0, 99);
                            self.core.select_effects.insert(0, None);
                        }
                        if self.core.select_options.len() == 1 {
                            self.returns.set_i32(0, 0);
                        } else {
                            self.emplace_process(units::SelectOption { step: 0, playerid: ctrl });
                        }
                    }
                } else {
                    arg.step = 0;
                }
                false
            }
            11 => {
                let clit = self.core.new_chains.front().unwrap().clone();
                let pe = clit.triggering_effect.unwrap();
                let phandler = pe.get_handler(self);
                if !self.core.select_effects.is_empty() {
                    let idx = self.returns.at_i32(0) as usize;
                    if let Some(eff) = self.core.select_effects[idx] {
                        let ctrl = self.card(phandler).current.controler;
                        eff.dec_count(self, ctrl as u32);
                        self.add_param(Value::Effect(pe));
                        eff.get_value_card(self, Some(phandler), 1);
                    }
                }
                arg.step = 0;
                false
            }
            _ => true,
        }
    }

    pub fn process_sort_chain(&mut self, arg: &mut units::SortChain) -> bool {
        let is_tp = arg.playerid == self.infos.turn_player;
        match arg.step {
            0 => {
                let chains = if is_tp { &self.core.tpchain } else { &self.core.ntpchain };
                let effs: Vec<EffectId> = chains.iter().map(|c| c.triggering_effect.unwrap()).collect();
                self.core.select_cards = effs.into_iter().map(|e| e.get_handler(self)).collect();
                self.emplace_process(units::SortCard { step: 0, playerid: arg.playerid, is_chain: true });
                false
            }
            1 => {
                if self.returns.at_i8(0) == -1 {
                    return true;
                }
                let chains = if is_tp { &mut self.core.tpchain } else { &mut self.core.ntpchain };
                let keys: Vec<i8> = (0..chains.len()).map(|i| self.returns.at_i8(i)).collect();
                let mut v: Vec<(i8, Chain)> = keys.into_iter().zip(chains.drain(..)).collect();
                v.sort_by_key(|x| x.0);
                let chains = if is_tp { &mut self.core.tpchain } else { &mut self.core.ntpchain };
                chains.extend(v.into_iter().map(|x| x.1));
                true
            }
            _ => true,
        }
    }

    pub fn solve_continuous(&mut self, playerid: u8, peffect: EffectId, e: &Event) {
        let ch = Chain {
            chain_id: 0,
            chain_count: 0,
            triggering_effect: Some(peffect),
            triggering_player: playerid,
            evt: e.clone(),
            target_cards: None,
            target_player: PLAYER_NONE as u8,
            target_param: 0,
            disable_player: PLAYER_NONE as u8,
            disable_reason: None,
            flag: 0,
            ..Default::default()
        };
        self.core.sub_solving_continuous.push_back(ch);
        self.emplace_process(units::SolveContinuous { step: 0, reason_player: 0, reason_effect: None });
    }

    fn is_delay_or_phase(&self, pe: EffectId) -> bool {
        let e = self.effect(pe);
        e.is_flag(EFFECT_FLAG_DELAY) || (e.code & 0xfffff000 == 0 && e.code & (EVENT_PHASE | EVENT_PHASE_START) != 0)
    }

    pub fn process_solve_continuous(&mut self, arg: &mut units::SolveContinuous) -> bool {
        match arg.step {
            0 => {
                while let Some(c) = self.core.sub_solving_continuous.pop_back() {
                    self.core.solving_continuous.push_front(c);
                }
                let clit = self.core.solving_continuous.front().unwrap().clone();
                let pe = clit.triggering_effect.unwrap();
                let tpl = clit.triggering_player;
                if !pe.check_count_limit(self, tpl) {
                    self.core.solving_continuous.pop_front();
                    return true;
                }
                self.core.continuous_chain.push_back(clit.clone());
                if self.is_delay_or_phase(pe) {
                    self.core.conti_solving = true;
                }
                arg.reason_effect = self.core.reason_effect;
                arg.reason_player = self.core.reason_player;
                if self.effect(pe).target.is_none() {
                    return false;
                }
                self.core.sub_solving_event.push_back(clit.evt.clone());
                self.emplace_process(units::ExecuteTarget { step: 0, triggering_player: tpl, shuffle_check_was_disabled: false, triggering_effect: pe });
                false
            }
            1 => false,
            2 => {
                let clit = self.core.solving_continuous.front().unwrap().clone();
                let pe = clit.triggering_effect.unwrap();
                let tpl = clit.triggering_player;
                if self.effect(pe).operation.is_none() {
                    return false;
                }
                pe.dec_count(self, tpl as u32);
                self.core.sub_solving_event.push_back(clit.evt.clone());
                self.emplace_process(units::ExecuteOperation { step: 0, triggering_player: tpl, shuffle_check_was_disabled: false, triggering_effect: pe });
                false
            }
            3 => {
                let pe = self.core.solving_continuous.front().unwrap().triggering_effect.unwrap();
                self.core.reason_effect = arg.reason_effect;
                self.core.reason_player = arg.reason_player;
                self.core.continuous_chain.pop_back();
                self.core.solving_continuous.pop_front();
                if self.is_delay_or_phase(pe) {
                    self.core.conti_solving = false;
                    self.adjust_all();
                    return false;
                }
                true
            }
            4 => {
                let tp = self.infos.turn_player;
                if self.core.conti_player == PLAYER_NONE as u8 {
                    self.core.conti_player = tp;
                }
                if self.core.conti_player == tp {
                    if let Some(c) = self.core.delayed_continuous_tp.pop_front() {
                        self.core.sub_solving_continuous.push_back(c);
                        self.emplace_process(units::SolveContinuous { step: 0, reason_player: 0, reason_effect: None });
                    } else {
                        self.core.conti_player = 1 - tp;
                    }
                }
                if self.core.conti_player == 1 - tp {
                    if let Some(c) = self.core.delayed_continuous_ntp.pop_front() {
                        self.core.sub_solving_continuous.push_back(c);
                        self.emplace_process(units::SolveContinuous { step: 0, reason_player: 0, reason_effect: None });
                    } else if let Some(c) = self.core.delayed_continuous_tp.pop_front() {
                        self.core.conti_player = tp;
                        self.core.sub_solving_continuous.push_back(c);
                        self.emplace_process(units::SolveContinuous { step: 0, reason_player: 0, reason_effect: None });
                    } else {
                        self.core.conti_player = PLAYER_NONE as u8;
                    }
                }
                true
            }
            _ => true,
        }
    }

    pub fn process_solve_chain(&mut self, arg: &mut units::SolveChain) -> bool {
        let step = arg.step;
        if self.core.current_chain.is_empty() && step == 0 {
            return true;
        }
        let tp = self.infos.turn_player;
        match step {
            0 => {
                if self.core.spsummon_rst {
                    if self.is_flag(DUEL_CANNOT_SUMMON_OATH_OLD) {
                        self.set_spsummon_counter(0, false, true);
                        self.set_spsummon_counter(1, false, true);
                        self.core.spsummon_rst = false;
                    }
                    if self.is_flag(DUEL_SPSUMMON_ONCE_OLD_NEGATE) {
                        for plr in 0..2 {
                            let rst = self.core.spsummon_once_map_rst[plr].clone();
                            for x in self.core.spsummon_once_map[plr].iter_mut() {
                                let r = rst.iter().find(|y| y.0 == x.0).map_or(0, |y| y.1);
                                x.1 = x.1.wrapping_sub(r);
                            }
                            for x in self.core.spsummon_once_map[plr].clone() {
                                match self.core.spsummon_once_map_rst[plr].iter_mut().find(|y| y.0 == x.0) {
                                    Some(y) => y.1 = 0,
                                    None => self.core.spsummon_once_map_rst[plr].push((x.0, 0)),
                                }
                            }
                        }
                    }
                }
                let cait = self.core.current_chain.last().unwrap().clone();
                let m = self.new_message(MSG_CHAIN_SOLVING);
                self.msg(m).u8(cait.chain_count);
                let h = cait.triggering_effect.unwrap().get_handler(self);
                self.add_to_disable_check_list(h);
                self.adjust_instant();
                self.raise_event_card(None, EVENT_CHAIN_ACTIVATING, cait.triggering_effect, 0, cait.triggering_player, cait.triggering_player, cait.chain_count as u32);
                self.process_instant_event();
                false
            }
            1 => {
                let cait = self.core.current_chain.last().unwrap().clone();
                let pe = cait.triggering_effect.unwrap();
                if cait.flag & CHAIN_DISABLE_ACTIVATE != 0 && self.is_chain_negatable(cait.chain_count) {
                    self.remove_oath_effect(Some(pe));
                    let e = self.effect(pe);
                    if e.is_flag(EFFECT_FLAG_COUNT_LIMIT) && e.count_flag as u32 & EFFECT_COUNT_CODE_OATH != 0 {
                        let (cc, cf, hi) = (e.count_code, e.count_flag, e.count_hopt_index);
                        self.dec_effect_code(cc, cf, hi, cait.triggering_player);
                    }
                    if let Some(counters) = self.core.current_chain.last_mut().unwrap().applied_chain_counters.take() {
                        self.restore_chain_counter(cait.triggering_player, &counters);
                    }
                    let cc = cait.chain_count as u32;
                    self.core.new_fchain.retain(|ch| !(ch.evt.event_code == EVENT_CHAINING && ch.evt.event_value == cc));
                    self.core.new_ochain.retain(|ch| !(ch.evt.event_code == EVENT_CHAINING && ch.evt.event_value == cc));
                    self.raise_event_card(None, EVENT_CHAIN_NEGATED, Some(pe), 0, cait.triggering_player, cait.triggering_player, cc);
                    self.process_instant_event();
                    arg.step = 9;
                    return false;
                }
                self.core.current_chain.last_mut().unwrap().applied_chain_counters = None;
                self.release_oath_relation(Some(pe));
                self.break_effect(true);
                self.core.chain_solving = true;
                self.raise_event_card(None, EVENT_CHAIN_SOLVING, Some(pe), 0, cait.triggering_player, cait.triggering_player, cait.chain_count as u32);
                self.process_instant_event();
                false
            }
            2 => {
                self.core.spsummon_state_count_tmp = self.core.spsummon_state_count;
                let cait = self.core.current_chain.last().unwrap().clone();
                let pe = cait.triggering_effect.unwrap();
                let pcard = pe.get_handler(self);
                let has_rel = pcard.is_has_relation_chain(self, pe, cait.chain_id);
                if cait.flag & CHAIN_CONTINUOUS_CARD != 0 && !has_rel {
                    arg.step = 3;
                    return false;
                }
                if self.effect(pe).type_ as u32 & EFFECT_TYPE_ACTIVATE != 0 && has_rel && cait.replace_op.is_none() {
                    pcard.enable_field_effect(self, true);
                    if self.is_flag(DUEL_1_FACEUP_FIELD) && self.card(pcard).data.type_ & TYPE_FIELD != 0 {
                        let opp = 1 - self.card(pcard).current.controler as usize;
                        if let Some(fs) = self.player[opp].list_szone[5] {
                            if self.card(fs).is_position(POS_FACEUP) != 0 {
                                fs.enable_field_effect(self, false);
                            }
                        }
                    }
                    self.adjust_instant();
                }
                if self.is_chain_disablable(cait.chain_count) && (!self.effect(pe).is_flag(EFFECT_FLAG_CONTINUOUS_TARGET) || cait.replace_op.is_some()) {
                    let disabled = self.is_chain_disabled(cait.chain_count)
                        || (self.card(pcard).get_status(STATUS_DISABLED | STATUS_FORBIDDEN) != 0 && pcard.is_has_relation_chain(self, pe, cait.chain_id));
                    if disabled {
                        let flag = self.core.current_chain.last().unwrap().flag;
                        if flag & CHAIN_DISABLE_EFFECT == 0 {
                            let m = self.new_message(MSG_CHAIN_DISABLED);
                            self.msg(m).u8(cait.chain_count);
                        }
                        self.raise_event_card(None, EVENT_CHAIN_DISABLED, Some(pe), 0, cait.triggering_player, cait.triggering_player, cait.chain_count as u32);
                        self.process_instant_event();
                        arg.step = 3;
                        return false;
                    }
                }
                false
            }
            3 => {
                let cait = self.core.current_chain.last().unwrap().clone();
                let pe = cait.triggering_effect.unwrap();
                let pcard = pe.get_handler(self);
                if cait.flag & CHAIN_CONTINUOUS_CARD != 0 && !pcard.is_has_relation_chain(self, pe, cait.chain_id) {
                    return false;
                }
                if let Some(rop) = cait.replace_op.clone() {
                    // (no original operation: nothing to restore, as the C++'s 0)
                    arg.backed_up_operation = self.effect(pe).operation.clone();
                    self.effect_mut(pe).operation = Some(rop);
                } else {
                    arg.backed_up_operation = None;
                }
                if self.effect(pe).operation.is_some() {
                    self.core.sub_solving_event.push_back(cait.evt.clone());
                    self.emplace_process(units::ExecuteOperation { step: 0, triggering_player: cait.triggering_player, shuffle_check_was_disabled: false, triggering_effect: pe });
                }
                false
            }
            4 => {
                let pe = self.core.current_chain.last().unwrap().triggering_effect.unwrap();
                if let Some(op) = arg.backed_up_operation.clone() {
                    self.effect_mut(pe).operation = Some(op);
                }
                self.core.special_summoning.clear();
                self.core.equiping_cards.clear();
                false
            }
            5 => {
                let cait = self.core.current_chain.last().unwrap().clone();
                let had_backup = arg.backed_up_operation.take().is_some();
                if !had_backup {
                    if let Some((_, op)) = cait.opinfos.iter().find(|x| x.0 == 0x200) {
                        if op.op_count != 0 && self.is_flag(DUEL_CANNOT_SUMMON_OATH_OLD) {
                            let tpl = cait.triggering_player as usize;
                            if self.core.spsummon_state_count_tmp[tpl] == self.core.spsummon_state_count[tpl] {
                                self.set_spsummon_counter(tpl as u8, true, false);
                            }
                            if op.op_player as u32 == PLAYER_ALL && self.core.spsummon_state_count_tmp[1 - tpl] == self.core.spsummon_state_count[1 - tpl] {
                                self.set_spsummon_counter(1 - tpl as u8, true, false);
                            }
                            if let Some(cards) = op.op_cards.clone() {
                                if op.op_player as u32 == PLAYER_ALL {
                                    let sumplayer = op.op_param as usize;
                                    let v = cards.to_vec();
                                    if self.is_flag(DUEL_SPSUMMON_ONCE_OLD_NEGATE) && self.core.global_flag & GLOBALFLAG_SPSUMMON_ONCE != 0 {
                                        for (k, &c) in v.iter().take(2).enumerate() {
                                            let code = self.card(c).spsummon_code;
                                            if code != 0 {
                                                let p = if k == 0 { sumplayer } else { 1 - sumplayer };
                                                inc_map(&mut self.core.spsummon_once_map[p], code);
                                            }
                                        }
                                    }
                                    if let Some(&c0) = v.first() {
                                        self.check_card_counter(c0, ACTIVITY_SPSUMMON, sumplayer as i32);
                                    }
                                    if let Some(&c1) = v.get(1) {
                                        self.check_card_counter(c1, ACTIVITY_SPSUMMON, 1 - sumplayer as i32);
                                    }
                                } else {
                                    let mut sumplayer = tpl;
                                    if self.effect(cait.triggering_effect.unwrap()).is_flag(EFFECT_FLAG_CARD_TARGET) && op.op_player == 0x10 {
                                        sumplayer = 1 - sumplayer;
                                    }
                                    for c in public_order(self, &cards) {
                                        if self.is_flag(DUEL_SPSUMMON_ONCE_OLD_NEGATE) && self.core.global_flag & GLOBALFLAG_SPSUMMON_ONCE != 0 {
                                            let code = self.card(c).spsummon_code;
                                            if code != 0 {
                                                inc_map(&mut self.core.spsummon_once_map[sumplayer], code);
                                            }
                                        }
                                        self.check_card_counter(c, ACTIVITY_SPSUMMON, sumplayer as i32);
                                    }
                                }
                            }
                        }
                    }
                }
                self.core.spsummon_state_count_tmp = [0, 0];
                self.core.chain_solving = false;
                if let Some(c) = self.core.delayed_continuous_tp.pop_front() {
                    self.core.conti_player = tp;
                    self.core.sub_solving_continuous.push_back(c);
                    self.emplace_process(units::SolveContinuous { step: 0, reason_player: 0, reason_effect: None });
                } else if let Some(c) = self.core.delayed_continuous_ntp.pop_front() {
                    self.core.conti_player = 1 - tp;
                    self.core.sub_solving_continuous.push_back(c);
                    self.emplace_process(units::SolveContinuous { step: 0, reason_player: 0, reason_effect: None });
                } else {
                    self.core.conti_player = PLAYER_NONE as u8;
                }
                self.raise_event_card(None, EVENT_CHAIN_SOLVED, cait.triggering_effect, 0, cait.triggering_player, cait.triggering_player, cait.chain_count as u32);
                self.adjust_disable_check_list();
                self.process_instant_event();
                arg.step = 9;
                false
            }
            10 => {
                let cait = self.core.current_chain.last().unwrap().clone();
                let m = self.new_message(MSG_CHAIN_SOLVED);
                self.msg(m).u8(cait.chain_count);
                let pe = cait.triggering_effect.unwrap();
                let pcard = pe.get_handler(self);
                if self.effect(pe).type_ as u32 & EFFECT_TYPE_ACTIVATE != 0 && cait.flag & CHAIN_ACTIVATING != 0 {
                    self.effect_mut(pe).type_ &= !(EFFECT_TYPE_ACTIVATE as u16);
                }
                if cait.flag & CHAIN_HAND_EFFECT != 0 && self.card(pcard).is_position(POS_FACEUP) == 0 && self.card(pcard).current.location as u32 == LOCATION_HAND {
                    let ctrl = self.card(pcard).current.controler;
                    self.shuffle(ctrl, LOCATION_HAND as u8);
                }
                if let Some(tc) = cait.target_cards.as_ref() {
                    for t in public_order(self, tc) {
                        t.release_relation_chain(self, pe, cait.chain_id);
                    }
                }
                let ptype = self.effect(pe).type_ as u32;
                if self.card(pcard).data.type_ & TYPE_EQUIP != 0
                    && ptype & EFFECT_TYPE_ACTIVATE != 0
                    && self.card(pcard).equiping_target.is_none()
                    && pcard.is_has_relation_chain(self, pe, cait.chain_id)
                {
                    self.card_mut(pcard).set_status(STATUS_LEAVE_CONFIRMED, true);
                }
                if self.is_flag(DUEL_1_FACEUP_FIELD)
                    && self.card(pcard).data.type_ & TYPE_FIELD != 0
                    && ptype & EFFECT_TYPE_ACTIVATE != 0
                    && !self.card(pcard).is_status(STATUS_LEAVE_CONFIRMED)
                    && pcard.is_has_relation_chain(self, pe, cait.chain_id)
                {
                    let opp = 1 - self.card(pcard).current.controler;
                    if let Some(fs) = self.player[opp as usize].list_szone[5] {
                        if self.card(fs).is_position(POS_FACEUP) != 0 {
                            self.destroy(CardSet::from_card(fs), None, REASON_RULE, opp, PLAYER_NONE as u8, 0, 0);
                        }
                    }
                }
                self.effect_mut(pe).active_type = 0;
                self.effect_mut(pe).active_handler = None;
                pcard.release_relation_chain(self, pe, cait.chain_id);
                for c in public_order(self, &self.core.delayed_enable_set.clone()) {
                    if self.card(c).current.location as u32 == LOCATION_MZONE {
                        c.enable_field_effect(self, true);
                    }
                }
                self.core.delayed_enable_set.clear();
                self.adjust_all();
                self.core.current_chain.pop();
                self.core.real_chain_count -= 1;
                if self.core.real_chain_count < 0 {
                    self.core.real_chain_count = 0;
                }
                if self.core.current_chain.is_empty() {
                    self.core.chain_limit.clear();
                    return false;
                }
                if (self.core.summoning_card.is_some() || self.core.summoning_proc_group_type != 0) && self.core.reserved.is_some() {
                    let r = self.core.reserved.take().unwrap();
                    self.core.subunits.push_back(r);
                }
                self.core.summoning_card = None;
                self.core.summoning_proc_group_type = 0;
                arg.step = units::RESTART;
                false
            }
            11 => {
                for c in public_order(self, &self.core.leave_confirmed.clone()) {
                    if !self.card(c).is_status(STATUS_LEAVE_CONFIRMED) {
                        self.core.leave_confirmed.remove(c);
                    }
                }
                if !self.core.leave_confirmed.is_empty() {
                    let lc = self.core.leave_confirmed.clone();
                    self.send_to(lc, None, REASON_RULE, PLAYER_NONE as u8, PLAYER_NONE as u8, LOCATION_GRAVE as u16, 0, POS_FACEUP as u8, false);
                }
                false
            }
            12 => {
                let pe = std::mem::take(&mut self.core.point_event);
                self.core.used_event.extend(pe);
                self.new_message(MSG_CHAIN_END);
                self.reset_chain();
                if (self.core.summoning_card.is_some() || self.core.summoning_proc_group_type != 0 || self.core.effect_damage_step == 1) && self.core.reserved.is_some() {
                    let r = self.core.reserved.take().unwrap();
                    self.core.subunits.push_back(r);
                }
                self.core.summoning_proc_group_type = 0;
                self.core.summoning_card = None;
                false
            }
            13 => {
                self.core.just_sent_cards.clear();
                self.raise_event_card(None, EVENT_CHAIN_END, None, 0, 0, 0, 0);
                self.process_instant_event();
                self.adjust_all();
                if !arg.skip_trigger || !arg.skip_new {
                    self.core.hint_timing[0] |= TIMING_CHAIN_END;
                    self.core.hint_timing[1] |= TIMING_CHAIN_END;
                    self.emplace_process(units::PointEvent { step: 0, skip_trigger: arg.skip_trigger, skip_freechain: arg.skip_freechain || arg.skip_new, skip_new: arg.skip_new });
                }
                self.returns.set_i32(0, 1);
                true
            }
            _ => true,
        }
    }

    pub fn break_effect(&mut self, clear_sent: bool) -> i32 {
        if clear_sent {
            self.core.just_sent_cards.clear();
        }
        self.core.hint_timing[0] &= TIMING_DAMAGE_STEP | TIMING_DAMAGE_CAL;
        self.core.hint_timing[1] &= TIMING_DAMAGE_STEP | TIMING_DAMAGE_CAL;
        let mut i = 0;
        while i < self.core.new_ochain.len() {
            let ch = self.core.new_ochain[i].clone();
            let pe = ch.triggering_effect.unwrap();
            if !self.effect(pe).is_flag(EFFECT_FLAG_DELAY) {
                if self.effect(pe).is_flag(EFFECT_FLAG_FIELD_ONLY) || self.effect(pe).type_ as u32 & EFFECT_TYPE_FIELD == 0 || pe.in_range_chain(self, &ch) {
                    let h = pe.get_handler(self);
                    let loc = h.get_info_location(self);
                    let code = self.card(h).data.code;
                    let m = self.new_message(MSG_MISSED_EFFECT);
                    self.msg(m).loc(loc).u32(code);
                }
                self.core.new_ochain.remove(i);
                continue;
            }
            i += 1;
        }
        let ie = std::mem::take(&mut self.core.instant_event);
        self.core.used_event.extend(ie);
        self.adjust_instant();
        if !self.is_flag(DUEL_RELAY) && !self.core.force_turn_end {
            let (winp, rea) = self.win_check_impl();
            self.emit_win_impl(winp, rea);
        }
        0
    }

    /// The win check of `break_effect` and `Adjust` (deck = 0 or LP = 0).
    pub(crate) fn win_check_impl(&mut self) -> (u32, u32) {
        let mut winp: u32 = 5;
        let mut rea: u32 = 1;
        let cl = |d: &mut Duel, p: u8, code| d.is_player_affected_by_effect(p, code).is_some();
        let (lp0, lp1) = (self.player[0].lp, self.player[1].lp);
        let (od0, od1) = (self.core.overdraw[0], self.core.overdraw[1]);
        if lp0 <= 0 && lp1 > 0 && !cl(self, 0, EFFECT_CANNOT_LOSE_LP) {
            winp = 1;
            rea = 1;
        }
        if od0 && !od1 && !cl(self, 0, EFFECT_CANNOT_LOSE_DECK) {
            winp = 1;
            rea = 2;
        }
        if lp1 <= 0 && lp0 > 0 && !cl(self, 1, EFFECT_CANNOT_LOSE_LP) {
            winp = 0;
            rea = 1;
        }
        if od1 && !od0 && !cl(self, 1, EFFECT_CANNOT_LOSE_DECK) {
            winp = 0;
            rea = 2;
        }
        if lp1 <= 0 && lp0 <= 0 && !(cl(self, 0, EFFECT_CANNOT_LOSE_LP) && cl(self, 1, EFFECT_CANNOT_LOSE_LP)) {
            winp = if cl(self, 0, EFFECT_CANNOT_LOSE_LP) {
                0
            } else if cl(self, 1, EFFECT_CANNOT_LOSE_LP) {
                1
            } else {
                PLAYER_NONE
            };
            rea = 1;
        }
        if od1 && od0 && !(cl(self, 0, EFFECT_CANNOT_LOSE_DECK) && cl(self, 1, EFFECT_CANNOT_LOSE_DECK)) {
            winp = if cl(self, 0, EFFECT_CANNOT_LOSE_DECK) {
                0
            } else if cl(self, 1, EFFECT_CANNOT_LOSE_DECK) {
                1
            } else {
                PLAYER_NONE
            };
            rea = 2;
        }
        (winp, rea)
    }

    pub(crate) fn emit_win_impl(&mut self, winp: u32, rea: u32) {
        if winp != 5 {
            let m = self.new_message(MSG_WIN);
            self.msg(m).u8(winp).u8(rea);
            self.core.overdraw = [false, false];
            self.core.win_player = 5;
            self.core.win_reason = 0;
        } else if self.core.win_player != 5 {
            let (wp, wr) = (self.core.win_player, self.core.win_reason);
            let m = self.new_message(MSG_WIN);
            self.msg(m).u8(wp).u8(wr);
            self.core.win_player = 5;
            self.core.win_reason = 0;
            self.core.overdraw = [false, false];
        }
    }

    /// The conditions of continuous effects, checked at fixed points of the game in id order.
    pub fn refresh_effect_conditions(&mut self) {
        if self.querying || self.core.refreshing_condition.is_some() {
            return;
        }
        let mut conditional: Vec<EffectId> = Vec::new();
        let add = |d: &Duel, c: &crate::duel::EffectContainer, out: &mut Vec<EffectId>| {
            for &(_, e) in &c.items {
                if d.effect(e).condition.is_some() {
                    out.push(e);
                }
            }
        };
        add(self, &self.field_effects.aura_effect, &mut conditional);
        for c in &self.cards {
            add(self, &c.single_effect, &mut conditional);
            add(self, &c.field_effect, &mut conditional);
            if c.equiping_target.is_some() {
                add(self, &c.equip_effect, &mut conditional);
            }
            if !c.effect_target_cards.is_empty() {
                add(self, &c.target_effect, &mut conditional);
            }
            if c.overlay_target.is_some() {
                add(self, &c.xmaterial_effect, &mut conditional);
            }
        }
        crate::effect::sort_effects_by_id(self, &mut conditional);
        conditional.dedup();
        for pe in conditional {
            self.core.refreshing_condition = Some(pe);
            pe.is_available(self);
        }
        self.core.refreshing_condition = None;
    }

    /// The effects offered as options in public order: by description.
    pub fn sort_select_options(&mut self) {
        let mut from = 0;
        while from < self.core.select_effects.len() && self.core.select_effects[from].is_none() {
            from += 1;
        }
        if self.core.select_options.len() <= from + 1 {
            return;
        }
        let mut opts: Vec<(u64, Option<EffectId>)> = (from..self.core.select_options.len()).map(|i| (self.core.select_options[i], self.core.select_effects.get(i).copied().flatten())).collect();
        let d = &*self;
        crate::card::sort_by_less(&mut opts, |a, b| {
            if a.0 != b.0 {
                return a.0 < b.0;
            }
            match (a.1, b.1) {
                (Some(x), Some(y)) => d.effect(x).id < d.effect(y).id,
                _ => false,
            }
        });
        for (k, i) in (from..self.core.select_options.len()).enumerate() {
            self.core.select_options[i] = opts[k].0;
            if i < self.core.select_effects.len() {
                self.core.select_effects[i] = opts[k].1;
            }
        }
    }

    pub fn adjust_instant(&mut self) {
        self.infos.event_id = self.infos.event_id.wrapping_add(1);
        self.refresh_effect_conditions();
        self.adjust_disable_check_list();
        self.adjust_self_destroy_set();
    }

    pub fn adjust_all(&mut self) {
        self.infos.event_id = self.infos.event_id.wrapping_add(1);
        self.core.readjust_map.clear();
        self.emplace_process(units::Adjust { step: 0 });
    }

    pub fn refresh_location_info_instant(&mut self) {
        let dis1 = self.player[0].disabled_location | (self.player[1].disabled_location << 16);
        self.player[0].disabled_location = 0;
        self.player[1].disabled_location = 0;
        let mut eset = Vec::new();
        self.filter_field_effect(EFFECT_DISABLE_FIELD, &mut eset, true);
        for pe in eset {
            let value = pe.get_value(self, 0) as u32;
            self.player[0].disabled_location |= value & 0xff7f;
            self.player[1].disabled_location |= (value >> 16) & 0xff7f;
        }
        let mut eset = Vec::new();
        self.filter_field_effect(EFFECT_USE_EXTRA_MZONE, &mut eset, true);
        for pe in eset {
            let p = pe.get_handler_player(self) as usize;
            let value = pe.get_value(self, 0) as u32;
            self.player[p].disabled_location |= (value >> 16) & 0x1f;
        }
        let mut eset = Vec::new();
        self.filter_field_effect(EFFECT_USE_EXTRA_SZONE, &mut eset, true);
        for pe in eset {
            let p = pe.get_handler_player(self) as usize;
            let value = pe.get_value(self, 0) as u32;
            self.player[p].disabled_location |= (value >> 8) & 0x1f00;
        }
        let d1 = self.player[1].disabled_location;
        self.player[0].disabled_location |= (((d1 >> 5) & 1) << 6) | (((d1 >> 6) & 1) << 5);
        let d0 = self.player[0].disabled_location;
        self.player[1].disabled_location |= (((d0 >> 5) & 1) << 6) | (((d0 >> 6) & 1) << 5);
        let dis2 = self.player[0].disabled_location | (self.player[1].disabled_location << 16);
        if dis1 != dis2 {
            let m = self.new_message(MSG_FIELD_DISABLED);
            self.msg(m).u32(dis2);
        }
    }
}

pub fn inc_map(m: &mut Vec<(u32, u32)>, key: u32) {
    match m.iter_mut().find(|x| x.0 == key) {
        Some(x) => x.1 = x.1.wrapping_add(1),
        None => m.push((key, 1)),
    }
}
