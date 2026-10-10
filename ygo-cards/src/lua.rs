//! The API as card scripts see it: the core's functions (`ygo_core::api`) with the changes the
//! global Lua scripts make (`proc_workaround.lua`, `utility.lua`, `chain.lua`,
//! `proc_fusion_spell.lua` redefine some functions; those are defined here, the rest is the core's).

pub use ygo_core::api::{CardOrGroup, Exclude, Filter, NO_EX, ValueFn};

pub mod effect {
    pub use ygo_core::api::effect::*;
}

pub mod group {
    pub use ygo_core::api::group::*;
}

pub mod debug {
    pub use ygo_core::api::debug::*;
}

pub mod card {
    use crate::constants::*;
    pub use ygo_core::api::card::*;
    use ygo_core::api::card as raw;
    use ygo_core::duel::Duel;
    use ygo_core::types::*;

    /// `c:RegisterEffect(e, forced)`. (`utility.lua` adds marker effects for extra codes, but
    /// `proc_workaround.lua` wraps it again without passing them on: they are never registered.)
    pub fn register_effect(d: &mut Duel, c: CardId, e: EffectId, forced: bool) -> Option<i32> {
        let reg = raw::register_effect(d, c, e, forced);
        match reg {
            Some(r) if r > 0 => {}
            _ => return reg,
        }
        super::wrap_cannot_summon_target(d, e);
        reg
    }
    /// `c:IsRelateToEffect(e)`: while an activated effect resolves, the relation to its chain link.
    pub fn is_relate_to_effect(d: &mut Duel, c: CardId, e: EffectId) -> bool {
        if !(super::duel::is_chain_solving(d) && ygo_core::api::effect::is_activated(d, e)) {
            return raw::is_relate_to_effect(d, c, e);
        }
        if super::duel::get_reason_effect(d) == Some(e) {
            return raw::is_relate_to_chain(d, c, 0);
        }
        for link in 0..super::duel::get_current_chain(d, false) {
            let te = super::duel::get_chain_info(d, link as u8, &[CHAININFO_TRIGGERING_EFFECT]);
            if te.first().and_then(|v| v.as_effect()) == Some(e) {
                return raw::is_relate_to_chain(d, c, link as u8);
            }
        }
        raw::is_relate_to_effect(d, c, e)
    }
    pub fn is_able_to_hand(d: &mut Duel, c: CardId, p: Option<u8>) -> bool {
        !raw::is_location(d, c, LOCATION_HAND) && raw::is_able_to_hand(d, c, p)
    }
    pub fn is_able_to_deck(d: &mut Duel, c: CardId) -> bool {
        !raw::is_location(d, c, LOCATION_DECK) && raw::is_able_to_deck(d, c)
    }
    pub fn is_able_to_extra(d: &mut Duel, c: CardId) -> bool {
        !raw::is_location(d, c, LOCATION_EXTRA) && raw::is_able_to_extra(d, c)
    }
    pub fn is_able_to_grave(d: &mut Duel, c: CardId, reason: Option<u32>) -> bool {
        !raw::is_location(d, c, LOCATION_GRAVE) && raw::is_able_to_grave(d, c, reason)
    }
    pub fn is_able_to_remove(d: &mut Duel, c: CardId, p: Option<u8>, pos: Option<u8>, reason: Option<u32>) -> bool {
        !raw::is_location(d, c, LOCATION_REMOVED) && raw::is_able_to_remove(d, c, p, pos, reason)
    }
    /// `c:IsCanBeXyzMaterial(xyz, player, reason)`
    pub fn is_can_be_xyz_material(d: &mut Duel, c: CardId, xyz: Option<CardId>, p: Option<u8>, reason: Option<u32>) -> bool {
        if xyz == Some(c) {
            return false;
        }
        let p = p.unwrap_or(d.core.reason_player);
        raw::is_can_be_xyz_material(d, c, xyz, Some(p), Some(reason.unwrap_or(REASON_XYZ | REASON_MATERIAL)))
    }
    /// `c:IsExactType(typ, ...)` (`utility.lua`: the type equals `typ`)
    pub fn is_exact_type(d: &mut Duel, c: CardId, typ: u32) -> bool {
        raw::get_type(d, c) == typ
    }
    pub fn is_exact_type_ctx(d: &mut Duel, c: CardId, typ: u32, scard: Option<CardId>, sumtype: u64, p: Option<u8>) -> bool {
        raw::get_type_ctx(d, c, scard, sumtype, p) == typ
    }
    /// `c:HasFlagEffect(id, ct = 1)` (`utility.lua`)
    pub fn has_flag_effect(d: &Duel, c: CardId, id: u32, ct: Option<usize>) -> bool {
        raw::get_flag_effect(d, c, id) >= ct.unwrap_or(1)
    }
}

pub mod duel {
    use crate::constants::*;
    pub use ygo_core::api::duel::*;
    use ygo_core::api::duel as raw;
    use ygo_core::duel::Duel;
    use ygo_core::types::*;

    /// `Duel.RegisterEffect(e, player)` with the wrappers of `chain.lua` (it records the card
    /// properties of the registering effect: not ported yet, see `chain.lua`) and
    /// `proc_workaround.lua` ("Card Advance" flag, the assumed properties of "cannot summon").
    pub fn register_effect(d: &mut Duel, e: EffectId, p: u8) {
        // (proc_workaround.lua, the last wrapper: the "Card Advance" flag for extra Tribute Summons)
        if ygo_core::api::effect::get_code(d, e) == EFFECT_EXTRA_SUMMON_COUNT && matches!(ygo_core::api::effect::get_value(d, e), Value::Int(1)) {
            let (reset, count) = ygo_core::api::effect::get_reset(d, e);
            raw::register_flag_effect(d, p, CARD_CARD_ADVANCE, reset, 0, count as u16, 0);
        }
        raw::register_effect(d, e, p);
        super::wrap_cannot_summon_target(d, e);
    }
    /// `Duel.IsMainPhase(player)`: also that it is `player`'s turn.
    pub fn is_main_phase(d: &Duel, p: Option<u8>) -> bool {
        raw::is_main_phase(d) && p.is_none_or(|p| raw::is_turn_player(d, p))
    }
    pub fn is_battle_phase(d: &Duel, p: Option<u8>) -> bool {
        raw::is_battle_phase(d) && p.is_none_or(|p| raw::is_turn_player(d, p))
    }
    pub fn is_damage_step(d: &Duel, p: Option<u8>) -> bool {
        let ph = raw::get_current_phase(d);
        (ph == PHASE_DAMAGE || ph == PHASE_DAMAGE_CAL) && p.is_none_or(|p| raw::is_turn_player(d, p))
    }
    /// `Duel.HasFlagEffect(player, id, ct = 1)`
    pub fn has_flag_effect(d: &mut Duel, p: u8, id: u32, ct: Option<usize>) -> bool {
        raw::get_flag_effect(d, p, id) >= ct.unwrap_or(1)
    }
    /// `Duel.MoveToField`: sets the reason player first.
    #[allow(clippy::too_many_arguments)]
    pub fn move_to_field(d: &mut Duel, c: CardId, move_player: u8, playerid: u8, destination: u16, positions: u8, enable: bool, zone: Option<u32>) -> bool {
        let rp = if raw::get_reason_effect(d).is_some() { raw::get_reason_player(d) } else { PLAYER_NONE as u8 };
        ygo_core::api::card::set_reason_player(d, c, rp);
        raw::move_to_field(d, c, move_player, playerid, destination, positions, enable, zone)
    }
    /// `Duel.Overlay`: also raises EVENT_MOVE for the attached cards (and the materials they had).
    pub fn overlay(d: &mut Duel, xyz: CardId, mats: impl Into<super::CardOrGroup>, send_to_grave: bool) {
        let mats: super::CardOrGroup = mats.into();
        let mut eg = mats.cards();
        let (mut re, mut r, mut rp) = (None, REASON_RULE, PLAYER_NONE as u8);
        let cre = raw::get_reason_effect(d);
        if raw::is_chain_solving(d) || cre.is_some_and(|e| !ygo_core::api::effect::is_has_property(d, e, EFFECT_FLAG_CANNOT_DISABLE, 0)) {
            re = cre;
            r = REASON_EFFECT;
            rp = raw::get_reason_player(d);
        }
        if !send_to_grave {
            for c in mats.cards().iter() {
                eg.extend(ygo_core::api::card::get_overlay_group(d, c).iter());
            }
        }
        raw::overlay(d, xyz, mats.cards(), send_to_grave);
        for c in ygo_core::api::group::iter(d, &eg) {
            ygo_core::api::card::set_reason_player(d, c, rp);
        }
        raw::raise_event(d, eg, EVENT_MOVE, re, r, rp, 0, 0);
    }
}

/// `proc_workaround.lua`: a "cannot (special) summon" effect first applies the properties the
/// summoning procedure assumes (`label_object` = a table with the assume function), then checks.
pub(crate) fn wrap_cannot_summon_target(d: &mut ygo_core::duel::Duel, e: ygo_core::types::EffectId) {
    use crate::constants::*;
    use ygo_core::types::*;
    let code = effect::get_code(d, e);
    if code != EFFECT_CANNOT_SUMMON && code != EFFECT_CANNOT_SPECIAL_SUMMON {
        return;
    }
    let Some(base_tg) = effect::get_target(d, e) else { return };
    let assume = |d: &mut ygo_core::duel::Duel, pe: Option<EffectId>, c: CardId| {
        if let Some(pe) = pe {
            if let Value::Multi(t) = effect::get_label_object(d, pe) {
                if let Some(Value::Fn(f)) = t.first() {
                    f.call(d, &[Value::Card(c)]);
                }
            }
        }
    };
    let wrapped = LuaFn::new(move |d, a| {
        // (e, c, sump, sumtype, sumpos, targetp, sum_eff, sum_proc_eff)
        let c = a.get(1).and_then(|v| v.as_card());
        let sumtype = a.get(3).map_or(0, |v| v.int());
        let sum_proc_eff = a.get(7).and_then(|v| v.as_effect());
        if let Some(c) = c {
            assume(d, sum_proc_eff, c);
            if code == EFFECT_CANNOT_SUMMON && sumtype == SUMMON_TYPE_NORMAL as i64 + 1 {
                let ns = ygo_core::api::card::is_has_effect(d, c, EFFECT_SUMMON_PROC, None);
                assume(d, ns.first().copied(), c);
            }
        }
        base_tg.call(d, a)
    });
    effect::set_target(d, e, Some(wrapped));
}
