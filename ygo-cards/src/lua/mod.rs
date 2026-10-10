//! The API as card scripts see it: the core's functions (`ygo_core::api`) with the changes the
//! global Lua scripts make (`proc_workaround.lua`, `utility.lua`, `chain.lua`,
//! `proc_fusion_spell.lua` redefine some functions; those are defined here, the rest is the core's).

pub use ygo_core::api::{CardOrGroup, Exclude, Filter, NO_EX, ValueFn};

pub mod card;
pub mod debug;
pub mod duel;
pub mod effect;
pub mod group;

/// `proc_workaround.lua`: a "cannot (special) summon" effect first applies the properties the
/// summoning procedure assumes (`label_object` = a table with the assume function), then checks.
pub(crate) fn wrap_cannot_summon_target(d: &mut ygo_core::duel::Duel, e: ygo_core::types::EffectId) {
    use crate::constants::*;
    use ygo_core::types::*;
    let code = effect::get_code(d, e);
    if code != EFFECT_CANNOT_SUMMON && code != EFFECT_CANNOT_SPECIAL_SUMMON {
        return;
    }
    let Some(base_tg) = effect::get_target(d, e) else { return };
    let assume = |d: &mut ygo_core::duel::Duel, pe: Option<EffectId>, c: CardId| {
        if let Some(pe) = pe {
            if let Value::Multi(t) = effect::get_label_object(d, pe) {
                if let Some(Value::Fn(f)) = t.first() {
                    f.call(d, &[Value::Card(c)]);
                }
            }
        }
    };
    let wrapped = LuaFn::new(move |d, a| {
        // (e, c, sump, sumtype, sumpos, targetp, sum_eff, sum_proc_eff)
        let c = a.get(1).and_then(|v| v.as_card());
        let sumtype = a.get(3).map_or(0, |v| v.int());
        let sum_proc_eff = a.get(7).and_then(|v| v.as_effect());
        if let Some(c) = c {
            assume(d, sum_proc_eff, c);
            if code == EFFECT_CANNOT_SUMMON && sumtype == SUMMON_TYPE_NORMAL as i64 + 1 {
                let ns = ygo_core::api::card::is_has_effect(d, c, EFFECT_SUMMON_PROC, None);
                assume(d, ns.first().copied(), c);
            }
        }
        base_tg.call(d, a)
    });
    effect::set_target(d, e, Some(wrapped));
}
