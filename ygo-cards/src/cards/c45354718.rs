//! Lyrilusc - Bird Call
//! ＬＬ－バード・コール
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 45354718;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LYRILUSC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_LYRILUSC) and c:IsMonster() and (c:IsAbleToHand() or c:IsAbleToGrave())
    card::is_set_card(d, c, &[SET_LYRILUSC as u16])
        && card::is_monster(d, c)
        && (card::is_able_to_hand(d, c, None) || card::is_able_to_grave(d, c, None))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.spfilter(c,e,tp,tc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) -> bool {
    // return c:IsSetCard(SET_LYRILUSC) and not c:IsCode(tc:GetCode()) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    let codes = card::get_code(d, tc);
    card::is_set_card(d, c, &[SET_LYRILUSC as u16])
        && !card::is_code(d, c, &codes)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    // if not tc then return end
    let Some(tc) = group::get_first(d, &g) else {
        return;
    };
    // aux.ToHandOrElse(tc,tp)
    aux::to_hand_or_else(d, tc, a.tp, None, None, None);
    // if not tc:IsLocation(LOCATION_HAND|LOCATION_GRAVE) then return end
    if !card::is_location(d, tc, LOCATION_HAND | LOCATION_GRAVE) {
        return;
    }
    // local sg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND,0,nil,e,tp,tc)
    let sg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, tc)), a.tp, LOCATION_HAND, 0, NO_EX);
    // if #sg>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if !sg.is_empty()
        && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sc=sg:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &sg, a.tp, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel).nil();
        // Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
