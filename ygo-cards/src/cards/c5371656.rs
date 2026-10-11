//! Sword of the Soul-Eater
use super::prelude::*;

pub const ID: u32 = 5371656;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,0,s.filter,nil,s.cost,s.target,s.operation)
    aux::add_equip_procedure(
        d,
        c,
        Some(0),
        Some(aux::equip_filter_fn(|d, x, _e, _tp| filter(d, x))),
        None,
        Some(cost_target(cost)),
        Some(std::sync::Arc::new(target)),
        Some(operation(op)),
        None,
        None,
    );
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPE_NORMAL) && card::is_level_below(d, c, 3)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    effect::set_label(d, a.e, &[1]);
    if a.chk == Some(0) {
        return true;
    }
    true
}
/// `s.rfilter(c)`
fn rfilter(d: &mut Duel, c: CardId) -> bool {
    let tpe = card::get_type(d, c);
    (tpe & TYPE_NORMAL) != 0 && (tpe & TYPE_TOKEN) == 0 && card::is_releasable(d, c, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,tc,chk)`: the equip target function, called with the chosen monster
fn target(d: &mut Duel, a: &EffArgs, tc: Option<CardId>) {
    let label = effect::get_label(d, a.e);
    effect::set_label(d, a.e, &[0]);
    if label == 1 {
        let ex = tc.map_or(NO_EX, |t| t.into());
        let rg = duel::get_matching_group(d, Some(&rfilter), a.tp, LOCATION_MZONE, 0, ex);
        let n = rg.len();
        duel::release(d, rg, REASON_COST, None);
        duel::set_target_param(d, (n * 1000) as u32);
    } else {
        duel::set_target_param(d, 0);
    }
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d);
    if card::is_relate_to_effect(d, c, a.e) {
        if let Some(tc) = tc {
            if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && card::is_controler(d, tc, a.tp) {
                let e1 = effect::create_effect(d, c);
                effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
                effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
                let param = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
                effect::set_value(d, e1, param);
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                card::register_effect(d, c, e1, false);
            }
        }
    }
}
