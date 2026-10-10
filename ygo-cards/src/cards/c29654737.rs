//! Amazoness Chain Master
use super::prelude::*;

pub const ID: u32 = 29654737;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // get card
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(1500))));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLocation(LOCATION_GRAVE) and e:GetHandler():IsReason(REASON_BATTLE)
    //     and Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)~=0
    let h = effect::get_handler(d, a.e).nil();
    card::is_location(d, h, LOCATION_GRAVE) && card::is_reason(d, h, REASON_BATTLE) && duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) != 0
}
fn op(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_HAND);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.ConfirmCards(tp,g)
        duel::confirm_cards(d, a.tp, g.clone());
        // local tg=g:Filter(Card.IsMonster,nil)
        let tg = group::filter(d, &g, &card::is_monster, NO_EX);
        // if #tg>0 then
        if !tg.is_empty() {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
            // local sg=tg:Select(tp,1,1,nil)
            let sg = group::select(d, &tg, a.tp, 1, 1, false, NO_EX).nil();
            // Duel.SendtoHand(sg,tp,REASON_EFFECT)
            duel::send_to_hand(d, sg, Some(a.tp), REASON_EFFECT, None);
        }
        // Duel.ShuffleHand(1-tp)
        duel::shuffle_hand(d, 1 - a.tp);
    }
}
