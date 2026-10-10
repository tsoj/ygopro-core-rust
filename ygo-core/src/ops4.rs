//! Operations (`operations.cpp` part 4: normal summon and flip summon).

use crate::card::public_order;
use crate::constants::*;
use crate::duel::Duel;
use crate::types::*;
use crate::units::{self, LocationReason};

/// The (new_min_tribute, new_zone, releasable) of an EFFECT_EXTRA_SUMMON_COUNT value.
pub fn extra_summon_values(retval: &[i64], default_zone: u32) -> (u8, u32, u32) {
    let new_min = if !retval.is_empty() { retval[0] as u8 } else { 0 };
    let new_zone = if retval.len() > 1 { retval[1] as u32 } else { default_zone };
    let mut releasable: u32 = 0xff00ff;
    if retval.len() > 2 {
        if retval[2] < 0 {
            releasable = releasable.wrapping_add(retval[2] as i32 as u32);
        } else {
            releasable = retval[2] as u32;
        }
    }
    (new_min, new_zone, releasable)
}

impl Duel {
    pub(crate) fn extra_releasable(&mut self, pextra: Option<EffectId>, target: CardId) -> u32 {
        match pextra {
            Some(pe) => {
                let mut retval = Vec::new();
                pe.get_values_card(self, Some(target), 0, &mut retval);
                extra_summon_values(&retval, 0x1f001f).2
            }
            None => 0xff00ff,
        }
    }
    pub(crate) fn hint_card(&mut self, code: u32) {
        let m = self.new_message(MSG_HINT);
        self.msg(m).u8(HINT_CARD).u8(0u8).u64(code);
    }
    fn spsum_param_swap(&self, proc: Option<EffectId>, new_zone: u32) -> u32 {
        if let Some(pe) = proc {
            if self.effect(pe).is_flag(EFFECT_FLAG_SPSUM_PARAM) && self.effect(pe).o_range != 0 {
                return (new_zone >> 16) | (new_zone & (0xffffu32 << 16));
            }
        }
        new_zone
    }

    pub fn process_summon_rule(&mut self, arg: &mut units::SummonRule) -> bool {
        let (sumplayer, target, ignore_count, min_tribute, zone) = (arg.sumplayer, arg.target, arg.ignore_count, arg.min_tribute, arg.zone);
        let mut summon_procedure_effect = arg.summon_procedure_effect;
        let sp = sumplayer as usize;
        match arg.step {
            0 => {
                if !target.is_summonable_card(self) {
                    return true;
                }
                if self.check_unique_onfield(target, sumplayer, LOCATION_MZONE as u8, None).is_some() {
                    return true;
                }
                if target.is_affected_by_effect(self, EFFECT_CANNOT_SUMMON).is_some() {
                    return true;
                }
                if self.card(target).current.location as u32 == LOCATION_MZONE {
                    if self.card(target).is_position(POS_FACEDOWN) != 0 {
                        return true;
                    }
                    if !ignore_count
                        && (self.core.extra_summon[sp] != 0 || target.is_affected_by_effect(self, EFFECT_EXTRA_SUMMON_COUNT).is_none())
                        && self.core.summon_count[sp] >= self.get_summon_count_limit(sumplayer)
                    {
                        return true;
                    }
                    if target.is_affected_by_effect(self, EFFECT_GEMINI_SUMMONABLE).is_none() {
                        return true;
                    }
                    if target.is_affected_by_effect(self, EFFECT_GEMINI_STATUS).is_some() {
                        return true;
                    }
                    if !self.is_player_can_summon(SUMMON_TYPE_GEMINI, sumplayer, target, sumplayer) {
                        return true;
                    }
                } else {
                    let mut eset = Vec::new();
                    let res = target.filter_summon_procedure(self, sumplayer, &mut eset, ignore_count, min_tribute, zone);
                    if let Some(pe) = summon_procedure_effect {
                        if res < 0 || !target.check_summon_procedure(self, pe, sumplayer, ignore_count, min_tribute, zone) {
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
                        if self.core.select_options.is_empty() {
                            return true;
                        }
                        if self.core.select_options.len() == 1 {
                            self.returns.set_i32(0, 0);
                        } else {
                            self.emplace_process(units::SelectOption { step: 0, playerid: sumplayer });
                        }
                    }
                }
                if self.core.summon_depth != 0 {
                    self.core.summon_cancelable = 0;
                }
                self.core.summon_depth += 1;
                self.card_mut(target).material_cards.clear();
                false
            }
            1 => {
                let mut eset = Vec::new();
                target.filter_effect(self, EFFECT_EXTRA_SUMMON_COUNT, &mut eset, true);
                if self.card(target).current.location as u32 == LOCATION_MZONE {
                    arg.step = 4;
                    arg.extra_summon_effect = if !ignore_count && self.core.extra_summon[sp] == 0 { eset.first().copied() } else { None };
                    return false;
                }
                if summon_procedure_effect.is_none() {
                    let idx = self.returns.at_i32(0) as usize;
                    summon_procedure_effect = self.core.select_effects[idx];
                    arg.summon_procedure_effect = summon_procedure_effect;
                }
                self.core.select_effects.clear();
                self.core.select_options.clear();
                if ignore_count || self.core.summon_count[sp] < self.get_summon_count_limit(sumplayer) {
                    self.core.select_effects.push(None);
                    self.core.select_options.push(1);
                }
                if !ignore_count && self.core.extra_summon[sp] == 0 {
                    for peff in eset {
                        let mut retval = Vec::new();
                        peff.get_values_card(self, Some(target), 0, &mut retval);
                        let (mut new_min_tribute, new_zone, releasable) = extra_summon_values(&retval, 0x1f001f);
                        let new_zone = self.spsum_param_swap(summon_procedure_effect, new_zone) & zone;
                        if let Some(proc) = summon_procedure_effect {
                            if new_min_tribute < min_tribute {
                                new_min_tribute = min_tribute;
                            }
                            if !target.is_summonable(self, proc, new_min_tribute, new_zone, releasable, Some(peff)) {
                                continue;
                            }
                        } else {
                            let rcount = target.get_summon_tribute_count(self);
                            let mut min = rcount & 0xffff;
                            let mut max = (rcount >> 16) & 0xffff;
                            if !self.is_player_can_summon(SUMMON_TYPE_ADVANCE, sumplayer, target, sumplayer) {
                                max = 0;
                            }
                            if min < min_tribute as i32 {
                                min = min_tribute as i32;
                            }
                            if max < min {
                                continue;
                            }
                            if min < new_min_tribute as i32 {
                                min = new_min_tribute as i32;
                            }
                            let ctrl = self.card(target).current.controler;
                            if !self.check_tribute(target, min, max, None, ctrl, new_zone, releasable, 0x1) {
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
                    self.emplace_process(units::SelectOption { step: 0, playerid: sumplayer });
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
                    let new_zone = self.spsum_param_swap(summon_procedure_effect, new_zone);
                    arg.zone &= new_zone;
                }
                if summon_procedure_effect.is_some() {
                    arg.step = 3;
                    return false;
                }
                self.core.select_cards.clear();
                let required = target.get_summon_tribute_count(self);
                let mut min = required & 0xffff;
                let max = required >> 16;
                if min < min_tribute as i32 {
                    min = min_tribute as i32;
                }
                let adv = self.is_player_can_summon(SUMMON_TYPE_ADVANCE, sumplayer, target, sumplayer);
                if max == 0 || !adv {
                    self.return_cards.clear();
                    arg.step = 4;
                } else {
                    let mut rc = CardSet::new();
                    let mut rce = CardSet::new();
                    let mut rco = CardSet::new();
                    let rcount = self.get_summon_release_list(target, Some(&mut rc), Some(&mut rce), Some(&mut rco), None, false, releasable, 0x1);
                    self.core.release_cards = rc;
                    self.core.release_cards_ex = rce;
                    self.core.release_cards_ex_oneof = rco;
                    if rcount == 0 {
                        self.return_cards.clear();
                        arg.step = 4;
                    } else {
                        let ct = self.get_tofield_count(Some(target), sumplayer, LOCATION_MZONE as u8, sumplayer as u32, LocationReason::ToField, zone, None);
                        let fcount = self.get_mzone_limit(sumplayer, sumplayer, LocationReason::ToField);
                        if min == 0 && ct > 0 && fcount > 0 {
                            self.emplace_process(units::SelectYesNo { step: 0, playerid: sumplayer, description: 90 });
                            arg.max_allowed_tributes = max as u8;
                        } else {
                            if min < -fcount + 1 {
                                min = -fcount + 1;
                            }
                            let cancelable = self.core.summon_cancelable != 0;
                            self.select_tribute_cards(Some(target), sumplayer, cancelable, min as u16, max as u16, sumplayer, zone);
                            arg.step = 4;
                        }
                    }
                }
                false
            }
            3 => {
                if self.returns.at_i32(0) != 0 {
                    self.return_cards.clear();
                } else {
                    let max = arg.max_allowed_tributes;
                    let cancelable = self.core.summon_cancelable != 0;
                    self.select_tribute_cards(Some(target), sumplayer, cancelable, 1, max as u16, sumplayer, zone);
                }
                arg.step = 4;
                false
            }
            4 => {
                self.returns.set_i32(0, 1);
                let proc = summon_procedure_effect.unwrap();
                if self.effect(proc).target.is_some() {
                    let pextra = arg.extra_summon_effect;
                    let releasable = self.extra_releasable(pextra, target);
                    self.add_param(Value::Card(target));
                    self.add_param(Value::Int(min_tribute as i64));
                    self.add_param(Value::Int(zone as i64));
                    self.add_param(Value::Int(releasable as i64));
                    self.add_param(Value::effect(pextra));
                    let ne = self.nil_event.clone();
                    self.core.sub_solving_event.push_back(ne);
                    self.emplace_process(units::ExecuteTarget { step: 0, triggering_player: sumplayer, shuffle_check_was_disabled: false, triggering_effect: proc });
                }
                false
            }
            5 => {
                if self.card(target).current.location as u32 == LOCATION_MZONE {
                    arg.step = 9;
                } else if summon_procedure_effect.is_some() {
                    if self.returns.at_i32(0) == 0 {
                        self.core.summon_depth -= 1;
                        return true;
                    }
                    arg.step = 6;
                } else {
                    if self.return_cards.canceled {
                        self.core.summon_depth -= 1;
                        return true;
                    }
                    arg.tributes = CardSet::from_iter(self.return_cards.list.iter().copied());
                }
                arg.summon_cost_effects.clear();
                let mut costs = Vec::new();
                target.filter_effect(self, EFFECT_SUMMON_COST, &mut costs, true);
                arg.summon_cost_effects = costs.clone();
                for pe in costs {
                    if self.effect(pe).operation.is_some() {
                        let ne = self.nil_event.clone();
                        self.core.sub_solving_event.push_back(ne);
                        self.emplace_process(units::ExecuteOperation { step: 0, triggering_player: sumplayer, shuffle_check_was_disabled: false, triggering_effect: pe });
                    }
                }
                false
            }
            6 => {
                let level = target.get_level(self) as i32;
                let mut min = if level < 5 {
                    0
                } else if level < 7 {
                    1
                } else {
                    2
                };
                min -= arg.tributes.len() as i32;
                if min > 0 {
                    let mut eset = Vec::new();
                    target.filter_effect(self, EFFECT_DECREASE_TRIBUTE, &mut eset, true);
                    let mut minul = 0;
                    let mut pdec = None;
                    for &peff in &eset {
                        if !self.effect(peff).is_flag(EFFECT_FLAG_COUNT_LIMIT) {
                            let dec = peff.get_value_card(self, Some(target), 0) as i32;
                            if minul < (dec & 0xffff) {
                                minul = dec & 0xffff;
                                pdec = Some(peff);
                            }
                        }
                    }
                    if let Some(pd) = pdec {
                        min -= minul;
                        let code = self.card(self.effect(pd).handler.unwrap()).data.code;
                        self.hint_card(code);
                    }
                    for pass in 0..2 {
                        for &pe in &eset {
                            let e = self.effect(pe);
                            let has_target = e.target.is_some();
                            if e.is_flag(EFFECT_FLAG_COUNT_LIMIT) && e.count_limit > 0 && has_target == (pass == 0) {
                                let dec = pe.get_value_card(self, Some(target), 0) as i32;
                                min -= dec & 0xffff;
                                pe.dec_count(self, 2);
                                let code = self.card(self.effect(pe).handler.unwrap()).data.code;
                                self.hint_card(code);
                                if min <= 0 {
                                    break;
                                }
                            }
                        }
                    }
                }
                {
                    let c = self.card_mut(target);
                    c.summon.location = LOCATION_HAND as u8;
                    c.summon.type_ = SUMMON_TYPE_NORMAL;
                    c.summon.pzone = false;
                }
                if !arg.tributes.is_empty() {
                    let tributes = std::mem::take(&mut arg.tributes);
                    for c in public_order(self, &tributes) {
                        self.card_mut(c).current.reason_card = Some(target);
                    }
                    target.set_material(self, tributes.clone());
                    self.release(tributes, None, REASON_SUMMON | REASON_MATERIAL, sumplayer);
                    self.card_mut(target).summon.type_ |= SUMMON_TYPE_ADVANCE;
                    self.adjust_all();
                }
                let c = self.card_mut(target);
                c.current.reason_effect = None;
                c.current.reason_player = sumplayer;
                arg.step = 7;
                false
            }
            7 => {
                let proc = summon_procedure_effect.unwrap();
                let v = proc.get_value_card(self, Some(target), 0) as u32;
                {
                    let c = self.card_mut(target);
                    c.summon.location = LOCATION_HAND as u8;
                    c.summon.pzone = false;
                    c.summon.type_ = (v & 0xfffffff) | SUMMON_TYPE_NORMAL;
                    c.current.reason_effect = Some(proc);
                    c.current.reason_player = sumplayer;
                }
                self.returns.set_i32(0, 1);
                if self.effect(proc).operation.is_some() {
                    let pextra = arg.extra_summon_effect;
                    let releasable = self.extra_releasable(pextra, target);
                    self.add_param(Value::Card(target));
                    self.add_param(Value::Int(min_tribute as i64));
                    self.add_param(Value::Int(zone as i64));
                    self.add_param(Value::Int(releasable as i64));
                    self.add_param(Value::effect(pextra));
                    let ne = self.nil_event.clone();
                    self.core.sub_solving_event.push_back(ne);
                    self.emplace_process(units::ExecuteOperation { step: 0, triggering_player: sumplayer, shuffle_check_was_disabled: false, triggering_effect: proc });
                }
                proc.dec_count(self, sumplayer as u32);
                false
            }
            8 => {
                self.core.summon_depth -= 1;
                if self.core.summon_depth != 0 {
                    return true;
                }
                self.break_effect(true);
                if ignore_count {
                    return false;
                }
                self.count_summon(arg.extra_summon_effect, sumplayer, target);
                false
            }
            9 => {
                let mut targetplayer = sumplayer;
                let mut positions = POS_FACEUP_ATTACK as u8;
                if self.is_flag(DUEL_NORMAL_SUMMON_FACEUP_DEF)
                    || self.is_player_affected_by_effect(sumplayer, EFFECT_NORMAL_SUMMON_FACEUP_DEFENSE).is_some()
                    || self.is_player_affected_by_effect(sumplayer, EFFECT_DEVINE_LIGHT).is_some()
                {
                    positions = POS_FACEUP as u8;
                }
                if let Some(proc) = summon_procedure_effect {
                    if self.effect(proc).is_flag(EFFECT_FLAG_SPSUM_PARAM) {
                        positions = (self.effect(proc).s_range as u8) & POS_FACEUP as u8;
                        if self.effect(proc).o_range != 0 {
                            targetplayer = 1 - sumplayer;
                        }
                    }
                }
                let mut eset = Vec::new();
                self.filter_player_effect(sumplayer, EFFECT_FORCE_NORMAL_SUMMON_POSITION, &mut eset, true);
                for eff in eset {
                    if let Some(t) = self.effect(eff).target.clone() {
                        self.add_param(Value::Effect(eff));
                        self.add_param(Value::Card(target));
                        self.add_param(Value::Int(sumplayer as i64));
                        self.add_param(Value::Int(self.card(target).summon.type_ as i64));
                        self.add_param(Value::Int(positions as i64));
                        self.add_param(Value::Int(targetplayer as i64));
                        if !self.check_condition(Some(&t), 6) {
                            continue;
                        }
                    }
                    positions &= eff.get_value(self, 0) as u8;
                }
                target.enable_field_effect(self, false);
                self.move_to_field(target, sumplayer, targetplayer, LOCATION_MZONE as u16, positions, false, 0, zone as u8, false, LocationReason::Summon, true);
                arg.step = 11;
                false
            }
            10 => {
                // gemini handling
                self.core.summon_depth -= 1;
                if self.core.summon_depth != 0 {
                    return true;
                }
                target.enable_field_effect(self, false);
                {
                    let c = self.card_mut(target);
                    c.summon.location = LOCATION_MZONE as u8;
                    c.summon.pzone = false;
                    c.summon.sequence = c.current.sequence as u8;
                    c.summon.type_ |= SUMMON_TYPE_NORMAL;
                    c.current.reason_effect = None;
                    c.current.reason_player = sumplayer;
                }
                let de = self.new_effect();
                let e = self.effect_mut(de);
                e.owner = Some(target);
                e.code = EFFECT_GEMINI_STATUS;
                e.type_ = EFFECT_TYPE_SINGLE as u16;
                e.flag[0] = EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_CLIENT_HINT;
                e.description = 64;
                e.reset_flag = RESET_EVENT + 0x1fe0000;
                target.add_effect(self, de);
                false
            }
            11 => {
                if ignore_count {
                    return false;
                }
                self.count_summon(arg.extra_summon_effect, sumplayer, target);
                false
            }
            12 => {
                let ctrl = self.card(target).current.controler;
                self.set_control(target, ctrl, 0, 0);
                self.core.phase_action = true;
                self.card_mut(target).current.reason = REASON_SUMMON;
                self.card_mut(target).summon.player = sumplayer;
                let code = self.card(target).data.code;
                let loc = target.get_info_location(self);
                let m = self.new_message(MSG_SUMMONING);
                self.msg(m).u32(code).loc(loc);
                if self.is_flag(DUEL_CANNOT_SUMMON_OATH_OLD) {
                    self.core.summon_state_count[sp] = self.core.summon_state_count[sp].wrapping_add(1);
                    self.core.normalsummon_state_count[sp] = self.core.normalsummon_state_count[sp].wrapping_add(1);
                    self.check_card_counter(target, ACTIVITY_SUMMON, sumplayer as i32);
                    self.check_card_counter(target, ACTIVITY_NORMALSUMMON, sumplayer as i32);
                }
                let mats = self.card(target).material_cards.clone();
                if !mats.is_empty() {
                    for mc in public_order(self, &mats) {
                        self.raise_single_event(mc, None, EVENT_BE_PRE_MATERIAL, summon_procedure_effect, REASON_SUMMON, sumplayer, sumplayer, 0);
                    }
                    self.raise_event(mats, EVENT_BE_PRE_MATERIAL, summon_procedure_effect, REASON_SUMMON, sumplayer, sumplayer, 0);
                }
                self.process_single_event();
                self.process_instant_event();
                false
            }
            13 => {
                if !self.core.current_chain.is_empty() || target.is_affected_by_effect(self, EFFECT_CANNOT_DISABLE_SUMMON).is_some() {
                    arg.step = 15;
                }
                false
            }
            14 => {
                self.card_mut(target).set_status(STATUS_SUMMONING, true);
                self.card_mut(target).set_status(STATUS_SUMMON_DISABLED, false);
                self.raise_event_card(Some(target), EVENT_SUMMON, summon_procedure_effect, 0, sumplayer, sumplayer, 0);
                self.process_instant_event();
                self.emplace_process(units::PointEvent { step: 0, skip_trigger: true, skip_freechain: true, skip_new: true });
                false
            }
            15 => {
                if self.card(target).is_status(STATUS_SUMMONING) {
                    arg.step = 15;
                    return false;
                }
                if let Some(proc) = summon_procedure_effect {
                    if !self.is_flag(DUEL_CANNOT_SUMMON_OATH_OLD) {
                        self.remove_oath_effect(Some(proc));
                        let e = self.effect(proc);
                        if e.is_flag(EFFECT_FLAG_COUNT_LIMIT) && e.count_flag as u32 & EFFECT_COUNT_CODE_OATH != 0 {
                            let (cc, cf, hi) = (e.count_code, e.count_flag, e.count_hopt_index);
                            self.dec_effect_code(cc, cf, hi, sumplayer);
                        }
                    }
                }
                for pe in arg.summon_cost_effects.clone() {
                    self.remove_oath_effect(Some(pe));
                }
                if self.card(target).current.location as u32 == LOCATION_MZONE {
                    self.send_to(CardSet::from_card(target), None, REASON_RULE, sumplayer, sumplayer, LOCATION_GRAVE as u16, 0, 0, false);
                }
                self.adjust_instant();
                self.emplace_process(units::PointEvent { step: 0, skip_trigger: false, skip_freechain: false, skip_new: false });
                true
            }
            16 => {
                if let Some(proc) = summon_procedure_effect {
                    self.release_oath_relation(Some(proc));
                }
                for pe in arg.summon_cost_effects.clone() {
                    self.release_oath_relation(Some(pe));
                }
                self.card_mut(target).set_status(STATUS_SUMMONING, false);
                self.card_mut(target).set_status(STATUS_SUMMON_TURN, true);
                target.enable_field_effect(self, true);
                if self.card(target).is_status(STATUS_DISABLED) {
                    target.reset(self, RESET_DISABLE, RESET_EVENT);
                }
                false
            }
            17 => {
                self.new_message(MSG_SUMMONED);
                self.adjust_instant();
                let mats = self.card(target).material_cards.clone();
                if !mats.is_empty() {
                    for mc in public_order(self, &mats) {
                        self.raise_single_event(mc, None, EVENT_BE_MATERIAL, summon_procedure_effect, REASON_SUMMON, sumplayer, sumplayer, 0);
                    }
                    self.raise_event(mats, EVENT_BE_MATERIAL, summon_procedure_effect, REASON_SUMMON, sumplayer, sumplayer, 0);
                }
                self.process_single_event();
                self.process_instant_event();
                false
            }
            18 => {
                if !self.is_flag(DUEL_CANNOT_SUMMON_OATH_OLD) {
                    self.core.summon_state_count[sp] = self.core.summon_state_count[sp].wrapping_add(1);
                    self.core.normalsummon_state_count[sp] = self.core.normalsummon_state_count[sp].wrapping_add(1);
                    self.check_card_counter(target, ACTIVITY_SUMMON, sumplayer as i32);
                    self.check_card_counter(target, ACTIVITY_NORMALSUMMON, sumplayer as i32);
                }
                self.raise_single_event(target, None, EVENT_SUMMON_SUCCESS, summon_procedure_effect, 0, sumplayer, sumplayer, 0);
                self.process_single_event();
                self.raise_event_card(Some(target), EVENT_SUMMON_SUCCESS, summon_procedure_effect, 0, sumplayer, sumplayer, 0);
                self.process_instant_event();
                if self.core.current_chain.is_empty() {
                    self.adjust_all();
                    self.core.hint_timing[sp] |= TIMING_SUMMON;
                    self.emplace_process(units::PointEvent { step: 0, skip_trigger: false, skip_freechain: false, skip_new: false });
                }
                true
            }
            _ => true,
        }
    }

    /// Counts a normal summon/set (or uses the extra summon effect).
    pub(crate) fn count_summon(&mut self, pextra: Option<EffectId>, sumplayer: u8, target: CardId) {
        let sp = sumplayer as usize;
        match pextra {
            None => self.core.summon_count[sp] += 1,
            Some(pe) => {
                self.core.extra_summon[sp] = 1;
                let code = self.card(self.effect(pe).handler.unwrap()).data.code;
                self.hint_card(code);
                if self.effect(pe).operation.is_some() {
                    self.add_param(Value::Card(target));
                    let ne = self.nil_event.clone();
                    self.core.sub_solving_event.push_back(ne);
                    self.emplace_process(units::ExecuteOperation { step: 0, triggering_player: sumplayer, shuffle_check_was_disabled: false, triggering_effect: pe });
                }
            }
        }
    }

    pub fn process_flip_summon(&mut self, arg: &mut units::FlipSummon) -> bool {
        let (sumplayer, target) = (arg.sumplayer, arg.target);
        let sp = sumplayer as usize;
        match arg.step {
            0 => {
                let c = self.card(target);
                if c.current.location as u32 != LOCATION_MZONE || c.current.position & POS_FACEDOWN == 0 {
                    return true;
                }
                if self.check_unique_onfield(target, sumplayer, LOCATION_MZONE as u8, None).is_some() {
                    return true;
                }
                let mut costs = Vec::new();
                target.filter_effect(self, EFFECT_FLIPSUMMON_COST, &mut costs, true);
                arg.flip_summon_cost_effects = costs.clone();
                for pe in costs {
                    if self.effect(pe).operation.is_some() {
                        let ne = self.nil_event.clone();
                        self.core.sub_solving_event.push_back(ne);
                        self.emplace_process(units::ExecuteOperation { step: 0, triggering_player: sumplayer, shuffle_check_was_disabled: false, triggering_effect: pe });
                    }
                }
                false
            }
            1 => {
                let fid = self.infos.field_id;
                self.infos.field_id += 1;
                {
                    let c = self.card_mut(target);
                    c.previous.position = c.current.position;
                    c.current.position = POS_FACEUP_ATTACK;
                    c.summon.player = sumplayer;
                    c.fieldid = fid;
                }
                self.core.phase_action = true;
                if self.is_flag(DUEL_CANNOT_SUMMON_OATH_OLD) {
                    self.core.flipsummon_state_count[sp] = self.core.flipsummon_state_count[sp].wrapping_add(1);
                    self.check_card_counter(target, ACTIVITY_FLIPSUMMON, sumplayer as i32);
                }
                let code = self.card(target).data.code;
                let loc = target.get_info_location(self);
                let m = self.new_message(MSG_FLIPSUMMONING);
                self.msg(m).u32(code).loc(loc);
                if target.is_affected_by_effect(self, EFFECT_CANNOT_DISABLE_FLIP_SUMMON).is_some() {
                    arg.step = 2;
                } else {
                    self.card_mut(target).set_status(STATUS_SUMMONING, true);
                    self.card_mut(target).set_status(STATUS_SUMMON_DISABLED, false);
                    self.raise_event_card(Some(target), EVENT_FLIP_SUMMON, None, 0, sumplayer, sumplayer, 0);
                    self.process_instant_event();
                    self.emplace_process(units::PointEvent { step: 0, skip_trigger: true, skip_freechain: true, skip_new: true });
                }
                false
            }
            2 => {
                if self.card(target).is_status(STATUS_SUMMONING) {
                    return false;
                }
                for pe in arg.flip_summon_cost_effects.clone() {
                    self.remove_oath_effect(Some(pe));
                }
                if self.card(target).current.location as u32 == LOCATION_MZONE {
                    self.send_to(CardSet::from_card(target), None, REASON_RULE, sumplayer, sumplayer, LOCATION_GRAVE as u16, 0, 0, false);
                }
                self.emplace_process(units::PointEvent { step: 0, skip_trigger: false, skip_freechain: false, skip_new: false });
                true
            }
            3 => {
                for pe in arg.flip_summon_cost_effects.clone() {
                    self.release_oath_relation(Some(pe));
                }
                self.card_mut(target).set_status(STATUS_SUMMONING, false);
                target.enable_field_effect(self, true);
                if self.card(target).is_status(STATUS_DISABLED) {
                    target.reset(self, RESET_DISABLE, RESET_EVENT);
                }
                self.card_mut(target).set_status(STATUS_FLIP_SUMMON_TURN, true);
                false
            }
            4 => {
                self.new_message(MSG_FLIPSUMMONED);
                if !self.is_flag(DUEL_CANNOT_SUMMON_OATH_OLD) {
                    self.core.flipsummon_state_count[sp] = self.core.flipsummon_state_count[sp].wrapping_add(1);
                    self.check_card_counter(target, ACTIVITY_FLIPSUMMON, sumplayer as i32);
                }
                self.adjust_instant();
                for code in [EVENT_FLIP, EVENT_FLIP_SUMMON_SUCCESS, EVENT_CHANGE_POS] {
                    self.raise_single_event(target, None, code, None, 0, sumplayer, sumplayer, 0);
                }
                self.process_single_event();
                for code in [EVENT_FLIP, EVENT_FLIP_SUMMON_SUCCESS, EVENT_CHANGE_POS] {
                    self.raise_event_card(Some(target), code, None, 0, sumplayer, sumplayer, 0);
                }
                self.process_instant_event();
                self.adjust_all();
                if self.core.current_chain.is_empty() {
                    self.core.hint_timing[sp] |= TIMING_FLIPSUMMON;
                    self.emplace_process(units::PointEvent { step: 0, skip_trigger: false, skip_freechain: false, skip_new: false });
                }
                true
            }
            _ => true,
        }
    }
}
