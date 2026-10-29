//! Operations (`operations.cpp` part 6: Special Summons).

use crate::card::{card_public_sort, public_order, sort_by_less};
use crate::constants::*;
use crate::duel::Duel;
use crate::ops1::new_spsummon_rule;
use crate::processor5::inc_map;
use crate::types::*;
use crate::units::{self, LocationReason};

/// The reason the materials of a Special Summon by a procedure get (`proc->value`).
fn proc_matreason(d: &Duel, proc: EffectId) -> u32 {
    let v = d.effect(proc).value as u32;
    if v == SUMMON_TYPE_SYNCHRO {
        REASON_SYNCHRO
    } else if v == SUMMON_TYPE_XYZ {
        REASON_XYZ
    } else if v == SUMMON_TYPE_LINK {
        REASON_LINK
    } else {
        REASON_SPSUMMON
    }
}

impl Duel {
    /// The procedure parameters (`target, must_use_mats, only_use_mats[, minc, maxc]`).
    fn add_spsummon_proc_params(&mut self, target: CardId) {
        self.add_param(Value::Card(target));
        let must = self.core.must_use_mats.clone().map_or(Value::Nil, Value::Group);
        self.add_param(must);
        let only = self.core.only_use_mats.clone().map_or(Value::Nil, Value::Group);
        self.add_param(only);
        if self.core.forced_summon_minc != 0 {
            self.add_param(Value::Int(self.core.forced_summon_minc as i64));
            self.add_param(Value::Int(self.core.forced_summon_maxc as i64));
        }
    }
    /// `positions &=` the values of the applicable EFFECT_FORCE_SPSUMMON_POSITION.
    #[allow(clippy::too_many_arguments)]
    fn force_spsummon_position_proc(&mut self, sumplayer: u8, pcard: CardId, sumtype: u32, mut positions: u8, targetplayer: u8, proc: EffectId) -> u8 {
        let mut eset = Vec::new();
        self.filter_player_effect(sumplayer, EFFECT_FORCE_SPSUMMON_POSITION, &mut eset, true);
        for eff in eset {
            if let Some(t) = self.effect(eff).target.clone() {
                self.add_param(Value::Effect(eff));
                self.add_param(Value::Card(pcard));
                self.add_param(Value::Int(sumplayer as i64));
                self.add_param(Value::Int(sumtype as i64));
                self.add_param(Value::Int(positions as i64));
                self.add_param(Value::Int(targetplayer as i64));
                self.add_param(Value::Effect(proc));
                if !self.check_condition(Some(&t), 7) {
                    continue;
                }
            }
            positions &= eff.get_value(self, 0) as u8;
        }
        positions
    }
    fn execute_operation(&mut self, peffect: EffectId, playerid: u8) {
        let ne = self.nil_event.clone();
        self.core.sub_solving_event.push_back(ne);
        self.emplace_process(units::ExecuteOperation { step: 0, triggering_player: playerid, shuffle_check_was_disabled: false, triggering_effect: peffect });
    }
    fn execute_spsummon_costs(&mut self, costs: &[EffectId], playerid: u8) {
        for &pe in costs {
            if self.effect(pe).operation.is_some() {
                self.execute_operation(pe, playerid);
            }
        }
    }
    fn spsummon_once_inc(&mut self, playerid: u8, cards: &[CardId]) {
        let mut codes: Vec<u32> = cards.iter().map(|&c| self.card(c).spsummon_code).filter(|&c| c != 0).collect();
        codes.sort_unstable();
        codes.dedup();
        for c in codes {
            inc_map(&mut self.core.spsummon_once_map[playerid as usize], c);
        }
    }
    fn write_spsummoning(&mut self, pcard: CardId) {
        let c = self.card(pcard);
        let code = if c.current.position & POS_FACEDOWN == 0 { c.data.code } else { 0 };
        let loc = pcard.get_info_location(self);
        let m = self.new_message(MSG_SPSUMMONING);
        self.msg(m).u32(code).loc(loc);
    }
    /// Removes the oath effects of a cancelled summon procedure.
    fn cancel_proc_oath(&mut self, proc: EffectId, sumplayer: u8) {
        self.remove_oath_effect(Some(proc));
        let e = self.effect(proc);
        if e.is_flag(EFFECT_FLAG_COUNT_LIMIT) && (e.count_flag as u32 & EFFECT_COUNT_CODE_OATH) != 0 {
            let (code, flag, idx) = (e.count_code, e.count_flag, e.count_hopt_index);
            self.dec_effect_code(code, flag, idx, sumplayer);
        }
    }

    pub fn process_spsummon_rule(&mut self, arg: &mut units::SpSummonRule) -> bool {
        let (sumplayer, target, summon_type) = (arg.sumplayer, arg.target, arg.summon_type);
        let sp = sumplayer as usize;
        match arg.step {
            0 => {
                let mut eset = Vec::new();
                let must_mats = self.core.must_use_mats.clone();
                let materials = self.core.only_use_mats.clone();
                let minc = self.core.forced_summon_minc;
                let maxc = self.core.forced_summon_maxc;
                target.filter_spsummon_procedure(self, sumplayer, &mut eset, summon_type);
                target.filter_spsummon_procedure_g(self, sumplayer, &mut eset);
                self.core.must_use_mats = must_mats;
                self.core.only_use_mats = materials;
                self.core.forced_summon_minc = minc;
                self.core.forced_summon_maxc = maxc;
                if eset.is_empty() {
                    return true;
                }
                self.core.select_effects.clear();
                self.core.select_options.clear();
                for pe in eset {
                    self.core.select_effects.push(Some(pe));
                    let d = self.effect(pe).description;
                    self.core.select_options.push(d);
                }
                self.sort_select_options();
                if self.core.select_options.len() == 1 {
                    self.returns.set_i32(0, 0);
                } else {
                    self.emplace_process(units::SelectOption { step: 0, playerid: sumplayer });
                }
                false
            }
            1 => {
                let idx = self.returns.at_i32(0) as usize;
                let peffect = self.core.select_effects[idx].unwrap();
                arg.summon_proc_effect = Some(peffect);
                if self.effect(peffect).code == EFFECT_SPSUMMON_PROC_G {
                    arg.step = 19;
                    return false;
                }
                self.returns.set_i32(0, 1);
                if self.effect(peffect).target.is_some() {
                    self.add_spsummon_proc_params(target);
                    let ne = self.nil_event.clone();
                    self.core.sub_solving_event.push_back(ne);
                    self.emplace_process(units::ExecuteTarget { step: 0, triggering_player: sumplayer, shuffle_check_was_disabled: false, triggering_effect: peffect });
                }
                false
            }
            2 => {
                if self.returns.at_i32(0) == 0 {
                    return true;
                }
                let mut costs = Vec::new();
                target.filter_effect(self, EFFECT_SPSUMMON_COST, &mut costs, true);
                arg.spsummon_cost_effects = costs.clone();
                self.execute_spsummon_costs(&costs, sumplayer);
                false
            }
            3 => {
                let proc = arg.summon_proc_effect.unwrap();
                self.card_mut(target).material_cards.clear();
                if self.effect(proc).operation.is_some() {
                    self.add_spsummon_proc_params(target);
                    self.core.forced_summon_minc = 0;
                    self.core.forced_summon_maxc = 0;
                    self.execute_operation(proc, sumplayer);
                }
                proc.dec_count(self, sumplayer as u32);
                false
            }
            4 => {
                self.core.must_use_mats = None;
                self.core.only_use_mats = None;
                let proc = arg.summon_proc_effect.unwrap();
                let mut targetplayer = sumplayer;
                let mut positions = POS_FACEUP as u8;
                let e = self.effect(proc);
                if e.is_flag(EFFECT_FLAG_SPSUM_PARAM) {
                    positions = e.s_range as u8;
                    targetplayer = if e.o_range == 0 { sumplayer } else { 1 - sumplayer };
                }
                if positions == 0 {
                    positions = POS_FACEUP_ATTACK as u8;
                }
                let mut retval = Vec::new();
                proc.get_values_card(self, Some(target), 0, &mut retval);
                let sumtype = (if !retval.is_empty() { retval[0] as u32 & 0xf00ffff } else { 0 }) | SUMMON_TYPE_SPECIAL;
                let zone = if retval.len() > 1 { retval[1] as u32 } else { 0xff };
                {
                    let c = self.card_mut(target);
                    c.summon.type_ = sumtype;
                    c.summon.location = c.current.location;
                    c.summon.sequence = c.current.sequence as u8;
                    c.summon.pzone = c.current.pzone;
                }
                target.enable_field_effect(self, false);
                let positions = self.force_spsummon_position_proc(sumplayer, target, sumtype, positions, targetplayer, proc);
                self.move_to_field(target, sumplayer, targetplayer, LOCATION_MZONE as u16, positions, false, 0, zone as u8, true, LocationReason::SpSummon, true);
                {
                    let c = self.card_mut(target);
                    c.current.reason = REASON_SPSUMMON;
                    c.current.reason_effect = Some(proc);
                    c.current.reason_player = sumplayer;
                    c.summon.player = sumplayer;
                }
                if self.is_flag(DUEL_CANNOT_SUMMON_OATH_OLD) {
                    self.set_spsummon_counter(sumplayer, true, false);
                    self.check_card_counter(target, ACTIVITY_SPSUMMON, sumplayer as i32);
                }
                let code = self.card(target).spsummon_code;
                if self.is_flag(DUEL_SPSUMMON_ONCE_OLD_NEGATE) && code != 0 {
                    inc_map(&mut self.core.spsummon_once_map[sp], code);
                }
                self.break_effect(true);
                false
            }
            5 => {
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
                let ctrl = self.card(target).current.controler;
                self.set_control(target, ctrl, 0, 0);
                self.core.phase_action = true;
                self.card_mut(target).current.reason_effect = arg.summon_proc_effect;
                self.write_spsummoning(target);
                false
            }
            6 => {
                let proc = arg.summon_proc_effect.unwrap();
                let matreason = proc_matreason(self, proc);
                let mats = self.card(target).material_cards.clone();
                if !mats.is_empty() {
                    for mcard in public_order(self, &mats) {
                        self.raise_single_event(mcard, None, EVENT_BE_PRE_MATERIAL, Some(proc), matreason, sumplayer, sumplayer, 0);
                    }
                }
                self.raise_event(mats, EVENT_BE_PRE_MATERIAL, Some(proc), matreason, sumplayer, sumplayer, 0);
                self.process_single_event();
                self.process_instant_event();
                false
            }
            7 => {
                if self.core.current_chain.is_empty() {
                    if target.is_affected_by_effect(self, EFFECT_CANNOT_DISABLE_SPSUMMON).is_some() {
                        arg.step = 14;
                    } else {
                        arg.step = 9;
                    }
                } else {
                    arg.step = 14;
                }
                false
            }
            10 => {
                {
                    let c = self.card_mut(target);
                    c.set_status(STATUS_SUMMONING, true);
                    c.set_status(STATUS_SUMMON_DISABLED, false);
                }
                self.raise_event_card(Some(target), EVENT_SPSUMMON, arg.summon_proc_effect, 0, sumplayer, sumplayer, 0);
                self.process_instant_event();
                self.emplace_process(units::PointEvent { step: 0, skip_trigger: true, skip_freechain: true, skip_new: true });
                false
            }
            11 => {
                if self.card(target).is_status(STATUS_SUMMONING) {
                    arg.step = 14;
                    return false;
                }
                for pe in arg.spsummon_cost_effects.clone() {
                    self.remove_oath_effect(Some(pe));
                }
                if !self.is_flag(DUEL_CANNOT_SUMMON_OATH_OLD) {
                    let proc = arg.summon_proc_effect.unwrap();
                    self.cancel_proc_oath(proc, sumplayer);
                }
                if self.card(target).current.location as u32 == LOCATION_MZONE {
                    self.send_to(CardSet::from_card(target), None, REASON_RULE, sumplayer, sumplayer, LOCATION_GRAVE as u16, 0, 0, false);
                }
                self.adjust_instant();
                self.emplace_process(units::PointEvent { step: 0, skip_trigger: false, skip_freechain: false, skip_new: false });
                true
            }
            15 => {
                self.release_oath_relation(arg.summon_proc_effect);
                for pe in arg.spsummon_cost_effects.clone() {
                    self.release_oath_relation(Some(pe));
                }
                {
                    let c = self.card_mut(target);
                    c.set_status(STATUS_SUMMONING, false);
                    c.set_status(STATUS_PROC_COMPLETE | STATUS_SPSUMMON_TURN, true);
                }
                target.enable_field_effect(self, true);
                if self.card(target).is_status(STATUS_DISABLED) {
                    target.reset(self, RESET_DISABLE, RESET_EVENT);
                }
                false
            }
            16 => {
                self.new_message(MSG_SPSUMMONED);
                self.adjust_instant();
                let proc = arg.summon_proc_effect.unwrap();
                let matreason = proc_matreason(self, proc);
                let mats = self.card(target).material_cards.clone();
                if !mats.is_empty() {
                    for mcard in public_order(self, &mats) {
                        self.raise_single_event(mcard, None, EVENT_BE_MATERIAL, Some(proc), matreason, sumplayer, sumplayer, 0);
                    }
                }
                self.raise_event(mats, EVENT_BE_MATERIAL, Some(proc), matreason, sumplayer, sumplayer, 0);
                self.process_single_event();
                self.process_instant_event();
                false
            }
            17 => {
                if !self.is_flag(DUEL_CANNOT_SUMMON_OATH_OLD) {
                    self.set_spsummon_counter(sumplayer, true, false);
                    self.check_card_counter(target, ACTIVITY_SPSUMMON, sumplayer as i32);
                }
                let code = self.card(target).spsummon_code;
                if !self.is_flag(DUEL_SPSUMMON_ONCE_OLD_NEGATE) && code != 0 {
                    inc_map(&mut self.core.spsummon_once_map[sp], code);
                }
                self.raise_single_event(target, None, EVENT_SPSUMMON_SUCCESS, arg.summon_proc_effect, 0, sumplayer, sumplayer, 0);
                self.process_single_event();
                self.raise_event_card(Some(target), EVENT_SPSUMMON_SUCCESS, arg.summon_proc_effect, 0, sumplayer, sumplayer, 0);
                self.process_instant_event();
                if self.core.current_chain.is_empty() {
                    self.adjust_all();
                    self.core.hint_timing[sp] |= TIMING_SPSUMMON;
                    self.emplace_process(units::PointEvent { step: 0, skip_trigger: false, skip_freechain: false, skip_new: false });
                }
                true
            }
            20 => {
                let proc = arg.summon_proc_effect.unwrap();
                let g = self.new_group(CardSet::new());
                arg.cards_to_summon_g = Some(g);
                if self.effect(proc).operation.is_some() {
                    let ne = self.nil_event.clone();
                    self.core.sub_solving_event.push_back(ne);
                    self.add_param(Value::Card(target));
                    self.add_param(Value::SharedGroup(g));
                    self.add_param(Value::Bool(arg.is_mid_chain));
                    self.emplace_process(units::ExecuteOperation { step: 0, triggering_player: sumplayer, shuffle_check_was_disabled: false, triggering_effect: proc });
                }
                proc.dec_count(self, sumplayer as u32);
                false
            }
            21 => {
                let g = arg.cards_to_summon_g.unwrap();
                let set = self.group(g).clone();
                for pcard in public_order(self, &set) {
                    let c = self.card(pcard);
                    if c.data.type_ & TYPE_MONSTER == 0
                        || c.current.location as u32 == LOCATION_MZONE
                        || self.check_unique_onfield(pcard, sumplayer, LOCATION_MZONE as u8, None).is_some()
                        || pcard.is_affected_by_effect(self, EFFECT_CANNOT_SPECIAL_SUMMON).is_some()
                    {
                        self.group_mut(g).remove(pcard);
                        continue;
                    }
                    let mut costs = Vec::new();
                    pcard.filter_effect(self, EFFECT_SPSUMMON_COST, &mut costs, true);
                    arg.spsummon_cost_effects = costs.clone();
                    self.execute_spsummon_costs(&costs, sumplayer);
                }
                false
            }
            22 => {
                let g = arg.cards_to_summon_g.unwrap();
                if self.group(g).is_empty() {
                    return true;
                }
                self.core.phase_action = true;
                let set = self.group(g).clone();
                arg.iter_order = public_order(self, &set);
                arg.iter_pos = 0;
                false
            }
            23 => {
                let proc = arg.summon_proc_effect.unwrap();
                let pcard = arg.iter_order[arg.iter_pos];
                pcard.enable_field_effect(self, false);
                {
                    let c = self.card_mut(pcard);
                    c.current.reason = REASON_SPSUMMON;
                    c.current.reason_effect = Some(proc);
                    c.current.reason_player = sumplayer;
                    c.summon.player = sumplayer;
                }
                let sumtype = (proc.get_value_card(self, Some(pcard), 0) as u32 & 0xff00ffff) | SUMMON_TYPE_SPECIAL;
                {
                    let c = self.card_mut(pcard);
                    c.summon.type_ = sumtype;
                    c.summon.location = c.current.location;
                    c.summon.sequence = c.current.sequence as u8;
                    c.summon.pzone = c.current.pzone;
                }
                let positions = self.force_spsummon_position_proc(sumplayer, pcard, sumtype, POS_FACEUP as u8, sumplayer, proc);
                if self.is_flag(DUEL_CANNOT_SUMMON_OATH_OLD) {
                    self.check_card_counter(pcard, ACTIVITY_SPSUMMON, sumplayer as i32);
                }
                let mut zone: u32 = 0xff;
                let mut flag1 = 0u32;
                let mut flag2 = 0u32;
                let mut ct1 = self.get_tofield_count(Some(pcard), sumplayer, LOCATION_MZONE as u8, sumplayer as u32, LocationReason::ToField, zone, Some(&mut flag1));
                let mut ct2 = self.get_spsummonable_count_fromex(Some(pcard), sumplayer, sumplayer, zone, Some(&mut flag2));
                for k in arg.iter_pos..arg.iter_order.len() {
                    if self.card(arg.iter_order[k]).current.location as u32 != LOCATION_EXTRA {
                        ct1 -= 1;
                    } else {
                        ct2 -= 1;
                    }
                }
                if self.card(pcard).current.location as u32 != LOCATION_EXTRA {
                    if ct2 == 0 {
                        zone = flag2;
                    }
                } else if ct1 == 0 {
                    zone = flag1;
                }
                self.move_to_field(pcard, sumplayer, sumplayer, LOCATION_MZONE as u16, positions, false, 0, zone as u8, true, LocationReason::SpSummon, true);
                false
            }
            24 => {
                let pcard = arg.iter_order[arg.iter_pos];
                arg.iter_pos += 1;
                self.write_spsummoning(pcard);
                let ctrl = self.card(pcard).current.controler;
                self.set_control(pcard, ctrl, 0, 0);
                self.card_mut(pcard).set_status(STATUS_SPSUMMON_STEP, true);
                if arg.iter_pos < arg.iter_order.len() {
                    arg.step = 22;
                }
                false
            }
            25 => {
                let g = arg.cards_to_summon_g.unwrap();
                if self.is_flag(DUEL_CANNOT_SUMMON_OATH_OLD) {
                    self.set_spsummon_counter(sumplayer, true, false);
                }
                let set = self.group(g).clone();
                let order = public_order(self, &set);
                if self.is_flag(DUEL_SPSUMMON_ONCE_OLD_NEGATE) {
                    self.spsummon_once_inc(sumplayer, &order);
                }
                if self.core.current_chain.is_empty() {
                    let mut cset = CardSet::new();
                    for &pcard in &order {
                        let c = self.card_mut(pcard);
                        c.set_status(STATUS_SUMMONING, true);
                        c.set_status(STATUS_SPSUMMON_STEP, false);
                        if pcard.is_affected_by_effect(self, EFFECT_CANNOT_DISABLE_SPSUMMON).is_none() {
                            cset.insert(pcard);
                        }
                    }
                    if !cset.is_empty() {
                        self.raise_event(cset, EVENT_SPSUMMON, arg.summon_proc_effect, 0, sumplayer, sumplayer, 0);
                        self.process_instant_event();
                        self.emplace_process(units::PointEvent { step: 0, skip_trigger: true, skip_freechain: true, skip_new: true });
                    }
                } else {
                    for &pcard in &order {
                        self.card_mut(pcard).set_status(STATUS_SPSUMMON_STEP, false);
                    }
                    arg.step = 26;
                }
                false
            }
            26 => {
                let g = arg.cards_to_summon_g.unwrap();
                let mut cset = CardSet::new();
                let set = self.group(g).clone();
                for pcard in public_order(self, &set) {
                    if !self.card(pcard).is_status(STATUS_SUMMONING) {
                        self.group_mut(g).remove(pcard);
                        if self.card(pcard).current.location as u32 == LOCATION_MZONE {
                            cset.insert(pcard);
                        }
                    }
                }
                if !cset.is_empty() {
                    self.send_to(cset, None, REASON_RULE, sumplayer, sumplayer, LOCATION_GRAVE as u16, 0, 0, false);
                    self.adjust_instant();
                }
                if self.group(g).is_empty() {
                    if !self.is_flag(DUEL_CANNOT_SUMMON_OATH_OLD) {
                        let proc = arg.summon_proc_effect.unwrap();
                        self.cancel_proc_oath(proc, sumplayer);
                    }
                    for pe in arg.spsummon_cost_effects.clone() {
                        self.remove_oath_effect(Some(pe));
                    }
                    self.emplace_process(units::PointEvent { step: 0, skip_trigger: false, skip_freechain: false, skip_new: false });
                    return true;
                }
                false
            }
            27 => {
                let g = arg.cards_to_summon_g.unwrap();
                self.release_oath_relation(arg.summon_proc_effect);
                for pe in arg.spsummon_cost_effects.clone() {
                    self.release_oath_relation(Some(pe));
                }
                let set = self.group(g).clone();
                for pcard in public_order(self, &set) {
                    {
                        let c = self.card_mut(pcard);
                        c.set_status(STATUS_SUMMONING, false);
                        c.set_status(STATUS_SPSUMMON_TURN, true);
                    }
                    pcard.enable_field_effect(self, true);
                    if self.card(pcard).is_status(STATUS_DISABLED) {
                        pcard.reset(self, RESET_DISABLE, RESET_EVENT);
                    }
                }
                false
            }
            28 => {
                let g = arg.cards_to_summon_g.unwrap();
                self.new_message(MSG_SPSUMMONED);
                let set = self.group(g).clone();
                if !self.is_flag(DUEL_CANNOT_SUMMON_OATH_OLD) {
                    self.set_spsummon_counter(sumplayer, true, false);
                    self.check_card_counter_group(&set, ACTIVITY_SPSUMMON, sumplayer as i32);
                }
                let order = public_order(self, &set);
                if !self.is_flag(DUEL_SPSUMMON_ONCE_OLD_NEGATE) {
                    self.spsummon_once_inc(sumplayer, &order);
                }
                for &pcard in &order {
                    let c = self.card(pcard);
                    let (re, rp, sp2) = (c.current.reason_effect, c.current.reason_player, c.summon.player);
                    self.raise_single_event(pcard, None, EVENT_SPSUMMON_SUCCESS, re, 0, rp, sp2, 0);
                }
                self.process_single_event();
                let set = self.group(g).clone();
                self.raise_event(set, EVENT_SPSUMMON_SUCCESS, arg.summon_proc_effect, 0, sumplayer, sumplayer, 0);
                self.process_instant_event();
                if self.core.current_chain.is_empty() {
                    self.adjust_all();
                    self.core.hint_timing[sp] |= TIMING_SPSUMMON;
                    self.emplace_process(units::PointEvent { step: 0, skip_trigger: false, skip_freechain: false, skip_new: false });
                }
                true
            }
            _ => true,
        }
    }

    pub fn process_spsummon_rule_group(&mut self, arg: &mut units::SpSummonRuleGroup) -> bool {
        let (sumplayer, summon_type) = (arg.sumplayer, arg.summon_type);
        match arg.step {
            0 => {
                let mut eset = Vec::new();
                self.filter_field_effect(EFFECT_SPSUMMON_PROC_G, &mut eset, true);
                self.core.select_effects.clear();
                self.core.select_options.clear();
                for peff in eset {
                    if peff.get_value(self, 0) as u32 != summon_type {
                        continue;
                    }
                    let pcard = peff.get_handler(self);
                    let ctrl = self.card(pcard).current.controler;
                    if ctrl != sumplayer && !self.effect(peff).is_flag(EFFECT_FLAG_BOTH_SIDE) {
                        continue;
                    }
                    let oreason = self.core.reason_effect;
                    let op = self.core.reason_player;
                    self.core.reason_effect = Some(peff);
                    self.core.reason_player = ctrl;
                    self.add_param(Value::Effect(peff));
                    self.add_param(Value::Card(pcard));
                    self.add_param(Value::Bool(true));
                    self.add_param(Value::effect(oreason));
                    self.add_param(Value::Int(sumplayer as i64));
                    let cond = self.effect(peff).condition().clone();
                    if self.check_condition(cond.as_ref(), 5) {
                        self.core.select_effects.push(Some(peff));
                        let d = self.effect(peff).description;
                        self.core.select_options.push(d);
                    }
                    self.core.reason_effect = oreason;
                    self.core.reason_player = op;
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
                false
            }
            1 => {
                let idx = self.returns.at_i32(0) as usize;
                let peffect = self.core.select_effects[idx].unwrap();
                let handler = peffect.get_handler(self);
                self.emplace_process(new_spsummon_rule(20, sumplayer, handler, summon_type, true, Some(peffect)));
                true
            }
            _ => false,
        }
    }

    pub fn process_spsummon_step(&mut self, arg: &mut units::SpSummonStep) -> bool {
        let (targets, target) = (arg.targets, arg.target);
        let mut zone = arg.zone;
        let param = self.card(target).spsummon_param;
        let playerid = ((param >> 24) & 0xf) as u8;
        let nocheck = ((param >> 16) & 0xff) != 0;
        let nolimit = ((param >> 8) & 0xff) != 0;
        let positions = (param & 0xff) as u8;
        match arg.step {
            0 => {
                if target.is_affected_by_effect(self, EFFECT_REVIVE_LIMIT).is_some() && !self.card(target).is_status(STATUS_PROC_COMPLETE) {
                    let loc = self.card(target).current.location as u32;
                    if (!nolimit && (loc & 0x38) != 0) || (!nocheck && !nolimit && (loc & 0x3) != 0) {
                        arg.step = 4;
                        return false;
                    }
                }
                let (sumtype, sumplayer) = {
                    let c = self.card(target);
                    (c.summon.type_ & 0xff00ffff, c.summon.player)
                };
                if self.card(target).current.location as u32 == LOCATION_MZONE
                    || ((positions as u32 & POS_FACEDOWN) == 0 && self.check_unique_onfield(target, playerid, LOCATION_MZONE as u8, None).is_some())
                    || !self.is_player_can_spsummon_ex(self.core.reason_effect, sumtype, positions, sumplayer, playerid, target, None)
                    || (!nocheck && self.card(target).data.type_ & TYPE_MONSTER == 0)
                {
                    arg.step = 4;
                    return false;
                }
                if self.get_useable_count(Some(target), playerid, LOCATION_MZONE as u8, sumplayer, LocationReason::ToField, zone, None) <= 0 {
                    if self.card(target).current.location as u32 != LOCATION_GRAVE {
                        self.core.ss_tograve_set.insert(target);
                    }
                    arg.step = 4;
                    return false;
                }
                if !nocheck {
                    let mut eset = Vec::new();
                    target.filter_effect(self, EFFECT_SPSUMMON_CONDITION, &mut eset, true);
                    for peff in eset {
                        let re = self.core.reason_effect;
                        self.add_param(Value::effect(re));
                        self.add_param(Value::Int(sumplayer as i64));
                        self.add_param(Value::Int(sumtype as i64));
                        self.add_param(Value::Int(positions as i64));
                        self.add_param(Value::Int(playerid as i64));
                        if !peff.check_value_condition(self, 5) {
                            arg.step = 4;
                            return false;
                        }
                    }
                }
                let mut costs = Vec::new();
                target.filter_effect(self, EFFECT_SPSUMMON_COST, &mut costs, true);
                arg.spsummon_cost_effects = costs.clone();
                self.execute_spsummon_costs(&costs, sumplayer);
                false
            }
            1 => {
                for pe in arg.spsummon_cost_effects.clone() {
                    self.release_oath_relation(Some(pe));
                }
                if targets.is_none() {
                    self.core.special_summoning.insert(target);
                }
                target.enable_field_effect(self, false);
                let sumplayer = self.card(target).summon.player;
                if self.is_flag(DUEL_CANNOT_SUMMON_OATH_OLD) {
                    self.check_card_counter(target, ACTIVITY_SPSUMMON, sumplayer as i32);
                }
                let extra = zone & 0xff == 0;
                if let Some(g) = targets {
                    if self.is_flag(DUEL_EMZONE) {
                        let mut flag1 = 0u32;
                        let mut flag2 = 0u32;
                        let mut ct1 = self.get_tofield_count(Some(target), playerid, LOCATION_MZONE as u8, sumplayer as u32, LocationReason::ToField, zone, Some(&mut flag1));
                        let mut ct2 = self.get_spsummonable_count_fromex(Some(target), playerid, sumplayer, zone, Some(&mut flag2));
                        let set = self.group(g).clone();
                        for pcard in public_order(self, &set) {
                            let l = self.card(pcard).current.location as u32;
                            if l == LOCATION_MZONE {
                                continue;
                            }
                            if l != LOCATION_EXTRA {
                                ct1 -= 1;
                            } else {
                                ct2 -= 1;
                            }
                        }
                        if self.card(target).current.location as u32 != LOCATION_EXTRA {
                            if ct2 == 0 {
                                zone &= flag2;
                                if !extra {
                                    zone &= !0x60;
                                }
                            }
                        } else if ct1 == 0 {
                            zone &= flag1;
                        }
                    }
                }
                self.move_to_field(target, sumplayer, playerid, LOCATION_MZONE as u16, positions, false, 0, zone as u8, false, LocationReason::SpSummon, true);
                false
            }
            2 => {
                self.write_spsummoning(target);
                false
            }
            3 => {
                self.returns.set_i32(0, 1);
                let ctrl = self.card(target).current.controler;
                self.set_control(target, ctrl, 0, 0);
                self.card_mut(target).set_status(STATUS_SPSUMMON_STEP, true);
                true
            }
            5 => {
                self.returns.set_i32(0, 0);
                let c = self.card_mut(target);
                c.current.reason = c.temp.reason;
                c.current.reason_effect = c.temp.reason_effect;
                c.current.reason_player = c.temp.reason_player;
                if let Some(g) = targets {
                    self.group_mut(g).remove(target);
                }
                true
            }
            _ => true,
        }
    }

    pub fn process_spsummon(&mut self, arg: &mut units::SpSummon) -> bool {
        let (reason_effect, reason_player, zone) = (arg.reason_effect, arg.reason_player, arg.zone);
        let g = arg.targets.unwrap();
        match arg.step {
            0 => {
                let set = self.group(g).clone();
                let tp = self.infos.turn_player;
                let mut cvs = Vec::new();
                let mut cvo = Vec::new();
                for pcard in public_order(self, &set) {
                    if self.card(pcard).summon.player == tp {
                        cvs.push(pcard);
                    } else {
                        cvo.push(pcard);
                    }
                }
                for (cv, p) in [(cvs, tp), (cvo, 1 - tp)] {
                    if cv.is_empty() {
                        continue;
                    }
                    let mut cv = cv;
                    if cv.len() > 1 {
                        sort_by_less(&mut cv, |a, b| card_public_sort(self, a, b));
                    }
                    self.core.hint_timing[p as usize] |= TIMING_SPSUMMON;
                    for pcard in cv {
                        self.emplace_process(units::SpSummonStep { step: 0, zone, targets: Some(g), target: pcard, spsummon_cost_effects: Vec::new() });
                    }
                }
                false
            }
            1 => {
                if !self.core.ss_tograve_set.is_empty() {
                    let set = self.core.ss_tograve_set.clone();
                    self.send_to(set, None, REASON_RULE, PLAYER_NONE as u8, PLAYER_NONE as u8, LOCATION_GRAVE as u16, 0, POS_FACEUP as u8, false);
                }
                false
            }
            2 => {
                self.core.ss_tograve_set.clear();
                if self.group(g).is_empty() {
                    self.returns.set_i32(0, 0);
                    self.core.operated_set.clear();
                    return true;
                }
                let set = self.group(g).clone();
                let order = public_order(self, &set);
                let tp = self.infos.turn_player;
                let mut has_tp = false;
                let mut has_ntp = false;
                let mut once: [Vec<CardId>; 2] = [Vec::new(), Vec::new()];
                for &pcard in &order {
                    let c = self.card(pcard);
                    if c.summon.player == tp {
                        has_tp = true;
                    } else {
                        has_ntp = true;
                    }
                    once[c.summon.player as usize].push(pcard);
                }
                if has_tp {
                    self.set_spsummon_counter(tp, true, false);
                }
                if has_ntp {
                    self.set_spsummon_counter(1 - tp, true, false);
                }
                for p in 0..2u8 {
                    let cards = std::mem::take(&mut once[p as usize]);
                    self.spsummon_once_inc(p, &cards);
                }
                for &pcard in &order {
                    {
                        let c = self.card_mut(pcard);
                        c.set_status(STATUS_SPSUMMON_STEP, false);
                        c.set_status(STATUS_SPSUMMON_TURN, true);
                    }
                    if self.card(pcard).is_position(POS_FACEUP) != 0 {
                        pcard.enable_field_effect(self, true);
                    }
                }
                self.adjust_instant();
                false
            }
            3 => {
                self.new_message(MSG_SPSUMMONED);
                let set = self.group(g).clone();
                for pcard in public_order(self, &set) {
                    let c = self.card(pcard);
                    let sp = c.summon.player;
                    if !self.is_flag(DUEL_CANNOT_SUMMON_OATH_OLD) {
                        self.check_card_counter(pcard, ACTIVITY_SPSUMMON, sp as i32);
                    }
                    let c = self.card(pcard);
                    let (re, rp, pos) = (c.current.reason_effect, c.current.reason_player, c.current.position);
                    if pos & POS_FACEDOWN == 0 {
                        self.raise_single_event(pcard, None, EVENT_SPSUMMON_SUCCESS, re, 0, rp, sp, 0);
                    }
                    let c = self.card(pcard);
                    let summontype = c.summon.type_ & 0xff000000;
                    if summontype != 0 && !c.material_cards.is_empty() && !c.is_status(STATUS_FUTURE_FUSION) {
                        let matreason = if summontype == SUMMON_TYPE_FUSION {
                            REASON_FUSION
                        } else if summontype == SUMMON_TYPE_RITUAL {
                            REASON_RITUAL
                        } else if summontype == SUMMON_TYPE_XYZ {
                            REASON_XYZ
                        } else if summontype == SUMMON_TYPE_LINK {
                            REASON_LINK
                        } else {
                            0
                        };
                        let mats = c.material_cards.clone();
                        let all = self.group(g).clone();
                        for mcard in public_order(self, &mats) {
                            self.raise_single_event(mcard, Some(&all), EVENT_BE_MATERIAL, re, matreason, rp, sp, 0);
                        }
                        self.raise_event(mats, EVENT_BE_MATERIAL, reason_effect, matreason, reason_player, sp, 0);
                    }
                    self.card_mut(pcard).set_status(STATUS_FUTURE_FUSION, false);
                }
                self.process_single_event();
                self.process_instant_event();
                false
            }
            4 => {
                let set = self.group(g).clone();
                self.raise_event(set, EVENT_SPSUMMON_SUCCESS, reason_effect, 0, reason_player, PLAYER_NONE as u8, 0);
                self.process_instant_event();
                false
            }
            5 => {
                let set = self.group(g).clone();
                self.returns.set_i32(0, set.len() as i32);
                self.core.operated_set = set;
                true
            }
            _ => true,
        }
    }
}
