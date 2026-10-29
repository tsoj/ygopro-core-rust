//! Raidraptor - Arsenal Falcon
use super::prelude::*;

pub const ID: u32 = 96157835;

// s.listed_series={SET_RAIDRAPTOR}
// s.listed_names={id}
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_RAIDRAPTOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,nil,7,2)
    xyz::add_procedure(d, c, None, Some(7), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(s.sptg1)
    effect::set_target(d, e1, Some(cost_target(sptg1)));
    // e1:SetOperation(s.spop1)
    effect::set_operation(d, e1, Some(operation(spop1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // extra attack
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_EXTRA_ATTACK)
    effect::set_code(d, e2, EFFECT_EXTRA_ATTACK);
    // e2:SetCondition(s.ctcon)
    effect::set_condition(d, e2, Some(condition(ctcon)));
    // e2:SetValue(s.ctval)
    effect::set_value_fn(d, e2, effect_value(ctval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // special summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCondition(s.spcon2)
    effect::set_condition(d, e3, Some(condition(spcon2)));
    // e3:SetTarget(s.sptg2)
    effect::set_target(d, e3, Some(cost_target(sptg2)));
    // e3:SetOperation(s.spop2)
    effect::set_operation(d, e3, Some(operation(spop2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spfilter1(c,e,tp)`
fn spfilter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_WINGEDBEAST) and c:GetLevel()==4 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_WINGEDBEAST) && card::get_level(d, c) == 4 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg1(e,tp,...,chk)`
fn sptg1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter1,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d: &mut Duel, c: CardId| spfilter1(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.spop1(e,tp,...)`
fn spop1(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter1,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d: &mut Duel, c: CardId| spfilter1(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.ctcon(e)`
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetOverlayGroup():IsExists(Card.IsSetCard,1,nil,SET_RAIDRAPTOR)
    let h = effect::get_handler(d, a.e).nil();
    let og = card::get_overlay_group(d, h);
    group::is_exists(d, &og, &|d: &mut Duel, c: CardId| card::is_set_card(d, c, &[SET_RAIDRAPTOR as u16]), 1, NO_EX)
}
/// `s.ctval(e,c)`
fn ctval(d: &mut Duel, e: EffectId) -> i64 {
    // return e:GetHandler():GetOverlayGroup():FilterCount(Card.IsSetCard,nil,SET_RAIDRAPTOR)-1
    let h = effect::get_handler(d, e).nil();
    let og = card::get_overlay_group(d, h);
    group::filter_count(d, &og, &|d: &mut Duel, c: CardId| card::is_set_card(d, c, &[SET_RAIDRAPTOR as u16]), NO_EX) as i64 - 1
}
/// `s.spcon2(e,tp,...)`
fn spcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:GetOverlayGroup():IsExists(Card.IsSetCard,1,nil,SET_RAIDRAPTOR) and c:IsPreviousLocation(LOCATION_MZONE)
    let og = card::get_overlay_group(d, c);
    group::is_exists(d, &og, &|d: &mut Duel, x: CardId| card::is_set_card(d, x, &[SET_RAIDRAPTOR as u16]), 1, NO_EX)
        && card::is_previous_location(d, c, LOCATION_MZONE)
}
/// `s.spfilter2(c,e,tp)`
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_RAIDRAPTOR) and c:IsType(TYPE_XYZ) and not c:IsCode(id)
    //   and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_RAIDRAPTOR as u16])
        && card::is_type(d, c, TYPE_XYZ)
        && !card::is_code(d, c, &[ID])
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg2(e,tp,...,chk)`
fn sptg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter2,tp,LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d: &mut Duel, c: CardId| spfilter2(d, c, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.spop2(e,tp,...)`
fn spop2(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter2,tp,LOCATION_EXTRA,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d: &mut Duel, c: CardId| spfilter2(d, c, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)>0 then
    if let Some(tc) = tc {
        if duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
            // local c=e:GetHandler()
            let c = effect::get_handler(d, a.e).nil();
            // if c:IsRelateToEffect(e) then
            if card::is_relate_to_effect(d, c, a.e) {
                // Duel.Overlay(tc,c)
                duel::overlay(d, tc, c, false);
            }
        }
    }
}
