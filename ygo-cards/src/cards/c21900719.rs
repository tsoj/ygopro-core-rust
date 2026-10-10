//! Twin Swords of Flashing Light - Tryce
use super::prelude::*;

pub const ID: u32 = 21900719;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,nil,nil,s.cost)
    aux::add_equip_procedure(d, c, None, None, None, Some(cost_target(cost)), None, None, None, None);
    // Atk down
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e2, -500);
    card::register_effect(d, c, e2, false);
    // Double Attack
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e3, EFFECT_EXTRA_ATTACK);
    effect::set_value(d, e3, 1);
    card::register_effect(d, c, e3, false);
}
/// `s.cost`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToGraveAsCost,tp,LOCATION_HAND,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e);
        return duel::is_existing_matching_card(d, Some(&card::is_able_to_grave_as_cost), a.tp, LOCATION_HAND, 0, 1, h.into());
    }
    // Duel.DiscardHand(tp,Card.IsAbleToGraveAsCost,1,1,REASON_COST)
    duel::discard_hand(d, a.tp, Some(&card::is_able_to_grave_as_cost), 1, 1, REASON_COST, NO_EX);
    true
}
