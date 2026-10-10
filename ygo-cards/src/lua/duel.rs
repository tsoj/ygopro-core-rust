//! `Duel.*` as scripts see it (see `card.rs`).

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

/// `Duel.SelectEffect(tp, {check, desc}...)`: the 1-based index of the chosen entry (`None` if
/// none can be chosen).
pub fn select_effect(d: &mut Duel, tp: u8, choices: &[(bool, u64)]) -> Option<usize> {
    let mut eff = Vec::new();
    let mut sel = Vec::new();
    for (i, &(ok, desc)) in choices.iter().enumerate() {
        if ok {
            eff.push(desc);
            sel.push(i + 1);
        }
    }
    if eff.is_empty() {
        return None;
    }
    let op = raw::select_option(d, tp, true, &eff);
    sel.get(op as usize).copied()
}
