//! PSY-Framelord Omega
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 74586817;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // 1 Tuner + 1+ non-Tuner monsters
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Banish this card and 1 random card from opponent's hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(main_phase_cond)));
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // c:RegisterFlagEffect(id,0,EFFECT_FLAG_UNCOPYABLE,0)
    card::register_flag_effect(d, c, ID, 0, EFFECT_FLAG_UNCOPYABLE, 0, 0, 0);
    // Return 1 banished card to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(_,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Shuffle this card and 1 other card from the GY to the Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e3, CATEGORY_TODECK);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetTarget(s.tdtg)
    effect::set_target(d, e3, Some(cost_target(tdtg)));
    // e3:SetOperation(s.tdop)
    effect::set_operation(d, e3, Some(operation(tdop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function() return Duel.IsMainPhase() end`
fn main_phase_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
}
/// `function(_,tp) return Duel.IsTurnPlayer(1-tp) end`
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
/// `Card.IsAbleToRemove` as a filter (no extra arguments)
fn able_to_remove(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_remove(d, c, None, None, None)
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemove()
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_HAND,1,nil) end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, c, None, None, None)
            && duel::is_existing_matching_card(d, Some(&able_to_remove), a.tp, 0, LOCATION_HAND, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_HAND,nil)
    let mut g = duel::get_matching_group(d, Some(&able_to_remove), a.tp, 0, LOCATION_HAND, NO_EX);
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 2, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetFieldGroup(1-tp,LOCATION_HAND,0)
    let g = duel::get_field_group(d, 1 - a.tp, LOCATION_HAND, 0);
    // if #g==0 or not c:IsRelateToEffect(e) or not c:IsFaceup() then return end
    if g.is_empty() || !card::is_relate_to_effect(d, c, a.e) || !card::is_faceup(d, c) {
        return;
    }
    // local hg=g:RandomSelect(1-tp,1)
    let hg = group::random_select(d, &g, 1 - a.tp, 1);
    // if #hg~=1 then return end
    if hg.len() != 1 {
        return;
    }
    // local rg=hg+c
    let mut rg = hg.clone();
    group::add_card(&mut rg, &CardSet::from_card(c));
    // if Duel.Remove(rg,POS_FACEUP,REASON_EFFECT|REASON_TEMPORARY)==0 then return end
    if duel::remove(d, rg.clone(), POS_FACEUP as u8, REASON_EFFECT | REASON_TEMPORARY, None, None) == 0 {
        return;
    }
    // local retg=c:HasFlagEffect(id) and rg or hg
    let mut retg = if card::has_flag_effect(d, c, ID, None) { rg } else { hg };
    // retg:Match(Card.IsLocation,nil,LOCATION_REMOVED)
    group::match_(d, &mut retg, &|d, c| card::is_location(d, c, LOCATION_REMOVED), NO_EX);
    // aux.DelayedOperation(retg,PHASE_STANDBY,id+1,e,tp,s.retop,s.retcon,RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN)
    aux::delayed_operation(
        d,
        retg,
        PHASE_STANDBY,
        ID + 1,
        a.e,
        a.tp,
        Some(Arc::new(retop)),
        Some(Arc::new(retcon)),
        Some(RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN),
        None,
        None,
        None,
    );
}
/// `s.retop(rg,e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, rg: &CardSet, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for tc in rg:Iter() do
    for tc in group::iter(d, rg) {
        // if tc==c then Duel.ReturnToField(tc)
        if tc == c {
            duel::return_to_field(d, tc, None, None);
        } else {
            // else Duel.SendtoHand(tc,tc:GetPreviousControler(),REASON_EFFECT)
            let p = card::get_previous_controler(d, tc);
            duel::send_to_hand(d, tc, Some(p), REASON_EFFECT, None);
        }
    }
    // (the Lua operation returns nothing; the result is not used)
    true
}
/// `s.retcon(rg,e,tp,eg,ep,ev,re,r,rp)`
fn retcon(d: &mut Duel, _rg: &CardSet, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_REMOVED) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_REMOVED);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_REMOVED,LOCATION_REMOVED,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, LOCATION_REMOVED, LOCATION_REMOVED, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_REMOVED,LOCATION_REMOVED,1,1,nil)
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_REMOVED, LOCATION_REMOVED, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, g, 1, 0, 0);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.SendtoGrave(tg,REASON_EFFECT|REASON_RETURN)
        duel::send_to_grave(d, tg, REASON_EFFECT | REASON_RETURN, None, None);
    }
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsAbleToDeck() and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_able_to_deck(d, chkc) && chkc != c;
    }
    // if chk==0 then return c:IsAbleToExtra()
    //     and Duel.IsExistingTarget(Card.IsAbleToDeck,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,c) end
    if a.chk == Some(0) {
        return card::is_able_to_extra(d, c)
            && duel::is_existing_target(d, Some(&card::is_able_to_deck), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToDeck,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,1,c)
    let mut g = duel::select_target(d, a.tp, Some(&card::is_able_to_deck), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, 1, false, c.into()).nil();
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), 2, 0, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_relate_to_effect(d, tc.nil(), a.e) {
        // local g=Group.CreateGroup(c,tc)
        let mut g = CardSet::from_card(c);
        group::add_card(&mut g, &CardSet::from_card(tc.nil()));
        // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
