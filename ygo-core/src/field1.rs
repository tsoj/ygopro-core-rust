//! The field (`field.cpp` part 1: moving cards, zones and their counts).

use crate::constants::*;
use crate::duel::Duel;
use crate::types::*;
use crate::units::LocationReason;

#[inline]
pub fn field_used_count(flag: u32) -> i32 {
    (flag & 0x1f).count_ones() as i32
}

impl Duel {
    pub fn reload_field_info(&mut self) {
        let mut chain_info = Vec::new();
        for ch in self.core.current_chain.clone() {
            let pe = ch.triggering_effect.unwrap();
            let h = pe.get_handler(self);
            chain_info.push((self.card(h).data.code, h.get_info_location(self), ch.triggering_controler, ch.triggering_location as u8, ch.triggering_sequence, self.effect(pe).description));
        }
        let m = self.new_message(MSG_RELOAD_FIELD);
        let opts = self.core.duel_options;
        self.msg(m).u32(opts);
        for p in 0..2 {
            let pl = &self.player[p];
            let lp = pl.lp;
            let mut data = Vec::new();
            let mut tmp = crate::duel::Message::default();
            tmp.u32(lp);
            for c in pl.list_mzone.iter().chain(pl.list_szone.iter()) {
                match c {
                    Some(c) => {
                        let card = &self.cards[c.0 as usize];
                        tmp.u8(1u8).u8(card.current.position).u32(card.xyz_materials.len());
                    }
                    None => {
                        tmp.u8(0u8);
                    }
                }
            }
            tmp.u32(pl.list_main.len()).u32(pl.list_hand.len()).u32(pl.list_grave.len()).u32(pl.list_remove.len()).u32(pl.list_extra.len()).u32(pl.extra_p_count);
            data.extend_from_slice(&tmp.data);
            self.msg(m).bytes(&data);
        }
        self.msg(m).u32(chain_info.len());
        for (code, loc, ctrl, tloc, tseq, desc) in chain_info {
            self.msg(m).u32(code).loc(loc).u8(ctrl).u8(tloc).u32(tseq).u64(desc);
        }
    }

    /// The core of moving cards. Checks Fusion/S/X monster redirection by the rule, sets fieldid_r.
    pub fn add_card(&mut self, playerid: u8, pcard: CardId, mut location: u8, sequence: u8, pzone: bool) {
        if self.card(pcard).current.location != 0 {
            return;
        }
        if !self.is_location_useable(playerid as u32, location as u32, sequence as u32) {
            return;
        }
        let p = playerid as usize;
        let dtype = self.card(pcard).data.type_;
        if pcard.is_extra_deck_monster(self) || dtype & TYPE_FUSION != 0 {
            if location as u32 & (LOCATION_HAND | LOCATION_DECK) != 0 {
                location = LOCATION_EXTRA as u8;
                self.card_mut(pcard).sendto_param.position = POS_FACEDOWN_DEFENSE as u8;
            }
        } else if location as u32 == LOCATION_EXTRA
            && (dtype & TYPE_PENDULUM == 0 || self.card(pcard).sendto_param.position as u32 & POS_FACEUP == 0)
        {
            location = LOCATION_DECK as u8;
            self.card_mut(pcard).sendto_param.position = POS_FACEDOWN_DEFENSE as u8;
        }
        {
            let c = self.card_mut(pcard);
            c.current.controler = playerid;
            c.current.location = location;
            if location as u32 & (LOCATION_DECK | LOCATION_HAND | LOCATION_EXTRA) != 0 {
                c.set_status(STATUS_JUST_POS | STATUS_CONTINUOUS_POS | STATUS_ATTACK_CANCELED | STATUS_ACT_FROM_HAND, false);
            }
        }
        match location as u32 {
            LOCATION_MZONE => {
                self.player[p].list_mzone[sequence as usize] = Some(pcard);
                self.card_mut(pcard).current.sequence = sequence as u32;
            }
            LOCATION_SZONE => {
                self.player[p].list_szone[sequence as usize] = Some(pcard);
                self.card_mut(pcard).current.sequence = sequence as u32;
            }
            LOCATION_DECK => {
                if sequence == 0 {
                    self.player[p].list_main.push(pcard);
                    let n = self.player[p].list_main.len() as u32;
                    self.card_mut(pcard).current.sequence = n - 1;
                } else if sequence == 1 {
                    self.player[p].list_main.insert(0, pcard);
                    self.reset_sequence(playerid, LOCATION_DECK as u8);
                } else {
                    self.player[p].list_main.push(pcard);
                    let n = self.player[p].list_main.len() as u32;
                    self.card_mut(pcard).current.sequence = n - 1;
                    if !self.core.shuffle_check_disabled {
                        self.core.shuffle_deck_check[p] = true;
                    }
                }
                self.card_mut(pcard).sendto_param.position = POS_FACEDOWN as u8;
            }
            LOCATION_HAND => {
                self.player[p].list_hand.push(pcard);
                let n = self.player[p].list_hand.len() as u32;
                self.card_mut(pcard).current.sequence = n - 1;
                let pos = if pcard.is_affected_by_effect(self, EFFECT_PUBLIC).is_some() { POS_FACEUP } else { POS_FACEDOWN };
                self.card_mut(pcard).sendto_param.position = pos as u8;
                if self.card(pcard).current.reason & REASON_DRAW == 0 && !self.core.shuffle_check_disabled {
                    self.core.shuffle_hand_check[p] = true;
                }
            }
            LOCATION_GRAVE => {
                self.player[p].list_grave.push(pcard);
                let n = self.player[p].list_grave.len() as u32;
                self.card_mut(pcard).current.sequence = n - 1;
            }
            LOCATION_REMOVED => {
                self.player[p].list_remove.push(pcard);
                let n = self.player[p].list_remove.len() as u32;
                self.card_mut(pcard).current.sequence = n - 1;
            }
            LOCATION_EXTRA => {
                let faceup_p = dtype & TYPE_PENDULUM != 0 && self.card(pcard).sendto_param.position as u32 & POS_FACEUP != 0;
                if self.player[p].extra_p_count == 0 || faceup_p {
                    self.player[p].list_extra.push(pcard);
                } else {
                    let at = self.player[p].list_extra.len() - self.player[p].extra_p_count as usize;
                    self.player[p].list_extra.insert(at, pcard);
                }
                if faceup_p {
                    self.player[p].extra_p_count += 1;
                }
                self.reset_sequence(playerid, LOCATION_EXTRA as u8);
            }
            _ => {}
        }
        self.card_mut(pcard).current.pzone = pzone;
        pcard.apply_field_effect(self);
        let fid = self.infos.field_id;
        self.infos.field_id += 1;
        let turn_id = self.infos.turn_id;
        {
            let c = self.card_mut(pcard);
            c.fieldid = fid;
            c.fieldid_r = fid;
            c.turnid = turn_id as u16;
        }
        let (ctrl, loc) = (self.card(pcard).current.controler, self.card(pcard).current.location);
        if self.check_unique_onfield(pcard, ctrl, loc, None).is_some() {
            self.card_mut(pcard).unique_fieldid = u32::MAX;
        }
        if location as u32 == LOCATION_MZONE {
            self.player[p].used_location |= 1 << sequence;
        }
        if location as u32 == LOCATION_SZONE {
            self.player[p].used_location |= 256 << sequence;
        }
    }

    pub fn remove_card(&mut self, pcard: CardId) {
        let c = self.card(pcard);
        if c.current.controler == PLAYER_NONE as u8 || c.current.location == 0 {
            return;
        }
        let playerid = c.current.controler;
        let p = playerid as usize;
        let seq = c.current.sequence as usize;
        let loc = c.current.location as u32;
        match loc {
            LOCATION_MZONE => self.player[p].list_mzone[seq] = None,
            LOCATION_SZONE => self.player[p].list_szone[seq] = None,
            LOCATION_DECK => {
                self.player[p].list_main.remove(seq);
                self.reset_sequence(playerid, LOCATION_DECK as u8);
                if !self.core.shuffle_check_disabled {
                    self.core.shuffle_deck_check[p] = true;
                }
            }
            LOCATION_HAND => {
                self.player[p].list_hand.remove(seq);
                self.reset_sequence(playerid, LOCATION_HAND as u8);
            }
            LOCATION_GRAVE => {
                self.player[p].list_grave.remove(seq);
                self.reset_sequence(playerid, LOCATION_GRAVE as u8);
            }
            LOCATION_REMOVED => {
                self.player[p].list_remove.remove(seq);
                self.reset_sequence(playerid, LOCATION_REMOVED as u8);
            }
            LOCATION_EXTRA => {
                self.player[p].list_extra.remove(seq);
                self.reset_sequence(playerid, LOCATION_EXTRA as u8);
                let c = self.card(pcard);
                if c.data.type_ & TYPE_PENDULUM != 0 && c.current.position & POS_FACEUP != 0 {
                    self.player[p].extra_p_count -= 1;
                }
            }
            _ => {}
        }
        pcard.cancel_field_effect(self);
        let (cloc, cseq) = (self.card(pcard).current.location as u32, self.card(pcard).current.sequence);
        if cloc == LOCATION_MZONE {
            self.player[p].used_location &= !(1 << cseq);
        }
        if cloc == LOCATION_SZONE {
            self.player[p].used_location &= !(256 << cseq);
        }
        if !self.core.current_chain.is_empty() {
            self.core.just_sent_cards.insert(pcard);
        }
        let c = self.card_mut(pcard);
        c.previous.controler = c.current.controler;
        c.previous.location = c.current.location;
        c.previous.sequence = c.current.sequence;
        c.previous.position = c.current.position;
        c.previous.pzone = c.current.pzone;
        c.current.controler = PLAYER_NONE as u8;
        c.current.location = 0;
        c.current.sequence = 0;
    }

    fn save_previous(&mut self, pcard: CardId) {
        let c = self.card_mut(pcard);
        c.previous.controler = c.current.controler;
        c.previous.location = c.current.location;
        c.previous.sequence = c.current.sequence;
        c.previous.position = c.current.position;
        c.previous.pzone = c.current.pzone;
    }

    /// Moves a card; checks Fusion/S/X monster redirection by the rule.
    pub fn move_card(&mut self, playerid: u8, pcard: CardId, mut location: u8, sequence: u8, pzone: bool) -> bool {
        if !self.is_location_useable(playerid as u32, location as u32, sequence as u32) {
            return false;
        }
        let preplayer = self.card(pcard).current.controler;
        let presequence = self.card(pcard).current.sequence as u8;
        if pcard.is_extra_deck_monster(self) && location as u32 & (LOCATION_HAND | LOCATION_DECK) != 0 {
            location = LOCATION_EXTRA as u8;
            self.card_mut(pcard).sendto_param.position = POS_FACEDOWN_DEFENSE as u8;
        }
        if self.card(pcard).current.location != 0 {
            let cur = self.card(pcard).current;
            if cur.location == location && cur.pzone == pzone {
                if cur.location as u32 == LOCATION_DECK {
                    if preplayer == playerid {
                        let code = self.card(pcard).data.code;
                        let info = pcard.get_info_location(self);
                        let m = self.new_message(MSG_MOVE);
                        self.msg(m).u32(code).loc(info);
                        let pp = preplayer as usize;
                        self.player[pp].list_main.remove(cur.sequence as usize);
                        let p = playerid as usize;
                        if sequence == 0 {
                            self.player[p].list_main.push(pcard);
                        } else if sequence == 1 {
                            self.player[p].list_main.insert(0, pcard);
                        } else {
                            self.player[p].list_main.push(pcard);
                            if !self.core.shuffle_check_disabled {
                                self.core.shuffle_deck_check[p] = true;
                            }
                        }
                        self.reset_sequence(playerid, LOCATION_DECK as u8);
                        self.card_mut(pcard).previous.controler = preplayer;
                        self.card_mut(pcard).current.controler = playerid;
                        let info = pcard.get_info_location(self);
                        let reason = self.card(pcard).current.reason;
                        self.msg(m).loc(info).u32(reason);
                        return true;
                    } else {
                        self.remove_card(pcard);
                    }
                } else if location as u32 & LOCATION_ONFIELD != 0 {
                    if playerid == preplayer && sequence == presequence {
                        return false;
                    }
                    let p = playerid as usize;
                    if location as u32 == LOCATION_MZONE {
                        if sequence as usize >= self.player[p].list_mzone.len() || self.player[p].list_mzone[sequence as usize].is_some() {
                            return false;
                        }
                    } else if sequence as usize >= self.player[p].list_szone.len() || self.player[p].list_szone[sequence as usize].is_some() {
                        return false;
                    }
                    let mut message = None;
                    if preplayer == playerid {
                        let code = self.card(pcard).data.code;
                        let info = pcard.get_info_location(self);
                        let m = self.new_message(MSG_MOVE);
                        self.msg(m).u32(code).loc(info);
                        message = Some(m);
                    }
                    if !self.core.current_chain.is_empty() {
                        self.core.just_sent_cards.insert(pcard);
                    }
                    self.save_previous(pcard);
                    let pp = preplayer as usize;
                    if location as u32 == LOCATION_MZONE {
                        self.player[pp].list_mzone[presequence as usize] = None;
                        self.player[pp].used_location &= !(1 << presequence);
                        self.player[p].list_mzone[sequence as usize] = Some(pcard);
                        self.player[p].used_location |= 1 << sequence;
                    } else {
                        self.player[pp].list_szone[presequence as usize] = None;
                        self.player[pp].used_location &= !(256 << presequence);
                        self.player[p].list_szone[sequence as usize] = Some(pcard);
                        self.player[p].used_location |= 256 << sequence;
                    }
                    self.card_mut(pcard).current.controler = playerid;
                    self.card_mut(pcard).current.sequence = sequence as u32;
                    if let Some(m) = message {
                        let info = pcard.get_info_location(self);
                        let reason = self.card(pcard).current.reason;
                        self.msg(m).loc(info).u32(reason);
                    } else {
                        let fid = self.infos.field_id;
                        self.infos.field_id += 1;
                        self.card_mut(pcard).fieldid = fid;
                        let (ctrl, loc) = (self.card(pcard).current.controler, self.card(pcard).current.location);
                        if self.check_unique_onfield(pcard, ctrl, loc, None).is_some() {
                            self.card_mut(pcard).unique_fieldid = u32::MAX;
                        }
                    }
                    return true;
                } else if location as u32 == LOCATION_HAND {
                    if preplayer == playerid {
                        return false;
                    }
                    self.remove_card(pcard);
                } else {
                    let ctrl = cur.controler;
                    let cp = ctrl as usize;
                    let list_len = match location as u32 {
                        LOCATION_GRAVE => self.player[cp].list_grave.len(),
                        LOCATION_REMOVED => self.player[cp].list_remove.len(),
                        _ => 0,
                    };
                    if (location as u32 == LOCATION_GRAVE || location as u32 == LOCATION_REMOVED) && cur.sequence as usize == list_len - 1 {
                        return false;
                    }
                    let code = self.card(pcard).data.code;
                    let info = pcard.get_info_location(self);
                    let m = self.new_message(MSG_MOVE);
                    self.msg(m).u32(code).loc(info);
                    let s = cur.sequence as usize;
                    let which = location as u32;
                    let list = match which {
                        LOCATION_GRAVE => &mut self.player[cp].list_grave,
                        LOCATION_REMOVED => &mut self.player[cp].list_remove,
                        _ => &mut self.player[cp].list_extra,
                    };
                    list.remove(s);
                    list.push(pcard);
                    let rs = match which {
                        LOCATION_GRAVE => LOCATION_GRAVE,
                        LOCATION_REMOVED => LOCATION_REMOVED,
                        _ => LOCATION_EXTRA,
                    };
                    self.reset_sequence(ctrl, rs as u8);
                    let info = pcard.get_info_location(self);
                    let reason = self.card(pcard).current.reason;
                    self.msg(m).loc(info).u32(reason);
                    return true;
                }
            } else {
                let c = self.card(pcard);
                if c.data.type_ & TYPE_PENDULUM != 0
                    && location as u32 == LOCATION_GRAVE
                    && pcard.is_capable_send_to_extra(self, playerid)
                    && {
                        let c = self.card(pcard);
                        (c.current.location as u32 == LOCATION_MZONE && !c.is_status(STATUS_SUMMON_DISABLED))
                            || (c.current.location as u32 == LOCATION_SZONE && !c.is_status(STATUS_ACTIVATE_DISABLED))
                    }
                {
                    location = LOCATION_EXTRA as u8;
                    self.card_mut(pcard).sendto_param.position = POS_FACEUP_DEFENSE as u8;
                }
                self.remove_card(pcard);
            }
        }
        self.add_card(playerid, pcard, location, sequence, pzone);
        true
    }

    pub fn swap_card_seq(&mut self, pcard1: CardId, pcard2: CardId, new_sequence1: u8, new_sequence2: u8) {
        let c1 = self.card(pcard1).current;
        let c2 = self.card(pcard2).current;
        let (p1, p2) = (c1.controler, c2.controler);
        let (l1, l2) = (c1.location, c2.location);
        let (s1, s2) = (c1.sequence as u8, c2.sequence as u8);
        let info1 = pcard1.get_info_location(self);
        let info2 = pcard2.get_info_location(self);
        if l1 as u32 & LOCATION_ONFIELD == 0 || l2 as u32 & LOCATION_ONFIELD == 0 {
            return;
        }
        if (new_sequence1 != s1 && !self.is_location_useable(p1 as u32, l1 as u32, new_sequence1 as u32))
            || (new_sequence2 != s2 && !self.is_location_useable(p2 as u32, l2 as u32, new_sequence2 as u32))
        {
            return;
        }
        if p1 == p2 && l1 == l2 && (new_sequence1 == s2 || new_sequence2 == s1) {
            return;
        }
        if l1 == l2 {
            {
                let a = self.card_mut(pcard1);
                a.previous.controler = p1;
                a.previous.location = l1;
                a.previous.sequence = s1 as u32;
                a.previous.position = a.current.position;
                a.previous.pzone = a.current.pzone;
                a.current.controler = p2;
                a.current.location = l2;
                a.current.sequence = new_sequence2 as u32;
            }
            {
                let b = self.card_mut(pcard2);
                b.previous.controler = p2;
                b.previous.location = l2;
                b.previous.sequence = s2 as u32;
                b.previous.position = b.current.position;
                b.previous.pzone = b.current.pzone;
                b.current.controler = p1;
                b.current.location = l1;
                b.current.sequence = new_sequence1 as u32;
            }
            if p1 != p2 {
                let f1 = self.infos.field_id;
                self.infos.field_id += 1;
                self.card_mut(pcard1).fieldid = f1;
                let f2 = self.infos.field_id;
                self.infos.field_id += 1;
                self.card_mut(pcard2).fieldid = f2;
                for pc in [pcard1, pcard2] {
                    let (ctrl, loc) = (self.card(pc).current.controler, self.card(pc).current.location);
                    if self.check_unique_onfield(pc, ctrl, loc, None).is_some() {
                        self.card_mut(pc).unique_fieldid = u32::MAX;
                    }
                }
            }
            let (pp1, pp2) = (p1 as usize, p2 as usize);
            if l1 as u32 == LOCATION_MZONE {
                self.player[pp1].list_mzone[s1 as usize] = None;
                self.player[pp1].used_location &= !(1 << s1);
                self.player[pp2].list_mzone[s2 as usize] = None;
                self.player[pp2].used_location &= !(1 << s2);
                self.player[pp2].list_mzone[new_sequence2 as usize] = Some(pcard1);
                self.player[pp2].used_location |= 1 << new_sequence2;
                self.player[pp1].list_mzone[new_sequence1 as usize] = Some(pcard2);
                self.player[pp1].used_location |= 1 << new_sequence1;
            } else if l1 as u32 == LOCATION_SZONE {
                self.player[pp1].list_szone[s1 as usize] = None;
                self.player[pp1].used_location &= !(256 << s1);
                self.player[pp2].list_szone[s2 as usize] = None;
                self.player[pp2].used_location &= !(256 << s2);
                self.player[pp2].list_szone[new_sequence2 as usize] = Some(pcard1);
                self.player[pp2].used_location |= 256 << new_sequence2;
                self.player[pp1].list_szone[new_sequence1 as usize] = Some(pcard2);
                self.player[pp1].used_location |= 256 << new_sequence1;
            }
        } else {
            self.remove_card(pcard1);
            self.remove_card(pcard2);
            self.add_card(p2, pcard1, l2, new_sequence2, false);
            self.add_card(p1, pcard2, l1, new_sequence1, false);
        }
        let move_msg = |d: &mut Duel, pc: CardId, info: LocInfo| {
            let code = d.card(pc).data.code;
            let now = pc.get_info_location(d);
            let m = d.new_message(MSG_MOVE);
            d.msg(m).u32(code).loc(info).loc(now).u32(0u32);
        };
        if s1 == new_sequence1 && s2 == new_sequence2 {
            let code1 = self.card(pcard1).data.code;
            let code2 = self.card(pcard2).data.code;
            let m = self.new_message(MSG_SWAP);
            self.msg(m).u32(code1).loc(info1).u32(code2).loc(info2);
        } else if s1 == new_sequence1 {
            move_msg(self, pcard1, info1);
            move_msg(self, pcard2, info2);
        } else {
            move_msg(self, pcard2, info2);
            move_msg(self, pcard1, info1);
        }
    }
    pub fn swap_card(&mut self, pcard1: CardId, pcard2: CardId) {
        let s1 = self.card(pcard1).current.sequence as u8;
        let s2 = self.card(pcard2).current.sequence as u8;
        self.swap_card_seq(pcard1, pcard2, s1, s2);
    }
    /// add EFFECT_SET_CONTROL
    pub fn set_control(&mut self, pcard: CardId, playerid: u8, reset_phase: u16, reset_count: u8) {
        if (self.core.remove_brainwashing && pcard.is_affected_by_effect(self, EFFECT_REMOVE_BRAINWASHING).is_some())
            || pcard.refresh_control_status(self).0 == playerid
        {
            return;
        }
        let pe = self.new_effect();
        let owner = match self.core.reason_effect {
            Some(re) => re.get_handler(self),
            None => pcard,
        };
        {
            let e = self.effect_mut(pe);
            e.owner = Some(owner);
            e.handler = Some(pcard);
            e.type_ = EFFECT_TYPE_SINGLE as u16;
            e.code = EFFECT_SET_CONTROL;
            e.value = playerid as i32;
            e.flag[0] = EFFECT_FLAG_CANNOT_DISABLE;
            e.reset_flag = RESET_EVENT | 0xc6c0000;
            if reset_count != 0 {
                e.reset_flag |= RESET_PHASE | reset_phase as u32;
                if e.reset_flag & (RESET_SELF_TURN | RESET_OPPO_TURN) == 0 {
                    e.reset_flag |= RESET_SELF_TURN | RESET_OPPO_TURN;
                }
                e.reset_count = reset_count as u16;
            }
        }
        pcard.add_effect(self, pe);
        self.card_mut(pcard).current.controler = playerid;
    }
    pub fn get_field_card(&self, playerid: u32, location: u32, sequence: u32) -> Option<CardId> {
        let pl = &self.player[playerid as usize];
        let s = sequence as usize;
        match location {
            LOCATION_MZONE => pl.list_mzone.get(s).copied().flatten(),
            LOCATION_MMZONE => {
                if self.is_flag(DUEL_3_COLUMNS_FIELD) {
                    if sequence < 3 {
                        return pl.list_mzone[s + 1];
                    }
                } else if sequence < 5 {
                    return pl.list_mzone[s];
                }
                None
            }
            LOCATION_EMZONE => {
                if sequence < 2 {
                    pl.list_mzone[s + 5]
                } else {
                    None
                }
            }
            LOCATION_SZONE => pl.list_szone.get(s).copied().flatten(),
            LOCATION_STZONE => {
                if self.is_flag(DUEL_3_COLUMNS_FIELD) {
                    if sequence < 3 {
                        return pl.list_szone[s + 1];
                    }
                } else if sequence < 5 {
                    return pl.list_szone[s];
                }
                None
            }
            LOCATION_FZONE => {
                if sequence == 0 {
                    pl.list_szone[5]
                } else {
                    None
                }
            }
            LOCATION_PZONE => {
                if sequence < 2 {
                    let pc = pl.list_szone[self.get_pzone_index(sequence as u8, playerid as u8) as usize];
                    pc.filter(|&c| self.card(c).current.pzone)
                } else {
                    None
                }
            }
            LOCATION_DECK => pl.list_main.get(s).copied(),
            LOCATION_HAND => pl.list_hand.get(s).copied(),
            LOCATION_GRAVE => pl.list_grave.get(s).copied(),
            LOCATION_REMOVED => pl.list_remove.get(s).copied(),
            LOCATION_EXTRA => pl.list_extra.get(s).copied(),
            _ => None,
        }
    }
    pub fn is_field_location_valid(&self, mut location: u32, mut sequence: u32) -> bool {
        if location == LOCATION_EMZONE {
            sequence += 5;
            location = LOCATION_MZONE;
        }
        if location == LOCATION_MMZONE {
            location = LOCATION_MZONE;
            sequence += self.is_flag(DUEL_3_COLUMNS_FIELD) as u32;
        }
        if location == LOCATION_STZONE {
            location = LOCATION_SZONE;
            sequence += self.is_flag(DUEL_3_COLUMNS_FIELD) as u32;
        }
        if location & LOCATION_ONFIELD == 0 {
            return true;
        }
        match location {
            LOCATION_MZONE => sequence < 7,
            LOCATION_SZONE => sequence < 8,
            LOCATION_FZONE => sequence < 2,
            LOCATION_PZONE => sequence < 3,
            _ => true,
        }
    }
    /// whether the given slot is available
    pub fn is_location_useable(&self, playerid: u32, mut location: u32, mut sequence: u32) -> bool {
        let p = playerid as usize;
        let flag = self.player[p].disabled_location | self.player[p].used_location;
        if location == LOCATION_EMZONE {
            sequence += 5;
            location = LOCATION_MZONE;
        }
        if location == LOCATION_MMZONE {
            location = LOCATION_MZONE;
            sequence += self.is_flag(DUEL_3_COLUMNS_FIELD) as u32;
        }
        if location == LOCATION_STZONE {
            location = LOCATION_SZONE;
            sequence += self.is_flag(DUEL_3_COLUMNS_FIELD) as u32;
        }
        if location == LOCATION_MZONE {
            if flag & (1u32.wrapping_shl(sequence)) != 0 {
                return false;
            }
            if sequence >= 5 {
                let oppo = self.player[1 - p].disabled_location | self.player[1 - p].used_location;
                if oppo & (1u32.wrapping_shl(11u32.wrapping_sub(sequence))) != 0 {
                    return false;
                }
            }
        } else if location == LOCATION_SZONE {
            if flag & (0x100u32.wrapping_shl(sequence)) != 0 {
                return false;
            }
        } else if location == LOCATION_FZONE {
            if flag & (0x100u32.wrapping_shl(5 + sequence)) != 0 {
                return false;
            }
        } else if location == LOCATION_PZONE {
            if !self.is_flag(DUEL_PZONE) {
                return false;
            }
            if flag & (0x100u32 << self.get_pzone_index(sequence as u8, playerid as u8)) != 0 {
                return false;
            }
        }
        true
    }
    /// usable count of LOCATION_MZONE or real LOCATION_SZONE of playerid requested by uplayer (may be negative)
    #[allow(clippy::too_many_arguments)]
    pub fn get_useable_count(
        &mut self,
        pcard: Option<CardId>,
        playerid: u8,
        location: u8,
        uplayer: u8,
        reason: LocationReason,
        zone: u32,
        list: Option<&mut u32>,
    ) -> i32 {
        if location as u32 == LOCATION_MZONE {
            if let Some(c) = pcard {
                if self.card(c).current.location as u32 == LOCATION_EXTRA {
                    return self.get_useable_count_fromex(pcard, playerid, uplayer, zone, list);
                }
            }
        }
        self.get_useable_count_other(pcard, playerid, location, uplayer, reason, zone, list)
    }
    pub fn get_useable_count_fromex(&mut self, pcard: Option<CardId>, playerid: u8, uplayer: u8, zone: u32, list: Option<&mut u32>) -> i32 {
        let (pc, use_temp) = match pcard {
            Some(c) => (c, false),
            None => {
                let t = self.temp_card;
                self.card_mut(t).current.location = LOCATION_EXTRA as u8;
                (t, true)
            }
        };
        let count = if self.is_flag(DUEL_EMZONE) {
            self.get_useable_count_fromex_rule4(pc, playerid, uplayer, zone, list)
        } else {
            self.get_useable_count_other(Some(pc), playerid, LOCATION_MZONE as u8, uplayer, LocationReason::ToField, zone, list)
        };
        if use_temp {
            self.card_mut(pc).current.location = 0;
        }
        count
    }
    pub fn get_spsummonable_count(&mut self, pcard: CardId, playerid: u8, zone: u32, list: Option<&mut u32>) -> i32 {
        if self.card(pcard).current.location as u32 == LOCATION_EXTRA {
            self.get_spsummonable_count_fromex(Some(pcard), playerid, playerid, zone, list)
        } else {
            self.get_tofield_count(Some(pcard), playerid, LOCATION_MZONE as u8, playerid as u32, LocationReason::ToField, zone, list)
        }
    }
    pub fn get_spsummonable_count_fromex(&mut self, pcard: Option<CardId>, playerid: u8, uplayer: u8, zone: u32, list: Option<&mut u32>) -> i32 {
        let (pc, use_temp) = match pcard {
            Some(c) => (c, false),
            None => {
                let t = self.temp_card;
                self.card_mut(t).current.location = LOCATION_EXTRA as u8;
                (t, true)
            }
        };
        let count = if self.is_flag(DUEL_EMZONE) {
            self.get_spsummonable_count_fromex_rule4(pc, playerid, uplayer, zone, list)
        } else {
            self.get_tofield_count(Some(pc), playerid, LOCATION_MZONE as u8, uplayer as u32, LocationReason::ToField, zone, list)
        };
        if use_temp {
            self.card_mut(pc).current.location = 0;
        }
        count
    }
    #[allow(clippy::too_many_arguments)]
    pub fn get_useable_count_other(
        &mut self,
        pcard: Option<CardId>,
        playerid: u8,
        location: u8,
        uplayer: u8,
        reason: LocationReason,
        zone: u32,
        list: Option<&mut u32>,
    ) -> i32 {
        let mut count = self.get_tofield_count(pcard, playerid, location, uplayer as u32, reason, zone, list);
        let limit = if location as u32 == LOCATION_MZONE {
            self.get_mzone_limit(playerid, uplayer, reason)
        } else {
            self.get_szone_limit(playerid, uplayer, reason)
        };
        if count > limit {
            count = limit;
        }
        count
    }
    #[allow(clippy::too_many_arguments)]
    pub fn get_tofield_count(
        &mut self,
        pcard: Option<CardId>,
        playerid: u8,
        location: u8,
        uplayer: u32,
        reason: LocationReason,
        zone: u32,
        list: Option<&mut u32>,
    ) -> i32 {
        if location as u32 != LOCATION_MZONE && location as u32 != LOCATION_SZONE {
            return 0;
        }
        let p = playerid as usize;
        let mut flag = self.player[p].disabled_location | self.player[p].used_location;
        if location as u32 == LOCATION_MZONE {
            flag |= !(self.get_forced_zones(pcard, playerid, location, uplayer, reason) as u32);
            flag = (flag | !zone) & 0x1f;
        } else {
            flag = ((flag >> 8) | !zone) & 0x1f;
        }
        let count = 5 - field_used_count(flag);
        if location as u32 == LOCATION_MZONE {
            flag |= (1u32 << 5) | (1u32 << 6);
        }
        if let Some(l) = list {
            *l = flag;
        }
        count
    }
    pub fn get_useable_count_fromex_rule4(&mut self, pcard: CardId, playerid: u8, uplayer: u8, zone: u32, list: Option<&mut u32>) -> i32 {
        let mut count = self.get_spsummonable_count_fromex_rule4(pcard, playerid, uplayer, zone, list);
        let limit = self.get_mzone_limit(playerid, uplayer, LocationReason::ToField);
        if count > limit {
            count = limit;
        }
        count
    }
    pub fn get_spsummonable_count_fromex_rule4(&mut self, pcard: CardId, playerid: u8, uplayer: u8, zone: u32, list: Option<&mut u32>) -> i32 {
        let p = playerid as usize;
        let mut flag = self.player[p].disabled_location | self.player[p].used_location;
        flag |= !(self.get_forced_zones(Some(pcard), playerid, LOCATION_MZONE as u8, uplayer as u32, LocationReason::ToField) as u32);
        if self.player[p].list_mzone[5].is_some() && self.is_location_useable(playerid as u32, LOCATION_MZONE, 6) && self.check_extra_link(playerid as i32, pcard, 6) {
            flag |= 1u32 << 5;
        } else if self.player[p].list_mzone[6].is_some()
            && self.is_location_useable(playerid as u32, LOCATION_MZONE, 5)
            && self.check_extra_link(playerid as i32, pcard, 5)
        {
            flag |= 1u32 << 6;
        } else if self.player[p].list_mzone[5].is_some() || self.player[p].list_mzone[6].is_some() {
            flag |= (1u32 << 5) | (1u32 << 6);
        } else {
            if !self.is_location_useable(playerid as u32, LOCATION_MZONE, 5) {
                flag |= 1u32 << 5;
            }
            if !self.is_location_useable(playerid as u32, LOCATION_MZONE, 6) {
                flag |= 1u32 << 6;
            }
        }
        let rule_zone = self.get_rule_zone_fromex(playerid as i32, Some(pcard));
        flag = flag | !zone | !rule_zone;
        if let Some(l) = list {
            *l = flag & 0x7f;
        }
        let mut count = 5 - field_used_count(flag & 0x1f);
        if !flag & ((1u32 << 5) | (1u32 << 6)) != 0 {
            count += 1;
        }
        count
    }
    pub fn get_mzone_limit(&mut self, playerid: u8, uplayer: u8, reason: LocationReason) -> i32 {
        let p = playerid as usize;
        let used_flag = self.player[p].used_location & 0x1f;
        let mut max = 5;
        let mut used_count = field_used_count(used_flag);
        if self.is_flag(DUEL_EMZONE) {
            max = 7;
            if self.player[p].list_mzone[5].is_some() {
                used_count += 1;
            }
            if self.player[p].list_mzone[6].is_some() {
                used_count += 1;
            }
        }
        let mut eset = Vec::new();
        if uplayer < 2 {
            self.filter_player_effect(playerid, EFFECT_MAX_MZONE, &mut eset, true);
        }
        for peff in eset {
            self.add_param(Value::Int(playerid as i64));
            self.add_param(Value::Int(uplayer as i64));
            self.add_param(Value::Int(reason as u32 as i64));
            let v = peff.get_value(self, 3) as i32;
            if max > v {
                max = v;
            }
        }
        max - used_count
    }
    pub fn get_szone_limit(&mut self, playerid: u8, uplayer: u8, reason: LocationReason) -> i32 {
        let p = playerid as usize;
        let used_flag = (self.player[p].used_location >> 8) & 0x1f;
        let mut eset = Vec::new();
        if uplayer < 2 {
            self.filter_player_effect(playerid, EFFECT_MAX_SZONE, &mut eset, true);
        }
        let mut max = 5;
        for peff in eset {
            self.add_param(Value::Int(playerid as i64));
            self.add_param(Value::Int(uplayer as i64));
            self.add_param(Value::Int(reason as u32 as i64));
            let v = peff.get_value(self, 3) as i32;
            if max > v {
                max = v;
            }
        }
        max - field_used_count(used_flag)
    }
    pub fn get_forced_zones(&mut self, pcard: Option<CardId>, playerid: u8, location: u8, uplayer: u32, reason: LocationReason) -> i32 {
        if location as u32 != LOCATION_MZONE {
            return 0xff;
        }
        let mut eset = Vec::new();
        let mut res: u32 = 0xff7fff7f;
        if uplayer < 2 {
            self.filter_player_effect(uplayer as u8, EFFECT_MUST_USE_MZONE, &mut eset, true);
        }
        if let Some(c) = pcard {
            c.filter_effect(self, EFFECT_MUST_USE_MZONE, &mut eset, true);
        }
        for peff in eset {
            if self.effect(peff).is_flag(EFFECT_FLAG_COUNT_LIMIT) && self.effect(peff).count_limit == 0 {
                continue;
            }
            if let Some(op) = self.effect(peff).operation.clone() {
                self.add_param_front(Value::Effect(peff));
                self.add_param(Value::Int(playerid as i64));
                self.add_param(Value::Int(uplayer as i64));
                self.add_param(Value::Int(reason as u32 as i64));
                if !self.check_condition(Some(&op), 4) {
                    continue;
                }
            }
            self.add_param(Value::Int(playerid as i64));
            self.add_param(Value::Int(uplayer as i64));
            self.add_param(Value::Int(reason as u32 as i64));
            if self.effect(peff).is_flag(EFFECT_FLAG_PLAYER_TARGET) {
                res &= peff.get_value(self, 3) as u32;
            } else {
                res &= peff.get_value_card(self, pcard, 3) as u32;
            }
        }
        if uplayer == playerid as u32 || uplayer > 1 {
            res &= 0xff;
        } else {
            res >>= 16;
            res &= 0xff;
        }
        res as i32
    }
    pub fn get_rule_zone_fromex(&mut self, playerid: i32, pcard: Option<CardId>) -> u32 {
        if self.is_flag(DUEL_EMZONE) {
            if self.is_flag(DUEL_FSX_MMZONE) {
                if let Some(c) = pcard {
                    if self.card(c).is_position(POS_FACEDOWN) != 0 && self.card(c).data.type_ & (self.get_extra_deck_types() & !TYPE_LINK) != 0 {
                        return 0x7f;
                    }
                }
            }
            self.get_linked_zone(playerid, false, false) | (1u32 << 5) | (1u32 << 6)
        } else {
            0x1f
        }
    }
}
