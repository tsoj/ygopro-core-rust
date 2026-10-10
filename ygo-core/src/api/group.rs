//! `Group.*` (`libgroup.cpp`). Groups are values (`CardSet`, ordered by card id); functions that
//! change a group take `&mut CardSet`. Scripts iterate a group in public order (`iter`), as
//! `GetFirst`/`GetNext` do. Note `g1 == g2` in Lua (`__eq`) compares only the sizes; `Group.Equal`
//! compares the cards.

use super::{check_matching, get_operation_value, return_cards, Exclude, Filter, ValueFn};
use crate::card::public_order;
use crate::constants::*;
use crate::duel::Duel;
use crate::types::*;
use crate::units;

/// `Group.CreateGroup(...)` / `Group.FromCards(...)`
pub fn create_group(cards: &[CardId]) -> Group {
    CardSet::from_iter(cards.iter().copied())
}
/// The cards in public order (what `GetFirst`/`GetNext` and `Iter` go through).
pub fn iter(d: &Duel, g: &Group) -> Vec<CardId> {
    public_order(d, g)
}
/// `g:GetFirst()`
pub fn get_first(d: &Duel, g: &Group) -> Option<CardId> {
    public_order(d, g).first().copied()
}
/// `g:TakeatPos(pos)`
pub fn take_at_pos(d: &Duel, g: &Group, pos: usize) -> Option<CardId> {
    if pos >= g.len() { None } else { Some(public_order(d, g)[pos]) }
}
pub fn get_count(g: &Group) -> usize {
    g.len()
}
pub fn add_card(g: &mut Group, cards: &CardSet) {
    g.extend(cards.iter());
}
pub fn remove_card(g: &mut Group, cards: &CardSet) {
    for c in cards.iter() {
        g.remove(c);
    }
}
pub fn filter(d: &mut Duel, g: &Group, f: Filter, ex: Exclude) -> Group {
    let mut cset = g.clone();
    ex.remove_from(&mut cset);
    let mut out = CardSet::new();
    for c in public_order(d, &cset) {
        if check_matching(d, f, c) {
            out.insert(c);
        }
    }
    out
}
/// `g:Match(f, ex)`: keeps the matching cards (in card id order).
pub fn match_(d: &mut Duel, g: &mut Group, f: Filter, ex: Exclude) {
    let cards = g.to_vec();
    let mut keep = CardSet::new();
    match ex {
        Exclude::Group(exg) => {
            // (the core walks the exclusion group alongside, in card id order)
            let ex_cards = exg.to_vec();
            let mut pex = 0;
            for c in cards {
                let removed_by_ex = pex < ex_cards.len() && ex_cards[pex] == c;
                if removed_by_ex {
                    pex += 1;
                }
                if !removed_by_ex && check_matching(d, f, c) {
                    keep.insert(c);
                }
            }
        }
        _ => {
            for c in cards {
                if !ex.contains(c) && check_matching(d, f, c) {
                    keep.insert(c);
                }
            }
        }
    }
    *g = keep;
}
pub fn filter_count(d: &mut Duel, g: &Group, f: Filter, ex: Exclude) -> usize {
    let mut cset = g.clone();
    ex.remove_from(&mut cset);
    let mut n = 0;
    for c in public_order(d, &cset) {
        if check_matching(d, f, c) {
            n += 1;
        }
    }
    n
}
fn select_card(d: &mut Duel, playerid: u8, cancelable: bool, min: u16, max: u16) -> Option<Group> {
    d.emplace_process(units::SelectCard { step: 0, playerid, cancelable, min: min as u8, max: max as u8 });
    d.run_pushed_units();
    return_cards(d, cancelable)
}
/// `g:FilterSelect(player, f, min, max, cancelable, ex)`: `None` if canceled.
#[allow(clippy::too_many_arguments)]
pub fn filter_select(d: &mut Duel, g: &Group, playerid: u8, f: Filter, min: u16, max: u16, cancelable: bool, ex: Exclude) -> Option<Group> {
    crate::lua::check_action_permission(d);
    let mut cset = g.clone();
    ex.remove_from(&mut cset);
    if playerid > 1 {
        return None;
    }
    let mut sel = Vec::new();
    for c in public_order(d, &cset) {
        if check_matching(d, f, c) {
            sel.push(c);
        }
    }
    d.core.select_cards = sel;
    select_card(d, playerid, cancelable, min, max)
}
/// `g:Select(player, min, max, cancelable, ex)`
pub fn select(d: &mut Duel, g: &Group, playerid: u8, min: u16, max: u16, cancelable: bool, ex: Exclude) -> Option<Group> {
    crate::lua::check_action_permission(d);
    let mut cset = g.clone();
    ex.remove_from(&mut cset);
    if playerid > 1 {
        return None;
    }
    d.core.select_cards = cset.to_vec();
    select_card(d, playerid, cancelable, min, max)
}
/// `g:SelectUnselect(g2, player, finishable, cancelable, min = 1, max = 1)`: the card picked, or
/// `None` (finished/canceled, or `g` and `g2` overlap).
#[allow(clippy::too_many_arguments)]
pub fn select_unselect(d: &mut Duel, g: &Group, g2: Option<&Group>, playerid: u8, finishable: bool, cancelable: bool, min: Option<u16>, max: Option<u16>) -> Option<CardId> {
    crate::lua::check_action_permission(d);
    if playerid > 1 {
        return None;
    }
    if let Some(g2) = g2 {
        if g2.iter().any(|c| g.contains(c)) {
            return None;
        }
    }
    let max = max.unwrap_or(1);
    let mut min = min.unwrap_or(1);
    if min > max {
        min = max;
    }
    d.core.select_cards = g.to_vec();
    d.core.unselect_cards = g2.map_or(Vec::new(), |g2| g2.to_vec());
    d.emplace_process(units::SelectUnselectCard { step: 0, playerid, cancelable, min: min as u8, max: max as u8, finishable });
    d.run_pushed_units();
    if d.return_cards.canceled { None } else { Some(d.return_cards.list[0]) }
}
/// `g:RandomSelect(player, count)`
pub fn random_select(d: &mut Duel, g: &Group, playerid: u8, count: usize) -> Group {
    let count = count.min(g.len());
    if count == 0 {
        return CardSet::new();
    }
    let out = if count == g.len() {
        g.clone()
    } else {
        let order = public_order(d, g); // (not creation order)
        let mut out = CardSet::new();
        while out.len() < count {
            let i = d.get_next_integer(0, g.len() as i32 - 1);
            out.insert(order[i as usize]);
        }
        out
    };
    let m = d.new_message(MSG_RANDOM_SELECTED);
    d.msg(m).u8(playerid).u32(count as u32);
    for c in out.iter() {
        let loc = c.get_info_location(d);
        d.msg(m).loc(loc);
    }
    out
}
/// `g:IsExists(f, count, ex)`
pub fn is_exists(d: &mut Duel, g: &Group, f: Filter, count: u16, ex: Exclude) -> bool {
    let mut cset = g.clone();
    ex.remove_from(&mut cset);
    let mut n = 0u32;
    for c in public_order(d, &cset) {
        if check_matching(d, f, c) {
            n += 1;
            if n >= count as u32 {
                break;
            }
        }
    }
    n >= count as u32
}
/// The cards of a sum check: the must-select cards first, with their sum values.
fn sum_cards(d: &mut Duel, g: &Group, f: ValueFn) -> (Vec<CardId>, i32) {
    let mut cv = d.core.must_select_cards.clone();
    let mcount = cv.len() as i32;
    let must = d.core.must_select_cards.clone();
    for c in public_order(d, g) {
        if !must.contains(&c) {
            cv.push(c);
        }
    }
    d.core.must_select_cards.clear();
    for &c in &cv {
        let v = get_operation_value(d, f, c) as u32;
        d.card_mut(c).sum_param = v;
        if v == 0 {
            crate::lua::lua_error("Group contains a card for which the value function returned 0.");
        }
    }
    (cv, mcount)
}
/// `g:CheckWithSumEqual(f, sum, min, max)`: (possible, should_continue)
pub fn check_with_sum_equal(d: &mut Duel, g: &Group, f: ValueFn, acc: u32, min: i32, max: i32) -> (bool, bool) {
    let min = min.max(0);
    let max = max.max(min);
    let (cv, mcount) = sum_cards(d, g, f);
    let mut should_continue = true;
    let res = Duel::check_with_sum_limit_m(d, &cv, acc as i32, 0, min, max, mcount, &mut Some(&mut should_continue));
    (res, should_continue)
}
/// The cards for a sum selection (`select_cards` without the must-select cards).
fn sum_select_cards(d: &mut Duel, g: &Group, f: ValueFn) -> (Vec<CardId>, i32) {
    let must = d.core.must_select_cards.clone();
    let sel: Vec<CardId> = g.iter().filter(|c| !must.contains(c)).collect();
    d.core.select_cards = sel.clone();
    let mut cv = must.clone();
    let mcount = cv.len() as i32;
    cv.extend(sel);
    for &c in &cv {
        let v = get_operation_value(d, f, c) as u32;
        d.card_mut(c).sum_param = v;
        if v == 0 {
            crate::lua::lua_error("Group contains a card for which the value function returned 0.");
        }
    }
    (cv, mcount)
}
/// `g:SelectWithSumEqual(player, f, sum, min, max)`
pub fn select_with_sum_equal(d: &mut Duel, g: &Group, playerid: u8, f: ValueFn, acc: u32, min: i32, max: i32) -> Option<Group> {
    crate::lua::check_action_permission(d);
    if playerid > 1 {
        return None;
    }
    let min = min.max(0);
    let max = max.max(min);
    let (cv, mcount) = sum_select_cards(d, g, f);
    if !Duel::check_with_sum_limit_m(d, &cv, acc as i32, 0, min, max, mcount, &mut None) {
        d.core.must_select_cards.clear();
        return Some(CardSet::new());
    }
    d.emplace_process(units::SelectSum { step: 0, playerid, acc: acc as i32, min, max });
    d.run_pushed_units();
    let out = CardSet::from_iter(d.return_cards.list.iter().copied());
    d.core.must_select_cards.clear();
    Some(out)
}
/// `g:CheckWithSumGreater(f, sum)`: (possible, should_continue)
pub fn check_with_sum_greater(d: &mut Duel, g: &Group, f: ValueFn, acc: u32) -> (bool, bool) {
    let (cv, mcount) = sum_cards(d, g, f);
    let mut should_continue = true;
    let res = Duel::check_with_sum_greater_limit_m(d, &cv, acc as i32, 0, 0xffff, mcount, &mut Some(&mut should_continue));
    (res, should_continue)
}
/// `g:SelectWithSumGreater(player, f, sum)`
pub fn select_with_sum_greater(d: &mut Duel, g: &Group, playerid: u8, f: ValueFn, acc: u32) -> Option<Group> {
    crate::lua::check_action_permission(d);
    if playerid > 1 {
        return None;
    }
    let (cv, mcount) = sum_select_cards(d, g, f);
    if !Duel::check_with_sum_greater_limit_m(d, &cv, acc as i32, 0, 0xffff, mcount, &mut None) {
        d.core.must_select_cards.clear();
        return Some(CardSet::new());
    }
    d.emplace_process(units::SelectSum { step: 0, playerid, acc: acc as i32, min: 0, max: 0 });
    d.run_pushed_units();
    let out = CardSet::from_iter(d.return_cards.list.iter().copied());
    d.core.must_select_cards.clear();
    Some(out)
}
fn extreme_group(d: &mut Duel, g: &Group, f: ValueFn, better: fn(i64, i64) -> bool) -> Option<(Group, i64)> {
    let cards = g.to_vec();
    let first = *cards.first()?;
    let mut best = get_operation_value(d, f, first);
    let mut out = CardSet::from_card(first);
    for &c in &cards[1..] {
        let op = get_operation_value(d, f, c);
        if op == best {
            out.insert(c);
        } else if better(op, best) {
            out = CardSet::from_card(c);
            best = op;
        }
    }
    Some((out, best))
}
/// `g:GetMinGroup(f)`: the cards with the smallest value, and the value (`None` if empty).
pub fn get_min_group(d: &mut Duel, g: &Group, f: ValueFn) -> Option<(Group, i64)> {
    extreme_group(d, g, f, |a, b| a < b)
}
pub fn get_max_group(d: &mut Duel, g: &Group, f: ValueFn) -> Option<(Group, i64)> {
    extreme_group(d, g, f, |a, b| a > b)
}
pub fn get_sum(d: &mut Duel, g: &Group, f: ValueFn) -> i64 {
    let mut s = 0i64;
    for c in public_order(d, g) {
        s = s.wrapping_add(get_operation_value(d, f, c));
    }
    s
}
/// `g:GetBitwiseAnd(f)` (always 0: the core starts from 0)
pub fn get_bitwise_and(d: &mut Duel, g: &Group, f: ValueFn) -> u64 {
    let mut t = 0u64;
    for c in public_order(d, g) {
        t &= get_operation_value(d, f, c) as u64;
    }
    t
}
pub fn get_bitwise_or(d: &mut Duel, g: &Group, f: ValueFn) -> u64 {
    let mut t = 0u64;
    for c in public_order(d, g) {
        t |= get_operation_value(d, f, c) as u64;
    }
    t
}
/// `g:GetClass(f)`: the distinct values, sorted.
pub fn get_class(d: &mut Duel, g: &Group, f: ValueFn) -> Vec<i64> {
    let mut s = std::collections::BTreeSet::new();
    for c in public_order(d, g) {
        s.insert(get_operation_value(d, f, c));
    }
    s.into_iter().collect()
}
pub fn get_class_count(d: &mut Duel, g: &Group, f: ValueFn) -> usize {
    get_class(d, g, f).len()
}
/// `g:Remove(f, ex)`: removes the matching cards (except the excluded ones).
pub fn remove(d: &mut Duel, g: &mut Group, f: Filter, ex: Exclude) {
    let cards = g.to_vec();
    let mut keep = CardSet::new();
    match ex {
        Exclude::Group(exg) => {
            let ex_cards = exg.to_vec();
            let mut pex = 0;
            for c in cards {
                let kept_by_ex = pex < ex_cards.len() && ex_cards[pex] == c;
                if kept_by_ex {
                    pex += 1;
                }
                if kept_by_ex || !check_matching(d, f, c) {
                    keep.insert(c);
                }
            }
        }
        _ => {
            for c in cards {
                if ex.contains(c) || !check_matching(d, f, c) {
                    keep.insert(c);
                }
            }
        }
    }
    *g = keep;
}
/// `g1 & g2`
pub fn intersection(g1: &Group, g2: &Group) -> Group {
    CardSet::from_iter(g1.iter().filter(|&c| g2.contains(c)))
}
/// `g1 + g2`
pub fn union(g1: &Group, g2: &Group) -> Group {
    let mut g = g1.clone();
    g.extend(g2.iter());
    g
}
/// `g1 - g2`
pub fn difference(g1: &Group, g2: &Group) -> Group {
    CardSet::from_iter(g1.iter().filter(|&c| !g2.contains(c)))
}
/// `g1 == g2` (Lua `__eq`: the sizes only)
pub fn lua_eq(g1: &Group, g2: &Group) -> bool {
    g1.len() == g2.len()
}
/// `g:Equal(g2)`
pub fn equal(g1: &Group, g2: &Group) -> bool {
    g1 == g2
}
pub fn is_contains(g: &Group, c: CardId) -> bool {
    g.contains(c)
}
/// `g:SearchCard(f)`: the first matching card in public order.
pub fn search_card(d: &mut Duel, g: &Group, f: Filter) -> Option<CardId> {
    public_order(d, g).into_iter().find(|&c| check_matching(d, f, c))
}
/// `g:Split(f, ex)`: (matching, the others and the excluded)
pub fn split(d: &mut Duel, g: &Group, f: Filter, ex: Exclude) -> (Group, Group) {
    let mut cset = g.clone();
    let mut notmatching = CardSet::new();
    match ex {
        Exclude::None => {}
        Exclude::Card(c) => {
            cset.remove(c);
            notmatching.insert(c);
        }
        Exclude::Group(exg) => {
            for c in exg.iter() {
                cset.remove(c);
                notmatching.insert(c);
            }
        }
    }
    let mut matching = CardSet::new();
    for c in cset.iter() {
        if check_matching(d, f, c) {
            matching.insert(c);
        } else {
            notmatching.insert(c);
        }
    }
    (matching, notmatching)
}
/// `g:Includes(g2)`
pub fn includes(g: &Group, g2: &Group) -> bool {
    g.len() >= g2.len() && g2.iter().all(|c| g.contains(c))
}
pub fn get_bin_class_count(d: &mut Duel, g: &Group, f: ValueFn) -> u32 {
    let mut er = 0u64;
    for c in public_order(d, g) {
        er |= get_operation_value(d, f, c) as u64;
    }
    er.count_ones()
}
