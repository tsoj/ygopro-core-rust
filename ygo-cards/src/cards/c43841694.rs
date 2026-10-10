//! Spellbook of Miracles
use super::prelude::*;

pub const ID: u32 = 43841694;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPELLBOOK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
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
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsType(TYPE_XYZ) and c:IsRace(RACE_SPELLCASTER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_type(d, c, TYPE_XYZ)
        && card::is_race(d, c, RACE_SPELLCASTER)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_SPELLBOOK) and c:IsSpell()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SPELLBOOK as u16]) && card::is_spell(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil,e,tp)
    //     and Duel.IsExistingTarget(s.filter2,tp,LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&filter2), tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g1=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g1 = duel::select_target(d, tp, Some(&|d, c| filter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // e:SetLabelObject(g1:GetFirst())
    let first = group::get_first(d, &g1.clone().nil());
    let label = match first {
        Some(tc) => Value::Card(tc),
        None => Value::Nil,
    };
    effect::set_label_object(d, a.e, label);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g2=Duel.SelectTarget(tp,s.filter2,tp,LOCATION_REMOVED,0,1,2,nil)
    let _g2 = duel::select_target(d, tp, Some(&filter2), tp, LOCATION_REMOVED, 0, 1, 2, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g1,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g1, 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let chain = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    let g = chain[0].as_group().cloned().nil();
    // local sg=g:Filter(Card.IsRelateToEffect,nil,e)
    let mut sg = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card();
    // sg:RemoveCard(tc)
    if let Some(tc) = tc {
        group::remove_card(&mut sg, &CardSet::from_card(tc));
    }
    let tc = tc.nil();
    // if tc:IsRelateToEffect(e) and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, tc, a.e) && duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.Overlay(tc,sg)
            duel::overlay(d, tc, sg, false);
        }
    }
}
