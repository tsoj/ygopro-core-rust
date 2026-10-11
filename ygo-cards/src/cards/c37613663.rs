//! Millennium Ankh
use super::prelude::*;

pub const ID: u32 = 37613663;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_FORBIDDEN_ONE, SET_EXODIA, SET_MILLENNIUM],
    listed_names: &[83257450],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(83257450) and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[83257450])
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.showfilter(c)
fn showfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_FORBIDDEN_ONE) and c:IsOriginalType(TYPE_MONSTER)
    //     and (c:IsLocation(LOCATION_HAND|LOCATION_DECK) or c:IsFaceup())
    card::is_set_card(d, c, &[SET_FORBIDDEN_ONE as u16])
        && card::is_original_type(d, c, TYPE_MONSTER)
        && (card::is_location(d, c, LOCATION_HAND | LOCATION_DECK) || card::is_faceup(d, c))
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp)
    //     and Duel.IsExistingMatchingCard(s.showfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_ONFIELD,0,5,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&showfilter), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_ONFIELD, 0, 5, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,e:GetHandler(),1,tp,0)
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(c)), 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, LOCATION_ONFIELD as i32);
    true
}
// function s.tdfilter(c)
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // if (c:IsSetCard(SET_EXODIA) and c:GetOriginalLevel()>=10) or c:IsSetCard(SET_MILLENNIUM) then return false end
    if (card::is_set_card(d, c, &[SET_EXODIA as u16]) && card::get_original_level(d, c) >= 10) || card::is_set_card(d, c, &[SET_MILLENNIUM as u16]) {
        return false;
    }
    // return c:IsOriginalType(TYPE_MONSTER) and c:IsFaceup() and c:IsAbleToDeck()
    card::is_original_type(d, c, TYPE_MONSTER) && card::is_faceup(d, c) && card::is_able_to_deck(d, c)
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local rg=Duel.GetMatchingGroup(s.showfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_ONFIELD,0,nil)
    let mut rg = duel::get_matching_group(d, Some(&showfilter), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_ONFIELD, 0, NO_EX);
    // if #rg>5 then
    if rg.len() > 5 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
        // rg=Duel.SelectMatchingCard(tp,s.showfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_ONFIELD,0,5,5,nil)
        rg = duel::select_matching_card(d, tp, Some(&showfilter), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_ONFIELD, 0, 5, 5, false, NO_EX).nil();
    }
    // if #rg==5 then
    if rg.len() == 5 {
        // Duel.ConfirmCards(1-tp,rg)
        duel::confirm_cards(d, 1 - tp, &rg);
        // if rg:IsExists(Card.IsLocation,1,nil,LOCATION_HAND) then Duel.ShuffleHand(tp) end
        if group::is_exists(d, &rg, &|d, c| card::is_location(d, c, LOCATION_HAND), 1, NO_EX) {
            duel::shuffle_hand(d, tp);
        }
        // if rg:IsExists(Card.IsLocation,1,nil,LOCATION_DECK) then Duel.ShuffleDeck(tp) end
        if group::is_exists(d, &rg, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX) {
            duel::shuffle_deck(d, tp);
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
        let sel = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel);
        // if sc then
        if let Some(sc) = sc {
            // local tdg=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_ONFIELD,0,nil)
            let tdg = duel::get_matching_group(d, Some(&tdfilter), tp, LOCATION_ONFIELD, 0, NO_EX);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // if Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)>0 and #tdg>0 then
            if duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 && !tdg.is_empty() {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SendtoDeck(tdg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
                duel::send_to_deck(d, tdg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
            }
        }
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Cannot Summon monsters for the rest of this turn (but you can Normal Set)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_CANNOT_FLIP_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_FLIP_SUMMON);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e3, EFFECT_CANNOT_SPECIAL_SUMMON);
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, tp);
    // Shuffle this card into the Deck instead of sending it to the GY
    // if c:IsRelateToEffect(e) and e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if card::is_relate_to_effect(d, c, a.e) && effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        // if c:IsHasEffect(EFFECT_CANNOT_TO_DECK) then return end
        if !card::is_has_effect(d, c, EFFECT_CANNOT_TO_DECK, None).is_empty() {
            return;
        }
        // c:CancelToGrave()
        card::cancel_to_grave(d, c, true);
        // Duel.SendtoDeck(c,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, CardSet::from_card(c), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
