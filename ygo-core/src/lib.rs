//! A Rust port of ygopro-core (edo9300) with the project's patches: see DESIGN.md.
#![allow(clippy::needless_range_loop, clippy::collapsible_if, clippy::collapsible_else_if)]

pub mod card;
pub mod card2;
pub mod card3;
pub mod carddb;
pub mod constants;
pub mod duel;
pub mod effect;
pub mod field1;
pub mod field2;
pub mod field3;
pub mod lua;
pub mod scope;
pub mod script;
pub mod stubs;
pub mod types;
pub mod units;
