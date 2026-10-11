//! Materiactor Zeptwing
use super::prelude::*;

pub const ID: u32 = 64911387;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MATERIACTOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_MATERIACTOR),tp,LOCATION_ONFIELD,0,1,nil) end)
    effect::set_condition(d, e1, Some(condition(cond1)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 "Materiactor" card from your Deck to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.thsptg)
    effect::set_target(d, e2, Some(cost_target(thsptg)));
    // e2:SetOperation(s.thspop)
    effect::set_operation(d, e2, Some(operation(thspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e2:Clone()
    let e4 = effect::clone(d, e2);
    // e4:SetCode(EVENT_FLIP)
    effect::set_code(d, e4, EVENT_FLIP);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `function(e,tp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_MATERIACTOR),tp,LOCATION_ONFIELD,0,1,nil) end`
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(
        d,
        Some(&|d: &mut Duel, c: CardId| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MATERIACTOR as u16])),
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
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
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
        duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MATERIACTOR) and c:IsAbleToHand() and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_MATERIACTOR as u16]) && card::is_able_to_hand(d, c, None) && !card::is_code(d, c, &[ID])
}
/// `s.thsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.thspfilter(c,e,tp,ft)`
fn thspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // return c:IsLevel(3) and c:IsType(TYPE_NORMAL)
    //     and (c:IsAbleToHand() or (ft>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::is_level(d, c, &[3])
        && card::is_type(d, c, TYPE_NORMAL)
        && (card::is_able_to_hand(d, c, None)
            || (ft > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
/// `s.thspop(e,tp,eg,ep,ev,re,r,rp)`
fn thspop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
        // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
        let nv = aux::necro_valley_filter(move |d: &mut Duel, c: CardId| thspfilter(d, c, e, tp, ft));
        // if Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.thspfilter),tp,LOCATION_GRAVE,0,1,nil,e,tp,ft)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if duel::is_existing_matching_card(d, Some(&nv), tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
        {
            // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,3))
            duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 3));
            // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thspfilter),tp,LOCATION_GRAVE,0,1,1,nil,e,tp,ft):GetFirst()
            let sg = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
            let Some(sc) = group::get_first(d, &sg) else {
                // if not sc then return end
                return;
            };
            // Duel.BreakEffect()
            duel::break_effect(d);
            // aux.ToHandOrElse(sc,tp,
            //     function() return ft>0 and sc:IsCanBeSpecialSummoned(e,0,tp,false,false) end,
            //     function() Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP) end,
            //     aux.Stringid(id,4))
            aux::to_hand_or_else(
                d,
                CardSet::from_card(sc),
                tp,
                Some(&|d: &mut Duel, c: CardId| ft > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)),
                Some(&|d: &mut Duel, _g: &CardSet| {
                    duel::special_summon(d, CardSet::from_card(sc), 0, tp, tp, false, false, POS_FACEUP as u8, None);
                    0
                }),
                Some(aux::stringid(ID, 4)),
            );
        }
    }
}
