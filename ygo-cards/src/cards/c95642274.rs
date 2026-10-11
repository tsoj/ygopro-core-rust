//! The Humble Sentry
use super::prelude::*;

pub const ID: u32 = 95642274;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,LOCATION_HAND,0,1,e:GetHandler())
    //     and not Duel.IsExistingMatchingCard(Card.IsPublic,tp,LOCATION_HAND,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_deck(d, c)), a.tp, LOCATION_HAND, 0, 1, h.into())
            && !duel::is_existing_matching_card(d, Some(&|d, c| card::is_public(d, c)), a.tp, LOCATION_HAND, 0, 1, h.into());
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, _a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local cg=Duel.GetFieldGroup(p,LOCATION_HAND,0)
    let cg = duel::get_field_group(d, p, LOCATION_HAND, 0);
    // Duel.ConfirmCards(1-p,cg)
    duel::confirm_cards(d, 1 - p, cg);
    // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,p,LOCATION_HAND,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_deck(d, c)), p, LOCATION_HAND, 0, NO_EX);
    // if #g>=1 then
    if g.len() >= 1 {
        // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_TODECK as u64);
        // local sg=g:Select(p,1,1,nil)
        let sg = group::select(d, &g, p, 1, 1, false, NO_EX).nil();
        // Duel.SendtoDeck(sg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, sg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
    // Duel.ShuffleHand(p)
    duel::shuffle_hand(d, p);
}
