//! Operations (`operations.cpp` part 5: setting monsters and spells/traps).

use crate::card::public_order;
use crate::constants::*;
use crate::duel::Duel;
use crate::ops4::extra_summon_values;
use crate::types::*;
use crate::units::{self, LocationReason};

impl Duel {
    pub fn process_monster_set(&mut self, arg: &mut units::MonsterSet) -> bool {
        let (setplayer, target, ignore_count, min_tribute, zone) = (arg.setplayer, arg.target, arg.ignore_count, arg.min_tribute, arg.zone);
        let mut proc = arg.summon_procedure_effect;
        let sp = setplayer as usize;
        let swap = |d: &Duel, proc: Option<EffectId>, z: u32| -> u32 {
            match proc {
                Some(pe) if d.effect(pe).is_flag(EFFECT_FLAG_SPSUM_PARAM) && d.effect(pe).o_range != 0 => (z >> 16) | (z & (0xffffu32 << 16)),
                _ => z,
            }
        };
        match arg.step {
            0 => {
                if target.is_affected_by_effect(self, EFFECT_UNSUMMONABLE_CARD).is_some() {
                    return true;
                }
                let c = self.card(target);
                if c.current.location as u32 != LOCATION_HAND || c.data.type_ & TYPE_MONSTER == 0 {
                    return true;
                }
                if target.is_affected_by_effect(self, EFFECT_CANNOT_MSET).is_some() {
                    return true;
                }
                let mut eset = Vec::new();
                let res = target.filter_set_procedure(self, setplayer, &mut eset, ignore_count, min_tribute, zone);
                if let Some(pe) = proc {
                    if res < 0 || !target.check_set_procedure(self, pe, setplayer, ignore_count, min_tribute, zone) {
                        return true;
                    }
                } else {
                    if res == -2 {
                        return true;
                    }
                    self.core.select_effects.clear();
                    self.core.select_options.clear();
                    if res > 0 {
                        self.core.select_effects.push(None);
                        self.core.select_options.push(1);
                    }
                    for pe in eset {
                        self.core.select_effects.push(Some(pe));
                        let d = self.effect(pe).description;
                        self.core.select_options.push(d);
                    }
                    self.sort_select_options();
                    if self.core.select_options.len() == 1 {
                        self.returns.set_i32(0, 0);
                    } else {
                        self.emplace_process(units::SelectOption { step: 0, playerid: setplayer });
                    }
                }
                self.card_mut(target).material_cards.clear();
                false
            }
            1 => {
                let mut eset = Vec::new();
                target.filter_effect(self, EFFECT_EXTRA_SET_COUNT, &mut eset, true);
                if proc.is_none() {
                    let idx = self.returns.at_i32(0) as usize;
                    proc = self.core.select_effects[idx];
                    arg.summon_procedure_effect = proc;
                }
                self.core.select_effects.clear();
                self.core.select_options.clear();
                if ignore_count || self.core.summon_count[sp] < self.get_summon_count_limit(setplayer) {
                    self.core.select_effects.push(None);
                    self.core.select_options.push(1);
                }
                if !ignore_count && self.core.extra_summon[sp] == 0 {
                    for peff in eset {
                        let mut retval = Vec::new();
                        peff.get_values_card(self, Some(target), 0, &mut retval);
                        let (mut new_min_tribute, new_zone, releasable) = extra_summon_values(&retval, 0x1f001f);
                        let new_zone = swap(self, proc, new_zone) & zone;
                        if let Some(pe) = proc {
                            if new_min_tribute < min_tribute {
                                new_min_tribute = min_tribute;
                            }
                            if !target.is_summonable(self, pe, new_min_tribute, new_zone, releasable, Some(peff)) {
                                continue;
                            }
                        } else {
                            let rcount = target.get_set_tribute_count(self);
                            let mut min = (rcount & 0xffff) as u16;
                            let mut max = ((rcount >> 16) & 0xffff) as u16;
                            if !self.is_player_can_mset(SUMMON_TYPE_ADVANCE, setplayer, target, setplayer) {
                                max = 0;
                            }
                            if min < min_tribute as u16 {
                                min = min_tribute as u16;
                            }
                            if max < min {
                                continue;
                            }
                            if min < new_min_tribute as u16 {
                                min = new_min_tribute as u16;
                            }
                            let ctrl = self.card(target).current.controler;
                            if !self.check_tribute(target, min as i32, max as i32, None, ctrl, new_zone, releasable, POS_FACEDOWN_DEFENSE) {
                                continue;
                            }
                        }
                        self.core.select_effects.push(Some(peff));
                        let d = self.effect(peff).description;
                        self.core.select_options.push(d);
                    }
                }
                self.sort_select_options();
                if self.core.select_options.len() == 1 {
                    self.returns.set_i32(0, 0);
                } else {
                    self.emplace_process(units::SelectOption { step: 0, playerid: setplayer });
                }
                false
            }
            2 => {
                let idx = self.returns.at_i32(0) as usize;
                let pextra = self.core.select_effects[idx];
                arg.extra_summon_effect = pextra;
                let mut releasable: u32 = 0xff00ff;
                if let Some(pe) = pextra {
                    let mut retval = Vec::new();
                    pe.get_values_card(self, Some(target), 0, &mut retval);
                    let (new_min_tribute, new_zone, rel) = extra_summon_values(&retval, 0x1f001f);
                    releasable = rel;
                    if min_tribute < new_min_tribute {
                        arg.min_tribute = new_min_tribute;
                    }
                    let nz = swap(self, proc, new_zone);
                    arg.zone &= nz;
                }
                if proc.is_some() {
                    arg.step = 3;
                    return false;
                }
                self.core.select_cards.clear();
                let required = target.get_set_tribute_count(self);
                let mut min = required & 0xffff;
                let max = required >> 16;
                if min < min_tribute as i32 {
                    min = min_tribute as i32;
                }
                let adv = self.is_player_can_mset(SUMMON_TYPE_ADVANCE, setplayer, target, setplayer);
                if max == 0 || !adv {
                    self.return_cards.clear();
                    arg.step = 3;
                } else {
                    let mut rc = CardSet::new();
                    let mut rce = CardSet::new();
                    let mut rco = CardSet::new();
                    let rcount = self.get_summon_release_list(target, Some(&mut rc), Some(&mut rce), Some(&mut rco), None, false, releasable, POS_FACEDOWN_DEFENSE);
                    self.core.release_cards = rc;
                    self.core.release_cards_ex = rce;
                    self.core.release_cards_ex_oneof = rco;
                    if rcount == 0 {
                        self.return_cards.clear();
                        arg.step = 3;
                    } else {
                        let ct = self.get_tofield_count(Some(target), setplayer, LOCATION_MZONE as u8, setplayer as u32, LocationReason::ToField, zone, None);
                        let fcount = self.get_mzone_limit(setplayer, setplayer, LocationReason::ToField);
                        if min == 0 && ct > 0 && fcount > 0 {
                            self.emplace_process(units::SelectYesNo { step: 0, playerid: setplayer, description: 90 });
                            arg.max_allowed_tributes = max as u8;
                        } else {
                            if min < -fcount + 1 {
                                min = -fcount + 1;
                            }
                            let cancelable = self.core.summon_cancelable != 0;
                            self.select_tribute_cards(Some(target), setplayer, cancelable, min as u16, max as u16, setplayer, zone);
                            arg.step = 3;
                        }
                    }
                }
                false
            }
            3 => {
                if self.returns.at_i32(0) != 0 {
                    self.return_cards.clear();
                } else {
                    let cancelable = self.core.summon_cancelable != 0;
                    let max = arg.max_allowed_tributes as u16;
                    self.select_tribute_cards(Some(target), setplayer, cancelable, 1, max, setplayer, zone);
                }
                false
            }
            4 => {
                if proc.is_some() {
                    arg.step = 5;
                } else {
                    if self.return_cards.canceled {
                        return true;
                    }
                    if !self.return_cards.list.is_empty() {
                        arg.tributes = CardSet::from_iter(self.return_cards.list.iter().copied());
                    }
                }
                let mut eset = Vec::new();
                target.filter_effect(self, EFFECT_MSET_COST, &mut eset, true);
                for pe in eset {
                    if self.effect(pe).operation.is_some() {
                        let ne = self.nil_event.clone();
                        self.core.sub_solving_event.push_back(ne);
                        self.emplace_process(units::ExecuteOperation { step: 0, triggering_player: setplayer, shuffle_check_was_disabled: false, triggering_effect: pe });
                    }
                }
                false
            }
            5 => {
                {
                    let c = self.card_mut(target);
                    c.summon.location = LOCATION_HAND as u8;
                    c.summon.pzone = false;
                    c.summon.type_ = SUMMON_TYPE_NORMAL;
                }
                if !arg.tributes.is_empty() {
                    let tributes = std::mem::take(&mut arg.tributes);
                    for c in public_order(self, &tributes) {
                        self.card_mut(c).current.reason_card = Some(target);
                    }
                    target.set_material(self, tributes.clone());
                    self.release(tributes, None, REASON_SUMMON | REASON_MATERIAL, setplayer);
                    self.card_mut(target).summon.type_ |= SUMMON_TYPE_ADVANCE;
                    self.adjust_all();
                }
                let c = self.card_mut(target);
                c.summon.player = setplayer;
                c.current.reason_effect = None;
                c.current.reason_player = setplayer;
                arg.step = 7;
                false
            }
            6 => {
                self.returns.set_i32(0, 1);
                let pe = proc.unwrap();
                if self.effect(pe).target.is_some() {
                    let pextra = arg.extra_summon_effect;
                    let releasable = self.extra_releasable(pextra, target);
                    self.add_param(Value::Card(target));
                    self.add_param(Value::Int(min_tribute as i64));
                    self.add_param(Value::Int(zone as i64));
                    self.add_param(Value::Int(releasable as i64));
                    self.add_param(Value::effect(pextra));
                    let ne = self.nil_event.clone();
                    self.core.sub_solving_event.push_back(ne);
                    self.emplace_process(units::ExecuteTarget { step: 0, triggering_player: setplayer, shuffle_check_was_disabled: false, triggering_effect: pe });
                }
                false
            }
            7 => {
                if self.returns.at_i32(0) == 0 {
                    return true;
                }
                let pe = proc.unwrap();
                let v = pe.get_value_card(self, Some(target), 0) as u32;
                {
                    let c = self.card_mut(target);
                    c.summon.type_ = (v & 0xfffffff) | SUMMON_TYPE_NORMAL;
                    c.summon.location = LOCATION_HAND as u8;
                    c.summon.pzone = false;
                    c.summon.player = setplayer;
                    c.current.reason_effect = Some(pe);
                    c.current.reason_player = setplayer;
                }
                if self.effect(pe).operation.is_some() {
                    let pextra = arg.extra_summon_effect;
                    let releasable = self.extra_releasable(pextra, target);
                    self.add_param(Value::Card(target));
                    self.add_param(Value::Int(min_tribute as i64));
                    self.add_param(Value::Int(zone as i64));
                    self.add_param(Value::Int(releasable as i64));
                    self.add_param(Value::effect(pextra));
                    let ne = self.nil_event.clone();
                    self.core.sub_solving_event.push_back(ne);
                    self.emplace_process(units::ExecuteOperation { step: 0, triggering_player: setplayer, shuffle_check_was_disabled: false, triggering_effect: pe });
                }
                pe.dec_count(self, setplayer as u32);
                false
            }
            8 => {
                self.break_effect(true);
                if ignore_count {
                    return false;
                }
                self.count_summon(arg.extra_summon_effect, setplayer, target);
                false
            }
            9 => {
                let mut targetplayer = setplayer;
                let mut positions = POS_FACEDOWN_DEFENSE as u8;
                if let Some(pe) = proc {
                    if self.effect(pe).is_flag(EFFECT_FLAG_SPSUM_PARAM) {
                        positions = (self.effect(pe).s_range as u8) & POS_FACEDOWN as u8;
                        if self.effect(pe).o_range != 0 {
                            targetplayer = 1 - setplayer;
                        }
                    }
                }
                target.enable_field_effect(self, false);
                self.move_to_field(target, setplayer, targetplayer, LOCATION_MZONE as u16, positions, false, 0, zone as u8, false, LocationReason::Set, true);
                false
            }
            10 => {
                let ctrl = self.card(target).current.controler;
                self.set_control(target, ctrl, 0, 0);
                self.core.phase_action = true;
                self.core.normalsummon_state_count[sp] = self.core.normalsummon_state_count[sp].wrapping_add(1);
                self.check_card_counter(target, ACTIVITY_NORMALSUMMON, setplayer as i32);
                self.card_mut(target).set_status(STATUS_SUMMON_TURN, true);
                let code = self.card(target).data.code;
                let loc = target.get_info_location(self);
                let m = self.new_message(MSG_SET);
                self.msg(m).u32(code).loc(loc);
                self.adjust_instant();
                self.raise_event_card(Some(target), EVENT_MSET, proc, 0, setplayer, setplayer, 0);
                self.process_instant_event();
                if self.core.current_chain.is_empty() {
                    self.adjust_all();
                    self.core.hint_timing[sp] |= TIMING_MSET;
                    self.emplace_process(units::PointEvent { step: 0, skip_trigger: false, skip_freechain: false, skip_new: false });
                }
                true
            }
            _ => true,
        }
    }

    fn monster_sset_type(&mut self, target: CardId) {
        if self.card(target).data.type_ & TYPE_MONSTER != 0 {
            let pe = target.is_affected_by_effect(self, EFFECT_MONSTER_SSET).unwrap();
            let type_val = pe.get_value(self, 0) as i32;
            let ne = self.new_effect();
            let e = self.effect_mut(ne);
            e.owner = Some(target);
            e.type_ = EFFECT_TYPE_SINGLE as u16;
            e.code = EFFECT_CHANGE_TYPE;
            e.reset_flag = RESET_EVENT + 0x1fe0000;
            e.value = type_val;
            target.add_effect(self, ne);
        }
    }

    fn can_sset(&mut self, target: CardId, setplayer: u8, toplayer: u8) -> bool {
        let dt = self.card(target).data.type_;
        if dt & TYPE_FIELD == 0 && self.get_useable_count(Some(target), toplayer, LOCATION_SZONE as u8, setplayer, LocationReason::ToField, 0xff, None) <= 0 {
            return false;
        }
        if dt & TYPE_MONSTER != 0 && target.is_affected_by_effect(self, EFFECT_MONSTER_SSET).is_none() {
            return false;
        }
        if self.card(target).current.location as u32 == LOCATION_SZONE {
            return false;
        }
        if !self.is_player_can_sset(setplayer, target) {
            return false;
        }
        target.is_affected_by_effect(self, EFFECT_CANNOT_SSET).is_none()
    }

    pub fn process_spell_set(&mut self, arg: &mut units::SpellSet) -> bool {
        let (setplayer, toplayer, target, reason_effect) = (arg.setplayer, arg.toplayer, arg.target, arg.reason_effect);
        match arg.step {
            0 => {
                if !self.can_sset(target, setplayer, toplayer) {
                    return true;
                }
                let mut eset = Vec::new();
                target.filter_effect(self, EFFECT_SSET_COST, &mut eset, true);
                for pe in eset {
                    if self.effect(pe).operation.is_some() {
                        let ne = self.nil_event.clone();
                        self.core.sub_solving_event.push_back(ne);
                        self.emplace_process(units::ExecuteOperation { step: 0, triggering_player: setplayer, shuffle_check_was_disabled: false, triggering_effect: pe });
                    }
                }
                false
            }
            1 => {
                target.enable_field_effect(self, false);
                let zone = if self.card(target).data.type_ & TYPE_FIELD != 0 { 1 << 5 } else { 0xff };
                self.move_to_field(target, setplayer, toplayer, LOCATION_SZONE as u16, POS_FACEDOWN as u8, false, 0, zone, false, LocationReason::None, true);
                false
            }
            2 => {
                self.core.phase_action = true;
                self.card_mut(target).set_status(STATUS_SET_TURN, true);
                self.monster_sset_type(target);
                let code = self.card(target).data.code;
                let loc = target.get_info_location(self);
                let m = self.new_message(MSG_SET);
                self.msg(m).u32(code).loc(loc);
                self.adjust_instant();
                self.raise_event_card(Some(target), EVENT_SSET, reason_effect, 0, setplayer, setplayer, 0);
                self.process_instant_event();
                if self.core.current_chain.is_empty() {
                    self.adjust_all();
                    self.core.hint_timing[setplayer as usize] |= TIMING_SSET;
                    self.emplace_process(units::PointEvent { step: 0, skip_trigger: false, skip_freechain: false, skip_new: false });
                }
                true
            }
            _ => true,
        }
    }

    pub fn process_spell_set_group(&mut self, arg: &mut units::SpellSetGroup) -> bool {
        let (setplayer, toplayer, confirm, reason_effect) = (arg.setplayer, arg.toplayer, arg.confirm, arg.reason_effect);
        match arg.step {
            0 => {
                self.core.operated_set.clear();
                let pt = self.group(arg.ptarget.unwrap()).clone();
                for t in public_order(self, &pt) {
                    if !self.can_sset(t, setplayer, toplayer) {
                        continue;
                    }
                    arg.set_cards.insert(t);
                }
                if arg.set_cards.is_empty() {
                    self.returns.set_i32(0, 0);
                    return true;
                }
                for c in public_order(self, &arg.set_cards.clone()) {
                    let mut eset = Vec::new();
                    c.filter_effect(self, EFFECT_SSET_COST, &mut eset, true);
                    for pe in eset {
                        if self.effect(pe).operation.is_some() {
                            let ne = self.nil_event.clone();
                            self.core.sub_solving_event.push_back(ne);
                            self.emplace_process(units::ExecuteOperation { step: 0, triggering_player: setplayer, shuffle_check_was_disabled: false, triggering_effect: pe });
                        }
                    }
                }
                self.core.set_group_pre_set.clear();
                self.core.set_group_set.clear();
                self.core.set_group_seq_of.clear();
                self.core.set_group_used_zones = 0;
                self.core.phase_action = true;
                false
            }
            1 => {
                let t = public_order(self, &arg.set_cards.clone())[0];
                if self.card(t).data.type_ & TYPE_FIELD != 0 {
                    self.returns.set_i8(2, 5);
                    return false;
                }
                let mut flag = 0u32;
                self.get_useable_count(Some(t), toplayer, LOCATION_SZONE as u8, setplayer, LocationReason::ToField, 0xff, Some(&mut flag));
                flag |= self.core.set_group_used_zones;
                if setplayer == toplayer {
                    flag = ((flag & 0xff) << 8) | 0xffff00ff;
                } else {
                    flag = ((flag & 0xff) << 24) | 0xffffff;
                }
                flag |= 0xe080e080;
                let code = self.card(t).data.code;
                let m = self.new_message(MSG_HINT);
                self.msg(m).u8(HINT_SELECTMSG).u8(setplayer).u64(code as u64);
                self.emplace_process(units::SelectPlace { step: 0, playerid: setplayer, count: 1, flag, disable_field: false });
                false
            }
            2 => {
                let t = public_order(self, &arg.set_cards.clone())[0];
                let seq = self.returns.at_i8(2) as u8;
                let n = self.core.set_group_pre_set.len();
                self.core.set_group_seq[n] = seq;
                match self.core.set_group_seq_of.iter_mut().find(|x| x.0 == t) {
                    Some(x) => x.1 = seq,
                    None => self.core.set_group_seq_of.push((t, seq)),
                }
                self.core.set_group_pre_set.insert(t);
                self.core.set_group_used_zones |= 1u32 << seq;
                arg.set_cards.remove(t);
                if !arg.set_cards.is_empty() {
                    arg.step = 0;
                }
                false
            }
            3 => {
                let t = public_order(self, &self.core.set_group_pre_set.clone())[0];
                self.core.set_group_cur = Some(t);
                t.enable_field_effect(self, false);
                let mut zone: u32 = 0;
                if self.card(t).data.type_ & TYPE_FIELD != 0 {
                    zone = 1 << 5;
                } else {
                    for i in 0..7 {
                        zone = 1 << i;
                        if self.core.set_group_used_zones & zone != 0 {
                            self.core.set_group_used_zones &= !zone;
                            break;
                        }
                    }
                }
                self.move_to_field(t, setplayer, toplayer, LOCATION_SZONE as u16, POS_FACEDOWN as u8, false, 0, zone as u8, false, LocationReason::None, false);
                false
            }
            4 => {
                let t = self.core.set_group_cur.unwrap();
                self.card_mut(t).set_status(STATUS_SET_TURN, true);
                self.monster_sset_type(t);
                let code = self.card(t).data.code;
                let loc = t.get_info_location(self);
                let m = self.new_message(MSG_SET);
                self.msg(m).u32(code).loc(loc);
                self.core.set_group_set.insert(t);
                self.core.set_group_pre_set.remove(t);
                if !self.core.set_group_pre_set.is_empty() {
                    arg.step = 2;
                }
                false
            }
            5 => {
                if confirm {
                    let set = self.core.set_group_set.clone();
                    let ordered = public_order(self, &set);
                    let m = self.new_message(MSG_CONFIRM_CARDS);
                    self.msg(m).u8(1 - setplayer).u32(set.len());
                    for c in ordered {
                        let cc = self.card(c);
                        let (code, ctrl, loc, seq) = (cc.data.code, cc.current.controler, cc.current.location, cc.current.sequence);
                        self.msg(m).u32(code).u8(ctrl).u8(loc).u32(seq);
                    }
                }
                false
            }
            6 => {
                self.core.operated_set.clear();
                for c in public_order(self, &self.core.set_group_set.clone()) {
                    self.core.operated_set.insert(c);
                }
                let mut ct = self.core.operated_set.len() as u8;
                if self.core.set_group_used_zones & (1 << 5) != 0 {
                    ct = ct.wrapping_sub(1);
                }
                if ct <= 1 {
                    return false;
                }
                let m = self.new_message(MSG_SHUFFLE_SET_CARD);
                self.msg(m).u8(LOCATION_SZONE).u8(ct);
                let mut ms = Vec::new();
                let mut seqs = Vec::new();
                for c in public_order(self, &self.core.operated_set.clone()) {
                    let seq = self.core.set_group_seq_of.iter().find(|x| x.0 == c).map_or(0, |x| x.1);
                    if self.card(c).data.type_ & TYPE_FIELD != 0 {
                        continue;
                    }
                    let loc = c.get_info_location(self);
                    self.msg(m).loc(loc);
                    ms.push(c);
                    seqs.push(seq);
                }
                for k in 0..ms.len() {
                    self.player[toplayer as usize].list_szone[seqs[k] as usize] = Some(ms[k]);
                    self.card_mut(ms[k]).current.sequence = seqs[k] as u32;
                }
                for _ in 0..ct {
                    self.msg(m).loc(LocInfo::default());
                }
                for &c in &ms {
                    let s = self.card(c).current.sequence as u8;
                    self.msg(m).u8(s);
                }
                false
            }
            7 => {
                self.adjust_instant();
                let os = self.core.operated_set.clone();
                self.raise_event(os, EVENT_SSET, reason_effect, 0, setplayer, setplayer, 0);
                self.process_instant_event();
                if self.core.current_chain.is_empty() {
                    self.adjust_all();
                    self.core.hint_timing[setplayer as usize] |= TIMING_SSET;
                    self.emplace_process(units::PointEvent { step: 0, skip_trigger: false, skip_freechain: false, skip_new: false });
                }
                false
            }
            8 => {
                self.returns.set_i32(0, self.core.operated_set.len() as i32);
                true
            }
            _ => true,
        }
    }
}
