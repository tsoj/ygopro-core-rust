//! Deep Sea Prima Donna
use super::prelude::*;

pub const ID: u32 = 50793215;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1+ non-Tuner monsters
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Your opponent cannot target a Synchro Monster that used this card as material with monster effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e1, EVENT_BE_MATERIAL);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return r==REASON_SYNCHRO end)
    effect::set_condition(d, e1, Some(condition(|_d, a| a.r == REASON_SYNCHRO)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add to your hand, or Special Summon, 1 Level 4 or lower WATER monster from your Deck, and if you do, add 1 of your opponent's banished cards to their hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SEARCH + CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.thsptg)
    effect::set_target(d, e2, Some(cost_target(thsptg)));
    // e2:SetOperation(s.thspop)
    effect::set_operation(d, e2, Some(operation(thspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Shuffle 1 banished card into the Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e3, CATEGORY_TODECK);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.tdtg)
    effect::set_target(d, e3, Some(cost_target(tdtg)));
    // e3:SetOperation(s.tdop)
    effect::set_operation(d, e3, Some(operation(tdop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rc=c:GetReasonCard()
    let rc = card::get_reason_card(d, c).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(3062)
    effect::set_description(d, e1, 3062);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1:SetValue(function(e,re,rp) return re:IsMonsterEffect() and rp==1-e:GetHandlerPlayer() end)
    effect::set_value_fn(d, e1, lua_fn(|d, a: Args| {
        effect::is_monster_effect(d, a.e(1)) && a.int(2) == 1 - effect::get_handler_player(d, a.e(0)) as i64
    }));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // rc:RegisterEffect(e1)
    card::register_effect(d, rc, e1, false);
}
// function s.thspfilter(c,e,tp,mzone_chk)
fn thspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mzone_chk: bool) -> bool {
    // return c:IsLevelBelow(4) and c:IsAttribute(ATTRIBUTE_WATER) and (c:IsAbleToHand()
    //   or (mzone_chk and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::is_level_below(d, c, 4)
        && card::is_attribute(d, c, ATTRIBUTE_WATER)
        && (card::is_able_to_hand(d, c, None) || (mzone_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
// function s.thsptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn thsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_REMOVED) and chkc:IsAbleToHand() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_REMOVED) && card::is_able_to_hand(d, chkc, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToHand,tp,0,LOCATION_REMOVED,1,nil)
    //   and Duel.IsExistingMatchingCard(s.thspfilter,tp,LOCATION_DECK,0,1,nil,e,tp,Duel.GetLocationCount(tp,LOCATION_MZONE)>0) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| card::is_able_to_hand(d, x, None)), a.tp, 0, LOCATION_REMOVED, 1, NO_EX)
            && {
                let mzone_chk = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
                duel::is_existing_matching_card(d, Some(&|d, x| thspfilter(d, x, a.e, a.tp, mzone_chk)), a.tp, LOCATION_DECK, 0, 1, NO_EX)
            };
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToHand,tp,0,LOCATION_REMOVED,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, x| card::is_able_to_hand(d, x, None)), a.tp, 0, LOCATION_REMOVED, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.thspop(e,tp,eg,ep,ev,re,r,rp)
fn thspop(d: &mut Duel, a: &EffArgs) {
    // local mzone_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let mzone_chk = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
    // local sc=Duel.SelectMatchingCard(tp,s.thspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,mzone_chk):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| thspfilter(d, x, a.e, a.tp, mzone_chk)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if not sc then return end
    let Some(sc) = sc else {
        return;
    };
    // local tohand_or_ss_chk=aux.ToHandOrElse(sc,tp,
    //   function() return mzone_chk and sc:IsCanBeSpecialSummoned(e,0,tp,false,false) end,
    //   function() return Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP) end,
    //   aux.Stringid(id,3))
    // The result is a number in Lua (always truthy), so the check below only tests the target.
    let _tohand_or_ss_chk = aux::to_hand_or_else(
        d,
        CardSet::from_card(sc),
        a.tp,
        Some(&|d, _x| mzone_chk && card::is_can_be_special_summoned(d, sc, a.e, 0, a.tp, false, false, None, None, None)),
        Some(&|d, _g| duel::special_summon(d, CardSet::from_card(sc), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None)),
        Some(aux::stringid(ID, 3)),
    );
    // local tc=Duel.GetFirstTarget()
    // if tohand_or_ss_chk and tc:IsRelateToEffect(e) then
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(tc), None, REASON_EFFECT, None);
    }
}
// function s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_REMOVED) and chkc:IsAbleToDeck() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_REMOVED) && card::is_able_to_deck(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToDeck,tp,LOCATION_REMOVED,LOCATION_REMOVED,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&card::is_able_to_deck), a.tp, LOCATION_REMOVED, LOCATION_REMOVED, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToDeck,tp,LOCATION_REMOVED,LOCATION_REMOVED,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&card::is_able_to_deck), a.tp, LOCATION_REMOVED, LOCATION_REMOVED, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 1, a.tp, 0);
    true
}
// function s.tdop(e,tp,eg,ep,ev,re,r,rp)
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, CardSet::from_card(tc), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
