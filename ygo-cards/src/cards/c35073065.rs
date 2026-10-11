//! Illusory Snatcher
//! イリュージョン・スナッチ
use super::prelude::*;

pub const ID: u32 = 35073065;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_MSET)
    effect::set_code(d, e2, EVENT_MSET);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let ec = group::get_first(d, &eg);
    // return ep==tp and ec:IsTributeSummoned()
    a.ep == a.tp && card::is_tribute_summoned(d, ec.nil())
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, a.eg.clone().nil());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ec=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let ec = group::get_first(d, &eg);
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if Duel.SpecialSummonStep(c,0,tp,tp,false,false,POS_FACEUP) then
    if duel::special_summon_step(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
        let ec = ec.nil();
        // if ec:IsRelateToEffect(e) and ec:IsFaceup() then
        if card::is_relate_to_effect(d, ec, a.e) && card::is_faceup(d, ec) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CHANGE_RACE)
            effect::set_code(d, e1, EFFECT_CHANGE_RACE);
            // if ec:IsHasEffect(EFFECT_ADD_RACE) and not ec:IsHasEffect(EFFECT_CHANGE_RACE) then
            if !card::is_has_effect(d, ec, EFFECT_ADD_RACE, None).is_empty() && card::is_has_effect(d, ec, EFFECT_CHANGE_RACE, None).is_empty() {
                // e1:SetValue(ec:GetOriginalRace())
                let v = card::get_original_race(d, ec) as i64;
                effect::set_value(d, e1, v);
            } else {
                // e1:SetValue(ec:GetRace())
                let v = card::get_race(d, ec) as i64;
                effect::set_value(d, e1, v);
            }
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_CHANGE_ATTRIBUTE)
            effect::set_code(d, e2, EFFECT_CHANGE_ATTRIBUTE);
            // if ec:IsHasEffect(EFFECT_ADD_ATTRIBUTE) and not ec:IsHasEffect(EFFECT_CHANGE_ATTRIBUTE) then
            if !card::is_has_effect(d, ec, EFFECT_ADD_ATTRIBUTE, None).is_empty() && card::is_has_effect(d, ec, EFFECT_CHANGE_ATTRIBUTE, None).is_empty() {
                // e2:SetValue(ec:GetOriginalAttribute())
                let v = card::get_original_attribute(d, ec) as i64;
                effect::set_value(d, e2, v);
            } else {
                // e2:SetValue(ec:GetAttribute())
                let v = card::get_attribute(d, ec) as i64;
                effect::set_value(d, e2, v);
            }
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
            // c:RegisterEffect(e2)
            card::register_effect(d, c, e2, false);
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            // e3:SetCode(EFFECT_CHANGE_LEVEL)
            effect::set_code(d, e3, EFFECT_CHANGE_LEVEL);
            // e3:SetValue(ec:GetLevel())
            let v = card::get_level(d, ec) as i64;
            effect::set_value(d, e3, v);
            // e3:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
            effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
            // c:RegisterEffect(e3)
            card::register_effect(d, c, e3, false);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
