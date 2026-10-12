//! Mad Tempest Xenovader△ Ploutonion
use super::prelude::*;

pub const ID: u32 = 100458037;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_XENOVADER],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // (Quick Effect): You can reveal this card and 1 other "Xenovader△" card in your hand; banish both (face-down), and if you do, draw 2 cards. You can only use this effect of "Mad Tempest Xenovader△ Ploutonion" once per turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.selfbancost)
    effect::set_cost(d, e1, Some(cost_target(selfbancost)));
    // e1:SetTarget(s.selfbantg)
    effect::set_target(d, e1, Some(cost_target(selfbantg)));
    // e1:SetOperation(s.selfbanop)
    effect::set_operation(d, e1, Some(operation(selfbanop)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is Normal or Special Summoned: Banish (face-down) the top 7 cards of your Deck
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    // e2a:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2a, CATEGORY_REMOVE);
    // e2a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2a, EVENT_SUMMON_SUCCESS);
    // e2a:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) ... end)
    effect::set_target(d, e2a, Some(cost_target(e2a_tg)));
    // e2a:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e2a, Some(operation(e2a_op)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
    // Once per turn (Quick Effect): You can shuffle 1 of your face-down banished "Xenovader△" monsters into the Deck, then you can shuffle 1 monster your opponent controls with the same Attribute into the Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e3, CATEGORY_TODECK);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.tdtg)
    effect::set_target(d, e3, Some(cost_target(tdtg)));
    // e3:SetOperation(s.tdop)
    effect::set_operation(d, e3, Some(operation(tdop)));
    // e3:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.selfbancostfilter(c,tp)`
fn selfbancostfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_XENOVADER) and not c:IsPublic() and c:IsAbleToRemove(tp,POS_FACEDOWN)
    card::is_set_card(d, c, &[SET_XENOVADER as u16]) && !card::is_public(d, c) && card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None)
}
/// `s.selfbancost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfbancost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return not c:IsPublic() and Duel.IsExistingMatchingCard(s.selfbancostfilter,tp,LOCATION_HAND,0,1,c,tp) end
    if a.chk == Some(0) {
        return !card::is_public(d, c) && duel::is_existing_matching_card(d, Some(&|d, x| selfbancostfilter(d, x, tp)), tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.selfbancostfilter,tp,LOCATION_HAND,0,1,1,c,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, x| selfbancostfilter(d, x, tp)), tp, LOCATION_HAND, 0, 1, 1, false, c.into()).nil();
    let sc = group::get_first(d, &g);
    // local g=Group.FromCards(c,sc)
    let mut revealed = CardSet::from_card(c);
    if let Some(sc) = sc {
        group::add_card(&mut revealed, &CardSet::from_card(sc));
    }
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, revealed.clone());
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    // sc:CreateEffectRelation(e)
    card::create_effect_relation(d, sc.nil(), e);
    // e:GetChainData().revealed_cards=g
    chain::set_effect_data(d, e, None, chain::field::CARD, Value::Group(revealed));
    true
}
/// `s.selfbantg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfbantg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemove(tp,POS_FACEDOWN) and Duel.IsPlayerCanDraw(tp,2) end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None) && duel::is_player_can_draw(d, tp, 2);
    }
    // local revealed_cards=e:GetChainData().revealed_cards
    let revealed = chain::effect_data(d, a.e, None, chain::field::CARD).as_group().cloned();
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,revealed_cards,2,tp,POS_FACEDOWN)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, revealed, 2, tp, POS_FACEDOWN as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 2);
    true
}
/// `s.selfbanop(e,tp,eg,ep,ev,re,r,rp)`
fn selfbanop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local revealed_cards=e:GetChainData().revealed_cards:Match(Card.IsRelateToEffect,nil,e)
    let mut revealed = chain::effect_data(d, a.e, None, chain::field::CARD).as_group().cloned().nil();
    let e = a.e;
    group::match_(d, &mut revealed, &|d, c| card::is_relate_to_effect(d, c, e), NO_EX);
    // if #revealed_cards==2 and Duel.Remove(revealed_cards,POS_FACEDOWN,REASON_EFFECT)==2 and revealed_cards:FilterCount(Card.IsLocation,nil,LOCATION_REMOVED)==2 then
    if revealed.len() == 2
        && duel::remove(d, revealed.clone(), POS_FACEDOWN as u8, REASON_EFFECT, None, None) == 2
        && group::filter_count(d, &revealed, &|d, c| card::is_location(d, c, LOCATION_REMOVED), NO_EX) == 2
    {
        // Duel.Draw(tp,2,REASON_EFFECT)
        duel::draw(d, tp, 2, REASON_EFFECT);
    }
}
/// The target of e2a: banish the top 7 cards of the Deck
fn e2a_tg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 7, a.tp, LOCATION_DECK as i32);
    true
}
/// The operation of e2a: banish the top 7 cards of the Deck face-down
fn e2a_op(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetDecktopGroup(tp,7)
    let g = duel::get_decktop_group(d, a.tp, 7);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.Remove(g,POS_FACEDOWN,REASON_EFFECT)
        duel::remove(d, g, POS_FACEDOWN as u8, REASON_EFFECT, None, None);
    }
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() and c:IsSetCard(SET_XENOVADER) and c:IsMonster() and c:IsAbleToDeck()
    card::is_facedown(d, c) && card::is_set_card(d, c, &[SET_XENOVADER as u16]) && card::is_monster(d, c) && card::is_able_to_deck(d, c)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tdfilter), tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, LOCATION_REMOVED as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,nil,1,1-tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, 1 - tp, LOCATION_MZONE as i32);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.tdfilter,tp,LOCATION_REMOVED,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&tdfilter), tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    // if not sc then return end
    let Some(sc) = group::get_first(d, &g) else {
        return;
    };
    // Duel.ConfirmCards(1-tp,sc)
    duel::confirm_cards(d, 1 - tp, CardSet::from_card(sc));
    // if Duel.SendtoDeck(sc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_DECK|LOCATION_EXTRA)
    if duel::send_to_deck(d, CardSet::from_card(sc), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0
        && card::is_location(d, sc, LOCATION_DECK | LOCATION_EXTRA)
    {
        // and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttribute,sc:GetAttribute()),tp,0,LOCATION_MZONE,1,nil)
        let attr = card::get_attribute(d, sc);
        if duel::is_existing_matching_card(d, Some(&aux::faceup_filter(move |d, c| card::is_attribute(d, c, attr))), tp, 0, LOCATION_MZONE, 1, NO_EX)
            // and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
            && duel::select_yes_no(d, tp, aux::stringid(ID, 3))
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
            // local g=Duel.SelectMatchingCard(tp,aux.FaceupFilter(Card.IsAttribute,sc:GetAttribute()),tp,0,LOCATION_MZONE,1,1,nil)
            let g = duel::select_matching_card(d, tp, Some(&aux::faceup_filter(move |d, c| card::is_attribute(d, c, attr))), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX)
                .nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.HintSelection(g)
                duel::hint_selection(d, g.clone(), true);
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
                duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
            }
        }
    }
}
