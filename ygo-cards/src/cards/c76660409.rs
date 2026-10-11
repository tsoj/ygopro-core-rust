//! Pendulum Back
use super::prelude::*;

pub const ID: u32 = 76660409;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TOHAND);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_field_card(d, a.tp, LOCATION_PZONE, 0).is_some() && duel::get_field_card(d, a.tp, LOCATION_PZONE, 1).is_some()
}
/// `s.filter(c,lsc,rsc)`
fn filter(d: &mut Duel, c: CardId, lsc: u32, rsc: u32) -> bool {
    let lv = card::get_level(d, c);
    lv > lsc && lv < rsc && card::is_able_to_hand(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local lsc=Duel.GetFieldCard(tp,LOCATION_PZONE,0):GetLeftScale()
    let lpz = duel::get_field_card(d, tp, LOCATION_PZONE, 0).nil();
    let mut lsc = card::get_left_scale(d, lpz);
    // local rsc=Duel.GetFieldCard(tp,LOCATION_PZONE,1):GetRightScale()
    let rpz = duel::get_field_card(d, tp, LOCATION_PZONE, 1).nil();
    let mut rsc = card::get_right_scale(d, rpz);
    // if lsc>rsc then lsc,rsc=rsc,lsc end
    if lsc > rsc {
        std::mem::swap(&mut lsc, &mut rsc);
    }
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && filter(d, chkc, lsc, rsc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, lsc, rsc)), tp, LOCATION_GRAVE, 0, 2, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, lsc, rsc)), tp, LOCATION_GRAVE, 0, 2, 2, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 2, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // local sg=g:Filter(Card.IsRelateToEffect,nil,e)
    let e = a.e;
    let sg = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, e), NO_EX);
    if !sg.is_empty() {
        duel::send_to_hand(d, sg, None, REASON_EFFECT, None);
    }
}
