//! The summon procedures (`proc_*.lua`): `Link.*`, `Xyz.*`, `Synchro.*`, ...

use crate::aux::ExtraMaterial;
use crate::prelude::*;
use std::sync::Arc;

pub mod link;
pub mod xyz;

/// A material filter `function(c, scard, sumtype, tp)` (e.g. `aux.FilterBoolFunctionEx(Card.IsRace,
/// RACE_DRAGON)` is `mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_DRAGON, sc, st, Some(tp)))`).
pub type MatFilter = Arc<dyn Fn(&mut Duel, CardId, Option<CardId>, u64, u8) -> bool + Send + Sync>;
pub fn mat_filter(f: impl Fn(&mut Duel, CardId, Option<CardId>, u64, u8) -> bool + Send + Sync + 'static) -> MatFilter {
    Arc::new(f)
}
/// A check of all the materials `function(g, sc, sumtype, tp)` (`specialchk`, ...).
pub type GroupCheck = Arc<dyn Fn(&mut Duel, &CardSet, CardId, u64, u8) -> bool + Send + Sync>;
pub fn group_check(f: impl Fn(&mut Duel, &CardSet, CardId, u64, u8) -> bool + Send + Sync + 'static) -> GroupCheck {
    Arc::new(f)
}

/// `ex[2](c, ex[3], tp, sg, mg, lc, ex[1], chk)`: an extra-material entry's check
pub(crate) fn extra_check(d: &mut Duel, ex: &ExtraMaterial, c: CardId, tp: u8, sg: &CardSet, mg: &CardSet, lc: CardId, chk: i64) -> bool {
    match &ex.check {
        None => true,
        Some(f) => f
            .call(d, &[Value::Card(c), Value::Effect(ex.effect), Value::Int(tp as i64), Value::Group(sg.clone()), Value::Group(mg.clone()), Value::Card(lc), Value::Group(ex.group.clone()), Value::Int(chk)])
            .truthy(),
    }
}
/// Extra-material entries in a label object (a table `{eg, efun, te}` per entry).
pub(crate) fn extra_to_value(t: &[ExtraMaterial]) -> Value {
    Value::Multi(t.iter().map(|ex| Value::Multi(vec![Value::Group(ex.group.clone()), ex.check.clone().map_or(Value::Nil, Value::Fn), Value::Effect(ex.effect)])).collect())
}
pub(crate) fn extra_from_value(v: &Value) -> Vec<ExtraMaterial> {
    let Value::Multi(t) = v else { return Vec::new() };
    t.iter()
        .filter_map(|x| match x {
            Value::Multi(e) => Some(ExtraMaterial { group: e[0].as_group().cloned().unwrap_or_default(), check: e[1].as_fn().cloned(), effect: e[2].as_effect()? }),
            _ => None,
        })
        .collect()
}
/// The operation of the extra-material entries used (`ex[3]:GetValue()(1, sumtype, ex[3], ex[1]&g, c, tp)`, then their count limit).
pub(crate) fn use_extra_materials(d: &mut Duel, filt: &[ExtraMaterial], g: &CardSet, sumtype: u32, c: CardId, tp: u8) {
    use crate::lua::{effect, group};
    for ex in filt {
        match effect::get_value(d, ex.effect) {
            Value::Fn(f) => {
                f.call(d, &[Value::Int(1), Value::Int(sumtype as i64), Value::Effect(ex.effect), Value::Group(group::intersection(&ex.group, g)), Value::Card(c), Value::Int(tp as i64)]);
            }
            _ => crate::lua_error("attempt to call a number value"),
        }
        if effect::check_count_limit(d, ex.effect, tp) {
            effect::use_count_limit(d, ex.effect, tp, 1, false);
        }
    }
}
