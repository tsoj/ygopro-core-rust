//! The field (`field.cpp` part 3: tributes and releases, counters, attack targets, the
//! "can this player ..." checks, chains).

use crate::card::public_order;
use crate::constants::*;
use crate::duel::{ActionCounter, Chain, Duel, Event};
use crate::field2::CardFilter;
use crate::types::*;
use crate::units::LocationReason;

/// Where `field::get_chain` found the chain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChainRef {
    Continuous(usize),
    Current(usize),
}

impl Duel {
    pub fn get_counter_map(&mut self, counter_type: u32) -> &mut ActionCounter {
        match counter_type {
            ACTIVITY_SUMMON => &mut self.core.summon_counter,
            ACTIVITY_NORMALSUMMON => &mut self.core.normalsummon_counter,
            ACTIVITY_SPSUMMON => &mut self.core.spsummon_counter,
            ACTIVITY_FLIPSUMMON => &mut self.core.flipsummon_counter,
            ACTIVITY_ATTACK => &mut self.core.attack_counter,
            ACTIVITY_CHAIN => &mut self.core.chain_counter,
            _ => unreachable!(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_release_list(
        &mut self,
        playerid: u8,
        mut release_list: Option<&mut CardSet>,
        mut ex_list: Option<&mut CardSet>,
        mut ex_list_oneof: Option<&mut CardSet>,
        use_hand: bool,
        fun: CardFilter,
        exc: Option<CardId>,
        exg: Option<&CardSet>,
        use_oppo: bool,
        reason: u32,
    ) -> i32 {
        let p = playerid as usize;
        let mut rcount = 0i32;
        let base_ok = |_d: &mut Duel, c: CardId| Some(c) != exc && !exg.is_some_and(|g| g.contains(c));
        for c in self.player[p].list_mzone.clone().into_iter().flatten() {
            if base_ok(self, c) && c.is_releasable_by_nonsummon(self, playerid, reason) && fun.is_none_or(|f| f(self, c)) {
                if let Some(l) = release_list.as_mut() {
                    l.insert(c);
                }
                self.card_mut(c).release_param = 1;
                rcount += 1;
            }
        }
        if use_hand {
            for c in self.player[p].list_hand.clone() {
                if base_ok(self, c) && c.is_releasable_by_nonsummon(self, playerid, reason) && fun.is_none_or(|f| f(self, c)) {
                    if let Some(l) = release_list.as_mut() {
                        l.insert(c);
                    }
                    self.card_mut(c).release_param = 1;
                    rcount += 1;
                }
            }
        }
        let mut ex_oneof_max = 0;
        for c in self.player[1 - p].list_mzone.clone().into_iter().flatten() {
            if !(base_ok(self, c)
                && (self.card(c).is_position(POS_FACEUP) != 0 || fun.is_none())
                && c.is_releasable_by_nonsummon(self, playerid, reason)
                && fun.is_none_or(|f| f(self, c)))
            {
                continue;
            }
            if use_oppo {
                if let Some(l) = release_list.as_mut() {
                    l.insert(c);
                }
                self.card_mut(c).release_param = 1;
                rcount += 1;
            } else {
                self.card_mut(c).release_param = 1;
                if c.is_affected_by_effect(self, EFFECT_EXTRA_RELEASE).is_some() {
                    if let Some(l) = ex_list.as_mut() {
                        l.insert(c);
                    }
                    rcount += 1;
                } else {
                    let Some(pe) = c.is_affected_by_effect(self, EFFECT_EXTRA_RELEASE_NONSUM) else { continue };
                    if self.effect(pe).is_flag(EFFECT_FLAG_COUNT_LIMIT) && self.effect(pe).count_limit == 0 {
                        continue;
                    }
                    self.add_param(Value::effect(self.core.reason_effect));
                    self.add_param(Value::Int(reason as i64));
                    self.add_param(Value::Int(self.core.reason_player as i64));
                    if !pe.check_value_condition(self, 3) {
                        continue;
                    }
                    if let Some(l) = ex_list_oneof.as_mut() {
                        l.insert(c);
                    }
                    ex_oneof_max = 1;
                }
            }
        }
        rcount + ex_oneof_max
    }
    #[allow(clippy::too_many_arguments)]
    pub fn check_release_list(
        &mut self,
        playerid: u8,
        min: i32,
        _max: i32,
        use_hand: bool,
        fun: CardFilter,
        exc: Option<CardId>,
        exg: Option<&CardSet>,
        check_field: bool,
        to_player: u8,
        mut zone: u8,
        to_check: Option<CardId>,
        use_oppo: bool,
        reason: u32,
    ) -> bool {
        let mut relcard = CardSet::new();
        let mut relcard_ex = CardSet::new();
        let mut relcard_oneof = CardSet::new();
        let mut has_to_choose_one = false;
        let mut must_choose_one = CardSet::new();
        self.get_release_list(playerid, Some(&mut relcard), Some(&mut relcard_ex), Some(&mut relcard_oneof), use_hand, fun, exc, exg, use_oppo, reason);
        relcard.extend(relcard_ex.iter());
        if check_field {
            zone &= (0x1f & self.get_forced_zones(to_check, playerid, LOCATION_MZONE as u8, to_player as u32, LocationReason::ToField)) as u8;
            let ct = self.get_useable_count(to_check, playerid, LOCATION_MZONE as u8, to_player, LocationReason::ToField, zone as u32, None);
            if ct < min {
                has_to_choose_one = true;
                for c in public_order(self, &relcard) {
                    let cc = self.card(c);
                    if cc.current.location as u32 == LOCATION_MZONE && cc.current.controler == playerid && (zone >> cc.current.sequence) & 1 != 0 {
                        must_choose_one.insert(c);
                    }
                }
            }
        }
        if has_to_choose_one && must_choose_one.is_empty() {
            return false;
        }
        let mut has_oneof = false;
        for c in self.core.must_select_cards.clone() {
            if relcard.remove(c) {
            } else if !has_oneof && relcard_oneof.contains(c) {
                has_oneof = true;
            } else {
                return false;
            }
        }
        let mut rcount = relcard.len() as i32;
        if !has_oneof && !relcard_oneof.is_empty() {
            rcount += 1;
        }
        rcount >= min
    }
    fn set_tribute_param(&mut self, c: CardId, target: CardId) {
        let v = if c.is_affected_by_effect_t(self, EFFECT_TRIPLE_TRIBUTE, Some(target)).is_some() {
            3
        } else if c.is_affected_by_effect_t(self, EFFECT_DOUBLE_TRIBUTE, Some(target)).is_some() {
            2
        } else {
            1
        };
        self.card_mut(c).release_param = v;
    }
    /// return: the max release count of mg or all monsters on field
    #[allow(clippy::too_many_arguments)]
    pub fn get_summon_release_list(
        &mut self,
        target: CardId,
        mut release_list: Option<&mut CardSet>,
        mut ex_list: Option<&mut CardSet>,
        mut ex_list_oneof: Option<&mut CardSet>,
        mg: Option<&CardSet>,
        ex: bool,
        releasable: u32,
        pos: u32,
    ) -> i32 {
        let p = self.card(target).current.controler as usize;
        let mut ex_tribute = CardSet::new();
        let mut eset = Vec::new();
        target.filter_effect(self, EFFECT_ADD_EXTRA_TRIBUTE, &mut eset, true);
        for peff in eset {
            if peff.get_value(self, 0) as u32 & pos != 0 {
                self.filter_inrange_cards(peff, &mut ex_tribute);
            }
        }
        let mut rcount: u32 = 0;
        for c in self.player[p].list_mzone.clone().into_iter().flatten() {
            if (releasable >> self.card(c).current.sequence) & 1 != 0 && c.is_releasable_by_summon(self, p as u8, target) {
                if mg.is_some_and(|g| !g.contains(c)) {
                    continue;
                }
                if let Some(l) = release_list.as_mut() {
                    l.insert(c);
                }
                self.set_tribute_param(c, target);
                rcount += self.card(c).release_param;
            }
        }
        let mut ex_oneof_max: u32 = 0;
        for c in self.player[1 - p].list_mzone.clone().into_iter().flatten() {
            if (releasable >> (self.card(c).current.sequence + 16)) & 1 == 0 || !c.is_releasable_by_summon(self, p as u8, target) {
                continue;
            }
            if mg.is_some_and(|g| !g.contains(c)) {
                continue;
            }
            self.set_tribute_param(c, target);
            if c.is_affected_by_effect(self, EFFECT_EXTRA_RELEASE).is_some() {
                if let Some(l) = ex_list.as_mut() {
                    l.insert(c);
                }
                rcount += self.card(c).release_param;
            } else if ex || ex_tribute.contains(c) {
                if let Some(l) = release_list.as_mut() {
                    l.insert(c);
                }
                rcount += self.card(c).release_param;
            } else {
                let Some(pe) = c.is_affected_by_effect(self, EFFECT_EXTRA_RELEASE_SUM) else { continue };
                if self.effect(pe).is_flag(EFFECT_FLAG_COUNT_LIMIT) && self.effect(pe).count_limit == 0 {
                    continue;
                }
                if let Some(l) = ex_list_oneof.as_mut() {
                    l.insert(c);
                }
                if ex_oneof_max < self.card(c).release_param {
                    ex_oneof_max = self.card(c).release_param;
                }
            }
        }
        for c in public_order(self, &ex_tribute) {
            if self.card(c).current.location as u32 == LOCATION_MZONE || !c.is_releasable_by_summon(self, p as u8, target) {
                continue;
            }
            if let Some(l) = release_list.as_mut() {
                l.insert(c);
            }
            self.set_tribute_param(c, target);
            rcount += self.card(c).release_param;
        }
        (rcount + ex_oneof_max) as i32
    }
    pub fn get_summon_count_limit(&mut self, playerid: u8) -> i32 {
        if self.is_flag(DUEL_UNLIMITED_SUMMONS) {
            return i32::MAX - 100;
        }
        let mut eset = Vec::new();
        self.filter_player_effect(playerid, EFFECT_SET_SUMMON_COUNT_LIMIT, &mut eset, true);
        let mut count = 1;
        for pe in eset {
            let c = pe.get_value(self, 0) as i32;
            if c > count {
                count = c;
            }
        }
        count
    }
    pub fn get_draw_count(&mut self, playerid: u8) -> i32 {
        let mut eset = Vec::new();
        self.filter_player_effect(playerid, EFFECT_DRAW_COUNT, &mut eset, true);
        let mut count = self.player[playerid as usize].draw_count;
        for pe in eset {
            let c = pe.get_value(self, 0) as i32;
            if c == 0 {
                return 0;
            }
            if c > count {
                count = c;
            }
        }
        count
    }
    pub fn get_ritual_material(&mut self, playerid: u8, peffect: Option<EffectId>, material: &mut CardSet, check_level: bool) {
        let p = playerid as usize;
        let mzonecheck = |d: &mut Duel, c: CardId| {
            (!check_level || c.get_level(d) > 0) && c.is_affect_by_effect(d, peffect) && c.is_releasable_by_effect(d, playerid, peffect)
        };
        for c in self.player[p].list_mzone.clone().into_iter().flatten() {
            if mzonecheck(self, c) && c.is_releasable_by_nonsummon(self, playerid, REASON_EFFECT) {
                material.insert(c);
            }
        }
        for c in self.player[1 - p].list_mzone.clone().into_iter().flatten() {
            if self.card(c).is_position(POS_FACEUP) != 0
                && mzonecheck(self, c)
                && c.is_releasable_by_nonsummon(self, playerid, REASON_EFFECT)
                && c.is_affected_by_effect(self, EFFECT_EXTRA_RELEASE).is_some()
            {
                material.insert(c);
            }
        }
        for c in self.player[p].list_hand.clone() {
            if self.card(c).data.type_ & TYPE_MONSTER != 0 && c.is_releasable_by_nonsummon(self, playerid, REASON_EFFECT) {
                material.insert(c);
            }
        }
        for c in self.player[p].list_grave.clone() {
            if self.card(c).data.type_ & TYPE_MONSTER != 0
                && c.is_affected_by_effect(self, EFFECT_EXTRA_RITUAL_MATERIAL).is_some()
                && c.is_removeable(self, playerid, POS_FACEUP as u8, REASON_EFFECT)
            {
                material.insert(c);
            }
        }
        for c in self.player[p].list_main.clone().into_iter().chain(self.player[p].list_extra.clone()) {
            if c.is_affected_by_effect(self, EFFECT_EXTRA_RITUAL_MATERIAL).is_some() && (!check_level || c.get_level(self) > 0) {
                material.insert(c);
            }
        }
        for c in self.player[p].list_mzone.clone().into_iter().flatten() {
            for m in self.card(c).xyz_materials.clone() {
                if m.is_affected_by_effect(self, EFFECT_EXTRA_RITUAL_MATERIAL).is_some() {
                    material.insert(m);
                }
            }
        }
    }
    pub fn get_fusion_material(&mut self, playerid: u8, material: &mut CardSet) {
        let p = playerid as usize;
        for c in self.player[p].list_mzone.clone().into_iter().flatten() {
            material.insert(c);
        }
        for c in self.player[p].list_szone.clone().into_iter().flatten() {
            if c.is_affected_by_effect(self, EFFECT_EXTRA_FUSION_MATERIAL).is_some() {
                material.insert(c);
            }
        }
        for c in self.player[p].list_hand.clone() {
            if self.card(c).data.type_ & TYPE_MONSTER != 0 || c.is_affected_by_effect(self, EFFECT_EXTRA_FUSION_MATERIAL).is_some() {
                material.insert(c);
            }
        }
        for c in self.player[p].list_grave.clone() {
            if c.is_affected_by_effect(self, EFFECT_EXTRA_FUSION_MATERIAL).is_some() {
                material.insert(c);
            }
        }
    }
    pub fn ritual_release(&mut self, material: &CardSet, release_deck: bool) {
        let mut rel = CardSet::new();
        let mut rem = CardSet::new();
        let mut tograve = CardSet::new();
        let mut to_grave_types = LOCATION_OVERLAY | LOCATION_EXTRA;
        if !release_deck {
            to_grave_types |= LOCATION_DECK;
        }
        for c in public_order(self, material) {
            let loc = self.card(c).current.location as u32;
            if loc == LOCATION_GRAVE {
                rem.insert(c);
            } else if loc & to_grave_types != 0 {
                tograve.insert(c);
            } else {
                rel.insert(c);
            }
        }
        let (re, rp) = (self.core.reason_effect, self.core.reason_player);
        let reason = REASON_RITUAL + REASON_EFFECT + REASON_MATERIAL;
        self.send_to(tograve, re, reason, rp, PLAYER_NONE as u8, LOCATION_GRAVE as u16, 0, POS_FACEUP as u8, false);
        self.release(rel, re, reason, rp);
        self.send_to(rem, re, reason, rp, PLAYER_NONE as u8, LOCATION_REMOVED as u16, 0, POS_FACEUP as u8, false);
    }
    pub fn get_overlay_group(&mut self, playerid: u8, self_: u8, oppo: u8, pset: &mut CardSet, pgroup: Option<&CardSet>) {
        if let Some(g) = pgroup {
            for c in public_order(self, g) {
                if self.card(c).get_status(STATUS_SUMMONING | STATUS_SPSUMMON_STEP) == 0 {
                    pset.extend(self.card(c).xyz_materials.clone());
                }
            }
        } else {
            for i in 0..2u8 {
                if (i == playerid && self_ != 0) || (i == 1 - playerid && oppo != 0) {
                    for c in self.player[i as usize].list_mzone.clone().into_iter().flatten() {
                        if self.card(c).get_status(STATUS_SUMMONING | STATUS_SPSUMMON_STEP) == 0 {
                            pset.extend(self.card(c).xyz_materials.clone());
                        }
                    }
                }
            }
        }
    }
    pub fn get_overlay_count(&mut self, playerid: u8, self_: u8, oppo: u8, pgroup: Option<&CardSet>) -> i32 {
        let mut count = 0usize;
        if let Some(g) = pgroup {
            for c in public_order(self, g) {
                if self.card(c).get_status(STATUS_SUMMONING | STATUS_SPSUMMON_STEP) == 0 {
                    count += self.card(c).xyz_materials.len();
                }
            }
            return count as i32;
        }
        for i in 0..2u8 {
            if (i == playerid && self_ != 0) || (i == 1 - playerid && oppo != 0) {
                for c in self.player[i as usize].list_mzone.iter().flatten() {
                    if self.card(*c).get_status(STATUS_SUMMONING | STATUS_SPSUMMON_STEP) == 0 {
                        count += self.card(*c).xyz_materials.len();
                    }
                }
            }
        }
        count as i32
    }
    /// increase the binary custom counter 1~5
    pub fn check_card_counter(&mut self, pcard: CardId, counter_type: u32, playerid: i32) {
        let n = self.get_counter_map(counter_type).len();
        for i in 0..n {
            let (amount, f) = {
                let m = self.get_counter_map(counter_type);
                (m[i].1.player_amount[playerid as usize], m[i].1.check_function.clone())
            };
            if amount != 0 {
                continue;
            }
            self.add_param(Value::Card(pcard));
            if !self.check_condition(f.as_ref(), 1) {
                self.get_counter_map(counter_type)[i].1.player_amount[playerid as usize] += 1;
            }
        }
    }
    pub fn check_card_counter_group(&mut self, pgroup: &CardSet, counter_type: u32, playerid: i32) {
        let n = self.get_counter_map(counter_type).len();
        let cards = public_order(self, pgroup);
        for i in 0..n {
            let (amount, f) = {
                let m = self.get_counter_map(counter_type);
                (m[i].1.player_amount[playerid as usize], m[i].1.check_function.clone())
            };
            if amount != 0 {
                continue;
            }
            for &c in &cards {
                self.add_param(Value::Card(c));
                if !self.check_condition(f.as_ref(), 1) {
                    self.get_counter_map(counter_type)[i].1.player_amount[playerid as usize] += 1;
                    break;
                }
            }
        }
    }
    pub fn check_chain_counter(&mut self, peffect: EffectId, playerid: i32, chainid: i32) -> Option<Vec<u32>> {
        let mut ret: Option<Vec<u32>> = None;
        for i in 0..self.core.chain_counter.len() {
            let (id, f) = (self.core.chain_counter[i].0, self.core.chain_counter[i].1.check_function.clone());
            self.add_param(Value::Effect(peffect));
            self.add_param(Value::Int(playerid as i64));
            self.add_param(Value::Int(chainid as i64));
            if !self.check_condition(f.as_ref(), 3) {
                self.core.chain_counter[i].1.player_amount[playerid as usize] += 1;
                ret.get_or_insert_with(Vec::new).push(id);
            }
        }
        ret
    }
    pub fn restore_chain_counter(&mut self, playerid: u8, counters: &[u32]) {
        for &id in counters {
            if let Some(x) = self.core.chain_counter.iter_mut().find(|x| x.0 == id) {
                x.1.player_amount[playerid as usize] = x.1.player_amount[playerid as usize].wrapping_sub(1);
            }
        }
    }
    pub fn set_spsummon_counter(&mut self, playerid: u8, add: bool, chain: bool) {
        let p = playerid as usize;
        let old = self.is_flag(DUEL_CANNOT_SUMMON_OATH_OLD);
        if old {
            if add {
                self.core.spsummon_state_count[p] = self.core.spsummon_state_count[p].wrapping_add(1);
                if chain {
                    self.core.spsummon_state_count_rst[p] = self.core.spsummon_state_count_rst[p].wrapping_add(1);
                }
            } else if chain {
                self.core.spsummon_state_count[p] = self.core.spsummon_state_count[p].wrapping_sub(self.core.spsummon_state_count_rst[p]);
                self.core.spsummon_state_count_rst[p] = 0;
            } else {
                self.core.spsummon_state_count[p] = self.core.spsummon_state_count[p].wrapping_sub(1);
            }
        } else {
            self.core.spsummon_state_count[p] = self.core.spsummon_state_count[p].wrapping_add(1);
        }
        for pe in self.field_effects.spsummon_count_eff.clone() {
            let pc = pe.get_handler(self);
            let matches = |d: &Duel| {
                let ctrl = d.card(pc).current.controler;
                (playerid == ctrl && d.effect(pe).s_range != 0) || (playerid != ctrl && d.effect(pe).o_range != 0)
            };
            if old {
                if add {
                    if pe.is_available(self) && matches(self) {
                        let c = self.card_mut(pc);
                        c.spsummon_counter[p] = c.spsummon_counter[p].wrapping_add(1);
                        if chain {
                            c.spsummon_counter_rst[p] = c.spsummon_counter_rst[p].wrapping_add(1);
                        }
                    }
                } else {
                    let c = self.card_mut(pc);
                    c.spsummon_counter[p] = c.spsummon_counter[p].wrapping_sub(c.spsummon_counter_rst[p]);
                    c.spsummon_counter_rst[p] = 0;
                }
            } else if pe.is_available(self) && matches(self) {
                let c = self.card_mut(pc);
                c.spsummon_counter[p] = c.spsummon_counter[p].wrapping_add(1);
            }
        }
    }
    pub fn check_spsummon_counter(&mut self, playerid: u8, ct: u8) -> bool {
        for pe in self.field_effects.spsummon_count_eff.clone() {
            let pc = pe.get_handler(self);
            let val = self.effect(pe).value as u16;
            if pe.is_available(self) && self.card(pc).spsummon_counter[playerid as usize] as u32 + ct as u32 > val as u32 {
                return false;
            }
        }
        true
    }
    pub fn check_lp_cost(&mut self, playerid: u8, lp: u32) -> bool {
        let mut eset = Vec::new();
        let mut val = lp as i32;
        self.filter_player_effect(playerid, EFFECT_LPCOST_CHANGE, &mut eset, true);
        for peff in eset {
            self.add_param(Value::effect(self.core.reason_effect));
            self.add_param(Value::Int(playerid as i64));
            self.add_param(Value::Int(val as i64));
            val = peff.get_value(self, 3) as i32;
        }
        if val <= 0 {
            return true;
        }
        let e = Event {
            event_cards: None,
            event_player: playerid,
            event_value: lp,
            reason: 0,
            reason_effect: self.core.reason_effect,
            reason_player: playerid,
            ..Default::default()
        };
        if self.effect_replace_check(EFFECT_LPCOST_REPLACE, &e) {
            return true;
        }
        val <= self.player[playerid as usize].lp
    }
    pub fn get_field_counter(&self, mut playerid: u8, self_: u8, oppo: u8, countertype: u16) -> u32 {
        let mut c = self_;
        let mut count = 0u32;
        for _ in 0..2 {
            if c != 0 {
                let pl = &self.player[playerid as usize];
                for pc in pl.list_mzone.iter().chain(pl.list_szone.iter()).flatten() {
                    count += pc.get_counter(self, countertype) as u32;
                }
            }
            playerid = 1 - playerid;
            c = oppo;
        }
        count
    }
    pub fn effect_replace_check(&mut self, code: u32, e: &Event) -> bool {
        for pe in self.field_effects.continuous_effect.range(code) {
            let hp = pe.get_handler_player(self);
            if pe.is_activateable(self, hp, e, false, false, false, false, false) {
                return true;
            }
        }
        false
    }
    pub fn get_attack_target(
        &mut self,
        pcard: CardId,
        v: &mut Vec<CardId>,
        chain_attack: bool,
        select_target: bool,
        mut must_attack_map: Option<&mut Vec<(EffectId, CardId)>>,
    ) -> i32 {
        self.card_mut(pcard).direct_attackable = 0;
        let p = self.card(pcard).current.controler as usize;
        let mut auto_attack = Vec::new();
        let mut only_attack = Vec::new();
        let mut must_attack = Vec::new();
        for at in self.player[1 - p].list_mzone.clone().into_iter().flatten() {
            if at.is_affected_by_effect(self, EFFECT_ONLY_BE_ATTACKED).is_some() {
                auto_attack.push(Some(at));
            }
            if pcard.is_affected_by_effect_t(self, EFFECT_ONLY_ATTACK_MONSTER, Some(at)).is_some() {
                only_attack.push(Some(at));
            }
            if let Some(pe) = pcard.is_affected_by_effect_t(self, EFFECT_MUST_ATTACK_MONSTER, Some(at)) {
                must_attack.push(Some(at));
                if let Some(m) = must_attack_map.as_mut() {
                    // (a multimap ordered by effect address: kept in insertion order)
                    m.push((pe, at));
                }
            }
        }
        let atype;
        let pv: Vec<Option<CardId>>;
        if !auto_attack.is_empty() {
            atype = 1;
            if auto_attack.len() != 1 {
                return atype;
            }
            pv = auto_attack;
        } else if pcard.is_affected_by_effect(self, EFFECT_ONLY_ATTACK_MONSTER).is_some() {
            atype = 2;
            if only_attack.len() != 1 {
                return atype;
            }
            pv = only_attack;
        } else if pcard.is_affected_by_effect(self, EFFECT_MUST_ATTACK_MONSTER).is_some() {
            atype = 3;
            if must_attack.is_empty() {
                return atype;
            }
            pv = must_attack;
        } else {
            atype = 4;
            let mut attack_tg = Vec::new();
            for at in self.player[1 - p].list_mzone.clone() {
                if at != self.core.attacker {
                    attack_tg.push(at);
                }
            }
            if self.is_player_affected_by_effect(p as u8, EFFECT_SELF_ATTACK).is_some()
                && (pcard.is_affected_by_effect(self, EFFECT_ATTACK_ALL).is_none() || attack_tg.is_empty())
            {
                for at in self.player[p].list_mzone.clone() {
                    if at != self.core.attacker {
                        attack_tg.push(at);
                    }
                }
            }
            pv = attack_tg;
        }
        let mut extra_count = 0;
        let mut eset = Vec::new();
        pcard.filter_effect(self, EFFECT_EXTRA_ATTACK, &mut eset, true);
        for pe in eset {
            let val = pe.get_value_card(self, Some(pcard), 0) as i32;
            if val > extra_count {
                extra_count = val;
            }
        }
        let mut extra_count_m = 0;
        let mut eset = Vec::new();
        pcard.filter_effect(self, EFFECT_EXTRA_ATTACK_MONSTER, &mut eset, true);
        for pe in eset {
            let val = pe.get_value_card(self, Some(pcard), 0) as i32;
            if val > extra_count_m {
                extra_count_m = val;
            }
        }
        let announce_count = self.card(pcard).announce_count as i32;
        if !chain_attack
            && announce_count > extra_count
            && (extra_count_m == 0 || announce_count > extra_count_m || self.card(pcard).announced_cards.findcard(0) != 0)
        {
            if let Some(pe) = pcard.is_affected_by_effect(self, EFFECT_ATTACK_ALL) {
                if self.card(pcard).attack_all_target != 0 {
                    for at in pv.iter().flatten().copied() {
                        self.add_param(Value::Card(at));
                        if !pe.check_value_condition(self, 1) {
                            continue;
                        }
                        let fid = self.card(at).fieldid_r;
                        let lim = pe.get_value_card(self, Some(at), 0) as u32;
                        if self.card(pcard).announced_cards.findcard(fid) >= lim {
                            continue;
                        }
                        if atype >= 2 && at.is_affected_by_effect_t(self, EFFECT_IGNORE_BATTLE_TARGET, Some(pcard)).is_some() {
                            continue;
                        }
                        if select_target && (atype == 2 || atype == 4) {
                            if at.is_affected_by_effect_t(self, EFFECT_CANNOT_BE_BATTLE_TARGET, Some(pcard)).is_some() {
                                continue;
                            }
                            if pcard.is_affected_by_effect_t(self, EFFECT_CANNOT_SELECT_BATTLE_TARGET, Some(at)).is_some() {
                                continue;
                            }
                        }
                        v.push(at);
                    }
                }
            }
            return atype;
        }
        // chain attack or announce count check passed
        let mut mcount = 0u32;
        for at in pv.iter().flatten().copied() {
            if atype >= 2 && at.is_affected_by_effect_t(self, EFFECT_IGNORE_BATTLE_TARGET, Some(pcard)).is_some() {
                continue;
            }
            if self.card(at).current.controler as usize != p {
                mcount += 1;
            }
            if chain_attack && self.core.chain_attack_target.is_some() && Some(at) != self.core.chain_attack_target {
                continue;
            }
            if select_target && (atype == 2 || atype == 4) {
                if at.is_affected_by_effect_t(self, EFFECT_CANNOT_BE_BATTLE_TARGET, Some(pcard)).is_some() {
                    continue;
                }
                if pcard.is_affected_by_effect_t(self, EFFECT_CANNOT_SELECT_BATTLE_TARGET, Some(at)).is_some() {
                    continue;
                }
            }
            v.push(at);
        }
        if atype <= 3 {
            return atype;
        }
        if (mcount == 0 || pcard.is_affected_by_effect(self, EFFECT_DIRECT_ATTACK).is_some() || self.core.attack_player != 0)
            && pcard.is_affected_by_effect(self, EFFECT_CANNOT_DIRECT_ATTACK).is_none()
            && !(!chain_attack && extra_count_m != 0 && announce_count > extra_count)
            && !(chain_attack && self.core.chain_attack_target.is_some())
        {
            self.card_mut(pcard).direct_attackable = 1;
        }
        atype
    }
    pub fn confirm_attack_target(&mut self) -> bool {
        let mut cv = Vec::new();
        let attacker = self.core.attacker.unwrap();
        let ca = self.core.chain_attack;
        self.get_attack_target(attacker, &mut cv, ca, false, None);
        match self.core.attack_target {
            Some(t) => cv.contains(&t),
            None => self.card(attacker).direct_attackable != 0,
        }
    }
    /// update the validity for EFFECT_ATTACK_ALL (approximate solution)
    pub fn attack_all_target_check(&mut self) {
        let Some(attacker) = self.core.attacker else { return };
        let Some(target) = self.core.attack_target else {
            self.card_mut(attacker).attack_all_target = 0;
            return;
        };
        let Some(pe) = attacker.is_affected_by_effect(self, EFFECT_ATTACK_ALL) else { return };
        if pe.get_value_card(self, Some(target), 0) == 0 {
            self.card_mut(attacker).attack_all_target = 0;
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn check_tribute(&mut self, pcard: CardId, min: i32, mut max: i32, mg: Option<&CardSet>, toplayer: u8, mut zone: u32, releasable: u32, pos: u32) -> bool {
        let sumplayer = self.card(pcard).current.controler;
        let ex = toplayer == 1 - sumplayer;
        let mut release_list = CardSet::new();
        let mut ex_list = CardSet::new();
        let m = self.get_summon_release_list(pcard, Some(&mut release_list), Some(&mut ex_list), None, mg, ex, releasable, pos);
        if max > m {
            max = m;
        }
        if min > max {
            return false;
        }
        zone &= 0x1f & self.get_forced_zones(Some(pcard), toplayer, LOCATION_MZONE as u8, sumplayer as u32, LocationReason::ToField) as u32;
        let mut s = 0;
        let mut ct = self.get_tofield_count(Some(pcard), toplayer, LOCATION_MZONE as u8, sumplayer as u32, LocationReason::ToField, zone, None);
        if ct <= 0 && max <= 0 {
            return false;
        }
        let to_check = if !ex_list.is_empty() && ex_list.len() as i32 >= min { ex_list.clone() } else { release_list.clone() };
        for c in to_check.iter() {
            let cc = self.card(c);
            if cc.current.location as u32 == LOCATION_MZONE && cc.current.controler == toplayer {
                s += 1;
                if (zone >> cc.current.sequence) & 1 != 0 {
                    ct += 1;
                }
            }
        }
        if ct <= 0 {
            return false;
        }
        max -= ex_list.len() as i32;
        let fcount = self.get_mzone_limit(toplayer, sumplayer, LocationReason::ToField);
        if s < -fcount + 1 {
            return false;
        }
        if max < 0 {
            max = 0;
        }
        if max < -fcount + 1 {
            return false;
        }
        true
    }
    pub fn check_with_sum_limit(d: &Duel, mats: &[CardId], acc: i32, mut index: i32, count: i32, min: i32, max: i32, should_continue: &mut Option<&mut bool>) -> bool {
        if count > max {
            return false;
        }
        while (index as usize) < mats.len() {
            let sp = d.card(mats[index as usize]).sum_param;
            let op1 = (sp & 0xffff) as i32;
            let op2 = ((sp >> 16) & 0xffff) as i32;
            if (op1 == acc || op2 == acc) && count >= min {
                return true;
            }
            index += 1;
            if acc > op1 && Self::check_with_sum_limit(d, mats, acc - op1, index, count + 1, min, max, should_continue) {
                return true;
            }
            if op2 != 0 && acc > op2 && Self::check_with_sum_limit(d, mats, acc - op2, index, count + 1, min, max, should_continue) {
                return true;
            }
        }
        if count + 1 < min {
            if let Some(sc) = should_continue.as_mut() {
                **sc = false;
            }
        }
        false
    }
    #[allow(clippy::too_many_arguments)]
    pub fn check_with_sum_limit_m(d: &Duel, mats: &[CardId], acc: i32, index: i32, min: i32, max: i32, must_count: i32, should_continue: &mut Option<&mut bool>) -> bool {
        if acc == 0 {
            return index == must_count && 0 >= min && 0 <= max;
        }
        if index == must_count {
            return Self::check_with_sum_limit(d, mats, acc, index, 1, min, max, should_continue);
        }
        if index as usize >= mats.len() {
            return false;
        }
        let sp = d.card(mats[index as usize]).sum_param;
        let op1 = (sp & 0xffff) as i32;
        let op2 = ((sp >> 16) & 0xffff) as i32;
        if acc >= op1 && Self::check_with_sum_limit_m(d, mats, acc - op1, index + 1, min, max, must_count, should_continue) {
            return true;
        }
        if op2 != 0 && acc >= op2 && Self::check_with_sum_limit_m(d, mats, acc - op2, index + 1, min, max, must_count, should_continue) {
            return true;
        }
        false
    }
    fn sum_greater_limit(d: &Duel, mats: &[CardId], acc: i32, mut index: i32, opmin: i32, memo: &mut std::collections::HashMap<u64, bool>) -> bool {
        let key = ((index as u64) << 42) | (((acc + 0x100000) as u64) << 21) | (opmin as u64 & 0x1fffff);
        if let Some(&r) = memo.get(&key) {
            return r;
        }
        let mut result = false;
        while (index as usize) < mats.len() {
            let sp = d.card(mats[index as usize]).sum_param;
            let op1 = (sp & 0xffff) as i32;
            let op2 = ((sp >> 16) & 0xffff) as i32;
            if (acc <= op1 && acc + opmin > op1) || (op2 != 0 && acc <= op2 && acc + opmin > op2) {
                result = true;
                break;
            }
            index += 1;
            if Self::sum_greater_limit(d, mats, acc - op1, index, opmin.min(op1), memo)
                || (op2 != 0 && Self::sum_greater_limit(d, mats, acc - op2, index, opmin.min(op2), memo))
            {
                result = true;
                break;
            }
        }
        memo.insert(key, result);
        result
    }
    pub fn check_with_sum_greater_limit(d: &Duel, mats: &[CardId], acc: i32, index: i32, opmin: i32, _should_continue: &mut Option<&mut bool>) -> bool {
        let mut memo = std::collections::HashMap::new();
        Self::sum_greater_limit(d, mats, acc, index, opmin, &mut memo)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn check_with_sum_greater_limit_m(d: &Duel, mats: &[CardId], acc: i32, index: i32, opmin: i32, must_count: i32, should_continue: &mut Option<&mut bool>) -> bool {
        if acc <= 0 {
            return index == must_count && acc + opmin > 0;
        }
        if index == must_count {
            return Self::check_with_sum_greater_limit(d, mats, acc, index, opmin, should_continue);
        }
        if index as usize >= mats.len() {
            return false;
        }
        let sp = d.card(mats[index as usize]).sum_param;
        let op1 = (sp & 0xffff) as i32;
        let op2 = ((sp >> 16) & 0xffff) as i32;
        if Self::check_with_sum_greater_limit_m(d, mats, acc - op1, index + 1, opmin.min(op1), must_count, should_continue) {
            return true;
        }
        if op2 != 0 && Self::check_with_sum_greater_limit_m(d, mats, acc - op2, index + 1, opmin.min(op2), must_count, should_continue) {
            return true;
        }
        if index < must_count && (acc - op1 <= 0 && (op2 == 0 || acc - op2 <= 0)) {
            if let Some(sc) = should_continue.as_mut() {
                **sc = false;
            }
        }
        false
    }
    pub fn is_player_can_draw(&mut self, playerid: u8) -> bool {
        self.is_player_affected_by_effect(playerid, EFFECT_CANNOT_DRAW).is_none()
    }
    pub fn is_player_can_discard_deck(&mut self, playerid: u8, count: u32) -> bool {
        if (self.player[playerid as usize].list_main.len() as u32) < count {
            return false;
        }
        self.is_player_affected_by_effect(playerid, EFFECT_CANNOT_DISCARD_DECK).is_none()
    }
    pub fn is_player_can_discard_deck_as_cost(&mut self, playerid: u8, count: u32) -> bool {
        let p = playerid as usize;
        if (self.player[p].list_main.len() as u32) < count {
            return false;
        }
        if self.is_player_affected_by_effect(playerid, EFFECT_CANNOT_DISCARD_DECK).is_some() {
            return false;
        }
        let Some(&topcard) = self.player[p].list_main.last() else { return true };
        if count == 1 && self.card(topcard).is_position(POS_FACEUP) != 0 {
            return topcard.is_capable_cost_to_grave(self, playerid);
        }
        let cant_remove_s = !self.is_player_can_action(playerid, EFFECT_CANNOT_REMOVE);
        let cant_remove_o = !self.is_player_can_action(1 - playerid, EFFECT_CANNOT_REMOVE);
        let mut eset = Vec::new();
        self.filter_field_effect(EFFECT_TO_GRAVE_REDIRECT, &mut eset, true);
        for peff in eset {
            let redirect = peff.get_value(self, 0) as u32;
            let p2 = peff.get_handler_player(self);
            if redirect & LOCATION_REMOVED != 0 {
                if topcard.is_affected_by_effect(self, EFFECT_CANNOT_REMOVE).is_some() {
                    continue;
                }
                if (cant_remove_s && p2 == playerid) || (cant_remove_o && p2 != playerid) {
                    continue;
                }
            }
            let e = self.effect(peff);
            if (p2 == playerid && e.s_range as u32 & LOCATION_DECK != 0) || (p2 != playerid && e.o_range as u32 & LOCATION_DECK != 0) {
                return false;
            }
        }
        true
    }
    pub fn is_player_can_discard_hand(&mut self, playerid: u8, pcard: CardId, peffect: Option<EffectId>, reason: u32) -> bool {
        if self.card(pcard).current.location as u32 != LOCATION_HAND {
            return false;
        }
        let mut eset = Vec::new();
        self.filter_player_effect(playerid, EFFECT_CANNOT_DISCARD_HAND, &mut eset, true);
        for peff in eset {
            let Some(t) = self.effect(peff).target.clone() else { return false };
            self.add_param(Value::Effect(peff));
            self.add_param(Value::Card(pcard));
            self.add_param(Value::effect(peffect));
            self.add_param(Value::Int(reason as i64));
            if self.check_condition(Some(&t), 4) {
                return false;
            }
        }
        true
    }
    pub fn is_player_can_action(&mut self, playerid: u8, actionlimit: u32) -> bool {
        let mut eset = Vec::new();
        self.filter_player_effect(playerid, actionlimit, &mut eset, true);
        eset.iter().all(|&e| self.effect(e).target.is_some())
    }
    /// The checks of a player effect with a target function and `params` (false if one matches).
    fn player_effects_allow(&mut self, playerid: u8, code: u32, params: &[Value]) -> bool {
        let mut eset = Vec::new();
        self.filter_player_effect(playerid, code, &mut eset, true);
        for peff in eset {
            let Some(t) = self.effect(peff).target.clone() else { return false };
            self.add_param(Value::Effect(peff));
            for v in params {
                self.add_param(v.clone());
            }
            if self.check_condition(Some(&t), 1 + params.len() as u32) {
                return false;
            }
        }
        true
    }
    pub fn is_player_can_summon(&mut self, mut sumtype: u32, playerid: u8, pcard: CardId, toplayer: u8) -> bool {
        sumtype |= SUMMON_TYPE_NORMAL;
        if !self.player_effects_allow(
            playerid,
            EFFECT_CANNOT_SUMMON,
            &[Value::Card(pcard), Value::Int(playerid as i64), Value::Int(sumtype as i64), Value::Int(POS_FACEUP as i64), Value::Int(toplayer as i64)],
        ) {
            return false;
        }
        let mut eset = Vec::new();
        self.filter_player_effect(playerid, EFFECT_FORCE_NORMAL_SUMMON_POSITION, &mut eset, true);
        let mut sumpos: u8 = POS_FACEUP_ATTACK as u8;
        if self.is_flag(DUEL_NORMAL_SUMMON_FACEUP_DEF)
            || self.is_player_affected_by_effect(playerid, EFFECT_NORMAL_SUMMON_FACEUP_DEFENSE).is_some()
            || self.is_player_affected_by_effect(playerid, EFFECT_DEVINE_LIGHT).is_some()
        {
            sumpos = POS_FACEUP as u8;
        }
        for eff in eset {
            if let Some(t) = self.effect(eff).target.clone() {
                self.add_param(Value::Effect(eff));
                self.add_param(Value::Card(pcard));
                self.add_param(Value::Int(playerid as i64));
                self.add_param(Value::Int(sumtype as i64));
                self.add_param(Value::Int(sumpos as i64));
                self.add_param(Value::Int(toplayer as i64));
                if !self.check_condition(Some(&t), 6) {
                    continue;
                }
            }
            sumpos &= eff.get_value(self, 0) as u8;
            if sumpos == 0 {
                return false;
            }
        }
        true
    }
    pub fn is_player_can_mset(&mut self, mut sumtype: u32, playerid: u8, pcard: CardId, toplayer: u8) -> bool {
        sumtype |= SUMMON_TYPE_NORMAL;
        self.player_effects_allow(
            playerid,
            EFFECT_CANNOT_MSET,
            &[Value::Card(pcard), Value::Int(playerid as i64), Value::Int(sumtype as i64), Value::Int(toplayer as i64), Value::Int(POS_FACEDOWN as i64)],
        )
    }
    pub fn is_player_can_sset(&mut self, playerid: u8, pcard: CardId) -> bool {
        self.player_effects_allow(playerid, EFFECT_CANNOT_SSET, &[Value::Card(pcard), Value::Int(playerid as i64)])
    }
    /// check player-effect EFFECT_CANNOT_SPECIAL_SUMMON without target
    pub fn is_player_can_spsummon(&mut self, playerid: u8) -> bool {
        if !self.is_player_can_action(playerid, EFFECT_CANNOT_SPECIAL_SUMMON) {
            return false;
        }
        self.is_player_can_spsummon_count(playerid, 1)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn is_player_can_spsummon_ex(
        &mut self,
        peffect: Option<EffectId>,
        mut sumtype: u32,
        mut sumpos: u8,
        playerid: u8,
        toplayer: u8,
        pcard: CardId,
        proc_effect: Option<EffectId>,
    ) -> bool {
        if pcard.is_affected_by_effect(self, EFFECT_CANNOT_SPECIAL_SUMMON).is_some() {
            return false;
        }
        let c = self.card(pcard);
        if c.is_status(STATUS_FORBIDDEN) {
            return false;
        }
        if c.data.type_ & TYPE_TOKEN != 0 && c.current.location as u32 & LOCATION_ONFIELD != 0 {
            return false;
        }
        if c.data.type_ & TYPE_LINK != 0 && c.data.type_ & TYPE_MONSTER != 0 {
            sumpos &= POS_FACEUP_ATTACK as u8;
        }
        if sumpos == 0 {
            return false;
        }
        sumtype |= SUMMON_TYPE_SPECIAL;
        if !pcard.check_cost_condition_sum(self, EFFECT_SPSUMMON_COST, playerid, sumtype) {
            return false;
        }
        if sumpos as u32 & POS_FACEDOWN != 0 && self.is_player_affected_by_effect(playerid, EFFECT_DEVINE_LIGHT).is_some() {
            sumpos = (sumpos & POS_FACEUP as u8) | ((sumpos & POS_FACEDOWN as u8) >> 1);
        }
        if !self.player_effects_allow(
            playerid,
            EFFECT_CANNOT_SPECIAL_SUMMON,
            &[
                Value::Card(pcard),
                Value::Int(playerid as i64),
                Value::Int(sumtype as i64),
                Value::Int(sumpos as i64),
                Value::Int(toplayer as i64),
                Value::effect(peffect),
                Value::effect(proc_effect),
            ],
        ) {
            return false;
        }
        let mut eset = Vec::new();
        self.filter_player_effect(playerid, EFFECT_FORCE_SPSUMMON_POSITION, &mut eset, true);
        for eff in eset {
            if let Some(t) = self.effect(eff).target.clone() {
                self.add_param(Value::Effect(eff));
                self.add_param(Value::Card(pcard));
                self.add_param(Value::Int(playerid as i64));
                self.add_param(Value::Int(sumtype as i64));
                self.add_param(Value::Int(sumpos as i64));
                self.add_param(Value::Int(toplayer as i64));
                self.add_param(Value::effect(peffect));
                if !self.check_condition(Some(&t), 7) {
                    continue;
                }
            }
            sumpos &= eff.get_value(self, 0) as u8;
            if sumpos == 0 {
                return false;
            }
        }
        if !self.check_spsummon_once(pcard, playerid) {
            return false;
        }
        self.check_spsummon_counter(playerid, 1)
    }
    pub fn is_player_can_flipsummon(&mut self, playerid: u8, pcard: CardId) -> bool {
        self.player_effects_allow(playerid, EFFECT_CANNOT_FLIP_SUMMON, &[Value::Card(pcard), Value::Int(playerid as i64)])
    }
    pub fn is_player_can_spsummon_monster(&mut self, playerid: u8, toplayer: u8, sumpos: u8, sumtype: u32, pdata: &CardData) -> bool {
        let t = self.temp_card;
        self.card_mut(t).data = *pdata;
        let re = self.core.reason_effect;
        let result = self.is_player_can_spsummon_ex(re, sumtype, sumpos, playerid, toplayer, t, None);
        self.card_mut(t).data = CardData::default();
        result
    }
    pub fn is_player_can_release(&mut self, playerid: u8, pcard: CardId, reason: u32) -> bool {
        let re = self.core.reason_effect;
        self.player_effects_allow(playerid, EFFECT_CANNOT_RELEASE, &[Value::Card(pcard), Value::Int(playerid as i64), Value::Int(reason as i64), Value::effect(re)])
    }
    pub fn is_player_can_spsummon_count(&mut self, playerid: u8, count: u32) -> bool {
        let mut eset = Vec::new();
        self.filter_player_effect(playerid, EFFECT_LEFT_SPSUMMON_COUNT, &mut eset, true);
        for peff in eset {
            self.add_param(Value::effect(self.core.reason_effect));
            self.add_param(Value::Int(playerid as i64));
            let v = peff.get_value(self, 2) as i32;
            if v < count as i32 {
                return false;
            }
        }
        self.check_spsummon_counter(playerid, count as u8)
    }
    pub fn is_player_can_place_counter(&mut self, playerid: u8, pcard: CardId, countertype: u16, count: u16) -> bool {
        self.player_effects_allow(
            playerid,
            EFFECT_CANNOT_PLACE_COUNTER,
            &[Value::Card(pcard), Value::Int(playerid as i64), Value::Int(countertype as i64), Value::Int(count as i64)],
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn is_player_can_remove_counter(&mut self, playerid: u8, pcard: Option<CardId>, self_: u8, oppo: u8, countertype: u16, count: u16, reason: u32) -> bool {
        let have = match pcard {
            Some(c) => c.get_counter(self, countertype) >= count as i32,
            None => self.get_field_counter(playerid, self_, oppo, countertype) >= count as u32,
        };
        if have {
            return true;
        }
        let e = Event {
            event_cards: None,
            event_player: playerid,
            event_value: count as u32,
            reason,
            reason_effect: self.core.reason_effect,
            reason_player: playerid,
            ..Default::default()
        };
        for pe in self.field_effects.continuous_effect.range(EFFECT_RCOUNTER_REPLACE + countertype as u32) {
            let hp = pe.get_handler_player(self);
            if pe.is_activateable(self, hp, &e, false, false, false, false, false) {
                return true;
            }
        }
        false
    }
    #[allow(clippy::too_many_arguments)]
    pub fn is_player_can_remove_overlay_card(&mut self, playerid: u8, pgroup: Option<&CardSet>, self_: u8, oppo: u8, min: u16, reason: u32) -> bool {
        if self.get_overlay_count(playerid, self_, oppo, pgroup) >= min as i32 {
            return true;
        }
        let e = Event {
            event_cards: None,
            event_player: playerid,
            event_value: min as u32,
            reason,
            reason_effect: self.core.reason_effect,
            reason_player: playerid,
            ..Default::default()
        };
        for pe in self.field_effects.continuous_effect.range(EFFECT_OVERLAY_REMOVE_REPLACE) {
            let hp = pe.get_handler_player(self);
            if pe.is_activateable(self, hp, &e, false, false, false, false, false) {
                return true;
            }
        }
        false
    }
    pub fn is_player_can_send_to_grave(&mut self, playerid: u8, pcard: CardId, reason: u32) -> bool {
        self.player_effects_allow(playerid, EFFECT_CANNOT_TO_GRAVE, &[Value::Card(pcard), Value::Int(playerid as i64), Value::Int(reason as i64)])
    }
    pub fn is_player_can_send_to_hand(&mut self, playerid: u8, pcard: CardId) -> bool {
        let re = self.core.reason_effect;
        if !self.player_effects_allow(playerid, EFFECT_CANNOT_TO_HAND, &[Value::Card(pcard), Value::Int(playerid as i64), Value::effect(re)]) {
            return false;
        }
        if pcard.is_extra_deck_monster(self) && !self.is_player_can_send_to_deck(playerid, pcard) {
            return false;
        }
        true
    }
    pub fn is_player_can_send_to_deck(&mut self, playerid: u8, pcard: CardId) -> bool {
        self.player_effects_allow(playerid, EFFECT_CANNOT_TO_DECK, &[Value::Card(pcard), Value::Int(playerid as i64)])
    }
    pub fn is_player_can_remove(&mut self, playerid: u8, pcard: CardId, reason: u32) -> bool {
        let re = self.core.reason_effect;
        self.player_effects_allow(playerid, EFFECT_CANNOT_REMOVE, &[Value::Card(pcard), Value::Int(playerid as i64), Value::Int(reason as i64), Value::effect(re)])
    }
    fn chain_effect_at(&self, chaincount: u8) -> Option<EffectId> {
        if chaincount as usize > self.core.current_chain.len() {
            return None;
        }
        if chaincount == 0 {
            self.core.current_chain.last()?.triggering_effect
        } else {
            self.core.current_chain[chaincount as usize - 1].triggering_effect
        }
    }
    pub fn is_chain_negatable(&mut self, chaincount: u8) -> bool {
        let Some(pe) = self.chain_effect_at(chaincount) else { return false };
        if self.effect(pe).is_flag(EFFECT_FLAG_CANNOT_INACTIVATE) {
            return false;
        }
        let mut eset = Vec::new();
        self.filter_field_effect(EFFECT_CANNOT_INACTIVATE, &mut eset, true);
        for peff in eset {
            self.add_param(Value::Int(chaincount as i64));
            if peff.check_value_condition(self, 1) {
                return false;
            }
        }
        true
    }
    pub fn is_chain_disablable(&mut self, chaincount: u8) -> bool {
        let Some(pe) = self.chain_effect_at(chaincount) else { return false };
        let h = pe.get_handler(self);
        if self.card(h).get_status(STATUS_FORBIDDEN) == 0 {
            if self.effect(pe).is_flag(EFFECT_FLAG_CANNOT_DISABLE) {
                return false;
            }
            let mut eset = Vec::new();
            self.filter_field_effect(EFFECT_CANNOT_DISEFFECT, &mut eset, true);
            for peff in eset {
                self.add_param(Value::Int(chaincount as i64));
                if peff.check_value_condition(self, 1) {
                    return false;
                }
            }
        }
        true
    }
    pub fn is_chain_disabled(&mut self, chaincount: u8) -> bool {
        if chaincount as usize > self.core.current_chain.len() {
            return false;
        }
        let idx = if chaincount == 0 {
            match self.core.current_chain.len().checked_sub(1) {
                Some(i) => i,
                None => return false,
            }
        } else {
            chaincount as usize - 1
        };
        let ch = &self.core.current_chain[idx];
        if ch.flag & CHAIN_DISABLE_EFFECT != 0 {
            return true;
        }
        let chain_id = ch.chain_id;
        let pcard = ch.triggering_effect.unwrap().get_handler(self);
        for pe in self.card(pcard).single_effect.range(EFFECT_DISABLE_CHAIN) {
            if pe.get_value(self, 0) == chain_id as i64 {
                self.effect_mut(pe).reset_flag |= RESET_CHAIN;
                return true;
            }
        }
        false
    }
    pub fn check_chain_target(&mut self, chaincount: u8, pcard: CardId) -> bool {
        if chaincount as usize > self.core.current_chain.len() {
            return false;
        }
        let idx = if chaincount == 0 {
            match self.core.current_chain.len().checked_sub(1) {
                Some(i) => i,
                None => return false,
            }
        } else {
            chaincount as usize - 1
        };
        let ch = self.core.current_chain[idx].clone();
        let pe = ch.triggering_effect.unwrap();
        let tp = ch.triggering_player;
        if !self.effect(pe).is_flag(EFFECT_FLAG_CARD_TARGET) {
            return false;
        }
        let Some(t) = self.effect(pe).target.clone() else { return false };
        self.add_param(Value::Effect(pe));
        self.add_param(Value::Int(tp as i64));
        self.add_param(Value::from_group(&ch.evt.event_cards));
        self.add_param(Value::Int(ch.evt.event_player as i64));
        self.add_param(Value::Int(ch.evt.event_value as i64));
        self.add_param(Value::effect(ch.evt.reason_effect));
        self.add_param(Value::Int(ch.evt.reason as i64));
        self.add_param(Value::Int(ch.evt.reason_player as i64));
        self.add_param(Value::Int(0));
        self.add_param(Value::Card(pcard));
        self.check_condition(Some(&t), 10)
    }
    /// `field::get_chain`
    pub fn get_chain(&self, mut chaincount: u8) -> Option<ChainRef> {
        if chaincount == 0
            && !self.core.continuous_chain.is_empty()
            && self.core.reason_effect.is_some_and(|re| self.effect(re).type_ as u32 & EFFECT_TYPE_CONTINUOUS != 0)
        {
            return Some(ChainRef::Continuous(self.core.continuous_chain.len() - 1));
        }
        if chaincount == 0 || chaincount as usize > self.core.current_chain.len() {
            chaincount = self.core.current_chain.len() as u8;
            if chaincount == 0 {
                return None;
            }
        }
        Some(ChainRef::Current(chaincount as usize - 1))
    }
    pub fn chain_at(&self, r: ChainRef) -> &Chain {
        match r {
            ChainRef::Continuous(i) => &self.core.continuous_chain[i],
            ChainRef::Current(i) => &self.core.current_chain[i],
        }
    }
    pub fn chain_at_mut(&mut self, r: ChainRef) -> &mut Chain {
        match r {
            ChainRef::Continuous(i) => &mut self.core.continuous_chain[i],
            ChainRef::Current(i) => &mut self.core.current_chain[i],
        }
    }
    fn is_cteffect_card(&self, peffect: EffectId) -> bool {
        let ph = peffect.get_handler(self);
        let e = self.effect(peffect);
        self.card(ph).data.type_ == (TYPE_TRAP | TYPE_CONTINUOUS)
            && e.type_ as u32 & EFFECT_TYPE_ACTIVATE != 0
            && e.code == EVENT_FREE_CHAIN
            && e.cost.is_none()
            && e.target.is_none()
            && e.operation.is_none()
    }
    pub fn get_cteffect(&mut self, peffect: EffectId, playerid: i32, store: bool) -> i32 {
        if !self.is_cteffect_card(peffect) {
            return 0;
        }
        let phandler = peffect.get_handler(self);
        if store {
            self.core.select_chains.clear();
            self.core.select_options.clear();
        }
        for (code, fe) in self.card(phandler).field_effect.items.clone() {
            if self.effect(fe).type_ as u32 & (EFFECT_TYPE_TRIGGER_F | EFFECT_TYPE_TRIGGER_O | EFFECT_TYPE_QUICK_O) == 0 {
                continue;
            }
            if !fe.in_range(self, phandler) {
                continue;
            }
            if code == EVENT_FREE_CHAIN || code == EVENT_PHASE + self.infos.phase as u32 {
                self.nil_event.event_code = code;
                let ev = self.nil_event.clone();
                if self.get_cteffect_evt(fe, playerid, &ev, store) && !store {
                    return 1;
                }
            } else {
                for ev in self.core.point_event.clone().iter().chain(self.core.instant_event.clone().iter()) {
                    if code != ev.event_code {
                        continue;
                    }
                    if self.get_cteffect_evt(fe, playerid, ev, store) && !store {
                        return 1;
                    }
                }
            }
        }
        (store && !self.core.select_chains.is_empty()) as i32
    }
    pub fn get_cteffect_evt(&mut self, feffect: EffectId, playerid: i32, e: &Event, store: bool) -> bool {
        if !feffect.is_activateable(self, playerid as u8, e, false, false, false, false, true) {
            return false;
        }
        if store {
            let ch = Chain { evt: e.clone(), triggering_effect: Some(feffect), ..Default::default() };
            self.core.select_chains.push_back(ch);
            let desc = self.effect(feffect).description;
            self.core.select_options.push(desc);
        }
        true
    }
    pub fn check_cteffect_hint(&mut self, peffect: EffectId, playerid: u8) -> bool {
        if !self.is_cteffect_card(peffect) {
            return false;
        }
        let phandler = peffect.get_handler(self);
        for (code, fe) in self.card(phandler).field_effect.items.clone() {
            if self.effect(fe).type_ as u32 & (EFFECT_TYPE_TRIGGER_F | EFFECT_TYPE_TRIGGER_O | EFFECT_TYPE_QUICK_O) == 0 {
                continue;
            }
            if !fe.in_range(self, phandler) {
                continue;
            }
            if code == EVENT_FREE_CHAIN || code == (EVENT_PHASE | self.infos.phase as u32) {
                self.nil_event.event_code = code;
                let ev = self.nil_event.clone();
                if self.get_cteffect_evt(fe, playerid as i32, &ev, false) && (code != EVENT_FREE_CHAIN || self.check_hint_timing(fe)) {
                    return true;
                }
            } else {
                for ev in self.core.point_event.clone().iter().chain(self.core.instant_event.clone().iter()) {
                    if code != ev.event_code {
                        continue;
                    }
                    if self.get_cteffect_evt(fe, playerid as i32, ev, false) {
                        return true;
                    }
                }
            }
        }
        false
    }
    pub fn check_nonpublic_trigger(&mut self, ch: &mut Chain) -> bool {
        let pe = ch.triggering_effect.unwrap();
        let phandler = pe.get_handler(self);
        let e = self.effect(pe);
        let (typ, range) = (e.type_ as u32, e.range as u32);
        if !e.is_flag(EFFECT_FLAG_FIELD_ONLY)
            && ((typ & EFFECT_TYPE_SINGLE != 0
                && !e.is_flag(EFFECT_FLAG_SINGLE_RANGE)
                && phandler.is_has_relation_chain(self, pe, ch.chain_id)
                && ch.triggering_location as u32 & (LOCATION_HAND | LOCATION_DECK) != 0)
                || range & (LOCATION_HAND | LOCATION_DECK) != 0)
        {
            ch.flag |= CHAIN_HAND_TRIGGER;
            self.core.new_ochain_h.push_back(ch.clone());
            if !self.is_flag(DUEL_TCG_SEGOC_NONPUBLIC)
                && ((ch.triggering_location as u32 == LOCATION_HAND && self.card(phandler).is_position(POS_FACEDOWN) != 0)
                    || ch.triggering_location as u32 == LOCATION_DECK
                    || (range != 0 && !pe.in_range_chain(self, ch)))
            {
                return false;
            }
        }
        true
    }
    pub fn check_trigger_effect(&self, ch: &Chain) -> bool {
        let pe = ch.triggering_effect.unwrap();
        let phandler = pe.get_handler(self);
        if self.is_flag(DUEL_TRIGGER_ONLY_IN_LOCATION) {
            return phandler.is_has_relation_chain(self, pe, ch.chain_id);
        }
        if self.effect(pe).type_ as u32 & EFFECT_TYPE_FIELD != 0 && !phandler.is_has_relation_chain(self, pe, ch.chain_id) {
            return false;
        }
        if self.effect(pe).code == EVENT_FLIP && self.infos.phase as u32 == PHASE_DAMAGE {
            return true;
        }
        if self.is_flag(DUEL_TRIGGER_WHEN_PRIVATE_KNOWLEDGE) {
            return true;
        }
        let hc = self.card(phandler);
        if hc.current.location as u32 & LOCATION_DECK != 0 && ch.triggering_location as u32 != LOCATION_DECK {
            return false;
        }
        if ch.triggering_location as u32 & (LOCATION_DECK | LOCATION_HAND | LOCATION_EXTRA) != 0 && ch.triggering_position as u32 & POS_FACEDOWN != 0 {
            return true;
        }
        hc.current.location as u32 & (LOCATION_DECK | LOCATION_HAND | LOCATION_EXTRA) == 0 || hc.is_position(POS_FACEUP) != 0
    }
    pub fn check_spself_from_hand_trigger(&self, ch: &Chain) -> bool {
        let pe = ch.triggering_effect.unwrap();
        let tp = ch.triggering_player;
        if self.effect(pe).status as u32 & EFFECT_STATUS_SUMMON_SELF != 0 && ch.flag & CHAIN_HAND_TRIGGER != 0 {
            return !self.core.current_chain.iter().any(|c| {
                c.triggering_player == tp
                    && self.effect(c.triggering_effect.unwrap()).status as u32 & EFFECT_STATUS_SUMMON_SELF != 0
                    && c.flag & CHAIN_HAND_TRIGGER != 0
            });
        }
        true
    }
    pub fn is_able_to_enter_bp(&mut self) -> bool {
        let tp = self.infos.turn_player;
        (self.is_flag(DUEL_ATTACK_FIRST_TURN) || self.infos.turn_id != 1 || self.is_player_affected_by_effect(tp, EFFECT_BP_FIRST_TURN).is_some())
            && (self.infos.phase as u32) < PHASE_BATTLE_START
            && self.is_player_affected_by_effect(tp, EFFECT_CANNOT_BP).is_none()
    }
}
