//! What card functions need: their arguments, and wrappers that make `LuaFn`s from Rust functions.

pub use ygo_core::duel::Duel;
pub use ygo_core::types::*;

/// The arguments of an effect's condition/cost/target/operation: `(e, tp, eg, ep, ev, re, r, rp, chk)`
/// and what the caller passes after them.
#[derive(Clone, Debug)]
pub struct EffArgs {
    pub e: EffectId,
    pub tp: u8,
    pub eg: Option<CardSet>,
    pub ep: u8,
    pub ev: u32,
    pub re: Option<EffectId>,
    pub r: u32,
    pub rp: u8,
    /// 0 when a cost or target function only checks; operations have none (`None`).
    pub chk: Option<i64>,
    pub extra: Vec<Value>,
}

fn group_of(v: &Value) -> Option<CardSet> {
    match v {
        Value::Group(g) => Some(g.clone()),
        Value::Card(c) => Some(CardSet::from_card(*c)),
        _ => None,
    }
}

impl EffArgs {
    /// From the arguments of a condition or operation (8 values).
    pub fn new(a: &[Value]) -> EffArgs {
        let get = |i: usize| a.get(i).cloned().unwrap_or(Value::Nil);
        EffArgs {
            e: get(0).as_effect().expect("effect function without an effect"),
            tp: get(1).int() as u8,
            eg: group_of(&get(2)),
            ep: get(3).int() as u8,
            ev: get(4).int() as u32,
            re: get(5).as_effect(),
            r: get(6).int() as u32,
            rp: get(7).int() as u8,
            chk: None,
            extra: a.get(8..).map_or(Vec::new(), |x| x.to_vec()),
        }
    }
    /// From the arguments of a cost or target function (9 values: `chk` last).
    pub fn with_chk(a: &[Value]) -> EffArgs {
        let mut x = EffArgs::new(a);
        x.chk = Some(a.get(8).map_or(0, |v| v.int()));
        x.extra = a.get(9..).map_or(Vec::new(), |x| x.to_vec());
        x
    }
    /// `chkc` of a target function: the card the core asks about (`Duel.CheckChainTarget`).
    pub fn chkc(&self) -> Option<CardId> {
        self.extra.first().and_then(|v| v.as_card())
    }
    /// The event cards (empty if there are none).
    pub fn eg(&self) -> CardSet {
        self.eg.clone().unwrap_or_default()
    }
}

/// An operation (`function(e,tp,eg,ep,ev,re,r,rp)` returning nothing).
pub fn operation(f: impl Fn(&mut Duel, &EffArgs) + Send + Sync + 'static) -> LuaFn {
    LuaFn::new(move |d, a| {
        f(d, &EffArgs::new(a));
        Value::Nil
    })
}
/// A condition (`function(e,tp,eg,ep,ev,re,r,rp)` returning a boolean).
pub fn condition(f: impl Fn(&mut Duel, &EffArgs) -> bool + Send + Sync + 'static) -> LuaFn {
    LuaFn::new(move |d, a| Value::Bool(f(d, &EffArgs::new(a))))
}
/// A cost or target function (`function(e,tp,eg,ep,ev,re,r,rp,chk)`): with `chk == 0` it returns
/// whether it can be done.
pub fn cost_target(f: impl Fn(&mut Duel, &EffArgs) -> bool + Send + Sync + 'static) -> LuaFn {
    LuaFn::new(move |d, a| Value::Bool(f(d, &EffArgs::with_chk(a))))
}
/// A function of a card (`function(e, c)`, e.g. the target of a field effect). A nil `c` (the
/// core passes none, e.g. to the value of EFFECT_CANNOT_BE_SYNCHRO_MATERIAL) gives false, as the
/// scripts' `c and ...` / `if not c then return false end` guards do.
pub fn effect_card_fn(f: impl Fn(&mut Duel, EffectId, CardId) -> bool + Send + Sync + 'static) -> LuaFn {
    LuaFn::new(move |d, a| {
        let e = a.first().and_then(|v| v.as_effect()).unwrap();
        let Some(c) = a.get(1).and_then(|v| v.as_card()) else { return Value::Bool(false) };
        Value::Bool(f(d, e, c))
    })
}
/// A value function of a card (`function(e, c)` returning a number).
pub fn effect_card_value(f: impl Fn(&mut Duel, EffectId, CardId) -> i64 + Send + Sync + 'static) -> LuaFn {
    LuaFn::new(move |d, a| {
        let e = a.first().and_then(|v| v.as_effect()).unwrap();
        let c = a.get(1).and_then(|v| v.as_card()).unwrap();
        Value::Int(f(d, e, c))
    })
}
/// `aux.TRUE`
pub fn aux_true() -> LuaFn {
    LuaFn::new(|_, _| Value::Bool(true))
}
/// `aux.FALSE`
pub fn aux_false() -> LuaFn {
    LuaFn::new(|_, _| Value::Bool(false))
}

/// A Lua number (possibly a float, e.g. `atk/2`) as the core reads it into an integer argument or
/// value: `std::round`, half away from zero. `lua_int(atk as f64 / 2.0)`.
pub fn lua_int(x: f64) -> i64 {
    x.round() as i64
}

fn global_key(name: &str) -> (u32, u64) {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in name.bytes() {
        h = (h ^ b as u64).wrapping_mul(0x100000001b3);
    }
    (crate::ns::LUA_GLOBALS, h)
}
/// A Lua global variable shared between scripts (`_replace_count`); `Value::Nil` if never set.
pub fn lua_global(d: &Duel, name: &str) -> Value {
    d.script_var(global_key(name))
}
/// `name = v` for a Lua global variable shared between scripts.
pub fn set_lua_global(d: &mut Duel, name: &str, v: Value) {
    d.set_script_var(global_key(name), v);
}

/// A card script's own state (`s.<i>`, kept in the duel): the variable `i` of the script `code`.
pub fn s_var(d: &Duel, code: u32, i: u64) -> Value {
    d.script_var((code, i))
}
pub fn set_s_var(d: &mut Duel, code: u32, i: u64, v: Value) {
    d.set_script_var((code, i), v);
}

/// The arguments of a Lua function by position, read as the function's parameters (a missing or
/// mistyped argument is a Lua error).
#[derive(Clone, Copy)]
pub struct Args<'a>(pub &'a [Value]);
impl Args<'_> {
    pub fn v(&self, i: usize) -> &Value {
        self.0.get(i).unwrap_or(&Value::Nil)
    }
    /// An effect argument
    pub fn e(&self, i: usize) -> EffectId {
        self.v(i).as_effect().nil()
    }
    /// A card argument
    pub fn c(&self, i: usize) -> CardId {
        self.v(i).as_card().nil()
    }
    /// A card argument that may be nil
    pub fn card(&self, i: usize) -> Option<CardId> {
        self.v(i).as_card()
    }
    /// A player argument
    pub fn p(&self, i: usize) -> u8 {
        self.v(i).int() as u8
    }
    pub fn int(&self, i: usize) -> i64 {
        self.v(i).int()
    }
    /// A group argument (a card as a group of one; empty if nil)
    pub fn g(&self, i: usize) -> CardSet {
        group_of(self.v(i)).unwrap_or_default()
    }
}
/// A function with other arguments than the effect functions (e.g. the value of a field effect:
/// `function(e, re, tp)`), returning what `f` returns.
pub fn lua_fn<R: Into<Value>>(f: impl Fn(&mut Duel, Args) -> R + Send + Sync + 'static) -> LuaFn {
    LuaFn::new(move |d, a| f(d, Args(a)).into())
}

/// Indexing `nil` in Lua is an error: `x.nil()` is `x`, or that error.
pub trait Nil<T> {
    fn nil(self) -> T;
}
impl<T> Nil<T> for Option<T> {
    #[track_caller]
    fn nil(self) -> T {
        match self {
            Some(x) => x,
            None => crate::lua_error("attempt to index a nil value"),
        }
    }
}

/// A filter kept inside an effect function (an `aux.*` helper's argument stored in a closure).
pub type CardFilter = std::sync::Arc<dyn Fn(&mut Duel, CardId) -> bool + Send + Sync>;
