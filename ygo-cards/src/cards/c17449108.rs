//! Nobleman of Extermination
use super::prelude::*;

pub const ID: u32 = 17449108;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy 1 face-down Spell/Trap card, banish it if it is a Trap and all cards with the same name
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() and c:IsAbleToRemove()
    card::is_facedown(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_SZONE) and s.filter(chkc) and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_SZONE) && filter(d, chkc) && chkc != c;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_SZONE,LOCATION_SZONE,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_SZONE, LOCATION_SZONE, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_SZONE,LOCATION_SZONE,1,1,c)
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_SZONE, LOCATION_SZONE, 1, 1, false, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g.clone(), 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,PLAYER_ALL,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, PLAYER_ALL as u8, LOCATION_DECK as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFacedown() and tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT,LOCATION_REMOVED)>0 and tc:IsTrap() then
    if card::is_facedown(d, tc)
        && card::is_relate_to_effect(d, tc, a.e)
        && duel::destroy(d, tc, REASON_EFFECT, Some(LOCATION_REMOVED), None) > 0
        && card::is_trap(d, tc)
    {
        // local code=tc:GetCode()
        let code = card::get_code(d, tc)[0];
        // local g=Duel.GetFieldGroup(tp,0,LOCATION_DECK)
        let g = duel::get_field_group(d, a.tp, 0, LOCATION_DECK);
        // Duel.ConfirmCards(tp,g)
        duel::confirm_cards(d, a.tp, g);
        // g=Duel.GetFieldGroup(tp,LOCATION_DECK,0)
        let g = duel::get_field_group(d, a.tp, LOCATION_DECK, 0);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, a.tp);
        // Duel.ShuffleDeck(1-tp)
        duel::shuffle_deck(d, 1 - a.tp);
        // local rmvg=Duel.GetMatchingGroup(Card.IsCode,tp,LOCATION_DECK,LOCATION_DECK,nil,code)
        let rmvg = duel::get_matching_group(d, Some(&|d, c| card::is_code(d, c, &[code])), a.tp, LOCATION_DECK, LOCATION_DECK, NO_EX);
        // if #rmvg>0 then Duel.Remove(rmvg,POS_FACEUP,REASON_EFFECT) end
        if !rmvg.is_empty() {
            duel::remove(d, rmvg, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
