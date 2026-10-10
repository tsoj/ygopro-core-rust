//! Demise of the Land
use super::prelude::*;

pub const ID: u32 = 48934760;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, 1 - tp), 1, NO_EX)
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    if !card::is_type(d, c, TYPE_FIELD) {
        return false;
    }
    let te = card::get_activate_effect(d, c).first().copied().nil();
    effect::is_activatable(d, te, tp, true, true)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,tp) end
    if a.chk == Some(0) {
        let tp = a.tp;
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 0));
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    duel::activate_field_spell(d, tc, a, None);
}
