//! The questions to the players (`playerop.cpp`): step 0 sends the question, step 1+ checks the
//! response (MSG_RETRY asks again).

use crate::card::{card_public_sort, sort_by_less};
use crate::constants::*;
use crate::duel::{Duel, ProgressiveBuffer, ReturnCards};
use crate::types::*;
use crate::units;

/// `parse_response_cards`: the selected entries of `select_cards` (card selections are sorted
/// and must not repeat).
fn parse_response_cards<T: Copy + Ord>(returns: &ProgressiveBuffer, return_cards: &mut ReturnCards<T>, select_cards: &[T], cancelable: bool, dedup: bool) -> bool {
    let typ = returns.at_i32(0);
    //allowed values are from -1 to 3
    if (typ.wrapping_add(1)) as u32 > 4 {
        return false;
    }
    if typ == -1 {
        if cancelable {
            return_cards.canceled = true;
            return true;
        }
        return false;
    }
    let list = &mut return_cards.list;
    if typ == 3 {
        for (i, &c) in select_cards.iter().enumerate() {
            if returns.bit_get(i + 32) {
                list.push(c);
            }
        }
    } else {
        let size = returns.at_u32(1);
        for i in 0..size as usize {
            let idx = match typ {
                0 => returns.at_u32(i + 2) as usize,
                1 => returns.at_u16(i + 4) as usize,
                _ => returns.at_u8(i + 8) as usize,
            };
            if idx >= select_cards.len() {
                return false;
            }
            list.push(select_cards[idx]);
        }
    }
    if dedup {
        // (the core sorts the card pointers: creation order here)
        list.sort();
        let n = list.len();
        list.dedup();
        return list.len() == n;
    }
    true
}

/// `is_declarable`: whether a card can be announced (the announce filter is a stack program).
fn is_declarable(cd: &CardData, opcodes: &[u64]) -> bool {
    let mut stack: Vec<i64> = Vec::new();
    let mut alias = false;
    let mut token = false;
    macro_rules! binary {
        ($f:expr) => {
            if stack.len() >= 2 {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                let f: fn(i64, i64) -> i64 = $f;
                stack.push(f(lhs, rhs));
            }
        };
    }
    macro_rules! unary {
        ($f:expr) => {
            if let Some(v) = stack.pop() {
                let f = $f;
                stack.push(f(v));
            }
        };
    }
    for &opcode in opcodes {
        match opcode {
            OPCODE_ADD => binary!(|a, b| a.wrapping_add(b)),
            OPCODE_SUB => binary!(|a, b| a.wrapping_sub(b)),
            OPCODE_MUL => binary!(|a, b| a.wrapping_mul(b)),
            OPCODE_DIV => binary!(|a, b| if b == 0 { 0 } else { a.wrapping_div(b) }),
            OPCODE_AND => binary!(|a, b| (a != 0 && b != 0) as i64),
            OPCODE_OR => binary!(|a, b| (a != 0 || b != 0) as i64),
            OPCODE_NEG => unary!(|v: i64| v.wrapping_neg()),
            OPCODE_NOT => unary!(|v: i64| (v == 0) as i64),
            OPCODE_BAND => binary!(|a, b| a & b),
            OPCODE_BOR => binary!(|a, b| a | b),
            OPCODE_BXOR => binary!(|a, b| a ^ b),
            OPCODE_BNOT => unary!(|v: i64| !v),
            OPCODE_LSHIFT => binary!(|a, b| a.wrapping_shl(b as u32)),
            OPCODE_RSHIFT => binary!(|a, b| a.wrapping_shr(b as u32)),
            OPCODE_ISCODE => unary!(|v: i64| (cd.code == v as u32) as i64),
            OPCODE_ISTYPE => unary!(|v: i64| (cd.type_ as i64) & v),
            OPCODE_ISRACE => unary!(|v: i64| (cd.race as i64) & v),
            OPCODE_ISATTRIBUTE => unary!(|v: i64| (cd.attribute as i64) & v),
            OPCODE_GETCODE => stack.push(cd.code as i64),
            OPCODE_GETTYPE => stack.push(cd.type_ as i64),
            OPCODE_GETRACE => stack.push(cd.race as i64),
            OPCODE_GETATTRIBUTE => stack.push(cd.attribute as i64),
            OPCODE_ISSETCARD => {
                if let Some(v) = stack.pop() {
                    let set_code = v as i32;
                    let settype = (set_code & 0xfff) as u16;
                    let setsubtype = (set_code & 0xf000) as u16;
                    let res = cd.setcodes.iter().any(|sc| (sc & 0xfff) == settype && (sc & 0xf000 & setsubtype) == setsubtype);
                    stack.push(res as i64);
                }
            }
            OPCODE_ALLOW_ALIASES => alias = true,
            OPCODE_ALLOW_TOKENS => token = true,
            _ => stack.push(opcode as i64),
        }
    }
    if stack.len() != 1 || stack[0] == 0 {
        return false;
    }
    cd.code == CARD_MARINE_DOLPHIN
        || cd.code == CARD_TWINKLE_MOSS
        || ((alias || cd.alias == 0) && (token || (cd.type_ & (TYPE_MONSTER + TYPE_TOKEN)) != (TYPE_MONSTER + TYPE_TOKEN)))
}

fn select_sum_check1(oparam: &[i32], size: i32, index: i32, acc: i32) -> bool {
    if acc == 0 || index == size {
        return false;
    }
    let o1 = oparam[index as usize] & 0xffff;
    let o2 = oparam[index as usize] >> 16;
    if index == size - 1 {
        return acc == o1 || acc == o2;
    }
    (acc > o1 && select_sum_check1(oparam, size, index + 1, acc - o1)) || (o2 > 0 && acc > o2 && select_sum_check1(oparam, size, index + 1, acc - o2))
}

impl Duel {
    fn retry(&mut self) -> bool {
        self.new_message(MSG_RETRY);
        false
    }
    fn hint_select_none(&mut self, playerid: u8) {
        let m = self.new_message(MSG_HINT);
        self.msg(m).u8(HINT_SELECTMSG).u8(playerid).u64(0u64);
    }
    fn simple_ai(&self, playerid: u8) -> bool {
        playerid == 1 && self.is_flag(DUEL_SIMPLE_AI)
    }
    /// `core.select_chains.sort(chain::chain_operation_sort)` (stable, by effect id).
    fn sort_select_chains(&mut self) {
        let mut v: Vec<_> = std::mem::take(&mut self.core.select_chains).into();
        v.sort_by_key(|ch| self.effect(ch.triggering_effect.unwrap()).id);
        self.core.select_chains = v.into();
    }
    fn sort_select_cards_public(&mut self) {
        let mut v = std::mem::take(&mut self.core.select_cards);
        sort_by_less(&mut v, |a, b| card_public_sort(self, a, b));
        self.core.select_cards = v;
    }
    /// The activatable effects of a battle/idle command (code, controler, location, sequence,
    /// description, client mode).
    fn write_select_chains_cmd(&mut self, m: usize) {
        self.sort_select_chains();
        let n = self.core.select_chains.len();
        self.msg(m).u32(n as u32);
        for i in 0..n {
            let peffect = self.core.select_chains[i].triggering_effect.unwrap();
            let pcard = peffect.get_handler(self);
            let c = self.card(pcard);
            let (code, ctrl, loc, seq) = (c.data.code, c.current.controler, c.current.location, c.current.sequence);
            let desc = self.effect(peffect).description;
            let mode = peffect.get_client_mode(self);
            self.msg(m).u32(code).u8(ctrl).u8(loc).u32(seq).u64(desc).u8(mode);
        }
    }
    fn write_cards_seq32(&mut self, m: usize, cards: &[CardId]) {
        self.msg(m).u32(cards.len() as u32);
        for &pcard in cards {
            let c = self.card(pcard);
            let (code, ctrl, loc, seq) = (c.data.code, c.current.controler, c.current.location, c.current.sequence);
            self.msg(m).u32(code).u8(ctrl).u8(loc).u32(seq);
        }
    }
    fn write_cards_info(&mut self, m: usize, cards: &[CardId]) {
        self.msg(m).u32(cards.len() as u32);
        for &pcard in cards {
            let code = self.card(pcard).data.code;
            let loc = pcard.get_info_location(self);
            self.msg(m).u32(code).loc(loc);
        }
    }

    pub fn process_select_battle_cmd(&mut self, arg: &mut units::SelectBattleCmd) -> bool {
        let playerid = arg.playerid;
        if arg.step == 0 {
            let m = self.new_message(MSG_SELECT_BATTLECMD);
            self.msg(m).u8(playerid);
            //Activatable
            self.write_select_chains_cmd(m);
            //Attackable
            let ac = self.core.attackable_cards.clone();
            self.msg(m).u32(ac.len() as u32);
            for pcard in ac {
                let c = self.card(pcard);
                let (code, ctrl, loc, seq, da) = (c.data.code, c.current.controler, c.current.location, c.current.sequence, c.direct_attackable);
                self.msg(m).u32(code).u8(ctrl).u8(loc).u8(seq as u8).u8(da);
            }
            //M2, EP
            let (to_m2, to_ep) = (self.core.to_m2, self.core.to_ep);
            self.msg(m).u8(to_m2 as u8).u8(to_ep as u8);
            false
        } else {
            let r = self.returns.at_i32(0);
            let t = (r & 0xffff) as u32;
            let s = (r >> 16) as u32;
            if t > 3
                || (t == 0 && s as usize >= self.core.select_chains.len())
                || (t == 1 && s as usize >= self.core.attackable_cards.len())
                || (t == 2 && !self.core.to_m2)
                || (t == 3 && !self.core.to_ep)
            {
                return self.retry();
            }
            true
        }
    }

    pub fn process_select_idle_cmd(&mut self, arg: &mut units::SelectIdleCmd) -> bool {
        let playerid = arg.playerid;
        let can_shuffle = |d: &Duel| d.infos.can_shuffle && d.player[playerid as usize].list_hand.len() > 1;
        if arg.step == 0 {
            let m = self.new_message(MSG_SELECT_IDLECMD);
            self.msg(m).u8(playerid);
            //idle summon
            let v = self.core.summonable_cards.clone();
            self.write_cards_seq32(m, &v);
            //idle spsummon
            let v = self.core.spsummonable_cards.clone();
            self.write_cards_seq32(m, &v);
            //idle pos change
            let v = self.core.repositionable_cards.clone();
            self.msg(m).u32(v.len() as u32);
            for pcard in v {
                let c = self.card(pcard);
                let (code, ctrl, loc, seq) = (c.data.code, c.current.controler, c.current.location, c.current.sequence);
                self.msg(m).u32(code).u8(ctrl).u8(loc).u8(seq as u8);
            }
            //idle mset
            let v = self.core.msetable_cards.clone();
            self.write_cards_seq32(m, &v);
            //idle sset
            let v = self.core.ssetable_cards.clone();
            self.write_cards_seq32(m, &v);
            //idle activate
            self.write_select_chains_cmd(m);
            //To BP
            let to_bp = self.infos.phase as u32 == PHASE_MAIN1 && self.core.to_bp;
            let to_ep = self.core.to_ep;
            let sh = can_shuffle(self);
            self.msg(m).u8(to_bp as u8).u8(to_ep as u8).u8(sh as u8);
            false
        } else {
            let r = self.returns.at_i32(0);
            let t = (r & 0xffff) as u32;
            let s = (r >> 16) as u32 as usize;
            let c = &self.core;
            if t > 8
                || (t == 0 && s >= c.summonable_cards.len())
                || (t == 1 && s >= c.spsummonable_cards.len())
                || (t == 2 && s >= c.repositionable_cards.len())
                || (t == 3 && s >= c.msetable_cards.len())
                || (t == 4 && s >= c.ssetable_cards.len())
                || (t == 5 && s >= c.select_chains.len())
                || (t == 6 && (self.infos.phase as u32 != PHASE_MAIN1 || !c.to_bp))
                || (t == 7 && !c.to_ep)
                || (t == 8 && !can_shuffle(self))
            {
                return self.retry();
            }
            true
        }
    }

    pub fn process_select_effect_yes_no(&mut self, arg: &mut units::SelectEffectYesNo) -> bool {
        let (playerid, pcard, description) = (arg.playerid, arg.pcard.unwrap(), arg.description);
        if arg.step == 0 {
            if self.simple_ai(playerid) {
                self.returns.set_i32(0, 1);
                return true;
            }
            let code = self.card(pcard).data.code;
            let loc = pcard.get_info_location(self);
            let m = self.new_message(MSG_SELECT_EFFECTYN);
            self.msg(m).u8(playerid).u32(code).loc(loc).u64(description);
            self.returns.set_i32(0, -1);
            false
        } else {
            let r = self.returns.at_i32(0);
            if r != 0 && r != 1 {
                return self.retry();
            }
            true
        }
    }

    pub fn process_select_yes_no(&mut self, arg: &mut units::SelectYesNo) -> bool {
        let (playerid, description) = (arg.playerid, arg.description);
        if arg.step == 0 {
            if self.simple_ai(playerid) {
                self.returns.set_i32(0, 1);
                return true;
            }
            let m = self.new_message(MSG_SELECT_YESNO);
            self.msg(m).u8(playerid).u64(description);
            self.returns.set_i32(0, -1);
            false
        } else {
            let r = self.returns.at_i32(0);
            if r != 0 && r != 1 {
                return self.retry();
            }
            true
        }
    }

    pub fn process_select_option(&mut self, arg: &mut units::SelectOption) -> bool {
        let playerid = arg.playerid;
        if arg.step == 0 {
            self.returns.set_i32(0, -1);
            if self.core.select_options.is_empty() {
                self.hint_select_none(playerid);
                return true;
            }
            if self.simple_ai(playerid) {
                self.returns.set_i32(0, 0);
                return true;
            }
            let opts = self.core.select_options.clone();
            let m = self.new_message(MSG_SELECT_OPTION);
            self.msg(m).u8(playerid).u8(opts.len() as u8);
            for o in opts {
                self.msg(m).u64(o);
            }
            false
        } else {
            let r = self.returns.at_i32(0);
            if r < 0 || r >= self.core.select_options.len() as i32 {
                return self.retry();
            }
            true
        }
    }

    fn parse_response_cards(&mut self, cancelable: bool) -> bool {
        parse_response_cards(&self.returns, &mut self.return_cards, &self.core.select_cards, cancelable, true)
    }

    pub fn process_select_card(&mut self, arg: &mut units::SelectCard) -> bool {
        let (playerid, cancelable) = (arg.playerid, arg.cancelable);
        let mut min = arg.min;
        let mut max = arg.max;
        if arg.step == 0 {
            self.return_cards.clear();
            self.returns.clear();
            if max == 0 || self.core.select_cards.is_empty() {
                self.hint_select_none(playerid);
                return true;
            }
            if max as usize > self.core.select_cards.len() {
                max = self.core.select_cards.len() as u8;
            }
            if min > max {
                min = max;
            }
            if self.simple_ai(playerid) {
                for i in 0..min as usize {
                    let c = self.core.select_cards[i];
                    self.return_cards.list.push(c);
                }
                return true;
            }
            arg.min = min;
            arg.max = max;
            let m = self.new_message(MSG_SELECT_CARD);
            self.msg(m).u8(playerid).u8((cancelable || min == 0) as u8).u32(min as u32).u32(max as u32);
            self.sort_select_cards_public();
            let sc = self.core.select_cards.clone();
            self.write_cards_info(m, &sc);
            false
        } else {
            if !self.parse_response_cards(cancelable || min == 0) {
                self.return_cards.clear();
                return self.retry();
            }
            if self.return_cards.canceled {
                return true;
            }
            let n = self.return_cards.list.len();
            if n < min as usize || n > max as usize {
                self.return_cards.clear();
                return self.retry();
            }
            true
        }
    }

    pub fn process_select_card_codes(&mut self, arg: &mut units::SelectCardCodes) -> bool {
        let (playerid, cancelable) = (arg.playerid, arg.cancelable);
        let mut min = arg.min;
        let mut max = arg.max;
        if arg.step == 0 {
            self.return_card_codes.clear();
            self.returns.clear();
            if max == 0 || self.core.select_cards_codes.is_empty() {
                self.hint_select_none(playerid);
                return true;
            }
            if max as usize > self.core.select_cards_codes.len() {
                max = self.core.select_cards_codes.len() as u8;
            }
            if min > max {
                min = max;
            }
            if self.simple_ai(playerid) {
                for i in 0..min as usize {
                    let c = self.core.select_cards_codes[i];
                    self.return_card_codes.list.push(c);
                }
                return true;
            }
            arg.min = min;
            arg.max = max;
            let codes = self.core.select_cards_codes.clone();
            let m = self.new_message(MSG_SELECT_CARD);
            self.msg(m).u8(playerid).u8((cancelable || min == 0) as u8).u32(min as u32).u32(max as u32).u32(codes.len() as u32);
            for (code, _) in codes {
                self.msg(m).u32(code).loc(LocInfo { controler: playerid, location: 0, sequence: 0, position: 0 });
            }
            false
        } else {
            if !parse_response_cards(&self.returns, &mut self.return_card_codes, &self.core.select_cards_codes, cancelable || min == 0, false) {
                self.return_card_codes.clear();
                return self.retry();
            }
            if self.return_card_codes.canceled {
                return true;
            }
            let n = self.return_card_codes.list.len();
            if n < min as usize || n > max as usize {
                self.return_card_codes.clear();
                return self.retry();
            }
            true
        }
    }

    pub fn process_select_unselect_card(&mut self, arg: &mut units::SelectUnselectCard) -> bool {
        let (playerid, cancelable, min, max, finishable) = (arg.playerid, arg.cancelable, arg.min, arg.max, arg.finishable);
        if arg.step == 0 {
            self.return_cards.clear();
            self.returns.clear();
            if self.core.select_cards.is_empty() && self.core.unselect_cards.is_empty() {
                self.hint_select_none(playerid);
                return true;
            }
            if self.simple_ai(playerid) {
                if finishable {
                    self.return_cards.canceled = true;
                } else {
                    let c = self.core.select_cards.first().copied().unwrap_or_else(|| self.core.unselect_cards[0]);
                    self.return_cards.list.push(c);
                }
                return true;
            }
            let m = self.new_message(MSG_SELECT_UNSELECT_CARD);
            self.msg(m).u8(playerid).u8(finishable as u8).u8(cancelable as u8).u32(min as u32).u32(max as u32);
            self.sort_select_cards_public();
            let sc = self.core.select_cards.clone();
            self.write_cards_info(m, &sc);
            let uc = self.core.unselect_cards.clone();
            self.write_cards_info(m, &uc);
            false
        } else {
            let r = self.returns.at_i32(0);
            if r == -1 {
                if cancelable || finishable {
                    self.return_cards.canceled = true;
                    return true;
                }
                self.return_cards.clear();
                return self.retry();
            }
            if r == 0 || r > 1 {
                self.return_cards.clear();
                return self.retry();
            }
            let ns = self.core.select_cards.len() as i32;
            let max_ = ns + self.core.unselect_cards.len() as i32;
            let retval = self.returns.at_i32(1);
            if retval < 0 || retval >= max_ {
                return self.retry();
            }
            let c = if retval >= ns { self.core.unselect_cards[(retval - ns) as usize] } else { self.core.select_cards[retval as usize] };
            self.return_cards.list.push(c);
            true
        }
    }

    pub fn process_select_chain(&mut self, arg: &mut units::SelectChain) -> bool {
        let (playerid, spe_count, forced) = (arg.playerid, arg.spe_count, arg.forced);
        if arg.step == 0 {
            self.returns.set_i32(0, -1);
            if self.simple_ai(playerid) {
                if self.core.select_chains.is_empty() {
                    self.returns.set_i32(0, -1);
                } else if forced {
                    self.returns.set_i32(0, 0);
                } else {
                    let act = !self.core.current_chain.iter().any(|ch| ch.triggering_player == 1);
                    self.returns.set_i32(0, if act { 0 } else { -1 });
                }
                return true;
            }
            let p = playerid as usize;
            let (h0, h1) = (self.core.hint_timing[p], self.core.hint_timing[1 - p]);
            let m = self.new_message(MSG_SELECT_CHAIN);
            self.msg(m).u8(playerid).u8(spe_count).u8(forced as u8).u32(h0).u32(h1);
            self.sort_select_chains();
            let n = self.core.select_chains.len();
            self.msg(m).u32(n as u32);
            for i in 0..n {
                let peffect = self.core.select_chains[i].triggering_effect.unwrap();
                let pcard = peffect.get_handler(self);
                let code = self.card(pcard).data.code;
                let loc = pcard.get_info_location(self);
                let desc = self.effect(peffect).description;
                let mode = peffect.get_client_mode(self);
                self.msg(m).u32(code).loc(loc).u64(desc).u8(mode);
            }
            false
        } else {
            let r = self.returns.at_i32(0);
            if !forced && r == -1 {
                return true;
            }
            if r < 0 || r >= self.core.select_chains.len() as i32 {
                return self.retry();
            }
            true
        }
    }

    pub fn process_select_place(&mut self, arg: &mut units::SelectPlace) -> bool {
        let (playerid, count, disable_field) = (arg.playerid, arg.count, arg.disable_field);
        let mut flag = arg.flag;
        if arg.step == 0 {
            if count == 0 {
                self.hint_select_none(playerid);
                return true;
            }
            if self.simple_ai(playerid) {
                flag = !flag;
                let filter: u32;
                let mut pzone = false;
                let r = &mut self.returns;
                if flag & 0x7f != 0 {
                    r.set_i8(0, 1);
                    r.set_i8(1, LOCATION_MZONE as i8);
                    filter = flag & 0x7f;
                } else if flag & 0x1f00 != 0 {
                    r.set_i8(0, 1);
                    r.set_i8(1, LOCATION_SZONE as i8);
                    filter = (flag >> 8) & 0x1f;
                } else if flag & 0xc000 != 0 {
                    r.set_i8(0, 1);
                    r.set_i8(1, LOCATION_SZONE as i8);
                    filter = (flag >> 14) & 0x3;
                    pzone = true;
                } else if flag & 0x7f0000 != 0 {
                    r.set_i8(0, 0);
                    r.set_i8(1, LOCATION_MZONE as i8);
                    filter = (flag >> 16) & 0x7f;
                } else if flag & 0x1f000000 != 0 {
                    r.set_i8(0, 0);
                    r.set_i8(1, LOCATION_SZONE as i8);
                    filter = (flag >> 24) & 0x1f;
                } else {
                    r.set_i8(0, 0);
                    r.set_i8(1, LOCATION_SZONE as i8);
                    filter = (flag >> 30) & 0x3;
                    pzone = true;
                }
                if !pzone {
                    for (bit, seq) in [(0x40, 6), (0x20, 5), (0x4, 2), (0x2, 1), (0x8, 3), (0x1, 0), (0x10, 4)] {
                        if filter & bit != 0 {
                            r.set_i8(2, seq);
                            break;
                        }
                    }
                } else if filter & 0x1 != 0 {
                    r.set_i8(2, 6);
                } else if filter & 0x2 != 0 {
                    r.set_i8(2, 7);
                }
                return true;
            }
            let m = self.new_message(if disable_field { MSG_SELECT_DISFIELD } else { MSG_SELECT_PLACE });
            self.msg(m).u8(playerid).u8(count).u32(flag);
            self.returns.set_i8(0, 0);
            false
        } else {
            let mut pt: usize = 0;
            for _ in 0..count {
                let select_player = self.returns.at_u8(pt);
                if select_player > 1 {
                    return self.retry();
                }
                let isplayerid = select_player == playerid;
                let location = self.returns.at_u8(pt + 1) as u32;
                if location != LOCATION_MZONE && location != LOCATION_SZONE {
                    return self.retry();
                }
                let ismzone = location == LOCATION_MZONE;
                let sequence = self.returns.at_u8(pt + 2);
                //szone allow max of 8 zones, so 0-7, mzones 7 zones, so 0-6
                if sequence > 7 - ismzone as u8 {
                    return self.retry();
                }
                let mut to_check = 0x1u32 << sequence;
                if !ismzone {
                    to_check <<= 8;
                }
                if !isplayerid {
                    to_check <<= 16;
                }
                if to_check & flag != 0 {
                    return self.retry();
                }
                flag |= to_check;
                pt += 3;
            }
            true
        }
    }

    pub fn process_select_position(&mut self, arg: &mut units::SelectPosition) -> bool {
        let (playerid, code) = (arg.playerid, arg.code);
        let mut positions = arg.positions;
        if arg.step == 0 {
            if positions == 0 {
                self.returns.set_i32(0, POS_FACEUP_ATTACK as i32);
                return true;
            }
            positions &= 0xf;
            if matches!(positions, 0x1 | 0x2 | 0x4 | 0x8) {
                self.returns.set_i32(0, positions as i32);
                return true;
            }
            if self.simple_ai(playerid) {
                let p = if positions & 0x4 != 0 {
                    0x4
                } else if positions & 0x1 != 0 {
                    0x1
                } else if positions & 0x8 != 0 {
                    0x8
                } else {
                    0x2
                };
                self.returns.set_i32(0, p);
                return true;
            }
            let m = self.new_message(MSG_SELECT_POSITION);
            self.msg(m).u8(playerid).u32(code).u8(positions);
            self.returns.set_i32(0, 0);
            false
        } else {
            let pos = self.returns.at_i32(0) as u32;
            if !matches!(pos, 0x1 | 0x2 | 0x4 | 0x8) || pos & positions as u32 == 0 {
                return self.retry();
            }
            true
        }
    }

    pub fn process_select_tribute_p(&mut self, arg: &mut units::SelectTributeP) -> bool {
        let (playerid, cancelable) = (arg.playerid, arg.cancelable);
        let mut min = arg.min;
        let mut max = arg.max;
        if arg.step == 0 {
            self.returns.clear();
            self.return_cards.clear();
            if max == 0 || self.core.select_cards.is_empty() {
                self.hint_select_none(playerid);
                return true;
            }
            let mut tm: u8 = 0;
            for &pcard in &self.core.select_cards {
                tm = tm.wrapping_add(self.card(pcard).release_param as u8);
            }
            if max > 5 {
                max = 5;
            }
            if max > tm {
                max = tm;
            }
            if min > max {
                min = max;
            }
            arg.min = min;
            arg.max = max;
            let m = self.new_message(MSG_SELECT_TRIBUTE);
            self.msg(m).u8(playerid).u8((cancelable || min == 0) as u8).u32(min as u32).u32(max as u32);
            self.sort_select_cards_public();
            let sc = self.core.select_cards.clone();
            self.msg(m).u32(sc.len() as u32);
            for pcard in sc {
                let c = self.card(pcard);
                let (code, ctrl, loc, seq, rp) = (c.data.code, c.current.controler, c.current.location, c.current.sequence, c.release_param);
                self.msg(m).u32(code).u8(ctrl).u8(loc).u32(seq).u8(rp as u8);
            }
            false
        } else {
            if !self.parse_response_cards(cancelable || min == 0) {
                self.return_cards.clear();
                return self.retry();
            }
            if self.return_cards.canceled {
                return true;
            }
            if self.return_cards.list.len() > max as usize {
                self.return_cards.clear();
                return self.retry();
            }
            let tt: i32 = self.return_cards.list.iter().map(|&c| self.card(c).release_param as i32).sum();
            if tt < min as i32 {
                self.return_cards.clear();
                return self.retry();
            }
            true
        }
    }

    pub fn process_select_counter(&mut self, arg: &mut units::SelectCounter) -> bool {
        let (playerid, countertype, self_, oppo) = (arg.playerid, arg.countertype, arg.self_, arg.oppo);
        let mut count = arg.count;
        if arg.step == 0 {
            if count == 0 {
                return true;
            }
            let mut avail = self_;
            let mut fp = playerid as usize;
            let mut total: u32 = 0;
            self.core.select_cards.clear();
            for _ in 0..2 {
                if avail != 0 {
                    let zones: Vec<CardId> = self.player[fp].list_mzone.iter().chain(self.player[fp].list_szone.iter()).flatten().copied().collect();
                    for pcard in zones {
                        let ct = pcard.get_counter(self, countertype);
                        if ct != 0 {
                            self.core.select_cards.push(pcard);
                            total = total.wrapping_add(ct as u32);
                        }
                    }
                }
                fp = 1 - fp;
                avail = oppo;
            }
            if count as u32 > total {
                count = total as u16;
            }
            if self.core.select_cards.len() == 1 {
                self.returns.set_i16(0, count as i16);
                return true;
            }
            let m = self.new_message(MSG_SELECT_COUNTER);
            let n = self.core.select_cards.len() as u32;
            self.msg(m).u8(playerid).u16(countertype).u16(count).u32(n);
            self.sort_select_cards_public();
            let sc = self.core.select_cards.clone();
            for pcard in sc {
                let c = self.card(pcard);
                let (code, ctrl, loc, seq) = (c.data.code, c.current.controler, c.current.location, c.current.sequence);
                let ct = pcard.get_counter(self, countertype);
                self.msg(m).u32(code).u8(ctrl).u8(loc).u8(seq as u8).u16(ct as u16);
            }
            false
        } else {
            let mut ct: u16 = 0;
            for i in 0..self.core.select_cards.len() {
                let pcard = self.core.select_cards[i];
                let r = self.returns.at_i16(i);
                if pcard.get_counter(self, countertype) < r as i32 {
                    return self.retry();
                }
                ct = ct.wrapping_add(r as u16);
            }
            if ct != count {
                return self.retry();
            }
            true
        }
    }

    pub fn process_select_sum(&mut self, arg: &mut units::SelectSum) -> bool {
        let (playerid, acc, min) = (arg.playerid, arg.acc, arg.min);
        let mut max = arg.max;
        if arg.step == 0 {
            self.return_cards.clear();
            self.returns.clear();
            if self.core.select_cards.is_empty() {
                self.hint_select_none(playerid);
                return true;
            }
            let m = self.new_message(MSG_SELECT_SUM);
            self.msg(m).u8(playerid).u8((max == 0) as u8);
            if max < min {
                max = min;
            }
            self.msg(m).u32((acc & 0xffff) as u32).u32(min as u32).u32(max as u32);
            let ms = self.core.must_select_cards.clone();
            self.msg(m).u32(ms.len() as u32);
            for pcard in ms {
                let code = self.card(pcard).data.code;
                let loc = pcard.get_info_location(self);
                let sp = self.card(pcard).sum_param;
                self.msg(m).u32(code).loc(loc).u32(sp);
            }
            let n = self.core.select_cards.len() as u32;
            self.msg(m).u32(n);
            self.sort_select_cards_public();
            let sc = self.core.select_cards.clone();
            for pcard in sc {
                let code = self.card(pcard).data.code;
                let loc = pcard.get_info_location(self);
                let sp = self.card(pcard).sum_param;
                self.msg(m).u32(code).loc(loc).u32(sp);
            }
            false
        } else {
            if !self.parse_response_cards(false) {
                self.return_cards.clear();
                return self.retry();
            }
            let tot = self.return_cards.list.len() as i32;
            let params: Vec<i32> = self.core.must_select_cards.iter().chain(self.return_cards.list.iter()).map(|&c| self.card(c).sum_param as i32).collect();
            if max != 0 {
                if tot < min || tot > max {
                    self.return_cards.clear();
                    return self.retry();
                }
                let mcount = self.core.must_select_cards.len() as i32;
                if !select_sum_check1(&params, tot + mcount, 0, acc) {
                    self.return_cards.clear();
                    return self.retry();
                }
                true
            } else {
                let mut sum: i32 = 0;
                let mut mx: i32 = 0;
                let mut mn: i32 = 0x7fffffff;
                for op in params {
                    let o1 = op & 0xffff;
                    let o2 = op >> 16;
                    let ms = if o2 != 0 && o2 < o1 { o2 } else { o1 };
                    sum += ms;
                    mx += if o2 > o1 { o2 } else { o1 };
                    if ms < mn {
                        mn = ms;
                    }
                }
                if mx < acc || sum - mn >= acc {
                    self.return_cards.clear();
                    return self.retry();
                }
                true
            }
        }
    }

    pub fn process_sort_card(&mut self, arg: &mut units::SortCard) -> bool {
        let (playerid, is_chain) = (arg.playerid, arg.is_chain);
        if arg.step == 0 {
            self.returns.clear();
            if self.simple_ai(playerid) {
                self.returns.set_i8(0, -1);
                return true;
            }
            if self.core.select_cards.is_empty() {
                self.hint_select_none(playerid);
                return true;
            }
            let m = self.new_message(if is_chain { MSG_SORT_CHAIN } else { MSG_SORT_CARD });
            let sc = self.core.select_cards.clone();
            self.msg(m).u8(playerid).u32(sc.len() as u32);
            for pcard in sc {
                let c = self.card(pcard);
                let (code, ctrl, loc, seq) = (c.data.code, c.current.controler, c.current.location, c.current.sequence);
                self.msg(m).u32(code).u8(ctrl).u32(loc as u32).u32(seq);
            }
            false
        } else {
            if self.returns.at_i8(0) == -1 {
                return true;
            }
            let m = self.core.select_cards.len() as u8;
            let mut c = vec![false; m as usize];
            for i in 0..m as usize {
                let v = self.returns.at_i8(i);
                if v < 0 || v as u8 >= m || c[v as usize] {
                    return self.retry();
                }
                c[v as usize] = true;
            }
            true
        }
    }

    pub fn process_announce_race(&mut self, arg: &mut units::AnnounceRace) -> bool {
        let (playerid, count, available) = (arg.playerid, arg.count, arg.available);
        if arg.step == 0 {
            if count == 0 {
                self.hint_select_none(playerid);
                return true;
            }
            let m = self.new_message(MSG_ANNOUNCE_RACE);
            self.msg(m).u8(playerid).u8(count).u64(available);
            false
        } else {
            let selected = self.returns.at_u64(0);
            if selected & !available != 0 || selected.count_ones() != count as u32 {
                return self.retry();
            }
            let m = self.new_message(MSG_HINT);
            self.msg(m).u8(HINT_RACE).u8(playerid).u64(selected);
            true
        }
    }

    pub fn process_announce_attribute(&mut self, arg: &mut units::AnnounceAttribute) -> bool {
        let (playerid, count, available) = (arg.playerid, arg.count, arg.available);
        if arg.step == 0 {
            if count == 0 {
                self.hint_select_none(playerid);
                return true;
            }
            let m = self.new_message(MSG_ANNOUNCE_ATTRIB);
            self.msg(m).u8(playerid).u8(count).u32(available);
            false
        } else {
            let selected = self.returns.at_u32(0);
            if selected & !available != 0 || selected.count_ones() != count as u32 {
                return self.retry();
            }
            let m = self.new_message(MSG_HINT);
            self.msg(m).u8(HINT_ATTRIB).u8(playerid).u64(selected as u64);
            true
        }
    }

    pub fn process_announce_card(&mut self, arg: &mut units::AnnounceCard) -> bool {
        let playerid = arg.playerid;
        if arg.step == 0 {
            let opts = self.core.select_options.clone();
            let m = self.new_message(MSG_ANNOUNCE_CARD);
            self.msg(m).u8(playerid).u8(opts.len() as u8);
            for o in opts {
                self.msg(m).u64(o);
            }
            false
        } else {
            let code = self.returns.at_i32(0);
            let data = self.read_card(code as u32);
            if data.code == 0 || !is_declarable(&data, &self.core.select_options) {
                return self.retry();
            }
            let m = self.new_message(MSG_HINT);
            self.msg(m).u8(HINT_CODE).u8(playerid).u64(code as i64 as u64);
            true
        }
    }

    pub fn process_announce_number(&mut self, arg: &mut units::AnnounceNumber) -> bool {
        let playerid = arg.playerid;
        if arg.step == 0 {
            let opts = self.core.select_options.clone();
            let m = self.new_message(MSG_ANNOUNCE_NUMBER);
            self.msg(m).u8(playerid).u8(opts.len() as u8);
            for o in opts {
                self.msg(m).u64(o);
            }
            false
        } else {
            let ret = self.returns.at_i32(0);
            if ret < 0 || ret >= self.core.select_options.len() as i32 {
                return self.retry();
            }
            let v = self.core.select_options[ret as usize];
            let m = self.new_message(MSG_HINT);
            self.msg(m).u8(HINT_NUMBER).u8(playerid).u64(v);
            true
        }
    }

    pub fn process_rock_paper_scissors(&mut self, arg: &mut units::RockPaperScissors) -> bool {
        let checkvalid = |d: &mut Duel, arg: &mut units::RockPaperScissors| {
            let ret = d.returns.at_i32(0);
            if !(1..=3).contains(&ret) {
                d.new_message(MSG_RETRY);
                arg.step -= 1;
                return false;
            }
            true
        };
        let repeat = arg.repeat;
        match arg.step {
            0 => {
                let m = self.new_message(MSG_ROCK_PAPER_SCISSORS);
                self.msg(m).u8(0u8);
                false
            }
            1 => {
                if checkvalid(self, arg) {
                    arg.hand0 = self.returns.at_i32(0) as u8;
                    let m = self.new_message(MSG_ROCK_PAPER_SCISSORS);
                    self.msg(m).u8(1u8);
                }
                false
            }
            2 => {
                if !checkvalid(self, arg) {
                    return false;
                }
                let hand0 = arg.hand0;
                let hand1 = self.returns.at_i32(0) as u8;
                let m = self.new_message(MSG_HAND_RES);
                self.msg(m).u8(hand0.wrapping_add(hand1 << 2));
                if hand0 == hand1 {
                    if repeat {
                        let m = self.new_message(MSG_ROCK_PAPER_SCISSORS);
                        self.msg(m).u8(0u8);
                        arg.step = 0;
                        return false;
                    }
                    self.returns.set_i32(0, PLAYER_NONE as i32);
                } else if (hand0 == 1 && hand1 == 2) || (hand0 == 2 && hand1 == 3) || (hand0 == 3 && hand1 == 1) {
                    self.returns.set_i32(0, 1);
                } else {
                    self.returns.set_i32(0, 0);
                }
                true
            }
            _ => true,
        }
    }
}
