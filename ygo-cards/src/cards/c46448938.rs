//! Spellbook of Judgment
use super::prelude::*;

pub const ID: u32 = 46448938;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPELLBOOK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_SEARCH + CATEGORY_TOHAND);
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
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.regcon)
    effect::set_condition(d, e1, Some(condition(regcon)));
    // e1:SetOperation(s.regop1)
    effect::set_operation(d, e1, Some(operation(regop1)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_CHAIN_NEGATED)
    effect::set_code(d, e2, EVENT_CHAIN_NEGATED);
    // e2:SetCondition(s.regcon)
    effect::set_condition(d, e2, Some(condition(regcon)));
    // e2:SetOperation(s.regop2)
    effect::set_operation(d, e2, Some(operation(regop2)));
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.effcon)
    effect::set_condition(d, e3, Some(condition(effcon)));
    // e3:SetOperation(s.effop)
    effect::set_operation(d, e3, Some(operation(effop)));
    // e3:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, a.tp);
    // e1:SetLabelObject(e3)
    effect::set_label_object(d, e1, Value::Effect(e3));
    // e2:SetLabelObject(e3)
    effect::set_label_object(d, e2, Value::Effect(e3));
}
/// `s.regcon(e,tp,eg,ep,ev,re,r,rp)`: `return re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellEffect()`
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::is_spell_effect(d, re)
}
/// `s.regop1(e,tp,eg,ep,ev,re,r,rp)`
fn regop1(d: &mut Duel, a: &EffArgs) {
    // local ct=e:GetLabelObject():GetLabel()
    let obj = effect::get_label_object(d, a.e).as_effect().nil();
    let ct = effect::get_label(d, obj);
    // e:GetLabelObject():SetLabel(ct+1)
    effect::set_label(d, obj, &[ct + 1]);
}
/// `s.regop2(e,tp,eg,ep,ev,re,r,rp)`
fn regop2(d: &mut Duel, a: &EffArgs) {
    // local ct=e:GetLabelObject():GetLabel()
    let obj = effect::get_label_object(d, a.e).as_effect().nil();
    let mut ct = effect::get_label(d, obj);
    // if ct==0 then ct=1 end
    if ct == 0 {
        ct = 1;
    }
    // e:GetLabelObject():SetLabel(ct-1)
    effect::set_label(d, obj, &[ct - 1]);
}
/// `s.effcon(e,tp,eg,ep,ev,re,r,rp)`: `return e:GetLabel()>0`
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    effect::get_label(d, a.e) > 0
}
/// `s.sfilter(c)`
fn sfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SPELLBOOK) and c:GetCode()~=id and c:IsSpell() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_SPELLBOOK as u16]) && card::get_code(d, c).first() != Some(&ID) && card::is_spell(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.spfilter(c,lv,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, lv: u32, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(lv) and c:IsRace(RACE_SPELLCASTER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_below(d, c, lv) && card::is_race(d, c, RACE_SPELLCASTER) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.sfilter,tp,LOCATION_DECK,0,1,e:GetLabel(),nil)
    let max = effect::get_label(d, a.e) as u16;
    let g = duel::select_matching_card(d, a.tp, Some(&sfilter), a.tp, LOCATION_DECK, 0, 1, max, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        let n = g.len() as u32;
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g.clone());
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,#g,e,tp)
        //   and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, n, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
            // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,#g,e,tp)
            let sg = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, n, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
