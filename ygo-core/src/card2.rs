//! Cards (`card.cpp` part 2: zones, equips, materials, effects of a card, resets, counters,
//! relations, effect filters).

use crate::card::*;
use crate::constants::*;
use crate::duel::Duel;
use crate::effect::sort_effects_by_id;
use crate::types::*;

/// Containers of `Card` that hold effects (the target of `indexer`).
pub const CONT_SINGLE: u8 = 1;
pub const CONT_EQUIP: u8 = 2;
pub const CONT_TARGET: u8 = 3;
pub const CONT_XMATERIAL: u8 = 4;
pub const CONT_FIELD: u8 = 5;

impl CardId {
    #[inline]
    fn cc<'a>(&self, d: &'a Duel) -> &'a Card {
        d.card(*self)
    }

    pub fn get_linked_zone(self, d: &mut Duel, free: bool) -> u32 {
        let (location, s, controler, summoning) = {
            let c = self.cc(d);
            (c.current.location as u32, c.current.sequence as i32, c.current.controler, c.get_status(STATUS_SUMMONING | STATUS_SPSUMMON_STEP) != 0)
        };
        if self.get_type(d, None, 0, 2) & TYPE_LINK == 0 || location & LOCATION_ONFIELD == 0 || summoning {
            return 0;
        }
        let mut zones: u32 = 0;
        let marker = self.get_link_marker(d);
        let m = |dir: u32| marker & dir != 0;
        if location == LOCATION_MZONE {
            if s > 0 && s <= 4 && m(LINK_MARKER_LEFT) {
                zones |= 1u32 << (s - 1);
            }
            if s <= 3 && m(LINK_MARKER_RIGHT) {
                zones |= 1u32 << (s + 1);
            }
            if s < 5 {
                if s > 0 && m(LINK_MARKER_BOTTOM_LEFT) {
                    zones |= 1u32 << (s + 7);
                }
                if s < 4 && m(LINK_MARKER_BOTTOM_RIGHT) {
                    zones |= 1u32 << (s + 9);
                }
                if m(LINK_MARKER_BOTTOM) {
                    zones |= 1u32 << (s + 8);
                }
            }
            if d.is_flag(DUEL_EMZONE) {
                if (s == 0 && m(LINK_MARKER_TOP_RIGHT)) || (s == 1 && m(LINK_MARKER_TOP)) || (s == 2 && m(LINK_MARKER_TOP_LEFT)) {
                    zones |= (1u32 << 5) | (1u32 << (16 + 6));
                }
                if (s == 2 && m(LINK_MARKER_TOP_RIGHT)) || (s == 3 && m(LINK_MARKER_TOP)) || (s == 4 && m(LINK_MARKER_TOP_LEFT)) {
                    zones |= (1u32 << 6) | (1u32 << (16 + 5));
                }
                if s == 5 {
                    if m(LINK_MARKER_BOTTOM_LEFT) {
                        zones |= 1u32 << 0;
                    }
                    if m(LINK_MARKER_BOTTOM) {
                        zones |= 1u32 << 1;
                    }
                    if m(LINK_MARKER_BOTTOM_RIGHT) {
                        zones |= 1u32 << 2;
                    }
                    if m(LINK_MARKER_TOP_LEFT) {
                        zones |= 1u32 << (16 + 4);
                    }
                    if m(LINK_MARKER_TOP) {
                        zones |= 1u32 << (16 + 3);
                    }
                    if m(LINK_MARKER_TOP_RIGHT) {
                        zones |= 1u32 << (16 + 2);
                    }
                }
                if s == 6 {
                    if m(LINK_MARKER_BOTTOM_LEFT) {
                        zones |= 1u32 << 2;
                    }
                    if m(LINK_MARKER_BOTTOM) {
                        zones |= 1u32 << 3;
                    }
                    if m(LINK_MARKER_BOTTOM_RIGHT) {
                        zones |= 1u32 << 4;
                    }
                    if m(LINK_MARKER_TOP_LEFT) {
                        zones |= 1u32 << (16 + 2);
                    }
                    if m(LINK_MARKER_TOP) {
                        zones |= 1u32 << (16 + 1);
                    }
                    if m(LINK_MARKER_TOP_RIGHT) {
                        zones |= 1u32 << 16;
                    }
                }
            }
        } else if location == LOCATION_SZONE {
            if s > 4 {
                return 0;
            }
            if s > 0 {
                if m(LINK_MARKER_LEFT) {
                    zones |= 1u32 << (s + 7);
                }
                if m(LINK_MARKER_TOP_LEFT) {
                    zones |= 1u32 << (s - 1);
                }
            }
            if s < 4 {
                if m(LINK_MARKER_RIGHT) {
                    zones |= 1u32 << (s + 9);
                }
                if m(LINK_MARKER_TOP_RIGHT) {
                    zones |= 1u32 << (s + 1);
                }
            }
            if m(LINK_MARKER_TOP) {
                zones |= 1u32 << s;
            }
        }
        if free {
            let p = controler as u32;
            for i in 0..8 {
                if !d.is_location_useable(p, LOCATION_MZONE, i) {
                    zones &= !(1u32 << i);
                }
            }
            for i in 0..8 {
                if !d.is_location_useable(p, LOCATION_SZONE, i) {
                    zones &= !(1u32 << (i + 8));
                }
            }
            for i in 0..8 {
                if !d.is_location_useable(1 - p, LOCATION_MZONE, i) {
                    zones &= !(1u32 << (i + 16));
                }
            }
            for i in 0..8 {
                if !d.is_location_useable(1 - p, LOCATION_SZONE, i) {
                    zones &= !(1u32 << (i + 24));
                }
            }
        }
        zones
    }
    pub fn get_linked_cards(self, d: &mut Duel, cset: &mut CardSet, zones: u32) {
        cset.clear();
        let c = self.cc(d);
        if c.data.type_ & TYPE_LINK == 0 || c.current.location as u32 & LOCATION_ONFIELD == 0 || c.get_status(STATUS_SUMMONING | STATUS_SPSUMMON_STEP) != 0 {
            return;
        }
        let p = c.current.controler as i32;
        let linked_zone = if zones != 0 { zones } else { self.get_linked_zone(d, false) };
        d.get_cards_in_zone(cset, linked_zone, p, LOCATION_ONFIELD as i32);
        d.get_cards_in_zone(cset, linked_zone >> 16, 1 - p, LOCATION_ONFIELD as i32);
        let cards = cset.to_vec();
        for pc in cards {
            if d.card(pc).current.location as u32 == LOCATION_SZONE && pc.get_type(d, None, 0, 2) & TYPE_LINK == 0 {
                cset.remove(pc);
            }
        }
    }
    pub fn get_mutual_linked_zone(self, d: &mut Duel) -> u32 {
        let linked_zone = self.get_linked_zone(d, false);
        if linked_zone == 0 {
            return 0;
        }
        let mut zones: u32 = 0;
        let mut cset = CardSet::new();
        self.get_linked_cards(d, &mut cset, linked_zone);
        for pcard in public_order(d, &cset) {
            let zone = pcard.get_linked_zone(d, false);
            let is_szone = if d.card(pcard).current.location as u32 == LOCATION_SZONE { 8 } else { 0 };
            let is_player = if self.cc(d).current.controler == d.card(pcard).current.controler { 0 } else { 16 };
            if self.is_mutual_linked(d, pcard, linked_zone, zone) {
                zones |= 1u32 << (d.card(pcard).current.sequence + is_szone + is_player);
            }
        }
        zones
    }
    pub fn get_mutual_linked_cards(self, d: &mut Duel, cset: &mut CardSet) {
        cset.clear();
        let c = self.cc(d);
        if c.data.type_ & TYPE_LINK == 0 || c.current.location as u32 & LOCATION_ONFIELD == 0 {
            return;
        }
        let p = c.current.controler as i32;
        let z = self.get_mutual_linked_zone(d);
        d.get_cards_in_zone(cset, z, p, LOCATION_ONFIELD as i32);
        d.get_cards_in_zone(cset, z >> 16, 1 - p, LOCATION_ONFIELD as i32);
    }
    pub fn is_link_state(self, d: &mut Duel) -> bool {
        if self.cc(d).current.location as u32 & LOCATION_ONFIELD == 0 {
            return false;
        }
        let mut cset = CardSet::new();
        self.get_linked_cards(d, &mut cset, 0);
        if !cset.is_empty() {
            return true;
        }
        let c = self.cc(d);
        let p = c.current.controler as i32;
        let is_szone = if c.current.location as u32 == LOCATION_SZONE { 8 } else { 0 };
        let seq = c.current.sequence;
        let linked_zone = d.get_linked_zone(p, false, true);
        (linked_zone >> (seq + is_szone)) & 1 != 0
    }
    pub fn is_mutual_linked(self, d: &mut Duel, pcard: CardId, mut zones1: u32, mut zones2: u32) -> bool {
        if zones1 == 0 {
            zones1 = self.get_linked_zone(d, false);
        }
        let is_szone = if d.card(pcard).current.location as u32 == LOCATION_SZONE { 8 } else { 0 };
        let is_player = if self.cc(d).current.controler == d.card(pcard).current.controler { 0 } else { 16 };
        if zones1 & (1u32 << (d.card(pcard).current.sequence + is_szone + is_player)) != 0 {
            if zones2 == 0 {
                zones2 = pcard.get_linked_zone(d, false);
            }
            let is_szone = if self.cc(d).current.location as u32 == LOCATION_SZONE { 8 } else { 0 };
            return zones2 & (1u32 << (self.cc(d).current.sequence + is_szone + is_player)) != 0;
        }
        false
    }
    pub fn is_extra_link_state(self, d: &mut Duel) -> bool {
        let c = self.cc(d);
        if c.current.location as u32 != LOCATION_MZONE {
            return false;
        }
        let ctrl = c.current.controler as usize;
        let mut checked: u32 = 1u32 << c.current.sequence;
        let mut linked_zone = self.get_mutual_linked_zone(d);
        loop {
            if ((linked_zone >> 5) | (linked_zone >> (16 + 6))) & ((linked_zone >> 6) | (linked_zone >> (16 + 5))) & 1 != 0 {
                return true;
            }
            let checking = (linked_zone & !checked) as i32;
            if checking == 0 {
                return false;
            }
            let mut rightmost = checking & checking.wrapping_neg();
            checked |= rightmost as u32;
            if rightmost < 0x10000 {
                for i in 0..16 {
                    if rightmost & 1 != 0 {
                        let pc = if i < 8 { d.player[ctrl].list_mzone[i] } else { d.player[ctrl].list_szone[i - 8] };
                        linked_zone |= pc.unwrap().get_mutual_linked_zone(d);
                        break;
                    }
                    rightmost >>= 1;
                }
            } else {
                rightmost >>= 16;
                for i in 0..16 {
                    if rightmost & 1 != 0 {
                        let pc = if i < 8 { d.player[1 - ctrl].list_mzone[i] } else { d.player[1 - ctrl].list_szone[i - 8] };
                        let zone = pc.unwrap().get_mutual_linked_zone(d);
                        linked_zone |= ((zone & 0xffff) << 16) | (zone >> 16);
                        break;
                    }
                    rightmost >>= 1;
                }
            }
        }
    }
    pub fn get_column_zone(self, d: &Duel, loc1: i32, left: i32, right: i32) -> u32 {
        let c = self.cc(d);
        let mut zones: u32 = 0;
        let loc2 = c.current.location as i32;
        let mut seq = c.current.sequence as i32;
        if loc1 & LOCATION_ONFIELD as i32 == 0 || loc2 & LOCATION_ONFIELD as i32 == 0 || left < 0 || right < 0 {
            return 0;
        }
        if loc2 & LOCATION_SZONE as i32 != 0 && seq >= 5 {
            return 0;
        }
        if loc2 & LOCATION_MZONE as i32 != 0 && (seq == 5 || seq == 6) {
            seq = if seq == 5 { 1 } else { 3 };
        }
        let seq1 = if seq - left < 0 { 0 } else { seq - left };
        let seq2 = if seq + right > 4 { 4 } else { seq + right };
        let cur_seq = c.current.sequence;
        let mzone = c.current.location as u32 == LOCATION_MZONE;
        let chkextra = |s: u32| !mzone || cur_seq < 5 || cur_seq != s;
        if loc1 & LOCATION_MZONE as i32 != 0 {
            for s in seq1..=seq2 {
                zones |= (1u32 << s) | (1u32 << (16 + (4 - s)));
                if d.is_flag(DUEL_EMZONE) {
                    if s == 1 && chkextra(5) {
                        zones |= (1u32 << 5) | (1u32 << (16 + 6));
                    } else if s == 3 && chkextra(6) {
                        zones |= (1u32 << 6) | (1u32 << (16 + 5));
                    }
                }
            }
        }
        if loc1 & LOCATION_SZONE as i32 != 0 {
            for s in seq1..=seq2 {
                zones |= (1u32 << (8 + s)) | (1u32 << (16 + 8 + (4 - s)));
            }
        }
        zones &= !((1u32 << cur_seq) << if loc2 == LOCATION_SZONE as i32 { 8 } else { 0 });
        zones
    }
    pub fn get_column_cards(self, d: &mut Duel, cset: &mut CardSet, left: i32, right: i32) {
        cset.clear();
        let c = self.cc(d);
        if c.current.location as u32 & LOCATION_ONFIELD == 0 {
            return;
        }
        let p = c.current.controler as i32;
        let column_zone = self.get_column_zone(d, LOCATION_ONFIELD as i32, left, right);
        d.get_cards_in_zone(cset, column_zone, p, LOCATION_ONFIELD as i32);
        d.get_cards_in_zone(cset, column_zone >> 16, 1 - p, LOCATION_ONFIELD as i32);
    }
    pub fn is_all_column(self, d: &mut Duel) -> bool {
        if self.cc(d).current.location as u32 & LOCATION_ONFIELD == 0 {
            return false;
        }
        let mut cset = CardSet::new();
        self.get_column_cards(d, &mut cset, 0, 0);
        let mut full = 3;
        let seq = self.cc(d).current.sequence;
        if d.is_flag(DUEL_EMZONE) && (seq == 1 || seq == 3) {
            full += 1;
        }
        cset.len() == full
    }
    pub fn equip(self, d: &mut Duel, target: CardId, send_msg: bool) {
        if self.cc(d).equiping_target.is_some() {
            return;
        }
        d.card_mut(target).equiping_cards.insert(self);
        d.card_mut(self).equiping_target = Some(target);
        for e in self.cc(d).equip_effect.all() {
            if e.is_disable_related(d) {
                d.add_to_disable_check_list(target);
            }
        }
        if send_msg {
            let a = self.get_info_location(d);
            let b = target.get_info_location(d);
            let m = d.new_message(MSG_EQUIP);
            d.msg(m).loc(a).loc(b);
        }
    }
    pub fn unequip(self, d: &mut Duel) {
        let Some(target) = self.cc(d).equiping_target else { return };
        for e in self.cc(d).equip_effect.all() {
            if e.is_disable_related(d) {
                d.add_to_disable_check_list(target);
            }
        }
        d.card_mut(target).equiping_cards.remove(self);
        let c = d.card_mut(self);
        c.pre_equip_target = Some(target);
        c.equiping_target = None;
    }
    pub fn get_union_count(self, d: &mut Duel) -> i32 {
        let mut count = 0;
        for pcard in public_order(d, &self.cc(d).equiping_cards.clone()) {
            if d.card(pcard).data.type_ & TYPE_UNION != 0 && pcard.is_affected_by_effect(d, EFFECT_UNION_STATUS).is_some() {
                count += 1;
            }
        }
        count
    }
    pub fn get_old_union_count(self, d: &mut Duel) -> i32 {
        let mut count = 0;
        for pcard in public_order(d, &self.cc(d).equiping_cards.clone()) {
            if d.card(pcard).data.type_ & TYPE_UNION != 0 && pcard.is_affected_by_effect(d, EFFECT_OLDUNION_STATUS).is_some() {
                count += 1;
            }
        }
        count
    }
    pub fn xyz_add(self, d: &mut Duel, mat: CardId) {
        if d.card(mat).current.location != 0 {
            return;
        }
        d.card_mut(self).xyz_materials.push(mat);
        let n = d.card(self).xyz_materials.len() as u32;
        {
            let m = d.card_mut(mat);
            m.overlay_target = Some(self);
            m.pre_overlay_target = None;
            m.current.controler = PLAYER_NONE as u8;
            m.current.location = LOCATION_OVERLAY as u8;
            m.current.sequence = n - 1;
        }
        for e in d.card(mat).xmaterial_effect.all() {
            if d.effect(e).type_ as u32 & EFFECT_TYPE_FIELD != 0 {
                d.add_effect(e, 2);
            }
        }
    }
    pub fn xyz_remove(self, d: &mut Duel, mat: CardId) {
        if d.card(mat).overlay_target != Some(self) {
            return;
        }
        let seq = d.card(mat).current.sequence as usize;
        d.card_mut(self).xyz_materials.remove(seq);
        if !d.core.current_chain.is_empty() {
            d.core.just_sent_cards.insert(mat);
        }
        {
            let m = d.card_mut(mat);
            m.previous.controler = m.current.controler;
            m.previous.location = m.current.location;
            m.previous.sequence = m.current.sequence;
            m.previous.pzone = m.current.pzone;
            m.current.controler = PLAYER_NONE as u8;
            m.current.location = 0;
            m.current.sequence = 0;
            m.pre_overlay_target = m.overlay_target.take();
        }
        let mats = d.card(self).xyz_materials.clone();
        for (i, c) in mats.into_iter().enumerate() {
            d.card_mut(c).current.sequence = i as u32;
        }
        for e in d.card(mat).xmaterial_effect.all() {
            if d.effect(e).type_ as u32 & EFFECT_TYPE_FIELD != 0 {
                d.remove_effect(e);
            }
        }
    }
    fn field_effect_applies(self, d: &Duel, e: EffectId) -> bool {
        let fx = d.effect(e);
        e.in_range(d, self)
            || (fx.range as u32 & LOCATION_HAND != 0 && fx.type_ as u32 & EFFECT_TYPE_TRIGGER_O != 0 && fx.code & EVENT_PHASE == 0)
    }
    pub fn apply_field_effect(self, d: &mut Duel) {
        if self.cc(d).current.controler == PLAYER_NONE as u8 {
            return;
        }
        for e in self.cc(d).field_effect.all() {
            if self.field_effect_applies(d, e) {
                d.add_effect(e, 2);
            }
        }
        let c = self.cc(d);
        if c.unique_code != 0 && c.current.location as u32 & c.unique_location != 0 {
            d.add_unique_card(self);
        }
        let c = d.card_mut(self);
        c.spsummon_counter = [0, 0];
        c.spsummon_counter_rst = [0, 0];
    }
    pub fn cancel_field_effect(self, d: &mut Duel) {
        if self.cc(d).current.controler == PLAYER_NONE as u8 {
            return;
        }
        for e in self.cc(d).field_effect.all() {
            if self.field_effect_applies(d, e) {
                d.remove_effect(e);
            }
        }
        let c = self.cc(d);
        if c.unique_code != 0 && c.current.location as u32 & c.unique_location != 0 {
            d.remove_unique_card(self);
        }
    }
    /// STATUS_EFFECT_ENABLED: the card is ready to use (false: before moving, summoning, chaining)
    pub fn enable_field_effect(self, d: &mut Duel, enabled: bool) {
        let c = self.cc(d);
        if c.current.location == 0 {
            return;
        }
        let is_enabled = c.get_status(STATUS_EFFECT_ENABLED) != 0;
        if (enabled && is_enabled) || (!enabled && !is_enabled) {
            return;
        }
        self.refresh_disable_status(d);
        if enabled {
            d.card_mut(self).set_status(STATUS_EFFECT_ENABLED, true);
            for e in self.cc(d).single_effect.all() {
                if d.effect(e).is_flag(EFFECT_FLAG_SINGLE_RANGE) && e.in_range(d, self) {
                    let id = d.infos.field_id;
                    d.infos.field_id += 1;
                    d.effect_mut(e).id = id;
                }
            }
            for e in self.cc(d).field_effect.all() {
                if e.in_range(d, self) {
                    let id = d.infos.field_id;
                    d.infos.field_id += 1;
                    d.effect_mut(e).id = id;
                }
            }
            if self.cc(d).current.location as u32 == LOCATION_SZONE {
                for e in self.cc(d).equip_effect.all() {
                    let id = d.infos.field_id;
                    d.infos.field_id += 1;
                    d.effect_mut(e).id = id;
                }
            }
            for e in self.cc(d).target_effect.all() {
                if e.in_range(d, self) {
                    let id = d.infos.field_id;
                    d.infos.field_id += 1;
                    d.effect_mut(e).id = id;
                }
            }
            if self.cc(d).get_status(STATUS_DISABLED) != 0 {
                self.reset(d, RESET_DISABLE, RESET_EVENT);
            }
        } else {
            d.card_mut(self).set_status(STATUS_EFFECT_ENABLED, false);
        }
        if self.cc(d).get_status(STATUS_DISABLED | STATUS_FORBIDDEN) != 0 {
            return;
        }
        self.filter_disable_related_cards(d);
    }
    fn indexer_find(self, d: &Duel, e: EffectId) -> Option<u8> {
        self.cc(d).indexer.iter().find(|x| x.0 == e).map(|x| x.1)
    }
    /// `card::add_effect`: returns the effect's id (0 if not added).
    pub fn add_effect(self, d: &mut Duel, peffect: EffectId) -> u32 {
        if self.cc(d).get_status(STATUS_COPYING_EFFECT) != 0 && d.effect(peffect).is_flag(EFFECT_FLAG_UNCOPYABLE) {
            d.uncopy.push(peffect);
            return 0;
        }
        if self.indexer_find(d, peffect).is_some() {
            return 0;
        }
        let mut check_target = CardSet::from_card(self);
        let (etype, ecode, single_range) = {
            let e = d.effect(peffect);
            (e.type_ as u32, e.code, e.is_flag(EFFECT_FLAG_SINGLE_RANGE))
        };
        let cont;
        if etype & EFFECT_TYPE_SINGLE != 0 {
            let remove_matching = |d: &mut Duel, codes: &[u32]| {
                for (code, rm) in d.card(self).single_effect.items.clone() {
                    if codes.contains(&code) && !d.effect(rm).is_flag(EFFECT_FLAG_SINGLE_RANGE) {
                        self.remove_effect(d, rm);
                    }
                }
            };
            if (ecode == EFFECT_SET_ATTACK || ecode == EFFECT_SET_BASE_ATTACK) && !single_range {
                remove_matching(d, &[EFFECT_SET_ATTACK, EFFECT_SET_ATTACK_FINAL, EFFECT_SET_BASE_ATTACK]);
            }
            if ecode == EFFECT_SET_ATTACK_FINAL && !single_range {
                remove_matching(d, &[EFFECT_UPDATE_ATTACK, EFFECT_SET_ATTACK, EFFECT_SET_ATTACK_FINAL]);
            }
            if (ecode == EFFECT_SET_DEFENSE || ecode == EFFECT_SET_BASE_DEFENSE) && !single_range {
                remove_matching(d, &[EFFECT_SET_DEFENSE, EFFECT_SET_DEFENSE_FINAL, EFFECT_SET_BASE_DEFENSE]);
            }
            if ecode == EFFECT_SET_DEFENSE_FINAL && !single_range {
                remove_matching(d, &[EFFECT_UPDATE_DEFENSE, EFFECT_SET_DEFENSE, EFFECT_SET_DEFENSE_FINAL]);
            }
            d.card_mut(self).single_effect.insert(ecode, peffect);
            cont = CONT_SINGLE;
        } else if etype & EFFECT_TYPE_EQUIP != 0 {
            d.card_mut(self).equip_effect.insert(ecode, peffect);
            cont = CONT_EQUIP;
            match self.cc(d).equiping_target {
                Some(t) => check_target = CardSet::from_card(t),
                None => check_target.clear(),
            }
        } else if etype & EFFECT_TYPE_TARGET != 0 {
            d.card_mut(self).target_effect.insert(ecode, peffect);
            cont = CONT_TARGET;
            if !self.cc(d).effect_target_cards.is_empty() {
                check_target = self.cc(d).effect_target_cards.clone();
            } else {
                check_target.clear();
            }
        } else if etype & EFFECT_TYPE_XMATERIAL != 0 {
            d.card_mut(self).xmaterial_effect.insert(ecode, peffect);
            cont = CONT_XMATERIAL;
            match self.cc(d).overlay_target {
                Some(t) => check_target = CardSet::from_card(t),
                None => check_target.clear(),
            }
        } else if etype & EFFECT_TYPE_FIELD != 0 {
            d.card_mut(self).field_effect.insert(ecode, peffect);
            cont = CONT_FIELD;
        } else {
            return 0;
        }
        if ecode == EFFECT_SELF_TOGRAVE {
            d.core.global_flag |= GLOBALFLAG_SELF_TOGRAVE;
        } else if ecode == EVENT_DETACH_MATERIAL {
            d.core.global_flag |= GLOBALFLAG_DETACH_EVENT;
        }
        let id = d.infos.field_id;
        d.infos.field_id += 1;
        let data_type = self.cc(d).data.type_;
        let initializing = self.cc(d).get_status(STATUS_INITIALIZING) != 0;
        let copying = self.cc(d).get_status(STATUS_COPYING_EFFECT) != 0;
        let (copy_id, copy_reset, copy_reset_count) = (d.infos.copy_id, d.core.copy_reset, d.core.copy_reset_count);
        {
            let e = d.effect_mut(peffect);
            e.id = id;
            e.initial_id = id;
            e.card_type = data_type;
            if initializing {
                e.flag[0] |= EFFECT_FLAG_INITIAL;
            }
            if copying {
                e.copy_id = copy_id;
                e.reset_flag |= copy_reset;
                e.reset_count = copy_reset_count as u16;
            }
        }
        let reason_effect = d.core.reason_effect;
        if d.effect(peffect).is_flag(EFFECT_FLAG_COPY_INHERIT) {
            if let Some(re) = reason_effect {
                let (rcopy, rflag, rcount) = {
                    let r = d.effect(re);
                    (r.copy_id, r.reset_flag, r.reset_count)
                };
                if rcopy != 0 {
                    let e = d.effect_mut(peffect);
                    e.copy_id = rcopy;
                    e.reset_flag |= rflag;
                    if e.reset_count > rcount {
                        e.reset_count = rcount;
                    }
                }
            }
        }
        d.card_mut(self).indexer.push((peffect, cont));
        d.effect_mut(peffect).handler = Some(self);
        if etype & EFFECT_TYPE_FIELD != 0 {
            let fx = d.effect(peffect);
            if peffect.in_range(d, self)
                || (self.cc(d).current.controler != PLAYER_NONE as u8
                    && fx.range as u32 & LOCATION_HAND != 0
                    && fx.type_ as u32 & EFFECT_TYPE_TRIGGER_O != 0
                    && fx.code & EVENT_PHASE == 0)
            {
                d.add_effect(peffect, 2);
            }
        }
        if self.cc(d).current.controler != PLAYER_NONE as u8 && !check_target.is_empty() && peffect.is_disable_related(d) {
            for t in public_order(d, &check_target) {
                d.add_to_disable_check_list(t);
            }
        }
        if d.effect(peffect).is_flag(EFFECT_FLAG_OATH) {
            d.field_effects.oath.push((peffect, reason_effect));
        }
        if d.effect(peffect).reset_flag & RESET_PHASE != 0 {
            if !d.field_effects.pheff.contains(&peffect) {
                d.field_effects.pheff.push(peffect);
            }
            if d.effect(peffect).reset_count == 0 {
                d.effect_mut(peffect).reset_count += 1;
            }
        }
        if d.effect(peffect).reset_flag & RESET_CHAIN != 0 && !d.field_effects.cheff.contains(&peffect) {
            d.field_effects.cheff.push(peffect);
        }
        if d.effect(peffect).is_flag(EFFECT_FLAG_COUNT_LIMIT) && !d.field_effects.rechargeable.contains(&peffect) {
            d.field_effects.rechargeable.push(peffect);
        }
        if d.effect(peffect).is_flag(EFFECT_FLAG_CLIENT_HINT) {
            let loc = self.get_info_location(d);
            let desc = d.effect(peffect).description;
            let m = d.new_message(MSG_CARD_HINT);
            d.msg(m).loc(loc).u8(CHINT_DESC_ADD).u64(desc);
        }
        if etype & EFFECT_TYPE_SINGLE != 0 && ecode == EFFECT_UPDATE_LEVEL && !single_range {
            let val = peffect.get_value_card(d, Some(self), 0) as i32;
            if val > 0 {
                d.raise_single_event(self, None, EVENT_LEVEL_UP, Some(peffect), 0, 0, 0, val as u32);
                d.process_single_event();
            }
        }
        d.effect(peffect).id
    }
    /// `card::remove_effect`
    pub fn remove_effect(self, d: &mut Duel, peffect: EffectId) {
        let Some(cont) = self.indexer_find(d, peffect) else { return };
        let mut check_target = CardSet::from_card(self);
        let etype = d.effect(peffect).type_ as u32;
        match cont {
            CONT_SINGLE => {
                d.card_mut(self).single_effect.remove_any(peffect);
            }
            CONT_EQUIP => {
                d.card_mut(self).equip_effect.remove_any(peffect);
                match self.cc(d).equiping_target {
                    Some(t) => check_target = CardSet::from_card(t),
                    None => check_target.clear(),
                }
            }
            CONT_TARGET => {
                d.card_mut(self).target_effect.remove_any(peffect);
                if !self.cc(d).effect_target_cards.is_empty() {
                    check_target = self.cc(d).effect_target_cards.clone();
                } else {
                    check_target.clear();
                }
            }
            CONT_XMATERIAL => {
                d.card_mut(self).xmaterial_effect.remove_any(peffect);
                match self.cc(d).overlay_target {
                    Some(t) => check_target = CardSet::from_card(t),
                    None => check_target.clear(),
                }
            }
            _ => {
                check_target.clear();
                if peffect.is_available(d) && peffect.is_disable_related(d) {
                    d.update_disable_check_list(peffect);
                }
                d.card_mut(self).field_effect.remove_any(peffect);
                let fx = d.effect(peffect);
                if peffect.in_range(d, self)
                    || (self.cc(d).current.controler != PLAYER_NONE as u8
                        && fx.range as u32 & LOCATION_HAND != 0
                        && fx.type_ as u32 & EFFECT_TYPE_TRIGGER_O != 0
                        && fx.code & EVENT_PHASE == 0)
                {
                    d.remove_effect(peffect);
                }
            }
        }
        let _ = etype;
        if self.cc(d).current.controler != PLAYER_NONE as u8 && !check_target.is_empty() && peffect.is_disable_related(d) {
            for t in public_order(d, &check_target) {
                d.add_to_disable_check_list(t);
            }
        }
        if d.effect(peffect).is_flag(EFFECT_FLAG_INITIAL) && d.effect(peffect).copy_id != 0 && self.cc(d).is_status(STATUS_EFFECT_REPLACED) {
            d.card_mut(self).set_status(STATUS_EFFECT_REPLACED, false);
            let dt = self.cc(d).data.type_;
            if dt & TYPE_NORMAL == 0 || dt & TYPE_PENDULUM != 0 {
                d.card_mut(self).set_status(STATUS_INITIALIZING, true);
                let code = self.cc(d).data.code;
                d.call_initial_effect(code, self);
                d.card_mut(self).set_status(STATUS_INITIALIZING, false);
            }
        }
        d.card_mut(self).indexer.retain(|x| x.0 != peffect);
        if d.effect(peffect).is_flag(EFFECT_FLAG_OATH) {
            d.field_effects.oath.retain(|x| x.0 != peffect);
        }
        if d.effect(peffect).reset_flag & RESET_PHASE != 0 {
            d.field_effects.pheff.retain(|&x| x != peffect);
        }
        if d.effect(peffect).reset_flag & RESET_CHAIN != 0 {
            d.field_effects.cheff.retain(|&x| x != peffect);
        }
        if d.effect(peffect).is_flag(EFFECT_FLAG_COUNT_LIMIT) {
            d.field_effects.rechargeable.retain(|&x| x != peffect);
        }
        let (ecode, etype) = (d.effect(peffect).code, d.effect(peffect).type_ as u32);
        if (ecode & 0xf0000) == EFFECT_COUNTER_PERMIT && etype & EFFECT_TYPE_SINGLE != 0 {
            let ct = (ecode & 0xffff) as u16;
            if let Some(i) = self.cc(d).counters.iter().position(|x| x.0 == ct) {
                let (t, v) = self.cc(d).counters[i];
                let c = self.cc(d);
                let (ctrl, loc, seq) = (c.current.controler, c.current.location, c.current.sequence);
                let m = d.new_message(MSG_REMOVE_COUNTER);
                d.msg(m).u16(t).u8(ctrl).u8(loc).u8(seq).u16(v[0].wrapping_add(v[1]));
                d.card_mut(self).counters.remove(i);
            }
        }
        if d.effect(peffect).is_flag(EFFECT_FLAG_CLIENT_HINT) {
            let loc = self.get_info_location(d);
            let desc = d.effect(peffect).description;
            let m = d.new_message(MSG_CARD_HINT);
            d.msg(m).loc(loc).u8(CHINT_DESC_REMOVE).u64(desc);
        }
        if ecode == EFFECT_UNIQUE_CHECK {
            d.remove_unique_card(self);
            let c = d.card_mut(self);
            c.unique_pos = [0, 0];
            c.unique_code = 0;
        }
        if !d.core.reseted_effects.contains(&peffect) {
            d.core.reseted_effects.push(peffect);
        }
    }
    fn add_effect_type_effect(self, d: &mut Duel, reset: u32, count: u32) {
        let dt = self.cc(d).data.type_;
        if dt & TYPE_MONSTER != 0 && dt & TYPE_EFFECT == 0 {
            let pe = d.new_effect();
            let owner = match d.core.reason_effect {
                Some(re) => re.get_handler(d),
                None => self,
            };
            let e = d.effect_mut(pe);
            e.owner = Some(owner);
            e.handler = Some(self);
            e.type_ = EFFECT_TYPE_SINGLE as u16;
            e.code = EFFECT_ADD_TYPE;
            e.value = TYPE_EFFECT as i32;
            e.flag[0] = EFFECT_FLAG_CANNOT_DISABLE;
            e.reset_flag = reset;
            e.reset_count = count as u16;
            self.add_effect(d, pe);
        }
    }
    pub fn copy_effect(self, d: &mut Duel, code: u32, reset: u32, count: u32) -> i32 {
        if d.read_card(code).type_ & TYPE_NORMAL != 0 {
            return -1;
        }
        d.card_mut(self).set_status(STATUS_COPYING_EFFECT, true);
        let cr = d.core.copy_reset;
        let crc = d.core.copy_reset_count;
        d.core.copy_reset = reset;
        d.core.copy_reset_count = count as u8;
        d.call_initial_effect(code, self);
        d.infos.copy_id = d.infos.copy_id.wrapping_add(1);
        d.card_mut(self).set_status(STATUS_COPYING_EFFECT, false);
        d.core.copy_reset = cr;
        d.core.copy_reset_count = crc;
        for e in std::mem::take(&mut d.uncopy) {
            d.delete_effect(e);
        }
        self.add_effect_type_effect(d, reset, count);
        d.infos.copy_id as i32 - 1
    }
    pub fn replace_effect(self, d: &mut Duel, code: u32, reset: u32, count: u32, recreating: bool) -> i32 {
        if d.read_card(code).type_ & TYPE_NORMAL != 0 {
            return -1;
        }
        if self.cc(d).is_status(STATUS_EFFECT_REPLACED) {
            d.card_mut(self).set_status(STATUS_EFFECT_REPLACED, false);
        }
        let mut own = Vec::new();
        self.get_own_effects(d, &mut own);
        for pe in own {
            if self.indexer_find(d, pe).is_some() && d.effect(pe).is_flag(EFFECT_FLAG_INITIAL | EFFECT_FLAG_COPY_INHERIT) {
                self.remove_effect(d, pe);
            }
        }
        let cr = d.core.copy_reset;
        let crc = d.core.copy_reset_count;
        d.core.copy_reset = reset;
        d.core.copy_reset_count = count as u8;
        d.card_mut(self).set_status(STATUS_INITIALIZING, true);
        if !recreating {
            d.card_mut(self).set_status(STATUS_COPYING_EFFECT, true);
        }
        d.call_initial_effect(code, self);
        d.card_mut(self).set_status(STATUS_INITIALIZING | STATUS_COPYING_EFFECT, false);
        d.infos.copy_id = d.infos.copy_id.wrapping_add(1);
        d.core.copy_reset = cr;
        d.core.copy_reset_count = crc;
        d.card_mut(self).set_status(STATUS_EFFECT_REPLACED, true);
        for e in std::mem::take(&mut d.uncopy) {
            d.delete_effect(e);
        }
        self.add_effect_type_effect(d, reset, count);
        d.infos.copy_id as i32 - 1
    }
    pub fn reset(self, d: &mut Duel, id: u32, reset_type: u32) {
        if reset_type != RESET_EVENT && reset_type != RESET_PHASE && reset_type != RESET_CODE && reset_type != RESET_COPY && reset_type != RESET_CARD {
            return;
        }
        if reset_type == RESET_EVENT {
            d.card_mut(self).relations.retain(|r| r.1 & 0xffff0000 & id == 0);
            let leave = RESET_TODECK | RESET_TOHAND | RESET_TOGRAVE | RESET_REMOVE | RESET_TEMP_REMOVE | RESET_OVERLAY | RESET_MSCHANGE;
            if id & leave != 0 {
                d.card_mut(self).relate_effect.clear();
            }
            if id & (leave | RESET_LEAVE | RESET_TOFIELD) != 0 {
                let c = d.card_mut(self);
                c.indestructable_effects.clear();
                c.announced_cards.clear();
                c.attacked_cards.clear();
                c.attack_announce_count = 0;
                c.announce_count = 0;
                c.attacked_count = 0;
                c.attack_all_target = 1;
            }
            if id & (leave | RESET_LEAVE | RESET_TOFIELD | RESET_TURN_SET) != 0 {
                d.card_mut(self).battled_cards.clear();
                self.reset_effect_count(d);
                for e in self.cc(d).field_effect.range(EFFECT_DISABLE_FIELD) {
                    d.effect_mut(e).value = 0;
                }
            }
            if id & (leave | RESET_TOFIELD | RESET_TURN_SET) != 0 {
                d.card_mut(self).counters.clear();
            }
            if id & (RESET_TODECK | RESET_TOHAND | RESET_TOGRAVE | RESET_REMOVE | RESET_TEMP_REMOVE | RESET_LEAVE | RESET_TOFIELD | RESET_TURN_SET | RESET_CONTROL)
                != 0
            {
                for e in self.cc(d).field_effect.range(EFFECT_USE_EXTRA_MZONE) {
                    let v = d.effect(e).value;
                    d.effect_mut(e).value = v & 0xffff;
                }
                for e in self.cc(d).field_effect.range(EFFECT_USE_EXTRA_SZONE) {
                    let v = d.effect(e).value;
                    d.effect_mut(e).value = v & 0xffff;
                }
            }
            if id & RESET_TOFIELD != 0 {
                d.card_mut(self).pre_equip_target = None;
            }
            if id & RESET_DISABLE != 0 {
                let counters = self.cc(d).counters.clone();
                for (t, v) in counters {
                    if v[1] > 0 {
                        let c = self.cc(d);
                        let (ctrl, loc, seq) = (c.current.controler, c.current.location, c.current.sequence);
                        let m = d.new_message(MSG_REMOVE_COUNTER);
                        d.msg(m).u16(t).u8(ctrl).u8(loc).u8(seq).u16(v[1]);
                        let c = d.card_mut(self);
                        if let Some(i) = c.counters.iter().position(|x| x.0 == t) {
                            c.counters[i].1[1] = 0;
                            if c.counters[i].1[0] == 0 {
                                c.counters.remove(i);
                            }
                        }
                    }
                }
            }
            if id & RESET_TURN_SET != 0 {
                let (_, peffect) = self.refresh_control_status(d);
                if let Some(pe) = peffect {
                    if d.effect(pe).type_ as u32 & EFFECT_TYPE_SINGLE == 0 || d.effect(pe).condition.is_some() {
                        let ne = d.new_effect();
                        let peid = d.effect(pe).id;
                        let ctrl = self.cc(d).current.controler;
                        let e = d.effect_mut(ne);
                        e.id = peid;
                        e.owner = Some(self);
                        e.handler = Some(self);
                        e.type_ = EFFECT_TYPE_SINGLE as u16;
                        e.code = EFFECT_SET_CONTROL;
                        e.value = ctrl as i32;
                        e.flag[0] = EFFECT_FLAG_CANNOT_DISABLE;
                        e.reset_flag = RESET_EVENT | 0xec0000;
                        self.add_effect(d, ne);
                    }
                }
            }
        }
        // (in effect id order)
        let idx: Vec<EffectId> = self.cc(d).indexer.iter().map(|x| x.0).collect();
        let mut expired = Vec::new();
        for e in idx {
            if e.reset(d, id, reset_type) {
                expired.push(e);
            }
        }
        sort_effects_by_id(d, &mut expired);
        for e in expired {
            if self.indexer_find(d, e).is_some() {
                self.remove_effect(d, e);
            }
        }
    }
    pub fn reset_effect_count(self, d: &mut Duel) {
        for (e, _) in self.cc(d).indexer.clone() {
            if d.effect(e).is_flag(EFFECT_FLAG_COUNT_LIMIT) {
                e.recharge(d);
            }
        }
    }
    /// refresh STATUS_DISABLED and STATUS_FORBIDDEN
    pub fn refresh_disable_status(self, d: &mut Duel) {
        self.filter_immune_effect(d);
        let forbidden = self.is_affected_by_effect(d, EFFECT_FORBIDDEN).is_some();
        d.card_mut(self).set_status(STATUS_FORBIDDEN, forbidden);
        let disabled = self.is_affected_by_effect(d, EFFECT_CANNOT_DISABLE).is_none() && self.is_affected_by_effect(d, EFFECT_DISABLE).is_some();
        d.card_mut(self).set_status(STATUS_DISABLED, disabled);
    }
    pub fn refresh_control_status(self, d: &mut Duel) -> (u8, Option<EffectId>) {
        let mut fin = self.cc(d).owner;
        let mut ceffect = None;
        let mut last_id = 0;
        if d.core.remove_brainwashing && self.is_affected_by_effect(d, EFFECT_REMOVE_BRAINWASHING).is_some() {
            last_id = d.core.last_control_changed_id;
        }
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_SET_CONTROL, &mut eset, true);
        if let Some(&pe) = eset.last() {
            if d.effect(pe).id >= last_id {
                fin = pe.get_value_card(d, Some(self), 0) as u8;
                ceffect = Some(pe);
            }
        }
        (fin, ceffect)
    }
    pub fn count_turn(self, d: &mut Duel, ct: u16) {
        d.card_mut(self).turn_counter = ct;
        let loc = self.get_info_location(d);
        let m = d.new_message(MSG_CARD_HINT);
        d.msg(m).loc(loc).u8(CHINT_TURN).u64(ct);
    }
    pub fn create_relation(self, d: &mut Duel, target: CardId, reset: u32) {
        let c = d.card_mut(self);
        if !c.relations.iter().any(|r| r.0 == target) {
            c.relations.push((target, reset));
        }
    }
    pub fn is_has_relation(self, d: &Duel, target: CardId) -> bool {
        self.cc(d).relations.iter().any(|r| r.0 == target)
    }
    pub fn release_relation(self, d: &mut Duel, target: CardId) {
        d.card_mut(self).relations.retain(|r| r.0 != target);
    }
    pub fn create_relation_chain(self, d: &mut Duel, te: EffectId, chain_id: u16) {
        let c = d.card_mut(self);
        if !c.relate_effect.contains(&(te, chain_id)) {
            c.relate_effect.push((te, chain_id));
        }
    }
    pub fn is_has_relation_chain(self, d: &Duel, te: EffectId, chain_id: u16) -> bool {
        self.cc(d).relate_effect.contains(&(te, chain_id))
    }
    pub fn release_relation_chain(self, d: &mut Duel, te: EffectId, chain_id: u16) {
        d.card_mut(self).relate_effect.retain(|x| *x != (te, chain_id));
    }
    pub fn clear_relate_effect(self, d: &mut Duel) {
        d.card_mut(self).relate_effect.clear();
    }
    pub fn create_relation_effect(self, d: &mut Duel, peffect: EffectId) {
        let found = d.core.current_chain.iter().rev().find(|ch| ch.triggering_effect == Some(peffect)).map(|ch| ch.chain_id);
        match found {
            Some(cid) => self.create_relation_chain(d, peffect, cid),
            None => self.create_relation_chain(d, peffect, 0),
        }
    }
    pub fn is_has_relation_effect(self, d: &Duel, peffect: EffectId) -> bool {
        self.cc(d).relate_effect.iter().any(|x| x.0 == peffect)
    }
    pub fn release_relation_effect(self, d: &mut Duel, peffect: EffectId) {
        let found = d.core.current_chain.iter().rev().find(|ch| ch.triggering_effect == Some(peffect)).map(|ch| ch.chain_id);
        match found {
            Some(cid) => self.release_relation_chain(d, peffect, cid),
            None => self.release_relation_chain(d, peffect, 0),
        }
    }
    pub fn leave_field_redirect(self, d: &mut Duel, reason: u32) -> u32 {
        if self.cc(d).data.type_ & TYPE_TOKEN != 0 {
            return 0;
        }
        let mut es = Vec::new();
        let mut redirects: u32 = 0;
        self.filter_effect(d, EFFECT_LEAVE_FIELD_REDIRECT, &mut es, true);
        for peff in es {
            d.add_param(Value::Int(reason as i64));
            let redirect = peff.get_value_card(d, Some(self), 1) as u32;
            let hp = peff.get_handler_player(d);
            if redirect & LOCATION_HAND != 0 && self.is_affected_by_effect(d, EFFECT_CANNOT_TO_HAND).is_none() && d.is_player_can_send_to_hand(hp, self) {
                redirects |= redirect;
            } else if redirect & LOCATION_DECK != 0 && self.is_affected_by_effect(d, EFFECT_CANNOT_TO_DECK).is_none() && d.is_player_can_send_to_deck(hp, self) {
                redirects |= redirect;
            } else if redirect & LOCATION_REMOVED != 0
                && self.is_affected_by_effect(d, EFFECT_CANNOT_REMOVE).is_none()
                && d.is_player_can_remove(hp, self, REASON_EFFECT)
            {
                redirects |= redirect;
            }
        }
        if redirects & LOCATION_REMOVED != 0 {
            return LOCATION_REMOVED;
        }
        if redirects & LOCATION_DECK != 0 {
            if redirects & LOCATION_DECKBOT == LOCATION_DECKBOT {
                return LOCATION_DECKBOT;
            }
            if redirects & LOCATION_DECKSHF == LOCATION_DECKSHF {
                return LOCATION_DECKSHF;
            }
            return LOCATION_DECK;
        }
        if redirects & LOCATION_HAND != 0 {
            return LOCATION_HAND;
        }
        0
    }
    pub fn destination_redirect(self, d: &mut Duel, destination: u8, reason: u32) -> u32 {
        if self.cc(d).data.type_ & TYPE_TOKEN != 0 {
            return 0;
        }
        let code = match destination as u32 {
            LOCATION_HAND => EFFECT_TO_HAND_REDIRECT,
            LOCATION_DECK => EFFECT_TO_DECK_REDIRECT,
            LOCATION_GRAVE => EFFECT_TO_GRAVE_REDIRECT,
            LOCATION_REMOVED => EFFECT_REMOVE_REDIRECT,
            _ => return 0,
        };
        let mut es = Vec::new();
        self.filter_effect(d, code, &mut es, true);
        for peff in es {
            d.add_param(Value::Int(reason as i64));
            let redirect = peff.get_value_card(d, Some(self), 1) as u32;
            let hp = peff.get_handler_player(d);
            if redirect & LOCATION_HAND != 0 && self.is_affected_by_effect(d, EFFECT_CANNOT_TO_HAND).is_none() && d.is_player_can_send_to_hand(hp, self) {
                return redirect;
            }
            if redirect & LOCATION_DECK != 0 && self.is_affected_by_effect(d, EFFECT_CANNOT_TO_DECK).is_none() && d.is_player_can_send_to_deck(hp, self) {
                return redirect;
            }
            if redirect & LOCATION_REMOVED != 0 && self.is_affected_by_effect(d, EFFECT_CANNOT_REMOVE).is_none() && d.is_player_can_remove(hp, self, REASON_EFFECT)
            {
                return redirect;
            }
            if redirect & LOCATION_GRAVE != 0
                && self.is_affected_by_effect(d, EFFECT_CANNOT_TO_GRAVE).is_none()
                && d.is_player_can_send_to_grave(hp, self, REASON_EFFECT)
            {
                return redirect;
            }
        }
        0
    }
    fn counter_index(self, d: &Duel, t: u16) -> Option<usize> {
        self.cc(d).counters.iter().position(|x| x.0 == t)
    }
    /// counters: [0] permanent, [1] reset while negated
    pub fn add_counter(self, d: &mut Duel, playerid: u8, countertype: u16, count: u16, singly: bool) -> bool {
        if !self.is_can_add_counter(d, playerid, countertype, count, singly, 0) {
            return false;
        }
        let cttype = countertype & !(COUNTER_NEED_ENABLE as u16);
        if self.counter_index(d, cttype).is_none() {
            let c = d.card_mut(self);
            let pos = c.counters.partition_point(|x| x.0 < cttype);
            c.counters.insert(pos, (cttype, [0, 0]));
        }
        let mut pcount = count;
        if singly {
            let mut eset = Vec::new();
            let mut limit: u32 = u16::MAX as u32 + 1;
            self.filter_effect(d, EFFECT_COUNTER_LIMIT + cttype as u32, &mut eset, true);
            for pe in eset {
                limit = limit.min(pe.get_value(d, 0) as u16 as u32);
            }
            if limit != u16::MAX as u32 + 1 {
                let mcount = (limit as u16).wrapping_sub(self.get_counter(d, cttype) as u16);
                if pcount > mcount {
                    pcount = mcount;
                }
            }
        }
        let i = self.counter_index(d, cttype).unwrap();
        {
            let c = d.card_mut(self);
            if countertype as u32 & COUNTER_WITHOUT_PERMIT != 0 && countertype as u32 & COUNTER_NEED_ENABLE == 0 {
                c.counters[i].1[0] = c.counters[i].1[0].wrapping_add(pcount);
            } else {
                c.counters[i].1[1] = c.counters[i].1[1].wrapping_add(pcount);
            }
        }
        let c = self.cc(d);
        let (ctrl, loc, seq) = (c.current.controler, c.current.location, c.current.sequence);
        let m = d.new_message(MSG_ADD_COUNTER);
        d.msg(m).u16(cttype).u8(ctrl).u8(loc).u8(seq).u16(pcount);
        let re = d.core.reason_effect;
        d.raise_single_event(self, None, EVENT_ADD_COUNTER + countertype as u32, re, REASON_EFFECT, playerid, playerid, pcount as u32);
        d.process_single_event();
        true
    }
    pub fn remove_counter(self, d: &mut Duel, countertype: u16, count: u16) -> bool {
        let Some(i) = self.counter_index(d, countertype) else { return false };
        {
            let c = d.card_mut(self);
            if c.counters[i].1[1] <= count {
                let remains = count - c.counters[i].1[1];
                c.counters[i].1[1] = 0;
                if c.counters[i].1[0] <= remains {
                    c.counters.remove(i);
                } else {
                    c.counters[i].1[0] -= remains;
                }
            } else {
                c.counters[i].1[1] -= count;
            }
        }
        let c = self.cc(d);
        let (ctrl, loc, seq) = (c.current.controler, c.current.location, c.current.sequence);
        let m = d.new_message(MSG_REMOVE_COUNTER);
        d.msg(m).u16(countertype).u8(ctrl).u8(loc).u8(seq).u16(count);
        true
    }
    pub fn is_can_add_counter(self, d: &mut Duel, playerid: u8, countertype: u16, mut count: u16, singly: bool, loc: u32) -> bool {
        if count > 0 {
            if !d.is_player_can_place_counter(playerid, self, countertype, count) {
                return false;
            }
            let c = self.cc(d);
            if loc == 0 && (c.current.location as u32 & LOCATION_ONFIELD == 0 || c.is_position(POS_FACEUP) == 0) {
                return false;
            }
            if countertype as u32 & COUNTER_NEED_ENABLE != 0 && c.is_status(STATUS_DISABLED) {
                return false;
            }
        }
        let mut check = countertype as u32 & COUNTER_WITHOUT_PERMIT != 0;
        if !check {
            let mut eset = Vec::new();
            self.filter_effect(d, EFFECT_COUNTER_PERMIT + (countertype as u32 & 0xffff), &mut eset, true);
            for pe in eset {
                let prange = pe.get_value(d, 0) as u32;
                if loc != 0 {
                    check = loc & prange != 0;
                } else if self.cc(d).current.location as u32 & LOCATION_ONFIELD != 0 {
                    let mut filter = true;
                    if let Some(t) = d.effect(pe).target.clone() {
                        d.add_param(Value::Effect(pe));
                        d.add_param(Value::Card(self));
                        filter = d.check_condition(Some(&t), 2);
                    }
                    let c = self.cc(d);
                    check = c.current.is_location(prange) && c.is_position(POS_FACEUP) != 0 && filter;
                } else {
                    check = true;
                }
                if check {
                    break;
                }
            }
        }
        if !check {
            return false;
        }
        let cttype = countertype & !(COUNTER_NEED_ENABLE as u16);
        let mut limit: u32 = u16::MAX as u32 + 1;
        let cur: u32 = match self.counter_index(d, cttype) {
            Some(i) => {
                let v = self.cc(d).counters[i].1;
                v[0] as u32 + v[1] as u32
            }
            None => 0,
        };
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_COUNTER_LIMIT + cttype as u32, &mut eset, true);
        for pe in eset {
            limit = limit.min(pe.get_value(d, 0) as u16 as u32);
        }
        if singly {
            count = 1;
        }
        if limit != u16::MAX as u32 + 1 && cur + count as u32 > limit {
            return false;
        }
        true
    }
    pub fn get_counter(self, d: &Duel, countertype: u16) -> i32 {
        match self.counter_index(d, countertype) {
            Some(i) => {
                let v = self.cc(d).counters[i].1;
                v[0] as i32 + v[1] as i32
            }
            None => 0,
        }
    }
    pub fn set_material(self, d: &mut Duel, materials: CardSet) {
        d.card_mut(self).material_cards = materials;
        for pc in public_order(d, &self.cc(d).material_cards.clone()) {
            d.card_mut(pc).current.reason_card = Some(self);
        }
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_MATERIAL_CHECK, &mut eset, true);
        for pe in eset {
            pe.get_value_card(d, Some(self), 0);
        }
    }
    pub fn add_card_target(self, d: &mut Duel, pcard: CardId) {
        d.card_mut(self).effect_target_cards.insert(pcard);
        d.card_mut(pcard).effect_target_owner.insert(self);
        for e in self.cc(d).target_effect.all() {
            if e.is_disable_related(d) {
                d.add_to_disable_check_list(pcard);
            }
        }
        let a = self.get_info_location(d);
        let b = pcard.get_info_location(d);
        let m = d.new_message(MSG_CARD_TARGET);
        d.msg(m).loc(a).loc(b);
    }
    pub fn cancel_card_target(self, d: &mut Duel, pcard: CardId) {
        if d.card_mut(self).effect_target_cards.remove(pcard) {
            d.card_mut(pcard).effect_target_owner.remove(self);
            for e in self.cc(d).target_effect.all() {
                if e.is_disable_related(d) {
                    d.add_to_disable_check_list(pcard);
                }
            }
            let a = self.get_info_location(d);
            let b = pcard.get_info_location(d);
            let m = d.new_message(MSG_CANCEL_TARGET);
            d.msg(m).loc(a).loc(b);
        }
    }
    pub fn clear_card_target(self, d: &mut Duel) {
        for pc in public_order(d, &self.cc(d).effect_target_owner.clone()) {
            d.card_mut(pc).effect_target_cards.remove(self);
            for e in d.card(pc).target_effect.all() {
                if e.is_disable_related(d) {
                    d.add_to_disable_check_list(self);
                }
            }
        }
        for pc in public_order(d, &self.cc(d).effect_target_cards.clone()) {
            d.card_mut(pc).effect_target_owner.remove(self);
            for e in self.cc(d).target_effect.all() {
                if e.is_disable_related(d) {
                    d.add_to_disable_check_list(pc);
                }
            }
            for (_, pe) in d.card(pc).single_effect.items.clone() {
                if d.effect(pe).owner == Some(self) && d.effect(pe).is_flag(EFFECT_FLAG_OWNER_RELATE) {
                    pc.remove_effect(d, pe);
                }
            }
        }
        let c = d.card_mut(self);
        c.effect_target_owner.clear();
        c.effect_target_cards.clear();
    }
    pub fn filter_effect(self, d: &mut Duel, code: u32, eset: &mut Vec<EffectId>, sort: bool) {
        for pe in self.cc(d).single_effect.range(code) {
            if pe.is_available(d) && (!d.effect(pe).is_flag(EFFECT_FLAG_SINGLE_RANGE) || self.is_affect_by_effect(d, Some(pe))) {
                eset.push(pe);
            }
        }
        for pc in public_order(d, &self.cc(d).equiping_cards.clone()) {
            for pe in d.card(pc).equip_effect.range(code) {
                if pe.is_available(d) && self.is_affect_by_effect(d, Some(pe)) {
                    eset.push(pe);
                }
            }
        }
        for pc in public_order(d, &self.cc(d).effect_target_owner.clone()) {
            for pe in d.card(pc).target_effect.range(code) {
                if pe.is_available(d) && pe.is_target(d, self) && self.is_affect_by_effect(d, Some(pe)) {
                    eset.push(pe);
                }
            }
        }
        for pc in self.cc(d).xyz_materials.clone() {
            for pe in d.card(pc).xmaterial_effect.range(code) {
                if d.effect(pe).type_ as u32 & EFFECT_TYPE_FIELD != 0 {
                    continue;
                }
                if pe.is_available(d) && self.is_affect_by_effect(d, Some(pe)) {
                    eset.push(pe);
                }
            }
        }
        for pe in d.field_effects.aura_effect.range(code) {
            if !d.effect(pe).is_flag(EFFECT_FLAG_PLAYER_TARGET) && pe.is_available(d) && pe.is_target(d, self) && self.is_affect_by_effect(d, Some(pe)) {
                eset.push(pe);
            }
        }
        if sort {
            sort_effects_by_id(d, eset);
        }
    }
    pub fn filter_single_effect(self, d: &mut Duel, code: u32, eset: &mut Vec<EffectId>, sort: bool) {
        for pe in self.cc(d).single_effect.range(code) {
            if pe.is_available(d) && !d.effect(pe).is_flag(EFFECT_FLAG_SINGLE_RANGE) {
                eset.push(pe);
            }
        }
        if sort {
            sort_effects_by_id(d, eset);
        }
    }
    pub fn filter_single_continuous_effect(self, d: &mut Duel, code: u32, eset: &mut Vec<EffectId>, sort: bool) {
        eset.extend(self.cc(d).single_effect.range(code));
        for pc in public_order(d, &self.cc(d).equiping_cards.clone()) {
            eset.extend(d.card(pc).equip_effect.range(code));
        }
        for pc in public_order(d, &self.cc(d).effect_target_owner.clone()) {
            for pe in d.card(pc).target_effect.range(code) {
                if pe.is_target(d, pc) {
                    eset.push(pe);
                }
            }
        }
        for pc in self.cc(d).xyz_materials.clone() {
            for pe in d.card(pc).xmaterial_effect.range(code) {
                if d.effect(pe).type_ as u32 & EFFECT_TYPE_FIELD != 0 {
                    continue;
                }
                eset.push(pe);
            }
        }
        if sort {
            sort_effects_by_id(d, eset);
        }
    }
    /// refresh this->immune_effect
    pub fn filter_immune_effect(self, d: &mut Duel) {
        // (as the C++, the list is cleared first and filled as it goes: the `is_target` checks below
        // can ask whether this card is immune, e.g. through its type, and see the partial list)
        d.card_mut(self).immune_effect.clear();
        let single: Vec<_> = self.cc(d).single_effect.range(EFFECT_IMMUNE_EFFECT);
        d.card_mut(self).immune_effect.extend(single);
        for pc in public_order(d, &self.cc(d).equiping_cards.clone()) {
            let v: Vec<_> = d.card(pc).equip_effect.range(EFFECT_IMMUNE_EFFECT);
            d.card_mut(self).immune_effect.extend(v);
        }
        for pc in public_order(d, &self.cc(d).effect_target_owner.clone()) {
            for pe in d.card(pc).target_effect.range(EFFECT_IMMUNE_EFFECT) {
                if pe.is_target(d, self) {
                    d.card_mut(self).immune_effect.push(pe);
                }
            }
        }
        for pc in self.cc(d).xyz_materials.clone() {
            for pe in d.card(pc).xmaterial_effect.range(EFFECT_IMMUNE_EFFECT) {
                if d.effect(pe).type_ as u32 & EFFECT_TYPE_FIELD != 0 {
                    continue;
                }
                d.card_mut(self).immune_effect.push(pe);
            }
        }
        for pe in d.field_effects.aura_effect.range(EFFECT_IMMUNE_EFFECT) {
            if pe.is_target(d, self) {
                d.card_mut(self).immune_effect.push(pe);
            }
        }
        let mut imm = std::mem::take(&mut d.card_mut(self).immune_effect);
        sort_effects_by_id(d, &mut imm);
        d.card_mut(self).immune_effect = imm;
    }
    pub fn filter_disable_related_cards(self, d: &mut Duel) {
        let mut own = Vec::new();
        self.get_own_effects(d, &mut own);
        for pe in own {
            if pe.is_disable_related(d) {
                let t = d.effect(pe).type_ as u32;
                if t & EFFECT_TYPE_FIELD != 0 {
                    d.update_disable_check_list(pe);
                } else if t & EFFECT_TYPE_EQUIP != 0 && self.cc(d).equiping_target.is_some() {
                    let et = self.cc(d).equiping_target.unwrap();
                    d.add_to_disable_check_list(et);
                } else if t & EFFECT_TYPE_TARGET != 0 && !self.cc(d).effect_target_cards.is_empty() {
                    for tc in public_order(d, &self.cc(d).effect_target_cards.clone()) {
                        d.add_to_disable_check_list(tc);
                    }
                } else if t & EFFECT_TYPE_XMATERIAL != 0 && self.cc(d).overlay_target.is_some() {
                    let ot = self.cc(d).overlay_target.unwrap();
                    d.add_to_disable_check_list(ot);
                }
            }
        }
    }
    /// return: an effect with code which affects this, or None
    pub fn is_affected_by_effect(self, d: &mut Duel, code: u32) -> Option<EffectId> {
        for pe in self.cc(d).single_effect.range(code) {
            if pe.is_available(d) && (!d.effect(pe).is_flag(EFFECT_FLAG_SINGLE_RANGE) || self.is_affect_by_effect(d, Some(pe))) {
                return Some(pe);
            }
        }
        for pc in public_order(d, &self.cc(d).equiping_cards.clone()) {
            for pe in d.card(pc).equip_effect.range(code) {
                if pe.is_available(d) && self.is_affect_by_effect(d, Some(pe)) {
                    return Some(pe);
                }
            }
        }
        for pc in public_order(d, &self.cc(d).effect_target_owner.clone()) {
            for pe in d.card(pc).target_effect.range(code) {
                if pe.is_available(d) && pe.is_target(d, self) && self.is_affect_by_effect(d, Some(pe)) {
                    return Some(pe);
                }
            }
        }
        for pc in self.cc(d).xyz_materials.clone() {
            for pe in d.card(pc).xmaterial_effect.range(code) {
                if d.effect(pe).type_ as u32 & EFFECT_TYPE_FIELD != 0 {
                    continue;
                }
                if pe.is_available(d) && self.is_affect_by_effect(d, Some(pe)) {
                    return Some(pe);
                }
            }
        }
        for pe in d.field_effects.aura_effect.range(code) {
            if !d.effect(pe).is_flag(EFFECT_FLAG_PLAYER_TARGET) && pe.is_target(d, self) && pe.is_available(d) && self.is_affect_by_effect(d, Some(pe)) {
                return Some(pe);
            }
        }
        None
    }
    /// `is_affected_by_effect(code, target)`: also requires the effect's value for `target`.
    pub fn is_affected_by_effect_t(self, d: &mut Duel, code: u32, target: Option<CardId>) -> Option<EffectId> {
        for pe in self.cc(d).single_effect.range(code) {
            if pe.is_available(d)
                && (!d.effect(pe).is_flag(EFFECT_FLAG_SINGLE_RANGE) || self.is_affect_by_effect(d, Some(pe)))
                && pe.get_value_card(d, target, 0) != 0
            {
                return Some(pe);
            }
        }
        for pc in public_order(d, &self.cc(d).equiping_cards.clone()) {
            for pe in d.card(pc).equip_effect.range(code) {
                if pe.is_available(d) && self.is_affect_by_effect(d, Some(pe)) && pe.get_value_card(d, target, 0) != 0 {
                    return Some(pe);
                }
            }
        }
        // (the C++ goes through the equips twice, the second time skipping field effects)
        for pc in public_order(d, &self.cc(d).equiping_cards.clone()) {
            for pe in d.card(pc).equip_effect.range(code) {
                if d.effect(pe).type_ as u32 & EFFECT_TYPE_FIELD != 0 {
                    continue;
                }
                if pe.is_available(d) && self.is_affect_by_effect(d, Some(pe)) && pe.get_value_card(d, target, 0) != 0 {
                    return Some(pe);
                }
            }
        }
        for pc in public_order(d, &self.cc(d).effect_target_owner.clone()) {
            for pe in d.card(pc).target_effect.range(code) {
                if pe.is_available(d) && pe.is_target(d, self) && self.is_affect_by_effect(d, Some(pe)) && pe.get_value_card(d, target, 0) != 0 {
                    return Some(pe);
                }
            }
        }
        for pc in self.cc(d).xyz_materials.clone() {
            for pe in d.card(pc).xmaterial_effect.range(code) {
                if d.effect(pe).type_ as u32 & EFFECT_TYPE_FIELD != 0 {
                    continue;
                }
                if pe.is_available(d) && self.is_affect_by_effect(d, Some(pe)) && pe.get_value_card(d, target, 0) != 0 {
                    return Some(pe);
                }
            }
        }
        for pe in d.field_effects.aura_effect.range(code) {
            if !d.effect(pe).is_flag(EFFECT_FLAG_PLAYER_TARGET)
                && pe.is_available(d)
                && pe.is_target(d, self)
                && self.is_affect_by_effect(d, Some(pe))
                && pe.get_value_card(d, target, 0) != 0
            {
                return Some(pe);
            }
        }
        None
    }
    pub fn get_card_effect(self, d: &mut Duel, code: u32, eset: &mut Vec<EffectId>) {
        for pe in self.cc(d).single_effect.all() {
            if (code == 0 || d.effect(pe).code == code)
                && pe.is_available(d)
                && (!d.effect(pe).is_flag(EFFECT_FLAG_SINGLE_RANGE) || self.is_affect_by_effect(d, Some(pe)))
            {
                eset.push(pe);
            }
        }
        for pe in self.cc(d).field_effect.all() {
            if (code == 0 || d.effect(pe).code == code) && self.is_affect_by_effect(d, Some(pe)) {
                eset.push(pe);
            }
        }
        for pc in public_order(d, &self.cc(d).equiping_cards.clone()) {
            for pe in d.card(pc).equip_effect.all() {
                if (code == 0 || d.effect(pe).code == code) && pe.is_available(d) && self.is_affect_by_effect(d, Some(pe)) {
                    eset.push(pe);
                }
            }
        }
        for pc in public_order(d, &self.cc(d).effect_target_owner.clone()) {
            for pe in d.card(pc).target_effect.range(code) {
                if (code == 0 || d.effect(pe).code == code) && pe.is_available(d) && pe.is_target(d, self) && self.is_affect_by_effect(d, Some(pe)) {
                    eset.push(pe);
                }
            }
        }
        for pc in self.cc(d).xyz_materials.clone() {
            for pe in d.card(pc).xmaterial_effect.all() {
                if d.effect(pe).type_ as u32 & EFFECT_TYPE_FIELD != 0 {
                    continue;
                }
                if (code == 0 || d.effect(pe).code == code) && pe.is_available(d) && self.is_affect_by_effect(d, Some(pe)) {
                    eset.push(pe);
                }
            }
        }
        for pe in d.field_effects.aura_effect.all() {
            if (code == 0 || d.effect(pe).code == code)
                && !d.effect(pe).is_flag(EFFECT_FLAG_PLAYER_TARGET)
                && pe.is_target(d, self)
                && pe.is_available(d)
                && self.is_affect_by_effect(d, Some(pe))
            {
                eset.push(pe);
            }
        }
    }
    pub fn get_own_effects(self, d: &Duel, eset: &mut Vec<EffectId>) {
        eset.extend(self.cc(d).indexer.iter().map(|x| x.0));
        sort_effects_by_id(d, eset);
    }
    /// `card::is_affect_by_effect`: whether this is affected by peffect
    pub fn is_affect_by_effect(self, d: &mut Duel, peffect: Option<EffectId>) -> bool {
        if self.cc(d).is_status(STATUS_SUMMONING) {
            if let Some(pe) = peffect {
                let code = d.effect(pe).code;
                if code != EFFECT_CANNOT_DISABLE_SUMMON && code != EFFECT_CANNOT_DISABLE_SPSUMMON {
                    return false;
                }
            }
        }
        let Some(pe) = peffect else { return true };
        if d.effect(pe).is_flag(EFFECT_FLAG_IGNORE_IMMUNE) {
            return true;
        }
        !pe.is_immuned(d, self)
    }
}
