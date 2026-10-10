//! Ecclesia and the Dark Dragon
use super::prelude::*;

pub const ID: u32 = 78397661;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ALBAZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1+ non-Tuner monsters
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Banish this card (until the End Phase), and if you do, Special Summon 1 "Fallen of Albaz" or 1 Level 4 or lower monster that mentions it from your Deck or GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(main_phase_cond)));
    // e1:SetTarget(s.rmsptg)
    effect::set_target(d, e1, Some(cost_target(rmsptg)));
    // e1:SetOperation(s.rmspop)
    effect::set_operation(d, e1, Some(operation(rmspop)));
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Shuffle both 1 Level 8 Fusion Monster in your GY or banishment and 1 card on the field, and this card into the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_TOEXTRA)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_TOEXTRA);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn main_phase_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsMainPhase()
    duel::is_main_phase(d, None)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return (c:IsCode(CARD_ALBAZ) or (c:IsLevelBelow(4) and c:ListsCode(CARD_ALBAZ))) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    (card::is_code(d, c, &[CARD_ALBAZ]) || (card::is_level_below(d, c, 4) && card::lists_code(d, c, &[CARD_ALBAZ])))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn rmsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return c:IsAbleToRemove() and Duel.GetMZoneCount(tp,c)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, c, None, None, None)
            && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(c)), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
fn rmspop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) and aux.RemoveUntil(c,nil,REASON_EFFECT,PHASE_END,id,e,tp,aux.DefaultFieldReturnOp)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if card::is_relate_to_effect(d, c, e)
        && aux::remove_until(d, c, None, REASON_EFFECT, PHASE_END, ID, e, tp, Some(aux::default_field_return()), None, None, None, None, None).is_some()
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp)
        let nf = aux::necro_valley_filter(|d, x| spfilter(d, x, e, tp));
        let g = duel::select_matching_card(d, tp, Some(&nf), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
fn fustdfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsLevel(8) and c:IsFusionMonster() and c:IsFaceup() and c:IsAbleToExtra() and c:IsCanBeEffectTarget(e)
    card::is_level(d, c, &[8])
        && card::is_fusion_monster(d, c)
        && card::is_faceup(d, c)
        && card::is_able_to_extra(d, c)
        && card::is_can_be_effect_target(d, c, Some(e))
}
fn fieldtdfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsAbleToDeck() and c:IsCanBeEffectTarget(e)
    card::is_able_to_deck(d, c) && card::is_can_be_effect_target(d, c, Some(e))
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return c:IsAbleToExtra()
    //     and Duel.IsExistingMatchingCard(s.fustdfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,c,e)
    //     and Duel.IsExistingMatchingCard(s.fieldtdfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil,e) end
    if a.chk == Some(0) {
        return card::is_able_to_extra(d, c)
            && duel::is_existing_matching_card(d, Some(&|d, x| fustdfilter(d, x, e)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, c.into())
            && duel::is_existing_matching_card(d, Some(&|d, x| fieldtdfilter(d, x, e)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // repeat ... until fieldg~=nil (fusiong and fieldg are the last selections)
    let (fusiong, fieldg) = loop {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        // fusiong=Duel.SelectMatchingCard(tp,s.fustdfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,c,e)
        let fusiong = duel::select_matching_card(d, tp, Some(&|d, x| fustdfilter(d, x, e)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, c.into()).nil();
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        // fieldg=Duel.SelectMatchingCard(tp,s.fieldtdfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,true,nil,e)
        if let Some(fieldg) = duel::select_matching_card(d, tp, Some(&|d, x| fieldtdfilter(d, x, e)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, true, NO_EX) {
            break (fusiong, fieldg);
        }
    };
    // local tg=fusiong:Merge(fieldg)
    let mut tg = fusiong;
    group::add_card(&mut tg, &fieldg);
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,tg+c,3,tp,0)
    let mut tgc = tg.clone();
    group::add_card(&mut tgc, &CardSet::from_card(c));
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(tgc), 3, tp, 0);
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tg=Duel.GetTargetCards(e)
    let mut tg = duel::get_target_cards(d, e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, e) {
        // tg:AddCard(c)
        group::add_card(&mut tg, &CardSet::from_card(c));
        // tg:Match(Card.IsAbleToDeck,nil)
        group::match_(d, &mut tg, &|d, x| card::is_able_to_deck(d, x), NO_EX);
        // if #tg==3 then
        if tg.len() == 3 {
            // Duel.SendtoDeck(tg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
            duel::send_to_deck(d, tg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        }
    }
}
