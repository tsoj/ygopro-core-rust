//! Fairy Knight Ingunar
use super::prelude::*;

pub const ID: u32 = 19684740;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    xyz::add_procedure(d, c, None, Some(6), 3, None, None, None, None, false, None);
    card::enable_revive_limit(d, c);
    // tohand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(2), None, None)));
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
}
/// `Duel.GetMatchingGroup(Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler())`
fn hand_group(d: &mut Duel, tp: u8, c: CardId) -> CardSet {
    duel::get_matching_group(d, Some(&|d, cc| card::is_able_to_hand(d, cc, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into())
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, cc| card::is_able_to_hand(d, cc, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    }
    let g = hand_group(d, a.tp, c);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g.clone()), g.len() as u8, 0, 0);
    duel::set_chain_limit(d, lua_fn(chlimit));
    true
}
/// `s.chlimit(e,ep,tp)`: `return tp==ep`
fn chlimit(d: &mut Duel, a: Args) -> bool {
    let _ = d;
    a.p(2) == a.p(1)
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let g = hand_group(d, a.tp, c);
    duel::send_to_hand(d, g, None, REASON_EFFECT, None);
}
