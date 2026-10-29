//! Operations (`operations.cpp` part 7: destroying, releasing and sending cards).

use crate::card::{card_public_sort, public_order, sort_by_less};
use crate::constants::*;
use crate::duel::{Duel, Event};
use crate::effect::sort_effects_by_id;
use crate::types::*;
use crate::units::{self, LocationReason, SendToExargs};

impl Duel {
    /// The reason of a card that is not moved after all goes back to what it was.
    pub(crate) fn restore_temp_reason(&mut self, pcard: CardId) {
        let c = self.card_mut(pcard);
        c.current.reason = c.temp.reason;
        c.current.reason_effect = c.temp.reason_effect;
        c.current.reason_player = c.temp.reason_player;
    }
    /// The parameters of an indestructibility value (`reason_effect, reason, reason_player`).
    fn add_indes_params(&mut self, pcard: CardId) {
        let c = self.card(pcard);
        let (re, r, rp) = (c.current.reason_effect, c.current.reason, c.current.reason_player);
        self.add_param(Value::effect(re));
        self.add_param(Value::Int(r as i64));
        self.add_param(Value::Int(rp as i64));
    }
    /// `++pcard->indestructable_effects[id]` (emplacing 0 first).
    fn inc_indestructable(&mut self, pcard: CardId, peff: EffectId) -> i32 {
        let id = self.effect(peff).id;
        let c = self.card_mut(pcard);
        match c.indestructable_effects.iter().position(|x| x.0 == id) {
            Some(i) => {
                c.indestructable_effects[i].1 += 1;
                c.indestructable_effects[i].1
            }
            None => {
                let pos = c.indestructable_effects.partition_point(|x| x.0 < id);
                c.indestructable_effects.insert(pos, (id, 1));
                1
            }
        }
    }
    fn hint_effect_owner(&mut self, peff: EffectId) {
        let owner = self.effect(peff).owner.unwrap();
        let code = self.card(owner).data.code;
        self.hint_card(code);
    }

    pub fn process_destroy_replace(&mut self, arg: &mut units::DestroyReplace) -> bool {
        let g = arg.targets.unwrap();
        let target = arg.target;
        if self.card(target).current.location as u32 & (LOCATION_GRAVE | LOCATION_REMOVED) != 0 {
            self.restore_temp_reason(target);
            self.card_mut(target).set_status(STATUS_DESTROY_CONFIRMED, false);
            self.group_mut(g).remove(target);
            return true;
        }
        if !self.group(g).contains(target) {
            return true;
        }
        self.returns.set_i32(0, 0);
        let mut eset = Vec::new();
        target.filter_single_continuous_effect(self, EFFECT_DESTROY_REPLACE, &mut eset, true);
        let step = if arg.battle { 10 } else { 0 };
        for peff in eset {
            self.emplace_process(units::OperationReplace { step, is_destroy: true, replace_effect: Some(peff), targets: Some(g), target: Some(target) });
        }
        true
    }

    pub fn process_destroy(&mut self, arg: &mut units::Destroy) -> bool {
        let g = arg.targets.unwrap();
        let (reason_effect, reason, reason_player) = (arg.reason_effect, arg.reason, arg.reason_player);
        match arg.step {
            0 => {
                let mut extra = CardSet::new();
                let mut indestructable_set = CardSet::new();
                let mut indestructable_effect_set: Vec<EffectId> = Vec::new();
                let set = self.group(g).clone();
                for pcard in public_order(self, &set) {
                    if !pcard.is_destructable(self) {
                        indestructable_set.insert(pcard);
                        continue;
                    }
                    let (creason, cre, crp) = {
                        let c = self.card(pcard);
                        (c.current.reason, c.current.reason_effect, c.current.reason_player)
                    };
                    if creason & (REASON_RULE | REASON_COST) == 0 {
                        let mut is_destructable = true;
                        if cre.is_none() || pcard.is_affect_by_effect(self, cre) {
                            if let Some(ie) = pcard.check_indestructable_by_effect(self, cre, crp) {
                                if reason_player as u32 != PLAYER_SELFDES {
                                    indestructable_effect_set.push(ie);
                                }
                                is_destructable = false;
                            }
                        } else {
                            is_destructable = false;
                        }
                        if !is_destructable {
                            indestructable_set.insert(pcard);
                            continue;
                        }
                    }
                    let mut eset = Vec::new();
                    pcard.filter_effect(self, EFFECT_INDESTRUCTABLE, &mut eset, true);
                    if !eset.is_empty() {
                        let mut is_destructable = true;
                        for peff in eset {
                            self.add_indes_params(pcard);
                            if peff.check_value_condition(self, 3) {
                                if reason_player as u32 != PLAYER_SELFDES {
                                    indestructable_effect_set.push(peff);
                                }
                                is_destructable = false;
                                break;
                            }
                        }
                        if !is_destructable {
                            indestructable_set.insert(pcard);
                            continue;
                        }
                    }
                    let mut eset = Vec::new();
                    pcard.filter_effect(self, EFFECT_INDESTRUCTABLE_COUNT, &mut eset, true);
                    if !eset.is_empty() {
                        let mut is_destructable = true;
                        for peff in eset {
                            if self.effect(peff).is_flag(EFFECT_FLAG_COUNT_LIMIT) {
                                if self.effect(peff).count_limit == 0 {
                                    continue;
                                }
                                self.add_indes_params(pcard);
                                if peff.check_value_condition(self, 3) {
                                    indestructable_effect_set.push(peff);
                                    is_destructable = false;
                                }
                            } else {
                                self.add_indes_params(pcard);
                                let ct = peff.get_value(self, 3) as i32;
                                if ct != 0 && self.inc_indestructable(pcard, peff) <= ct {
                                    indestructable_effect_set.push(peff);
                                    is_destructable = false;
                                }
                            }
                        }
                        if !is_destructable {
                            indestructable_set.insert(pcard);
                            continue;
                        }
                    }
                    let mut eset = Vec::new();
                    pcard.filter_effect(self, EFFECT_DESTROY_SUBSTITUTE, &mut eset, true);
                    if !eset.is_empty() {
                        let mut sub = false;
                        for peff in eset {
                            self.add_indes_params(pcard);
                            if peff.check_value_condition(self, 3) {
                                extra.insert(self.effect(peff).handler.unwrap());
                                sub = true;
                            }
                        }
                        if sub {
                            self.restore_temp_reason(pcard);
                            self.core.destroy_canceled.insert(pcard);
                            self.group_mut(g).remove(pcard);
                        }
                    }
                }
                for pcard in public_order(self, &indestructable_set) {
                    self.restore_temp_reason(pcard);
                    self.card_mut(pcard).set_status(STATUS_DESTROY_CONFIRMED, false);
                    self.group_mut(g).remove(pcard);
                }
                for rep in public_order(self, &extra) {
                    if !self.group(g).contains(rep) {
                        let c = self.card_mut(rep);
                        c.temp.reason = c.current.reason;
                        c.temp.reason_effect = c.current.reason_effect;
                        c.temp.reason_player = c.current.reason_player;
                        c.current.reason = REASON_EFFECT | REASON_DESTROY | REASON_REPLACE;
                        c.current.reason_effect = None;
                        c.current.reason_player = c.current.controler;
                        let owner = c.owner;
                        c.sendto_param.set(owner, POS_FACEUP as u8, LOCATION_GRAVE as u8, 0);
                        self.group_mut(g).insert(rep);
                    }
                }
                sort_effects_by_id(self, &mut indestructable_effect_set);
                indestructable_effect_set.dedup();
                for peffect in indestructable_effect_set {
                    peffect.dec_count(self, PLAYER_NONE);
                    self.hint_effect_owner(peffect);
                }
                self.operation_replace(EFFECT_DESTROY_REPLACE, 5, Some(g));
                false
            }
            1 => {
                let set = self.group(g).clone();
                for pcard in public_order(self, &set) {
                    self.emplace_process(units::DestroyReplace { step: 0, battle: false, targets: Some(g), target: pcard });
                }
                false
            }
            2 => {
                let canceled = std::mem::take(&mut self.core.destroy_canceled);
                for pcard in public_order(self, &canceled) {
                    self.card_mut(pcard).set_status(STATUS_DESTROY_CONFIRMED, false);
                }
                false
            }
            3 => {
                if self.group(g).is_empty() {
                    self.returns.set_i32(0, 0);
                    self.core.operated_set.clear();
                    return true;
                }
                let mut cv = self.group(g).to_vec();
                if cv.len() > 1 {
                    sort_by_less(&mut cv, |a, b| card_public_sort(self, a, b));
                }
                for pcard in cv {
                    if self.card(pcard).current.location as u32 & (LOCATION_GRAVE | LOCATION_REMOVED) != 0 {
                        self.restore_temp_reason(pcard);
                        self.group_mut(g).remove(pcard);
                        continue;
                    }
                    self.card_mut(pcard).current.reason |= REASON_DESTROY;
                    let c = self.card(pcard);
                    let ctrl = match c.overlay_target() {
                        Some(o) => self.card(o).current.controler,
                        None => c.current.controler,
                    };
                    self.core.hint_timing[ctrl as usize] |= TIMING_DESTROY;
                    let c = self.card(pcard);
                    let (re, r, rp) = (c.current.reason_effect, c.current.reason, c.current.reason_player);
                    self.raise_single_event(pcard, None, EVENT_DESTROY, re, r, rp, 0, 0);
                }
                self.adjust_instant();
                self.process_single_event();
                let set = self.group(g).clone();
                self.raise_event(set, EVENT_DESTROY, reason_effect, reason, reason_player, 0, 0);
                self.process_instant_event();
                false
            }
            4 => {
                let set = self.group(g).clone();
                let sendtargets = self.new_group(set.clone());
                for pcard in public_order(self, &set) {
                    self.card_mut(pcard).set_status(STATUS_DESTROY_CONFIRMED, false);
                    let sp = self.card(pcard).sendto_param;
                    let mut dest = sp.location as u32;
                    if dest == 0 {
                        dest = LOCATION_GRAVE;
                    }
                    if (dest == LOCATION_HAND && !pcard.is_capable_send_to_hand(self, reason_player))
                        || (dest == LOCATION_DECK && !pcard.is_capable_send_to_deck(self, reason_player))
                        || (dest == LOCATION_REMOVED && !pcard.is_removeable(self, reason_player, sp.position, reason))
                    {
                        dest = LOCATION_GRAVE;
                    }
                    self.card_mut(pcard).sendto_param.location = dest as u8;
                }
                self.operation_replace(EFFECT_SEND_REPLACE, 5, Some(sendtargets));
                self.emplace_process(units::SendTo { step: 1, reason_player, reason: reason | REASON_DESTROY, targets: Some(sendtargets), reason_effect, extra_args: None });
                false
            }
            5 => {
                let mut set = self.group(g).clone();
                set.retain(|&c| self.card(c).current.reason & REASON_REPLACE == 0);
                self.returns.set_i32(0, set.len() as i32);
                self.core.operated_set = set;
                true
            }
            10 => {
                let set = self.group(g).clone();
                for pcard in public_order(self, &set) {
                    if !pcard.is_destructable(self) {
                        self.restore_temp_reason(pcard);
                        self.card_mut(pcard).set_status(STATUS_DESTROY_CONFIRMED, false);
                        self.group_mut(g).remove(pcard);
                        continue;
                    }
                    let mut eset = Vec::new();
                    pcard.filter_effect(self, EFFECT_INDESTRUCTABLE, &mut eset, true);
                    if !eset.is_empty() {
                        let mut indes = false;
                        for peff in eset {
                            self.add_indes_params(pcard);
                            if peff.check_value_condition(self, 3) {
                                self.hint_effect_owner(peff);
                                indes = true;
                                break;
                            }
                        }
                        if indes {
                            self.restore_temp_reason(pcard);
                            self.card_mut(pcard).set_status(STATUS_DESTROY_CONFIRMED, false);
                            self.group_mut(g).remove(pcard);
                            continue;
                        }
                    }
                    let mut eset = Vec::new();
                    pcard.filter_effect(self, EFFECT_INDESTRUCTABLE_COUNT, &mut eset, true);
                    if !eset.is_empty() {
                        let mut indes = false;
                        for peff in eset {
                            if self.effect(peff).is_flag(EFFECT_FLAG_COUNT_LIMIT) {
                                if self.effect(peff).count_limit == 0 {
                                    continue;
                                }
                                self.add_indes_params(pcard);
                                if peff.check_value_condition(self, 3) {
                                    peff.dec_count(self, PLAYER_NONE);
                                    self.hint_effect_owner(peff);
                                    indes = true;
                                }
                            } else {
                                self.add_indes_params(pcard);
                                let ct = peff.get_value(self, 3) as i32;
                                if ct != 0 && self.inc_indestructable(pcard, peff) <= ct {
                                    self.hint_effect_owner(peff);
                                    indes = true;
                                }
                            }
                        }
                        if indes {
                            self.restore_temp_reason(pcard);
                            self.card_mut(pcard).set_status(STATUS_DESTROY_CONFIRMED, false);
                            self.group_mut(g).remove(pcard);
                            continue;
                        }
                    }
                    let mut eset = Vec::new();
                    pcard.filter_effect(self, EFFECT_DESTROY_SUBSTITUTE, &mut eset, true);
                    if !eset.is_empty() {
                        let mut sub = false;
                        for peff in eset {
                            self.add_indes_params(pcard);
                            if peff.check_value_condition(self, 3) {
                                let h = self.effect(peff).handler.unwrap();
                                self.core.battle_destroy_rep.insert(h);
                                sub = true;
                            }
                        }
                        if sub {
                            self.restore_temp_reason(pcard);
                            self.core.destroy_canceled.insert(pcard);
                            self.group_mut(g).remove(pcard);
                        }
                    }
                }
                if !self.group(g).is_empty() {
                    self.operation_replace(EFFECT_DESTROY_REPLACE, 12, Some(g));
                }
                false
            }
            11 => {
                let set = self.group(g).clone();
                for pcard in public_order(self, &set) {
                    self.emplace_process(units::DestroyReplace { step: 0, battle: true, targets: Some(g), target: pcard });
                }
                false
            }
            12 => {
                let canceled = std::mem::take(&mut self.core.destroy_canceled);
                for pcard in public_order(self, &canceled) {
                    self.card_mut(pcard).set_status(STATUS_DESTROY_CONFIRMED, false);
                }
                true
            }
            _ => true,
        }
    }

    pub fn process_release_replace(&mut self, arg: &mut units::ReleaseReplace) -> bool {
        let g = arg.targets.unwrap();
        let target = arg.target;
        if self.card(target).current.location as u32 & (LOCATION_GRAVE | LOCATION_REMOVED) != 0 {
            self.restore_temp_reason(target);
            self.group_mut(g).remove(target);
            return true;
        }
        if !self.group(g).contains(target) {
            return true;
        }
        if self.card(target).current.reason & REASON_RULE == 0 {
            self.returns.set_i32(0, 0);
            let mut eset = Vec::new();
            target.filter_single_continuous_effect(self, EFFECT_RELEASE_REPLACE, &mut eset, true);
            for peff in eset {
                self.emplace_process(units::OperationReplace { step: 0, is_destroy: false, replace_effect: Some(peff), targets: Some(g), target: Some(target) });
            }
        }
        true
    }

    pub fn process_release(&mut self, arg: &mut units::Release) -> bool {
        let g = arg.targets.unwrap();
        let (reason_effect, reason, reason_player) = (arg.reason_effect, arg.reason, arg.reason_player);
        match arg.step {
            0 => {
                let set = self.group(g).clone();
                for pcard in public_order(self, &set) {
                    let (st, rc, cr, cre) = {
                        let c = self.card(pcard);
                        (c.get_status(STATUS_SUMMONING | STATUS_SPSUMMON_STEP), c.current.reason_card, c.current.reason, c.current.reason_effect)
                    };
                    if st != 0
                        || (reason & REASON_SUMMON != 0 && !pcard.is_releasable_by_summon(self, reason_player, rc.unwrap()))
                        || (cr & (REASON_RULE | REASON_SUMMON | REASON_COST) == 0
                            && (!pcard.is_affect_by_effect(self, cre) || !pcard.is_releasable_by_nonsummon(self, reason_player, reason)))
                    {
                        self.restore_temp_reason(pcard);
                        self.group_mut(g).remove(pcard);
                        continue;
                    }
                }
                if reason & REASON_RULE != 0 {
                    return false;
                }
                self.operation_replace(EFFECT_RELEASE_REPLACE, 5, Some(g));
                false
            }
            1 => {
                let set = self.group(g).clone();
                for pcard in public_order(self, &set) {
                    self.emplace_process(units::ReleaseReplace { step: 0, targets: Some(g), target: pcard });
                }
                false
            }
            2 => {
                if self.group(g).is_empty() {
                    self.returns.set_i32(0, 0);
                    self.core.operated_set.clear();
                    return true;
                }
                let mut cv = self.group(g).to_vec();
                if cv.len() > 1 {
                    sort_by_less(&mut cv, |a, b| card_public_sort(self, a, b));
                }
                for pcard in cv {
                    if self.card(pcard).current.location as u32 & (LOCATION_GRAVE | LOCATION_REMOVED) != 0 {
                        self.restore_temp_reason(pcard);
                        self.group_mut(g).remove(pcard);
                        continue;
                    }
                    self.card_mut(pcard).current.reason |= REASON_RELEASE;
                }
                self.adjust_instant();
                false
            }
            3 => {
                for peffect in self.core.dec_count_reserve.clone() {
                    let h = peffect.get_handler(self);
                    let code = self.card(h).data.code;
                    self.hint_card(code);
                }
                let set = self.group(g).clone();
                let sendtargets = self.new_group(set);
                self.operation_replace(EFFECT_SEND_REPLACE, 5, Some(sendtargets));
                self.emplace_process(units::SendTo { step: 1, reason_player, reason: reason | REASON_RELEASE, targets: Some(sendtargets), reason_effect, extra_args: None });
                false
            }
            4 => {
                for peffect in std::mem::take(&mut self.core.dec_count_reserve) {
                    peffect.dec_count(self, PLAYER_NONE);
                }
                let set = self.group(g).clone();
                self.returns.set_i32(0, set.len() as i32);
                self.core.operated_set = set;
                true
            }
            _ => true,
        }
    }

    pub fn process_send_to_replace(&mut self, arg: &mut units::SendToReplace) -> bool {
        let g = arg.targets.unwrap();
        let target = arg.target;
        let sp = self.card(target).sendto_param;
        if !self.group(g).contains(target) {
            return true;
        }
        let c = self.card(target);
        if c.current.location == sp.location && c.current.controler == sp.playerid {
            self.restore_temp_reason(target);
            self.group_mut(g).remove(target);
            return true;
        }
        if c.current.reason & REASON_RULE == 0 {
            self.returns.set_i32(0, 0);
            let mut eset = Vec::new();
            target.filter_single_continuous_effect(self, EFFECT_SEND_REPLACE, &mut eset, true);
            for peff in eset {
                self.emplace_process(units::OperationReplace { step: 0, is_destroy: false, replace_effect: Some(peff), targets: Some(g), target: Some(target) });
            }
        }
        true
    }

    /// The MSG_MOVE of a card the sending moves (`card` leaves its Xyz monster first).
    fn send_to_move(&mut self, ex: &mut SendToExargs, pcard: CardId, playerid: u8, dest: u8, seq: u8, position: Option<u32>) {
        let code = self.card(pcard).data.code;
        let from = pcard.get_info_location(self);
        if let Some(o) = self.card(pcard).overlay_target() {
            ex.detach.insert(o);
            o.xyz_remove(self, pcard);
        }
        self.move_card(playerid, pcard, dest, seq, false);
        // (`sendto_param` after `move_card`: `add_card` sets the position in the hand and the deck)
        let position = position.unwrap_or(self.card(pcard).sendto_param.position as u32);
        self.card_mut(pcard).current.position = position;
        let to = pcard.get_info_location(self);
        let r = self.card(pcard).current.reason;
        let m = self.new_message(MSG_MOVE);
        self.msg(m).u32(code).loc(from).loc(to).u32(r);
    }

    pub fn process_send_to(&mut self, arg: &mut units::SendTo) -> bool {
        let g = arg.targets.unwrap();
        let (reason_effect, reason, reason_player) = (arg.reason_effect, arg.reason, arg.reason_player);
        match arg.step {
            0 => {
                let set = self.group(g).clone();
                let crp = self.core.reason_player;
                for pcard in public_order(self, &set) {
                    let (dest, pos, st, cr, cre) = {
                        let c = self.card(pcard);
                        (c.sendto_param.location as u32, c.sendto_param.position, c.get_status(STATUS_SUMMONING | STATUS_SPSUMMON_STEP), c.current.reason, c.current.reason_effect)
                    };
                    if reason & REASON_RULE == 0
                        && (st != 0
                            || (cr & (REASON_COST | REASON_SUMMON | REASON_MATERIAL) == 0 && !pcard.is_affect_by_effect(self, cre))
                            || (dest == LOCATION_HAND && !pcard.is_capable_send_to_hand(self, crp))
                            || (dest == LOCATION_DECK && !pcard.is_capable_send_to_deck(self, crp))
                            || (dest == LOCATION_REMOVED && !pcard.is_removeable(self, crp, pos, reason))
                            || (dest == LOCATION_GRAVE && !pcard.is_capable_send_to_grave(self, crp, reason))
                            || (dest == LOCATION_EXTRA && !pcard.is_capable_send_to_extra(self, crp)))
                    {
                        self.restore_temp_reason(pcard);
                        self.group_mut(g).remove(pcard);
                        continue;
                    }
                }
                if reason & REASON_RULE != 0 {
                    return false;
                }
                self.operation_replace(EFFECT_SEND_REPLACE, 5, Some(g));
                false
            }
            1 => {
                let set = self.group(g).clone();
                for pcard in public_order(self, &set) {
                    self.emplace_process(units::SendToReplace { step: 0, targets: Some(g), target: pcard });
                }
                false
            }
            2 => {
                if self.group(g).is_empty() {
                    self.returns.set_i32(0, 0);
                    self.core.operated_set.clear();
                    return true;
                }
                let mut leave_p = CardSet::new();
                let set = self.group(g).clone();
                for pcard in public_order(self, &set) {
                    let c = self.card(pcard);
                    let onfield = c.current.location as u32 & LOCATION_ONFIELD != 0;
                    if onfield && !c.is_status(STATUS_SUMMON_DISABLED) && !c.is_status(STATUS_ACTIVATE_DISABLED) {
                        let (re, r, rp) = (c.current.reason_effect, c.current.reason, c.current.reason_player);
                        self.raise_single_event(pcard, None, EVENT_LEAVE_FIELD_P, re, r, rp, 0, 0);
                        leave_p.insert(pcard);
                    }
                    if onfield {
                        let c = self.card(pcard);
                        if c.current.position & POS_FACEUP != 0 {
                            let in_mzone = c.current.location as u32 & LOCATION_MZONE != 0;
                            let code = pcard.get_code(self);
                            let code2 = pcard.get_another_code(self);
                            let typ = pcard.get_type(self, None, 0, PLAYER_NONE as u8);
                            {
                                let c = self.card_mut(pcard);
                                c.previous.code = code;
                                c.previous.code2 = code2;
                                c.previous.type_ = typ;
                            }
                            if in_mzone {
                                let level = pcard.get_level(self);
                                let rank = pcard.get_rank(self);
                                let attribute = pcard.get_attribute(self, None, 0, PLAYER_NONE as u8);
                                let race = pcard.get_race(self, None, 0, PLAYER_NONE as u8);
                                let attack = pcard.get_attack(self);
                                let defense = pcard.get_defense(self);
                                let c = self.card_mut(pcard);
                                c.previous.level = level;
                                c.previous.rank = rank;
                                c.previous.attribute = attribute;
                                c.previous.race = race;
                                c.previous.attack = attack;
                                c.previous.defense = defense;
                            }
                        } else {
                            let mut eset = Vec::new();
                            pcard.filter_effect(self, EFFECT_ADD_CODE, &mut eset, true);
                            let code2 = match eset.last() {
                                Some(&e) => e.get_value_card(self, Some(pcard), 0) as u32,
                                None => 0,
                            };
                            let c = self.card_mut(pcard);
                            c.previous.code = if c.data.alias != 0 && eset.is_empty() { c.data.alias } else { c.data.code };
                            c.previous.code2 = code2;
                            c.previous.type_ = c.data.type_;
                            c.previous.level = c.data.level;
                            c.previous.rank = c.data.level;
                            c.previous.attribute = c.data.attribute;
                            c.previous.race = c.data.race;
                            c.previous.attack = c.data.attack;
                            c.previous.defense = c.data.defense;
                        }
                        let mut eset = Vec::new();
                        pcard.filter_effect(self, EFFECT_ADD_SETCODE, &mut eset, true);
                        self.card_mut(pcard).previous.setcodes.clear();
                        for eff in eset {
                            let v = eff.get_value_card(self, Some(pcard), 0) as u16;
                            self.card_mut(pcard).previous.setcodes.insert(v);
                        }
                    }
                }
                if !leave_p.is_empty() {
                    self.raise_event(leave_p, EVENT_LEAVE_FIELD_P, reason_effect, reason, reason_player, 0, 0);
                }
                self.process_single_event();
                self.process_instant_event();
                false
            }
            3 => {
                let set = self.group(g).clone();
                let order = public_order(self, &set);
                for &pcard in &order {
                    pcard.enable_field_effect(self, false);
                }
                self.adjust_disable_check_list();
                for &pcard in &order {
                    let mut dest = self.card(pcard).sendto_param.location as u32;
                    let mut redirect: u32 = 0;
                    let mut redirect_seq: u32 = 0;
                    let mut check_cb = false;
                    if dest & LOCATION_GRAVE != 0 && pcard.is_affected_by_effect(self, EFFECT_TO_GRAVE_REDIRECT_CB).is_some() {
                        check_cb = true;
                    }
                    let c = self.card(pcard);
                    if c.current.location as u32 & LOCATION_ONFIELD != 0 && !c.is_status(STATUS_SUMMON_DISABLED) && !c.is_status(STATUS_ACTIVATE_DISABLED) {
                        let r = c.current.reason;
                        redirect = pcard.leave_field_redirect(self, r);
                        redirect_seq = redirect >> 16;
                        redirect &= 0xffff;
                    }
                    if redirect != 0 {
                        let c = self.card_mut(pcard);
                        c.current.reason &= !REASON_TEMPORARY;
                        c.current.reason |= REASON_REDIRECT;
                        c.sendto_param.location = redirect as u8;
                        c.sendto_param.sequence = redirect_seq as u8;
                        dest = redirect;
                        if dest == LOCATION_REMOVED {
                            let p = c.sendto_param.position as u32;
                            if p & POS_FACEDOWN_ATTACK != 0 {
                                c.sendto_param.position = ((p & !POS_FACEDOWN_ATTACK) | POS_FACEUP_ATTACK) as u8;
                            }
                            let p = c.sendto_param.position as u32;
                            if p & POS_FACEDOWN_DEFENSE != 0 {
                                c.sendto_param.position = ((p & !POS_FACEDOWN_DEFENSE) | POS_FACEUP_DEFENSE) as u8;
                            }
                        }
                    }
                    let r = self.card(pcard).current.reason;
                    let mut redirect = pcard.destination_redirect(self, dest as u8, r);
                    if redirect != 0 {
                        redirect_seq = redirect >> 16;
                        redirect &= 0xffff;
                    }
                    let c = self.card_mut(pcard);
                    if redirect != 0 && c.current.location as u32 != redirect {
                        c.current.reason |= REASON_REDIRECT;
                        c.sendto_param.location = redirect as u8;
                        c.sendto_param.sequence = redirect_seq as u8;
                    }
                    if check_cb {
                        c.sendto_param.playerid |= 0x1 << 4;
                    }
                }
                false
            }
            4 => {
                let mut param = Box::<SendToExargs>::default();
                param.check_decktop_visibility = [false, false];
                param.cv = self.group(g).to_vec();
                if param.cv.len() > 1 {
                    sort_by_less(&mut param.cv, |a, b| card_public_sort(self, a, b));
                }
                if self.core.global_flag & GLOBALFLAG_DECK_REVERSE_CHECK != 0 {
                    let d0 = self.player[0].list_main.len() as i32 - 1;
                    let mut s0 = d0;
                    let d1 = self.player[1].list_main.len() as i32 - 1;
                    let mut s1 = d1;
                    for &pcard in &param.cv {
                        let c = self.card(pcard);
                        if c.current.location as u32 != LOCATION_DECK {
                            continue;
                        }
                        if c.current.controler == 0 && c.current.sequence == s0 as u32 {
                            s0 -= 1;
                        }
                        if c.current.controler == 1 && c.current.sequence == s1 as u32 {
                            s1 -= 1;
                        }
                    }
                    for (p, d, s) in [(0u8, d0, s0), (1u8, d1, s1)] {
                        if s != d && s > 0 {
                            let ptop = self.player[p as usize].list_main[s as usize];
                            let c = self.card(ptop);
                            if self.core.deck_reversed || c.current.position == POS_FACEUP_DEFENSE {
                                let (code, pos) = (c.data.code, c.current.position);
                                let m = self.new_message(MSG_DECK_TOP);
                                self.msg(m).u8(p).u32((d - s) as u32).u32(code).u32(pos);
                            }
                        }
                    }
                }
                param.cvit = 0;
                arg.extra_args = Some(param);
                false
            }
            5 => {
                let param = arg.extra_args.as_mut().unwrap();
                if param.cvit == param.cv.len() {
                    arg.step = 8;
                    return false;
                }
                let pcard = param.cv[param.cvit];
                param.predirect = None;
                let check_cb = self.card(pcard).sendto_param.playerid >> 4;
                if check_cb != 0 {
                    param.predirect = pcard.is_affected_by_effect(self, EFFECT_TO_GRAVE_REDIRECT_CB);
                }
                pcard.enable_field_effect(self, false);
                if self.card(pcard).data.type_ & TYPE_TOKEN != 0 {
                    let code = self.card(pcard).data.code;
                    let from = pcard.get_info_location(self);
                    let r = self.card(pcard).current.reason;
                    let m = self.new_message(MSG_MOVE);
                    self.msg(m).u32(code).loc(from).loc(LocInfo::default()).u32(r);
                    if !self.core.current_chain.is_empty() {
                        self.core.just_sent_cards.insert(pcard);
                    }
                    let fid = self.infos.field_id;
                    self.infos.field_id += 1;
                    {
                        let c = self.card_mut(pcard);
                        c.previous.controler = c.current.controler;
                        c.previous.location = c.current.location;
                        c.previous.sequence = c.current.sequence;
                        c.previous.position = c.current.position;
                        c.previous.pzone = c.current.pzone;
                        c.current.reason &= !REASON_TEMPORARY;
                        c.fieldid = fid;
                        c.fieldid_r = fid;
                    }
                    pcard.reset(self, RESET_LEAVE, RESET_EVENT);
                    pcard.clear_relate_effect(self);
                    self.remove_card(pcard);
                    let param = arg.extra_args.as_mut().unwrap();
                    param.leave_field.insert(pcard);
                    param.cvit += 1;
                    arg.step = 4;
                    self.card_mut(pcard).set_status(STATUS_LEAVE_CONFIRMED, false);
                    return false;
                }
                let ctrl = self.card(pcard).current.controler;
                let predirect = arg.extra_args.as_ref().unwrap().predirect;
                if predirect.is_some() && self.get_useable_count(Some(pcard), ctrl, LOCATION_SZONE as u8, ctrl, LocationReason::ToField, 0xff, None) > 0 {
                    self.emplace_process(units::SelectEffectYesNo { step: 0, playerid: ctrl, pcard: Some(pcard), description: 97 });
                } else {
                    self.returns.set_i32(0, 0);
                }
                false
            }
            6 => {
                if self.returns.at_i32(0) != 0 {
                    return false;
                }
                let mut param = arg.extra_args.take().unwrap();
                let pcard = param.cv[param.cvit];
                let c = self.card(pcard);
                let oloc = c.current.location;
                let playerid = c.sendto_param.playerid & 0x7;
                let dest = c.sendto_param.location;
                let seq = c.sendto_param.sequence;
                let control_player = match c.overlay_target() {
                    Some(o) => self.card(o).current.controler,
                    None => c.current.controler,
                };
                let cp = control_player as usize;
                match dest as u32 {
                    LOCATION_GRAVE => self.core.hint_timing[cp] |= TIMING_TOGRAVE,
                    LOCATION_HAND => {
                        self.card_mut(pcard).set_status(STATUS_PROC_COMPLETE, false);
                        self.core.hint_timing[cp] |= TIMING_TOHAND;
                    }
                    LOCATION_DECK => {
                        self.card_mut(pcard).set_status(STATUS_PROC_COMPLETE, false);
                        self.core.hint_timing[cp] |= TIMING_TODECK;
                    }
                    LOCATION_REMOVED => self.core.hint_timing[cp] |= TIMING_REMOVE,
                    _ => {}
                }
                let c = self.card(pcard);
                if c.current.controler != playerid || c.current.location != dest {
                    self.send_to_move(&mut param, pcard, playerid, dest, seq, None);
                }
                let c = self.card(pcard);
                if c.current.location as u32 == LOCATION_DECK && (self.core.deck_reversed || c.current.position == POS_FACEUP_DEFENSE) {
                    param.check_decktop_visibility[cp] = true;
                }
                self.card_mut(pcard).set_status(STATUS_LEAVE_CONFIRMED, false);
                if self.card(pcard).status & (STATUS_SUMMON_DISABLED | STATUS_ACTIVATE_DISABLED) != 0 {
                    let c = self.card_mut(pcard);
                    c.set_status(STATUS_SUMMON_DISABLED | STATUS_ACTIVATE_DISABLED, false);
                    c.previous.location = 0;
                } else if oloc as u32 & LOCATION_ONFIELD != 0 {
                    pcard.reset(self, RESET_LEAVE, RESET_EVENT);
                    param.leave_field.insert(pcard);
                } else if oloc as u32 == LOCATION_GRAVE {
                    param.leave_grave.insert(pcard);
                }
                if self.card(pcard).previous.location as u32 == LOCATION_OVERLAY {
                    self.card_mut(pcard).previous.controler = control_player;
                }
                param.cvit += 1;
                arg.extra_args = Some(param);
                arg.step = 4;
                false
            }
            7 => {
                // crystal beast redirection
                let param = arg.extra_args.as_ref().unwrap();
                let pcard = param.cv[param.cvit];
                let ctrl = self.card(pcard).current.controler;
                let mut flag = 0u32;
                self.get_useable_count(Some(pcard), ctrl, LOCATION_SZONE as u8, ctrl, LocationReason::ToField, 0xff, Some(&mut flag));
                let flag = ((flag << 8) & 0xff00) | 0xffffe0ff;
                let code = self.card(pcard).data.code;
                let m = self.new_message(MSG_HINT);
                self.msg(m).u8(HINT_SELECTMSG).u8(ctrl).u64(code);
                self.emplace_process(units::SelectPlace { step: 0, playerid: ctrl, count: 1, flag, disable_field: false });
                false
            }
            8 => {
                let mut param = arg.extra_args.take().unwrap();
                let pcard = param.cv[param.cvit];
                let oloc = self.card(pcard).current.location;
                let seq = self.returns.at_i8(2) as u8;
                let ctrl = self.card(pcard).current.controler;
                self.send_to_move(&mut param, pcard, ctrl, LOCATION_SZONE as u8, seq, Some(POS_FACEUP));
                self.card_mut(pcard).set_status(STATUS_LEAVE_CONFIRMED, false);
                if self.card(pcard).status & (STATUS_SUMMON_DISABLED | STATUS_ACTIVATE_DISABLED) != 0 {
                    let c = self.card_mut(pcard);
                    c.set_status(STATUS_SUMMON_DISABLED | STATUS_ACTIVATE_DISABLED, false);
                    c.previous.location = 0;
                } else if oloc as u32 & LOCATION_ONFIELD != 0 {
                    pcard.reset(self, RESET_LEAVE + RESET_MSCHANGE, RESET_EVENT);
                    pcard.clear_card_target(self);
                    param.leave_field.insert(pcard);
                }
                let predirect = param.predirect.unwrap();
                if self.effect(predirect).operation.is_some() {
                    let c = self.card(pcard);
                    let e = Event {
                        trigger_card: None,
                        event_cards: Some(self.group(g).clone()),
                        event_code: 0,
                        reason_effect,
                        reason: c.current.reason,
                        reason_player: c.current.controler,
                        event_player: c.current.controler,
                        event_value: 0,
                        global_id: 0,
                    };
                    self.core.sub_solving_event.push_back(e);
                    self.emplace_process(units::ExecuteOperation { step: 0, triggering_player: ctrl, shuffle_check_was_disabled: false, triggering_effect: predirect });
                }
                param.cvit += 1;
                arg.extra_args = Some(param);
                arg.step = 4;
                false
            }
            9 => {
                let mut param = arg.extra_args.take().unwrap();
                if self.core.global_flag & GLOBALFLAG_DECK_REVERSE_CHECK != 0 {
                    for p in 0..2u8 {
                        if !param.check_decktop_visibility[p as usize] {
                            continue;
                        }
                        let Some(&ptop) = self.player[p as usize].list_main.last() else {
                            continue;
                        };
                        let c = self.card(ptop);
                        if !self.core.deck_reversed && c.current.position != POS_FACEUP_DEFENSE {
                            continue;
                        }
                        let (code, pos) = (c.data.code, c.current.position);
                        let m = self.new_message(MSG_DECK_TOP);
                        self.msg(m).u8(p).u32(0u32).u32(code).u32(pos);
                    }
                }
                let set = self.group(g).clone();
                for pcard in public_order(self, &set) {
                    let c = self.card(pcard);
                    let is_token = c.data.type_ & TYPE_TOKEN != 0;
                    let nloc = c.current.location as u32;
                    let to_removed_token = is_token && c.sendto_param.location as u32 == LOCATION_REMOVED;
                    let temporary = c.current.reason & REASON_TEMPORARY != 0;
                    if !is_token {
                        pcard.enable_field_effect(self, true);
                    }
                    if nloc == LOCATION_HAND {
                        pcard.reset(self, RESET_TOHAND, RESET_EVENT);
                    }
                    if nloc == LOCATION_DECK || nloc == LOCATION_EXTRA {
                        pcard.reset(self, RESET_TODECK, RESET_EVENT);
                    }
                    if nloc == LOCATION_GRAVE {
                        pcard.reset(self, RESET_TOGRAVE, RESET_EVENT);
                    }
                    if nloc == LOCATION_REMOVED || to_removed_token {
                        if temporary {
                            pcard.reset(self, RESET_TEMP_REMOVE, RESET_EVENT);
                        } else {
                            pcard.reset(self, RESET_REMOVE, RESET_EVENT);
                        }
                    }
                    pcard.refresh_disable_status(self);
                }
                // (in public order: the order the single events are handled in shows, e.g.
                // which equipped monster is destroyed first when two equip cards leave)
                for pcard in public_order(self, &param.leave_field) {
                    let c = self.card(pcard);
                    let (re, r, rp) = (c.current.reason_effect, c.current.reason, c.current.reason_player);
                    self.raise_single_event(pcard, None, EVENT_LEAVE_FIELD, re, r, rp, 0, 0);
                }
                for pcard in public_order(self, &param.leave_grave) {
                    let c = self.card(pcard);
                    let (re, r, rp) = (c.current.reason_effect, c.current.reason, c.current.reason_player);
                    self.raise_single_event(pcard, None, EVENT_LEAVE_GRAVE, re, r, rp, 0, 0);
                }
                let detach_event = self.core.global_flag & GLOBALFLAG_DETACH_EVENT != 0 && !param.detach.is_empty();
                if detach_event {
                    for pcard in public_order(self, &param.detach) {
                        if self.card(pcard).current.location as u32 & LOCATION_MZONE != 0 {
                            self.raise_single_event(pcard, None, EVENT_DETACH_MATERIAL, reason_effect, reason, reason_player, 0, 0);
                        }
                    }
                }
                self.process_single_event();
                if !param.leave_field.is_empty() {
                    let s = std::mem::take(&mut param.leave_field);
                    self.raise_event(s, EVENT_LEAVE_FIELD, reason_effect, reason, reason_player, 0, 0);
                }
                if !param.leave_grave.is_empty() {
                    let s = std::mem::take(&mut param.leave_grave);
                    self.raise_event(s, EVENT_LEAVE_GRAVE, reason_effect, reason, reason_player, 0, 0);
                }
                if detach_event {
                    let s = std::mem::take(&mut param.detach);
                    self.raise_event(s, EVENT_DETACH_MATERIAL, reason_effect, reason, reason_player, 0, 0);
                }
                arg.extra_args = Some(param);
                self.process_instant_event();
                self.adjust_instant();
                false
            }
            10 => {
                let mut tohand = CardSet::new();
                let mut todeck = CardSet::new();
                let mut tograve = CardSet::new();
                let mut remove = CardSet::new();
                let mut discard = CardSet::new();
                let mut released = CardSet::new();
                let mut destroyed = CardSet::new();
                let mut equipings = CardSet::new();
                let mut overlays = CardSet::new();
                let set = self.group(g).clone();
                for pcard in public_order(self, &set) {
                    let nloc = self.card(pcard).current.location as u32;
                    if self.card(pcard).equiping_target().is_some() {
                        pcard.unequip(self);
                    }
                    let eqs = self.card(pcard).equiping_cards.clone();
                    if !eqs.is_empty() {
                        for equipc in public_order(self, &eqs) {
                            equipc.unequip(self);
                            if self.card(equipc).current.location as u32 == LOCATION_SZONE {
                                equipings.insert(equipc);
                            }
                        }
                    }
                    pcard.clear_card_target(self);
                    let ev = |d: &Duel| {
                        let c = d.card(pcard);
                        (c.current.reason_effect, c.current.reason, c.current.reason_player)
                    };
                    if nloc == LOCATION_HAND {
                        if self.card(pcard).owner != self.card(pcard).current.controler {
                            let de = self.new_effect();
                            let e = self.effect_mut(de);
                            e.owner = Some(pcard);
                            e.code = 0;
                            e.type_ = EFFECT_TYPE_SINGLE as u16;
                            e.flag[0] = EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_CLIENT_HINT;
                            e.description = 67;
                            e.reset_flag = RESET_EVENT + 0x1fe0000;
                            pcard.add_effect(self, de);
                        }
                        if !self.core.current_chain.is_empty() {
                            // Added to the hand by a currently resolving effect
                            let de = self.new_effect();
                            let e = self.effect_mut(de);
                            e.owner = Some(pcard);
                            e.code = 0;
                            e.type_ = EFFECT_TYPE_SINGLE as u16;
                            e.flag[0] = EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_CLIENT_HINT;
                            e.description = 225;
                            e.reset_flag = RESET_EVENT
                                | RESET_TOFIELD
                                | RESET_LEAVE
                                | RESET_TODECK
                                | RESET_TOHAND
                                | RESET_TEMP_REMOVE
                                | RESET_REMOVE
                                | RESET_TOGRAVE
                                | RESET_TURN_SET
                                | RESET_CHAIN;
                            pcard.add_effect(self, de);
                        }
                        tohand.insert(pcard);
                        let (re, r, rp) = ev(self);
                        self.raise_single_event(pcard, None, EVENT_TO_HAND, re, r, rp, 0, 0);
                    }
                    if nloc == LOCATION_DECK || nloc == LOCATION_EXTRA {
                        todeck.insert(pcard);
                        let (re, r, rp) = ev(self);
                        self.raise_single_event(pcard, None, EVENT_TO_DECK, re, r, rp, 0, 0);
                    }
                    let (re, r, rp) = ev(self);
                    if nloc == LOCATION_GRAVE && r & REASON_RETURN == 0 {
                        tograve.insert(pcard);
                        self.raise_single_event(pcard, None, EVENT_TO_GRAVE, re, r, rp, 0, 0);
                    }
                    let c = self.card(pcard);
                    if nloc == LOCATION_REMOVED || (c.data.type_ & TYPE_TOKEN != 0 && c.sendto_param.location as u32 == LOCATION_REMOVED) {
                        remove.insert(pcard);
                        self.raise_single_event(pcard, None, EVENT_REMOVE, re, r, rp, 0, 0);
                    }
                    if r & REASON_DISCARD != 0 {
                        discard.insert(pcard);
                        self.raise_single_event(pcard, None, EVENT_DISCARD, re, r, rp, 0, 0);
                    }
                    if r & REASON_RELEASE != 0 {
                        released.insert(pcard);
                        self.raise_single_event(pcard, None, EVENT_RELEASE, re, r, rp, 0, 0);
                    }
                    // non-battle destroy
                    if r & REASON_DESTROY != 0 && r & REASON_BATTLE == 0 {
                        destroyed.insert(pcard);
                        self.raise_single_event(pcard, None, EVENT_DESTROYED, re, r, rp, 0, 0);
                    }
                    for &mcard in &self.card(pcard).xyz_materials {
                        overlays.insert(mcard);
                    }
                    self.raise_single_event(pcard, None, EVENT_MOVE, re, r, rp, 0, 0);
                }
                for (s, code) in [
                    (tohand, EVENT_TO_HAND),
                    (todeck, EVENT_TO_DECK),
                    (tograve, EVENT_TO_GRAVE),
                    (remove, EVENT_REMOVE),
                    (discard, EVENT_DISCARD),
                    (released, EVENT_RELEASE),
                    (destroyed, EVENT_DESTROYED),
                ] {
                    if !s.is_empty() {
                        self.raise_event(s, code, reason_effect, reason, reason_player, 0, 0);
                    }
                }
                let set = self.group(g).clone();
                self.raise_event(set, EVENT_MOVE, reason_effect, reason, reason_player, 0, 0);
                self.process_single_event();
                self.process_instant_event();
                if !equipings.is_empty() {
                    self.destroy(equipings, None, REASON_RULE + REASON_LOST_TARGET, PLAYER_NONE as u8, PLAYER_NONE as u8, 0, 0);
                }
                if !overlays.is_empty() {
                    self.send_to(overlays, None, REASON_RULE + REASON_LOST_TARGET, PLAYER_NONE as u8, PLAYER_NONE as u8, LOCATION_GRAVE as u16, 0, POS_FACEUP as u8, false);
                }
                self.adjust_instant();
                false
            }
            11 => {
                let set = self.group(g).clone();
                self.returns.set_i32(0, set.len() as i32);
                self.core.operated_set = set;
                true
            }
            _ => true,
        }
    }
}
