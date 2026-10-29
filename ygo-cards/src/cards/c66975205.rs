//! Yummyusment☆Mignon
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 66975205;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_YUMMY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // "Yummy" monsters you control gain 500 ATK for each LIGHT Beast monster on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1, LOCATION_FZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_YUMMY))
    effect::set_target(d, e1, Some(effect_card_fn(yummy_target)));
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 Level 1 "Yummy" monster from your GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsLink,1),tp,LOCATION_MZONE,0,1,nil) end)
    effect::set_condition(d, e2, Some(condition(econd)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Place 2 "Yummy" monsters from your GY and/or banishment and this card on the bottom of the Deck in any order
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e3, CATEGORY_TODECK);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.tdtg)
    effect::set_target(d, e3, Some(cost_target(tdtg)));
    // e3:SetOperation(s.tdop)
    effect::set_operation(d, e3, Some(operation(tdop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// aux.TargetBoolFunction(Card.IsSetCard,SET_YUMMY)
fn yummy_target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_YUMMY as u16])
}
// e2:SetCondition(function(e,tp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsLink,1),tp,LOCATION_MZONE,0,1,nil) end)
fn econd(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_link(d, c, &[1])), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
// function s.atkfilter(c)
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_BEAST) and c:IsFaceup()
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_BEAST as u64) && card::is_faceup(d, c)
}
// function s.atkval(e,c)
fn atkval(d: &mut Duel, _e: EffectId) -> i64 {
    // return 500*Duel.GetMatchingGroupCount(s.atkfilter,0,LOCATION_MZONE,LOCATION_MZONE,nil)
    500 * duel::get_matching_group_count(d, Some(&atkfilter), 0, LOCATION_MZONE, LOCATION_MZONE, NO_EX) as i64
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_YUMMY) and c:IsLevel(1) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_YUMMY as u16]) && card::is_level(d, c, &[1]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, a.tp, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
// function s.tdfilter(c)
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_YUMMY) and c:IsMonster() and (c:IsAbleToDeck() or c:IsAbleToExtra()) and c:IsFaceup()
    card::is_set_card(d, c, &[SET_YUMMY as u16])
        && card::is_monster(d, c)
        && (card::is_able_to_deck(d, c) || card::is_able_to_extra(d, c))
        && card::is_faceup(d, c)
}
// function s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToDeck() and Duel.IsExistingTarget(s.tdfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,2,nil) end
    if a.chk == Some(0) {
        return card::is_able_to_deck(d, c)
            && duel::is_existing_target(d, Some(&tdfilter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 2, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.tdfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,2,2,nil)
    let g = duel::select_target(d, a.tp, Some(&tdfilter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 2, 2, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g+c,3,tp,0)
    let mut gc = g;
    group::add_card(&mut gc, &CardSet::from_card(c));
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(gc), 3, a.tp, 0);
    true
}
// function s.tdop(e,tp,eg,ep,ev,re,r,rp)
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local tg=Duel.GetTargetCards(e)
    let mut tg = duel::get_target_cards(d, a.e).nil();
    // if #tg==0 then return end
    if tg.is_empty() {
        return;
    }
    // tg:AddCard(c)
    group::add_card(&mut tg, &CardSet::from_card(c));
    // if Duel.SendtoDeck(tg,nil,SEQ_DECKBOTTOM,REASON_EFFECT)>0 then
    if duel::send_to_deck(d, tg, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None) > 0 {
        // local ct=Duel.GetOperatedGroup():Filter(Card.IsControler,nil,tp):FilterCount(Card.IsLocation,nil,LOCATION_DECK)
        let operated = duel::get_operated_group(d);
        let mine = group::filter(d, &operated, &|d, x| card::is_controler(d, x, a.tp), NO_EX);
        let ct = group::filter_count(d, &mine, &|d, x| card::is_location(d, x, LOCATION_DECK), NO_EX);
        // if ct>1 then
        if ct > 1 {
            // Duel.SortDeckbottom(tp,tp,ct)
            duel::sort_deckbottom(d, a.tp, a.tp, ct as u16);
        }
    }
}
