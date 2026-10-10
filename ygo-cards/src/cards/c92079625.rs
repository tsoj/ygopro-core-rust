//! Helshaddoll Hollow
use super::prelude::*;

pub const ID: u32 = 92079625;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_SHADDOLL}
    listed_series: &[SET_SHADDOLL],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send 1 "Shaddoll" monster from your Extra Deck to the GY and banish 1 opponent's face-up monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Send cards from the top of your Deck to the GY, equal to the number of different original Attributes among the monsters on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e2, CATEGORY_DECKDES);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsReason(REASON_EFFECT) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_reason(d, h, REASON_EFFECT)
    })));
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.tgfilter(c,att)`
fn tgfilter(d: &mut Duel, c: CardId, att: u32) -> bool {
    // return c:IsSetCard(SET_SHADDOLL) and c:IsAbleToGrave() and c:IsAttribute(att)
    card::is_set_card(d, c, &[SET_SHADDOLL as u16]) && card::is_able_to_grave(d, c, None) && card::is_attribute(d, c, att)
}
/// `s.rmfilter(c,tp)`; the chain check calls it without `tp` (a `nil` player, an error in Lua when it is used)
fn rmfilter(d: &mut Duel, c: CardId, tp: Option<u8>) -> bool {
    // return c:IsAbleToRemove() and c:IsFaceup() and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_EXTRA,0,1,nil,c:GetAttribute())
    if !(card::is_able_to_remove(d, c, None, None, None) && card::is_faceup(d, c)) {
        return false;
    }
    let att = card::get_attribute(d, c);
    let tp = tp.nil();
    duel::is_existing_matching_card(d, Some(&|d, x| tgfilter(d, x, att)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and s.rmfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MZONE) && rmfilter(d, chkc, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.rmfilter,tp,0,LOCATION_MZONE,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| rmfilter(d, c, Some(tp))), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.rmfilter,tp,0,LOCATION_MZONE,1,1,nil,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| rmfilter(d, c, Some(tp))), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_EXTRA as i32);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, tp, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and tc:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_EXTRA,0,1,1,nil,tc:GetAttribute()):GetFirst()
    let att = card::get_attribute(d, tc);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| tgfilter(d, c, att)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SendtoGrave(sc,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_GRAVE) then
    if let Some(sc) = sc {
        if duel::send_to_grave(d, sc, REASON_EFFECT, None, None) > 0 && card::is_location(d, sc, LOCATION_GRAVE) {
            // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local attct=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil):GetClassCount(Card.GetOriginalAttribute)
    let attct = attribute_count(d, tp);
    // if chk==0 then return attct>0 and Duel.IsPlayerCanDiscardDeck(tp,attct) end
    if a.chk == Some(0) {
        return attct > 0 && duel::is_player_can_discard_deck(d, tp, attct as u32);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,attct)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, tp, attct as i32);
    true
}
/// The number of different original Attributes among the face-up monsters on the field
fn attribute_count(d: &mut Duel, tp: u8) -> usize {
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    group::get_class_count(d, &g, &|d, c| card::get_original_attribute(d, c) as i64)
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local attct=...
    let attct = attribute_count(d, tp);
    // if attct>0 and Duel.IsPlayerCanDiscardDeck(tp,attct) then
    if attct > 0 && duel::is_player_can_discard_deck(d, tp, attct as u32) {
        // Duel.DiscardDeck(tp,attct,REASON_EFFECT)
        duel::discard_deck(d, tp, attct as u16, REASON_EFFECT);
    }
}
