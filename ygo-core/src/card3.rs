//! Cards (`card.cpp` part 3: summon procedures and the "can this card ..." checks).

use crate::card::*;
use crate::constants::*;
use crate::duel::Duel;
use crate::types::*;
use crate::units::LocationReason;

impl CardId {
    #[inline]
    fn c3<'a>(&self, d: &'a Duel) -> &'a Card {
        d.card(*self)
    }

    pub fn fusion_check(self, d: &mut Duel, fusion_m: Option<&Group>, cg: Option<&Group>, chkf: u64) -> bool {
        for pe in self.c3(d).single_effect.range(EFFECT_FUSION_MATERIAL) {
            let Some(cond) = d.effect(pe).condition.clone() else { continue };
            d.add_param(Value::Effect(pe));
            d.add_param(fusion_m.map_or(Value::Nil, |g| Value::Group(g.clone())));
            d.add_param(cg.map_or(Value::Nil, |g| Value::Group(g.clone())));
            d.add_param(Value::Int(chkf as i64));
            d.add_param(Value::effect(d.core.reason_effect));
            let oreason = d.core.reason_effect;
            let op = d.core.reason_player;
            d.core.reason_effect = Some(pe);
            d.core.reason_player = pe.get_handler_player(d);
            let res = d.check_condition(Some(&cond), 5);
            d.core.reason_effect = oreason;
            d.core.reason_player = op;
            if res {
                return true;
            }
        }
        false
    }
    pub fn fusion_filter_valid(self, d: &mut Duel, fusion_m: Option<&Group>, cg: Option<&Group>, chkf: u64, eset: &mut Vec<EffectId>) {
        for pe in self.c3(d).single_effect.range(EFFECT_FUSION_MATERIAL) {
            let Some(cond) = d.effect(pe).condition.clone() else { continue };
            d.add_param(Value::Effect(pe));
            d.add_param(fusion_m.map_or(Value::Nil, |g| Value::Group(g.clone())));
            d.add_param(cg.map_or(Value::Nil, |g| Value::Group(g.clone())));
            d.add_param(Value::Int(chkf as i64));
            d.add_param(Value::effect(d.core.reason_effect));
            let oreason = d.core.reason_effect;
            let op = d.core.reason_player;
            d.core.reason_effect = Some(pe);
            d.core.reason_player = pe.get_handler_player(d);
            let res = d.check_condition(Some(&cond), 5);
            d.core.reason_effect = oreason;
            d.core.reason_player = op;
            if res {
                eset.push(pe);
            }
        }
    }
    pub fn check_fusion_substitute(self, d: &mut Duel, fcard: Option<CardId>) -> bool {
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_FUSION_SUBSTITUTE, &mut eset, true);
        if eset.is_empty() {
            return false;
        }
        for pe in eset {
            if !d.effect(pe).has_value() || pe.get_value_card(d, fcard, 0) != 0 {
                return true;
            }
        }
        false
    }
    pub fn is_not_tuner(self, d: &mut Duel, scard: Option<CardId>, playerid: u8) -> bool {
        if self.get_type(d, scard, SUMMON_TYPE_SYNCHRO as u64, playerid) & TYPE_TUNER == 0 {
            return true;
        }
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_NONTUNER, &mut eset, true);
        for pe in eset {
            if !d.effect(pe).has_value() || pe.get_value_card(d, scard, 0) != 0 {
                return true;
            }
        }
        false
    }
    pub fn check_unique_code(self, d: &mut Duel, pcard: CardId) -> bool {
        let uc = self.c3(d).unique_code;
        if uc == 0 {
            return false;
        }
        if uc == 1 {
            let f = self.c3(d).unique_function.clone();
            d.add_param(Value::Card(pcard));
            return d.get_function_value(f.as_ref(), 1) != 0;
        }
        let code1 = pcard.get_code(d);
        let code2 = pcard.get_another_code(d);
        code1 == uc || (code2 != 0 && code2 == uc)
    }
    pub fn get_unique_target(self, d: &mut Duel, cset: &mut CardSet, controler: u8, icard: Option<CardId>) {
        cset.clear();
        for p in 0..2u8 {
            if self.c3(d).unique_pos[p as usize] == 0 {
                continue;
            }
            let pl = (controler ^ p) as usize;
            let uloc = self.c3(d).unique_location;
            if uloc & LOCATION_MZONE != 0 {
                for pc in d.player[pl].list_mzone.clone().into_iter().flatten() {
                    if Some(pc) != icard
                        && d.card(pc).is_position(POS_FACEUP) != 0
                        && d.card(pc).get_status(STATUS_SPSUMMON_STEP) == 0
                        && self.check_unique_code(d, pc)
                    {
                        cset.insert(pc);
                    }
                }
            }
            if uloc & LOCATION_SZONE != 0 {
                for pc in d.player[pl].list_szone.clone().into_iter().flatten() {
                    if Some(pc) != icard && d.card(pc).is_position(POS_FACEUP) != 0 && self.check_unique_code(d, pc) {
                        cset.insert(pc);
                    }
                }
            }
        }
    }
    pub fn check_cost_condition(self, d: &mut Duel, ecode: u32, playerid: u8) -> bool {
        let mut eset = Vec::new();
        d.filter_player_effect(playerid, ecode, &mut eset, false);
        self.filter_effect(d, ecode, &mut eset, true);
        for pe in eset {
            let cost = d.effect(pe).cost.clone();
            d.add_param(Value::Effect(pe));
            d.add_param(Value::Card(self));
            d.add_param(Value::Int(playerid as i64));
            if !d.check_condition(cost.as_ref(), 3) {
                return false;
            }
        }
        true
    }
    pub fn check_cost_condition_sum(self, d: &mut Duel, ecode: u32, playerid: u8, sumtype: u32) -> bool {
        let mut eset = Vec::new();
        d.filter_player_effect(playerid, ecode, &mut eset, false);
        self.filter_effect(d, ecode, &mut eset, true);
        for pe in eset {
            let cost = d.effect(pe).cost.clone();
            d.add_param(Value::Effect(pe));
            d.add_param(Value::Card(self));
            d.add_param(Value::Int(playerid as i64));
            d.add_param(Value::Int(sumtype as i64));
            if !d.check_condition(cost.as_ref(), 4) {
                return false;
            }
        }
        true
    }
    /// check if this is a normal summonable card
    pub fn is_summonable_card(self, d: &mut Duel) -> bool {
        let t = self.c3(d).data.type_;
        if t & TYPE_MONSTER == 0
            || t & (TYPE_RITUAL | TYPE_SPSUMMON | TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ | TYPE_LINK | TYPE_TOKEN | TYPE_TRAPMONSTER) != 0
        {
            return false;
        }
        self.is_affected_by_effect(d, EFFECT_UNSUMMONABLE_CARD).is_none()
    }
    pub fn is_fusion_summonable_card(self, d: &mut Duel, mut summon_type: u32) -> bool {
        if self.c3(d).data.type_ & TYPE_FUSION == 0 {
            return false;
        }
        summon_type |= SUMMON_TYPE_FUSION;
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_SPSUMMON_CONDITION, &mut eset, true);
        for pe in eset {
            d.add_param(Value::effect(d.core.reason_effect));
            d.add_param(Value::Int(d.core.reason_player as i64));
            d.add_param(Value::Int(summon_type as i64));
            d.add_param(Value::Int(0));
            d.add_param(Value::Int(0));
            if !pe.check_value_condition(d, 5) {
                return false;
            }
        }
        true
    }
    fn extra_summon_retval(retval: &[i64], default_zone: u32) -> (u8, u32, u32) {
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
    /// -2: has EFFECT_LIMIT_SUMMON_PROC, none available; -1: has one, at least 1 available;
    /// 0: none, ordinary summon not available; 1: none, ordinary summon available
    pub fn filter_summon_procedure(self, d: &mut Duel, playerid: u8, peset: &mut Vec<EffectId>, ignore_count: bool, min_tribute: u8, zone: u32) -> i32 {
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_LIMIT_SUMMON_PROC, &mut eset, true);
        if !eset.is_empty() {
            for pe in eset {
                if self.check_summon_procedure(d, pe, playerid, ignore_count, min_tribute, zone) {
                    peset.push(pe);
                }
            }
            return if peset.is_empty() { -2 } else { -1 };
        }
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_SUMMON_PROC, &mut eset, true);
        for pe in eset {
            if self.check_summon_procedure(d, pe, playerid, ignore_count, min_tribute, zone) {
                peset.push(pe);
            }
        }
        // ordinary summon
        if !d.is_player_can_summon(SUMMON_TYPE_NORMAL, playerid, self, playerid) {
            return 0;
        }
        if d.check_unique_onfield(self, playerid, LOCATION_MZONE as u8, None).is_some() {
            return 0;
        }
        let rcount = self.get_summon_tribute_count(d);
        let mut min = (rcount & 0xffff) as u8;
        let mut max = ((rcount >> 16) & 0xffff) as u8;
        if !d.is_player_can_summon(SUMMON_TYPE_ADVANCE, playerid, self, playerid) {
            max = 0;
        }
        if min < min_tribute {
            min = min_tribute;
        }
        if max < min {
            return 0;
        }
        if !ignore_count && d.core.extra_summon[playerid as usize] == 0 && d.core.summon_count[playerid as usize] >= d.get_summon_count_limit(playerid) {
            let mut eset = Vec::new();
            self.filter_effect(d, EFFECT_EXTRA_SUMMON_COUNT, &mut eset, true);
            for pe in eset {
                let mut retval = Vec::new();
                pe.get_values_card(d, Some(self), 0, &mut retval);
                let (mut new_min, mut new_zone, releasable) = Self::extra_summon_retval(&retval, 0x1f);
                if new_min < min {
                    new_min = min;
                }
                new_zone &= zone;
                let ctrl = self.c3(d).current.controler;
                if d.check_tribute(self, new_min as i32, max as i32, None, ctrl, new_zone, releasable, 0x1) {
                    return 1;
                }
            }
        } else {
            let ctrl = self.c3(d).current.controler;
            return d.check_tribute(self, min as i32, max as i32, None, ctrl, zone, 0xff00ff, 0x1) as i32;
        }
        0
    }
    pub fn check_summon_procedure(self, d: &mut Duel, peffect: EffectId, playerid: u8, ignore_count: bool, min_tribute: u8, zone: u32) -> bool {
        if !peffect.check_count_limit(d, playerid) {
            return false;
        }
        let mut toplayer = playerid;
        let spsum_param = d.effect(peffect).is_flag(EFFECT_FLAG_SPSUM_PARAM);
        let o_range = d.effect(peffect).o_range;
        if spsum_param && o_range != 0 {
            toplayer = 1 - playerid;
        }
        let sumtype = peffect.get_value_card(d, Some(self), 0) as u32;
        if !d.is_player_can_summon(sumtype, playerid, self, toplayer) {
            return false;
        }
        if d.check_unique_onfield(self, toplayer, LOCATION_MZONE as u8, None).is_some() {
            return false;
        }
        if !ignore_count && d.core.extra_summon[playerid as usize] == 0 && d.core.summon_count[playerid as usize] >= d.get_summon_count_limit(playerid) {
            let mut eset = Vec::new();
            self.filter_effect(d, EFFECT_EXTRA_SUMMON_COUNT, &mut eset, true);
            for peff in eset {
                let mut retval = Vec::new();
                peff.get_values_card(d, Some(self), 0, &mut retval);
                let (mut new_min_tribute, mut new_zone, releasable) = Self::extra_summon_retval(&retval, 0x1f001f);
                if new_min_tribute < min_tribute {
                    new_min_tribute = min_tribute;
                }
                if spsum_param && o_range != 0 {
                    new_zone = (new_zone >> 16) | (new_zone & (0xffffu32 << 16));
                }
                new_zone &= zone;
                if self.is_summonable(d, peffect, new_min_tribute, new_zone, releasable, Some(peffect)) {
                    return true;
                }
            }
        } else {
            return self.is_summonable(d, peffect, min_tribute, zone, 0xff00ff, None);
        }
        false
    }
    pub fn filter_set_procedure(self, d: &mut Duel, playerid: u8, peset: &mut Vec<EffectId>, ignore_count: bool, min_tribute: u8, zone: u32) -> i32 {
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_LIMIT_SET_PROC, &mut eset, true);
        if !eset.is_empty() {
            for pe in eset {
                if self.check_set_procedure(d, pe, playerid, ignore_count, min_tribute, zone) {
                    peset.push(pe);
                }
            }
            return if peset.is_empty() { -2 } else { -1 };
        }
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_SET_PROC, &mut eset, true);
        for pe in eset {
            if self.check_set_procedure(d, pe, playerid, ignore_count, min_tribute, zone) {
                peset.push(pe);
            }
        }
        if !d.is_player_can_mset(SUMMON_TYPE_NORMAL, playerid, self, playerid) {
            return 0;
        }
        let rcount = self.get_set_tribute_count(d);
        let mut min = rcount & 0xffff;
        let mut max = (rcount >> 16) & 0xffff;
        if !d.is_player_can_mset(SUMMON_TYPE_ADVANCE, playerid, self, playerid) {
            max = 0;
        }
        if min < min_tribute as i32 {
            min = min_tribute as i32;
        }
        if max < min {
            return 0;
        }
        if !ignore_count && d.core.extra_summon[playerid as usize] == 0 && d.core.summon_count[playerid as usize] >= d.get_summon_count_limit(playerid) {
            let mut eset = Vec::new();
            self.filter_effect(d, EFFECT_EXTRA_SET_COUNT, &mut eset, true);
            for peff in eset {
                let mut retval = Vec::new();
                peff.get_values_card(d, Some(self), 0, &mut retval);
                let (mut new_min, mut new_zone, releasable) = Self::extra_summon_retval(&retval, 0x1f);
                if (new_min as i32) < min {
                    new_min = min as u8;
                }
                new_zone &= zone;
                let ctrl = self.c3(d).current.controler;
                if d.check_tribute(self, new_min as i32, max, None, ctrl, new_zone, releasable, POS_FACEDOWN_DEFENSE) {
                    return 1;
                }
            }
        } else {
            let ctrl = self.c3(d).current.controler;
            return d.check_tribute(self, min, max, None, ctrl, zone, 0xff00ff, POS_FACEDOWN_DEFENSE) as i32;
        }
        0
    }
    pub fn check_set_procedure(self, d: &mut Duel, peffect: EffectId, playerid: u8, ignore_count: bool, min_tribute: u8, zone: u32) -> bool {
        if !peffect.check_count_limit(d, playerid) {
            return false;
        }
        let mut toplayer = playerid;
        let spsum_param = d.effect(peffect).is_flag(EFFECT_FLAG_SPSUM_PARAM);
        let o_range = d.effect(peffect).o_range;
        if spsum_param && o_range != 0 {
            toplayer = 1 - playerid;
        }
        let sumtype = peffect.get_value_card(d, Some(self), 0) as u32;
        if !d.is_player_can_mset(sumtype, playerid, self, toplayer) {
            return false;
        }
        if !ignore_count && d.core.extra_summon[playerid as usize] == 0 && d.core.summon_count[playerid as usize] >= d.get_summon_count_limit(playerid) {
            let mut eset = Vec::new();
            self.filter_effect(d, EFFECT_EXTRA_SET_COUNT, &mut eset, true);
            for peff in eset {
                let mut retval = Vec::new();
                peff.get_values_card(d, Some(self), 0, &mut retval);
                let (mut new_min_tribute, mut new_zone, releasable) = Self::extra_summon_retval(&retval, 0x1f001f);
                if spsum_param && o_range != 0 {
                    new_zone = (new_zone >> 16) | (new_zone & (0xffffu32 << 16));
                }
                if new_min_tribute < min_tribute {
                    new_min_tribute = min_tribute;
                }
                new_zone &= zone;
                if self.is_summonable(d, peffect, new_min_tribute, new_zone, releasable, Some(peff)) {
                    return true;
                }
            }
        } else {
            return self.is_summonable(d, peffect, min_tribute, zone, 0xff00ff, None);
        }
        false
    }
    pub fn filter_spsummon_procedure(self, d: &mut Duel, playerid: u8, peset: &mut Vec<EffectId>, summon_type: u32) {
        for pe in self.c3(d).field_effect.range(EFFECT_SPSUMMON_PROC) {
            let (topos, toplayer);
            if d.effect(pe).is_flag(EFFECT_FLAG_SPSUM_PARAM) {
                topos = d.effect(pe).s_range as u8;
                toplayer = if d.effect(pe).o_range == 0 { playerid } else { 1 - playerid };
            } else {
                topos = POS_FACEUP as u8;
                toplayer = playerid;
            }
            if !pe.is_available(d) || !pe.check_count_limit(d, playerid) {
                continue;
            }
            if self.is_spsummonable(d, pe)
                && (topos as u32 & POS_FACEDOWN != 0 || d.check_unique_onfield(self, toplayer, LOCATION_MZONE as u8, None).is_none())
            {
                let sumeffect = d.core.reason_effect.unwrap_or(pe);
                let mut retval = Vec::new();
                pe.get_values_card(d, Some(self), 0, &mut retval);
                let sumtype = if !retval.is_empty() { retval[0] as u32 } else { 0 };
                let zone = if retval.len() > 1 { retval[1] as u32 } else { 0xff };
                let ignore_zone_check = retval.len() > 2 && retval[2] != 0;
                if zone != 0xff
                    && !ignore_zone_check
                    && d.get_useable_count(Some(self), toplayer, LOCATION_MZONE as u8, playerid, LocationReason::ToField, zone, None) <= 0
                {
                    continue;
                }
                if summon_type != 0 && summon_type != sumtype {
                    continue;
                }
                if !d.is_player_can_spsummon_ex(Some(sumeffect), sumtype, topos, playerid, toplayer, self, Some(pe)) {
                    continue;
                }
                peset.push(pe);
            }
        }
    }
    pub fn filter_spsummon_procedure_g(self, d: &mut Duel, playerid: u8, peset: &mut Vec<EffectId>) {
        for pe in self.c3(d).field_effect.range(EFFECT_SPSUMMON_PROC_G) {
            if !pe.is_available(d) || !pe.check_count_limit(d, playerid) {
                continue;
            }
            if self.c3(d).current.controler != playerid && !d.effect(pe).is_flag(EFFECT_FLAG_BOTH_SIDE) {
                continue;
            }
            let oreason = d.core.reason_effect;
            let op = d.core.reason_player;
            d.core.reason_effect = Some(pe);
            d.core.reason_player = self.c3(d).current.controler;
            let cond = d.effect(pe).condition.clone();
            d.add_param(Value::Effect(pe));
            d.add_param(Value::Card(self));
            if d.check_condition(cond.as_ref(), 2) {
                peset.push(pe);
            }
            d.core.reason_effect = oreason;
            d.core.reason_player = op;
        }
    }
    /// check the condition of sp_summon procedure peffect
    pub fn is_spsummonable(self, d: &mut Duel, peffect: EffectId) -> bool {
        let oreason = d.core.reason_effect;
        let op = d.core.reason_player;
        d.core.reason_effect = Some(peffect);
        d.core.reason_player = self.c3(d).current.controler;
        d.add_param(Value::Effect(peffect));
        d.add_param(Value::Card(self));
        d.add_param(Value::from_group(&d.core.must_use_mats.clone()));
        d.add_param(Value::from_group(&d.core.only_use_mats.clone()));
        let mut param_count = 4;
        if d.core.forced_summon_minc != 0 {
            d.add_param(Value::Int(d.core.forced_summon_minc as i64));
            d.add_param(Value::Int(d.core.forced_summon_maxc as i64));
            param_count += 2;
        }
        let cond = d.effect(peffect).condition.clone();
        let result = d.check_condition(cond.as_ref(), param_count);
        d.core.reason_effect = oreason;
        d.core.reason_player = op;
        result
    }
    /// check the condition of summon/set procedure peffect
    pub fn is_summonable(self, d: &mut Duel, peffect: EffectId, min_tribute: u8, zone: u32, releasable: u32, exeffect: Option<EffectId>) -> bool {
        let oreason = d.core.reason_effect;
        let op = d.core.reason_player;
        d.core.reason_effect = Some(peffect);
        d.core.reason_player = self.c3(d).current.controler;
        d.add_param(Value::Effect(peffect));
        d.add_param(Value::Card(self));
        d.add_param(Value::Int(min_tribute as i64));
        d.add_param(Value::Int(zone as i64));
        d.add_param(Value::Int(releasable as i64));
        d.add_param(Value::effect(exeffect));
        let cond = d.effect(peffect).condition.clone();
        let result = d.check_condition(cond.as_ref(), 6);
        d.core.reason_effect = oreason;
        d.core.reason_player = op;
        result
    }
    fn mark_summon_self(self, d: &mut Duel) {
        if let Some(re) = d.core.reason_effect {
            if re.get_handler_opt(d) == Some(self) {
                d.effect_mut(re).status |= EFFECT_STATUS_SUMMON_SELF as u16;
            }
        }
    }
    /// whether playerid can summon this (ordinary summon if there is no summon procedure)
    pub fn is_can_be_summoned(self, d: &mut Duel, playerid: u8, ignore_count: bool, peffect: Option<EffectId>, min_tribute: u8, zone: u32) -> bool {
        if !self.is_summonable_card(d) {
            return false;
        }
        self.mark_summon_self(d);
        if !ignore_count
            && (d.core.extra_summon[playerid as usize] != 0 || self.is_affected_by_effect(d, EFFECT_EXTRA_SUMMON_COUNT).is_none())
            && d.core.summon_count[playerid as usize] >= d.get_summon_count_limit(playerid)
        {
            return false;
        }
        if self.c3(d).is_status(STATUS_FORBIDDEN) {
            return false;
        }
        if !self.check_cost_condition(d, EFFECT_SUMMON_COST, playerid) {
            return false;
        }
        let loc = self.c3(d).current.location as u32;
        if loc == LOCATION_MZONE {
            if self.c3(d).is_position(POS_FACEDOWN) != 0
                || self.is_affected_by_effect(d, EFFECT_GEMINI_SUMMONABLE).is_none()
                || self.is_affected_by_effect(d, EFFECT_GEMINI_STATUS).is_some()
                || !d.is_player_can_summon(SUMMON_TYPE_GEMINI, playerid, self, playerid)
                || self.is_affected_by_effect(d, EFFECT_CANNOT_SUMMON).is_some()
            {
                return false;
            }
        } else if loc == LOCATION_HAND {
            if self.is_affected_by_effect(d, EFFECT_CANNOT_SUMMON).is_some() {
                return false;
            }
            let mut proc = Vec::new();
            let res = self.filter_summon_procedure(d, playerid, &mut proc, ignore_count, min_tribute, zone);
            if let Some(pe) = peffect {
                if res < 0 || !self.check_summon_procedure(d, pe, playerid, ignore_count, min_tribute, zone) {
                    return false;
                }
            } else if proc.is_empty() && (res == 0 || res == -2) {
                return false;
            }
        }
        true
    }
    /// the min/max number of tributes for an ordinary advance summon of this
    pub fn get_summon_tribute_count(self, d: &mut Duel) -> i32 {
        let (mut min, mut max);
        let mut minul = 0;
        let mut maxul = 0;
        let level = self.get_level(d) as i32;
        if level < 5 {
            return 0;
        } else if level < 7 {
            min = 1;
            max = 1;
        } else {
            min = 2;
            max = 2;
        }
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_DECREASE_TRIBUTE, &mut eset, true);
        for pe in eset {
            let dec = pe.get_value_card(d, Some(self), 0) as i32;
            if !d.effect(pe).is_flag(EFFECT_FLAG_COUNT_LIMIT) {
                if minul < (dec & 0xffff) {
                    minul = dec & 0xffff;
                }
                if maxul < (dec >> 16) {
                    maxul = dec >> 16;
                }
            } else if d.effect(pe).count_limit > 0 {
                min -= dec & 0xffff;
                max -= dec >> 16;
            }
        }
        min -= minul;
        max -= maxul;
        if min < 0 {
            min = 0;
        }
        if max < min {
            max = min;
        }
        min + (max << 16)
    }
    pub fn get_set_tribute_count(self, d: &mut Duel) -> i32 {
        let (mut min, mut max);
        let level = self.get_level(d) as i32;
        if level < 5 {
            return 0;
        } else if level < 7 {
            min = 1;
            max = 1;
        } else {
            min = 2;
            max = 2;
        }
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_DECREASE_TRIBUTE_SET, &mut eset, true);
        if let Some(&last) = eset.last() {
            let dec = last.get_value_card(d, Some(self), 0) as i32;
            min -= dec & 0xffff;
            max -= dec >> 16;
        }
        if min < 0 {
            min = 0;
        }
        if max < min {
            max = min;
        }
        min + (max << 16)
    }
    fn summoned_this_turn_by_controler(self, d: &Duel) -> bool {
        let c = self.c3(d);
        (c.is_status(STATUS_SUMMON_TURN) || c.is_status(STATUS_FLIP_SUMMON_TURN) || c.is_status(STATUS_SPSUMMON_TURN))
            && (c.summon.player == c.current.controler || !d.is_flag(DUEL_CAN_REPOS_IF_NON_SUMPLAYER))
    }
    pub fn is_can_be_flip_summoned(self, d: &mut Duel, playerid: u8) -> bool {
        let c = self.c3(d);
        if c.is_status(STATUS_FORM_CHANGED) {
            return false;
        }
        if self.summoned_this_turn_by_controler(d) {
            return false;
        }
        let c = self.c3(d);
        if c.announce_count > 0 || c.current.location as u32 != LOCATION_MZONE || c.current.position & POS_FACEDOWN == 0 {
            return false;
        }
        if d.check_unique_onfield(self, playerid, LOCATION_MZONE as u8, None).is_some() {
            return false;
        }
        if !d.is_player_can_flipsummon(playerid, self) {
            return false;
        }
        if self.c3(d).is_status(STATUS_FORBIDDEN) {
            return false;
        }
        if self.is_affected_by_effect(d, EFFECT_CANNOT_FLIP_SUMMON).is_some() || self.is_affected_by_effect(d, EFFECT_CANNOT_CHANGE_POSITION).is_some() {
            return false;
        }
        self.check_cost_condition(d, EFFECT_FLIPSUMMON_COST, playerid)
    }
    /// check if this can be sp_summoned by EFFECT_SPSUMMON_PROC: the number of procedures
    pub fn is_special_summonable(self, d: &mut Duel, playerid: u8, summon_type: u32) -> i32 {
        if self.c3(d).data.type_ & TYPE_MONSTER == 0 {
            return 0;
        }
        if self.is_affected_by_effect(d, EFFECT_CANNOT_SPECIAL_SUMMON).is_some() {
            return 0;
        }
        if self.c3(d).is_status(STATUS_FORBIDDEN) {
            return 0;
        }
        if !self.check_cost_condition_sum(d, EFFECT_SPSUMMON_COST, playerid, summon_type) {
            return 0;
        }
        let mut eset = Vec::new();
        self.filter_spsummon_procedure(d, playerid, &mut eset, summon_type);
        eset.len() as i32
    }
    #[allow(clippy::too_many_arguments)]
    pub fn is_can_be_special_summoned(
        self,
        d: &mut Duel,
        reason_effect: EffectId,
        mut sumtype: u32,
        mut sumpos: u8,
        sumplayer: u8,
        toplayer: u8,
        nocheck: bool,
        nolimit: bool,
        zone: u32,
    ) -> bool {
        if reason_effect.get_handler_opt(d) == Some(self) {
            d.effect_mut(reason_effect).status |= EFFECT_STATUS_SUMMON_SELF as u16;
        }
        let loc = self.c3(d).current.location as u32;
        if loc == LOCATION_MZONE {
            return false;
        }
        if !nolimit && self.is_affected_by_effect(d, EFFECT_REVIVE_LIMIT).is_some() && !self.c3(d).is_status(STATUS_PROC_COMPLETE) {
            if loc & (LOCATION_GRAVE | LOCATION_REMOVED | LOCATION_SZONE) != 0 || (!nocheck && loc & (LOCATION_DECK | LOCATION_HAND) != 0) {
                return false;
            }
            if self.c3(d).data.type_ & TYPE_PENDULUM != 0 && loc == LOCATION_EXTRA && self.c3(d).current.position & POS_FACEUP != 0 {
                return false;
            }
            if loc == LOCATION_OVERLAY {
                return false;
            }
        }
        if self.c3(d).data.type_ & TYPE_PENDULUM != 0
            && loc == LOCATION_EXTRA
            && self.c3(d).current.position & POS_FACEUP != 0
            && (sumtype == SUMMON_TYPE_FUSION || sumtype == SUMMON_TYPE_SYNCHRO || sumtype == SUMMON_TYPE_XYZ)
        {
            return false;
        }
        if sumpos as u32 & POS_FACEDOWN != 0 && d.is_player_affected_by_effect(sumplayer, EFFECT_DEVINE_LIGHT).is_some() {
            sumpos = (sumpos & POS_FACEUP as u8) | ((sumpos & POS_FACEDOWN as u8) >> 1);
        }
        if sumpos as u32 & POS_FACEUP != 0 && d.check_unique_onfield(self, toplayer, LOCATION_MZONE as u8, None).is_some() {
            sumpos &= !(POS_FACEUP as u8);
            if sumpos == 0 {
                return false;
            }
        }
        sumtype |= SUMMON_TYPE_SPECIAL;
        if (sumplayer == 0 || sumplayer == 1) && !d.is_player_can_spsummon_ex(Some(reason_effect), sumtype, sumpos, sumplayer, toplayer, self, None) {
            return false;
        }
        if self.is_affected_by_effect(d, EFFECT_CANNOT_SPECIAL_SUMMON).is_some() {
            return false;
        }
        if self.c3(d).is_status(STATUS_FORBIDDEN) {
            return false;
        }
        if zone != 0xff && d.get_useable_count(Some(self), toplayer, LOCATION_MZONE as u8, sumplayer, LocationReason::ToField, zone, None) <= 0 {
            return false;
        }
        if !self.check_cost_condition_sum(d, EFFECT_SPSUMMON_COST, sumplayer, sumtype) {
            return false;
        }
        if !nocheck {
            if self.c3(d).data.type_ & TYPE_MONSTER == 0 {
                return false;
            }
            let mut eset = Vec::new();
            self.filter_effect(d, EFFECT_SPSUMMON_CONDITION, &mut eset, true);
            for pe in eset {
                d.add_param(Value::Effect(reason_effect));
                d.add_param(Value::Int(sumplayer as i64));
                d.add_param(Value::Int(sumtype as i64));
                d.add_param(Value::Int(sumpos as i64));
                d.add_param(Value::Int(toplayer as i64));
                if !pe.check_value_condition(d, 5) {
                    return false;
                }
            }
        }
        true
    }
    pub fn is_setable_mzone(self, d: &mut Duel, playerid: u8, ignore_count: bool, peffect: Option<EffectId>, min_tribute: u8, zone: u32) -> bool {
        if !self.is_summonable_card(d) {
            return false;
        }
        self.mark_summon_self(d);
        if self.c3(d).current.location as u32 != LOCATION_HAND {
            return false;
        }
        if self.c3(d).is_status(STATUS_FORBIDDEN) {
            return false;
        }
        if self.is_affected_by_effect(d, EFFECT_CANNOT_MSET).is_some() {
            return false;
        }
        if !ignore_count
            && (d.core.extra_summon[playerid as usize] != 0 || self.is_affected_by_effect(d, EFFECT_EXTRA_SET_COUNT).is_none())
            && d.core.summon_count[playerid as usize] >= d.get_summon_count_limit(playerid)
        {
            return false;
        }
        if !self.check_cost_condition(d, EFFECT_MSET_COST, playerid) {
            return false;
        }
        let mut eset = Vec::new();
        let res = self.filter_set_procedure(d, playerid, &mut eset, ignore_count, min_tribute, zone);
        if let Some(pe) = peffect {
            if res < 0 || !self.check_set_procedure(d, pe, playerid, ignore_count, min_tribute, zone) {
                return false;
            }
        } else if eset.is_empty() && (res == 0 || res == -2) {
            return false;
        }
        true
    }
    pub fn is_setable_szone(self, d: &mut Duel, playerid: u8, ignore_fd: bool) -> bool {
        let dt = self.c3(d).data.type_;
        let ctrl = self.c3(d).current.controler;
        if dt & TYPE_FIELD == 0 && !ignore_fd && d.get_useable_count(Some(self), playerid, LOCATION_SZONE as u8, ctrl, LocationReason::ToField, 0xff, None) <= 0 {
            return false;
        }
        if dt & TYPE_MONSTER != 0 && self.is_affected_by_effect(d, EFFECT_MONSTER_SSET).is_none() {
            return false;
        }
        if self.c3(d).is_status(STATUS_FORBIDDEN) {
            return false;
        }
        if self.is_affected_by_effect(d, EFFECT_CANNOT_SSET).is_some() {
            return false;
        }
        if !d.is_player_can_sset(playerid, self) {
            return false;
        }
        self.check_cost_condition(d, EFFECT_SSET_COST, playerid)
    }
    pub fn is_can_be_disabled_by_effect(self, d: &mut Duel, reason_effect: Option<EffectId>, is_monster_effect: bool) -> bool {
        if is_monster_effect && self.c3(d).is_status(STATUS_DISABLED) {
            return false;
        }
        if !is_monster_effect && self.get_type(d, None, 0, 2) & TYPE_TRAPMONSTER == 0 && self.c3(d).is_status(STATUS_DISABLED) {
            return false;
        }
        if self.is_affected_by_effect(d, EFFECT_CANNOT_DISABLE).is_some() {
            return false;
        }
        self.is_affect_by_effect(d, reason_effect)
    }
    pub fn is_destructable(self, d: &Duel) -> bool {
        let c = self.c3(d);
        c.overlay_target.is_none() && c.current.location as u32 & (LOCATION_GRAVE | LOCATION_REMOVED) == 0
    }
    pub fn is_destructable_by_battle(self, d: &mut Duel, pcard: Option<CardId>) -> bool {
        self.is_affected_by_effect_t(d, EFFECT_INDESTRUCTABLE_BATTLE, pcard).is_none()
    }
    pub fn check_indestructable_by_effect(self, d: &mut Duel, peffect: Option<EffectId>, playerid: u8) -> Option<EffectId> {
        let pe = peffect?;
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_INDESTRUCTABLE_EFFECT, &mut eset, true);
        for peff in eset {
            d.add_param(Value::Effect(pe));
            d.add_param(Value::Int(playerid as i64));
            d.add_param(Value::Card(self));
            if peff.check_value_condition(d, 3) {
                return Some(peff);
            }
        }
        None
    }
    pub fn is_destructable_by_effect(self, d: &mut Duel, peffect: Option<EffectId>, playerid: u8) -> bool {
        if !self.is_affect_by_effect(d, peffect) {
            return false;
        }
        if self.check_indestructable_by_effect(d, peffect, playerid).is_some() {
            return false;
        }
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_INDESTRUCTABLE, &mut eset, true);
        for peff in eset {
            d.add_param(Value::effect(peffect));
            d.add_param(Value::Int(REASON_EFFECT as i64));
            d.add_param(Value::Int(playerid as i64));
            if peff.check_value_condition(d, 3) {
                return false;
            }
        }
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_INDESTRUCTABLE_COUNT, &mut eset, true);
        for peff in eset {
            if d.effect(peff).is_flag(EFFECT_FLAG_COUNT_LIMIT) {
                if d.effect(peff).count_limit == 0 {
                    continue;
                }
                d.add_param(Value::effect(peffect));
                d.add_param(Value::Int(REASON_EFFECT as i64));
                d.add_param(Value::Int(playerid as i64));
                if peff.check_value_condition(d, 3) {
                    return false;
                }
            } else {
                d.add_param(Value::effect(peffect));
                d.add_param(Value::Int(REASON_EFFECT as i64));
                d.add_param(Value::Int(playerid as i64));
                let ct = peff.get_value(d, 3) as i32;
                if ct != 0 {
                    let id = d.effect(peff).id;
                    let c = d.card_mut(self);
                    let cur = match c.indestructable_effects.iter().position(|x| x.0 == id) {
                        Some(i) => c.indestructable_effects[i].1,
                        None => {
                            let pos = c.indestructable_effects.partition_point(|x| x.0 < id);
                            c.indestructable_effects.insert(pos, (id, 0));
                            0
                        }
                    };
                    if cur + 1 <= ct {
                        return false;
                    }
                }
            }
        }
        true
    }
    pub fn is_removeable(self, d: &mut Duel, playerid: u8, pos: u8, reason: u32) -> bool {
        if !d.is_player_can_remove(playerid, self, reason) {
            return false;
        }
        if self.is_affected_by_effect(d, EFFECT_CANNOT_REMOVE).is_some() {
            return false;
        }
        if self.c3(d).data.type_ & TYPE_TOKEN != 0 && pos as u32 & POS_FACEDOWN != 0 {
            return false;
        }
        true
    }
    /// The destination after redirects when sent to `dest` as a cost.
    fn cost_destination(self, d: &mut Duel, mut dest: u32) -> (u32, bool) {
        let mut redirchk = false;
        let op_param = self.c3(d).sendto_param;
        d.card_mut(self).sendto_param.location = dest as u8;
        let mut redirect = 0;
        if self.c3(d).current.location as u32 & LOCATION_ONFIELD != 0 {
            redirect = self.leave_field_redirect(d, REASON_COST) & 0xffff;
        }
        if redirect != 0 {
            redirchk = true;
            dest = redirect;
        }
        redirect = self.destination_redirect(d, dest as u8, REASON_COST) & 0xffff;
        if redirect != 0 {
            redirchk = true;
            dest = redirect;
        }
        d.card_mut(self).sendto_param = op_param;
        (dest, redirchk)
    }
    pub fn is_removeable_as_cost(self, d: &mut Duel, playerid: u8, pos: u8) -> bool {
        if self.c3(d).current.location as u32 == LOCATION_REMOVED {
            return false;
        }
        if self.is_affected_by_effect(d, EFFECT_CANNOT_USE_AS_COST).is_some() {
            return false;
        }
        if !self.is_removeable(d, playerid, pos, REASON_COST) {
            return false;
        }
        let (dest, redirchk) = self.cost_destination(d, LOCATION_REMOVED);
        !(dest != LOCATION_REMOVED || (redirchk && pos as u32 & POS_FACEDOWN != 0))
    }
    pub fn is_releasable_by_summon(self, d: &mut Duel, playerid: u8, pcard: CardId) -> bool {
        let c = self.c3(d);
        if c.is_status(STATUS_SUMMONING) || c.overlay_target.is_some() || c.current.location as u32 & (LOCATION_GRAVE | LOCATION_REMOVED) != 0 {
            return false;
        }
        if !d.is_player_can_release(playerid, self, REASON_SUMMON) {
            return false;
        }
        if self.is_affected_by_effect_t(d, EFFECT_UNRELEASABLE_SUM, Some(pcard)).is_some() {
            return false;
        }
        if pcard.is_affected_by_effect_t(d, EFFECT_TRIBUTE_LIMIT, Some(self)).is_some() {
            return false;
        }
        true
    }
    pub fn is_releasable_by_nonsummon(self, d: &mut Duel, playerid: u8, reason: u32) -> bool {
        let c = self.c3(d);
        if c.is_status(STATUS_SUMMONING) || c.overlay_target.is_some() || c.current.location as u32 & (LOCATION_GRAVE | LOCATION_REMOVED) != 0 {
            return false;
        }
        if c.current.location as u32 == LOCATION_HAND && c.data.type_ & (TYPE_SPELL | TYPE_TRAP) != 0 {
            return false;
        }
        if !d.is_player_can_release(playerid, self, reason) {
            return false;
        }
        self.is_affected_by_effect(d, EFFECT_UNRELEASABLE_NONSUM).is_none()
    }
    pub fn is_releasable_by_effect(self, d: &mut Duel, playerid: u8, peffect: Option<EffectId>) -> bool {
        let Some(pe) = peffect else { return true };
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_UNRELEASABLE_EFFECT, &mut eset, true);
        for peff in eset {
            d.add_param(Value::Effect(pe));
            d.add_param(Value::Int(playerid as i64));
            d.add_param(Value::Card(self));
            if peff.check_value_condition(d, 3) {
                return false;
            }
        }
        true
    }
    pub fn is_capable_send_to_grave(self, d: &mut Duel, playerid: u8, reason: u32) -> bool {
        if self.is_affected_by_effect(d, EFFECT_CANNOT_TO_GRAVE).is_some() {
            return false;
        }
        d.is_player_can_send_to_grave(playerid, self, reason)
    }
    pub fn is_capable_send_to_hand(self, d: &mut Duel, playerid: u8) -> bool {
        if self.c3(d).is_status(STATUS_LEAVE_CONFIRMED) {
            return false;
        }
        if self.c3(d).current.location as u32 == LOCATION_EXTRA && self.is_extra_deck_monster(d) {
            return false;
        }
        if self.is_affected_by_effect(d, EFFECT_CANNOT_TO_HAND).is_some() {
            return false;
        }
        if self.is_extra_deck_monster(d) && !self.is_capable_send_to_deck(d, playerid) {
            return false;
        }
        d.is_player_can_send_to_hand(playerid, self)
    }
    pub fn is_capable_send_to_deck(self, d: &mut Duel, playerid: u8) -> bool {
        if self.c3(d).is_status(STATUS_LEAVE_CONFIRMED) {
            return false;
        }
        if self.c3(d).current.location as u32 == LOCATION_EXTRA && self.is_extra_deck_monster(d) {
            return false;
        }
        if self.is_affected_by_effect(d, EFFECT_CANNOT_TO_DECK).is_some() {
            return false;
        }
        d.is_player_can_send_to_deck(playerid, self)
    }
    pub fn is_capable_send_to_extra(self, d: &mut Duel, playerid: u8) -> bool {
        if !self.is_extra_deck_monster(d) && self.c3(d).data.type_ & TYPE_PENDULUM == 0 {
            return false;
        }
        if self.is_affected_by_effect(d, EFFECT_CANNOT_TO_DECK).is_some() {
            return false;
        }
        d.is_player_can_send_to_deck(playerid, self)
    }
    pub fn is_capable_cost_to_grave(self, d: &mut Duel, playerid: u8) -> bool {
        let c = self.c3(d);
        if c.data.type_ & TYPE_TOKEN != 0 {
            return false;
        }
        if c.data.type_ & TYPE_PENDULUM != 0 && c.current.location as u32 & LOCATION_ONFIELD != 0 && self.is_capable_send_to_extra(d, playerid) {
            return false;
        }
        if self.c3(d).current.location as u32 == LOCATION_GRAVE {
            return false;
        }
        if self.is_affected_by_effect(d, EFFECT_CANNOT_USE_AS_COST).is_some() || self.is_affected_by_effect(d, EFFECT_CANNOT_TO_GRAVE_AS_COST).is_some() {
            return false;
        }
        if !self.is_capable_send_to_grave(d, playerid, REASON_COST) {
            return false;
        }
        self.cost_destination(d, LOCATION_GRAVE).0 == LOCATION_GRAVE
    }
    pub fn is_capable_cost_to_hand(self, d: &mut Duel, playerid: u8) -> bool {
        if self.c3(d).data.type_ & TYPE_TOKEN != 0 || self.is_extra_deck_monster(d) {
            return false;
        }
        if self.c3(d).current.location as u32 == LOCATION_HAND {
            return false;
        }
        if self.is_affected_by_effect(d, EFFECT_CANNOT_USE_AS_COST).is_some() {
            return false;
        }
        if !self.is_capable_send_to_hand(d, playerid) {
            return false;
        }
        self.cost_destination(d, LOCATION_HAND).0 == LOCATION_HAND
    }
    pub fn is_capable_cost_to_deck(self, d: &mut Duel, playerid: u8) -> bool {
        if self.c3(d).data.type_ & TYPE_TOKEN != 0 || self.is_extra_deck_monster(d) {
            return false;
        }
        if self.c3(d).current.location as u32 == LOCATION_DECK {
            return false;
        }
        if self.is_affected_by_effect(d, EFFECT_CANNOT_USE_AS_COST).is_some() {
            return false;
        }
        if !self.is_capable_send_to_deck(d, playerid) {
            return false;
        }
        self.cost_destination(d, LOCATION_DECK).0 == LOCATION_DECK
    }
    pub fn is_capable_cost_to_extra(self, d: &mut Duel, playerid: u8) -> bool {
        if !self.is_extra_deck_monster(d) {
            return false;
        }
        if self.c3(d).current.location as u32 == LOCATION_EXTRA {
            return false;
        }
        if self.is_affected_by_effect(d, EFFECT_CANNOT_USE_AS_COST).is_some() {
            return false;
        }
        if !self.is_capable_send_to_deck(d, playerid) {
            return false;
        }
        self.cost_destination(d, LOCATION_EXTRA).0 == LOCATION_EXTRA
    }
    pub fn is_capable_attack(self, d: &mut Duel) -> bool {
        if self.c3(d).is_position(POS_FACEUP_ATTACK) == 0
            && !(self.c3(d).is_position(POS_FACEUP_DEFENSE) != 0 && self.is_affected_by_effect(d, EFFECT_DEFENSE_ATTACK).is_some())
        {
            return false;
        }
        if self.is_affected_by_effect(d, EFFECT_FORBIDDEN).is_some() {
            return false;
        }
        if self.is_affected_by_effect(d, EFFECT_CANNOT_ATTACK).is_some() && self.is_affected_by_effect(d, EFFECT_UNSTOPPABLE_ATTACK).is_none() {
            return false;
        }
        if self.is_affected_by_effect(d, EFFECT_ATTACK_DISABLED).is_some() && self.is_affected_by_effect(d, EFFECT_UNSTOPPABLE_ATTACK).is_none() {
            return false;
        }
        let tp = d.infos.turn_player;
        d.is_player_affected_by_effect(tp, EFFECT_SKIP_BP).is_none()
    }
    pub fn is_capable_attack_announce(self, d: &mut Duel, playerid: u8) -> bool {
        if !self.is_capable_attack(d) {
            return false;
        }
        if self.is_affected_by_effect(d, EFFECT_CANNOT_ATTACK_ANNOUNCE).is_some() && self.is_affected_by_effect(d, EFFECT_UNSTOPPABLE_ATTACK).is_none() {
            return false;
        }
        self.check_cost_condition(d, EFFECT_ATTACK_COST, playerid)
    }
    pub fn is_capable_change_position(self, d: &mut Duel, playerid: u8) -> bool {
        let c = self.c3(d);
        if c.is_status(STATUS_FORM_CHANGED) {
            return false;
        }
        if self.summoned_this_turn_by_controler(d) {
            return false;
        }
        let c = self.c3(d);
        if c.data.type_ & TYPE_LINK != 0 && c.data.type_ & TYPE_MONSTER != 0 {
            return false;
        }
        if c.announce_count > 0 || c.is_status(STATUS_FORBIDDEN) {
            return false;
        }
        if self.is_affected_by_effect(d, EFFECT_CANNOT_CHANGE_POSITION).is_some() {
            return false;
        }
        d.is_player_affected_by_effect(playerid, EFFECT_CANNOT_CHANGE_POSITION).is_none()
    }
    pub fn is_capable_change_position_by_effect(self, d: &Duel, _playerid: u8) -> bool {
        let t = self.c3(d).data.type_;
        !(t & TYPE_LINK != 0 && t & TYPE_MONSTER != 0)
    }
    pub fn is_capable_turn_set(self, d: &mut Duel, playerid: u8) -> bool {
        let t = self.c3(d).data.type_;
        if t & TYPE_TOKEN != 0 || (t & TYPE_LINK != 0 && t & TYPE_MONSTER != 0) {
            return false;
        }
        if self.c3(d).is_position(POS_FACEDOWN) != 0 {
            return false;
        }
        if self.is_affected_by_effect(d, EFFECT_CANNOT_TURN_SET).is_some() {
            return false;
        }
        d.is_player_affected_by_effect(playerid, EFFECT_CANNOT_TURN_SET).is_none()
    }
    pub fn is_capable_change_control(self, d: &mut Duel) -> bool {
        self.is_affected_by_effect(d, EFFECT_CANNOT_CHANGE_CONTROL).is_none()
    }
    pub fn is_control_can_be_changed(self, d: &mut Duel, ignore_mzone: bool, zone: u32) -> bool {
        let c = self.c3(d);
        if c.current.controler == PLAYER_NONE as u8 || c.current.location as u32 != LOCATION_MZONE {
            return false;
        }
        let ctrl = c.current.controler;
        if !ignore_mzone && d.get_useable_count(Some(self), 1 - ctrl, LOCATION_MZONE as u8, ctrl, LocationReason::Control, zone, None) <= 0 {
            return false;
        }
        if !d.is_flag(DUEL_TRAP_MONSTERS_NOT_USE_ZONE)
            && self.get_type(d, None, 0, 2) & TYPE_TRAPMONSTER != 0
            && d.get_useable_count(Some(self), 1 - ctrl, LOCATION_SZONE as u8, ctrl, LocationReason::Control, 0xff, None) <= 0
        {
            return false;
        }
        self.is_affected_by_effect(d, EFFECT_CANNOT_CHANGE_CONTROL).is_none()
    }
    pub fn is_capable_be_battle_target(self, d: &mut Duel, pcard: CardId) -> bool {
        if self.is_affected_by_effect_t(d, EFFECT_CANNOT_BE_BATTLE_TARGET, Some(pcard)).is_some() {
            return false;
        }
        if pcard.is_affected_by_effect_t(d, EFFECT_CANNOT_SELECT_BATTLE_TARGET, Some(self)).is_some() {
            return false;
        }
        self.is_affected_by_effect_t(d, EFFECT_IGNORE_BATTLE_TARGET, Some(pcard)).is_none()
    }
    pub fn is_capable_be_effect_target(self, d: &mut Duel, peffect: EffectId, playerid: u8) -> bool {
        let c = self.c3(d);
        if c.is_status(STATUS_SUMMONING) || c.is_status(STATUS_BATTLE_DESTROYED) {
            return false;
        }
        if c.current.location as u32 & (LOCATION_DECK | LOCATION_EXTRA | LOCATION_HAND) != 0 {
            return false;
        }
        if c.data.type_ & TYPE_TOKEN != 0 && c.current.location as u32 & LOCATION_ONFIELD == 0 {
            return false;
        }
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_CANNOT_BE_EFFECT_TARGET, &mut eset, true);
        for peff in eset {
            d.add_param(Value::Int(playerid as i64));
            if peff.get_value_effect(d, Some(peffect), 1) != 0 {
                return false;
            }
        }
        let mut eset = Vec::new();
        let h = peffect.get_handler(d);
        h.filter_effect(d, EFFECT_CANNOT_SELECT_EFFECT_TARGET, &mut eset, true);
        for peff in eset {
            d.add_param(Value::Card(self));
            if peff.get_value_effect(d, Some(peffect), 1) != 0 {
                return false;
            }
        }
        true
    }
    fn cannot_be_material(self, d: &mut Duel, scard: Option<CardId>, sumtype: u64, playerid: u8) -> bool {
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_CANNOT_BE_MATERIAL, &mut eset, true);
        for pe in eset {
            d.add_param(Value::Int(sumtype as i64));
            d.add_param(Value::Int(playerid as i64));
            if pe.get_value_card(d, scard, 2) != 0 {
                return true;
            }
        }
        false
    }
    pub fn is_can_be_fusion_material(self, d: &mut Duel, fcard: Option<CardId>, summon_type: u64, playerid: u8) -> bool {
        if Some(self) == fcard {
            return false;
        }
        if self.c3(d).is_status(STATUS_FORBIDDEN) {
            return false;
        }
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_CANNOT_BE_FUSION_MATERIAL, &mut eset, true);
        for pe in eset {
            d.add_param(Value::Int(summon_type as i64));
            if pe.get_value_card(d, fcard, 1) != 0 {
                return false;
            }
        }
        if self.cannot_be_material(d, fcard, summon_type, playerid) {
            return false;
        }
        if fcard.is_some() {
            let mut eset = Vec::new();
            self.filter_effect(d, EFFECT_EXTRA_FUSION_MATERIAL, &mut eset, true);
            if !eset.is_empty() {
                for pe in eset {
                    if pe.get_value_card(d, fcard, 0) != 0 {
                        return true;
                    }
                }
                return false;
            }
        } else {
            let c = self.c3(d);
            if c.current.location as u32 & LOCATION_ONFIELD == 0
                && c.data.type_ & TYPE_MONSTER == 0
                && self.is_affected_by_effect(d, EFFECT_EXTRA_FUSION_MATERIAL).is_none()
            {
                return false;
            }
        }
        true
    }
    pub fn is_can_be_synchro_material(self, d: &mut Duel, scard: Option<CardId>, playerid: u8) -> bool {
        if Some(self) == scard {
            return false;
        }
        let t = self.c3(d).data.type_;
        if t & TYPE_XYZ != 0 && !(self.is_affected_by_effect(d, EFFECT_RANK_LEVEL).is_some() || self.is_affected_by_effect(d, EFFECT_RANK_LEVEL_S).is_some()) {
            return false;
        }
        if t & TYPE_LINK != 0 && self.is_affected_by_effect(d, EFFECT_SYNCHRO_LEVEL).is_none() {
            return false;
        }
        if self.get_type(d, scard, SUMMON_TYPE_SYNCHRO as u64, playerid) & TYPE_MONSTER == 0 {
            return false;
        }
        if let Some(sc) = scard {
            let c = self.c3(d);
            if c.current.location as u32 == LOCATION_MZONE
                && c.current.controler != d.card(sc).current.controler
                && self.is_affected_by_effect(d, EFFECT_SYNCHRO_MATERIAL).is_none()
            {
                return false;
            }
        }
        if self.c3(d).is_status(STATUS_FORBIDDEN) {
            return false;
        }
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_CANNOT_BE_SYNCHRO_MATERIAL, &mut eset, true);
        for pe in eset {
            if pe.get_value_card(d, scard, 0) != 0 {
                return false;
            }
        }
        !self.cannot_be_material(d, scard, SUMMON_TYPE_SYNCHRO as u64, playerid)
    }
    pub fn is_can_be_ritual_material(self, d: &mut Duel, scard: Option<CardId>, playerid: u8) -> bool {
        if Some(self) == scard {
            return false;
        }
        if self.get_type(d, None, 0, 2) & TYPE_MONSTER == 0 {
            return false;
        }
        if self.cannot_be_material(d, scard, SUMMON_TYPE_RITUAL as u64, playerid) {
            return false;
        }
        if self.c3(d).current.location as u32 == LOCATION_GRAVE {
            let mut eset = Vec::new();
            self.filter_effect(d, EFFECT_EXTRA_RITUAL_MATERIAL, &mut eset, true);
            for pe in eset {
                if pe.get_value_card(d, scard, 0) != 0 {
                    return true;
                }
            }
            return false;
        }
        true
    }
    pub fn is_can_be_xyz_material(self, d: &mut Duel, scard: Option<CardId>, playerid: u8, reason: u32) -> bool {
        if Some(self) == scard {
            return false;
        }
        let c = self.c3(d);
        if c.data.type_ & TYPE_TOKEN != 0 {
            return false;
        }
        if (c.current.location as u32 & LOCATION_ONFIELD == 0 || reason & REASON_MATERIAL != 0) && c.is_status(STATUS_FORBIDDEN) {
            return false;
        }
        if reason & REASON_EFFECT != 0 {
            return self.c3(d).current.controler == playerid || self.is_capable_change_control(d);
        }
        if self.get_type(d, scard, SUMMON_TYPE_XYZ as u64, playerid) & TYPE_MONSTER == 0 {
            return false;
        }
        if reason & REASON_MATERIAL != 0 {
            let mut eset = Vec::new();
            self.filter_effect(d, EFFECT_CANNOT_BE_XYZ_MATERIAL, &mut eset, true);
            for pe in eset {
                if pe.get_value_card(d, scard, 0) != 0 {
                    return false;
                }
            }
            if self.cannot_be_material(d, scard, SUMMON_TYPE_XYZ as u64, playerid) {
                return false;
            }
        }
        true
    }
    pub fn is_can_be_link_material(self, d: &mut Duel, scard: Option<CardId>, playerid: u8) -> bool {
        if Some(self) == scard {
            return false;
        }
        if self.get_type(d, scard, SUMMON_TYPE_LINK as u64, playerid) & TYPE_MONSTER == 0 {
            return false;
        }
        if self.c3(d).is_status(STATUS_FORBIDDEN) {
            return false;
        }
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_CANNOT_BE_LINK_MATERIAL, &mut eset, true);
        for pe in eset {
            if pe.get_value_card(d, scard, 0) != 0 {
                return false;
            }
        }
        !self.cannot_be_material(d, scard, SUMMON_TYPE_LINK as u64, playerid)
    }
    pub fn is_can_be_material(self, d: &mut Duel, scard: Option<CardId>, sumtype: u64, playerid: u8) -> bool {
        if Some(self) == scard {
            return false;
        }
        if sumtype & SUMMON_TYPE_FUSION as u64 != 0 {
            return self.is_can_be_fusion_material(d, scard, sumtype, playerid);
        }
        if sumtype & SUMMON_TYPE_SYNCHRO as u64 != 0 {
            return self.is_can_be_synchro_material(d, scard, playerid);
        }
        if sumtype & SUMMON_TYPE_RITUAL as u64 != 0 {
            return self.is_can_be_ritual_material(d, scard, playerid);
        }
        if sumtype & SUMMON_TYPE_XYZ as u64 != 0 {
            return self.is_can_be_xyz_material(d, scard, playerid, REASON_XYZ | REASON_MATERIAL);
        }
        if sumtype & SUMMON_TYPE_LINK as u64 != 0 {
            return self.is_can_be_link_material(d, scard, playerid);
        }
        if self.c3(d).is_status(STATUS_FORBIDDEN) {
            return false;
        }
        !self.cannot_be_material(d, scard, sumtype, playerid)
    }
    pub fn recreate(self, d: &mut Duel, code: u32) -> bool {
        if code == 0 {
            return false;
        }
        let data = d.read_card(code);
        d.card_mut(self).data = data;
        true
    }
}
