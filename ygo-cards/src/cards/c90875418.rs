//! Theorealize Medius
//! 予幻使 メディウス
use super::prelude::*;

pub const ID: u32 = 90875418;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_POWER_PATRON, SET_THEOREALIZE, SET_ARS_MAGNA, SET_DIACTORUS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_SEARCH + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.handspcon)
    effect::set_condition(d, e1, Some(condition(handspcon)));
    // e1:SetTarget(s.handsptg)
    effect::set_target(d, e1, Some(cost_target(handsptg)));
    // e1:SetOperation(s.handspop)
    effect::set_operation(d, e1, Some(operation(handspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);

    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_REMOVE)
    effect::set_code(d, e2, EVENT_REMOVE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.effcon)
    effect::set_condition(d, e2, Some(condition(effcon)));
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.handspcon(e,tp,eg,ep,ev,re,r,rp)`
fn handspcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_POWER_PATRON),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_POWER_PATRON as u16]))), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
}
/// `s.handsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn handsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.tohandtheorealizefilter(c)`
fn tohandtheorealizefilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_THEOREALIZE) and c:IsSpellTrap() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_THEOREALIZE as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.handspop(e,tp,eg,ep,ev,re,r,rp)`
fn handspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0
    //     and Duel.IsExistingMatchingCard(s.tohandtheorealizefilter,tp,LOCATION_DECK,0,1,nil)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
        && duel::is_existing_matching_card(d, Some(&tohandtheorealizefilter), tp, LOCATION_DECK, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.tohandtheorealizefilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&tohandtheorealizefilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
    }
}
/// `s.effconfilter(c)`
fn effconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsFaceup() and c:IsLocation(LOCATION_REMOVED)
    card::is_monster(d, c) && card::is_faceup(d, c) && card::is_location(d, c, LOCATION_REMOVED)
}
/// `s.effcon(e,tp,eg,ep,ev,re,r,rp)`
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.effconfilter,1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &effconfilter, 1, NO_EX)
}
/// `s.tohandarsmagnafilter(c)`
fn tohandarsmagnafilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ARS_MAGNA) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_ARS_MAGNA as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.spfilter(c,e,tp,mc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: Option<CardId>) -> bool {
    // return c:IsSetCard(SET_DIACTORUS) and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_DIACTORUS as u16])
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.map_or(NO_EX, Exclude::Card), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local option_1=Duel.IsExistingMatchingCard(s.tohandarsmagnafilter,tp,LOCATION_DECK,0,1,nil)
    let option_1 = duel::is_existing_matching_card(d, Some(&tohandarsmagnafilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local option_2=c:IsAbleToRemove() and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c)
    let option_2 = card::is_able_to_remove(d, c, None, Some(POS_FACEUP as u8), Some(REASON_EFFECT))
        && duel::is_existing_matching_card(d, Some(&|d, sc| spfilter(d, sc, a.e, tp, Some(c))), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    // if chk==0 then return option_1 or option_2 end
    if a.chk == Some(0) {
        return option_1 || option_2;
    }
    // local choice=Duel.SelectEffect(tp,
    //     {option_1,aux.Stringid(id,3)},
    //     {option_2,aux.Stringid(id,4)})
    let choice = duel::select_effect(d, tp, &[(option_1, aux::stringid(ID, 3)), (option_2, aux::stringid(ID, 4))]);
    let choice_v = choice.map_or(0, |v| v as i64);
    // e:GetChainData().choice=choice
    chain::set_effect_data(d, a.e, None, chain::field::CARD + 1, Value::Int(choice_v));
    // if choice==1 then
    if choice == Some(1) {
        // e:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND)
        effect::set_category(d, a.e, CATEGORY_SEARCH + CATEGORY_TOHAND);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // elseif choice==2 then
    } else if choice == Some(2) {
        // e:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,c,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(c)), 1, tp, 0);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local choice=e:GetChainData().choice
    let choice = chain::effect_data(d, a.e, None, chain::field::CARD + 1).int();
    // if choice==1 then
    if choice == 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.tohandarsmagnafilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&tohandarsmagnafilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
    // elseif choice==2 then
    } else if choice == 2 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // if c:IsRelateToEffect(e) and Duel.Remove(c,POS_FACEUP,REASON_EFFECT)>0 and c:IsLocation(LOCATION_REMOVED) then
        if card::is_relate_to_effect(d, c, a.e)
            && duel::remove(d, c, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0
            && card::is_location(d, c, LOCATION_REMOVED)
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp)
            let g = duel::select_matching_card(d, tp, Some(&|d, sc| spfilter(d, sc, a.e, tp, None)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
