//! Operations (`operations.cpp` part 1: chain changes and the functions that queue units).

use crate::card::public_order;
use crate::constants::*;
use crate::duel::Duel;
use crate::types::*;
use crate::units::{self, LocationReason};

impl Duel {
    fn chain_index(&self, chaincount: u8) -> Option<usize> {
        let n = self.core.current_chain.len();
        if n == 0 {
            return None;
        }
        let cc = if chaincount as usize > n || chaincount < 1 { n } else { chaincount as usize };
        Some(cc - 1)
    }
    pub fn negate_chain(&mut self, chaincount: u8) -> bool {
        let Some(i) = self.chain_index(chaincount) else { return false };
        let chaincount = (i + 1) as u8;
        let ch = self.core.current_chain[i].clone();
        let te = ch.triggering_effect.unwrap();
        let h = te.get_handler(self);
        let re = self.core.reason_effect;
        if ch.flag & CHAIN_DISABLE_ACTIVATE == 0 && self.is_chain_negatable(ch.chain_count) && h.is_affect_by_effect(self, re) {
            let rp = self.core.reason_player;
            {
                let c = &mut self.core.current_chain[i];
                c.flag |= CHAIN_DISABLE_ACTIVATE;
                c.disable_reason = re;
                c.disable_player = rp;
            }
            if self.effect(te).type_ as u32 & EFFECT_TYPE_ACTIVATE != 0
                && h.is_has_relation_chain(self, te, ch.chain_id)
                && self.card(h).current.location as u32 == LOCATION_SZONE
            {
                self.card_mut(h).set_status(STATUS_LEAVE_CONFIRMED, true);
                self.card_mut(h).set_status(STATUS_ACTIVATE_DISABLED, true);
            }
            let m = self.new_message(MSG_CHAIN_NEGATED);
            self.msg(m).u8(chaincount);
            if !self.is_flag(DUEL_RETURN_TO_DECK_TRIGGERS)
                && (ch.triggering_location as u32 == LOCATION_DECK
                    || (ch.triggering_location as u32 == LOCATION_EXTRA && ch.triggering_position as u32 & POS_FACEDOWN != 0))
            {
                h.release_relation_chain(self, te, ch.chain_id);
            }
            return true;
        }
        false
    }
    pub fn disable_chain(&mut self, chaincount: u8) -> bool {
        let Some(i) = self.chain_index(chaincount) else { return false };
        let chaincount = (i + 1) as u8;
        let ch = self.core.current_chain[i].clone();
        let te = ch.triggering_effect.unwrap();
        let h = te.get_handler(self);
        let re = self.core.reason_effect;
        if ch.flag & CHAIN_DISABLE_EFFECT == 0 && self.is_chain_disablable(ch.chain_count) && h.is_affect_by_effect(self, re) {
            let rp = self.core.reason_player;
            {
                let c = &mut self.core.current_chain[i];
                c.flag |= CHAIN_DISABLE_EFFECT;
                c.disable_reason = re;
                c.disable_player = rp;
            }
            let m = self.new_message(MSG_CHAIN_DISABLED);
            self.msg(m).u8(chaincount);
            if !self.is_flag(DUEL_RETURN_TO_DECK_TRIGGERS)
                && (ch.triggering_location as u32 == LOCATION_DECK
                    || (ch.triggering_location as u32 == LOCATION_EXTRA && ch.triggering_position as u32 & POS_FACEDOWN != 0))
            {
                h.release_relation_chain(self, te, ch.chain_id);
            }
            return true;
        }
        false
    }
    pub fn change_chain_effect(&mut self, chaincount: u8, rep_op: Option<LuaFn>) {
        let Some(i) = self.chain_index(chaincount) else { return };
        self.core.current_chain[i].replace_op = rep_op;
        let te = self.core.current_chain[i].triggering_effect.unwrap();
        let h = self.effect(te).handler.unwrap();
        if self.effect(te).type_ as u32 & EFFECT_TYPE_ACTIVATE != 0 && self.card(h).current.location as u32 == LOCATION_SZONE {
            self.card_mut(h).set_status(STATUS_LEAVE_CONFIRMED, true);
        }
    }
    pub fn change_target(&mut self, chaincount: u8, targets: &CardSet) {
        let Some(i) = self.chain_index(chaincount) else { return };
        let ch = self.core.current_chain[i].clone();
        let Some(ot) = ch.target_cards.clone() else { return };
        let te = ch.triggering_effect.unwrap();
        for c in public_order(self, &ot) {
            c.release_relation_chain(self, te, ch.chain_id);
        }
        self.core.current_chain[i].target_cards = Some(targets.clone());
        for c in public_order(self, targets) {
            c.create_relation_chain(self, te, ch.chain_id);
        }
        if self.effect(te).is_flag(EFFECT_FLAG_CARD_TARGET) {
            let ordered = public_order(self, targets);
            let locs: Vec<LocInfo> = ordered.iter().map(|&c| c.get_info_location(self)).collect();
            let m = self.new_message(MSG_BECOME_TARGET);
            self.msg(m).u32(locs.len());
            for l in locs {
                self.msg(m).loc(l);
            }
        }
    }
    pub fn change_target_player(&mut self, chaincount: u8, playerid: u8) {
        if let Some(i) = self.chain_index(chaincount) {
            self.core.current_chain[i].target_player = playerid;
        }
    }
    pub fn change_target_param(&mut self, chaincount: u8, param: i32) {
        if let Some(i) = self.chain_index(chaincount) {
            self.core.current_chain[i].target_param = param;
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn remove_counter_op(&mut self, reason: u32, pcard: Option<CardId>, rplayer: u32, self_: u8, oppo: u8, countertype: u32, count: u32) {
        self.emplace_process(units::RemoveCounter {
            step: 0,
            rplayer: rplayer as u8,
            self_,
            oppo,
            countertype: countertype as u16,
            count: count as u16,
            reason,
            pcard,
        });
    }
    #[allow(clippy::too_many_arguments)]
    pub fn remove_overlay_card(&mut self, reason: u32, pgroup: Option<&CardSet>, rplayer: u32, self_: u8, oppo: u8, min: u16, max: u16) {
        let g = pgroup.map(|g| self.new_group(g.clone()));
        self.emplace_process(units::RemoveOverlay {
            step: 0,
            min,
            max,
            replaced_amount: 0,
            has_used_overlay_remove_replace_effect: false,
            rplayer: rplayer as u8,
            self_,
            oppo,
            reason,
            pgroup: g,
        });
    }
    pub fn xyz_overlay(&mut self, target: CardId, materials: CardSet, send_materials_to_grave: bool) {
        let ng = self.new_group(materials);
        self.emplace_process(units::XyzOverlay { step: 0, send_materials_to_grave, target, materials: Some(ng) });
    }
    #[allow(clippy::too_many_arguments)]
    pub fn get_control(&mut self, targets: CardSet, reason_effect: Option<EffectId>, chose_player: u8, playerid: u8, reset_phase: u16, reset_count: u8, zone: u32) {
        let ng = self.new_group(targets);
        self.emplace_process(units::GetControl {
            step: 0,
            chose_player,
            playerid,
            reset_count,
            reset_phase,
            zone,
            reason_effect,
            targets: Some(ng),
            destroy_set: CardSet::new(),
            iter_order: Vec::new(),
            iter_pos: 0,
        });
    }
    #[allow(clippy::too_many_arguments)]
    pub fn swap_control(&mut self, reason_effect: Option<EffectId>, reason_player: u32, targets1: CardSet, targets2: CardSet, reset_phase: u32, reset_count: u32) {
        let ng1 = self.new_group(targets1);
        let ng2 = self.new_group(targets2);
        self.emplace_process(units::SwapControl {
            step: 0,
            reset_count: reset_count as u8,
            reason_player: reason_player as u8,
            self_selected_sequence: 0,
            reset_phase: reset_phase as u16,
            reason_effect,
            targets1: Some(ng1),
            targets2: Some(ng2),
            iter1: Vec::new(),
            pos1: 0,
            iter2: Vec::new(),
            pos2: 0,
        });
    }
    pub fn equip_op(&mut self, equip_player: u8, equip_card: CardId, target: CardId, faceup: bool, is_step: bool) {
        self.emplace_process(units::Equip { step: 0, equip_player, is_step, faceup, equip_card, target });
    }
    pub fn draw(&mut self, reason_effect: Option<EffectId>, reason: u32, reason_player: u8, playerid: u8, count: u16) {
        self.emplace_process(units::Draw { step: 0, count, reason_player, playerid, reason, reason_effect, drawn_set: CardSet::new() });
    }
    #[allow(clippy::too_many_arguments)]
    pub fn damage(&mut self, mut reason_effect: Option<EffectId>, reason: u32, reason_player: u8, mut reason_card: Option<CardId>, playerid: u8, amount: u32, is_step: bool) {
        if reason & REASON_BATTLE != 0 {
            reason_effect = None;
        } else {
            reason_card = None;
        }
        self.emplace_process(units::Damage { step: 0, reason_player, playerid, is_step, is_reflected: false, amount, reason, reason_card, reason_effect });
    }
    pub fn recover(&mut self, reason_effect: Option<EffectId>, reason: u32, reason_player: u32, playerid: u32, amount: u32, is_step: bool) {
        self.emplace_process(units::Recover { step: 0, reason_player: reason_player as u8, playerid: playerid as u8, is_step, amount, reason, reason_effect });
    }
    pub fn summon(&mut self, sumplayer: u8, target: CardId, proc: Option<EffectId>, ignore_count: bool, min_tribute: u8, zone: u32) {
        self.emplace_process(units::SummonRule {
            step: 0,
            sumplayer,
            min_tribute,
            max_allowed_tributes: 0,
            ignore_count,
            zone,
            target,
            summon_procedure_effect: proc,
            extra_summon_effect: None,
            tributes: CardSet::new(),
            summon_cost_effects: Vec::new(),
        });
    }
    pub fn mset(&mut self, setplayer: u8, target: CardId, proc: Option<EffectId>, ignore_count: bool, min_tribute: u8, zone: u32) {
        self.emplace_process(units::MonsterSet {
            step: 0,
            setplayer,
            min_tribute,
            max_allowed_tributes: 0,
            ignore_count,
            zone,
            target,
            summon_procedure_effect: proc,
            extra_summon_effect: None,
            tributes: CardSet::new(),
        });
    }
    pub fn special_summon_rule(&mut self, sumplayer: u8, target: CardId, summon_type: u32) {
        self.emplace_process(new_spsummon_rule(0, sumplayer, target, summon_type, false, None));
    }
    pub fn special_summon_rule_group(&mut self, sumplayer: u8, summon_type: u32) {
        self.emplace_process(units::SpSummonRuleGroup { step: 0, sumplayer, summon_type });
    }
    fn force_spsummon_position(&mut self, eset: &[EffectId], pcard: CardId, sumtype: u32, mut pos: u32, playerid: u8) -> u32 {
        for &eff in eset {
            if let Some(t) = self.effect(eff).target.clone() {
                self.add_param(Value::Effect(eff));
                self.add_param(Value::Card(pcard));
                self.add_param(Value::Int(self.core.reason_player as i64));
                self.add_param(Value::Int(sumtype as i64));
                self.add_param(Value::Int(pos as i64));
                self.add_param(Value::Int(playerid as i64));
                self.add_param(Value::effect(self.core.reason_effect));
                if !self.check_condition(Some(&t), 7) {
                    continue;
                }
            }
            pos &= eff.get_value(self, 0) as u32;
        }
        pos
    }
    fn mark_spsummon(&mut self, pcard: CardId, sumtype: u32, sumplayer: u8) {
        let (re, rp) = (self.core.reason_effect, self.core.reason_player);
        let c = self.card_mut(pcard);
        c.temp.reason = c.current.reason;
        c.temp.reason_effect = c.current.reason_effect;
        c.temp.reason_player = c.current.reason_player;
        c.summon.type_ = (sumtype & 0xf00ffff) | SUMMON_TYPE_SPECIAL;
        c.summon.location = c.current.location;
        c.summon.sequence = c.current.sequence as u8;
        c.summon.pzone = c.current.pzone;
        c.summon.player = sumplayer;
        c.current.reason = REASON_SPSUMMON;
        c.current.reason_effect = re;
        c.current.reason_player = rp;
    }
    #[allow(clippy::too_many_arguments)]
    pub fn special_summon(&mut self, target: CardSet, sumtype: u32, sumplayer: u8, playerid: u8, nocheck: bool, nolimit: bool, mut positions: u8, zone: u32) {
        if positions as u32 & POS_FACEDOWN != 0 && self.is_player_affected_by_effect(sumplayer, EFFECT_DEVINE_LIGHT).is_some() {
            positions = (positions & POS_FACEUP as u8) | ((positions & POS_FACEDOWN as u8) >> 1);
        }
        let mut eset = Vec::new();
        self.filter_player_effect(sumplayer, EFFECT_FORCE_SPSUMMON_POSITION, &mut eset, true);
        for c in target.iter() {
            self.mark_spsummon(c, sumtype, sumplayer);
            let st = self.card(c).summon.type_;
            let pos = self.force_spsummon_position(&eset, c, st, positions as u32, playerid);
            self.card_mut(c).spsummon_param = ((playerid as u32) << 24) + ((nocheck as u32) << 16) + ((nolimit as u32) << 8) + pos;
        }
        let g = self.new_group(target);
        let (re, rp) = (self.core.reason_effect, self.core.reason_player);
        self.emplace_process(units::SpSummon { step: 0, reason_player: rp, zone, reason_effect: re, targets: Some(g) });
    }
    #[allow(clippy::too_many_arguments)]
    pub fn special_summon_step(&mut self, target: CardId, sumtype: u32, sumplayer: u8, playerid: u8, nocheck: bool, nolimit: bool, mut positions: u8, zone: u32) {
        if positions as u32 & POS_FACEDOWN != 0 && self.is_player_affected_by_effect(sumplayer, EFFECT_DEVINE_LIGHT).is_some() {
            positions = (positions & POS_FACEUP as u8) | ((positions & POS_FACEDOWN as u8) >> 1);
        }
        if positions as u32 & POS_FACEUP != 0 && self.check_unique_onfield(target, playerid, LOCATION_MZONE as u8, None).is_some() {
            positions &= !(POS_FACEUP as u8);
        }
        let mut eset = Vec::new();
        self.filter_player_effect(sumplayer, EFFECT_FORCE_SPSUMMON_POSITION, &mut eset, true);
        self.mark_spsummon(target, sumtype, sumplayer);
        let st = (sumtype & 0xf00ffff) | SUMMON_TYPE_SPECIAL;
        let pos = self.force_spsummon_position(&eset, target, st, positions as u32, playerid);
        self.card_mut(target).spsummon_param = ((playerid as u32) << 24) + ((nocheck as u32) << 16) + ((nolimit as u32) << 8) + pos;
        self.emplace_process(units::SpSummonStep { step: 0, zone, targets: None, target, spsummon_cost_effects: Vec::new() });
    }
    pub fn special_summon_complete(&mut self, reason_effect: Option<EffectId>, reason_player: u8) {
        let set = std::mem::take(&mut self.core.special_summoning);
        let g = self.new_group(set);
        self.emplace_process(units::SpSummon { step: 1, reason_player, zone: 0, reason_effect, targets: Some(g) });
    }
    #[allow(clippy::too_many_arguments)]
    pub fn destroy(&mut self, mut targets: CardSet, reason_effect: Option<EffectId>, reason: u32, reason_player: u8, playerid: u8, mut destination: u16, sequence: u32) {
        for c in targets.to_vec() {
            if self.card(c).is_status(STATUS_DESTROY_CONFIRMED) && !self.core.destroy_canceled.contains(c) {
                targets.remove(c);
                continue;
            }
            {
                let cc = self.card_mut(c);
                cc.temp.reason = cc.current.reason;
                cc.current.reason = reason;
                if reason_player as u32 != PLAYER_SELFDES {
                    cc.temp.reason_effect = cc.current.reason_effect;
                    cc.temp.reason_player = cc.current.reason_player;
                    if reason_effect.is_some() {
                        cc.current.reason_effect = reason_effect;
                    }
                    cc.current.reason_player = reason_player;
                }
            }
            let mut p = playerid as u32;
            if destination as u32 & (LOCATION_HAND | LOCATION_DECK | LOCATION_REMOVED) == 0 {
                destination = LOCATION_GRAVE as u16;
            }
            let owner = self.card(c).owner as u32;
            if destination != 0 && p == PLAYER_NONE {
                p = owner;
            }
            if destination as u32 & (LOCATION_GRAVE | LOCATION_REMOVED) != 0 {
                p = owner;
            }
            let cc = self.card_mut(c);
            cc.set_status(STATUS_DESTROY_CONFIRMED, true);
            cc.sendto_param.set(p as u8, POS_FACEUP as u8, destination as u8, sequence as u8);
        }
        let g = self.new_group(targets);
        self.emplace_process(units::Destroy { step: 0, reason_player, reason, targets: Some(g), reason_effect });
    }
    pub fn release(&mut self, targets: CardSet, reason_effect: Option<EffectId>, reason: u32, reason_player: u8) {
        for c in targets.iter() {
            let cc = self.card_mut(c);
            cc.temp.reason = cc.current.reason;
            cc.temp.reason_effect = cc.current.reason_effect;
            cc.temp.reason_player = cc.current.reason_player;
            cc.current.reason = reason;
            cc.current.reason_effect = reason_effect;
            cc.current.reason_player = reason_player;
            let owner = cc.owner;
            cc.sendto_param.set(owner, POS_FACEUP as u8, LOCATION_GRAVE as u8, 0);
        }
        let g = self.new_group(targets);
        self.emplace_process(units::Release { step: 0, reason_player, reason, targets: Some(g), reason_effect });
    }
    /// set current.reason, sendto_param; send-to in scripts: here->PROCESSOR_SENDTO, step 0
    #[allow(clippy::too_many_arguments)]
    pub fn send_to(&mut self, targets: CardSet, reason_effect: Option<EffectId>, reason: u32, reason_player: u8, playerid: u8, destination: u16, sequence: u32, position: u8, ignore: bool) {
        if destination as u32 & LOCATION_ONFIELD != 0 {
            return;
        }
        for c in targets.iter() {
            let cc = self.card_mut(c);
            cc.temp.reason = cc.current.reason;
            cc.temp.reason_effect = cc.current.reason_effect;
            cc.temp.reason_player = cc.current.reason_player;
            cc.current.reason = reason;
            cc.current.reason_effect = reason_effect;
            cc.current.reason_player = reason_player;
            let mut p = playerid as u32;
            if p == PLAYER_NONE && destination as u32 & LOCATION_HAND != 0 && cc.current.location as u32 & LOCATION_DECK != 0 && cc.current.controler == reason_player {
                p = reason_player as u32;
            }
            if p == PLAYER_NONE {
                p = cc.owner as u32;
            }
            if destination as u32 == LOCATION_GRAVE && cc.current.location as u32 == LOCATION_REMOVED {
                cc.current.reason |= REASON_RETURN;
            }
            let mut pos = position;
            if destination as u32 != LOCATION_REMOVED && !ignore {
                pos = POS_FACEUP as u8;
            } else if position == 0 {
                pos = cc.current.position as u8;
            }
            cc.sendto_param.set(p as u8, pos, destination as u8, sequence as u8);
        }
        let g = self.new_group(targets);
        self.emplace_process(units::SendTo { step: 0, reason_player, reason, targets: Some(g), reason_effect, extra_args: None });
    }
    #[allow(clippy::too_many_arguments)]
    pub fn move_to_field(
        &mut self,
        target: CardId,
        move_player: u8,
        playerid: u8,
        mut destination: u16,
        positions: u8,
        enable: bool,
        ret: u8,
        mut zone: u8,
        rule: bool,
        reason: LocationReason,
        confirm: bool,
    ) {
        let d = destination as u32;
        if d & (LOCATION_MZONE | LOCATION_MMZONE | LOCATION_EMZONE | LOCATION_SZONE | LOCATION_STZONE | LOCATION_PZONE | LOCATION_FZONE) == 0 || positions == 0 {
            return;
        }
        let cur = self.card(target).current;
        if d & LOCATION_PZONE != 0 && cur.is_location(LOCATION_PZONE) && playerid == cur.controler {
            return;
        }
        if d == cur.location as u32 && playerid == cur.controler {
            return;
        }
        let mut pzone = false;
        match d {
            LOCATION_PZONE => {
                destination = LOCATION_SZONE as u16;
                pzone = true;
            }
            LOCATION_FZONE => {
                destination = LOCATION_SZONE as u16;
                zone = 1 << 5;
            }
            LOCATION_MMZONE => {
                destination = LOCATION_MZONE as u16;
                zone = 0x1f;
            }
            LOCATION_STZONE => {
                destination = LOCATION_SZONE as u16;
                zone = 0x1f;
            }
            LOCATION_EMZONE => {
                destination = LOCATION_MZONE as u16;
                zone = (1 << 5) | (1 << 6);
            }
            _ => {}
        }
        self.card_mut(target).to_field_param = ((move_player as u32) << 24) + ((playerid as u32) << 16) + (((destination as u32) & 0xff) << 8) + positions as u32;
        self.emplace_process(units::MoveToField { step: 0, enable, ret, pzone, zone, rule, location_reason: reason, confirm, target });
    }
    #[allow(clippy::too_many_arguments)]
    pub fn change_position(&mut self, targets: CardSet, reason_effect: Option<EffectId>, reason_player: u8, au: u8, ad: u8, du: u8, dd: u8, flag: u32, enable: bool) {
        let g = self.new_group(targets);
        let set = self.group(g).clone();
        for c in public_order(self, &set) {
            let cc = self.card_mut(c);
            cc.position_param = match cc.current.position {
                POS_FACEUP_ATTACK => au,
                POS_FACEDOWN_DEFENSE => dd,
                POS_FACEUP_DEFENSE => du,
                _ => ad,
            } as u32;
            cc.position_param |= flag;
        }
        self.emplace_process(units::ChangePos { step: 0, reason_player, enable, oppo_selection: false, reason_effect, targets: Some(g), to_grave_set: CardSet::new() });
    }
    pub fn change_position_one(&mut self, target: CardId, reason_effect: Option<EffectId>, reason_player: u8, npos: u8, flag: u32, enable: bool) {
        let g = self.new_group(CardSet::from_card(target));
        let c = self.card_mut(target);
        c.position_param = npos as u32;
        c.position_param |= flag;
        self.emplace_process(units::ChangePos { step: 0, reason_player, enable, oppo_selection: false, reason_effect, targets: Some(g), to_grave_set: CardSet::new() });
    }
    pub fn operation_replace(&mut self, typ: u32, step: u16, targets: Option<GroupRef>) {
        let is_destroy = typ == EFFECT_DESTROY_REPLACE;
        let mut opp_effects = Vec::new();
        for re in self.field_effects.continuous_effect.range(typ) {
            if re.get_handler_player(self) == self.infos.turn_player {
                self.emplace_process(units::OperationReplace { step, is_destroy, replace_effect: Some(re), targets, target: None });
            } else {
                opp_effects.push(re);
            }
        }
        for pe in opp_effects {
            self.emplace_process(units::OperationReplace { step, is_destroy, replace_effect: Some(pe), targets, target: None });
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn select_tribute_cards(&mut self, target: Option<CardId>, playerid: u8, cancelable: bool, min: u16, max: u16, toplayer: u8, zone: u32) {
        self.emplace_process(units::SelectTribute {
            step: 0,
            min,
            max,
            playerid,
            cancelable,
            toplayer,
            zone: zone as u8,
            target,
            extra_release_effect: None,
            must_choose_one: CardSet::new(),
        });
    }
}

/// `Processors::SpSummonRule(step, sumplayer, target, summon_type, is_mid_chain, summon_proc_effect)`
pub fn new_spsummon_rule(step: u16, sumplayer: u8, target: CardId, summon_type: u32, is_mid_chain: bool, summon_proc_effect: Option<EffectId>) -> units::SpSummonRule {
    units::SpSummonRule {
        step,
        sumplayer,
        is_mid_chain,
        summon_type,
        target,
        summon_proc_effect,
        cards_to_summon_g: None,
        spsummon_cost_effects: Vec::new(),
        iter_order: Vec::new(),
        iter_pos: 0,
    }
}
