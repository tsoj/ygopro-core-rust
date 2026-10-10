//! The field (`field.cpp` part 2: piles, shuffles, the effect registry, effect filters).

use crate::card::public_order;
use crate::constants::*;
use crate::duel::{Duel, Message, ProgressiveBuffer};
use crate::effect::sort_effects_by_id;
use crate::types::*;
use crate::units;

/// A card filter of a card script (a Lua function and its extra arguments).
pub type CardFilter<'a> = Option<&'a dyn Fn(&mut Duel, CardId) -> bool>;

impl Duel {
    pub fn get_linked_zone(&mut self, playerid: i32, free: bool, actually_linked: bool) -> u32 {
        let p = playerid as usize;
        let mut zones = 0u32;
        let own: Vec<CardId> = self.player[p].list_mzone.iter().chain(self.player[p].list_szone.iter()).flatten().copied().collect();
        for c in own {
            zones |= c.get_linked_zone(self, free);
        }
        let mut oppzones = 0u32;
        let opp: Vec<CardId> = self.player[1 - p].list_mzone.iter().chain(self.player[1 - p].list_szone.iter()).flatten().copied().collect();
        for c in opp {
            oppzones |= c.get_linked_zone(self, free);
        }
        zones |= ((oppzones & 0xffff) << 16) | (oppzones >> 16);
        if !actually_linked {
            let mut eset = Vec::new();
            self.filter_field_effect(EFFECT_BECOME_LINKED_ZONE, &mut eset, true);
            for peff in eset {
                let value = peff.get_value(self, 0) as u32;
                if value != 0 {
                    zones |= (value >> (16 * playerid as u32)) & 0x7f;
                }
            }
        }
        zones
    }
    pub fn get_linked_cards_field(&mut self, self_: u8, location1: u8, location2: u8, cset: &mut CardSet) {
        let z = self.get_linked_zone(self_ as i32, false, false);
        self.get_cards_in_zone(cset, z, self_ as i32, location1 as i32);
        let z = self.get_linked_zone(1 - self_ as i32, false, false);
        self.get_cards_in_zone(cset, z, 1 - self_ as i32, location2 as i32);
    }
    pub fn check_extra_link(&mut self, playerid: i32, pcard: CardId, sequence: i32) -> bool {
        let p = playerid as usize;
        if self.player[p].list_mzone[sequence as usize].is_some() {
            return false;
        }
        let saved = self.card(pcard).current;
        self.player[p].list_mzone[sequence as usize] = Some(pcard);
        {
            let c = self.card_mut(pcard);
            c.current.controler = playerid as u8;
            c.current.location = LOCATION_MZONE as u8;
            c.current.sequence = sequence as u32;
            c.current.position = POS_FACEUP_ATTACK;
        }
        let ret = pcard.is_extra_link_state(self);
        self.player[p].list_mzone[sequence as usize] = None;
        {
            let c = self.card_mut(pcard);
            c.current.controler = saved.controler;
            c.current.location = saved.location;
            c.current.sequence = saved.sequence as u8 as u32;
            c.current.position = saved.position as u8 as u32;
        }
        ret
    }
    pub fn get_cards_in_zone(&self, cset: &mut CardSet, zone: u32, playerid: i32, location: i32) {
        if location as u32 & LOCATION_ONFIELD == 0 {
            return;
        }
        let p = playerid as usize;
        if location as u32 & LOCATION_MZONE != 0 {
            let mut icheck = 1u32;
            for c in &self.player[p].list_mzone {
                if zone & icheck != 0 {
                    if let Some(c) = c {
                        cset.insert(*c);
                    }
                }
                icheck <<= 1;
            }
        }
        if location as u32 & LOCATION_SZONE != 0 {
            let mut icheck = 1u32 << 8;
            for c in &self.player[p].list_szone {
                if zone & icheck != 0 {
                    if let Some(c) = c {
                        cset.insert(*c);
                    }
                }
                icheck <<= 1;
            }
        }
    }
    fn pile_mut(&mut self, playerid: u8, location: u8) -> &mut Vec<CardId> {
        let p = playerid as usize;
        match location as u32 {
            LOCATION_HAND => &mut self.player[p].list_hand,
            LOCATION_DECK => &mut self.player[p].list_main,
            LOCATION_GRAVE => &mut self.player[p].list_grave,
            LOCATION_REMOVED => &mut self.player[p].list_remove,
            _ => &mut self.player[p].list_extra,
        }
    }
    pub fn shuffle(&mut self, playerid: u8, location: u8) {
        if location as u32 & (LOCATION_HAND | LOCATION_DECK | LOCATION_EXTRA) == 0 {
            return;
        }
        let p = playerid as usize;
        if self.pile_mut(playerid, location).is_empty() {
            return;
        }
        if location as u32 == LOCATION_HAND {
            let shuffle = self.player[p].list_hand.iter().any(|&c| self.card(c).is_position(POS_FACEUP) == 0);
            if !shuffle {
                self.core.shuffle_hand_check[p] = false;
                return;
            }
        }
        let before = self.pile_mut(playerid, location).clone();
        if location as u32 == LOCATION_HAND || !self.is_flag(DUEL_PSEUDO_SHUFFLE) {
            let mut upper_bound = before.len() as i32;
            if location as u32 == LOCATION_EXTRA {
                upper_bound -= self.player[p].extra_p_count as i32;
            }
            if upper_bound > 1 {
                for i in 0..upper_bound - 1 {
                    let r = self.get_next_integer(i, upper_bound - 1);
                    self.pile_mut(playerid, location).swap(i as usize, r as usize);
                }
                self.reset_sequence(playerid, location);
            }
        }
        let to_shuffle = self.pile_mut(playerid, location).clone();
        if location as u32 == LOCATION_HAND || location as u32 == LOCATION_EXTRA {
            let mut tmp = Message::default();
            tmp.u8(playerid).u32(to_shuffle.len());
            for &c in &to_shuffle {
                let card = self.card(c);
                tmp.u32(card.data.code | if card.is_position(POS_FACEUP) != 0 { 0x80000000 } else { 0 });
            }
            if location as u32 == LOCATION_HAND {
                for &c in &to_shuffle {
                    let idx = before.iter().position(|&x| x == c).unwrap_or(before.len());
                    tmp.u8(idx as u8);
                }
            }
            let m = self.new_message(if location as u32 == LOCATION_HAND { MSG_SHUFFLE_HAND } else { MSG_SHUFFLE_EXTRA });
            self.msg(m).bytes(&tmp.data[..]);
            if location as u32 == LOCATION_HAND {
                self.core.shuffle_hand_check[p] = false;
                for &c in &to_shuffle {
                    let mut own = Vec::new();
                    c.get_own_effects(self, &mut own);
                    for pe in own {
                        if self.effect(pe).is_flag(EFFECT_FLAG_CLIENT_HINT) && !self.effect(pe).is_flag(EFFECT_FLAG_PLAYER_TARGET) {
                            let loc = c.get_info_location(self);
                            let desc = self.effect(pe).description;
                            let mm = self.new_message(MSG_CARD_HINT);
                            self.msg(mm).loc(loc).u8(CHINT_DESC_ADD).u64(desc);
                        }
                    }
                }
            }
        } else {
            let m = self.new_message(MSG_SHUFFLE_DECK);
            self.msg(m).u8(playerid);
            self.core.shuffle_deck_check[p] = false;
            if self.core.global_flag & GLOBALFLAG_DECK_REVERSE_CHECK != 0 {
                let ptop = *to_shuffle.last().unwrap();
                if self.core.deck_reversed || self.card(ptop).current.position == POS_FACEUP_DEFENSE {
                    let (code, pos) = (self.card(ptop).data.code, self.card(ptop).current.position);
                    let m = self.new_message(MSG_DECK_TOP);
                    self.msg(m).u8(playerid).u32(0u32).u32(code).u32(pos);
                }
            }
        }
    }
    pub fn reset_sequence(&mut self, playerid: u8, location: u8) {
        if location as u32 & LOCATION_ONFIELD != 0 {
            return;
        }
        let list = match location as u32 {
            LOCATION_DECK | LOCATION_HAND | LOCATION_EXTRA | LOCATION_GRAVE | LOCATION_REMOVED => self.pile_mut(playerid, location).clone(),
            _ => return,
        };
        for (i, c) in list.into_iter().enumerate() {
            self.card_mut(c).current.sequence = i as u32;
        }
    }
    pub fn swap_deck_and_grave(&mut self, playerid: u8) {
        let p = playerid as usize;
        for c in self.player[p].list_grave.clone() {
            if !self.core.current_chain.is_empty() {
                self.core.just_sent_cards.insert(c);
            }
            let s = self.card(c).current.sequence;
            self.card_mut(c).previous.location = LOCATION_GRAVE as u8;
            self.card_mut(c).previous.sequence = s;
            c.enable_field_effect(self, false);
            c.cancel_field_effect(self);
        }
        for c in self.player[p].list_main.clone() {
            if !self.core.current_chain.is_empty() {
                self.core.just_sent_cards.insert(c);
            }
            let s = self.card(c).current.sequence;
            self.card_mut(c).previous.location = LOCATION_DECK as u8;
            self.card_mut(c).previous.sequence = s;
            c.enable_field_effect(self, false);
            c.cancel_field_effect(self);
        }
        let pl = &mut self.player[p];
        std::mem::swap(&mut pl.list_grave, &mut pl.list_main);
        let m = self.new_message(MSG_SWAP_GRAVE_DECK);
        self.msg(m).u8(playerid);
        let mut ex = Vec::new();
        let mut buff = ProgressiveBuffer::default();
        let mut i = 0usize;
        let main = std::mem::take(&mut self.player[p].list_main);
        let mut kept = Vec::new();
        for c in main {
            if c.is_extra_deck_monster(self) {
                buff.bit_toggle(i, true);
                ex.push(c);
            } else {
                buff.bit_toggle(i, false);
                kept.push(c);
            }
            i += 1;
        }
        self.player[p].list_main = kept;
        let (re, rp) = (self.core.reason_effect, self.core.reason_player);
        for c in self.player[p].list_grave.clone() {
            {
                let cc = self.card_mut(c);
                cc.current.position = POS_FACEUP;
                cc.current.location = LOCATION_GRAVE as u8;
                cc.current.reason = REASON_EFFECT;
                cc.current.reason_effect = re;
                cc.current.reason_player = rp;
            }
            c.apply_field_effect(self);
            c.enable_field_effect(self, true);
            c.reset(self, RESET_TOGRAVE, RESET_EVENT);
        }
        let reset_to_hidden = |d: &mut Duel, c: CardId, loc: u32| {
            {
                let cc = d.card_mut(c);
                cc.current.position = POS_FACEDOWN_DEFENSE;
                cc.current.location = loc as u8;
                cc.set_status(STATUS_JUST_POS | STATUS_CONTINUOUS_POS | STATUS_ATTACK_CANCELED | STATUS_ACT_FROM_HAND, false);
                cc.current.reason = REASON_EFFECT;
                cc.current.reason_effect = re;
                cc.current.reason_player = rp;
            }
            c.apply_field_effect(d);
            c.enable_field_effect(d, true);
            c.reset(d, RESET_TODECK, RESET_EVENT);
            d.raise_single_event(c, None, EVENT_LEAVE_GRAVE, re, 0, rp, 0, 0);
        };
        for c in self.player[p].list_main.clone() {
            reset_to_hidden(self, c, LOCATION_DECK);
        }
        for &c in &ex {
            reset_to_hidden(self, c, LOCATION_EXTRA);
        }
        let n = self.player[p].list_extra.len() as u32 - self.player[p].extra_p_count;
        self.msg(m).u32(n);
        let at = self.player[p].list_extra.len() - self.player[p].extra_p_count as usize;
        for (k, &c) in ex.iter().enumerate() {
            self.player[p].list_extra.insert(at + k, c);
        }
        self.reset_sequence(playerid, LOCATION_GRAVE as u8);
        self.reset_sequence(playerid, LOCATION_EXTRA as u8);
        let bd = buff.data.clone();
        self.msg(m).u32(bd.len()).bytes(&bd);
        self.shuffle(playerid, LOCATION_DECK as u8);
        if !self.player[p].list_main.is_empty() || !ex.is_empty() {
            let mut cset = CardSet::from_iter(ex.iter().copied());
            cset.extend(self.player[p].list_main.iter().copied());
            self.raise_event(cset, EVENT_LEAVE_GRAVE, re, 0, rp, 0, 0);
            self.process_single_event();
            self.process_instant_event();
        }
    }
    pub fn reverse_deck(&mut self, playerid: u8) {
        let p = playerid as usize;
        let count = self.player[p].list_main.len();
        if count == 0 {
            return;
        }
        for i in 0..count / 2 {
            let a = self.player[p].list_main[i];
            let b = self.player[p].list_main[count - 1 - i];
            self.card_mut(a).current.sequence = (count - 1 - i) as u32;
            self.card_mut(b).current.sequence = i as u32;
            self.player[p].list_main.swap(i, count - 1 - i);
        }
    }
    pub fn get_player_count(&self, playerid: u8) -> i32 {
        self.player[playerid as usize].extra_lists_main.len() as i32 + 1
    }
    fn swap_piles(&mut self, playerid: u8, index: usize) {
        let p = playerid as usize;
        for c in self.player[p].list_main.clone() {
            c.enable_field_effect(self, false);
            c.cancel_field_effect(self);
        }
        let pl = &mut self.player[p];
        std::mem::swap(&mut pl.list_main, &mut pl.extra_lists_main[index]);
        for c in self.player[p].list_main.clone() {
            c.apply_field_effect(self);
            c.enable_field_effect(self, true);
        }
        for c in self.player[p].list_hand.clone() {
            c.enable_field_effect(self, false);
            c.cancel_field_effect(self);
        }
        let pl = &mut self.player[p];
        std::mem::swap(&mut pl.list_hand, &mut pl.extra_lists_hand[index]);
        for c in self.player[p].list_hand.clone() {
            c.apply_field_effect(self);
            c.enable_field_effect(self, true);
        }
        for c in self.player[p].list_extra.clone() {
            c.enable_field_effect(self, false);
            c.cancel_field_effect(self);
        }
        let pl = &mut self.player[p];
        std::mem::swap(&mut pl.list_extra, &mut pl.extra_lists_extra[index]);
        std::mem::swap(&mut pl.extra_p_count, &mut pl.extra_extra_p_count[index]);
        for c in self.player[p].list_extra.clone() {
            c.apply_field_effect(self);
            c.enable_field_effect(self, true);
        }
        let pl = &self.player[p];
        let mut tmp = Message::default();
        tmp.u8(playerid).u32(pl.list_main.len()).u32(pl.list_extra.len()).u32(pl.extra_p_count).u32(pl.list_hand.len());
        if self.core.deck_reversed && !pl.list_main.is_empty() {
            tmp.u32(self.card(*pl.list_main.last().unwrap()).data.code);
        } else {
            tmp.u32(0u32);
        }
        for &c in pl.list_hand.iter().chain(pl.list_extra.iter()) {
            tmp.u32(self.card(c).data.code).u32(self.card(c).current.position);
        }
        let m = self.new_message(MSG_TAG_SWAP);
        self.msg(m).bytes(&tmp.data[1..]);
    }
    pub fn tag_swap(&mut self, playerid: u8) {
        let p = playerid as usize;
        if self.player[p].extra_lists_main.is_empty() {
            return;
        }
        let idx = self.player[p].tag_index as usize;
        self.swap_piles(playerid, idx);
        let n = self.player[p].extra_lists_main.len() as u32;
        self.player[p].tag_index = (self.player[p].tag_index + 1) % n;
    }
    pub fn relay_check(&mut self, playerid: u8) -> bool {
        let p = playerid as usize;
        if self.player[p].exchanges as usize >= self.player[p].extra_lists_main.len() {
            return false;
        }
        self.player[p].exchanges += 1;
        self.core.force_turn_end = true;
        self.player[p].recharge = true;
        true
    }
    pub fn next_player(&mut self, playerid: u8) {
        let p = playerid as usize;
        let idx = self.player[p].exchanges as usize - 1;
        self.swap_piles(playerid, idx);
        self.player[p].lp = self.player[p].start_lp;
        let lp = self.player[p].start_lp;
        let m = self.new_message(MSG_LPUPDATE);
        self.msg(m).u8(playerid).u32(lp);
        self.player[p].recharge = false;
    }
    pub fn has_separate_pzone(&self, _p: u8) -> bool {
        self.is_flag(DUEL_SEPARATE_PZONE)
    }
    pub fn get_pzone_zones_flag(&self) -> u32 {
        let mut flag = 0;
        if !self.has_separate_pzone(0) {
            flag |= 0xC000;
        }
        if !self.has_separate_pzone(1) {
            flag |= 0xC000 << 16;
        }
        flag
    }
    pub fn get_pzone_index(&self, seq: u8, p: u8) -> u8 {
        if seq > 1 {
            return 0;
        }
        if self.has_separate_pzone(p) {
            return seq + 6;
        }
        if self.is_flag(DUEL_3_COLUMNS_FIELD) {
            return seq * 2 + 1;
        }
        seq * 4
    }
    pub fn get_extra_deck_types(&self) -> u32 {
        if self.is_flag(DUEL_EXTRA_DECK_RITUAL) {
            TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ | TYPE_LINK | TYPE_RITUAL
        } else {
            TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ | TYPE_LINK
        }
    }
    fn field_effect_container(&mut self, e: EffectId) -> Option<&mut crate::duel::EffectContainer> {
        let t = self.effect(e).type_ as u32;
        let fe = &mut self.field_effects;
        if t & EFFECT_TYPE_ACTIONS == 0 {
            return Some(&mut fe.aura_effect);
        }
        if t & EFFECT_TYPE_IGNITION != 0 {
            Some(&mut fe.ignition_effect)
        } else if t & EFFECT_TYPE_TRIGGER_O != 0 && t & EFFECT_TYPE_FIELD != 0 {
            Some(&mut fe.trigger_o_effect)
        } else if t & EFFECT_TYPE_TRIGGER_F != 0 && t & EFFECT_TYPE_FIELD != 0 {
            Some(&mut fe.trigger_f_effect)
        } else if t & EFFECT_TYPE_QUICK_O != 0 {
            Some(&mut fe.quick_o_effect)
        } else if t & EFFECT_TYPE_QUICK_F != 0 {
            Some(&mut fe.quick_f_effect)
        } else if t & EFFECT_TYPE_ACTIVATE != 0 {
            Some(&mut fe.activate_effect)
        } else if t & EFFECT_TYPE_CONTINUOUS != 0 {
            Some(&mut fe.continuous_effect)
        } else {
            None
        }
    }
    /// `field::add_effect`: registers a field effect (`owner_player` for effects without a handler).
    pub fn add_effect(&mut self, peffect: EffectId, owner_player: u8) {
        if self.effect(peffect).handler.is_none() {
            let id = self.infos.field_id;
            self.infos.field_id += 1;
            let e = self.effect_mut(peffect);
            e.flag[0] |= EFFECT_FLAG_FIELD_ONLY;
            e.handler = e.owner;
            e.effect_owner = owner_player;
            e.id = id;
            e.initial_id = id;
        }
        let owner = self.effect(peffect).owner.unwrap();
        let otype = self.card(owner).data.type_;
        self.effect_mut(peffect).card_type = otype;
        let (code, typ) = (self.effect(peffect).code, self.effect(peffect).type_ as u32);
        if let Some(cont) = self.field_effect_container(peffect) {
            if cont.contains(peffect) {
                // (the indexer holds each effect once: emplace into the container happened before
                // in C++ too, but the indexer emplace keeps the first entry)
            }
            cont.insert(code, peffect);
        }
        if typ & EFFECT_TYPE_ACTIONS == 0 {
            if code == EFFECT_SELF_TOGRAVE {
                self.core.global_flag |= GLOBALFLAG_SELF_TOGRAVE;
            } else if code == EFFECT_SPSUMMON_COUNT_LIMIT {
                if !self.field_effects.spsummon_count_eff.contains(&peffect) {
                    self.field_effects.spsummon_count_eff.push(peffect);
                }
            } else if code == EFFECT_REMOVE_BRAINWASHING {
                self.core.global_flag |= GLOBALFLAG_BRAINWASHING_CHECK;
            } else if code == EFFECT_REVERSE_DECK {
                self.core.global_flag |= GLOBALFLAG_DECK_REVERSE_CHECK;
            }
            if typ & EFFECT_TYPE_GRANT != 0 && !self.field_effects.grant_effect.entries.iter().any(|x| x.0 == peffect) {
                let iid = self.effect(peffect).initial_id;
                let pos = self.field_effects.grant_effect.entries.partition_point(|x| self.effect(x.0).initial_id < iid);
                self.field_effects.grant_effect.entries.insert(pos, (peffect, Vec::new()));
            }
        }
        if code == EVENT_DETACH_MATERIAL {
            self.core.global_flag |= GLOBALFLAG_DETACH_EVENT;
        }
        if self.effect(peffect).is_flag(EFFECT_FLAG_FIELD_ONLY) {
            if peffect.is_disable_related(self) {
                self.update_disable_check_list(peffect);
            }
            if self.effect(peffect).is_flag(EFFECT_FLAG_OATH) && !self.field_effects.oath.iter().any(|x| x.0 == peffect) {
                let re = self.core.reason_effect;
                self.field_effects.oath.push((peffect, re));
            }
            if self.effect(peffect).reset_flag & RESET_PHASE != 0 && !self.field_effects.pheff.contains(&peffect) {
                self.field_effects.pheff.push(peffect);
            }
            if self.effect(peffect).reset_flag & RESET_CHAIN != 0 && !self.field_effects.cheff.contains(&peffect) {
                self.field_effects.cheff.push(peffect);
            }
            if self.effect(peffect).is_flag(EFFECT_FLAG_COUNT_LIMIT) && !self.field_effects.rechargeable.contains(&peffect) {
                self.field_effects.rechargeable.push(peffect);
            }
            if self.effect(peffect).is_flag(EFFECT_FLAG_PLAYER_TARGET) && self.effect(peffect).is_flag(EFFECT_FLAG_CLIENT_HINT) {
                let mut target_player = [false, false];
                if self.effect(peffect).s_range != 0 {
                    target_player[owner_player as usize & 1] = true;
                }
                if self.effect(peffect).o_range != 0 {
                    target_player[1 - (owner_player as usize & 1)] = true;
                }
                let desc = self.effect(peffect).description;
                for (i, &t) in target_player.iter().enumerate() {
                    if t {
                        let m = self.new_message(MSG_PLAYER_HINT);
                        self.msg(m).u8(i as u8).u8(PHINT_DESC_ADD).u64(desc);
                    }
                }
            }
        }
    }
    pub fn is_field_effect_registered(&self, peffect: EffectId) -> bool {
        let fe = &self.field_effects;
        [
            &fe.aura_effect,
            &fe.ignition_effect,
            &fe.trigger_o_effect,
            &fe.trigger_f_effect,
            &fe.quick_o_effect,
            &fe.quick_f_effect,
            &fe.activate_effect,
            &fe.continuous_effect,
        ]
        .iter()
        .any(|c| c.contains(peffect))
    }
    /// `field::remove_effect`
    pub fn remove_effect(&mut self, peffect: EffectId) {
        if !self.is_field_effect_registered(peffect) {
            return;
        }
        let (typ, code) = (self.effect(peffect).type_ as u32, self.effect(peffect).code);
        if typ & EFFECT_TYPE_ACTIONS == 0 {
            self.field_effects.aura_effect.remove_any(peffect);
            if code == EFFECT_SPSUMMON_COUNT_LIMIT {
                self.field_effects.spsummon_count_eff.retain(|&x| x != peffect);
            }
            if typ & EFFECT_TYPE_GRANT != 0 {
                self.erase_grant_effect(peffect);
            }
        } else {
            let fe = &mut self.field_effects;
            if typ & EFFECT_TYPE_IGNITION != 0 {
                fe.ignition_effect.remove_any(peffect);
            } else if typ & EFFECT_TYPE_TRIGGER_O != 0 {
                fe.trigger_o_effect.remove_any(peffect);
            } else if typ & EFFECT_TYPE_TRIGGER_F != 0 {
                fe.trigger_f_effect.remove_any(peffect);
            } else if typ & EFFECT_TYPE_QUICK_O != 0 {
                fe.quick_o_effect.remove_any(peffect);
            } else if typ & EFFECT_TYPE_QUICK_F != 0 {
                fe.quick_f_effect.remove_any(peffect);
            } else if typ & EFFECT_TYPE_ACTIVATE != 0 {
                fe.activate_effect.remove_any(peffect);
            } else if typ & EFFECT_TYPE_CONTINUOUS != 0 {
                fe.continuous_effect.remove_any(peffect);
            }
        }
        if self.effect(peffect).is_flag(EFFECT_FLAG_FIELD_ONLY) {
            if peffect.is_disable_related(self) {
                self.update_disable_check_list(peffect);
            }
            if self.effect(peffect).is_flag(EFFECT_FLAG_OATH) {
                self.field_effects.oath.retain(|x| x.0 != peffect);
            }
            if self.effect(peffect).reset_flag & RESET_PHASE != 0 {
                self.field_effects.pheff.retain(|&x| x != peffect);
            }
            if self.effect(peffect).reset_flag & RESET_CHAIN != 0 {
                self.field_effects.cheff.retain(|&x| x != peffect);
            }
            if self.effect(peffect).is_flag(EFFECT_FLAG_COUNT_LIMIT) {
                self.field_effects.rechargeable.retain(|&x| x != peffect);
            }
            if !self.core.reseted_effects.contains(&peffect) {
                self.core.reseted_effects.push(peffect);
            }
            if self.effect(peffect).is_flag(EFFECT_FLAG_PLAYER_TARGET) && self.effect(peffect).is_flag(EFFECT_FLAG_CLIENT_HINT) {
                let owner = self.effect(peffect).effect_owner as usize & 1;
                let mut target_player = [false, false];
                if self.effect(peffect).s_range != 0 {
                    target_player[owner] = true;
                }
                if self.effect(peffect).o_range != 0 {
                    target_player[1 - owner] = true;
                }
                let desc = self.effect(peffect).description;
                for (i, &t) in target_player.iter().enumerate() {
                    if t {
                        let m = self.new_message(MSG_PLAYER_HINT);
                        self.msg(m).u8(i as u8).u8(PHINT_DESC_REMOVE).u64(desc);
                    }
                }
            }
        }
    }
    pub fn remove_oath_effect(&mut self, reason_effect: Option<EffectId>) {
        let mut removed: Vec<EffectId> = self.field_effects.oath.iter().filter(|x| x.1 == reason_effect).map(|x| x.0).collect();
        self.field_effects.oath.retain(|x| x.1 != reason_effect);
        sort_effects_by_id(self, &mut removed);
        for pe in removed {
            if self.effect(pe).is_flag(EFFECT_FLAG_FIELD_ONLY) {
                self.remove_effect(pe);
            } else {
                let h = self.effect(pe).handler.unwrap();
                h.remove_effect(self, pe);
            }
        }
    }
    pub fn release_oath_relation(&mut self, reason_effect: Option<EffectId>) {
        for x in self.field_effects.oath.iter_mut() {
            if x.1 == reason_effect {
                x.1 = None;
            }
        }
    }
    pub fn reset_phase(&mut self, phase: u32) {
        let mut v = self.field_effects.pheff.clone();
        sort_effects_by_id(self, &mut v);
        for pe in v {
            if !self.field_effects.pheff.contains(&pe) {
                continue;
            }
            if pe.reset(self, phase, RESET_PHASE) {
                if self.effect(pe).is_flag(EFFECT_FLAG_FIELD_ONLY) {
                    self.remove_effect(pe);
                } else {
                    let h = self.effect(pe).handler.unwrap();
                    h.remove_effect(self, pe);
                }
            }
        }
    }
    pub fn reset_chain(&mut self) {
        self.core.chain_limit_p.clear();
        self.core.effect_count_code_chain.clear();
        let mut v = self.field_effects.cheff.clone();
        sort_effects_by_id(self, &mut v);
        for pe in v {
            if !self.field_effects.cheff.contains(&pe) {
                continue;
            }
            if self.effect(pe).is_flag(EFFECT_FLAG_FIELD_ONLY) {
                self.remove_effect(pe);
            } else {
                let h = self.effect(pe).handler.unwrap();
                h.remove_effect(self, pe);
            }
        }
    }
    fn get_count_map(&mut self, flag: u8) -> &mut Vec<(u64, u32)> {
        if flag as u32 & EFFECT_COUNT_CODE_DUEL != 0 {
            return &mut self.core.effect_count_code_duel;
        }
        if flag as u32 & EFFECT_COUNT_CODE_CHAIN != 0 {
            return &mut self.core.effect_count_code_chain;
        }
        &mut self.core.effect_count_code
    }
    fn count_map_key(code: u32, flag: u8, hopt_index: u8, playerid: u8) -> u64 {
        // (generate_count_map_key(code, hopt_index, flag, playerid) as called by the C++)
        (code as u64) << 32 | ((flag as u64) << 16 | (hopt_index as u64) << 8 | playerid as u64)
    }
    pub fn add_effect_code(&mut self, code: u32, flag: u8, hopt_index: u8, playerid: u8) {
        let key = Self::count_map_key(code, flag, hopt_index, playerid);
        let m = self.get_count_map(flag);
        match m.iter_mut().find(|x| x.0 == key) {
            Some(x) => x.1 += 1,
            None => m.push((key, 1)),
        }
    }
    pub fn get_effect_code(&mut self, code: u32, flag: u8, hopt_index: u8, playerid: u8) -> u32 {
        let key = Self::count_map_key(code, flag, hopt_index, playerid);
        self.get_count_map(flag).iter().find(|x| x.0 == key).map_or(0, |x| x.1)
    }
    pub fn dec_effect_code(&mut self, code: u32, flag: u8, hopt_index: u8, playerid: u8) {
        let key = Self::count_map_key(code, flag, hopt_index, playerid);
        if let Some(x) = self.get_count_map(flag).iter_mut().find(|x| x.0 == key) {
            if x.1 != 0 {
                x.1 -= 1;
            }
        }
    }
    pub fn filter_field_effect(&mut self, code: u32, eset: &mut Vec<EffectId>, sort: bool) {
        for pe in self.field_effects.aura_effect.range(code) {
            if pe.is_available(self) {
                eset.push(pe);
            }
        }
        if sort {
            sort_effects_by_id(self, eset);
        }
    }
    fn range_lists(&self, peffect: EffectId) -> Option<Vec<Vec<Option<CardId>>>> {
        let mut self_p = peffect.get_handler_player(self);
        if self_p == PLAYER_NONE as u8 {
            return None;
        }
        let mut range = self.effect(peffect).s_range as u32;
        let mut out = Vec::new();
        for _ in 0..2 {
            let pl = &self.player[self_p as usize & 1];
            if range & LOCATION_MZONE != 0 {
                out.push(pl.list_mzone.clone());
            }
            if range & LOCATION_SZONE != 0 {
                out.push(pl.list_szone.clone());
            }
            let wrap = |v: &Vec<CardId>| v.iter().map(|&c| Some(c)).collect::<Vec<_>>();
            if range & LOCATION_GRAVE != 0 {
                out.push(wrap(&pl.list_grave));
            }
            if range & LOCATION_REMOVED != 0 {
                out.push(wrap(&pl.list_remove));
            }
            if range & LOCATION_HAND != 0 {
                out.push(wrap(&pl.list_hand));
            }
            if range & LOCATION_DECK != 0 {
                out.push(wrap(&pl.list_main));
            }
            if range & LOCATION_EXTRA != 0 {
                out.push(wrap(&pl.list_extra));
            }
            range = self.effect(peffect).o_range as u32;
            self_p = 1 - self_p;
        }
        Some(out)
    }
    pub fn filter_affected_cards(&mut self, peffect: EffectId, cset: &mut CardSet) {
        let t = self.effect(peffect).type_ as u32;
        if t & EFFECT_TYPE_ACTIONS != 0 || t & EFFECT_TYPE_FIELD == 0 || self.effect(peffect).is_flag(EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_SPSUM_PARAM) {
            return;
        }
        let Some(lists) = self.range_lists(peffect) else { return };
        for list in lists {
            for c in list.into_iter().flatten() {
                if peffect.is_target(self, c) {
                    cset.insert(c);
                }
            }
        }
    }
    pub fn filter_inrange_cards(&mut self, peffect: EffectId, cset: &mut CardSet) {
        if self.effect(peffect).is_flag(EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_SPSUM_PARAM) {
            return;
        }
        let Some(lists) = self.range_lists(peffect) else { return };
        for list in lists {
            for c in list.into_iter().flatten() {
                if peffect.is_fit_target_function(self, c) {
                    cset.insert(c);
                }
            }
        }
    }
    pub fn filter_player_effect(&mut self, playerid: u8, code: u32, eset: &mut Vec<EffectId>, sort: bool) {
        for pe in self.field_effects.aura_effect.range(code) {
            if pe.is_target_player(self, playerid) && pe.is_available(self) {
                eset.push(pe);
            }
        }
        if sort {
            sort_effects_by_id(self, eset);
        }
    }
    /// `field::filter_matching_card`: counts (or collects, or finds one of) the matching cards.
    #[allow(clippy::too_many_arguments)]
    pub fn filter_matching_card(
        &mut self,
        filter: CardFilter,
        self_p: u8,
        location1: u32,
        location2: u32,
        mut pgroup: Option<&mut CardSet>,
        pexception: Option<CardId>,
        pexgroup: Option<&CardSet>,
        mut pret: Option<&mut Option<CardId>>,
        fcount: i32,
        is_target: bool,
    ) -> bool {
        if self_p != 0 && self_p != 1 {
            return false;
        }
        let mut count = 0i32;
        #[derive(Clone, Copy)]
        enum Extra {
            None,
            MZone,
            SZone,
            PZone,
        }
        let mut checkc = |d: &mut Duel, pcard: Option<CardId>, extra: Extra| -> bool {
            let Some(pc) = pcard else { return false };
            let ok_extra = match extra {
                Extra::None => true,
                Extra::MZone => d.card(pc).get_status(STATUS_SUMMONING | STATUS_SUMMON_DISABLED | STATUS_SPSUMMON_STEP) == 0,
                Extra::SZone => !d.card(pc).is_status(STATUS_ACTIVATE_DISABLED),
                Extra::PZone => d.card(pc).current.pzone && !d.card(pc).is_status(STATUS_ACTIVATE_DISABLED),
            };
            if ok_extra
                && Some(pc) != pexception
                && !pexgroup.is_some_and(|g| g.contains(pc))
                && filter.is_none_or(|f| f(d, pc))
                && (!is_target || {
                    let (re, rp) = (d.core.reason_effect, d.core.reason_player);
                    re.is_some_and(|re| pc.is_capable_be_effect_target(d, re, rp))
                })
            {
                if let Some(r) = pret.as_mut() {
                    **r = Some(pc);
                    return true;
                }
                count += 1;
                if fcount != 0 && count >= fcount {
                    return true;
                }
                if let Some(g) = pgroup.as_mut() {
                    g.insert(pc);
                }
            }
            false
        };
        let mut s = self_p as usize;
        let mut location = location1;
        for _ in 0..2 {
            if location & LOCATION_MZONE != 0 {
                for c in self.player[s].list_mzone.clone() {
                    if checkc(self, c, Extra::MZone) {
                        return true;
                    }
                }
            } else {
                if location & LOCATION_MMZONE != 0 {
                    for c in self.player[s].list_mzone[..5].to_vec() {
                        if checkc(self, c, Extra::MZone) {
                            return true;
                        }
                    }
                }
                if location & LOCATION_EMZONE != 0 {
                    let (mut b, mut e) = (5, 7);
                    if self.is_flag(DUEL_3_COLUMNS_FIELD) {
                        b += 1;
                        e -= 1;
                    }
                    for c in self.player[s].list_mzone[b..e].to_vec() {
                        if checkc(self, c, Extra::MZone) {
                            return true;
                        }
                    }
                }
            }
            if location & LOCATION_SZONE != 0 {
                for c in self.player[s].list_szone.clone() {
                    if checkc(self, c, Extra::SZone) {
                        return true;
                    }
                }
            } else {
                if location & LOCATION_STZONE != 0 {
                    let (mut b, mut e) = (0, 5);
                    if self.is_flag(DUEL_3_COLUMNS_FIELD) {
                        b += 1;
                        e -= 1;
                    }
                    for c in self.player[s].list_szone[b..e].to_vec() {
                        if checkc(self, c, Extra::SZone) {
                            return true;
                        }
                    }
                }
                if location & LOCATION_FZONE != 0 {
                    let c = self.player[s].list_szone[5];
                    if checkc(self, c, Extra::SZone) {
                        return true;
                    }
                }
                if location & LOCATION_PZONE != 0 {
                    let i0 = self.get_pzone_index(0, s as u8) as usize;
                    let i1 = self.get_pzone_index(1, s as u8) as usize;
                    let c0 = self.player[s].list_szone[i0];
                    if checkc(self, c0, Extra::PZone) {
                        return true;
                    }
                    let c1 = self.player[s].list_szone[i1];
                    if checkc(self, c1, Extra::PZone) {
                        return true;
                    }
                }
            }
            for (flag, list) in [
                (LOCATION_DECK, self.player[s].list_main.clone()),
                (LOCATION_EXTRA, self.player[s].list_extra.clone()),
                (LOCATION_HAND, self.player[s].list_hand.clone()),
                (LOCATION_GRAVE, self.player[s].list_grave.clone()),
                (LOCATION_REMOVED, self.player[s].list_remove.clone()),
            ] {
                if location & flag != 0 {
                    for c in list {
                        if checkc(self, Some(c), Extra::None) {
                            return true;
                        }
                    }
                }
            }
            location = location2;
            s = 1 - s;
        }
        false
    }
    /// Duel.GetFieldGroup(), Duel.GetFieldGroupCount()
    pub fn filter_field_card(&mut self, self_p: u8, location1: u32, location2: u32, mut pgroup: Option<&mut CardSet>) -> i32 {
        if self_p != 0 && self_p != 1 {
            return 0;
        }
        let mut location = location1;
        let mut count = 0usize;
        let mut s = self_p as usize;
        let add = |g: &mut Option<&mut CardSet>, c: CardId| {
            if let Some(g) = g.as_mut() {
                g.insert(c);
            }
        };
        for _ in 0..2 {
            if location & LOCATION_MZONE != 0 {
                for c in self.player[s].list_mzone.clone().into_iter().flatten() {
                    if self.card(c).get_status(STATUS_SUMMONING | STATUS_SPSUMMON_STEP) == 0 {
                        add(&mut pgroup, c);
                        count += 1;
                    }
                }
            } else {
                if location & LOCATION_MMZONE != 0 {
                    let (mut b, mut e) = (0, 5);
                    if self.is_flag(DUEL_3_COLUMNS_FIELD) {
                        b += 1;
                        e -= 1;
                    }
                    for c in self.player[s].list_mzone[b..e].iter().flatten() {
                        add(&mut pgroup, *c);
                        count += 1;
                    }
                }
                if location & LOCATION_EMZONE != 0 {
                    for c in self.player[s].list_mzone[5..7].iter().flatten() {
                        add(&mut pgroup, *c);
                        count += 1;
                    }
                }
            }
            if location & LOCATION_SZONE != 0 {
                for c in self.player[s].list_szone.iter().flatten() {
                    add(&mut pgroup, *c);
                    count += 1;
                }
            } else {
                if location & LOCATION_STZONE != 0 {
                    let (mut b, mut e) = (0, 5);
                    if self.is_flag(DUEL_3_COLUMNS_FIELD) {
                        b += 1;
                        e -= 1;
                    }
                    for c in self.player[s].list_szone[b..e].iter().flatten() {
                        add(&mut pgroup, *c);
                        count += 1;
                    }
                }
                if location & LOCATION_FZONE != 0 {
                    if let Some(c) = self.player[s].list_szone[5] {
                        add(&mut pgroup, c);
                        count += 1;
                    }
                }
                if location & LOCATION_PZONE != 0 {
                    for i in 0..2 {
                        let idx = self.get_pzone_index(i, s as u8) as usize;
                        if let Some(c) = self.player[s].list_szone[idx] {
                            if self.card(c).current.pzone {
                                add(&mut pgroup, c);
                                count += 1;
                            }
                        }
                    }
                }
            }
            let pl = &self.player[s];
            for (flag, list) in [
                (LOCATION_HAND, &pl.list_hand),
                (LOCATION_DECK, &pl.list_main),
                (LOCATION_EXTRA, &pl.list_extra),
                (LOCATION_GRAVE, &pl.list_grave),
                (LOCATION_REMOVED, &pl.list_remove),
            ] {
                if location & flag != 0 {
                    for &c in list.iter() {
                        add(&mut pgroup, c);
                    }
                    count += list.len();
                }
            }
            location = location2;
            s = 1 - s;
        }
        count as i32
    }
    pub fn is_player_affected_by_effect(&mut self, playerid: u8, code: u32) -> Option<EffectId> {
        for pe in self.field_effects.aura_effect.range(code) {
            if pe.is_target_player(self, playerid) && pe.is_available(self) {
                return Some(pe);
            }
        }
        None
    }
    pub fn get_player_effect(&mut self, playerid: u8, code: u32, eset: &mut Vec<EffectId>) {
        for pe in self.field_effects.aura_effect.all() {
            if (code == 0 || self.effect(pe).code == code) && pe.is_target_player(self, playerid) && pe.is_available(self) {
                eset.push(pe);
            }
        }
    }
    /// put all cards in the target of peffect into effects.disable_check_set
    pub fn update_disable_check_list(&mut self, peffect: EffectId) {
        let mut cset = CardSet::new();
        self.filter_affected_cards(peffect, &mut cset);
        for c in public_order(self, &cset) {
            self.add_to_disable_check_list(c);
        }
    }
    pub fn add_to_disable_check_list(&mut self, pcard: CardId) {
        self.field_effects.disable_check_set.insert(pcard);
    }
    pub fn adjust_disable_check_list(&mut self) {
        let set = self.field_effects.disable_check_set.clone();
        for c in public_order(self, &set) {
            if !self.card(c).is_status(STATUS_TO_ENABLE | STATUS_TO_DISABLE) {
                let pre_disable = self.card(c).get_status(STATUS_DISABLED | STATUS_FORBIDDEN);
                c.refresh_disable_status(self);
                let new_disable = self.card(c).get_status(STATUS_DISABLED | STATUS_FORBIDDEN);
                if pre_disable != new_disable && self.card(c).is_status(STATUS_EFFECT_ENABLED) {
                    c.filter_disable_related_cards(self);
                    if pre_disable != 0 {
                        self.card_mut(c).set_status(STATUS_TO_ENABLE, true);
                    } else {
                        self.card_mut(c).set_status(STATUS_TO_DISABLE, true);
                    }
                }
            }
            let cc = self.card(c);
            if cc.is_status(STATUS_DISABLED) && cc.is_status(STATUS_TO_DISABLE) && !cc.is_status(STATUS_TO_ENABLE) {
                c.reset(self, RESET_DISABLE, RESET_EVENT);
            }
            self.card_mut(c).set_status(STATUS_TO_ENABLE | STATUS_TO_DISABLE, false);
        }
        self.field_effects.disable_check_set.clear();
    }
    /// adjust SetUniqueOnField(), EFFECT_SELF_DESTROY, EFFECT_SELF_TOGRAVE
    pub fn adjust_self_destroy_set(&mut self) {
        if self.core.selfdes_disabled || !self.core.unique_destroy_set.is_empty() || !self.core.self_destroy_set.is_empty() || !self.core.self_tograve_set.is_empty() {
            return;
        }
        let mut p = self.infos.turn_player;
        for _ in 0..2 {
            let mut uniq_set = Vec::new();
            for ucard in self.unique_cards_in_order(p) {
                let uc = self.card(ucard);
                if uc.is_position(POS_FACEUP) != 0 && uc.get_status(STATUS_EFFECT_ENABLED) != 0 && uc.get_status(STATUS_DISABLED | STATUS_FORBIDDEN) == 0 {
                    let mut cset = CardSet::new();
                    ucard.get_unique_target(self, &mut cset, p, None);
                    if cset.is_empty() {
                        self.card_mut(ucard).unique_fieldid = 0;
                    } else if cset.len() == 1 {
                        let f = self.card(cset.first().unwrap()).fieldid;
                        self.card_mut(ucard).unique_fieldid = f;
                    } else {
                        uniq_set.push(ucard);
                    }
                }
            }
            uniq_set.sort_by_key(|&c| self.card(c).fieldid);
            for c in uniq_set {
                self.emplace_process(units::SelfDestroyUnique { step: 0, playerid: p, unique_card: c });
                self.core.unique_destroy_set.insert(c);
            }
            p = 1 - p;
        }
        let mut cset = CardSet::new();
        for p in 0..2 {
            for c in self.player[p].list_mzone.clone().into_iter().flatten() {
                if self.card(c).is_position(POS_FACEUP) != 0 && !self.card(c).is_status(STATUS_BATTLE_DESTROYED) {
                    cset.insert(c);
                }
            }
            for c in self.player[p].list_szone.clone().into_iter().flatten() {
                if self.card(c).is_position(POS_FACEUP) != 0 {
                    cset.insert(c);
                }
            }
        }
        let ordered = public_order(self, &cset);
        for &c in &ordered {
            if c.is_affected_by_effect(self, EFFECT_SELF_DESTROY).is_some() {
                self.core.self_destroy_set.insert(c);
            }
        }
        if self.core.global_flag & GLOBALFLAG_SELF_TOGRAVE != 0 {
            for &c in &ordered {
                if c.is_affected_by_effect(self, EFFECT_SELF_TOGRAVE).is_some() {
                    self.core.self_tograve_set.insert(c);
                }
            }
        }
        if !self.core.self_destroy_set.is_empty() {
            self.emplace_process(units::SelfDestroy { step: 0 });
        }
        if !self.core.self_tograve_set.is_empty() {
            self.emplace_process(units::SelfToGrave { step: 0 });
        }
    }
    pub fn erase_grant_effect(&mut self, peffect: EffectId) {
        let Some(i) = self.field_effects.grant_effect.entries.iter().position(|x| x.0 == peffect) else { return };
        let (_, gains) = self.field_effects.grant_effect.entries.remove(i);
        for (c, e) in gains {
            c.remove_effect(self, e);
        }
    }
    pub fn adjust_grant_effect(&mut self) -> bool {
        let mut adjusted = false;
        let keys: Vec<EffectId> = self.field_effects.grant_effect.entries.iter().map(|x| x.0).collect();
        for peffect in keys {
            let Some(idx) = self.field_effects.grant_effect.entries.iter().position(|x| x.0 == peffect) else { continue };
            let Value::Effect(geffect) = self.effect(peffect).label_object.clone() else { continue };
            let mut cset = CardSet::new();
            if peffect.is_available(self) {
                self.filter_affected_cards(peffect, &mut cset);
            }
            let gains = self.field_effects.grant_effect.entries[idx].1.clone();
            let mut add_set = CardSet::new();
            for c in public_order(self, &cset) {
                if c.is_affect_by_effect(self, Some(peffect)) && !gains.iter().any(|x| x.0 == c) {
                    add_set.insert(c);
                }
            }
            let mut remove_set = CardSet::new();
            for &(c, _) in &gains {
                if !c.is_affect_by_effect(self, Some(peffect)) || !cset.contains(c) {
                    remove_set.insert(c);
                }
            }
            for c in public_order(self, &add_set) {
                let ce = geffect.clone_effect(self, false);
                self.effect_mut(ce).owner = Some(c);
                c.add_effect(self, ce);
                if let Some(i) = self.field_effects.grant_effect.entries.iter().position(|x| x.0 == peffect) {
                    self.field_effects.grant_effect.entries[i].1.push((c, ce));
                }
            }
            for c in public_order(self, &remove_set) {
                let Some(i) = self.field_effects.grant_effect.entries.iter().position(|x| x.0 == peffect) else { continue };
                if let Some(j) = self.field_effects.grant_effect.entries[i].1.iter().position(|x| x.0 == c) {
                    let (_, e) = self.field_effects.grant_effect.entries[i].1.remove(j);
                    c.remove_effect(self, e);
                }
            }
            if !add_set.is_empty() || !remove_set.is_empty() {
                adjusted = true;
            }
        }
        adjusted
    }
    pub fn add_unique_card(&mut self, pcard: CardId) {
        let con = self.card(pcard).current.controler as usize;
        let up = self.card(pcard).unique_pos;
        if up[0] != 0 && !self.core.unique_cards[con].contains(&pcard) {
            self.core.unique_cards[con].push(pcard);
        }
        if up[1] != 0 && !self.core.unique_cards[1 - con].contains(&pcard) {
            self.core.unique_cards[1 - con].push(pcard);
        }
        self.card_mut(pcard).unique_fieldid = 0;
    }
    pub fn remove_unique_card(&mut self, pcard: CardId) {
        let con = self.card(pcard).current.controler;
        if con == PLAYER_NONE as u8 {
            return;
        }
        let con = con as usize;
        let up = self.card(pcard).unique_pos;
        if up[0] != 0 {
            self.core.unique_cards[con].retain(|&c| c != pcard);
        }
        if up[1] != 0 {
            self.core.unique_cards[1 - con].retain(|&c| c != pcard);
        }
    }
    pub fn unique_cards_in_order(&self, playerid: u8) -> Vec<CardId> {
        let mut v = self.core.unique_cards[playerid as usize].clone();
        v.sort();
        v
    }
    /// return: pcard->unique_effect or None
    pub fn check_unique_onfield(&mut self, pcard: CardId, controler: u8, location: u8, icard: Option<CardId>) -> Option<EffectId> {
        if controler > 1 {
            return None;
        }
        for ucard in self.unique_cards_in_order(controler) {
            let uc = self.card(ucard);
            if ucard != pcard
                && Some(ucard) != icard
                && uc.is_position(POS_FACEUP) != 0
                && uc.get_status(STATUS_EFFECT_ENABLED) != 0
                && uc.get_status(STATUS_DISABLED | STATUS_FORBIDDEN) == 0
                && uc.unique_fieldid != 0
                && ucard.check_unique_code(self, pcard)
                && self.card(ucard).unique_location & location as u32 != 0
            {
                return self.card(ucard).unique_effect;
            }
        }
        let pc = self.card(pcard);
        if pc.unique_code == 0 || pc.unique_location & location as u32 == 0 || pc.get_status(STATUS_DISABLED | STATUS_FORBIDDEN) != 0 {
            return None;
        }
        let mut cset = CardSet::new();
        pcard.get_unique_target(self, &mut cset, controler, icard);
        if pcard.check_unique_code(self, pcard) {
            cset.insert(pcard);
        }
        if cset.len() >= 2 {
            return self.card(pcard).unique_effect;
        }
        None
    }
    pub fn check_spsummon_once(&self, pcard: CardId, playerid: u8) -> bool {
        let code = self.card(pcard).spsummon_code;
        if code == 0 {
            return true;
        }
        match self.core.spsummon_once_map[playerid as usize].iter().find(|x| x.0 == code) {
            None => true,
            Some(x) => x.1 == 0,
        }
    }
}
