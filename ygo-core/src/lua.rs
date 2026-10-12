//! The interpreter interface of ygopro-core (`interpreter.h`) for card functions written in Rust:
//! parameters are collected with `add_param`, then a function is called with them.

use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};

use crate::duel::Duel;
use crate::types::*;

/// The payload of a Lua error (`lua_error`): it unwinds to the card function call that is running,
/// which fails (the error is logged, the call returns nil).
pub struct LuaError(pub String);

/// `luaL_error`
pub fn lua_error(msg: impl Into<String>) -> ! {
    resume_unwind(Box::new(LuaError(msg.into())))
}

/// What fails a card function: a Lua error, or a panic of the Rust port (a bug, typically where
/// the Lua fails too, e.g. on a nil value in a state normal play never reaches, as an activation
/// the belief replay forced), which fails the call like a Lua error instead of the whole engine.
/// `Err`: an unwind that is not a failure (a card function interrupted to wait, `scope.rs`).
pub fn script_failure(payload: Box<dyn std::any::Any + Send>) -> Result<String, Box<dyn std::any::Any + Send>> {
    let payload = match payload.downcast::<LuaError>() {
        Ok(e) => return Ok(e.0),
        Err(p) => p,
    };
    if payload.is::<crate::scope::Interrupt>() || payload.is::<crate::scope::StopHere>() {
        return Err(payload);
    }
    if let Some(s) = payload.downcast_ref::<String>() {
        return Ok(format!("engine error: {s}"));
    }
    if let Some(s) = payload.downcast_ref::<&str>() {
        return Ok(format!("engine error: {s}"));
    }
    Err(payload)
}

/// `scriptlib::check_action_permission`
pub fn check_action_permission(d: &Duel) {
    if d.no_action != 0 {
        lua_error("Action is not allowed here.");
    }
}

impl Duel {
    #[inline]
    pub fn add_param(&mut self, v: Value) {
        self.params.push(v);
    }
    #[inline]
    pub fn add_param_front(&mut self, v: Value) {
        self.params.insert(0, v);
    }
    /// Calls `f` with the collected parameters (all of them, as `interpreter::push_param`).
    /// (as `interpreter::call_lua`: no actions during it, `no_action`; operations run through the
    /// coroutine path, `protected_call` alone)
    pub fn call_function(&mut self, f: &LuaFn, _param_count: u32) -> Value {
        let args = std::mem::take(&mut self.params);
        self.no_action += 1;
        let r = self.protected_call(f, &args).unwrap_or(Value::Nil);
        self.no_action -= 1;
        r
    }
    /// Runs a card function; `None` if it raised a Lua error (logged, as `lua_pcall` fails).
    pub fn protected_call(&mut self, f: &LuaFn, args: &[Value]) -> Option<Value> {
        self.call_depth += 1;
        let r = catch_unwind(AssertUnwindSafe(|| {
            crate::api::script_step(self);
            f.call(self, args)
        }));
        self.call_depth -= 1;
        if self.call_depth == 0 {
            self.restore_assumes();
        }
        match r {
            Ok(v) => Some(v),
            Err(payload) => match script_failure(payload) {
                Ok(msg) => {
                    self.handle_message(&msg, 0);
                    None
                }
                Err(other) => resume_unwind(other),
            },
        }
    }
    /// `interpreter::check_condition`: no function means true.
    pub fn check_condition(&mut self, f: Option<&LuaFn>, param_count: u32) -> bool {
        match f {
            None => {
                self.params.clear();
                true
            }
            Some(f) => self.call_function(f, param_count).truthy(),
        }
    }
    /// `interpreter::get_function_value`
    pub fn get_function_value(&mut self, f: Option<&LuaFn>, param_count: u32) -> i64 {
        match f {
            None => {
                self.params.clear();
                0
            }
            Some(f) => self.call_function(f, param_count).int(),
        }
    }
    /// `interpreter::get_function_value(..., std::vector&)`
    pub fn get_function_values(&mut self, f: Option<&LuaFn>, param_count: u32, result: &mut Vec<i64>) -> bool {
        match f {
            None => {
                self.params.clear();
                false
            }
            Some(f) => {
                result.extend(self.call_function(f, param_count).ints());
                true
            }
        }
    }
    /// `call_card_function(pcard, "initial_effect")` / `call_code_function(code, "initial_effect")`
    pub fn call_initial_effect(&mut self, code: u32, c: CardId) {
        self.params.clear();
        if let Some(f) = self.scripts.get(code) {
            // (`call_card_function` goes through `call_lua`)
            self.no_action += 1;
            f(self, c);
            self.no_action -= 1;
        }
    }
}
