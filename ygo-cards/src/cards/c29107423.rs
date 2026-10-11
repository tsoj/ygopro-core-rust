//! Ruddy Rose Witch
//! Scripted by Rundas
use super::prelude::*;

pub const ID: u32 = 29107423;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[17720747, CARD_BLACK_ROSE_DRAGON, 40139997], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 "Witch of the Black Rose" from your Deck to your hand, and if you do, take 1 Level 3 or lower Plant monster from your Deck and place it on top of your Deck, then immediately after this effect resolves, you can Normal Summon 1 "Witch of the Black Rose" from your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e1, Some(cost::self_tribute()));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return 1 of your "Black Rose Dragon" or "Ruddy Rose Dragon" that is banished or in your GY to the Extra Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOEXTRA)
    effect::set_category(d, e2, CATEGORY_TOEXTRA);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.textg)
    effect::set_target(d, e2, Some(cost_target(textg)));
    // e2:SetOperation(s.texop)
    effect::set_operation(d, e2, Some(operation(texop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thfilter(c,tp)`
fn thfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsCode(17720747) and c:IsAbleToHand() and Duel.IsExistingMatchingCard(s.topdeckfilter,tp,LOCATION_DECK,0,1,c)
    card::is_code(d, c, &[17720747])
        && card::is_able_to_hand(d, c, None)
        && duel::is_existing_matching_card(d, Some(&topdeckfilter), tp, LOCATION_DECK, 0, 1, c.into())
}
/// `s.topdeckfilter(c)`
fn topdeckfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelBelow(3) and c:IsRace(RACE_PLANT)
    card::is_level_below(d, c, 3) && card::is_race(d, c, RACE_PLANT)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.sumfilter(c)`
fn sumfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(17720747) and c:IsSummonable(true,nil)
    card::is_code(d, c, &[17720747]) && card::is_summonable(d, c, true, None, 0, None)
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| thfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SendtoHand(sc,nil,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_HAND) then
    if let Some(sc) = sc {
        if duel::send_to_hand(d, CardSet::from_card(sc), None, REASON_EFFECT, None) > 0 && card::is_location(d, sc, LOCATION_HAND) {
            // Duel.ConfirmCards(1-tp,sc)
            duel::confirm_cards(d, 1 - tp, CardSet::from_card(sc));
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, tp);
            // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
            duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
            // local topc=Duel.SelectMatchingCard(tp,s.topdeckfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
            let g = duel::select_matching_card(d, tp, Some(&topdeckfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            let topc = group::get_first(d, &g);
            // if topc then
            if let Some(topc) = topc {
                // Duel.ShuffleDeck(tp)
                duel::shuffle_deck(d, tp);
                // Duel.MoveSequence(topc,0)
                duel::move_sequence(d, topc, 0, None);
                // Duel.ConfirmDecktop(tp,1)
                let _ = duel::confirm_decktop(d, tp, 1);
                // if Duel.IsExistingMatchingCard(s.sumfilter,tp,LOCATION_HAND,0,1,nil)
                //     and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
                if duel::is_existing_matching_card(d, Some(&sumfilter), tp, LOCATION_HAND, 0, 1, NO_EX) && duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
                    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
                    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SUMMON as u64);
                    // local sumc=Duel.SelectMatchingCard(tp,s.sumfilter,tp,LOCATION_HAND,0,1,1,nil):GetFirst()
                    let g = duel::select_matching_card(d, tp, Some(&sumfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
                    let sumc = group::get_first(d, &g);
                    // if sumc then
                    if let Some(sumc) = sumc {
                        // Duel.Summon(tp,sumc,true,nil)
                        duel::summon(d, tp, sumc, true, None, 0, None);
                    }
                }
            }
        }
    }
}
/// `s.texfilter(c)`
fn texfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_BLACK_ROSE_DRAGON,40139997) and c:IsFaceup() and c:IsAbleToExtra()
    card::is_code(d, c, &[CARD_BLACK_ROSE_DRAGON, 40139997]) && card::is_faceup(d, c) && card::is_able_to_extra(d, c)
}
/// `s.textg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn textg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.texfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&texfilter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, h.into());
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOEXTRA,nil,1,tp,LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    true
}
/// `s.texop(e,tp,eg,ep,ev,re,r,rp)`
fn texop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.texfilter),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(texfilter)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
