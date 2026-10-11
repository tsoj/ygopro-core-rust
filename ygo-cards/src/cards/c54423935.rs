//! Raidraptor Replica
use super::prelude::*;

pub const ID: u32 = 54423935;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RAIDRAPTOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon 1 "Raidraptor" monster from deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_END_PHASE));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// s.filter(c,e,tp)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_RAIDRAPTOR)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp,c:GetCode())
    let code = card::get_code(d, c)[0];
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_RAIDRAPTOR as u16])
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, code)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
// s.spfilter(c,e,tp,cd)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, cd: u32) -> bool {
    // return c:IsCode(cd) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[cd]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and chkc:IsCode(e:GetLabel()) end
    if let Some(chkc) = a.chkc() {
        let label = effect::get_label(d, e) as u32;
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && card::is_code(d, chkc, &[label]);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // e:SetLabel(g:GetFirst():GetCode())
    let first = group::get_first(d, &g).nil();
    let code = card::get_code(d, first)[0];
    effect::set_label(d, e, &[code as i64]);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
// s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, e) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,tc:GetCode())
            let tcode = card::get_code(d, tc)[0];
            let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, e, tp, tcode)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            // local sc=g:GetFirst()
            let sc = group::get_first(d, &g);
            // if sc and Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)>0 then
            if let Some(sc) = sc {
                if duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
                    // Cannot be targeted for attacks
                    // local e1=Effect.CreateEffect(e:GetHandler())
                    let h = effect::get_handler(d, e).nil();
                    let e1 = effect::create_effect(d, h);
                    // e1:SetDescription(3007)
                    effect::set_description(d, e1, 3007);
                    // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
                    effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
                    // e1:SetType(EFFECT_TYPE_SINGLE)
                    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                    // e1:SetCode(EFFECT_CANNOT_BE_BATTLE_TARGET)
                    effect::set_code(d, e1, EFFECT_CANNOT_BE_BATTLE_TARGET);
                    // e1:SetLabelObject(sc)
                    effect::set_label_object(d, e1, Value::Card(sc));
                    // e1:SetLabel(sc:GetFieldID())
                    let fid = card::get_field_id(d, sc);
                    effect::set_label(d, e1, &[fid as i64]);
                    // e1:SetCondition(s.imcon)
                    effect::set_condition(d, e1, Some(condition(imcon)));
                    // e1:SetValue(aux.imval1)
                    effect::set_value_fn(d, e1, aux::imval1());
                    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                    // tc:RegisterEffect(e1)
                    card::register_effect(d, tc, e1, false);
                    // Cannot be targeted by opponent's card effects
                    // local e2=e1:Clone()
                    let e2 = effect::clone(d, e1);
                    // e2:SetDescription(3061)
                    effect::set_description(d, e2, 3061);
                    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_CLIENT_HINT)
                    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_CLIENT_HINT, 0);
                    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
                    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
                    // e2:SetValue(aux.tgoval)
                    effect::set_value_fn(d, e2, aux::tgoval());
                    // tc:RegisterEffect(e2)
                    card::register_effect(d, tc, e2, false);
                }
            }
        }
    }
}
// s.imcon(e)
fn imcon(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // local sc=e:GetLabelObject()
    // return sc and sc:GetFieldID()==e:GetLabel() and sc:IsFaceup() and sc:IsLocation(LOCATION_MZONE) and sc:IsControler(tp)
    let Some(sc) = effect::get_label_object(d, e).as_card() else {
        return false;
    };
    let fid = card::get_field_id(d, sc) as i64;
    fid == effect::get_label(d, e)
        && card::is_faceup(d, sc)
        && card::is_location(d, sc, LOCATION_MZONE)
        && card::is_controler(d, sc, tp)
}
