//! Mirror Force Dragon
use super::prelude::*;

pub const ID: u32 = 84687358;

pub const INFO: ScriptInfo = ScriptInfo { material_trap: Some(44095762), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Special Summon Condition
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_UNCOPYABLE, 0);
    card::register_effect(d, c, e1, false);
    // Destroy monsters
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_BE_BATTLE_TARGET);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(descon1)));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_BECOME_TARGET);
    effect::set_condition(d, e3, Some(condition(descon2)));
    card::register_effect(d, c, e3, false);
}
/// `s.tgfilter(c,tp)`
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_location(d, c, LOCATION_MZONE) && card::is_controler(d, c, tp)
}
/// `s.descon1(e,tp,eg,ep,ev,re,r,rp)`
fn descon1(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| tgfilter(d, c, tp), 1, NO_EX)
}
/// `s.descon2(e,tp,eg,ep,ev,re,r,rp)`
fn descon2(d: &mut Duel, a: &EffArgs) -> bool {
    if a.rp != 1 - a.tp {
        return false;
    }
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| tgfilter(d, c, tp), 1, NO_EX)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, None, a.tp, 0, LOCATION_ONFIELD, 1, NO_EX) && !card::is_status(d, h, STATUS_CHAINING);
    }
    let g = duel::get_matching_group(d, None, a.tp, 0, LOCATION_ONFIELD, NO_EX);
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, None, a.tp, 0, LOCATION_ONFIELD, NO_EX);
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
