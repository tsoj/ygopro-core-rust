//! Operations (`operations.cpp` part 8: milling, moving to the field, changing positions,
//! replacement effects and activating effects).

use crate::card::{card_public_sort, public_order, sort_by_less};
use crate::constants::*;
use crate::duel::{Chain, Duel, Event};
use crate::types::*;
use crate::units::{self, LocationReason};

/// The location reason `get_useable_count` gets for a move to the field.
fn real_location_reason(reason: LocationReason, current_location: u8) -> LocationReason {
    if reason != LocationReason::None {
        if reason != LocationReason::Summon && reason != LocationReason::SpSummon && reason != LocationReason::Set {
            return reason;
        }
        LocationReason::ToField
    } else if current_location as u32 == LOCATION_MZONE {
        LocationReason::Control
    } else {
        LocationReason::ToField
    }
}

impl Duel {
    /// The hint effects of a card added to the hand (owner's card in the opponent's hand,
    /// added by a resolving effect).
    pub(crate) fn add_to_hand_hints(&mut self, pcard: CardId) {
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
            e.reset_flag = RESET_EVENT | RESET_TOFIELD | RESET_LEAVE | RESET_TODECK | RESET_TOHAND | RESET_TEMP_REMOVE | RESET_REMOVE | RESET_TOGRAVE | RESET_TURN_SET | RESET_CHAIN;
            pcard.add_effect(self, de);
        }
    }
    fn card_reason_triple(&self, pcard: CardId) -> (Option<EffectId>, u32, u8) {
        let c = self.card(pcard);
        (c.current.reason_effect, c.current.reason, c.current.reason_player)
    }

    pub fn process_discard_deck(&mut self, arg: &mut units::DiscardDeck) -> bool {
        let (playerid, count, reason) = (arg.playerid, arg.count, arg.reason);
        let p = playerid as usize;
        match arg.step {
            0 => {
                if self.is_player_affected_by_effect(playerid, EFFECT_CANNOT_DISCARD_DECK).is_some() {
                    self.core.operated_set.clear();
                    self.returns.set_i32(0, 0);
                    return true;
                }
                let n = self.player[p].list_main.len();
                for i in 0..(count as usize).min(n) {
                    let c = self.player[p].list_main[n - 1 - i];
                    let (re, rp) = (self.core.reason_effect, self.core.reason_player);
                    {
                        let cc = self.card_mut(c);
                        cc.sendto_param.location = LOCATION_GRAVE as u8;
                        cc.current.reason_effect = re;
                        cc.current.reason_player = rp;
                        cc.current.reason = reason;
                    }
                    let redirect = c.destination_redirect(self, LOCATION_GRAVE as u8, reason) & 0xffff;
                    if redirect != 0 {
                        self.card_mut(c).sendto_param.location = redirect as u8;
                    }
                }
                if self.core.global_flag & GLOBALFLAG_DECK_REVERSE_CHECK != 0 && n > count as usize {
                    let ptop = self.player[p].list_main[n - 1 - count as usize];
                    let c = self.card(ptop);
                    if self.core.deck_reversed || c.current.position == POS_FACEUP_DEFENSE {
                        let (code, pos) = (c.data.code, c.current.position);
                        let m = self.new_message(MSG_DECK_TOP);
                        self.msg(m).u8(playerid).u32(count as u32).u32(code).u32(pos);
                    }
                }
                false
            }
            1 => {
                let mut tohand = CardSet::new();
                let mut todeck = CardSet::new();
                let mut tograve = CardSet::new();
                let mut remove = CardSet::new();
                self.core.discarded_set.clear();
                for _ in 0..count {
                    let Some(&pcard) = self.player[p].list_main.last() else { break };
                    let dest = self.card(pcard).sendto_param.location as u32;
                    if dest == LOCATION_GRAVE {
                        pcard.reset(self, RESET_TOGRAVE, RESET_EVENT);
                    } else if dest == LOCATION_HAND {
                        pcard.reset(self, RESET_TOHAND, RESET_EVENT);
                        self.card_mut(pcard).set_status(STATUS_PROC_COMPLETE, false);
                    } else if dest == LOCATION_DECK {
                        pcard.reset(self, RESET_TODECK, RESET_EVENT);
                        self.card_mut(pcard).set_status(STATUS_PROC_COMPLETE, false);
                    } else if dest == LOCATION_REMOVED {
                        if self.card(pcard).current.reason & REASON_TEMPORARY != 0 {
                            pcard.reset(self, RESET_TEMP_REMOVE, RESET_EVENT);
                        } else {
                            pcard.reset(self, RESET_REMOVE, RESET_EVENT);
                        }
                    }
                    let code = self.card(pcard).data.code;
                    let from = pcard.get_info_location(self);
                    pcard.enable_field_effect(self, false);
                    pcard.cancel_field_effect(self);
                    self.player[p].list_main.pop();
                    if !self.core.current_chain.is_empty() {
                        self.core.just_sent_cards.insert(pcard);
                    }
                    let owner = {
                        let c = self.card_mut(pcard);
                        c.previous.controler = c.current.controler;
                        c.previous.location = c.current.location;
                        c.previous.sequence = c.current.sequence;
                        c.previous.position = c.current.position;
                        c.previous.pzone = c.current.pzone;
                        c.current.controler = PLAYER_NONE as u8;
                        c.current.location = 0;
                        c.owner
                    };
                    self.add_card(owner, pcard, dest as u8, 0, false);
                    pcard.enable_field_effect(self, true);
                    self.card_mut(pcard).current.position = POS_FACEUP;
                    let to = pcard.get_info_location(self);
                    let r = self.card(pcard).current.reason;
                    let m = self.new_message(MSG_MOVE);
                    self.msg(m).u32(code).loc(from).loc(to).u32(r);
                    if dest == LOCATION_HAND {
                        self.add_to_hand_hints(pcard);
                        tohand.insert(pcard);
                        let (re, r, rp) = self.card_reason_triple(pcard);
                        self.raise_single_event(pcard, None, EVENT_TO_HAND, re, r, rp, 0, 0);
                    } else if dest == LOCATION_DECK || dest == LOCATION_EXTRA {
                        todeck.insert(pcard);
                        let (re, r, rp) = self.card_reason_triple(pcard);
                        self.raise_single_event(pcard, None, EVENT_TO_DECK, re, r, rp, 0, 0);
                    } else if dest == LOCATION_GRAVE {
                        tograve.insert(pcard);
                        let (re, r, rp) = self.card_reason_triple(pcard);
                        self.raise_single_event(pcard, None, EVENT_TO_GRAVE, re, r, rp, 0, 0);
                    } else if dest == LOCATION_REMOVED {
                        remove.insert(pcard);
                        let (re, r, rp) = self.card_reason_triple(pcard);
                        self.raise_single_event(pcard, None, EVENT_REMOVE, re, r, rp, 0, 0);
                    }
                    let (re, r, rp) = self.card_reason_triple(pcard);
                    self.raise_single_event(pcard, None, EVENT_MOVE, re, r, rp, 0, 0);
                    self.core.discarded_set.insert(pcard);
                }
                let (cre, crp) = (self.core.reason_effect, self.core.reason_player);
                for (s, code) in [(tohand, EVENT_TO_HAND), (todeck, EVENT_TO_DECK), (tograve, EVENT_TO_GRAVE), (remove, EVENT_REMOVE)] {
                    if !s.is_empty() {
                        self.raise_event(s, code, cre, reason, crp, 0, 0);
                    }
                }
                let ds = self.core.discarded_set.clone();
                self.raise_event(ds, EVENT_MOVE, cre, reason, crp, 0, 0);
                self.process_single_event();
                self.process_instant_event();
                self.adjust_instant();
                false
            }
            2 => {
                std::mem::swap(&mut self.core.operated_set, &mut self.core.discarded_set);
                let n = self.core.operated_set.len();
                self.returns.set_i32(0, n as i32);
                self.core.discarded_set.clear();
                true
            }
            _ => true,
        }
    }

    // move a card from anywhere to field, including sp_summon, Duel.MoveToField(), Duel.ReturnToField()
    // ret: 0 = default, 1 = return after temporarily banished, 2 = trap_monster return to LOCATION_SZONE
    // call move_card() in step 2
    pub fn process_move_to_field(&mut self, arg: &mut units::MoveToField) -> bool {
        let (target, enable, ret, pzone, zone, rule, reason, confirm) = (arg.target, arg.enable, arg.ret, arg.pzone, arg.zone, arg.rule, arg.location_reason, arg.confirm);
        let tfp = self.card(target).to_field_param;
        let mut move_player = ((tfp >> 24) & 0xff) as u8;
        let playerid = ((tfp >> 16) & 0xff) as u8;
        let location = ((tfp >> 8) & 0xff) as u8;
        let positions = (tfp & 0xff) as u8;
        let is_field_spell = |d: &Duel| {
            let t = d.card(target).data.type_;
            location as u32 == LOCATION_SZONE && zone == (0x1 << 5) && t & TYPE_FIELD != 0 && t & (TYPE_SPELL | TYPE_TRAP) != 0
        };
        match arg.step {
            0 => {
                self.returns.set_i32(0, 0);
                if ret == 1 {
                    let c = self.card(target);
                    if c.current.reason & REASON_TEMPORARY == 0 {
                        return true;
                    }
                    let o1 = self.effect(c.current.reason_effect.unwrap()).owner;
                    let o2 = self.effect(self.core.reason_effect.unwrap()).owner;
                    if o1 != o2 {
                        return true;
                    }
                }
                let ttype = self.card(target).data.type_;
                if is_field_spell(self) {
                    if let Some(pcard) = self.get_field_card(playerid as u32, LOCATION_SZONE, 5) {
                        let ctrl = self.card(pcard).current.controler;
                        if !self.is_flag(DUEL_1_FACEUP_FIELD) {
                            self.send_to(CardSet::from_card(pcard), None, REASON_RULE, ctrl, PLAYER_NONE as u8, LOCATION_GRAVE as u16, 0, 0, false);
                        } else {
                            self.destroy(CardSet::from_card(pcard), None, REASON_RULE, ctrl, PLAYER_NONE as u8, 0, 0);
                        }
                        self.adjust_all();
                    }
                } else if pzone && location as u32 == LOCATION_SZONE && ttype & TYPE_PENDULUM != 0 && self.is_flag(DUEL_PZONE) {
                    let mut flag: u32 = 0;
                    if self.is_location_useable(playerid as u32, LOCATION_PZONE, 0) && zone & 1 != 0 {
                        flag |= 0x1u32 << (self.get_pzone_index(0, playerid) + 8);
                    }
                    if self.is_location_useable(playerid as u32, LOCATION_PZONE, 1) && zone & 2 != 0 {
                        flag |= 0x1u32 << (self.get_pzone_index(1, playerid) + 8);
                    }
                    if flag == 0 {
                        return true;
                    }
                    if move_player != playerid {
                        flag <<= 16;
                    }
                    flag = !flag;
                    let code = self.card(target).data.code;
                    let m = self.new_message(MSG_HINT);
                    self.msg(m).u8(HINT_SELECTMSG).u8(move_player).u64(code);
                    self.emplace_process(units::SelectPlace { step: 0, playerid: move_player, count: 1, flag, disable_field: false });
                } else {
                    let mut flag: u32 = 0;
                    let real_reason = real_location_reason(reason, self.card(target).current.location);
                    let mut ct = self.get_useable_count(Some(target), playerid, location, move_player, real_reason, zone as u32, Some(&mut flag));
                    if location as u32 == LOCATION_MZONE && zone & 0x60 != 0 && zone != 0xff && !rule {
                        if zone & 0x20 != 0 && self.is_location_useable(playerid as u32, location as u32, 5) {
                            flag &= !(1u32 << 5);
                            ct += 1;
                        }
                        if zone & 0x40 != 0 && self.is_location_useable(playerid as u32, location as u32, 6) {
                            flag &= !(1u32 << 6);
                            ct += 1;
                        }
                    }
                    if location as u32 == LOCATION_SZONE {
                        flag |= !(zone as u32);
                    }
                    if ret == 1
                        && (ct <= 0
                            || self.card(target).is_status(STATUS_FORBIDDEN)
                            || ((positions as u32 & POS_FACEDOWN) == 0 && self.check_unique_onfield(target, playerid, location, None).is_some()))
                    {
                        arg.step = 3;
                        let (cre, crp) = (self.core.reason_effect, self.core.reason_player);
                        self.send_to(CardSet::from_card(target), cre, REASON_RULE, crp, PLAYER_NONE as u8, LOCATION_GRAVE as u16, 0, 0, false);
                        return false;
                    }
                    if ct <= 0 || !flag == 0 {
                        return true;
                    }
                    if !confirm && (zone & zone.wrapping_sub(1)) == 0 {
                        for seq in 0..8u8 {
                            if (1u32 << seq) & zone as u32 != 0 {
                                self.returns.set_i8(2, seq as i8);
                                return false;
                            }
                        }
                    }
                    if !self.is_flag(DUEL_TRAP_MONSTERS_NOT_USE_ZONE) && ret == 2 {
                        let s = self.card(target).previous.sequence;
                        self.returns.set_i8(2, s as i8);
                        return false;
                    }
                    if reason == LocationReason::Summon || reason == LocationReason::SpSummon || reason == LocationReason::Set {
                        let mut eset = Vec::new();
                        self.filter_player_effect(move_player, EFFECT_OPPO_CHOOSES_SPSUMMON_ZONE, &mut eset, true);
                        for peff in eset {
                            let e = self.effect(peff);
                            if e.is_flag(EFFECT_FLAG_COUNT_LIMIT) && e.count_limit == 0 {
                                continue;
                            }
                            if let Some(op) = e.operation.clone() {
                                self.add_param_front(Value::Effect(peff));
                                self.add_param(Value::Card(target));
                                self.add_param(Value::Int(move_player as i64));
                                self.add_param(Value::Int(reason as u32 as i64));
                                if !self.check_condition(Some(&op), 4) {
                                    continue;
                                }
                            }
                            peff.dec_count(self, PLAYER_NONE);
                            move_player = 1 - move_player;
                        }
                    }
                    if move_player == playerid {
                        if location as u32 == LOCATION_SZONE {
                            flag = ((flag & 0xff) << 8) | 0xffff00ff;
                        } else {
                            flag = (flag & 0xff) | 0xffffff00;
                        }
                    } else if location as u32 == LOCATION_SZONE {
                        flag = ((flag & 0xff) << 24) | 0xffffff;
                    } else {
                        flag = ((flag & 0xff) << 16) | 0xff00ffff;
                    }
                    flag |= 0xe080e080;
                    let code = self.card(target).data.code;
                    let m = self.new_message(MSG_HINT);
                    self.msg(m).u8(HINT_SELECTMSG).u8(move_player).u64(code);
                    self.emplace_process(units::SelectPlace { step: 0, playerid: move_player, count: 1, flag, disable_field: false });
                }
                false
            }
            1 => {
                let mut seq = self.returns.at_i8(2) as i32 as u32;
                if is_field_spell(self) {
                    seq = 5;
                }
                let loc = location as u32;
                if ret != 1 && location != self.card(target).current.location {
                    let cur = self.card(target).current.location as u32;
                    let mut resetflag = 0;
                    if loc & LOCATION_ONFIELD != 0 {
                        resetflag |= RESET_TOFIELD;
                    }
                    if cur & LOCATION_ONFIELD != 0 {
                        resetflag |= RESET_LEAVE;
                    }
                    let peffect = target.is_affected_by_effect(self, EFFECT_PRE_MONSTER);
                    let pre_trap = peffect.is_some_and(|pe| self.effect(pe).value as u32 & TYPE_TRAP != 0);
                    if loc & LOCATION_ONFIELD != 0 && cur & LOCATION_ONFIELD != 0 && !pre_trap && ret != 2 {
                        resetflag |= RESET_MSCHANGE;
                    }
                    target.reset(self, resetflag, RESET_EVENT);
                    target.clear_card_target(self);
                }
                if self.card(target).current.location as u32 & LOCATION_ONFIELD == 0 {
                    target.clear_relate_effect(self);
                }
                let c = self.card_mut(target);
                if ret == 1 {
                    c.current.reason &= !REASON_TEMPORARY;
                }
                if (ret == 0 && location != c.current.location) || ret == 1 {
                    c.set_status(STATUS_SUMMON_TURN, false);
                    c.set_status(STATUS_FLIP_SUMMON_TURN, false);
                    c.set_status(STATUS_SPSUMMON_TURN, false);
                    c.set_status(STATUS_SET_TURN, false);
                    c.set_status(STATUS_FORM_CHANGED, false);
                }
                c.temp.sequence = seq;
                if loc != LOCATION_MZONE {
                    self.returns.set_i32(0, positions as i32);
                    return false;
                }
                let t = c.data.type_;
                if t & TYPE_LINK != 0 && t & TYPE_MONSTER != 0 {
                    self.returns.set_i32(0, POS_FACEUP_ATTACK as i32);
                    return false;
                }
                let code = c.data.code;
                self.emplace_process(units::SelectPosition { step: 0, playerid: move_player, positions, code });
                false
            }
            2 => {
                if self.core.global_flag & GLOBALFLAG_DECK_REVERSE_CHECK != 0 && self.card(target).current.location as u32 == LOCATION_DECK {
                    let curp = self.card(target).current.controler as usize;
                    let curs = self.card(target).current.sequence as usize;
                    if curs > 0 && curs == self.player[curp].list_main.len() - 1 {
                        let ptop = self.player[curp].list_main[curs - 1];
                        let c = self.card(ptop);
                        if self.core.deck_reversed || c.current.position == POS_FACEUP_DEFENSE {
                            let (code, pos) = (c.data.code, c.current.position);
                            let m = self.new_message(MSG_DECK_TOP);
                            self.msg(m).u8(curp as u8).u32(1u32).u32(code).u32(pos);
                        }
                    }
                }
                let code = self.card(target).data.code;
                let from = target.get_info_location(self);
                if let Some(o) = self.card(target).overlay_target {
                    o.xyz_remove(self, target);
                }
                let tseq = self.card(target).temp.sequence;
                self.move_card(playerid, target, location, tseq as u8, pzone);
                let npos = self.returns.at_i32(0) as u32;
                {
                    let c = self.card_mut(target);
                    c.current.position = npos;
                    c.set_status(STATUS_LEAVE_CONFIRMED, false);
                }
                let to = target.get_info_location(self);
                let r = self.card(target).current.reason;
                let m = self.new_message(MSG_MOVE);
                self.msg(m).u32(code).loc(from).loc(to).u32(r);
                if self.card(target).current.location as u32 != LOCATION_MZONE {
                    let eqs = self.card(target).equiping_cards.clone();
                    if !eqs.is_empty() {
                        self.destroy(eqs.clone(), None, REASON_LOST_TARGET + REASON_RULE, PLAYER_NONE as u8, PLAYER_NONE as u8, 0, 0);
                        for pc in public_order(self, &eqs) {
                            pc.unequip(self);
                        }
                    }
                    let mats = self.card(target).xyz_materials.clone();
                    if !mats.is_empty() {
                        self.send_to(CardSet::from_iter(mats), None, REASON_LOST_TARGET + REASON_RULE, PLAYER_NONE as u8, PLAYER_NONE as u8, LOCATION_GRAVE as u16, 0, POS_FACEUP as u8, false);
                    }
                }
                if self.card(target).previous.location as u32 == LOCATION_SZONE && self.card(target).equiping_target.is_some() {
                    target.unequip(self);
                }
                if self.card(target).current.location as u32 == LOCATION_MZONE {
                    let mut eset = Vec::new();
                    self.filter_player_effect(0, EFFECT_MUST_USE_MZONE, &mut eset, false);
                    self.filter_player_effect(1, EFFECT_MUST_USE_MZONE, &mut eset, false);
                    target.filter_effect(self, EFFECT_MUST_USE_MZONE, &mut eset, true);
                    let real_reason = real_location_reason(reason, self.card(target).current.location);
                    for peff in eset {
                        let e = self.effect(peff);
                        if e.is_flag(EFFECT_FLAG_COUNT_LIMIT) && e.count_limit == 0 {
                            continue;
                        }
                        let ctrl = self.card(target).current.controler;
                        if let Some(op) = e.operation.clone() {
                            self.add_param_front(Value::Effect(peff));
                            self.add_param(Value::Int(ctrl as i64));
                            self.add_param(Value::Int(move_player as i64));
                            self.add_param(Value::Int(real_reason as u32 as i64));
                            if !self.check_condition(Some(&op), 4) {
                                continue;
                            }
                        }
                        self.add_param(Value::Int(ctrl as i64));
                        self.add_param(Value::Int(move_player as i64));
                        self.add_param(Value::Int(real_reason as u32 as i64));
                        let mut value = if self.effect(peff).is_flag(EFFECT_FLAG_PLAYER_TARGET) {
                            peff.get_value(self, 3) as u32
                        } else {
                            peff.get_value_card(self, Some(target), 3) as u32
                        };
                        if peff.get_handler_player(self) != self.card(target).current.controler {
                            value >>= 16;
                        }
                        if value & (0x1u32.wrapping_shl(self.card(target).current.sequence)) != 0 {
                            peff.dec_count(self, PLAYER_NONE);
                        }
                    }
                    if let Some(teffect) = target.is_affected_by_effect(self, EFFECT_PRE_MONSTER) {
                        let mut typ = self.effect(teffect).value as u32;
                        if typ & TYPE_TRAP != 0 {
                            typ |= TYPE_TRAPMONSTER | self.card(target).data.type_;
                        }
                        target.reset(self, EFFECT_PRE_MONSTER, RESET_CODE);
                        let pe = self.new_effect();
                        {
                            let e = self.effect_mut(pe);
                            e.owner = Some(target);
                            e.type_ = EFFECT_TYPE_SINGLE as u16;
                            e.code = EFFECT_CHANGE_TYPE;
                            e.flag[0] = EFFECT_FLAG_CANNOT_DISABLE;
                            e.reset_flag = RESET_EVENT + 0x1fc0000;
                            e.value = (TYPE_MONSTER | typ) as i32;
                        }
                        target.add_effect(self, pe);
                        if !self.is_flag(DUEL_TRAP_MONSTERS_NOT_USE_ZONE) && typ & TYPE_TRAPMONSTER != 0 {
                            let prev_seq = self.card(target).previous.sequence;
                            let pe = self.new_effect();
                            {
                                let e = self.effect_mut(pe);
                                e.owner = Some(target);
                                e.type_ = EFFECT_TYPE_FIELD as u16;
                                e.range = LOCATION_MZONE as u16;
                                e.code = EFFECT_USE_EXTRA_SZONE;
                                e.flag[0] = EFFECT_FLAG_CANNOT_DISABLE;
                                e.reset_flag = RESET_EVENT + 0x1fe0000;
                                e.value = (1u32.wrapping_add(0x10000u32.wrapping_shl(prev_seq))) as i32;
                            }
                            target.add_effect(self, pe);
                        }
                    }
                }
                if enable || (ret == 1 && self.card(target).is_position(POS_FACEUP) != 0) {
                    target.enable_field_effect(self, true);
                }
                let c = self.card(target);
                if ret == 1 && c.current.location as u32 == LOCATION_MZONE && c.data.type_ & TYPE_MONSTER == 0 {
                    self.send_to(CardSet::from_card(target), None, REASON_RULE, PLAYER_NONE as u8, PLAYER_NONE as u8, LOCATION_GRAVE as u16, 0, 0, false);
                } else {
                    let (re, r, rp) = self.card_reason_triple(target);
                    if self.card(target).previous.location as u32 == LOCATION_GRAVE {
                        self.raise_single_event(target, None, EVENT_LEAVE_GRAVE, re, r, move_player, 0, 0);
                        self.raise_event_card(Some(target), EVENT_LEAVE_GRAVE, re, r, move_player, 0, 0);
                    }
                    self.raise_single_event(target, None, EVENT_MOVE, re, r, rp, 0, 0);
                    self.raise_event_card(Some(target), EVENT_MOVE, re, r, rp, 0, 0);
                    self.process_single_event();
                    self.process_instant_event();
                }
                self.adjust_disable_check_list();
                false
            }
            3 => {
                self.returns.set_i32(0, 1);
                true
            }
            4 => {
                self.returns.set_i32(0, 0);
                true
            }
            _ => true,
        }
    }

    pub fn process_change_pos(&mut self, arg: &mut units::ChangePos) -> bool {
        let g = arg.targets.unwrap();
        let (reason_effect, reason_player, enable) = (arg.reason_effect, arg.reason_player, arg.enable);
        match arg.step {
            0 => {
                let set = self.group(g).clone();
                for pcard in public_order(self, &set) {
                    let c = self.card(pcard);
                    let npos = (c.position_param & 0xff) as u8;
                    let opos = c.current.position as u8;
                    let t = c.data.type_;
                    let loc = c.current.location as u32;
                    let st = c.get_status(STATUS_SUMMONING | STATUS_SPSUMMON_STEP);
                    if (loc != LOCATION_MZONE && loc != LOCATION_SZONE)
                        || (t & TYPE_LINK != 0 && t & TYPE_MONSTER != 0)
                        || st != 0
                        || (reason_effect.is_some() && !pcard.is_affect_by_effect(self, reason_effect))
                        || npos == opos
                        || (t & TYPE_TOKEN == 0 && opos as u32 & POS_FACEUP != 0 && npos as u32 & POS_FACEDOWN != 0 && !pcard.is_capable_turn_set(self, reason_player))
                    {
                        self.group_mut(g).remove(pcard);
                        continue;
                    }
                    //For cards that cannot be changed to an specific position via effects
                    if reason_effect.is_none() {
                        continue;
                    }
                    let mut eset = Vec::new();
                    pcard.filter_effect(self, EFFECT_CANNOT_CHANGE_POS_E, &mut eset, true);
                    if eset.is_empty() {
                        continue;
                    }
                    let mut disallowpos: u8 = 0;
                    for eff in eset {
                        let val = eff.get_value_effect(self, reason_effect, 0);
                        disallowpos |= if val != 0 { val as u8 } else { (POS_FACEUP | POS_FACEDOWN) as u8 };
                    }
                    if npos & disallowpos != 0 {
                        self.group_mut(g).remove(pcard);
                        continue;
                    }
                }
                arg.to_grave_set.clear();
                false
            }
            1 => {
                let playerid = if arg.oppo_selection { 1 - reason_player } else { reason_player };
                let mut ssets = CardSet::new();
                let set = self.group(g).clone();
                for pcard in public_order(self, &set) {
                    let c = self.card(pcard);
                    let npos = c.position_param & 0xff;
                    let opos = c.current.position;
                    if opos & POS_FACEUP != 0 && npos & POS_FACEDOWN != 0 && pcard.get_type(self, None, 0, PLAYER_NONE as u8) & TYPE_TRAPMONSTER != 0 && self.card(pcard).current.controler == playerid {
                        ssets.insert(pcard);
                    }
                }
                if !ssets.is_empty() {
                    self.return_cards.clear();
                    self.refresh_location_info_instant();
                    let fcount = self.get_useable_count(None, playerid, LOCATION_SZONE as u8, playerid, LocationReason::None, 0xff, None);
                    if fcount <= 0 {
                        for pcard in public_order(self, &ssets) {
                            arg.to_grave_set.insert(pcard);
                            self.group_mut(g).remove(pcard);
                        }
                        arg.step = 2;
                    } else if ssets.len() as i32 > fcount {
                        self.core.select_cards = public_order(self, &ssets);
                        let ct = (ssets.len() as i32 - fcount) as u8;
                        let m = self.new_message(MSG_HINT);
                        self.msg(m).u8(HINT_SELECTMSG).u8(playerid).u64(502u64);
                        self.emplace_process(units::SelectCard { step: 0, playerid, cancelable: false, min: ct, max: ct });
                    }
                } else {
                    arg.step = 2;
                }
                false
            }
            2 => {
                for pcard in self.return_cards.list.clone() {
                    arg.to_grave_set.insert(pcard);
                    self.group_mut(g).remove(pcard);
                }
                false
            }
            3 => {
                if !arg.oppo_selection {
                    arg.oppo_selection = true;
                    arg.step = 0;
                }
                false
            }
            4 => {
                let mut equipings = CardSet::new();
                let mut flips = CardSet::new();
                let mut ssets = CardSet::new();
                let mut pos_changed = CardSet::new();
                let mut cv = self.group(g).to_vec();
                if cv.len() > 1 {
                    sort_by_less(&mut cv, |a, b| card_public_sort(self, a, b));
                }
                for pcard in cv {
                    let c = self.card(pcard);
                    let mut npos = (c.position_param & 0xff) as u8;
                    let opos = c.current.position as u8;
                    let flag = (c.position_param >> 16) as u8;
                    if c.data.type_ & TYPE_TOKEN != 0 && npos as u32 & POS_FACEDOWN != 0 {
                        npos = POS_FACEUP_DEFENSE as u8;
                    }
                    {
                        let c = self.card_mut(pcard);
                        c.previous.position = opos as u32;
                        c.current.position = npos as u32;
                    }
                    if npos as u32 & POS_DEFENSE != 0 && pcard.is_affected_by_effect(self, EFFECT_DEFENSE_ATTACK).is_none() {
                        self.card_mut(pcard).set_status(STATUS_ATTACK_CANCELED, true);
                    }
                    self.card_mut(pcard).set_status(STATUS_JUST_POS, true);
                    let c = self.card(pcard);
                    let (code, ctrl, loc, seq) = (c.data.code, c.current.controler, c.current.location, c.current.sequence);
                    let m = self.new_message(MSG_POS_CHANGE);
                    self.msg(m).u32(code).u8(ctrl).u8(loc).u8(seq as u8).u8(opos).u8(npos);
                    self.core.hint_timing[ctrl as usize] |= TIMING_POS_CHANGE;
                    if opos as u32 & POS_FACEDOWN != 0 && npos as u32 & POS_FACEUP != 0 {
                        let fid = self.infos.field_id;
                        self.infos.field_id += 1;
                        self.card_mut(pcard).fieldid = fid;
                        if self.check_unique_onfield(pcard, ctrl, loc, None).is_some() {
                            self.card_mut(pcard).unique_fieldid = u32::MAX;
                        }
                        if loc as u32 == LOCATION_MZONE {
                            self.raise_single_event(pcard, None, EVENT_FLIP, reason_effect, 0, reason_player, 0, flag as u32);
                            flips.insert(pcard);
                        }
                        if enable {
                            let rtype = reason_effect.map(|re| self.effect(re).type_ as u32);
                            if rtype.is_none() || rtype.unwrap() & 0x7f0 == 0 || loc as u32 != LOCATION_MZONE {
                                pcard.enable_field_effect(self, true);
                            } else {
                                self.core.delayed_enable_set.insert(pcard);
                            }
                        } else {
                            pcard.refresh_disable_status(self);
                        }
                    }
                    if self.card(pcard).current.location as u32 == LOCATION_MZONE {
                        self.raise_single_event(pcard, None, EVENT_CHANGE_POS, reason_effect, 0, reason_player, 0, 0);
                        pos_changed.insert(pcard);
                    }
                    let mut trapmonster = false;
                    if opos as u32 & POS_FACEUP != 0 && npos as u32 & POS_FACEDOWN != 0 {
                        if pcard.get_type(self, None, 0, PLAYER_NONE as u8) & TYPE_TRAPMONSTER != 0 {
                            trapmonster = true;
                        }
                        if self.card(pcard).status & (STATUS_SUMMON_DISABLED | STATUS_ACTIVATE_DISABLED) != 0 {
                            self.card_mut(pcard).set_status(STATUS_SUMMON_DISABLED | STATUS_ACTIVATE_DISABLED, false);
                        }
                        pcard.reset(self, RESET_TURN_SET, RESET_EVENT);
                        pcard.clear_card_target(self);
                        self.card_mut(pcard).set_status(STATUS_SET_TURN, true);
                        pcard.enable_field_effect(self, false);
                        let c = self.card_mut(pcard);
                        c.previous.location = 0;
                        c.summon.type_ &= 0xdf00ffff;
                        if c.summon.type_ & SUMMON_TYPE_PENDULUM == SUMMON_TYPE_PENDULUM {
                            c.summon.type_ &= 0xf000ffff;
                        }
                        c.spsummon_counter = [0, 0];
                        c.spsummon_counter_rst = [0, 0];
                    }
                    if npos as u32 & POS_FACEDOWN != 0 {
                        let eqs = self.card(pcard).equiping_cards.clone();
                        if !eqs.is_empty() {
                            for pc in public_order(self, &eqs) {
                                equipings.insert(pc);
                                pc.unequip(self);
                            }
                        }
                        if self.card(pcard).equiping_target.is_some() {
                            pcard.unequip(self);
                        }
                    }
                    if trapmonster {
                        self.refresh_location_info_instant();
                        let ctrl = self.card(pcard).current.controler;
                        self.move_to_field(pcard, ctrl, ctrl, LOCATION_SZONE as u16, POS_FACEDOWN as u8, false, 2, 0xff, false, LocationReason::None, true);
                        self.raise_single_event(pcard, None, EVENT_SSET, reason_effect, 0, reason_player, 0, 0);
                        ssets.insert(pcard);
                    }
                }
                self.adjust_instant();
                self.process_single_event();
                if !flips.is_empty() {
                    self.raise_event(flips, EVENT_FLIP, reason_effect, 0, reason_player, 0, 0);
                }
                if !ssets.is_empty() {
                    self.raise_event(ssets, EVENT_SSET, reason_effect, 0, reason_player, 0, 0);
                }
                if !pos_changed.is_empty() {
                    self.raise_event(pos_changed, EVENT_CHANGE_POS, reason_effect, 0, reason_player, 0, 0);
                }
                self.process_instant_event();
                if !equipings.is_empty() {
                    self.destroy(equipings, None, REASON_LOST_TARGET + REASON_RULE, PLAYER_NONE as u8, PLAYER_NONE as u8, 0, 0);
                }
                let to_grave_set = std::mem::take(&mut arg.to_grave_set);
                if !to_grave_set.is_empty() {
                    self.send_to(to_grave_set, None, REASON_RULE, PLAYER_NONE as u8, PLAYER_NONE as u8, LOCATION_GRAVE as u16, 0, POS_FACEUP as u8, false);
                }
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

    /// The continuous chain of a replacement effect (steps 0, 5, 10, 12).
    fn replace_chain(&mut self, replace_effect: EffectId, e: &Event) {
        let ch = Chain {
            chain_id: 0,
            chain_count: 0,
            triggering_effect: Some(replace_effect),
            triggering_player: e.event_player,
            evt: e.clone(),
            target_cards: None,
            target_player: PLAYER_NONE as u8,
            target_param: 0,
            disable_player: PLAYER_NONE as u8,
            disable_reason: None,
            flag: 0,
            ..Default::default()
        };
        self.core.continuous_chain.push_back(ch);
    }
    fn replace_event(&mut self, replace_effect: EffectId, g: GroupRef, from: CardId) -> Event {
        let (re, r, rp) = self.card_reason_triple(from);
        Event {
            trigger_card: None,
            event_cards: Some(self.group(g).clone()),
            event_code: 0,
            event_player: replace_effect.get_handler_player(self),
            event_value: 0,
            reason: r,
            reason_effect: re,
            reason_player: rp,
            global_id: 0,
        }
    }
    fn self_destroy_related(&self, pcard: CardId) -> bool {
        self.card(pcard).current.reason_effect.is_some_and(|re| re.is_self_destroy_related(self))
    }

    pub fn process_operation_replace(&mut self, arg: &mut units::OperationReplace) -> bool {
        // steps 15/16 (the destroy-replace chain of a battle) have no replace effect or targets (nullptr in the C++)
        match arg.step {
            15 => {
                let Some(ch) = self.core.desrep_chain.pop_front() else { return true };
                self.core.continuous_chain.push_back(ch);
                let ch = self.core.continuous_chain.back().unwrap();
                let reffect = ch.triggering_effect.unwrap();
                if self.effect(reffect).operation.is_none() {
                    return false;
                }
                let e = ch.evt.clone();
                self.core.sub_solving_event.push_back(e);
                let rp = reffect.get_handler_player(self);
                self.emplace_process(units::ExecuteOperation { step: 0, triggering_player: rp, shuffle_check_was_disabled: false, triggering_effect: reffect });
                return false;
            }
            16 => {
                self.core.continuous_chain.pop_back();
                arg.step = 14;
                return false;
            }
            _ => {}
        }
        let replace_effect = arg.replace_effect.unwrap();
        let g = arg.targets.unwrap();
        let (target, is_destroy) = (arg.target, arg.is_destroy);
        let hp = replace_effect.get_handler_player(self);
        match arg.step {
            0 | 10 => {
                if self.returns.at_i32(0) != 0 {
                    return true;
                }
                if self.effect(replace_effect).target.is_none() {
                    return true;
                }
                let e = self.replace_event(replace_effect, g, target.unwrap());
                if !replace_effect.is_activateable(self, hp, &e, false, false, false, false, false) {
                    return true;
                }
                self.replace_chain(replace_effect, &e);
                if arg.step == 0 {
                    self.core.solving_event.push_front(e.clone());
                }
                self.core.sub_solving_event.push_back(e);
                self.emplace_process(units::ExecuteTarget { step: 0, triggering_player: hp, shuffle_check_was_disabled: false, triggering_effect: replace_effect });
                false
            }
            1 => {
                let target = target.unwrap();
                if self.returns.at_i32(0) != 0 {
                    if !self.self_destroy_related(target) {
                        self.group_mut(g).remove(target);
                        self.restore_temp_reason(target);
                        if is_destroy {
                            self.core.destroy_canceled.insert(target);
                        }
                    }
                    replace_effect.dec_count(self, hp as u32);
                } else {
                    arg.step = 2;
                }
                false
            }
            2 | 7 => {
                if self.effect(replace_effect).operation.is_none() {
                    return false;
                }
                let e = self.core.solving_event.front().unwrap().clone();
                self.core.sub_solving_event.push_back(e);
                self.emplace_process(units::ExecuteOperation { step: 0, triggering_player: hp, shuffle_check_was_disabled: false, triggering_effect: replace_effect });
                false
            }
            3 | 8 => {
                self.core.continuous_chain.pop_back();
                self.core.solving_event.pop_front();
                true
            }
            5 | 12 => {
                let Some(pc) = self.group(g).first() else { return true };
                let e = self.replace_event(replace_effect, g, pc);
                if !replace_effect.is_activateable(self, hp, &e, false, false, false, false, false) || !self.effect(replace_effect).has_value() {
                    return true;
                }
                self.replace_chain(replace_effect, &e);
                if arg.step == 5 {
                    self.core.solving_event.push_front(e.clone());
                }
                self.core.sub_solving_event.push_back(e);
                self.emplace_process(units::ExecuteTarget { step: 0, triggering_player: hp, shuffle_check_was_disabled: false, triggering_effect: replace_effect });
                false
            }
            6 => {
                if self.returns.at_i32(0) != 0 {
                    let set = self.group(g).clone();
                    for pc in public_order(self, &set) {
                        if replace_effect.get_value_card(self, Some(pc), 0) != 0 && !self.self_destroy_related(pc) {
                            self.restore_temp_reason(pc);
                            if is_destroy {
                                self.core.destroy_canceled.insert(pc);
                            }
                            self.group_mut(g).remove(pc);
                        }
                    }
                    replace_effect.dec_count(self, hp as u32);
                } else {
                    arg.step = 7;
                }
                false
            }
            11 => {
                let target = target.unwrap();
                if self.returns.at_i32(0) != 0 {
                    self.group_mut(g).remove(target);
                    self.restore_temp_reason(target);
                    if is_destroy {
                        self.core.destroy_canceled.insert(target);
                    }
                    replace_effect.dec_count(self, hp as u32);
                    let front = self.core.continuous_chain.front().unwrap().clone();
                    self.core.desrep_chain.push_back(front);
                }
                self.core.continuous_chain.pop_front();
                true
            }
            13 => {
                if self.returns.at_i32(0) != 0 {
                    let set = self.group(g).clone();
                    for pc in public_order(self, &set) {
                        if replace_effect.get_value_card(self, Some(pc), 0) != 0 {
                            self.restore_temp_reason(pc);
                            if is_destroy {
                                self.core.destroy_canceled.insert(pc);
                            }
                            self.group_mut(g).remove(pc);
                        }
                    }
                    replace_effect.dec_count(self, hp as u32);
                    let front = self.core.continuous_chain.front().unwrap().clone();
                    self.core.desrep_chain.push_back(front);
                }
                self.core.continuous_chain.pop_front();
                true
            }
            _ => true,
        }
    }

    pub fn process_activate_effect(&mut self, arg: &mut units::ActivateEffect) -> bool {
        let peffect = arg.peffect;
        match arg.step {
            0 => {
                let phandler = peffect.get_handler(self);
                let playerid = self.card(phandler).current.controler;
                self.nil_event.event_code = EVENT_FREE_CHAIN;
                let ne = self.nil_event.clone();
                if !peffect.is_activateable(self, playerid, &ne, false, false, false, false, false) {
                    return true;
                }
                let mut ch = Chain { flag: 0, ..Default::default() };
                ch.chain_id = self.infos.field_id as u16;
                self.infos.field_id += 1;
                ch.evt.event_code = self.effect(peffect).code;
                ch.evt.event_player = PLAYER_NONE as u8;
                ch.evt.event_value = 0;
                ch.evt.event_cards = None;
                ch.evt.reason = 0;
                ch.evt.reason_effect = None;
                ch.evt.reason_player = PLAYER_NONE as u8;
                ch.triggering_effect = Some(peffect);
                self.set_triggering_state(&mut ch, phandler);
                ch.triggering_player = playerid;
                self.core.new_chains.push_back(ch);
                self.card_mut(phandler).set_status(STATUS_CHAINING, true);
                peffect.dec_count(self, playerid as u32);
                self.emplace_process(units::AddChain { step: 0, is_activated_effect: false });
                self.emplace_process(units::QuickEffect { step: 0, skip_freechain: false, is_opponent: false, priority_player: 1 - playerid });
                self.infos.priorities = [0, 0];
                false
            }
            1 => {
                self.core.chain_limit.clear();
                for i in 0..self.core.current_chain.len() {
                    let te = self.core.current_chain[i].triggering_effect.unwrap();
                    let h = te.get_handler(self);
                    self.card_mut(h).set_status(STATUS_CHAINING, false);
                }
                self.emplace_process(units::SolveChain { step: 0, skip_trigger: false, skip_freechain: false, skip_new: false, backed_up_operation: None });
                true
            }
            _ => true,
        }
    }
}
