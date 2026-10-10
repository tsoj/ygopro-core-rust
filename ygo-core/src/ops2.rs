//! Operations (`operations.cpp` part 2: draw, damage, recover, LP costs, counters, Xyz materials).

use crate::card::{card_public_sort, public_order, sort_by_less};
use crate::constants::*;
use crate::duel::{Duel, Event};
use crate::types::*;
use crate::units;

impl Duel {
    pub fn process_draw(&mut self, arg: &mut units::Draw) -> bool {
        let (reason_effect, reason, reason_player, playerid, count) = (arg.reason_effect, arg.reason, arg.reason_player, arg.playerid, arg.count);
        let p = playerid as usize;
        match arg.step {
            0 => {
                let mut cv = Vec::new();
                let mut drawn: u32 = 0;
                let mut public_count = 0u32;
                if reason & REASON_RULE == 0 && !self.is_player_can_draw(playerid) {
                    self.returns.set_i32(0, 0);
                    return true;
                }
                if count == 0 {
                    self.returns.set_i32(0, 0);
                    return true;
                }
                self.core.overdraw[p] = false;
                for _ in 0..count {
                    if self.player[p].list_main.is_empty() {
                        self.core.overdraw[p] = true;
                        break;
                    }
                    drawn += 1;
                    let c = *self.player[p].list_main.last().unwrap();
                    c.enable_field_effect(self, false);
                    c.cancel_field_effect(self);
                    self.player[p].list_main.pop();
                    if !self.core.current_chain.is_empty() {
                        self.core.just_sent_cards.insert(c);
                    }
                    {
                        let cc = self.card_mut(c);
                        cc.previous.controler = cc.current.controler;
                        cc.previous.location = cc.current.location;
                        cc.previous.sequence = cc.current.sequence;
                        cc.previous.position = cc.current.position;
                        cc.previous.pzone = cc.current.pzone;
                        cc.current.controler = PLAYER_NONE as u8;
                        cc.current.reason_effect = reason_effect;
                        cc.current.reason_player = reason_player;
                        cc.current.reason = reason | REASON_DRAW;
                        cc.current.location = 0;
                    }
                    self.add_card(playerid, c, LOCATION_HAND as u8, 0, false);
                    c.enable_field_effect(self, true);
                    let public = c.is_affected_by_effect(self, EFFECT_PUBLIC).is_some();
                    if public {
                        public_count += 1;
                    }
                    self.card_mut(c).current.position = if public { POS_FACEUP } else { POS_FACEDOWN };
                    cv.push(c);
                    c.reset(self, RESET_TOHAND, RESET_EVENT);
                }
                self.core.hint_timing[p] |= TIMING_DRAW + TIMING_TOHAND;
                self.adjust_instant();
                arg.count = drawn as u16;
                arg.drawn_set = CardSet::from_iter(cv.iter().copied());
                if drawn > 0 {
                    if self.core.global_flag & GLOBALFLAG_DECK_REVERSE_CHECK != 0 {
                        if let Some(&ptop) = self.player[p].list_main.last() {
                            if self.core.deck_reversed || self.card(ptop).current.position == POS_FACEUP_DEFENSE {
                                let (code, pos) = (self.card(ptop).data.code, self.card(ptop).current.position);
                                let m = self.new_message(MSG_DECK_TOP);
                                self.msg(m).u8(playerid).u32(drawn).u32(code).u32(pos);
                            }
                        }
                    }
                    let m = self.new_message(MSG_DRAW);
                    self.msg(m).u8(playerid).u32(drawn);
                    for &c in &cv {
                        let (code, pos) = (self.card(c).data.code, self.card(c).current.position);
                        self.msg(m).u32(code).u32(pos);
                    }
                    let drawn_set = arg.drawn_set.clone();
                    if self.core.deck_reversed && public_count < drawn {
                        let ordered = public_order(self, &drawn_set);
                        let m = self.new_message(MSG_CONFIRM_CARDS);
                        self.msg(m).u8(1 - playerid).u32(drawn_set.len());
                        for c in ordered {
                            let cc = self.card(c);
                            let (code, ctrl, loc, seq) = (cc.data.code, cc.current.controler, cc.current.location, cc.current.sequence);
                            self.msg(m).u32(code).u8(ctrl).u8(loc).u32(seq);
                        }
                        self.shuffle(playerid, LOCATION_HAND as u8);
                    }
                    for c in public_order(self, &drawn_set) {
                        if self.card(c).owner != self.card(c).current.controler {
                            let de = self.new_effect();
                            let e = self.effect_mut(de);
                            e.owner = Some(c);
                            e.code = 0;
                            e.type_ = EFFECT_TYPE_SINGLE as u16;
                            e.flag[0] = EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_CLIENT_HINT;
                            e.description = 67;
                            e.reset_flag = RESET_EVENT + 0x1fe0000;
                            c.add_effect(self, de);
                        }
                        self.raise_single_event(c, None, EVENT_DRAW, reason_effect, reason, reason_player, playerid, 0);
                        self.raise_single_event(c, None, EVENT_TO_HAND, reason_effect, reason, reason_player, playerid, 0);
                        self.raise_single_event(c, None, EVENT_MOVE, reason_effect, reason, reason_player, playerid, 0);
                    }
                    self.process_single_event();
                    self.raise_event(drawn_set.clone(), EVENT_DRAW, reason_effect, reason, reason_player, playerid, drawn);
                    self.raise_event(drawn_set.clone(), EVENT_TO_HAND, reason_effect, reason, reason_player, playerid, drawn);
                    self.raise_event(drawn_set, EVENT_MOVE, reason_effect, reason, reason_player, playerid, drawn);
                    self.process_instant_event();
                }
                false
            }
            1 => {
                std::mem::swap(&mut self.core.operated_set, &mut arg.drawn_set);
                self.returns.set_u32(0, count as u32);
                true
            }
            _ => true,
        }
    }

    pub fn process_damage(&mut self, arg: &mut units::Damage) -> bool {
        let (reason_effect, reason, reason_player, reason_card, amount, is_step) = (arg.reason_effect, arg.reason, arg.reason_player, arg.reason_card, arg.amount, arg.is_step);
        let mut playerid = arg.playerid;
        match arg.step {
            0 => {
                self.returns.set_u32(0, amount);
                if amount == 0 {
                    return true;
                }
                if reason & REASON_RDAMAGE == 0 {
                    let mut eset = Vec::new();
                    self.filter_player_effect(playerid, EFFECT_REVERSE_DAMAGE, &mut eset, true);
                    for peff in eset {
                        self.add_param(Value::effect(reason_effect));
                        self.add_param(Value::Int(reason as i64));
                        self.add_param(Value::Int(reason_player as i64));
                        self.add_param(Value::card(reason_card));
                        if peff.check_value_condition(self, 4) {
                            self.recover(reason_effect, (reason & REASON_RRECOVER) | REASON_RDAMAGE | REASON_EFFECT, reason_player as u32, playerid as u32, amount, is_step);
                            arg.step = 2;
                            return false;
                        }
                    }
                }
                let mut eset = Vec::new();
                self.filter_player_effect(playerid, EFFECT_REFLECT_DAMAGE, &mut eset, true);
                for peff in eset {
                    self.add_param(Value::effect(reason_effect));
                    self.add_param(Value::Int(amount as i64));
                    self.add_param(Value::Int(reason as i64));
                    self.add_param(Value::Int(reason_player as i64));
                    self.add_param(Value::card(reason_card));
                    if peff.check_value_condition(self, 5) {
                        playerid = 1 - playerid;
                        arg.is_reflected = true;
                        break;
                    }
                }
                let mut val = amount;
                let mut eset = Vec::new();
                self.filter_player_effect(playerid, EFFECT_CHANGE_DAMAGE, &mut eset, true);
                for peff in eset {
                    self.add_param(Value::effect(reason_effect));
                    self.add_param(Value::Int(val as i64));
                    self.add_param(Value::Int(reason as i64));
                    self.add_param(Value::Int(reason_player as i64));
                    self.add_param(Value::card(reason_card));
                    val = peff.get_value(self, 5) as u32;
                    self.returns.set_u32(0, val);
                    if val == 0 {
                        return true;
                    }
                }
                arg.amount = val;
                if is_step {
                    arg.step = 1;
                    self.core.recover_damage_reserve.push_back(units::Unit::Damage(arg.clone()));
                    return true;
                }
                false
            }
            1 => {
                if arg.is_reflected {
                    playerid = 1 - playerid;
                }
                if arg.is_reflected || reason & REASON_RRECOVER != 0 {
                    arg.step = 2;
                }
                let p = playerid as usize;
                self.core.hint_timing[p] |= TIMING_DAMAGE;
                self.player[p].lp = self.player[p].lp.wrapping_sub(amount as i32);
                let m = self.new_message(MSG_DAMAGE);
                self.msg(m).u8(playerid).u32(amount);
                self.raise_event_card(reason_card, EVENT_DAMAGE, reason_effect, reason, reason_player, playerid, amount);
                if reason == REASON_BATTLE {
                    if let Some(rc) = reason_card {
                        if self.player[p].lp <= 0
                            && self.core.attack_target.is_none()
                            && rc.is_affected_by_effect(self, EFFECT_MATCH_KILL).is_some()
                            && self.is_player_affected_by_effect(playerid, EFFECT_CANNOT_LOSE_LP).is_none()
                        {
                            let code = self.card(rc).data.code;
                            let m = self.new_message(MSG_MATCH_KILL);
                            self.msg(m).u32(code);
                        }
                        self.raise_single_event(rc, None, EVENT_BATTLE_DAMAGE, None, 0, reason_player, playerid, amount);
                        self.raise_event_card(Some(rc), EVENT_BATTLE_DAMAGE, None, 0, reason_player, playerid, amount);
                        self.process_single_event();
                    }
                }
                if self.is_player_affected_by_effect(playerid, EFFECT_CANNOT_LOSE_LP).is_some() && self.player[p].lp < 0 {
                    self.player[p].lp = 0;
                }
                self.process_instant_event();
                false
            }
            2 => {
                self.returns.set_u32(0, amount);
                true
            }
            3 => {
                self.returns.set_u32(0, 0);
                true
            }
            _ => true,
        }
    }

    pub fn process_recover(&mut self, arg: &mut units::Recover) -> bool {
        let (reason_effect, reason, reason_player, playerid, amount, is_step) = (arg.reason_effect, arg.reason, arg.reason_player, arg.playerid, arg.amount, arg.is_step);
        match arg.step {
            0 => {
                self.returns.set_u32(0, amount);
                if amount == 0 {
                    return true;
                }
                if reason & REASON_RRECOVER == 0 {
                    let mut eset = Vec::new();
                    self.filter_player_effect(playerid, EFFECT_REVERSE_RECOVER, &mut eset, true);
                    for peff in eset {
                        self.add_param(Value::effect(reason_effect));
                        self.add_param(Value::Int(reason as i64));
                        self.add_param(Value::Int(reason_player as i64));
                        if peff.check_value_condition(self, 3) {
                            self.damage(reason_effect, (reason & REASON_RDAMAGE) | REASON_RRECOVER | REASON_EFFECT, reason_player, None, playerid, amount, is_step);
                            arg.step = 2;
                            return false;
                        }
                    }
                }
                if is_step {
                    arg.step = 1;
                    self.core.recover_damage_reserve.push_back(units::Unit::Recover(arg.clone()));
                    return true;
                }
                false
            }
            1 => {
                if reason & REASON_RDAMAGE != 0 {
                    arg.step = 2;
                }
                let p = playerid as usize;
                self.core.hint_timing[p] |= TIMING_RECOVER;
                self.player[p].lp = self.player[p].lp.wrapping_add(amount as i32);
                let m = self.new_message(MSG_RECOVER);
                self.msg(m).u8(playerid).u32(amount);
                self.raise_event_card(None, EVENT_RECOVER, reason_effect, reason, reason_player, playerid, amount);
                self.process_instant_event();
                false
            }
            2 => {
                self.returns.set_u32(0, amount);
                true
            }
            3 => {
                self.returns.set_u32(0, 0);
                true
            }
            _ => true,
        }
    }

    /// Offers the replacement effects of `code` (and the normal way if `normal_option` is set):
    /// shared by LP costs, counter and material removal.
    fn offer_replacements(&mut self, normal_option: Option<u64>, code: u32, e: &Event, player: u8, yesno_desc: u64) -> bool {
        self.core.select_options.clear();
        self.core.select_effects.clear();
        if let Some(o) = normal_option {
            self.core.select_options.push(o);
            self.core.select_effects.push(None);
        }
        for pe in self.field_effects.continuous_effect.range(code) {
            let hp = pe.get_handler_player(self);
            if pe.is_activateable(self, hp, e, false, false, false, false, false) {
                let d = self.effect(pe).description;
                self.core.select_options.push(d);
                self.core.select_effects.push(Some(pe));
            }
        }
        self.sort_select_options();
        if self.core.select_options.is_empty() {
            return false;
        }
        if self.core.select_options.len() == 1 {
            self.returns.set_i32(0, 0);
        } else if self.core.select_effects[0].is_none() && self.core.select_effects.len() == 2 {
            let h = self.effect(self.core.select_effects[1].unwrap()).handler;
            self.emplace_process(units::SelectEffectYesNo { step: 0, playerid: player, pcard: h, description: yesno_desc });
        } else {
            self.emplace_process(units::SelectOption { step: 0, playerid: player });
        }
        true
    }

    pub fn process_pay_lp_cost(&mut self, arg: &mut units::PayLPCost) -> bool {
        let playerid = arg.playerid;
        let p = playerid as usize;
        match arg.step {
            0 => {
                let mut val = arg.cost as i32;
                let mut eset = Vec::new();
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
                arg.cost = val as u32;
                let e = Event {
                    event_cards: None,
                    event_player: playerid,
                    event_value: val as u32,
                    reason: 0,
                    reason_effect: self.core.reason_effect,
                    reason_player: playerid,
                    ..Default::default()
                };
                let normal = if val <= self.player[p].lp { Some(11) } else { None };
                if !self.offer_replacements(normal, EFFECT_LPCOST_REPLACE, &e, playerid, 218) {
                    return true;
                }
                false
            }
            1 => {
                let idx = self.returns.at_i32(0) as usize;
                let cost = arg.cost;
                match self.core.select_effects[idx] {
                    None => {
                        self.player[p].lp = self.player[p].lp.wrapping_sub(cost as i32);
                        let m = self.new_message(MSG_PAY_LPCOST);
                        self.msg(m).u8(playerid).u32(cost);
                        let re = self.core.reason_effect;
                        self.raise_event_card(None, EVENT_PAY_LPCOST, re, 0, playerid, playerid, cost);
                        self.process_instant_event();
                    }
                    Some(pe) => {
                        let e = Event {
                            event_cards: None,
                            event_player: playerid,
                            event_value: cost,
                            reason: 0,
                            reason_effect: self.core.reason_effect,
                            reason_player: playerid,
                            ..Default::default()
                        };
                        self.solve_continuous(playerid, pe, &e);
                    }
                }
                true
            }
            _ => true,
        }
    }

    pub fn process_remove_counter(&mut self, arg: &mut units::RemoveCounter) -> bool {
        let (reason, pcard, rplayer, self_, oppo, countertype, count) = (arg.reason, arg.pcard, arg.rplayer, arg.self_, arg.oppo, arg.countertype, arg.count);
        match arg.step {
            0 => {
                let have = match pcard {
                    Some(c) => c.get_counter(self, countertype) >= count as i32,
                    None => self.get_field_counter(rplayer, self_, oppo, countertype) != 0,
                };
                let e = Event {
                    event_cards: None,
                    event_player: rplayer,
                    event_value: count as u32,
                    reason,
                    reason_effect: self.core.reason_effect,
                    reason_player: rplayer,
                    ..Default::default()
                };
                let normal = if have { Some(10) } else { None };
                let any = self.offer_replacements(normal, EFFECT_RCOUNTER_REPLACE + countertype as u32, &e, rplayer, 220);
                if !any {
                    self.returns.set_i32(0, 0);
                    return true;
                }
                if self.core.select_options.len() == 1 {
                    self.returns.set_i32(0, 0);
                }
                false
            }
            1 => {
                let idx = self.returns.at_i32(0) as usize;
                if let Some(pe) = self.core.select_effects[idx] {
                    let e = Event {
                        event_cards: None,
                        event_player: rplayer,
                        event_value: count as u32,
                        reason,
                        reason_effect: self.core.reason_effect,
                        reason_player: rplayer,
                        ..Default::default()
                    };
                    self.solve_continuous(rplayer, pe, &e);
                    arg.step = 3;
                    return false;
                }
                if let Some(c) = pcard {
                    let r = c.remove_counter(self, countertype, count);
                    self.returns.set_i32(0, r as i32);
                    arg.step = 3;
                    return false;
                }
                self.emplace_process(units::SelectCounter { step: 0, countertype, count, playerid: rplayer, self_, oppo });
                false
            }
            2 => {
                for i in 0..self.core.select_cards.len() {
                    let v = self.returns.at_i16(i);
                    if v > 0 {
                        let c = self.core.select_cards[i];
                        c.remove_counter(self, countertype, v as u16);
                    }
                }
                false
            }
            3 => {
                let re = self.core.reason_effect;
                self.raise_event_card(None, EVENT_REMOVE_COUNTER + countertype as u32, re, reason, rplayer, rplayer, count as u32);
                self.process_instant_event();
                false
            }
            4 => {
                self.returns.set_i32(0, 1);
                true
            }
            _ => true,
        }
    }

    pub fn process_remove_overlay(&mut self, arg: &mut units::RemoveOverlay) -> bool {
        let (reason, rplayer, self_, oppo) = (arg.reason, arg.rplayer, arg.self_, arg.oppo);
        let pgroup = arg.pgroup.map(|g| self.group(g).clone());
        match arg.step {
            0 => {
                let have = self.get_overlay_count(rplayer, self_, oppo, pgroup.as_ref()) >= arg.min as i32;
                let e = Event {
                    event_cards: None,
                    event_player: rplayer,
                    event_value: arg.min as u32,
                    reason,
                    reason_effect: self.core.reason_effect,
                    reason_player: rplayer,
                    ..Default::default()
                };
                let normal = if have { Some(12) } else { None };
                let any = self.offer_replacements(normal, EFFECT_OVERLAY_REMOVE_REPLACE, &e, rplayer, 219);
                if !any {
                    self.returns.set_i32(0, 0);
                    return true;
                }
                if self.core.select_options.len() == 1 {
                    self.returns.set_i32(0, 0);
                }
                false
            }
            1 => {
                let idx = self.returns.at_i32(0) as usize;
                if let Some(pe) = self.core.select_effects[idx] {
                    let e = Event {
                        event_cards: None,
                        event_player: rplayer,
                        event_value: arg.min as u32 + ((arg.max as u32) << 16),
                        reason,
                        reason_effect: self.core.reason_effect,
                        reason_player: rplayer,
                        ..Default::default()
                    };
                    self.solve_continuous(rplayer, pe, &e);
                    arg.has_used_overlay_remove_replace_effect = true;
                }
                false
            }
            2 => {
                let mut cancelable = false;
                let (mut min, mut max) = (arg.min as i32, arg.max as i32);
                if arg.has_used_overlay_remove_replace_effect {
                    let replace_count = self.returns.at_i32(0);
                    if replace_count >= max {
                        return true;
                    }
                    min -= replace_count;
                    max -= replace_count;
                    if min <= 0 {
                        cancelable = true;
                        min = 0;
                    }
                    arg.replaced_amount = replace_count as u16;
                }
                let mut cset = CardSet::new();
                self.get_overlay_group(rplayer, self_, oppo, &mut cset, pgroup.as_ref());
                self.core.select_cards = public_order(self, &cset);
                let m = self.new_message(MSG_HINT);
                self.msg(m).u8(HINT_SELECTMSG).u8(rplayer).u64(519u64);
                self.emplace_process(units::SelectCard { step: 0, playerid: rplayer, cancelable, min: min as u8, max: max as u8 });
                false
            }
            3 => {
                let set = CardSet::from_iter(self.return_cards.list.iter().copied());
                let re = self.core.reason_effect;
                self.send_to(set, re, reason, rplayer, PLAYER_NONE as u8, LOCATION_GRAVE as u16, 0, POS_FACEUP as u8, false);
                false
            }
            4 => {
                let v = self.returns.at_i32(0);
                self.returns.set_i32(0, v + arg.replaced_amount as i32);
                true
            }
            _ => true,
        }
    }

    pub fn process_xyz_overlay(&mut self, arg: &mut units::XyzOverlay) -> bool {
        let target = arg.target;
        let materials = self.group(arg.materials.unwrap()).clone();
        let send_to_grave = arg.send_materials_to_grave;
        match arg.step {
            0 => {
                for c in public_order(self, &materials) {
                    if self.card(c).overlay_target.is_none() {
                        c.enable_field_effect(self, false);
                    }
                }
                if !send_to_grave {
                    return false;
                }
                let mut to_grave = CardSet::new();
                for c in public_order(self, &materials) {
                    to_grave.extend(self.card(c).xyz_materials.clone());
                }
                self.send_to(to_grave, None, REASON_RULE + REASON_LOST_TARGET, PLAYER_NONE as u8, PLAYER_NONE as u8, LOCATION_GRAVE as u16, 0, POS_FACEUP as u8, false);
                false
            }
            1 => {
                let mut des = CardSet::new();
                let mut from_grave = CardSet::new();
                let mut cv: Vec<CardId> = materials.iter().filter(|&c| self.card(c).overlay_target != Some(target)).collect();
                if !send_to_grave {
                    let prev_size = cv.len();
                    for c in public_order(self, &materials) {
                        let mats = self.card(c).xyz_materials.clone();
                        for (k, m) in mats.into_iter().enumerate() {
                            cv.insert(k, m);
                        }
                    }
                    let cur_size = cv.len();
                    if cur_size - prev_size >= 2 {
                        let end = (cur_size - prev_size) - 1;
                        sort_by_less(&mut cv[..end], |a, b| card_public_sort(self, a, b));
                    }
                    if prev_size >= 2 {
                        sort_by_less(&mut cv[cur_size - prev_size..], |a, b| card_public_sort(self, a, b));
                    }
                } else if cv.len() > 1 {
                    sort_by_less(&mut cv, |a, b| card_public_sort(self, a, b));
                }
                let mut decktop: [Option<usize>; 2] = [None, None];
                let s = [self.player[0].list_main.len(), self.player[1].list_main.len()];
                if self.core.global_flag & GLOBALFLAG_DECK_REVERSE_CHECK != 0 {
                    for p in 0..2usize {
                        if s[p] > 0 && materials.contains(*self.player[p].list_main.last().unwrap()) {
                            let m = self.new_message(MSG_DECK_TOP);
                            self.msg(m).u8(p as u8);
                            decktop[p] = Some(m);
                        }
                    }
                }
                let (re, rp) = (self.core.reason_effect, self.core.reason_player);
                for c in cv {
                    self.card_mut(c).current.reason = REASON_XYZ + REASON_MATERIAL;
                    c.reset(self, RESET_LEAVE + RESET_OVERLAY, RESET_EVENT);
                    if self.card(c).unique_code != 0 {
                        self.remove_unique_card(c);
                    }
                    if self.card(c).equiping_target.is_some() {
                        c.unequip(self);
                    }
                    for eq in public_order(self, &self.card(c).equiping_cards.clone()) {
                        des.insert(eq);
                        eq.unequip(self);
                    }
                    c.clear_card_target(self);
                    let code = self.card(c).data.code;
                    let loc = c.get_info_location(self);
                    let m = self.new_message(MSG_MOVE);
                    self.msg(m).u32(code).loc(loc);
                    if let Some(ot) = self.card(c).overlay_target {
                        ot.xyz_remove(self, c);
                    } else {
                        self.remove_card(c);
                        self.add_to_disable_check_list(c);
                        if self.card(c).previous.location as u32 == LOCATION_GRAVE {
                            from_grave.insert(c);
                            self.raise_single_event(c, None, EVENT_LEAVE_GRAVE, re, 0, rp, 0, 0);
                        }
                    }
                    target.xyz_add(self, c);
                    let loc = c.get_info_location(self);
                    let reason = self.card(c).current.reason;
                    self.msg(m).loc(loc).u32(reason);
                }
                let rev = self.core.deck_reversed;
                for p in 0..2usize {
                    let Some(m) = decktop[p] else { continue };
                    let list = &self.player[p].list_main;
                    if list.is_empty() || (!rev && self.card(*list.last().unwrap()).current.position != POS_FACEUP_DEFENSE) {
                        self.msg(m).data.clear();
                    } else {
                        let ptop = *list.last().unwrap();
                        let n = (s[p] - list.len()) as u32;
                        let (code, pos) = (self.card(ptop).data.code, self.card(ptop).current.position);
                        self.msg(m).u32(n).u32(code).u32(pos);
                    }
                }
                if !from_grave.is_empty() {
                    self.raise_event(from_grave, EVENT_LEAVE_GRAVE, re, 0, rp, 0, 0);
                    self.process_single_event();
                    self.process_instant_event();
                }
                if !des.is_empty() {
                    self.destroy(des, None, REASON_LOST_TARGET + REASON_RULE, PLAYER_NONE as u8, PLAYER_NONE as u8, 0, 0);
                } else {
                    self.adjust_instant();
                }
                if self.card(target).current.location as u32 & LOCATION_ONFIELD != 0 {
                    self.adjust_all();
                }
                false
            }
            _ => true,
        }
    }
}
