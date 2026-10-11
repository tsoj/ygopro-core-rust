//! Levia-Dragon of Atlantis - Daedalus
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 64603351;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_UMI, CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // This card's name becomes "Umi" while in the Monster Zone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(CARD_UMI)
    effect::set_value(d, e1, CARD_UMI as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If you control "Atlantis, the Dragon City" or "Umi": You can Special Summon this card from your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // You can send up to 3 face-up "Umi" you control to the GY; add that many cards that mention "Atlantis, the Dragon City" from your Deck to your hand, except Level 7 monsters, then you can send 1 card on the field to the GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_TOGRAVE)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TOGRAVE);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCost(s.thcost)
    effect::set_cost(d, e3, Some(cost_target(thcost)));
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON,CARD_UMI),tp,LOCATION_ONFIELD,0,1,nil)
    duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON, CARD_UMI])),
        a.tp,
        LOCATION_ONFIELD,
        0,
        1,
        NO_EX,
    )
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.thcostfilter(c)`
fn thcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_UMI) and c:IsFaceup() and c:IsAbleToGraveAsCost()
    card::is_code(d, c, &[CARD_UMI]) && card::is_faceup(d, c) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:ListsCode(CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON) and not c:IsLevel(7) and c:IsAbleToHand()
    card::lists_code(d, c, &[CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON as u32]) && !card::is_level(d, c, &[7]) && card::is_able_to_hand(d, c, None)
}
/// `s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thcostfilter,tp,LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thcostfilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // local max_count=Duel.GetMatchingGroupCount(s.thfilter,tp,LOCATION_DECK,0,nil)
    let max_count = duel::get_matching_group_count(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    // max_count=math.min(max_count,3)
    let max_count = max_count.min(3);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thcostfilter,tp,LOCATION_ONFIELD,0,1,max_count,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thcostfilter), a.tp, LOCATION_ONFIELD, 0, 1, max_count as u16, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    let n = g.len() as i64;
    duel::send_to_grave(d, g, REASON_COST, None, None);
    // e:GetChainData().search_count=#g
    chain::set_effect_data(d, a.e, None, chain::field::CARD + 0, Value::Int(n));
    true
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,PLAYER_EITHER,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE as u64, None, 1, PLAYER_EITHER as u8, LOCATION_ONFIELD as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local search_count=e:GetChainData().search_count
    let search_count = chain::effect_data(d, a.e, None, chain::field::CARD + 0).int();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,search_count,search_count,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, search_count as u16, search_count as u16, false, NO_EX).nil();
    // if #g==search_count and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 then
    if g.len() as i64 == search_count && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g.clone());
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, tp);
        // if Duel.IsExistingMatchingCard(Card.IsAbleToGrave,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX) && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
            // local sg=Duel.SelectMatchingCard(tp,Card.IsAbleToGrave,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
            let sg = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
            // if #sg==0 then return end
            if sg.is_empty() {
                return;
            }
            // Duel.HintSelection(sg)
            duel::hint_selection(d, sg.clone(), true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoGrave(sg,REASON_EFFECT)
            duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
        }
    }
}
