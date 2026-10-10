//! Agido the Ancient Sentinel
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 62320425;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID, CARD_EXCHANGE_SPIRIT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Each player sends the top 5 cards of their Deck to the GY
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
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsPreviousLocation(LOCATION_HAND|LOCATION_DECK) end)
    effect::set_condition(d, e2, Some(condition(e2_condition)));
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spconfilter(c,tp)`
fn spconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(1-tp) and c:IsPreviousLocation(LOCATION_HAND|LOCATION_DECK)
    card::is_controler(d, c, 1 - tp) && card::is_previous_location(d, c, LOCATION_HAND | LOCATION_DECK)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.spconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| spconfilter(d, c, tp), 1, NO_EX)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevel(4) and c:IsAttribute(ATTRIBUTE_EARTH) and c:IsRace(RACE_FAIRY) and not c:IsCode(id)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level(d, c, &[4])
        && card::is_attribute(d, c, ATTRIBUTE_EARTH)
        && card::is_race(d, c, RACE_FAIRY)
        && !card::is_code(d, c, &[ID])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,1,nil,e,tp)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if card::is_relate_to_effect(d, c, e)
        && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, e, tp))), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
        let nf = aux::necro_valley_filter(|d, x| spfilter(d, x, e, tp));
        let g = duel::select_matching_card(d, tp, Some(&nf), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
/// `function(e) return e:GetHandler():IsPreviousLocation(LOCATION_HAND|LOCATION_DECK) end`
fn e2_condition(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_HAND | LOCATION_DECK)
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,5) and Duel.IsPlayerCanDiscardDeck(1-tp,5) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, a.tp, 5) && duel::is_player_can_discard_deck(d, 1 - a.tp, 5);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,PLAYER_ALL,5)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, PLAYER_ALL as u8, 5);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if (Duel.DiscardDeck(tp,5,REASON_EFFECT)+Duel.DiscardDeck(1-tp,5,REASON_EFFECT))<=0
    //     or not Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_GRAVE,0,1,nil,CARD_EXCHANGE_SPIRIT) then return end
    let n1 = duel::discard_deck(d, tp, 5, REASON_EFFECT);
    let n2 = duel::discard_deck(d, 1 - tp, 5, REASON_EFFECT);
    if n1 + n2 <= 0 || !duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &[CARD_EXCHANGE_SPIRIT])), tp, LOCATION_GRAVE, 0, 1, NO_EX) {
        return;
    }
    // local b1=Duel.IsPlayerCanDiscardDeck(tp,5)
    let b1 = duel::is_player_can_discard_deck(d, tp, 5);
    // local b2=Duel.IsPlayerCanDiscardDeck(1-tp,5)
    let b2 = duel::is_player_can_discard_deck(d, 1 - tp, 5);
    // if (b1 or b2) and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
    if (b1 || b2) && duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
        // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,4)},{b2,aux.Stringid(id,5)})
        let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 4)), (b2, aux::stringid(ID, 5))]);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.DiscardDeck(op==1 and tp or 1-tp,5,REASON_EFFECT)
        let p = if op == Some(1) { tp } else { 1 - tp };
        duel::discard_deck(d, p, 5, REASON_EFFECT);
    }
}
