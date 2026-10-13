//! Effects (`effect.h`, `effect.cpp`).

use crate::constants::*;
use crate::duel::{Chain, Duel, Event};
use crate::types::*;
use crate::units::LocationReason;

/// `effect`
#[derive(Clone, Debug)]
pub struct Effect {
    /// Deleted (`duel::delete_effect`): the C++ frees the effect but may still read it through a
    /// stale pointer (e.g. a card's `reason_effect`), which reads the old data; so the data stays.
    pub deleted: bool,
    pub count_limit: u8,
    pub count_limit_max: u8,
    pub count_flag: u8,
    pub count_hopt_index: u8,
    pub effect_owner: u8,
    pub type_: u16,
    pub copy_id: u16,
    pub range: u16,
    pub s_range: u16,
    pub o_range: u16,
    pub reset_count: u16,
    pub active_location: u16,
    pub active_sequence: u16,
    pub status: u16,
    pub code: u32,
    pub flag: [u32; 2],
    pub id: u32,
    pub initial_id: u32,
    pub reset_flag: u32,
    pub count_code: u32,
    pub hint_timing: [u32; 2],
    pub card_type: u32,
    pub active_type: u32,
    pub label_object: Value,
    pub condition: Option<LuaFn>,
    pub cost: Option<LuaFn>,
    pub target: Option<LuaFn>,
    /// The value when it is a number (`EFFECT_FLAG_FUNC_VALUE` not set).
    pub value: i32,
    /// The value when it is a function (`EFFECT_FLAG_FUNC_VALUE` set).
    pub value_fn: Option<LuaFn>,
    pub operation: Option<LuaFn>,
    pub category: u64,
    pub owner: Option<CardId>,
    pub handler: Option<CardId>,
    pub active_handler: Option<CardId>,
    pub description: u64,
    pub label: Vec<i64>,
    /// Data the scripts attach to this effect object (not part of the C++ effect, e.g. the
    /// card properties `chain.lua` records when the effect is registered). Not copied by `Clone`.
    pub script_data: Option<std::sync::Arc<[Value]>>,
}

impl Effect {
    pub fn new() -> Self {
        Effect {
            deleted: false,
            count_limit: 0,
            count_limit_max: 0,
            count_flag: 0,
            count_hopt_index: 0,
            effect_owner: PLAYER_NONE as u8,
            type_: 0,
            copy_id: 0,
            range: 0,
            s_range: 0,
            o_range: 0,
            reset_count: 0,
            active_location: 0,
            active_sequence: 0,
            status: 0,
            code: 0,
            flag: [0, 0],
            id: 0,
            initial_id: 0,
            reset_flag: 0,
            count_code: 0,
            hint_timing: [0, 0],
            card_type: 0,
            active_type: 0,
            label_object: Value::Nil,
            condition: None,
            cost: None,
            target: None,
            value: 0,
            value_fn: None,
            operation: None,
            category: 0,
            owner: None,
            handler: None,
            active_handler: None,
            description: 0,
            label: Vec::new(),
            script_data: None,
        }
    }
    #[inline]
    pub fn is_flag(&self, f: u32) -> bool {
        self.flag[0] & f != 0
    }
    #[inline]
    pub fn is_flag2(&self, f: u32) -> bool {
        self.flag[1] & f != 0
    }
    #[inline]
    pub fn has_count_limit(&self) -> bool {
        self.is_flag(EFFECT_FLAG_COUNT_LIMIT)
    }
    #[inline]
    pub fn has_function_value(&self) -> bool {
        self.is_flag(EFFECT_FLAG_FUNC_VALUE)
    }
    /// Whether the value is "set" (C++ `if(value)`: a function, or a nonzero number).
    #[inline]
    pub fn has_value(&self) -> bool {
        if self.has_function_value() { self.value_fn.is_some() } else { self.value != 0 }
    }
}

impl Default for Effect {
    fn default() -> Self {
        Effect::new()
    }
}

/// `effect_sort_id`
pub fn effect_sort_id(d: &Duel, e1: EffectId, e2: EffectId) -> bool {
    d.effect(e1).id < d.effect(e2).id
}

pub fn sort_effects_by_id(d: &Duel, v: &mut [EffectId]) {
    if v.len() < 2 {
        return;
    }
    // (a stable sort; the ids looked up once: small sets by insertion, larger ones keyed by
    // (id, index) so that an unstable sort keeps the order of equal ids)
    if v.len() <= 16 {
        let mut ids = [0u32; 16];
        for (i, &e) in v.iter().enumerate() {
            ids[i] = d.effect(e).id;
        }
        for i in 1..v.len() {
            let (id, e) = (ids[i], v[i]);
            let mut j = i;
            while j > 0 && ids[j - 1] > id {
                ids[j] = ids[j - 1];
                v[j] = v[j - 1];
                j -= 1;
            }
            ids[j] = id;
            v[j] = e;
        }
        return;
    }
    let mut keyed: Vec<(u32, u32, EffectId)> = v.iter().enumerate().map(|(i, &e)| (d.effect(e).id, i as u32, e)).collect();
    if keyed.windows(2).all(|w| w[0].0 <= w[1].0) {
        return;
    }
    keyed.sort_unstable_by_key(|k| (k.0, k.1));
    for (x, k) in v.iter_mut().zip(keyed) {
        *x = k.2;
    }
}

impl EffectId {
    #[inline]
    fn fx(self, d: &Duel) -> &Effect {
        d.effect(self)
    }
    pub fn is_flag(self, d: &Duel, f: u32) -> bool {
        d.effect(self).is_flag(f)
    }
    pub fn is_disable_related(self, d: &Duel) -> bool {
        let code = d.effect(self).code;
        code == EFFECT_IMMUNE_EFFECT || code == EFFECT_DISABLE || code == EFFECT_CANNOT_DISABLE || code == EFFECT_FORBIDDEN
    }
    pub fn is_self_destroy_related(self, d: &Duel) -> bool {
        let code = d.effect(self).code;
        code == EFFECT_UNIQUE_CHECK || code == EFFECT_SELF_DESTROY || code == EFFECT_SELF_TOGRAVE
    }
    pub fn is_can_be_forbidden(self, d: &Duel) -> bool {
        let e = self.fx(d);
        let ctr = e.code & 0xf0000;
        if e.is_flag(EFFECT_FLAG_CANNOT_DISABLE) && !e.is_flag(EFFECT_FLAG_CANNOT_NEGATE) {
            return false;
        }
        if e.code == EFFECT_CHANGE_CODE || ctr == EFFECT_COUNTER_PERMIT || ctr == EFFECT_COUNTER_LIMIT {
            return false;
        }
        true
    }
    /// check if a single/field/equip effect is available
    pub fn is_available(self, d: &mut Duel) -> bool {
        let e = self.fx(d).clone_header();
        let typ = e.type_ as u32;
        if typ & EFFECT_TYPE_ACTIONS != 0 {
            return false;
        }
        if typ & (EFFECT_TYPE_SINGLE | EFFECT_TYPE_XMATERIAL) != 0 && typ & EFFECT_TYPE_FIELD == 0 {
            let phandler = self.get_handler(d);
            let powner = self.get_owner_opt(d); // nullptr for core-created effects (EFFECT_ATTACK_DISABLED)
            if d.card(phandler).current.controler == PLAYER_NONE as u8
                && e.code != EFFECT_ADD_SETCODE
                && e.code != EFFECT_ADD_CODE
                && e.code != EFFECT_REVIVE_LIMIT
            {
                return false;
            }
            if !self.is_in_range_of_symbolic_mzone(d, phandler) {
                return false;
            }
            if e.is_flag(EFFECT_FLAG_SINGLE_RANGE) && !self.in_range(d, phandler) {
                return false;
            }
            if e.is_flag(EFFECT_FLAG_SINGLE_RANGE)
                && d.card(phandler).get_status(STATUS_EFFECT_ENABLED) == 0
                && !e.is_flag(EFFECT_FLAG_IMMEDIATELY_APPLY)
            {
                return false;
            }
            if e.is_flag(EFFECT_FLAG_SINGLE_RANGE)
                && d.card(phandler).current.location as u32 & LOCATION_ONFIELD != 0
                && d.card(phandler).is_position(POS_FACEUP) == 0
            {
                return false;
            }
            if e.is_flag(EFFECT_FLAG_OWNER_RELATE) && self.is_can_be_forbidden(d) && d.card(powner.unwrap()).is_status(STATUS_FORBIDDEN) {
                return false;
            }
            if powner == Some(phandler) && self.is_can_be_forbidden(d) && d.card(phandler).get_status(STATUS_FORBIDDEN) != 0 {
                return false;
            }
            if e.is_flag(EFFECT_FLAG_OWNER_RELATE) && !e.is_flag(EFFECT_FLAG_CANNOT_DISABLE) && d.card(powner.unwrap()).is_status(STATUS_DISABLED) {
                return false;
            }
            if powner == Some(phandler)
                && !e.is_flag(EFFECT_FLAG_CANNOT_DISABLE)
                && e.code != EFFECT_DISABLE
                && d.card(phandler).get_status(STATUS_DISABLED) != 0
            {
                return false;
            }
        }
        if typ & EFFECT_TYPE_EQUIP != 0 {
            let handler = e.handler.unwrap();
            let owner = e.owner.unwrap();
            if d.card(handler).current.controler == PLAYER_NONE as u8 {
                return false;
            }
            if e.is_flag(EFFECT_FLAG_OWNER_RELATE) && self.is_can_be_forbidden(d) && d.card(owner).is_status(STATUS_FORBIDDEN) {
                return false;
            }
            if owner == handler && self.is_can_be_forbidden(d) && d.card(handler).get_status(STATUS_FORBIDDEN) != 0 {
                return false;
            }
            if e.is_flag(EFFECT_FLAG_OWNER_RELATE) && !e.is_flag(EFFECT_FLAG_CANNOT_DISABLE) && d.card(owner).is_status(STATUS_DISABLED) {
                return false;
            }
            if owner == handler && !e.is_flag(EFFECT_FLAG_CANNOT_DISABLE) && d.card(handler).get_status(STATUS_DISABLED) != 0 {
                return false;
            }
            if !e.is_flag(EFFECT_FLAG_SET_AVAILABLE) {
                if d.card(handler).get_status(STATUS_EFFECT_ENABLED) == 0 {
                    return false;
                }
                if d.card(handler).is_position(POS_FACEUP) == 0 {
                    return false;
                }
            }
        }
        if typ & (EFFECT_TYPE_FIELD | EFFECT_TYPE_TARGET) != 0 {
            let phandler = self.get_handler(d);
            let powner = self.get_owner(d);
            if !e.is_flag(EFFECT_FLAG_FIELD_ONLY) {
                let hc = d.card(phandler);
                if hc.current.controler == PLAYER_NONE as u8 {
                    return false;
                }
                if !self.is_in_range_of_symbolic_mzone(d, phandler) || !self.in_range(d, phandler) {
                    return false;
                }
                let hc = d.card(phandler);
                if hc.get_status(STATUS_EFFECT_ENABLED) == 0 && !e.is_flag(EFFECT_FLAG_IMMEDIATELY_APPLY) {
                    return false;
                }
                if hc.current.location as u32 & LOCATION_ONFIELD != 0 && hc.is_position(POS_FACEUP) == 0 {
                    return false;
                }
                if e.is_flag(EFFECT_FLAG_OWNER_RELATE) && self.is_can_be_forbidden(d) && d.card(powner).is_status(STATUS_FORBIDDEN) {
                    return false;
                }
                if powner == phandler && self.is_can_be_forbidden(d) && d.card(phandler).get_status(STATUS_FORBIDDEN) != 0 {
                    return false;
                }
                if e.is_flag(EFFECT_FLAG_OWNER_RELATE) && !e.is_flag(EFFECT_FLAG_CANNOT_DISABLE) && d.card(powner).is_status(STATUS_DISABLED) {
                    return false;
                }
                if powner == phandler && !e.is_flag(EFFECT_FLAG_CANNOT_DISABLE) && d.card(phandler).get_status(STATUS_DISABLED) != 0 {
                    return false;
                }
                if d.card(phandler).is_status(STATUS_BATTLE_DESTROYED) {
                    return false;
                }
            }
        }
        let Some(condition) = d.effect(self).condition.clone() else {
            return true;
        };
        let refreshing = d.core.refreshing_condition == Some(self);
        if refreshing {
            d.mute_errors = true;
            d.muted_error = false;
        }
        d.add_param(Value::Effect(self));
        let res = d.check_condition(Some(&condition), 1);
        if refreshing {
            d.mute_errors = false;
            if d.muted_error {
                return res;
            }
        }
        if d.core.refreshing_condition != Some(self) {
            return res;
        }
        // (written only on a change: a write copies an effect shared with another duel)
        let available = d.effect(self).status as u32 & EFFECT_STATUS_AVAILABLE != 0;
        if res && !available {
            let id = d.infos.field_id;
            d.infos.field_id += 1;
            let e = d.effect_mut(self);
            e.id = id;
            e.status |= EFFECT_STATUS_AVAILABLE as u16;
        } else if !res && available {
            d.effect_mut(self).status &= !(EFFECT_STATUS_AVAILABLE as u16);
        }
        res
    }
    /// reset_count: count of effect reset; count_limit: left count of activation
    pub fn check_count_limit(self, d: &mut Duel, playerid: u8) -> bool {
        if let Some(h) = self.fx(d).handler {
            d.note_read(h, 4096);
        }
        let e = self.fx(d);
        if e.is_flag(EFFECT_FLAG_COUNT_LIMIT) {
            if e.count_limit == 0 {
                return false;
            }
            if e.count_code != 0 || e.count_flag != 0 {
                let count = e.count_limit_max as u32;
                let (count_flag, count_hopt_index, count_code) = (e.count_flag, e.count_hopt_index, e.count_code);
                if count_flag as u32 & EFFECT_COUNT_CODE_SINGLE != 0 {
                    let fieldid = d.card(self.get_handler(d)).fieldid;
                    if d.get_effect_code(fieldid, count_flag, count_hopt_index, PLAYER_NONE as u8) >= count {
                        return false;
                    }
                } else if d.get_effect_code(count_code, count_flag, count_hopt_index, playerid) >= count {
                    return false;
                }
            }
        }
        true
    }
    /// check if an EFFECT_TYPE_ACTIONS effect can be activated
    #[allow(clippy::too_many_arguments)]
    pub fn is_activateable(
        self,
        d: &mut Duel,
        playerid: u8,
        e: &Event,
        neglect_cond: bool,
        neglect_cost: bool,
        neglect_target: bool,
        neglect_loc: bool,
        neglect_faceup: bool,
    ) -> bool {
        let fx = self.fx(d).clone_header();
        let typ = fx.type_ as u32;
        if typ & EFFECT_TYPE_ACTIONS == 0 {
            return false;
        }
        if !self.check_count_limit(d, playerid) {
            return false;
        }
        if !fx.is_flag(EFFECT_FLAG_FIELD_ONLY) {
            if typ & EFFECT_TYPE_ACTIVATE != 0 {
                let handler = fx.handler.unwrap();
                if d.card(handler).current.controler != playerid && fx.flag[0] & EFFECT_FLAG_BOTH_SIDE == 0 {
                    return false;
                }
                if d.check_unique_onfield(handler, playerid, LOCATION_SZONE as u8, None).is_some() {
                    return false;
                }
                let htype = d.card(handler).data.type_;
                if htype & TYPE_COUNTER == 0 {
                    if !(1132..=1149).contains(&fx.code)
                        && d.infos.phase as u32 == PHASE_DAMAGE
                        && !fx.is_flag(EFFECT_FLAG_DAMAGE_STEP)
                        && d.get_cteffect(self, playerid as i32, false) == 0
                    {
                        return false;
                    }
                    if !(1134..=1136).contains(&fx.code)
                        && d.infos.phase as u32 == PHASE_DAMAGE_CAL
                        && !fx.is_flag(EFFECT_FLAG_DAMAGE_CAL)
                        && d.get_cteffect(self, playerid as i32, false) == 0
                    {
                        return false;
                    }
                }
                let mut zone: u32 = 0xff;
                if htype & (TYPE_FIELD | TYPE_PENDULUM) == 0 && fx.is_flag(EFFECT_FLAG_LIMIT_ZONE) {
                    d.add_param(Value::Int(playerid as i64));
                    d.add_param(Value::from_group(&e.event_cards));
                    d.add_param(Value::Int(e.event_player as i64));
                    d.add_param(Value::Int(e.event_value as i64));
                    d.add_param(Value::effect(e.reason_effect));
                    d.add_param(Value::Int(e.reason as i64));
                    d.add_param(Value::Int(e.reason_player as i64));
                    zone = self.get_value(d, 7) as u32;
                    if zone == 0 {
                        return false;
                    }
                }
                // additional check for each location
                let value = fx.value;
                let limit_zone = fx.is_flag(EFFECT_FLAG_LIMIT_ZONE);
                if d.card(handler).current.location as u32 == LOCATION_SZONE {
                    if d.card(handler).is_position(POS_FACEUP) != 0 {
                        return false;
                    }
                    if d.card(handler).equiping_target.is_some() {
                        return false;
                    }
                    if htype & (TYPE_FIELD | TYPE_PENDULUM) == 0 && limit_zone && zone & (1u32 << d.card(handler).current.sequence) == 0 {
                        return false;
                    }
                } else if !((htype & TYPE_FIELD != 0 && (!limit_zone && value <= 0))
                    || (!limit_zone && (value as u32 & LOCATION_FZONE) != 0)
                    || (!limit_zone && (value as u32 & LOCATION_HAND) != 0))
                {
                    if !limit_zone && (value as u32 & LOCATION_MZONE) != 0 {
                        if d.get_useable_count(Some(handler), playerid, LOCATION_MZONE as u8, playerid, LocationReason::ToField, 0xff, None) <= 0 {
                            return false;
                        }
                    } else if htype & TYPE_PENDULUM != 0 || (!limit_zone && (value as u32 & LOCATION_PZONE) != 0) {
                        if !d.is_location_useable(playerid as u32, LOCATION_PZONE, 0) && !d.is_location_useable(playerid as u32, LOCATION_PZONE, 1) {
                            return false;
                        }
                    } else if d.get_useable_count(Some(handler), playerid, LOCATION_SZONE as u8, playerid, LocationReason::ToField, zone, None) <= 0 {
                        return false;
                    }
                }
                // check activate in hand/in set turn
                let mut ecode = 0u32;
                let hloc = d.card(handler).current.location as u32;
                if hloc == LOCATION_HAND && !neglect_loc {
                    if htype & TYPE_TRAP != 0 {
                        ecode = EFFECT_TRAP_ACT_IN_HAND;
                    } else if htype & TYPE_SPELL != 0 && d.infos.turn_player != playerid {
                        if htype & TYPE_QUICKPLAY != 0 || handler.is_affected_by_effect(d, EFFECT_BECOME_QUICK).is_some() {
                            ecode = EFFECT_QP_ACT_IN_NTPHAND;
                        } else {
                            return false;
                        }
                    }
                } else if hloc == LOCATION_SZONE {
                    if htype & TYPE_TRAP != 0 && d.card(handler).get_status(STATUS_SET_TURN) != 0 {
                        ecode = EFFECT_TRAP_ACT_IN_SET_TURN;
                    }
                    if htype & TYPE_SPELL != 0
                        && (htype & TYPE_QUICKPLAY != 0 || handler.is_affected_by_effect(d, EFFECT_BECOME_QUICK).is_some())
                        && d.card(handler).get_status(STATUS_SET_TURN) != 0
                    {
                        ecode = EFFECT_QP_ACT_IN_SET_TURN;
                    }
                }
                if ecode != 0 {
                    let mut available = false;
                    let mut eset = Vec::new();
                    handler.filter_effect(d, ecode, &mut eset, true);
                    for peff in eset {
                        if peff.check_count_limit(d, playerid) {
                            available = true;
                            break;
                        }
                    }
                    if !available {
                        return false;
                    }
                }
                if d.card(handler).is_status(STATUS_FORBIDDEN) {
                    return false;
                }
                if handler.is_affected_by_effect(d, EFFECT_CANNOT_TRIGGER).is_some() {
                    return false;
                }
            } else if typ & EFFECT_TYPE_CONTINUOUS == 0 {
                let phandler = self.get_handler(d);
                let ptype = phandler.get_type(d, None, 0, 2);
                if !d.is_flag(DUEL_TRIGGER_WHEN_PRIVATE_KNOWLEDGE) && ptype & TYPE_MONSTER == 0 && self.get_active_type(d) & TYPE_MONSTER != 0 {
                    return false;
                }
                let ptype = phandler.get_type(d, None, 0, 2);
                if !fx.is_flag2(EFFECT_FLAG2_CONTINUOUS_EQUIP) && ptype & TYPE_CONTINUOUS != 0 && ptype & TYPE_EQUIP != 0 {
                    return false;
                }
                let pc = d.card(phandler);
                if !neglect_faceup && pc.current.location as u32 & (LOCATION_ONFIELD | LOCATION_REMOVED) != 0 {
                    if pc.is_position(POS_FACEUP) == 0 && !fx.is_flag(EFFECT_FLAG_SET_AVAILABLE) {
                        return false;
                    }
                    if pc.is_position(POS_FACEUP) != 0 && !pc.is_status(STATUS_EFFECT_ENABLED) {
                        return false;
                    }
                }
                if typ & (EFFECT_TYPE_FLIP | EFFECT_TYPE_TRIGGER_F) == 0 && !(typ & EFFECT_TYPE_TRIGGER_O != 0 && typ & EFFECT_TYPE_SINGLE != 0) {
                    if !(1132..=1149).contains(&fx.code) && d.infos.phase as u32 == PHASE_DAMAGE && !fx.is_flag(EFFECT_FLAG_DAMAGE_STEP) {
                        return false;
                    }
                    if !(1134..=1136).contains(&fx.code) && d.infos.phase as u32 == PHASE_DAMAGE_CAL && !fx.is_flag(EFFECT_FLAG_DAMAGE_CAL) {
                        return false;
                    }
                }
                let pc = d.card(phandler);
                if pc.current.location as u32 == LOCATION_OVERLAY {
                    return false;
                }
                if typ & EFFECT_TYPE_FIELD != 0 && pc.current.controler != playerid && !fx.is_flag(EFFECT_FLAG_BOTH_SIDE | EFFECT_FLAG_EVENT_PLAYER) {
                    return false;
                }
                if !d.is_flag(DUEL_TRIGGER_WHEN_PRIVATE_KNOWLEDGE)
                    && !d.is_flag(DUEL_RETURN_TO_DECK_TRIGGERS)
                    && (pc.current.location as u32 == LOCATION_DECK
                        || (pc.current.location as u32 == LOCATION_EXTRA && pc.current.position & POS_FACEDOWN != 0))
                {
                    if typ & EFFECT_TYPE_SINGLE != 0 && fx.code != EVENT_TO_DECK {
                        return false;
                    }
                    if typ & EFFECT_TYPE_FIELD != 0 && fx.range as u32 & (LOCATION_DECK | LOCATION_EXTRA) == 0 {
                        return false;
                    }
                }
                if d.card(phandler).is_status(STATUS_FORBIDDEN) {
                    return false;
                }
                if phandler.is_affected_by_effect(d, EFFECT_CANNOT_TRIGGER).is_some() {
                    return false;
                }
            } else {
                let phandler = self.get_handler(d);
                let owner = fx.owner.unwrap();
                let pc = d.card(phandler);
                if typ & EFFECT_TYPE_FIELD != 0 && pc.is_status(STATUS_BATTLE_DESTROYED) {
                    return false;
                }
                if (typ & EFFECT_TYPE_FIELD != 0 || (typ & EFFECT_TYPE_SINGLE != 0 && fx.is_flag(EFFECT_FLAG_SINGLE_RANGE)))
                    && pc.current.location as u32 & LOCATION_ONFIELD != 0
                    && (pc.is_position(POS_FACEUP) == 0 || !pc.is_status(STATUS_EFFECT_ENABLED))
                {
                    return false;
                }
                if typ & EFFECT_TYPE_SINGLE != 0 && fx.is_flag(EFFECT_FLAG_SINGLE_RANGE) && !self.in_range(d, phandler) {
                    return false;
                }
                if fx.is_flag(EFFECT_FLAG_OWNER_RELATE) && self.is_can_be_forbidden(d) && d.card(owner).is_status(STATUS_FORBIDDEN) {
                    return false;
                }
                if phandler == owner && self.is_can_be_forbidden(d) && d.card(phandler).is_status(STATUS_FORBIDDEN) {
                    return false;
                }
                if fx.is_flag(EFFECT_FLAG_OWNER_RELATE) && !fx.is_flag(EFFECT_FLAG_CANNOT_DISABLE) && d.card(owner).is_status(STATUS_DISABLED) {
                    return false;
                }
                if phandler == owner && !fx.is_flag(EFFECT_FLAG_CANNOT_DISABLE) && d.card(phandler).is_status(STATUS_DISABLED) {
                    return false;
                }
            }
        } else if self.get_owner_player(d) != playerid && !fx.is_flag(EFFECT_FLAG_BOTH_SIDE) {
            return false;
        }
        let oreason = d.core.reason_effect;
        let op = d.core.reason_player;
        d.core.reason_effect = Some(self);
        d.core.reason_player = playerid;
        let mut result = true;
        if typ & EFFECT_TYPE_CONTINUOUS == 0 {
            result = self.is_action_check(d, playerid);
        }
        if result {
            result = self.is_activate_ready(d, playerid, e, neglect_cond, neglect_cost, neglect_target);
            // (ext) forced: skip the cost/target checks only if they fail (the checks can have
            // side effects the activation relies on, e.g. labels)
            if !result && !(neglect_cost && neglect_target) && d.ext_is_forced_activatable(self, playerid) {
                result = self.is_activate_ready(d, playerid, e, neglect_cond, true, true);
                if result && typ & EFFECT_TYPE_TRIGGER_F != 0 {
                    d.ext_note_forced_used(self); // (a mandatory effect only the forcing activates)
                }
            }
        }
        d.core.reason_effect = oreason;
        d.core.reason_player = op;
        result
    }
    /// check functions: value of EFFECT_CANNOT_ACTIVATE, target, cost of EFFECT_ACTIVATE_COST
    pub fn is_action_check(self, d: &mut Duel, playerid: u8) -> bool {
        let mut eset = Vec::new();
        d.filter_player_effect(playerid, EFFECT_CANNOT_ACTIVATE, &mut eset, true);
        for peff in eset {
            d.add_param(Value::Effect(self));
            d.add_param(Value::Int(playerid as i64));
            if peff.check_value_condition(d, 2) {
                return false;
            }
        }
        let mut eset = Vec::new();
        d.filter_player_effect(playerid, EFFECT_ACTIVATE_COST, &mut eset, true);
        for peff in eset {
            let target = d.effect(peff).target.clone();
            d.add_param(Value::Effect(peff));
            d.add_param(Value::Effect(self));
            d.add_param(Value::Int(playerid as i64));
            if !d.check_condition(target.as_ref(), 3) {
                continue;
            }
            let cost = d.effect(peff).cost.clone();
            d.add_param(Value::Effect(peff));
            d.add_param(Value::Effect(self));
            d.add_param(Value::Int(playerid as i64));
            if !d.check_condition(cost.as_ref(), 3) {
                return false;
            }
        }
        true
    }
    fn push_event_params(d: &mut Duel, reason_effect: EffectId, playerid: u8, e: &Event) {
        d.add_param(Value::Effect(reason_effect));
        d.add_param(Value::Int(playerid as i64));
        d.add_param(Value::from_group(&e.event_cards));
        d.add_param(Value::Int(e.event_player as i64));
        d.add_param(Value::Int(e.event_value as i64));
        d.add_param(Value::effect(e.reason_effect));
        d.add_param(Value::Int(e.reason as i64));
        d.add_param(Value::Int(e.reason_player as i64));
    }
    /// check functions: condition, cost(chk=0), target(chk=0)
    pub fn is_activate_ready_by(
        self,
        d: &mut Duel,
        reason_effect: EffectId,
        playerid: u8,
        e: &Event,
        neglect_cond: bool,
        neglect_cost: bool,
        neglect_target: bool,
    ) -> bool {
        let rhandler = reason_effect.get_handler(d);
        if !self.is_in_range_of_symbolic_mzone(d, rhandler) {
            return false;
        }
        let (condition, cost, target, typ) = {
            let fx = d.effect(self);
            (fx.condition.clone(), fx.cost.clone(), fx.target.clone(), fx.type_ as u32)
        };
        if !neglect_cond {
            if let Some(condition) = condition {
                Self::push_event_params(d, reason_effect, playerid, e);
                if !d.check_condition(Some(&condition), 8) {
                    return false;
                }
            }
        }
        if !neglect_cost && typ & EFFECT_TYPE_CONTINUOUS == 0 {
            if let Some(cost) = cost {
                Self::push_event_params(d, reason_effect, playerid, e);
                d.add_param(Value::Int(0));
                if !d.check_condition(Some(&cost), 9) {
                    return false;
                }
            }
        }
        if !neglect_target {
            if let Some(target) = target {
                Self::push_event_params(d, reason_effect, playerid, e);
                d.add_param(Value::Int(0));
                if !d.check_condition(Some(&target), 9) {
                    return false;
                }
            }
        }
        true
    }
    pub fn is_activate_ready(self, d: &mut Duel, playerid: u8, e: &Event, neglect_cond: bool, neglect_cost: bool, neglect_target: bool) -> bool {
        self.is_activate_ready_by(d, self, playerid, e, neglect_cond, neglect_cost, neglect_target)
    }
    pub fn is_condition_check(self, d: &mut Duel, playerid: u8, e: &Event) -> bool {
        let phandler = self.get_handler(d);
        let typ = d.effect(self).type_ as u32;
        if typ & EFFECT_TYPE_ACTIVATE == 0
            && d.card(phandler).current.location as u32 & (LOCATION_ONFIELD | LOCATION_REMOVED) != 0
            && d.card(phandler).is_position(POS_FACEUP) == 0
        {
            return false;
        }
        if !self.is_in_range_of_symbolic_mzone(d, phandler) {
            return false;
        }
        let Some(condition) = d.effect(self).condition.clone() else {
            return true;
        };
        let oreason = d.core.reason_effect;
        let op = d.core.reason_player;
        d.core.reason_effect = Some(self);
        d.core.reason_player = playerid;
        Self::push_event_params(d, self, playerid, e);
        let res = d.check_condition(Some(&condition), 8);
        d.core.reason_effect = oreason;
        d.core.reason_player = op;
        res
    }
    pub fn is_activate_check(self, d: &mut Duel, playerid: u8, e: &Event, neglect_cond: bool, neglect_cost: bool, neglect_target: bool) -> bool {
        let oreason = d.core.reason_effect;
        let op = d.core.reason_player;
        d.core.reason_effect = Some(self);
        d.core.reason_player = playerid;
        let result = self.is_activate_ready(d, playerid, e, neglect_cond, neglect_cost, neglect_target);
        d.core.reason_effect = oreason;
        d.core.reason_player = op;
        result
    }
    pub fn is_target(self, d: &mut Duel, pcard: CardId) -> bool {
        let fx = self.fx(d).clone_header();
        let typ = fx.type_ as u32;
        if typ & EFFECT_TYPE_ACTIONS != 0 {
            return false;
        }
        if typ & (EFFECT_TYPE_SINGLE | EFFECT_TYPE_EQUIP | EFFECT_TYPE_XMATERIAL) != 0 && typ & EFFECT_TYPE_FIELD == 0 {
            return true;
        }
        if typ & EFFECT_TYPE_TARGET != 0 && typ & EFFECT_TYPE_FIELD == 0 {
            return self.is_fit_target_function(d, pcard);
        }
        let pc = d.card(pcard);
        if !fx.is_flag(EFFECT_FLAG_SET_AVAILABLE) && pc.current.location as u32 & LOCATION_ONFIELD != 0 && pc.is_position(POS_FACEUP) == 0 {
            return false;
        }
        if !fx.is_flag(EFFECT_FLAG_IGNORE_RANGE) {
            if pc.get_status(STATUS_SUMMONING | STATUS_SUMMON_DISABLED | STATUS_ACTIVATE_DISABLED | STATUS_SPSUMMON_STEP) != 0 {
                return false;
            }
            if fx.is_flag(EFFECT_FLAG_SPSUM_PARAM) {
                return false;
            }
            let ctrl = pc.current.controler;
            let cur = pc.current.clone_loc();
            if fx.is_flag(EFFECT_FLAG_ABSOLUTE_TARGET) {
                if ctrl == 0 {
                    if !cur.is_location(fx.s_range as u32) {
                        return false;
                    }
                } else if !cur.is_location(fx.o_range as u32) {
                    return false;
                }
            } else if ctrl == self.get_handler_player(d) {
                if !cur.is_location(fx.s_range as u32) {
                    return false;
                }
            } else if !cur.is_location(fx.o_range as u32) {
                return false;
            }
        }
        self.is_fit_target_function(d, pcard)
    }
    pub fn is_fit_target_function(self, d: &mut Duel, pcard: CardId) -> bool {
        if let Some(target) = d.effect(self).target.clone() {
            d.add_param(Value::Effect(self));
            d.add_param(Value::Card(pcard));
            if !d.check_condition(Some(&target), 2) {
                return false;
            }
        }
        true
    }
    pub fn is_target_player(self, d: &Duel, playerid: u8) -> bool {
        let fx = self.fx(d);
        if !fx.is_flag(EFFECT_FLAG_PLAYER_TARGET) {
            return false;
        }
        let self_p = self.get_handler_player(d);
        if fx.is_flag(EFFECT_FLAG_ABSOLUTE_TARGET) {
            if fx.s_range != 0 && playerid == 0 {
                return true;
            }
            if fx.o_range != 0 && playerid == 1 {
                return true;
            }
        } else {
            if fx.s_range != 0 && self_p == playerid {
                return true;
            }
            if fx.o_range != 0 && self_p != playerid {
                return true;
            }
        }
        false
    }
    pub fn is_player_effect_target(self, d: &mut Duel, pcard: CardId) -> bool {
        self.is_fit_target_function(d, pcard)
    }
    pub fn is_immuned(self, d: &mut Duel, pcard: CardId) -> bool {
        let immune = d.card(pcard).immune_effect.clone();
        for peffect in immune {
            if peffect.is_available(d) && d.effect(peffect).has_value() {
                d.add_param(Value::Effect(self));
                d.add_param(Value::Card(pcard));
                if peffect.check_value_condition(d, 2) {
                    return true;
                }
            }
        }
        false
    }
    pub fn is_chainable(self, d: &mut Duel, tp: u8) -> bool {
        let fx = self.fx(d).clone_header();
        let typ = fx.type_ as u32;
        if typ & EFFECT_TYPE_ACTIONS == 0 {
            return false;
        }
        let sp = self.get_speed(d);
        // Curse of Field is the exception
        if typ & EFFECT_TYPE_ACTIVATE != 0 && sp <= 1 && !fx.is_flag2(EFFECT_FLAG2_COF) {
            return false;
        }
        if let Some(last) = d.core.current_chain.last() {
            let last_eff = last.triggering_effect.unwrap();
            if !fx.is_flag(EFFECT_FLAG_FIELD_ONLY)
                && typ & EFFECT_TYPE_TRIGGER_O != 0
                && d.card(self.get_handler(d)).current.location as u32 == LOCATION_HAND
            {
                if last_eff.get_speed(d) > 2 {
                    return false;
                }
            } else if sp < last_eff.get_speed(d) {
                return false;
            }
        }
        let limits = d.core.chain_limit.clone();
        for ch_lim in limits {
            d.add_param(Value::Effect(self));
            d.add_param(Value::Int(ch_lim.player as i64));
            d.add_param(Value::Int(tp as i64));
            if !d.check_condition(Some(&ch_lim.function), 3) {
                return false;
            }
        }
        let limits = d.core.chain_limit_p.clone();
        for ch_lim in limits {
            d.add_param(Value::Effect(self));
            d.add_param(Value::Int(ch_lim.player as i64));
            d.add_param(Value::Int(tp as i64));
            if !d.check_condition(Some(&ch_lim.function), 3) {
                return false;
            }
        }
        true
    }
    /// return: this can be reset by reset_level or not; RESET_DISABLE is valid only when owner == handler
    pub fn reset(self, d: &mut Duel, mut reset_level: u32, reset_type: u32) -> bool {
        match reset_type {
            RESET_EVENT => {
                let fx = self.fx(d);
                if fx.reset_flag & RESET_EVENT == 0 {
                    return false;
                }
                if fx.owner != fx.handler {
                    reset_level &= !RESET_DISABLE;
                }
                reset_level & 0xffff0000 & fx.reset_flag != 0
            }
            RESET_CARD => {
                let fx = self.fx(d);
                fx.owner.is_some_and(|o| d.card(o).data.code == reset_level)
            }
            RESET_PHASE => {
                if self.fx(d).reset_flag & RESET_PHASE == 0 {
                    return false;
                }
                let pid = self.get_handler_player(d);
                let tp = d.infos.turn_player;
                let fx = d.effect_mut(self);
                if ((fx.reset_flag & RESET_SELF_TURN != 0 && pid == tp) || (fx.reset_flag & RESET_OPPO_TURN != 0 && pid != tp))
                    && reset_level & 0x3ff & fx.reset_flag != 0
                {
                    fx.reset_count = fx.reset_count.wrapping_sub(1);
                }
                fx.reset_count == 0
            }
            RESET_CODE => {
                let fx = self.fx(d);
                fx.code == reset_level && fx.type_ as u32 & EFFECT_TYPE_SINGLE != 0 && fx.type_ as u32 & EFFECT_TYPE_ACTIONS == 0
            }
            RESET_COPY => self.fx(d).copy_id as u32 == reset_level,
            _ => false,
        }
    }
    pub fn dec_count(self, d: &mut Duel, playerid: u32) {
        let fx = self.fx(d);
        if !fx.is_flag(EFFECT_FLAG_COUNT_LIMIT) || fx.count_limit == 0 {
            return;
        }
        let (count_code, count_flag, hopt) = (fx.count_code, fx.count_flag, fx.count_hopt_index);
        if (count_code == 0 && count_flag == 0) || fx.is_flag(EFFECT_FLAG_NO_TURN_RESET) {
            d.effect_mut(self).count_limit -= 1;
        }
        if count_code != 0 || count_flag != 0 {
            if count_flag as u32 & EFFECT_COUNT_CODE_SINGLE != 0 {
                let fieldid = d.card(self.get_handler(d)).fieldid;
                d.add_effect_code(fieldid, count_flag, hopt, PLAYER_NONE as u8);
            } else {
                d.add_effect_code(count_code, count_flag, hopt, playerid as u8);
            }
        }
    }
    pub fn inc_count(self, d: &mut Duel, playerid: u32) {
        let fx = self.fx(d);
        if !fx.is_flag(EFFECT_FLAG_COUNT_LIMIT) || fx.count_limit == 0 {
            return;
        }
        let (count_code, count_flag, hopt) = (fx.count_code, fx.count_flag, fx.count_hopt_index);
        if ((count_code == 0 && count_flag == 0) || fx.is_flag(EFFECT_FLAG_NO_TURN_RESET)) && fx.count_limit != fx.count_limit_max {
            d.effect_mut(self).count_limit += 1;
        }
        if count_code != 0 || count_flag != 0 {
            if count_flag as u32 & EFFECT_COUNT_CODE_SINGLE != 0 {
                let fieldid = d.card(self.get_handler(d)).fieldid;
                d.dec_effect_code(fieldid, count_flag, hopt, PLAYER_NONE as u8);
            } else {
                d.dec_effect_code(count_code, count_flag, hopt, playerid as u8);
            }
        }
    }
    pub fn recharge(self, d: &mut Duel) {
        let fx = d.effect_mut(self);
        if fx.is_flag(EFFECT_FLAG_COUNT_LIMIT) {
            fx.count_limit = fx.count_limit_max;
        }
    }
    pub fn get_client_mode(self, d: &Duel) -> u8 {
        let fx = self.fx(d);
        if fx.is_flag(EFFECT_FLAG_FIELD_ONLY) {
            EFFECT_CLIENT_MODE_RESOLVE as u8
        } else if fx.type_ as u32 & EFFECT_TYPE_ACTIONS == 0 {
            EFFECT_CLIENT_MODE_RESET as u8
        } else {
            EFFECT_CLIENT_MODE_NORMAL as u8
        }
    }
    /// `get_value(extraargs)`: the extra arguments were added with `add_param` before.
    pub fn get_value(self, d: &mut Duel, extraargs: u32) -> i64 {
        let fx = self.fx(d);
        if fx.has_function_value() {
            let f = fx.value_fn.clone();
            d.add_param_front(Value::Effect(self));
            d.get_function_value(f.as_ref(), 1 + extraargs)
        } else {
            let v = fx.value as i64;
            d.params.clear();
            v
        }
    }
    pub fn get_value_card(self, d: &mut Duel, pcard: Option<CardId>, extraargs: u32) -> i64 {
        let fx = self.fx(d);
        if fx.has_function_value() {
            let f = fx.value_fn.clone();
            d.add_param_front(Value::card(pcard));
            d.add_param_front(Value::Effect(self));
            d.get_function_value(f.as_ref(), 2 + extraargs)
        } else {
            let v = fx.value as i64;
            d.params.clear();
            v
        }
    }
    pub fn get_value_effect(self, d: &mut Duel, peffect: Option<EffectId>, extraargs: u32) -> i64 {
        let fx = self.fx(d);
        if fx.has_function_value() {
            let f = fx.value_fn.clone();
            d.add_param_front(Value::effect(peffect));
            d.add_param_front(Value::Effect(self));
            d.get_function_value(f.as_ref(), 2 + extraargs)
        } else {
            let v = fx.value as i64;
            d.params.clear();
            v
        }
    }
    pub fn get_values(self, d: &mut Duel, extraargs: u32, result: &mut Vec<i64>) {
        let fx = self.fx(d);
        if fx.has_function_value() {
            let f = fx.value_fn.clone();
            d.add_param_front(Value::Effect(self));
            d.get_function_values(f.as_ref(), 1 + extraargs, result);
        } else {
            let v = fx.value as i64;
            d.params.clear();
            result.push(v);
        }
    }
    pub fn get_values_card(self, d: &mut Duel, pcard: Option<CardId>, extraargs: u32, result: &mut Vec<i64>) {
        let fx = self.fx(d);
        if fx.has_function_value() {
            let f = fx.value_fn.clone();
            d.add_param_front(Value::card(pcard));
            d.add_param_front(Value::Effect(self));
            d.get_function_values(f.as_ref(), 2 + extraargs, result);
        } else {
            let v = fx.value as i64;
            d.params.clear();
            result.push(v);
        }
    }
    pub fn get_values_effect(self, d: &mut Duel, peffect: Option<EffectId>, extraargs: u32, result: &mut Vec<i64>) {
        let fx = self.fx(d);
        if fx.has_function_value() {
            let f = fx.value_fn.clone();
            d.add_param_front(Value::effect(peffect));
            d.add_param_front(Value::Effect(self));
            d.get_function_values(f.as_ref(), 2 + extraargs, result);
        } else {
            let v = fx.value as i64;
            d.params.clear();
            result.push(v);
        }
    }
    pub fn check_value_condition(self, d: &mut Duel, extraargs: u32) -> bool {
        let fx = self.fx(d);
        if fx.has_function_value() {
            let f = fx.value_fn.clone();
            d.add_param_front(Value::Effect(self));
            d.check_condition(f.as_ref(), 1 + extraargs)
        } else {
            let v = fx.value;
            d.params.clear();
            v != 0
        }
    }
    pub fn get_speed(self, d: &mut Duel) -> i32 {
        let fx = self.fx(d);
        let typ = fx.type_ as u32;
        if typ & EFFECT_TYPE_ACTIONS == 0 {
            return 0;
        }
        if typ & (EFFECT_TYPE_TRIGGER_O | EFFECT_TYPE_TRIGGER_F | EFFECT_TYPE_IGNITION) != 0 {
            1
        } else if typ & (EFFECT_TYPE_QUICK_O | EFFECT_TYPE_QUICK_F) != 0 {
            2
        } else if typ & EFFECT_TYPE_ACTIVATE != 0 {
            let handler = fx.handler.unwrap();
            let htype = d.card(handler).data.type_;
            if htype & TYPE_MONSTER != 0 {
                0
            } else if htype & TYPE_SPELL != 0 {
                if htype & TYPE_QUICKPLAY != 0 || handler.is_affected_by_effect(d, EFFECT_BECOME_QUICK).is_some() {
                    2
                } else {
                    1
                }
            } else if htype & TYPE_COUNTER != 0 {
                3
            } else {
                2
            }
        } else {
            0
        }
    }
    /// `effect::clone`
    pub fn clone_effect(self, d: &mut Duel, majestic: bool) -> EffectId {
        let ce = d.new_effect();
        let src = d.effect(self).clone();
        let mut c = src.clone();
        c.handler = None;
        c.script_data = None;
        if majestic && src.is_flag2(EFFECT_FLAG2_MAJESTIC_MUST_COPY) {
            // (the copy already holds every field: the C++ copies them again)
        }
        *d.effect_mut(ce) = c;
        ce
    }
    pub fn get_real_handler(self, d: &Duel) -> Option<CardId> {
        let fx = self.fx(d);
        if fx.active_handler.is_some() {
            return fx.active_handler;
        }
        if fx.type_ as u32 & EFFECT_TYPE_XMATERIAL != 0 {
            let h = d.card(fx.handler.unwrap());
            return if h.overlay_target.is_some() { h.overlay_target } else { h.pre_overlay_target };
        }
        None
    }
    pub fn get_owner(self, d: &Duel) -> CardId {
        if let Some(rh) = self.get_real_handler(d) {
            return rh;
        }
        self.fx(d).owner.unwrap()
    }
    pub fn get_owner_opt(self, d: &Duel) -> Option<CardId> {
        if let Some(rh) = self.get_real_handler(d) {
            return Some(rh);
        }
        self.fx(d).owner
    }
    pub fn get_owner_player(self, d: &Duel) -> u8 {
        let fx = self.fx(d);
        if fx.effect_owner != PLAYER_NONE as u8 {
            return fx.effect_owner;
        }
        d.card(self.get_owner(d)).current.controler
    }
    pub fn get_handler(self, d: &Duel) -> CardId {
        if let Some(rh) = self.get_real_handler(d) {
            return rh;
        }
        self.fx(d).handler.unwrap()
    }
    pub fn get_handler_opt(self, d: &Duel) -> Option<CardId> {
        if let Some(rh) = self.get_real_handler(d) {
            return Some(rh);
        }
        self.fx(d).handler
    }
    pub fn get_handler_player(self, d: &Duel) -> u8 {
        let fx = self.fx(d);
        if fx.is_flag(EFFECT_FLAG_FIELD_ONLY) {
            return fx.effect_owner;
        }
        d.card(self.get_handler(d)).current.controler
    }
    pub fn in_range(self, d: &Duel, pcard: CardId) -> bool {
        let fx = self.fx(d);
        if fx.type_ as u32 & EFFECT_TYPE_XMATERIAL != 0 {
            return d.card(fx.handler.unwrap()).overlay_target.is_some();
        }
        let mut real_range = fx.range as u32;
        if fx.range as u32 & (LOCATION_MMZONE | LOCATION_EMZONE) != 0 {
            real_range &= !(LOCATION_MMZONE | LOCATION_EMZONE);
            real_range |= LOCATION_MZONE;
        }
        d.card(pcard).current.is_location(real_range)
    }
    pub fn is_in_range_of_symbolic_mzone(self, d: &Duel, pcard: CardId) -> bool {
        let range = self.fx(d).range as u32;
        if range & (LOCATION_MMZONE | LOCATION_EMZONE) == 0 {
            return true;
        }
        let cur = &d.card(pcard).current;
        !cur.is_location(LOCATION_MZONE) || cur.is_location(range)
    }
    pub fn in_range_chain(self, d: &Duel, ch: &Chain) -> bool {
        let fx = self.fx(d);
        if fx.type_ as u32 & EFFECT_TYPE_XMATERIAL != 0 {
            return d.card(fx.handler.unwrap()).overlay_target.is_some();
        }
        fx.range & ch.triggering_location != 0
    }
    pub fn set_activate_location(self, d: &mut Duel) {
        let phandler = self.get_handler(d);
        let (loc, seq) = (d.card(phandler).current.location, d.card(phandler).current.sequence);
        let fx = d.effect_mut(self);
        fx.active_location = loc as u16;
        fx.active_sequence = seq as u16;
    }
    pub fn set_active_type(self, d: &mut Duel) {
        let phandler = self.get_handler(d);
        let mut t = phandler.get_type(d, None, 0, 2);
        if t & TYPE_TRAPMONSTER != 0 {
            t &= !TYPE_TRAP;
        }
        d.effect_mut(self).active_type = t;
    }
    pub fn get_active_type(self, d: &mut Duel) -> u32 {
        let fx = self.fx(d);
        if fx.type_ & 0x7f0 != 0 {
            if fx.active_type != 0 {
                fx.active_type
            } else if fx.type_ as u32 & EFFECT_TYPE_ACTIVATE != 0 && d.card(self.get_handler(d)).data.type_ & TYPE_PENDULUM != 0 {
                TYPE_PENDULUM + TYPE_SPELL
            } else {
                self.get_handler(d).get_type(d, None, 0, 2)
            }
        } else {
            fx.owner.unwrap().get_type(d, None, 0, 2)
        }
    }
}

/// The fields of an effect without its functions and labels (cheap to copy out of the duel).
#[derive(Clone, Copy, Debug)]
pub struct EffectHeader {
    pub type_: u16,
    pub code: u32,
    pub flag: [u32; 2],
    pub handler: Option<CardId>,
    pub owner: Option<CardId>,
    pub value: i32,
    pub range: u16,
    pub s_range: u16,
    pub o_range: u16,
}

impl EffectHeader {
    #[inline]
    pub fn is_flag(&self, f: u32) -> bool {
        self.flag[0] & f != 0
    }
    #[inline]
    pub fn is_flag2(&self, f: u32) -> bool {
        self.flag[1] & f != 0
    }
}

impl Effect {
    pub fn clone_header(&self) -> EffectHeader {
        EffectHeader {
            type_: self.type_,
            code: self.code,
            flag: self.flag,
            handler: self.handler,
            owner: self.owner,
            value: self.value,
            range: self.range,
            s_range: self.s_range,
            o_range: self.o_range,
        }
    }
}

impl Value {
    pub fn from_group(g: &Option<Group>) -> Value {
        match g {
            Some(g) => Value::Group(g.clone()),
            None => Value::Nil,
        }
    }
}
