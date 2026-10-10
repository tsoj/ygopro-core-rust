//! Pot of The Forbidden
use super::prelude::*;

pub const ID: u32 = 91501248;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // FLIP: Activate one of the forbidden effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:IsAbleToHand()
    card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // local b1=Duel.IsPlayerCanDraw(tp,2)
    let b1 = duel::is_player_can_draw(d, tp, 2);
    // local b2=Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    // local b3=Duel.IsExistingMatchingCard(aux.TRUE,tp,0,LOCATION_MZONE,1,nil)
    let b3 = duel::is_existing_matching_card(d, None, tp, 0, LOCATION_MZONE, 1, NO_EX);
    // local b4=Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,0,LOCATION_HAND,1,nil)
    let b4 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_deck(d, c)), tp, 0, LOCATION_HAND, 1, NO_EX);
    // if chk==0 then return b1 or b2 or b3 or b4 end
    if a.chk == Some(0) {
        return b1 || b2 || b3 || b4;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,0)},
    //     {b2,aux.Stringid(id,1)},
    //     {b3,aux.Stringid(id,2)},
    //     {b4,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 0)), (b2, aux::stringid(ID, 1)), (b3, aux::stringid(ID, 2)), (b4, aux::stringid(ID, 3))]).nil();
    // e:SetLabel(op)
    effect::set_label(d, e, &[op as i64]);
    if op == 1 {
        // e:SetCategory(CATEGORY_DRAW)
        effect::set_category(d, e, CATEGORY_DRAW);
        // Duel.SetTargetPlayer(tp)
        duel::set_target_player(d, tp);
        // Duel.SetTargetParam(2)
        duel::set_target_param(d, 2);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 2);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_TOHAND)
        effect::set_category(d, e, CATEGORY_TOHAND);
        // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
        let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,0,0)
        let n = g.len() as u8;
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, 0, 0);
    } else if op == 3 {
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, e, CATEGORY_DESTROY);
        // local g=Duel.GetMatchingGroup(aux.TRUE,tp,0,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(d, None, tp, 0, LOCATION_MZONE, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
        let n = g.len() as u8;
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    } else {
        // e:SetCategory(CATEGORY_TODECK)
        effect::set_category(d, e, CATEGORY_TODECK);
        // Duel.SetTargetPlayer(tp)
        duel::set_target_player(d, tp);
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,0,1-tp,LOCATION_HAND)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 0, 1 - tp, LOCATION_HAND as i32);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
        let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
        // Duel.Draw(p,d,REASON_EFFECT)
        duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
    } else if op == 2 {
        // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
        let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    } else if op == 3 {
        // local g=Duel.GetMatchingGroup(aux.TRUE,tp,0,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(d, None, tp, 0, LOCATION_MZONE, NO_EX);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    } else {
        // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
        let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
        // local g=Duel.GetFieldGroup(p,0,LOCATION_HAND)
        let g = duel::get_field_group(d, p, 0, LOCATION_HAND);
        // if #g>0 then
        if !g.is_empty() {
            // Duel.ConfirmCards(p,g)
            duel::confirm_cards(d, p, g.clone());
            // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_TODECK)
            duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_TODECK as u64);
            // local sg=g:FilterSelect(p,Card.IsAbleToDeck,1,1,nil)
            let sg = group::filter_select(d, &g, p, &|d, c| card::is_able_to_deck(d, c), 1, 1, false, NO_EX).nil();
            // Duel.SendtoDeck(sg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
            duel::send_to_deck(d, sg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
            // Duel.ShuffleHand(1-p)
            duel::shuffle_hand(d, 1 - p);
        }
    }
}
