//! Gunkan Suship Shirauo
//! Logical Nonsense
use super::prelude::*;

pub const ID: u32 = 78362751;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID, CARD_SUSHIP_SHARI], listed_series: &[SET_GUNKAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --If you control a "Gunkan Suship Shari", or an Xyz Monster that has "Gunkan Suship Shari" as material: You can Special Summon this card from your hand
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
    // e1:SetCondition(s.selfspcon)
    effect::set_condition(d, e1, Some(condition(selfspcon)));
    // e1:SetTarget(s.selfsptg)
    effect::set_target(d, e1, Some(cost_target(selfsptg)));
    // e1:SetOperation(s.selfspop)
    effect::set_operation(d, e1, Some(operation(selfspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --During your Main Phase: You can Special Summon 1 "Gunkan" monster from your hand, except "Gunkan Suship Shirauo", then you can take any number of "Gunkan Suship Shari" from your Deck or GY and place them on top of your Deck in any order
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.selfspconfilter(c)
fn selfspconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and (c:IsCode(CARD_SUSHIP_SHARI) or c:GetOverlayGroup():IsExists(Card.IsCode,1,nil,CARD_SUSHIP_SHARI))
    if !card::is_faceup(d, c) {
        return false;
    }
    if card::is_code(d, c, &[CARD_SUSHIP_SHARI]) {
        return true;
    }
    let overlay = card::get_overlay_group(d, c);
    group::is_exists(d, &overlay, &|d, x| card::is_code(d, x, &[CARD_SUSHIP_SHARI]), 1, NO_EX)
}
// function s.selfspcon(e,tp,eg,ep,ev,re,r,rp)
fn selfspcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.selfspconfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&selfspconfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
// function s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
// function s.selfspop(e,tp,eg,ep,ev,re,r,rp)
fn selfspop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GUNKAN) and not c:IsCode(id) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_GUNKAN as u16]) && !card::is_code(d, c, &[ID]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
// function s.tdfilter(c)
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_SUSHIP_SHARI) and (c:IsLocation(LOCATION_DECK) or c:IsAbleToDeck())
    card::is_code(d, c, &[CARD_SUSHIP_SHARI]) && (card::is_location(d, c, LOCATION_DECK) || card::is_able_to_deck(d, c))
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g==0 or Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)==0 then return end
    if g.is_empty() || duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    // local dg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.tdfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,nil)
    let f = aux::necro_valley_filter(tdfilter);
    let dg = duel::get_matching_group(d, Some(&f), tp, LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX);
    // if #dg==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,2)) then return end
    if dg.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local sg=dg:Select(tp,1,#dg,nil)
    let sg = group::select(d, &dg, tp, 1, dg.len() as u16, false, NO_EX).nil();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.HintSelection(sg)
        duel::hint_selection(d, sg.clone(), true);
        // Duel.SendtoDeck(sg,nil,SEQ_DECKTOP,REASON_EFFECT)
        duel::send_to_deck(d, sg.clone(), None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
        // Duel.MoveToDeckTop(sg)
        duel::move_to_deck_top(d, sg.clone());
        // if #sg>1 then
        if sg.len() > 1 {
            // Duel.SortDecktop(tp,tp,#sg)
            duel::sort_decktop(d, tp, tp, sg.len() as u16);
        }
    }
}
