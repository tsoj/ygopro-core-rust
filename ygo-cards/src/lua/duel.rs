//! `Duel.*` as scripts see it (see `card.rs`).

use crate::constants::*;
pub use ygo_core::api::duel::*;
use ygo_core::api::duel as raw;
use ygo_core::duel::Duel;
use ygo_core::types::*;

/// `Duel.RegisterEffect(e, player)` with the wrappers of `chain.lua` (it records the card
/// properties of the registering effect) and
/// `proc_workaround.lua` ("Card Advance" flag, the assumed properties of "cannot summon").
pub fn register_effect(d: &mut Duel, e: EffectId, p: u8) {
    // (proc_workaround.lua, the last wrapper: the "Card Advance" flag for extra Tribute Summons)
    if ygo_core::api::effect::get_code(d, e) == EFFECT_EXTRA_SUMMON_COUNT && matches!(ygo_core::api::effect::get_value(d, e), Value::Int(1)) {
        let (reset, count) = ygo_core::api::effect::get_reset(d, e);
        raw::register_flag_effect(d, p, CARD_CARD_ADVANCE, reset, 0, count as u16, 0);
    }
    // (chain.lua)
    crate::chain::record_registering_props(d, e);
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
/// `Duel.MoveToDeckTop(targets)`
pub fn move_to_deck_top(d: &mut Duel, targets: impl Into<super::CardOrGroup>) {
    let t: super::CardOrGroup = targets.into();
    for c in ygo_core::api::group::iter(d, &t.cards()) {
        if ygo_core::api::card::is_location(d, c, LOCATION_DECK) {
            raw::move_sequence(d, c, SEQ_DECKTOP as u8, None);
        }
    }
}
/// `Duel.MoveToDeckBottom(targets)` / `Duel.MoveToDeckBottom(count, tp)`
pub fn move_to_deck_bottom(d: &mut Duel, targets: impl Into<super::CardOrGroup>) {
    let t: super::CardOrGroup = targets.into();
    for c in ygo_core::api::group::iter(d, &t.cards()) {
        if ygo_core::api::card::is_location(d, c, LOCATION_DECK) {
            raw::move_sequence(d, c, SEQ_DECKBOTTOM as u8, None);
        }
    }
}
pub fn move_decktop_to_bottom(d: &mut Duel, count: u32, tp: u8) {
    for _ in 0..count {
        let mg = raw::get_decktop_group(d, tp, 1);
        let c = ygo_core::api::group::get_first(d, &mg).unwrap_or_else(|| crate::lua_error("attempt to index a nil value"));
        raw::move_sequence(d, c, SEQ_DECKBOTTOM as u8, None);
    }
}
/// `Duel.GetTargetCards(e)`: the targets still related to `e` (`None` without targets).
pub fn get_target_cards(d: &mut Duel, e: EffectId) -> Option<CardSet> {
    let tg = raw::get_chain_info(d, 0, &[crate::constants::CHAININFO_TARGET_CARDS]);
    match tg.first() {
        Some(Value::Group(g)) => {
            let g = g.clone();
            Some(ygo_core::api::group::filter(d, &g, &|d, c| super::card::is_relate_to_effect(d, c, e), super::NO_EX))
        }
        _ => None,
    }
}
/// `Duel.GetZoneWithLinkedCount(count, tp)`
pub fn get_zone_with_linked_count(d: &mut Duel, count: usize, tp: u8) -> u32 {
    let g = raw::get_matching_group(d, Some(&|d, c| ygo_core::api::card::is_type(d, c, TYPE_LINK)), tp, LOCATION_MZONE, LOCATION_MZONE, super::NO_EX);
    let mut rzone = 0u32;
    for zone in [0x1u32, 0x2, 0x4, 0x8, 0x10, 0x20, 0x40] {
        let mut ct = 0;
        for tc in ygo_core::api::group::iter(d, &g) {
            if zone & ygo_core::api::card::get_linked_zone(d, tc, Some(tp)) != 0 {
                ct += 1;
            }
        }
        if ct >= count {
            rzone |= zone;
        }
    }
    rzone
}
/// `Duel.AnnounceAnotherAttribute(g, tp)`
pub fn announce_another_attribute(d: &mut Duel, g: &CardSet, tp: u8) -> u32 {
    let att = ygo_core::api::group::get_bitwise_or(d, g, &|d, c| ygo_core::api::card::get_attribute(d, c) as i64) as u32;
    raw::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATTRIBUTE as u64);
    raw::announce_attribute(d, tp, 1, if att & att.wrapping_sub(1) == 0 { !att & ATTRIBUTE_ALL } else { ATTRIBUTE_ALL })
}
pub fn announce_another_race(d: &mut Duel, g: &CardSet, tp: u8) -> u64 {
    let race = ygo_core::api::group::get_bitwise_or(d, g, &|d, c| ygo_core::api::card::get_race(d, c) as i64);
    raw::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RACE as u64);
    raw::announce_race(d, tp, 1, if race & race.wrapping_sub(1) == 0 { !race & RACE_ALL } else { RACE_ALL })
}
/// `Duel.SummonOrSet(tp, c, ign, e, minc, zone)`: Normal Summon or Set, as the player chooses.
pub fn summon_or_set(d: &mut Duel, tp: u8, c: CardId, ign: bool, e: Option<EffectId>, minc: u8, zone: Option<u32>) {
    let s1 = ygo_core::api::card::is_summonable(d, c, ign, e, minc as u16, zone.map(|z| z as u16));
    let s2 = ygo_core::api::card::is_m_setable(d, c, ign, e, minc as u16, zone);
    if (s1 && s2 && raw::select_position(d, tp, c, (POS_FACEUP_ATTACK + POS_FACEDOWN_DEFENSE) as u8) == POS_FACEUP_ATTACK) || !s2 {
        raw::summon(d, tp, c, ign, e, minc, zone);
    } else {
        raw::m_set(d, tp, c, ign, e, minc, zone);
    }
}
/// `Duel.ActivateFieldSpell(c, e, tp, eg, ep, ev, re, r, rp, target_p)`: places `c` as the
/// active Field Spell (the old one is sent to the GY/destroyed).
pub fn activate_field_spell(d: &mut Duel, c: Option<CardId>, a: &crate::prelude::EffArgs, target_p: Option<u8>) -> bool {
    let tp = a.tp;
    let target_p = target_p.unwrap_or(tp);
    let Some(c) = c else { return false };
    let fc = raw::get_field_card(d, target_p, LOCATION_FZONE, 0);
    if raw::is_duel_type(d, DUEL_1_FIELD as u64) {
        if let Some(fc) = fc {
            raw::destroy(d, fc, REASON_RULE, None, None);
        }
        let of = raw::get_field_card(d, 1 - target_p, LOCATION_FZONE, 0);
        if let Some(of) = of.filter(|&of| raw::destroy(d, of, REASON_RULE, None, None) == 0) {
            let _ = of;
            raw::send_to_grave(d, c, REASON_RULE, None, None);
            return false;
        }
        raw::break_effect(d);
    } else {
        if fc.is_some_and(|fc| raw::send_to_grave(d, fc, REASON_RULE, None, None) == 0) {
            raw::send_to_grave(d, c, REASON_RULE, None, None);
            return false;
        }
        raw::break_effect(d);
    }
    move_to_field(d, c, tp, target_p, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None);
    let te = *ygo_core::api::card::get_activate_effect(d, c).first().unwrap_or_else(|| crate::lua_error("attempt to index a nil value (local 'te')"));
    ygo_core::api::effect::use_count_limit(d, te, tp, 1, true);
    let tep = ygo_core::api::card::get_controler(d, c);
    if let Some(cost) = ygo_core::api::effect::get_cost(d, te) {
        cost.call(d, &[Value::Effect(te), Value::Int(tep as i64), a.eg.clone().map_or(Value::Nil, Value::Group), Value::Int(a.ep as i64), Value::Int(a.ev as i64), Value::effect(a.re), Value::Int(a.r as i64), Value::Int(a.rp as i64), Value::Int(1)]);
    }
    let ch = raw::get_current_chain(d, false);
    raw::raise_event(d, c, 4179255, Some(te), 0, tp, target_p, ch as u32);
    true
}
/// `Duel.GoatConfirm(tp, loc)`: reveals the hand and confirms the deck (GOAT format rules)
pub fn goat_confirm(d: &mut Duel, tp: u8, loc: u32) {
    let g = raw::get_field_group(d, tp, loc & (LOCATION_HAND | LOCATION_DECK), 0);
    let (dg, hg) = ygo_core::api::group::split(d, &g, &|d, c| ygo_core::api::card::is_location(d, c, LOCATION_DECK), super::NO_EX);
    raw::confirm_cards(d, tp, &dg);
    raw::confirm_cards(d, 1 - tp, &hg);
    if !hg.is_empty() {
        raw::shuffle_hand(d, tp);
    }
    if !dg.is_empty() {
        raw::shuffle_deck(d, tp);
    }
}
/// `Duel.CallCoin(tp)`: the player calls a coin toss; whether the call was right
pub fn call_coin(d: &mut Duel, tp: u8) -> bool {
    let call = raw::announce_coin(d, tp, None);
    let r = raw::toss_coin(d, tp, 1);
    r.first().is_some_and(|&x| x == call)
}
/// `Duel.CountHeads(...)`
pub fn count_heads(results: &[i32]) -> usize {
    results.iter().filter(|&&r| r == COIN_HEADS as i32).count()
}
pub fn count_tails(results: &[i32]) -> usize {
    results.iter().filter(|&&r| r == COIN_TAILS as i32).count()
}
pub fn check_pendulum_zones(d: &Duel, player: u8) -> bool {
    raw::check_location(d, player, LOCATION_PZONE, 0) || raw::check_location(d, player, LOCATION_PZONE, 1)
}
/// `local tc=Duel.GetFirstTarget()`: the first target (nil without targets)
pub fn first_target(d: &Duel) -> Option<CardId> {
    raw::get_first_target(d).first().copied()
}
/// `Duel.CheckReleaseGroupCost`/`Duel.SelectReleaseGroupCost` (`proc_workaround.lua`)
pub use crate::aux::{check_release_group_cost, select_release_group_cost};

/// `Duel.GetTargetGroup(filter, player, loc1, loc2, ex, ...)`: `GetMatchingGroup` of the cards
/// the current effect can target
pub fn get_target_group(d: &mut Duel, f: Option<super::Filter>, p: u8, s: u32, o: u32, ex: super::Exclude) -> CardSet {
    let mut g = raw::get_matching_group(d, f, p, s, o, ex);
    let re = raw::get_reason_effect(d);
    ygo_core::api::group::match_(d, &mut g, &|d, c| ygo_core::api::card::is_can_be_effect_target(d, c, re), super::NO_EX);
    g
}
fn is_phase_of(d: &Duel, phase: u32, p: Option<u8>) -> bool {
    raw::get_current_phase(d) == phase && p.is_none_or(|p| raw::is_turn_player(d, p))
}
/// `Duel.IsDrawPhase(player)`
pub fn is_draw_phase(d: &Duel, p: Option<u8>) -> bool {
    is_phase_of(d, PHASE_DRAW, p)
}
pub fn is_standby_phase(d: &Duel, p: Option<u8>) -> bool {
    is_phase_of(d, PHASE_STANDBY, p)
}
pub fn is_main_phase1(d: &Duel, p: Option<u8>) -> bool {
    is_phase_of(d, PHASE_MAIN1, p)
}
/// `Duel.IsStartOfBattlePhase(player)` (`Duel.IsStartStep`)
pub fn is_start_of_battle_phase(d: &Duel, p: Option<u8>) -> bool {
    is_phase_of(d, PHASE_BATTLE_START, p)
}
pub fn is_start_step(d: &Duel, p: Option<u8>) -> bool {
    is_phase_of(d, PHASE_BATTLE_START, p)
}
pub fn is_battle_step(d: &Duel, p: Option<u8>) -> bool {
    is_phase_of(d, PHASE_BATTLE_STEP, p)
}
pub fn is_damage_calculation(d: &Duel, p: Option<u8>) -> bool {
    is_phase_of(d, PHASE_DAMAGE_CAL, p)
}
/// `Duel.IsEndOfBattlePhase(player)` (`Duel.IsEndStep`)
pub fn is_end_of_battle_phase(d: &Duel, p: Option<u8>) -> bool {
    is_phase_of(d, PHASE_BATTLE, p)
}
pub fn is_end_step(d: &Duel, p: Option<u8>) -> bool {
    is_phase_of(d, PHASE_BATTLE, p)
}
pub fn is_main_phase2(d: &Duel, p: Option<u8>) -> bool {
    is_phase_of(d, PHASE_MAIN2, p)
}
pub fn is_end_phase(d: &Duel, p: Option<u8>) -> bool {
    is_phase_of(d, PHASE_END, p)
}
/// `Duel.IsPlayerCanAdditionalTributeSummon(player)`: no "Card Advance"-like effect used yet
pub fn is_player_can_additional_tribute_summon(d: &mut Duel, p: u8) -> bool {
    !has_flag_effect(d, p, CARD_CARD_ADVANCE, None)
}
/// `Duel.SwapSequence(c1, c2)`: whether the cards swapped
pub fn swap_sequence(d: &mut Duel, c1: CardId, c2: CardId) -> bool {
    use ygo_core::api::card as k;
    let seq1 = k::get_sequence(d, c1);
    let seq2 = k::get_sequence(d, c2);
    raw::swap_sequence(d, c1, c2);
    k::is_on_field(d, c1) && k::is_on_field(d, c2) && k::get_sequence(d, c1) == seq2 && k::get_sequence(d, c2) == seq1
}
/// `Duel.GetFusionMaterial(tp)` as proc_fusion_spell.lua redefines it (also the Extra Deck cards
/// with an EFFECT_EXTRA_FUSION_MATERIAL effect)
pub fn get_fusion_material(d: &mut Duel, tp: u8) -> CardSet {
    crate::procs::fusion::get_fusion_material(d, tp)
}
/// The core's `Duel.GetFusionMaterial`
pub fn get_fusion_material_raw(d: &mut Duel, tp: u8) -> CardSet {
    raw::get_fusion_material(d, tp)
}

macro_rules! from_code {
    ($(#[$m:meta] $name:ident, $field:ident, $t:ty;)*) => {$(
        #[$m]
        pub fn $name(d: &Duel, code: u32) -> Option<$t> {
            raw::get_card_data_from_code(d, code).map(|x| x.$field as $t)
        }
    )*};
}
from_code! {
    /// `Duel.GetCardAliasFromCode(code)` (`None`: unknown code, Lua returns nothing)
    get_card_alias_from_code, alias, u32;
    /// `Duel.GetCardTypeFromCode(code)`
    get_card_type_from_code, type_, u32;
    /// `Duel.GetCardLevelFromCode(code)`
    get_card_level_from_code, level, u32;
    /// `Duel.GetCardAttributeFromCode(code)`
    get_card_attribute_from_code, attribute, u32;
    /// `Duel.GetCardRaceFromCode(code)`
    get_card_race_from_code, race, u64;
    /// `Duel.GetCardAttackFromCode(code)`
    get_card_attack_from_code, attack, i32;
    /// `Duel.GetCardDefenseFromCode(code)`
    get_card_defense_from_code, defense, i32;
    /// `Duel.GetCardRscaleFromCode(code)`
    get_card_rscale_from_code, rscale, u32;
    /// `Duel.GetCardLscaleFromCode(code)`
    get_card_lscale_from_code, lscale, u32;
    /// `Duel.GetCardLinkMarkerFromCode(code)`
    get_card_link_marker_from_code, link_marker, u32;
}
/// `Duel.GetCardSetcodeFromCode(code)`: all set codes (empty: unknown code)
pub fn get_card_setcode_from_code(d: &Duel, code: u32) -> Vec<u16> {
    raw::get_card_data_from_code(d, code).map_or(vec![], |x| x.setcodes.iter().map(|s| s as u16).collect())
}
/// `Duel.CheckReleaseGroupEx(...)`: `Duel.CheckReleaseGroup` with `use_hand` defaulting to true
pub fn check_release_group_ex(d: &mut Duel, p: u8, f: Option<super::Filter>, min: u16, ra: raw::ReleaseArgs, ex: super::Exclude) -> bool {
    raw::check_release_group(d, p, f, min, ra, ex, true)
}
/// `Duel.SelectReleaseGroupEx(...)`: `Duel.SelectReleaseGroup` with `use_hand` defaulting to true
pub fn select_release_group_ex(d: &mut Duel, p: u8, f: Option<super::Filter>, min: u16, max: u16, ra: raw::ReleaseArgs, ex: super::Exclude) -> Option<CardSet> {
    raw::select_release_group(d, p, f, min, max, ra, ex, true)
}
