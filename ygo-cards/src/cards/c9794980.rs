//! Knightmare Incarnation Idlee
use super::prelude::*;

pub const ID: u32 = 9794980;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // to grave
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.gycon)
    effect::set_condition(d, e2, Some(condition(gycon)));
    // e2:SetTarget(s.gytg)
    effect::set_target(d, e2, Some(cost_target(gytg)));
    // e2:SetOperation(s.gyop)
    effect::set_operation(d, e2, Some(operation(gyop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // indes
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetTarget(aux.TargetBoolFunction(Card.IsLevel,9))
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_level(d, c, &[9]))));
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(Card.IsType,tp,LOCATION_MZONE,LOCATION_MZONE,nil,TYPE_LINK)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_type(d, c, TYPE_LINK)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // return g:GetSum(Card.GetLink)>=8
    group::get_sum(d, &g, &|d, c| card::get_link(d, c) as i64) >= 8
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false)
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
/// `s.gycon(e,tp,eg,ep,ev,re,r,rp)`
fn gycon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct1=Duel.GetMatchingGroupCount(Card.IsType,tp,LOCATION_MZONE,0,nil,TYPE_LINK)
    let ct1 = duel::get_matching_group_count(d, Some(&|d, c| card::is_type(d, c, TYPE_LINK)), a.tp, LOCATION_MZONE, 0, NO_EX);
    // local ct2=Duel.GetMatchingGroupCount(Card.IsType,tp,0,LOCATION_MZONE,nil,TYPE_LINK)
    let ct2 = duel::get_matching_group_count(d, Some(&|d, c| card::is_type(d, c, TYPE_LINK)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // return ct1<ct2
    ct1 < ct2
}
/// `s.gytg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn gytg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(Card.IsType,tp,LOCATION_MZONE,LOCATION_MZONE,nil,TYPE_LINK)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_type(d, c, TYPE_LINK)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g), n, 0, 0);
    true
}
/// `s.gyop(e,tp,eg,ep,ev,re,r,rp)`
fn gyop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsType,tp,LOCATION_MZONE,LOCATION_MZONE,nil,TYPE_LINK)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_type(d, c, TYPE_LINK)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g>0 then Duel.SendtoGrave(g,REASON_EFFECT) end
    if !g.is_empty() {
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
