//! Basic types shared by the engine: ids for cards and effects, groups, the values passed to
//! card functions (the Lua values of ygopro-core), and card data.

use std::fmt;
use std::sync::Arc;

use crate::duel::Duel;

/// A card of the duel: index into `Duel::cards`, `cardid - 1` of ygopro-core (cards are numbered
/// in creation order, and sets of cards are ordered by it).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CardId(pub u32);

/// An effect: index into `Duel::effects`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct EffectId(pub u32);

/// A set of cards ordered by card id (ygopro-core's `card_set`, and the container of a Lua group).
#[derive(Clone, Default, PartialEq, Eq, Hash, Debug)]
pub struct CardSet(Vec<CardId>);

impl CardSet {
    pub fn new() -> Self {
        CardSet(Vec::new())
    }
    pub fn from_card(c: CardId) -> Self {
        CardSet(vec![c])
    }
    pub fn from_iter<I: IntoIterator<Item = CardId>>(it: I) -> Self {
        let mut v: Vec<CardId> = it.into_iter().collect();
        v.sort_unstable();
        v.dedup();
        CardSet(v)
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    #[inline]
    pub fn contains(&self, c: CardId) -> bool {
        self.0.binary_search(&c).is_ok()
    }
    /// Inserts `c`; returns whether it was new (like `std::set::insert(...).second`).
    pub fn insert(&mut self, c: CardId) -> bool {
        match self.0.binary_search(&c) {
            Ok(_) => false,
            Err(i) => {
                self.0.insert(i, c);
                true
            }
        }
    }
    /// Removes `c`; returns whether it was there.
    pub fn remove(&mut self, c: CardId) -> bool {
        match self.0.binary_search(&c) {
            Ok(i) => {
                self.0.remove(i);
                true
            }
            Err(_) => false,
        }
    }
    pub fn clear(&mut self) {
        self.0.clear();
    }
    pub fn extend<I: IntoIterator<Item = CardId>>(&mut self, it: I) {
        for c in it {
            self.insert(c);
        }
    }
    pub fn iter(&self) -> std::iter::Copied<std::slice::Iter<'_, CardId>> {
        self.0.iter().copied()
    }
    pub fn first(&self) -> Option<CardId> {
        self.0.first().copied()
    }
    pub fn as_slice(&self) -> &[CardId] {
        &self.0
    }
    pub fn retain<F: FnMut(&CardId) -> bool>(&mut self, f: F) {
        self.0.retain(f);
    }
    pub fn to_vec(&self) -> Vec<CardId> {
        self.0.clone()
    }
}

impl<'a> IntoIterator for &'a CardSet {
    type Item = CardId;
    type IntoIter = std::iter::Copied<std::slice::Iter<'a, CardId>>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().copied()
    }
}

/// A Lua group. Groups are values here (see DESIGN.md).
pub type Group = CardSet;

/// A function of a card script (a Lua function reference of ygopro-core).
#[derive(Clone)]
pub struct LuaFn(pub Arc<dyn Fn(&mut Duel, &[Value]) -> Value + Send + Sync>);

impl LuaFn {
    pub fn new<F: Fn(&mut Duel, &[Value]) -> Value + Send + Sync + 'static>(f: F) -> Self {
        LuaFn(Arc::new(f))
    }
    #[inline]
    pub fn call(&self, d: &mut Duel, args: &[Value]) -> Value {
        (self.0)(d, args)
    }
    pub fn ptr_eq(&self, other: &LuaFn) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl fmt::Debug for LuaFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LuaFn({:p})", Arc::as_ptr(&self.0) as *const ())
    }
}

/// A Lua value: parameters and results of card functions, and what effects keep as labels.
#[derive(Clone, Debug, Default)]
pub enum Value {
    #[default]
    Nil,
    Bool(bool),
    Int(i64),
    Card(CardId),
    Group(Group),
    Effect(EffectId),
    Fn(LuaFn),
    /// Several results (a Lua function returning more than one value), or a table of values.
    Multi(Vec<Value>),
}

impl Value {
    pub fn card(c: Option<CardId>) -> Value {
        c.map_or(Value::Nil, Value::Card)
    }
    pub fn effect(e: Option<EffectId>) -> Value {
        e.map_or(Value::Nil, Value::Effect)
    }
    /// Lua truthiness (`lua_toboolean`): only nil and false are false.
    pub fn truthy(&self) -> bool {
        match self {
            Value::Nil | Value::Bool(false) => false,
            Value::Multi(v) => v.first().is_some_and(|x| x.truthy()),
            _ => true,
        }
    }
    /// The integer value as the interpreter reads it (booleans count as 0/1, others as 0).
    pub fn int(&self) -> i64 {
        match self {
            Value::Int(i) => *i,
            Value::Bool(b) => *b as i64,
            Value::Multi(v) => v.first().map_or(0, |x| x.int()),
            _ => 0,
        }
    }
    /// All results as integers (`get_function_value(..., std::vector&)`).
    pub fn ints(&self) -> Vec<i64> {
        match self {
            Value::Multi(v) => v.iter().map(|x| x.int()).collect(),
            Value::Nil => vec![],
            x => vec![x.int()],
        }
    }
    pub fn as_card(&self) -> Option<CardId> {
        match self {
            Value::Card(c) => Some(*c),
            _ => None,
        }
    }
    pub fn as_effect(&self) -> Option<EffectId> {
        match self {
            Value::Effect(e) => Some(*e),
            _ => None,
        }
    }
    pub fn as_group(&self) -> Option<&Group> {
        match self {
            Value::Group(g) => Some(g),
            _ => None,
        }
    }
    pub fn as_fn(&self) -> Option<&LuaFn> {
        match self {
            Value::Fn(f) => Some(f),
            _ => None,
        }
    }
    pub fn is_nil(&self) -> bool {
        matches!(self, Value::Nil)
    }
}

/// Set codes (archetypes) of a card: a small sorted set, inline (cheap to copy).
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct SetCodes {
    len: u8,
    codes: [u16; 15],
}

impl SetCodes {
    pub fn insert(&mut self, sc: u16) {
        let n = self.len as usize;
        match self.codes[..n].binary_search(&sc) {
            Ok(_) => {}
            Err(i) => {
                if n < self.codes.len() {
                    self.codes.copy_within(i..n, i + 1);
                    self.codes[i] = sc;
                    self.len += 1;
                }
            }
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = u16> + '_ {
        self.codes[..self.len as usize].iter().copied()
    }
    pub fn clear(&mut self) {
        self.len = 0;
    }
    pub fn len(&self) -> usize {
        self.len as usize
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// `card_data`: the printed properties of a card.
#[derive(Clone, Copy, Default, Debug)]
pub struct CardData {
    pub code: u32,
    pub alias: u32,
    pub setcodes: SetCodes,
    pub type_: u32,
    pub level: u32,
    pub attribute: u32,
    pub race: u64,
    pub attack: i32,
    pub defense: i32,
    pub lscale: u32,
    pub rscale: u32,
    pub link_marker: u32,
}

/// `loc_info`
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct LocInfo {
    pub controler: u8,
    pub location: u8,
    pub sequence: u32,
    pub position: u32,
}

/// A response of a player (the bytes given to OCG_DuelSetResponse).
pub type Response = Vec<u8>;
