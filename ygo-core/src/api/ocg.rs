//! The API of the core (`ocgapi.cpp`): creating a duel, adding cards, running it, responses
//! and queries.

use crate::constants::*;
use crate::duel::Duel;
use crate::processor1::Status;
use crate::types::*;
use crate::units;

/// `OCG_NewCardInfo`
#[derive(Clone, Copy, Debug, Default)]
pub struct NewCardInfo {
    pub team: u8,
    pub duelist: u8,
    pub code: u32,
    pub con: u8,
    pub loc: u32,
    pub seq: u32,
    pub pos: u32,
}

/// `OCG_QueryInfo`
#[derive(Clone, Copy, Debug, Default)]
pub struct QueryInfo {
    pub flags: u32,
    pub con: u8,
    pub loc: u32,
    pub seq: u32,
    pub overlay_seq: u32,
}

impl Duel {
    /// `OCG_DuelNewCard`
    pub fn ocg_new_card(&mut self, info: &NewCardInfo) {
        if info.loc.count_ones() > 1 {
            return;
        }
        let mut duelist = info.duelist as usize;
        if duelist == 0 {
            if self.is_location_useable(info.con as u32, info.loc, info.seq) {
                let pcard = self.new_card(info.code);
                self.card_mut(pcard).owner = info.team;
                self.add_card(info.con, pcard, info.loc as u8, info.seq as u8, false);
                self.card_mut(pcard).current.position = info.pos;
                if info.loc & LOCATION_ONFIELD == 0 || info.pos & POS_FACEUP != 0 {
                    pcard.enable_field_effect(self, true);
                    self.adjust_instant();
                }
                if info.loc == LOCATION_MZONE {
                    self.card_mut(pcard).set_status(STATUS_PROC_COMPLETE, true);
                }
            }
        } else {
            if info.team > 1 || info.loc & (LOCATION_DECK | LOCATION_EXTRA) == 0 {
                return;
            }
            let pcard = self.new_card(info.code);
            let player = &mut self.player[info.team as usize];
            if duelist > player.extra_lists_main.len() {
                player.extra_lists_main.resize(duelist, Vec::new());
                player.extra_lists_extra.resize(duelist, Vec::new());
                player.extra_lists_hand.resize(duelist, Vec::new());
                player.extra_extra_p_count.resize(duelist, 0);
            }
            duelist -= 1;
            let list = if info.loc == LOCATION_DECK { &mut player.extra_lists_main[duelist] } else { &mut player.extra_lists_extra[duelist] };
            list.push(pcard);
            let seq = list.len() as u32 - 1;
            let c = self.card_mut(pcard);
            c.current.location = info.loc as u8;
            c.owner = info.team;
            c.current.controler = info.team;
            c.current.position = POS_FACEDOWN_DEFENSE;
            c.current.sequence = seq;
        }
    }

    /// `OCG_StartDuel`
    pub fn ocg_start_duel(&mut self) {
        self.emplace_process(units::Startup { step: 0 });
    }

    /// `OCG_DuelProcess`: runs until there are messages, a question or the end.
    pub fn ocg_process(&mut self) -> Status {
        self.buff.clear();
        self.script_steps = 0;
        loop {
            let flag = self.process();
            self.generate_buffer();
            if !self.buff.is_empty() || flag != Status::Continue {
                return flag;
            }
        }
    }

    /// `OCG_DuelGetMessage`: the messages (each with a u32 length before it).
    pub fn ocg_get_message(&mut self) -> &[u8] {
        self.generate_buffer();
        &self.buff
    }

    /// `OCG_DuelSetResponse`
    pub fn ocg_set_response(&mut self, resp: &[u8]) {
        self.set_response(resp);
    }

    /// `OCG_DuelQueryCount`
    pub fn ocg_query_count(&self, team: u8, loc: u32) -> u32 {
        if team > 1 || loc.count_ones() != 1 {
            return 0;
        }
        let p = &self.player[team as usize];
        match loc {
            LOCATION_HAND => p.list_hand.len() as u32,
            LOCATION_GRAVE => p.list_grave.len() as u32,
            LOCATION_REMOVED => p.list_remove.len() as u32,
            LOCATION_EXTRA => p.list_extra.len() as u32,
            LOCATION_DECK => p.list_main.len() as u32,
            LOCATION_MZONE => p.list_mzone.iter().flatten().count() as u32,
            LOCATION_SZONE => p.list_szone.iter().flatten().count() as u32,
            _ => 0,
        }
    }

    /// `OCG_DuelQuery`: the queried properties of one card (empty if there is none).
    pub fn ocg_query(&mut self, info: &QueryInfo) -> Vec<u8> {
        if (info.loc & !LOCATION_OVERLAY).count_ones() != 1 {
            return Vec::new();
        }
        self.query_buffer.clear();
        let pcard = if info.loc & LOCATION_OVERLAY != 0 {
            self.get_field_card(info.con as u32, info.loc & LOCATION_OVERLAY, info.seq)
                .and_then(|ol| self.card(ol).xyz_materials.get(info.overlay_seq as usize).copied())
        } else {
            self.get_field_card(info.con as u32, info.loc, info.seq)
        };
        match pcard {
            None => Vec::new(),
            Some(c) => {
                c.get_infos(self, info.flags);
                self.query_buffer.clone()
            }
        }
    }

    /// `OCG_DuelQueryLocation`
    pub fn ocg_query_location(&mut self, info: &QueryInfo) -> Vec<u8> {
        self.query_buffer.clear();
        if info.con <= 1 && (info.loc & !LOCATION_OVERLAY).count_ones() == 1 {
            if info.loc & LOCATION_OVERLAY != 0 {
                self.query_buffer.extend_from_slice(&0i16.to_le_bytes());
            } else {
                let p = &self.player[info.con as usize];
                let list: Vec<Option<CardId>> = match info.loc {
                    LOCATION_MZONE => p.list_mzone.clone(),
                    LOCATION_SZONE => p.list_szone.clone(),
                    LOCATION_HAND => p.list_hand.iter().map(|&c| Some(c)).collect(),
                    LOCATION_GRAVE => p.list_grave.iter().map(|&c| Some(c)).collect(),
                    LOCATION_REMOVED => p.list_remove.iter().map(|&c| Some(c)).collect(),
                    LOCATION_EXTRA => p.list_extra.iter().map(|&c| Some(c)).collect(),
                    LOCATION_DECK => p.list_main.iter().map(|&c| Some(c)).collect(),
                    _ => Vec::new(),
                };
                for c in list {
                    match c {
                        None => self.query_buffer.extend_from_slice(&0i16.to_le_bytes()),
                        Some(c) => c.get_infos(self, info.flags),
                    }
                }
            }
            let n = self.query_buffer.len() as u32;
            let mut out = n.to_le_bytes().to_vec();
            out.extend_from_slice(&self.query_buffer);
            self.query_buffer = out;
        }
        self.query_buffer.clone()
    }

    /// `OCG_DuelQueryField`
    pub fn ocg_query_field(&mut self) -> Vec<u8> {
        let mut q: Vec<u8> = Vec::new();
        q.extend_from_slice(&(self.core.duel_options as u32).to_le_bytes());
        for p in 0..2 {
            let pl = &self.player[p];
            q.extend_from_slice(&(pl.lp as u32).to_le_bytes());
            for zone in [&pl.list_mzone, &pl.list_szone] {
                for c in zone.iter() {
                    match c {
                        Some(c) => {
                            let cc = self.card(*c);
                            q.push(1);
                            q.push(cc.current.position as u8);
                            q.extend_from_slice(&(cc.xyz_materials.len() as u32).to_le_bytes());
                        }
                        None => q.push(0),
                    }
                }
            }
            for n in [pl.list_main.len(), pl.list_hand.len(), pl.list_grave.len(), pl.list_remove.len(), pl.list_extra.len()] {
                q.extend_from_slice(&(n as u32).to_le_bytes());
            }
            q.extend_from_slice(&pl.extra_p_count.to_le_bytes());
        }
        q.extend_from_slice(&(self.core.current_chain.len() as u32).to_le_bytes());
        for i in 0..self.core.current_chain.len() {
            let ch = &self.core.current_chain[i];
            let (tc, tl, ts) = (ch.triggering_controler, ch.triggering_location, ch.triggering_sequence);
            let peffect = ch.triggering_effect.unwrap();
            let h = peffect.get_handler(self);
            let info = h.get_info_location(self);
            q.extend_from_slice(&self.card(h).data.code.to_le_bytes());
            q.push(info.controler);
            q.push(info.location);
            q.extend_from_slice(&info.sequence.to_le_bytes());
            q.extend_from_slice(&info.position.to_le_bytes());
            q.push(tc);
            q.push(tl as u8);
            q.extend_from_slice(&ts.to_le_bytes());
            q.extend_from_slice(&self.effect(peffect).description.to_le_bytes());
        }
        self.query_buffer = q.clone();
        q
    }
}
