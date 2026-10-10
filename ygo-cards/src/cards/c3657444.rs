//! Cyber Valley
use super::prelude::*;

pub const ID: u32 = 3657444;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When this card is targeted for an attack: You can banish this card; draw 1 card, then end the Battle Phase
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EVENT_BE_BATTLE_TARGET);
    // e1:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e1, Some(cost::self_banish()));
    // e1:SetTarget(s.drawtg1)
    effect::set_target(d, e1, Some(cost_target(drawtg1)));
    // e1:SetOperation(s.drawop1)
    effect::set_operation(d, e1, Some(operation(drawop1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can target 1 face-up monster you control and this card; banish both those targets, then draw 2 cards
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_REMOVE+CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_REMOVE + CATEGORY_DRAW);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTarget(s.drawtg2)
    effect::set_target(d, e2, Some(cost_target(drawtg2)));
    // e2:SetOperation(s.drawop2)
    effect::set_operation(d, e2, Some(operation(drawop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // You can target 1 card in your GY; banish both this card and 1 card from your hand, then place that target on top of the Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_REMOVE+CATEGORY_TODECK)
    effect::set_category(d, e3, CATEGORY_REMOVE + CATEGORY_TODECK);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTarget(s.tdtg)
    effect::set_target(d, e3, Some(cost_target(tdtg)));
    // e3:SetOperation(s.tdop)
    effect::set_operation(d, e3, Some(operation(tdop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.drawtg1(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drawtg1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.drawop1(e,tp,eg,ep,ev,re,r,rp)`
fn drawop1(d: &mut Duel, a: &EffArgs) {
    // if Duel.Draw(tp,1,REASON_EFFECT)>0 then
    if duel::draw(d, a.tp, 1, REASON_EFFECT) > 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SkipPhase(1-tp,PHASE_BATTLE,RESET_PHASE|PHASE_BATTLE_STEP,1)
        duel::skip_phase(d, 1 - a.tp, PHASE_BATTLE, RESET_PHASE | PHASE_BATTLE_STEP, 1, 0);
    }
}
/// `aux.FaceupFilter(Card.IsAbleToRemove)`
fn faceup_removable(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.drawtg2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn drawtg2(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() and chkc:IsAbleToRemove() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && faceup_removable(d, chkc);
    }
    // if chk==0 then return c:IsAbleToRemove() and Duel.IsPlayerCanDraw(tp,2)
    //     and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsAbleToRemove),tp,LOCATION_MZONE,0,1,c) end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, c, None, None, None)
            && duel::is_player_can_draw(d, a.tp, 2)
            && duel::is_existing_target(d, Some(&faceup_removable), a.tp, LOCATION_MZONE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsAbleToRemove),tp,LOCATION_MZONE,0,1,1,c)
    let mut g = duel::select_target(d, a.tp, Some(&faceup_removable), a.tp, LOCATION_MZONE, 0, 1, 1, false, c.into()).nil();
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 2, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    true
}
/// `s.drawop2(e,tp,eg,ep,ev,re,r,rp)`
fn drawop2(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFacedown() or not c:IsRelateToEffect(e) or not tc:IsRelateToEffect(e) then return end
    if card::is_facedown(d, tc) || !card::is_relate_to_effect(d, c, a.e) || !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local sg=Group.FromCards(c,tc)
    let mut sg = CardSet::from_card(c);
    group::add_card(&mut sg, &CardSet::from_card(tc));
    // if Duel.Remove(sg,POS_FACEUP,REASON_EFFECT)==2 then
    if duel::remove(d, sg, POS_FACEUP as u8, REASON_EFFECT, None, None) == 2 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,2,REASON_EFFECT)
        duel::draw(d, a.tp, 2, REASON_EFFECT);
    }
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToDeck,tp,LOCATION_GRAVE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,LOCATION_HAND,0,1,nil)
    //     and c:IsAbleToRemove() end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| card::is_able_to_deck(d, x)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), a.tp, LOCATION_HAND, 0, 1, NO_EX)
            && card::is_able_to_remove(d, c, None, None, None);
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToDeck,tp,LOCATION_GRAVE,0,1,1,nil)
    let _g = duel::select_target(d, a.tp, Some(&|d, x| card::is_able_to_deck(d, x)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,c,2,0,LOCATION_MZONE|LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(c)), 2, 0, (LOCATION_MZONE | LOCATION_HAND) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local hg=Duel.SelectMatchingCard(tp,Card.IsAbleToRemove,tp,LOCATION_HAND,0,1,1,nil)
    let mut hg = duel::select_matching_card(d, a.tp, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #hg>0 then
    if !hg.is_empty() {
        // hg:AddCard(c)
        group::add_card(&mut hg, &CardSet::from_card(c));
        // if Duel.Remove(hg,POS_FACEUP,REASON_EFFECT)==2 then
        if duel::remove(d, hg, POS_FACEUP as u8, REASON_EFFECT, None, None) == 2 {
            // local tc=Duel.GetFirstTarget()
            let tc = duel::first_target(d).nil();
            // if tc:IsRelateToEffect(e) then
            if card::is_relate_to_effect(d, tc, a.e) {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SendtoDeck(tc,nil,SEQ_DECKTOP,REASON_EFFECT)
                duel::send_to_deck(d, tc, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
                // Duel.ConfirmCards(1-tp,tc)
                duel::confirm_cards(d, 1 - a.tp, CardSet::from_card(tc));
            }
        }
    }
}
