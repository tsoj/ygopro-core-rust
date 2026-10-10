//! Kelbek the Ancient Vanguard
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 25926710;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_EXCHANGE_SPIRIT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
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
    // Mill 5 cards from each Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DECKDES+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_DECKDES + CATEGORY_SET);
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
    // e2:SetTarget(s.mltg)
    effect::set_target(d, e2, Some(cost_target(mltg)));
    // e2:SetOperation(s.mlop)
    effect::set_operation(d, e2, Some(operation(mlop)));
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
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpecialSummoned() and c:IsAbleToHand()
    card::is_special_summoned(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && thfilter(d, chkc);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingTarget(s.thfilter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            && duel::is_existing_target(d, Some(&thfilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)<1 then return end
    if !card::is_relate_to_effect(d, c, a.e) || duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) < 1 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
/// `function(e) return e:GetHandler():IsPreviousLocation(LOCATION_HAND|LOCATION_DECK) end`
fn e2_condition(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_HAND | LOCATION_DECK)
}
/// `s.mltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn mltg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,5) and Duel.IsPlayerCanDiscardDeck(1-tp,5) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, a.tp, 5) && duel::is_player_can_discard_deck(d, 1 - a.tp, 5);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,PLAYER_ALL,5)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, PLAYER_ALL as u8, 5);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, a.tp, 0);
    true
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsTrap() and c:IsSSetable()
    card::is_trap(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.mlop(e,tp,eg,ep,ev,re,r,rp)`
fn mlop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if (Duel.DiscardDeck(tp,5,REASON_EFFECT)+Duel.DiscardDeck(1-tp,5,REASON_EFFECT))>0
    //     and Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_GRAVE,0,1,nil,CARD_EXCHANGE_SPIRIT)
    //     and Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.setfilter),tp,LOCATION_GRAVE,0,1,nil)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    let n1 = duel::discard_deck(d, tp, 5, REASON_EFFECT);
    let n2 = duel::discard_deck(d, 1 - tp, 5, REASON_EFFECT);
    if n1 + n2 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &[CARD_EXCHANGE_SPIRIT])), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&aux::necro_valley_filter(setfilter)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
        // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_GRAVE,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&setfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SSet(tp,g)
            duel::s_set(d, tp, g, None, true);
        }
    }
}
