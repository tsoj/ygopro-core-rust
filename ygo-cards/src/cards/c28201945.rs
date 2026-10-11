//! Traptrix Nepenthes
use super::prelude::*;

pub const ID: u32 = 28201945;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_TRAP_HOLE,SET_HOLE,SET_TRAPTRIX}
    listed_series: &[SET_TRAP_HOLE, SET_HOLE, SET_TRAPTRIX],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // immune
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetValue(s.efilter)
    effect::set_value_fn(d, e1, lua_fn(efilter));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // spsummon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.efilter(e,te)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    // local c=te:GetHandler()
    let c = effect::get_handler(d, a.e(1)).nil();
    // return c:IsNormalTrap() and (c:IsSetCard(SET_TRAP_HOLE) or c:IsSetCard(SET_HOLE))
    card::is_normal_trap(d, c) && (card::is_set_card(d, c, &[SET_TRAP_HOLE as u16]) || card::is_set_card(d, c, &[SET_HOLE as u16]))
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    // local c=re:GetHandler()
    let c = effect::get_handler(d, re).nil();
    // return rp==tp and re:IsHasType(EFFECT_TYPE_ACTIVATE) and c:IsNormalTrap() and (c:IsSetCard(SET_TRAP_HOLE) or c:IsSetCard(SET_HOLE))
    a.rp == a.tp
        && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
        && card::is_normal_trap(d, c)
        && (card::is_set_card(d, c, &[SET_TRAP_HOLE as u16]) || card::is_set_card(d, c, &[SET_HOLE as u16]))
}
/// `s.filter(c,e,tp,ft)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // return c:IsSetCard(SET_TRAPTRIX) and c:GetCode()~=id and c:IsMonster() and (c:IsAbleToHand() or (ft>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::is_set_card(d, c, &[SET_TRAPTRIX as u16])
        && card::get_code(d, c).first() != Some(&ID)
        && card::is_monster(d, c)
        && (card::is_able_to_hand(d, c, None) || (ft > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return e:GetHandler():IsRelateToEffect(e)
    if a.chk == Some(0) {
        let c = effect::get_handler(d, e).nil();
        // and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,e,tp,ft) end
        return card::is_relate_to_effect(d, c, e)
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, e, tp, ft)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil,e,tp,ft)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, e, tp, ft)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    // if tc then
    if let Some(tc) = group::get_first(d, &g) {
        // if ft>0 and tc:IsCanBeSpecialSummoned(e,0,tp,false,false)
        //     and (not tc:IsAbleToHand() or Duel.SelectYesNo(tp,aux.Stringid(id,1))) then
        if ft > 0
            && card::is_can_be_special_summoned(d, tc, e, 0, tp, false, false, None, None, None)
            && (!card::is_able_to_hand(d, tc, None) || duel::select_yes_no(d, tp, aux::stringid(ID, 1)))
        {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        } else {
            // Duel.SendtoHand(tc,nil,REASON_EFFECT)
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,tc)
            duel::confirm_cards(d, 1 - tp, tc);
        }
    }
}
