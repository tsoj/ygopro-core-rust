//! The API of the core: `ocg` is `ocgapi.cpp` (creating and running duels); the others are the
//! functions card scripts use (`libeffect.cpp`, `libcard.cpp`, `libgroup.cpp`, `libduel.cpp`,
//! `libdebug.cpp`), as free functions named after the Lua ones: `e:SetType(v)` is
//! `effect::set_type(d, e, v)`, `Duel.Destroy(g, r)` is `duel::destroy(d, g, r, ..)`.

pub mod effect;
pub mod ocg;

pub use ocg::{NewCardInfo, QueryInfo};
