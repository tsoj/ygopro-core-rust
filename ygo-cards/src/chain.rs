//! `Chain.*` (`chain.lua`): data kept per chain link, and named chain information.

use crate::constants::*;
use crate::lua::duel;
use crate::prelude::*;

/// `Duel::script_var` namespaces of the chain data.
const NS_EFFECT_DATA: u32 = 3;

/// The fields scripts store in `e:GetChainData()`.
pub mod field {
    pub const COST_LP_PAID: u8 = 1;
    pub const COST_DISCARDED_CARDS: u8 = 2;
    pub const COST_DETACHED_MATERIALS: u8 = 3;
    pub const COST_CHOICE: u8 = 4;
}

/// `Chain.GetID(ch)`
pub fn get_id(d: &Duel, ch: u8) -> i64 {
    duel::get_chain_info(d, ch, &[CHAININFO_CHAIN_ID]).first().map_or(0, |v| v.int())
}

/// The chain link `Chain.Data(ch)` refers to (errors as the script does).
fn data_link(d: &Duel, ch: Option<u8>) -> u8 {
    let current = duel::get_current_chain(d, false);
    if current == 0 {
        crate::lua_error("Attempted to access Chain Data table while there is currently no Chain");
    }
    match ch {
        Some(c) if c as i32 > current => crate::lua_error("Attempted to access Chain Link higher than current Chain"),
        Some(c) if c != 0 => c,
        _ => current as u8,
    }
}

fn effect_key(d: &Duel, e: EffectId, ch: Option<u8>, f: u8) -> (u32, u64) {
    let cid = get_id(d, data_link(d, ch)) as u64;
    (NS_EFFECT_DATA, (cid << 40) | ((e.0 as u64) << 8) | f as u64)
}

/// `e:GetChainData(ch).<field>`
pub fn effect_data(d: &Duel, e: EffectId, ch: Option<u8>, f: u8) -> Value {
    d.script_var(effect_key(d, e, ch, f))
}
/// `e:GetChainData(ch).<field> = v`
pub fn set_effect_data(d: &mut Duel, e: EffectId, ch: Option<u8>, f: u8, v: Value) {
    let k = effect_key(d, e, ch, f);
    d.set_script_var(k, v);
}
