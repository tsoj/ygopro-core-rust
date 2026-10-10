//! The interpreter interface of ygopro-core (`interpreter.h`) for card functions written in Rust:
//! parameters are collected with `add_param`, then a function is called with them.

use crate::duel::Duel;
use crate::types::*;

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
    pub fn call_function(&mut self, f: &LuaFn, _param_count: u32) -> Value {
        let args = std::mem::take(&mut self.params);
        f.call(self, &args)
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
            f(self, c);
        }
    }
}
