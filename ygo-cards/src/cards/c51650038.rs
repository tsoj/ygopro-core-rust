//! Madolche Dessert
//! マドルチェ・デセール
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 51650038;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MADOLCHE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Return 2 Effect monsters, including a "Madolche" monster, to the hand/Extra Deck and Special Summon 1 "Madolche" monster from the hand/Extra Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_TOEXTRA+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_TOEXTRA + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Attach this card to an Xyz monster you control
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetCode(EVENT_TO_DECK)
    effect::set_code(d, e2, EVENT_TO_DECK);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.attachcon)
    effect::set_condition(d, e2, Some(condition(attachcon)));
    // e2:SetTarget(s.attachtg)
    effect::set_target(d, e2, Some(cost_target(attachtg)));
    // e2:SetOperation(s.attachop)
    effect::set_operation(d, e2, Some(operation(attachop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thfilter(c,e)`
fn thfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsType(TYPE_EFFECT) and c:IsFaceup() and c:IsCanBeEffectTarget(e)
    card::is_type(d, c, TYPE_EFFECT) && card::is_faceup(d, c) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsSetCard,1,nil,SET_MADOLCHE)
    (group::is_exists(d, sg, &|d, c| card::is_set_card(d, c, &[SET_MADOLCHE as u16]), 1, NO_EX), false)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local rg=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,e)
    let rg = duel::get_matching_group(d, Some(&|d, c| thfilter(d, c, e)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return matches!(
            aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false),
            Err(true)
        );
    }
    // local tg=aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,1,tp,HINTMSG_TARGET)
    let tg = aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_TARGET as u64), None, None, false)
        .ok()
        .nil();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,tg,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(tg), 2, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_EXTRA) as i32);
    true
}
/// `s.spfilter(c,e,tp,atk)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, atk: i64) -> bool {
    // if c:IsLocation(LOCATION_HAND) and Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return false end
    if card::is_location(d, c, LOCATION_HAND) && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return false;
    }
    // if c:IsLocation(LOCATION_EXTRA) and Duel.GetLocationCountFromEx(tp,tp,nil,c)<=0 then return false end
    if card::is_location(d, c, LOCATION_EXTRA)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 <= 0
    {
        return false;
    }
    // return c:IsSetCard(SET_MADOLCHE) and c:IsAttackBelow(atk) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_MADOLCHE as u16])
        && card::is_attack_below(d, c, atk as i32)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local tg=Duel.GetTargetCards(e):Filter(aux.FaceupFilter(Card.IsType,TYPE_EFFECT),nil)
    let tg0 = duel::get_target_cards(d, e).nil();
    let tg = group::filter(d, &tg0, &|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT), NO_EX);
    // if #tg==0 then return end
    if tg.is_empty() {
        return;
    }
    // local exg,hg=tg:Split(Card.IsType,nil,TYPE_EXTRA)
    let (exg, hg) = group::split(d, &tg, &|d, c| card::is_type(d, c, TYPE_EXTRA), NO_EX);
    // local ct=0
    let mut ct: i32 = 0;
    // if #exg>0 then ct=Duel.SendtoDeck(exg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT) end
    if !exg.is_empty() {
        ct = duel::send_to_deck(d, exg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
    // if #hg>0 then ct=ct+Duel.SendtoHand(hg,nil,REASON_EFFECT) end
    if !hg.is_empty() {
        ct += duel::send_to_hand(d, hg.clone(), None, REASON_EFFECT, None);
    }
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // if hg:IsExists(function(c) return c:IsLocation(LOCATION_HAND) and c:IsControler(tp) end,1,nil) then Duel.ShuffleHand(tp) end
    if group::is_exists(d, &hg, &|d, c| card::is_location(d, c, LOCATION_HAND) && card::is_controler(d, c, tp), 1, NO_EX) {
        duel::shuffle_hand(d, tp);
    }
    // local atk=tg:Filter(Card.IsLocation,nil,LOCATION_HAND|LOCATION_EXTRA):GetSum(Card.GetBaseAttack)
    let loc = group::filter(d, &tg, &|d, c| card::is_location(d, c, LOCATION_HAND | LOCATION_EXTRA), NO_EX);
    let atk = group::get_sum(d, &loc, &|d, c| card::get_base_attack(d, c) as i64);
    // if Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_EXTRA,0,1,nil,e,tp,atk) and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, atk)), tp, LOCATION_HAND | LOCATION_EXTRA, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_EXTRA,0,1,1,nil,e,tp,atk)
        let sg = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, atk)), tp, LOCATION_HAND | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
/// `s.attachconfilter(c,tp)`
fn attachconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_MADOLCHE) and c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_GRAVE)
    card::is_set_card(d, c, &[SET_MADOLCHE as u16]) && card::is_previous_controler(d, c, tp) && card::is_previous_location(d, c, LOCATION_GRAVE)
}
/// `s.attachcon(e,tp,eg,ep,ev,re,r,rp)`
fn attachcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.attachconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| attachconfilter(d, c, tp), 1, NO_EX)
}
/// `s.attachtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,TYPE_XYZ),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(h)), 1, tp, 0);
    true
}
/// `s.attachop(e,tp,eg,ep,ev,re,r,rp)`
fn attachop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // local tc=Duel.SelectMatchingCard(tp,aux.FaceupFilter(Card.IsType,TYPE_XYZ),tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc then
    if let Some(tc) = tc {
        // Duel.HintSelection(tc)
        duel::hint_selection(d, CardSet::from_card(tc), true);
        // if tc:IsImmuneToEffect(e) then return end
        if card::is_immune_to_effect(d, tc, a.e) {
            return;
        }
        // Duel.Overlay(tc,c)
        duel::overlay(d, tc, CardSet::from_card(c), false);
    }
}
