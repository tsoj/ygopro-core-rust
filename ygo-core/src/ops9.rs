//! Operations (`operations.cpp` part 9: selecting releases and tributes, coins and dice).

use crate::card::public_order;
use crate::constants::*;
use crate::duel::{Duel, Event};
use crate::types::*;
use crate::units::{self, LocationReason};

/// `std::set_difference` of two ranges sorted by card id (with multiplicity).
fn set_difference(a: &[CardId], b: &[CardId]) -> Vec<CardId> {
    let mut out = Vec::new();
    let mut j = 0;
    for &x in a {
        while j < b.len() && b[j] < x {
            j += 1;
        }
        if j < b.len() && b[j] == x {
            j += 1;
        } else {
            out.push(x);
        }
    }
    out
}

impl Duel {
    fn hint_select_release(&mut self, playerid: u8) {
        let m = self.new_message(MSG_HINT);
        self.msg(m).u8(HINT_SELECTMSG).u8(playerid).u64(500u64);
    }

    pub fn process_select_release(&mut self, arg: &mut units::SelectRelease) -> bool {
        let (playerid, cancelable, max, check_field, to_check, toplayer) = (arg.playerid, arg.cancelable, arg.max, arg.check_field, arg.to_check, arg.toplayer);
        let mut min = arg.min;
        let mut zone = arg.zone;
        match arg.step {
            0 => {
                if check_field {
                    zone &= (0x1f & self.get_forced_zones(to_check, toplayer, LOCATION_MZONE as u8, playerid as u32, LocationReason::ToField)) as u8;
                    let ct = self.get_useable_count(to_check, toplayer, LOCATION_MZONE as u8, playerid, LocationReason::ToField, zone as u32, None);
                    if ct < min as i32 {
                        let mut must = CardSet::new();
                        let rc = self.core.release_cards.clone();
                        for pcard in public_order(self, &rc) {
                            let c = self.card(pcard);
                            if c.current.location as u32 == LOCATION_MZONE && c.current.controler == toplayer && ((zone as u32).wrapping_shr(c.current.sequence) & 1) != 0 {
                                must.insert(pcard);
                            }
                        }
                        arg.must_choose_one = Some(must);
                    }
                }
                self.core.operated_set.clear();
                self.return_cards.clear();
                let (rc, rce, rco) = (self.core.release_cards.len(), self.core.release_cards_ex.len(), self.core.release_cards_ex_oneof.len());
                // if all the available cards are equal to the minimum, then the selection will always use all the forced cards without needing to check
                let allminimum = rco <= 1 && (rce + rc + rco) as i32 == min as i32;
                // if only must use cards are available, then the selection will always be correct
                let allmust = rce as i32 >= max as i32 || (rc + rco) == 0;
                // only self cards available, no need for special check
                let onlyself = (rce + rco) == 0;
                if (!allminimum && !allmust && !onlyself) || arg.must_choose_one.is_some() {
                    arg.step = 1;
                    return false;
                }
                let mut sc: Vec<CardId> = self.core.release_cards_ex.to_vec();
                if (rce as i32) < max as i32 {
                    let mut v = self.core.release_cards.to_vec();
                    v.extend(sc);
                    sc = v;
                    let mut v = self.core.release_cards_ex_oneof.to_vec();
                    v.extend(sc);
                    sc = v;
                }
                self.core.select_cards = sc;
                if allmust {
                    min = rce as u16;
                }
                self.hint_select_release(playerid);
                self.emplace_process(units::SelectCard { step: 0, playerid, cancelable, min: min as u8, max: max as u8 });
                false
            }
            1 => {
                if self.return_cards.canceled {
                    return true;
                }
                let count = self.return_cards.list.len() as i32;
                for c in self.return_cards.list.clone() {
                    self.core.operated_set.insert(c);
                }
                if min as i32 - count > 0 {
                    // something wrong happened when selecting
                    self.return_cards.clear();
                    self.return_cards.canceled = true;
                } else if let Some(first) = self.core.release_cards_ex_oneof.first() {
                    let peffect = first.is_affected_by_effect(self, EFFECT_EXTRA_RELEASE_NONSUM);
                    self.core.dec_count_reserve.push(peffect.unwrap());
                }
                true
            }
            2 => {
                self.core.unselect_cards = self.core.operated_set.to_vec();
                let mut finishable = self.core.operated_set.len() as i32 >= min as i32;
                let mut must_chosen = arg.must_choose_one.is_none();
                let mut to_select: u32 = 0;
                let rce = self.core.release_cards_ex.clone();
                for pcard in public_order(self, &rce) {
                    if !self.core.operated_set.contains(pcard) {
                        finishable = false;
                        to_select += 1;
                    }
                }
                if !must_chosen {
                    let must = arg.must_choose_one.as_ref().unwrap();
                    for pcard in public_order(self, &self.core.operated_set) {
                        if must.contains(pcard) {
                            must_chosen = true;
                            break;
                        }
                    }
                }
                let mut curmax = max as i32 - self.core.operated_set.len() as i32;
                if !must_chosen {
                    curmax -= 1;
                }
                curmax -= to_select as i32;
                finishable = finishable && must_chosen;
                let mut diff: Vec<CardId> = self.core.release_cards_ex.to_vec();
                if curmax <= 0 {
                    if !must_chosen {
                        let must = arg.must_choose_one.as_ref().unwrap().to_vec();
                        if curmax == 0 {
                            let mut v = must;
                            v.extend(diff);
                            diff = v;
                        } else {
                            diff = must;
                        }
                    }
                } else {
                    let mut v = self.core.release_cards.to_vec();
                    v.extend(diff);
                    diff = v;
                    if arg.extra_release_nonsum_effect.is_none() {
                        let mut v = self.core.release_cards_ex_oneof.to_vec();
                        v.extend(diff);
                        diff = v;
                    }
                }
                diff.sort();
                self.core.select_cards = set_difference(&diff, &self.core.unselect_cards);
                self.hint_select_release(playerid);
                let canc = finishable || self.core.operated_set.is_empty();
                self.emplace_process(units::SelectUnselectCard { step: 0, playerid, cancelable: canc, min: min as u8, max: max as u8, finishable });
                false
            }
            3 => {
                if self.return_cards.canceled && (self.core.operated_set.is_empty() || self.core.operated_set.len() as i32 >= min as i32) {
                    if let Some(e) = arg.extra_release_nonsum_effect {
                        self.core.dec_count_reserve.push(e);
                    }
                    self.return_cards.list = self.core.operated_set.to_vec();
                    if !self.core.operated_set.is_empty() {
                        self.return_cards.canceled = false;
                    }
                    return true;
                }
                let pcard = self.return_cards.list[0];
                if !self.core.operated_set.contains(pcard) {
                    self.core.operated_set.insert(pcard);
                    if self.core.release_cards_ex_oneof.contains(pcard) {
                        arg.extra_release_nonsum_effect = pcard.is_affected_by_effect(self, EFFECT_EXTRA_RELEASE_NONSUM);
                    }
                } else {
                    self.core.operated_set.remove(pcard);
                    if self.core.release_cards_ex_oneof.contains(pcard) {
                        arg.extra_release_nonsum_effect = None;
                    }
                }
                if self.core.operated_set.len() as i32 == max as i32 {
                    self.return_cards.list = self.core.operated_set.to_vec();
                    if let Some(e) = arg.extra_release_nonsum_effect {
                        self.core.dec_count_reserve.push(e);
                    }
                    return true;
                }
                arg.step = 1;
                false
            }
            _ => true,
        }
    }

    pub fn process_select_tribute(&mut self, arg: &mut units::SelectTribute) -> bool {
        let (target, playerid, cancelable, toplayer) = (arg.target, arg.playerid, arg.cancelable, arg.toplayer);
        let mut min = arg.min;
        let mut max = arg.max;
        let mut zone = arg.zone;
        match arg.step {
            0 => {
                self.core.operated_set.clear();
                zone &= (0x1f & self.get_forced_zones(target, toplayer, LOCATION_MZONE as u8, playerid as u32, LocationReason::ToField)) as u8;
                let ct = self.get_tofield_count(target, toplayer, LOCATION_MZONE as u8, playerid as u32, LocationReason::ToField, zone as u32, None);
                if ct > 0 {
                    self.hint_select_release(playerid);
                    if self.core.release_cards_ex.len() + self.core.release_cards_ex_oneof.len() == 0 {
                        let rc = self.core.release_cards.clone();
                        self.core.select_cards = public_order(self, &rc);
                        self.emplace_process(units::SelectTributeP { step: 0, playerid, cancelable, min: min as u8, max: max as u8 });
                        return true;
                    }
                    if self.core.release_cards_ex.len() >= max as usize {
                        let rce = self.core.release_cards_ex.clone();
                        self.core.select_cards = public_order(self, &rce);
                        self.emplace_process(units::SelectTributeP { step: 0, playerid, cancelable, min: min as u8, max: max as u8 });
                        return true;
                    }
                }
                false
            }
            1 => {
                let mut rmax: i32 = 0;
                zone &= (0x1f & self.get_forced_zones(target, toplayer, LOCATION_MZONE as u8, playerid as u32, LocationReason::ToField)) as u8;
                let ct = self.get_tofield_count(target, toplayer, LOCATION_MZONE as u8, playerid as u32, LocationReason::ToField, zone as u32, None);
                let mut must_choose_one = CardSet::new();
                let rc = self.core.release_cards.clone();
                for pcard in public_order(self, &rc) {
                    let c = self.card(pcard);
                    if c.current.location as u32 == LOCATION_MZONE && c.current.controler == toplayer && ((zone as u32).wrapping_shr(c.current.sequence) & 1) != 0 && ct <= 0 {
                        must_choose_one.insert(pcard);
                    }
                    rmax += c.release_param as i32;
                }
                self.hint_select_release(playerid);
                if self.core.release_cards_ex.is_empty() && self.core.release_cards_ex_oneof.is_empty() && min as i32 > rmax {
                    if rmax > 0 {
                        self.core.select_cards = public_order(self, &rc);
                    }
                    self.emplace_process(units::SelectTributeP { step: 0, playerid, cancelable, min: min as u8, max: max as u8 });
                    return true;
                }
                let force = !must_choose_one.is_empty();
                self.core.select_cards.clear();
                self.core.unselect_cards.clear();
                let (rcs, rce) = (self.core.release_cards.clone(), self.core.release_cards_ex.clone());
                self.core.release_cards_ex_oneof.retain(|&c| !rcs.contains(c) && !rce.contains(c));
                let rco = self.core.release_cards_ex_oneof.clone();
                let mut sc = Vec::new();
                if (max as usize).wrapping_sub(rce.len()) == 1 && force {
                    sc.extend(public_order(self, &must_choose_one));
                    sc.extend(public_order(self, &rce));
                } else if max <= 1 && force {
                    sc.extend(public_order(self, &must_choose_one));
                } else {
                    sc.extend(public_order(self, &rcs));
                    sc.extend(public_order(self, &rce));
                    sc.extend(public_order(self, &rco));
                }
                self.core.select_cards = sc;
                self.emplace_process(units::SelectUnselectCard { step: 0, playerid, cancelable, min: min as u8, max: max as u8, finishable: false });
                arg.step = 2;
                arg.must_choose_one = must_choose_one;
                false
            }
            2 => {
                let ops = self.core.operated_set.clone();
                let rmin = ops.len() as u32;
                let mut rmax: u32 = 0;
                for pcard in public_order(self, &ops) {
                    rmax = rmax.wrapping_add(self.card(pcard).release_param);
                }
                let (oldmin, oldmax) = (min, max);
                if rmax > min as u32 {
                    min = 0;
                } else {
                    min = min.wrapping_sub(rmax as u16);
                }
                if rmin > max as u32 {
                    max = 0;
                } else {
                    max = max.wrapping_sub(rmin as u16);
                }
                let must = arg.must_choose_one.clone();
                let mut force = !must.is_empty();
                for pcard in public_order(self, &must) {
                    if ops.contains(pcard) {
                        force = false;
                    }
                }
                self.core.select_cards.clear();
                self.core.unselect_cards.clear();
                let rce = self.core.release_cards_ex.clone();
                let rce_order = public_order(self, &rce);
                let exsize = rce_order.iter().filter(|&&c| !ops.contains(c)).count() as i32;
                let mut sc = Vec::new();
                if max as i32 - exsize == 1 && force {
                    sc.extend(public_order(self, &must).into_iter().filter(|&c| !ops.contains(c)));
                    sc.extend(rce_order.iter().copied().filter(|&c| !ops.contains(c)));
                } else if max <= 1 && force {
                    sc.extend(public_order(self, &must));
                } else if exsize != 0 && exsize == max as i32 {
                    sc.extend(rce_order.iter().copied().filter(|&c| !ops.contains(c)));
                } else {
                    let rc = self.core.release_cards.clone();
                    sc.extend(public_order(self, &rc).into_iter().filter(|&c| !ops.contains(c)));
                    sc.extend(rce_order.iter().copied().filter(|&c| !ops.contains(c)));
                    if arg.extra_release_effect.is_none() {
                        let rco = self.core.release_cards_ex_oneof.clone();
                        sc.extend(public_order(self, &rco).into_iter().filter(|&c| !ops.contains(c)));
                    }
                }
                self.core.select_cards = sc;
                let canc = rmin == 0 && cancelable;
                let finishable = min == 0 && !force && exsize == 0;
                self.core.unselect_cards = public_order(self, &ops);
                self.hint_select_release(playerid);
                self.emplace_process(units::SelectUnselectCard { step: 0, playerid, cancelable: canc, min: oldmin as u8, max: oldmax as u8, finishable });
                false
            }
            3 => {
                if self.return_cards.canceled && self.core.operated_set.is_empty() {
                    return true;
                }
                if !self.return_cards.canceled {
                    let pcard = self.return_cards.list[0];
                    if !self.core.operated_set.contains(pcard) {
                        self.core.operated_set.insert(pcard);
                        if self.core.release_cards_ex_oneof.contains(pcard) {
                            arg.extra_release_effect = pcard.is_affected_by_effect(self, EFFECT_EXTRA_RELEASE_SUM);
                        }
                    } else {
                        self.core.operated_set.remove(pcard);
                        if self.core.release_cards_ex_oneof.contains(pcard) {
                            arg.extra_release_effect = None;
                        }
                    }
                }
                let ops = self.core.operated_set.clone();
                let rmin = ops.len() as u32;
                let mut rmax: u32 = 0;
                for pcard in public_order(self, &ops) {
                    rmax = rmax.wrapping_add(self.card(pcard).release_param);
                }
                if rmax > min as u32 {
                    min = 0;
                } else {
                    min = min.wrapping_sub(rmax as u16);
                }
                if rmin > max as u32 {
                    max = 0;
                } else {
                    max = max.wrapping_sub(rmin as u16);
                }
                if (self.return_cards.canceled && min == 0) || max == 0 {
                    self.return_cards.clear();
                    self.return_cards.list = ops.to_vec();
                    if let Some(pe) = arg.extra_release_effect {
                        pe.dec_count(self, PLAYER_NONE);
                    }
                    return true;
                }
                arg.step = 1;
                false
            }
            _ => true,
        }
    }

    /// The first EFFECT_TOSS_*_REPLACE / _CHOOSE effect that can be activated.
    fn find_toss_replace(&mut self, code: u32, e: &Event) -> Option<(u8, EffectId)> {
        for peffect in self.field_effects.continuous_effect.range(code) {
            let handler_player = peffect.get_handler_player(self);
            if peffect.is_activateable(self, handler_player, e, false, false, false, false, false) {
                return Some((handler_player, peffect));
            }
        }
        None
    }

    pub fn process_toss_coin(&mut self, arg: &mut units::TossCoin) -> bool {
        let (reason_effect, reason_player, playerid, count) = (arg.reason_effect, arg.reason_player, arg.playerid, arg.count);
        match arg.step {
            0 => {
                let e = Event { event_player: playerid, event_value: count as u32, reason_effect, reason_player, ..Default::default() };
                self.core.coin_results.clear();
                for (code, step) in [(EFFECT_TOSS_COIN_REPLACE, None), (EFFECT_TOSS_COIN_CHOOSE, Some(2))] {
                    if let Some((hp, peffect)) = self.find_toss_replace(code, &e) {
                        self.core.coin_results.resize(count as usize, false);
                        self.solve_continuous(hp, peffect, &e);
                        match step {
                            None => return true,
                            Some(s) => {
                                arg.step = s;
                                return false;
                            }
                        }
                    }
                }
                let m = self.new_message(MSG_TOSS_COIN);
                self.msg(m).u8(playerid).u8(count);
                for _ in 0..count {
                    let coin = self.get_next_integer(0, 1);
                    self.core.coin_results.push(coin != 0);
                    self.msg(m).u8(coin as u8);
                }
                self.raise_event_card(None, EVENT_TOSS_COIN_NEGATE, reason_effect, 0, reason_player, playerid, count as u32);
                self.process_instant_event();
                false
            }
            1 => {
                let mut heads: u8 = 0;
                let mut tails: u8 = 0;
                for &r in &self.core.coin_results {
                    heads = heads.wrapping_add((r as u32 == COIN_HEADS) as u8);
                    tails = tails.wrapping_add((r as u32 == COIN_TAILS) as u8);
                }
                let v = ((tails as u32) << 16) | ((heads as u32) << 8) | count as u32;
                self.raise_event_card(None, EVENT_TOSS_COIN, reason_effect, 0, reason_player, playerid, v);
                self.process_instant_event();
                true
            }
            3 => {
                arg.step = 0;
                let m = self.new_message(MSG_TOSS_COIN);
                self.msg(m).u8(playerid).u8(count);
                for i in 0..count as usize {
                    let r = self.core.coin_results[i] as u8;
                    self.msg(m).u8(r);
                }
                self.raise_event_card(None, EVENT_TOSS_COIN_NEGATE, reason_effect, 0, reason_player, playerid, count as u32);
                self.process_instant_event();
                false
            }
            _ => true,
        }
    }

    pub fn process_toss_dice(&mut self, arg: &mut units::TossDice) -> bool {
        let (reason_effect, reason_player, playerid, count1, count2) = (arg.reason_effect, arg.reason_player, arg.playerid, arg.count1, arg.count2);
        let value = count1 as u32 + ((count2 as u32) << 16);
        let total = count1 as usize + count2 as usize;
        match arg.step {
            0 => {
                let e = Event { event_player: playerid, event_value: value, reason_effect, reason_player, ..Default::default() };
                self.core.dice_results.clear();
                for (code, step) in [(EFFECT_TOSS_DICE_REPLACE, None), (EFFECT_TOSS_DICE_CHOOSE, Some(2))] {
                    if let Some((hp, peffect)) = self.find_toss_replace(code, &e) {
                        self.core.dice_results.resize(total, 0);
                        self.solve_continuous(hp, peffect, &e);
                        match step {
                            None => return true,
                            Some(s) => {
                                arg.step = s;
                                return false;
                            }
                        }
                    }
                }
                let m = self.new_message(MSG_TOSS_DICE);
                self.msg(m).u8(playerid).u8(count1);
                for _ in 0..count1 {
                    let dice = self.get_next_integer(1, 6) as u8;
                    self.core.dice_results.push(dice);
                    self.msg(m).u8(dice);
                }
                if count2 > 0 {
                    let m = self.new_message(MSG_TOSS_DICE);
                    self.msg(m).u8(1 - playerid).u8(count2);
                    for _ in 0..count2 {
                        let dice = self.get_next_integer(1, 6) as u8;
                        self.core.dice_results.push(dice);
                        self.msg(m).u8(dice);
                    }
                }
                self.raise_event_card(None, EVENT_TOSS_DICE_NEGATE, reason_effect, 0, reason_player, playerid, value);
                self.process_instant_event();
                false
            }
            1 => {
                self.raise_event_card(None, EVENT_TOSS_DICE, reason_effect, 0, reason_player, playerid, value);
                self.process_instant_event();
                true
            }
            3 => {
                arg.step = 0;
                let m = self.new_message(MSG_TOSS_DICE);
                self.msg(m).u8(playerid).u8(count1);
                for i in 0..count1 as usize {
                    let r = self.core.dice_results[i];
                    self.msg(m).u8(r);
                }
                if count2 > 0 {
                    let m = self.new_message(MSG_TOSS_DICE);
                    self.msg(m).u8(1 - playerid).u8(count2);
                    for i in 0..count2 as usize {
                        let r = self.core.dice_results[count1 as usize + i];
                        self.msg(m).u8(r);
                    }
                }
                self.raise_event_card(None, EVENT_TOSS_DICE_NEGATE, reason_effect, 0, reason_player, playerid, value);
                self.process_instant_event();
                false
            }
            _ => true,
        }
    }
}
