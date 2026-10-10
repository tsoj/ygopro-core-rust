//! Functions of the core not ported yet.
#![allow(unused_variables)]

use crate::duel::Duel;
use crate::types::*;
use crate::units::Unit;

impl Duel {
    pub fn emplace_process<U: Into<Unit>>(&mut self, u: U) {
        self.core.subunits.push_back(u.into());
    }
    #[allow(clippy::too_many_arguments)]
    pub fn raise_event(&mut self, event_cards: CardSet, event_code: u32, reason_effect: Option<EffectId>, reason: u32, reason_player: u8, event_player: u8, event_value: u32) {
        todo!()
    }
    #[allow(clippy::too_many_arguments)]
    pub fn raise_single_event(&mut self, trigger_card: CardId, event_cards: Option<&CardSet>, event_code: u32, reason_effect: Option<EffectId>, reason: u32, reason_player: u8, event_player: u8, event_value: u32) {
        todo!()
    }
    pub fn process_single_event(&mut self) -> i32 {
        todo!()
    }
    pub fn process_instant_event(&mut self) -> i32 {
        todo!()
    }
    pub fn check_hint_timing(&mut self, peffect: EffectId) -> bool {
        todo!()
    }
    #[allow(clippy::too_many_arguments)]
    pub fn send_to(&mut self, targets: CardSet, reason_effect: Option<EffectId>, reason: u32, reason_player: u8, playerid: u8, destination: u16, sequence: u32, position: u8, ignore: bool) {
        todo!()
    }
    pub fn release(&mut self, targets: CardSet, reason_effect: Option<EffectId>, reason: u32, reason_player: u8) {
        todo!()
    }
}
