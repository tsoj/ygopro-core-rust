//! `Group.*` as scripts see it.

pub use ygo_core::api::group::*;

use ygo_core::duel::Duel;
use ygo_core::types::*;

/// `g:CheckSameProperty(f)`: whether all cards share a property bit, and the shared bits
/// (`None` for an empty group, where Lua returns `true, nil`).
pub fn check_same_property(d: &mut Duel, g: &Group, f: &dyn Fn(&mut Duel, CardId) -> u64) -> (bool, Option<u64>) {
    let mut prop: Option<u64> = None;
    for tc in iter(d, g) {
        let v = f(d, tc);
        let p = prop.map_or(v, |p| p & v);
        prop = Some(p);
        if p == 0 {
            return (false, Some(0));
        }
    }
    (prop != Some(0), prop)
}
fn checkrecbin(d: &mut Duel, c: CardId, g: &Group, val: u64, f: &dyn Fn(&mut Duel, CardId) -> u64) -> bool {
    let prop = f(d, c) & !val;
    if prop == 0 {
        return false;
    }
    let rest = difference(g, &CardSet::from_card(c));
    let mut i = 1u64;
    while i <= prop && i != 0 {
        if prop & i != 0 && (g.len() < 2 || iter(d, &rest).into_iter().any(|cc| checkrecbin(d, cc, &rest, val | i, f))) {
            return true;
        }
        i <<= 1;
    }
    false
}
/// `g:CheckDifferentPropertyBinary(f)`: every card has a property bit the others don't use.
pub fn check_different_property_binary(d: &mut Duel, g: &Group, f: &dyn Fn(&mut Duel, CardId) -> u64) -> bool {
    if g.len() < 2 {
        return true;
    }
    iter(d, g).into_iter().any(|c| checkrecbin(d, c, g, 0, f))
}
fn checkrec(d: &mut Duel, c: CardId, g: &Group, t: &mut Vec<i64>, f: &dyn Fn(&mut Duel, CardId) -> Vec<i64>) -> bool {
    let rest = difference(g, &CardSet::from_card(c));
    for prop in f(d, c) {
        if !t.contains(&prop) {
            t.push(prop);
            if g.len() < 2 || iter(d, &rest).into_iter().any(|cc| checkrec(d, cc, &rest, t, f)) {
                return true;
            }
            t.pop();
        }
    }
    false
}
/// `g:CheckDifferentProperty(f)`: every card has a property (of those `f` returns) the others
/// don't use.
pub fn check_different_property(d: &mut Duel, g: &Group, f: &dyn Fn(&mut Duel, CardId) -> Vec<i64>) -> bool {
    if g.len() < 2 {
        return true;
    }
    iter(d, g).into_iter().any(|c| checkrec(d, c, g, &mut Vec::new(), f))
}
/// `g:GetLinkedZone(tp)`
pub fn get_linked_zone(d: &mut Duel, g: &Group, tp: u8) -> u32 {
    get_bitwise_or(d, g, &|d, c| ygo_core::api::card::get_linked_zone(d, c, Some(tp)) as i64) as u32
}
