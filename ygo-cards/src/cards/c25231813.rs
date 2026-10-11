//! Silver Wing
use super::prelude::*;

pub const ID: u32 = 25231813;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,s.filter)
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, x, _e, _tp| filter(d, x))), None, None, None, None, None, None);
    // indestructable (battle, twice)
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_COUNT);
    effect::set_count_limit(d, e3, 2, 0, 0, 0);
    effect::set_value_fn(d, e3, lua_fn(indval));
    card::register_effect(d, c, e3, false);
    // destroy replace
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_EQUIP);
    effect::set_code(d, e4, EFFECT_DESTROY_REPLACE);
    effect::set_target(d, e4, Some(cost_target(reptg2)));
    effect::set_operation(d, e4, Some(operation(repop2)));
    card::register_effect(d, c, e4, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_level_above(d, c, 8) && card::is_race(d, c, RACE_DRAGON) && card::is_type(d, c, TYPE_SYNCHRO)
}
/// `s.indval(e,re,r,rp)`
fn indval(_d: &mut Duel, a: Args) -> bool {
    (a.int(2) as u32 & REASON_BATTLE) != 0
}
/// `s.reptg2(e,tp,...,chk)`
fn reptg2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        let et = card::get_equip_target(d, c).nil();
        return card::is_reason(d, et, REASON_EFFECT)
            && !card::is_reason(d, et, REASON_REPLACE)
            && card::is_destructable(d, c, Some(a.e))
            && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED);
    }
    if duel::select_effect_yes_no(d, tp, c, Some(96)) {
        card::set_status(d, c, STATUS_DESTROY_CONFIRMED, true);
        true
    } else {
        false
    }
}
/// `s.repop2(e,tp,...,chk)`
fn repop2(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    card::set_status(d, c, STATUS_DESTROY_CONFIRMED, false);
    duel::destroy(d, c, REASON_EFFECT | REASON_REPLACE, None, None);
}
