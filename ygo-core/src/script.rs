//! The registry of card scripts: card code -> `initial_effect`.

use std::collections::HashMap;

use crate::duel::Duel;
use crate::types::CardId;

pub type InitialEffect = fn(&mut Duel, CardId);

#[derive(Default)]
pub struct Scripts {
    pub map: HashMap<u32, InitialEffect>,
}

impl Scripts {
    pub fn get(&self, code: u32) -> Option<InitialEffect> {
        self.map.get(&code).copied()
    }
    pub fn insert(&mut self, code: u32, f: InitialEffect) {
        self.map.insert(code, f);
    }
}
