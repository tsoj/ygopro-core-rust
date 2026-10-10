//! Ojamagic
use super::prelude::*;

pub const ID: u32 = 24643836;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[12482652, 42941100, 79335209], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 each of "Ojama Green", "Ojama Yellow", and "Ojama Black" from your Deck to your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    effect::set_condition(d, e1, Some(condition(|d, a| {
        // function(e) return e:GetHandler():IsPreviousLocation(LOCATION_HAND|LOCATION_ONFIELD) end
        let h = effect::get_handler(d, a.e).nil();
        card::is_previous_location(d, h, LOCATION_HAND | LOCATION_ONFIELD)
    })));
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,3,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 3, a.tp, LOCATION_DECK as i32);
    true
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(12482652,42941100,79335209) and c:IsAbleToHand()
    card::is_code(d, c, &[12482652, 42941100, 79335209]) && card::is_able_to_hand(d, c, None)
}
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsCode,1,nil,12482652)
    //     and sg:IsExists(Card.IsCode,1,nil,42941100)
    //     and sg:IsExists(Card.IsCode,1,nil,79335209)
    let ok = group::is_exists(d, sg, &|d, c| card::is_code(d, c, &[12482652]), 1, NO_EX)
        && group::is_exists(d, sg, &|d, c| card::is_code(d, c, &[42941100]), 1, NO_EX)
        && group::is_exists(d, sg, &|d, c| card::is_code(d, c, &[79335209]), 1, NO_EX);
    (ok, false)
}
fn thop(d: &mut Duel, a: &EffArgs) {
    // local sg=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let sg = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    // if #sg<3 then return end
    if sg.len() < 3 {
        return;
    }
    // local g=aux.SelectUnselectGroup(sg,e,tp,3,3,s.rescon,1,tp,HINTMSG_ATOHAND)
    let g = aux::select_unselect_group(d, &sg, Some(a.e), a.tp, Some(3), Some(3), Some(&rescon), 1, a.tp, Some(HINTMSG_ATOHAND as u64), None, None, false).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
