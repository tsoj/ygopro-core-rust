//! Neiroy, the Dream Mirror Traitor
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 90176467;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[18189187, CARD_DREAM_MIRROR_JOY, CARD_DREAM_MIRROR_TERROR],
    listed_series: &[SET_DREAM_MIRROR],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Search and change Attribute
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Search and Special Summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(18189187) and c:IsAbleToHand()
    card::is_code(d, c, &[18189187]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
        // if c:IsRelateToEffect(e) and c:IsFaceup() and not c:IsAttribute(ATTRIBUTE_LIGHT) and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if card::is_relate_to_effect(d, c, a.e)
            && card::is_faceup(d, c)
            && !card::is_attribute(d, c, ATTRIBUTE_LIGHT)
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Change Attribute to LIGHT
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
            effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
            // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
            effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
            // e1:SetRange(LOCATION_MZONE)
            effect::set_range(d, e1, LOCATION_MZONE);
            // e1:SetValue(ATTRIBUTE_LIGHT)
            effect::set_value(d, e1, ATTRIBUTE_LIGHT as i64);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
    }
}
/// `s.spcfilter(c,e,tp)`
fn spcfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_DREAM_MIRROR) and c:HasLevel() and Duel.GetMZoneCount(tp,c)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp,c)
    card::is_set_card(d, c, &[SET_DREAM_MIRROR as u16])
        && card::has_level(d, c)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d: &mut Duel, x| spfilter(d, x, e, tp, c)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,tc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) -> bool {
    // return c:IsSetCard(SET_DREAM_MIRROR) and not c:IsLevel(tc:GetLevel()) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    //   and Duel.IsExistingMatchingCard(s.thfilter2,tp,LOCATION_DECK,0,1,nil,c)
    let lv = card::get_level(d, tc);
    card::is_set_card(d, c, &[SET_DREAM_MIRROR as u16])
        && !card::is_level(d, c, &[lv])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
        && duel::is_existing_matching_card(d, Some(&|d: &mut Duel, x| thfilter2(d, x, c)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.thfilter2(c,tc)`
fn thfilter2(d: &mut Duel, c: CardId, tc: CardId) -> bool {
    // return c:IsCode(CARD_DREAM_MIRROR_JOY,CARD_DREAM_MIRROR_TERROR) and c:IsAbleToHand() and tc:ListsCode(c:GetCode())
    card::is_code(d, c, &[CARD_DREAM_MIRROR_JOY, CARD_DREAM_MIRROR_TERROR])
        && card::is_able_to_hand(d, c, None)
        && {
            let codes = card::get_code(d, c);
            card::lists_code(d, tc, &codes)
        }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.spcfilter,1,false,nil,c,e,tp) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&|d: &mut Duel, x| spcfilter(d, x, a.e, a.tp)), 1, 1, false, None, c.into());
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.spcfilter,1,1,false,nil,c,e,tp)
    let g = duel::select_release_group_cost(d, a.tp, Some(&|d: &mut Duel, x| spcfilter(d, x, a.e, a.tp)), 1, 1, false, None, c.into());
    // e:SetLabelObject(g:GetFirst())
    let first = group::get_first(d, &g);
    effect::set_label_object(d, a.e, first.map_or(Value::Nil, Value::Card));
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local rc=e:GetLabelObject()
    // if not rc or Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
    let rc = match effect::get_label_object(d, a.e).as_card() {
        Some(rc) => rc,
        None => return,
    };
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,rc):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&|d: &mut Duel, x| spfilter(d, x, a.e, a.tp, rc)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if sc then
    if let Some(sc) = group::get_first(d, &sel) {
        // Duel.ConfirmCards(1-tp,sc)
        duel::confirm_cards(d, 1 - a.tp, sc);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter2,tp,LOCATION_DECK,0,1,1,nil,sc)
        let g = duel::select_matching_card(d, a.tp, Some(&|d: &mut Duel, x| thfilter2(d, x, sc)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 then
        if !g.is_empty() && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - a.tp, g);
            // Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
            duel::special_summon(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        }
    }
}
