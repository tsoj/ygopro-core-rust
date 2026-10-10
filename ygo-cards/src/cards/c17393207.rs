//! Gravekeeper's Commandant
use super::prelude::*;

pub const ID: u32 = 17393207;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_NECROVALLEY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // search
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_cost(d, e1, Some(cost::self_discard_to_grave()));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_NECROVALLEY) and c:IsAbleToHand()
    card::is_code(d, c, &[CARD_NECROVALLEY]) && card::is_able_to_hand(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetFirstMatchingCard(s.filter,tp,LOCATION_DECK,0,nil)
    let tg = duel::get_first_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, NO_EX);
    // if tg then
    if let Some(tg) = tg {
        // Duel.SendtoHand(tg,nil,REASON_EFFECT)
        duel::send_to_hand(d, tg, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,tg)
        duel::confirm_cards(d, 1 - a.tp, tg);
    }
}
