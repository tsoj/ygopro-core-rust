//! Virtual World Beast - Jiujiu
use super::prelude::*;

pub const ID: u32 = 35252119;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Cannot be destroyed
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.indcon)
    effect::set_condition(d, e1, Some(condition(indcon)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Send to grave
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e3, CATEGORY_TOGRAVE);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCost(s.gycost)
    effect::set_cost(d, e3, Some(cost_target(gycost)));
    // e3:SetTarget(s.gytg)
    effect::set_target(d, e3, Some(cost_target(gytg)));
    // e3:SetOperation(s.gyop)
    effect::set_operation(d, e3, Some(operation(gyop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.indfilter1(c,tp)
fn indfilter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsMonster() and Duel.IsExistingMatchingCard(s.indfilter2,tp,LOCATION_GRAVE,0,1,c,c)
    card::is_monster(d, c) && duel::is_existing_matching_card(d, Some(&|d, x| indfilter2(d, x, c)), tp, LOCATION_GRAVE, 0, 1, c.into())
}
// function s.indfilter2(c,tc)
fn indfilter2(d: &mut Duel, c: CardId, tc: CardId) -> bool {
    // return c:IsMonster() and c:GetOriginalAttribute()==tc:GetOriginalAttribute()
    //     and c:GetOriginalRace()==tc:GetOriginalRace() and not c:IsCode(tc:GetCode())
    if !card::is_monster(d, c) {
        return false;
    }
    if card::get_original_attribute(d, c) != card::get_original_attribute(d, tc) {
        return false;
    }
    if card::get_original_race(d, c) != card::get_original_race(d, tc) {
        return false;
    }
    let codes = card::get_code(d, tc);
    !card::is_code(d, c, &codes)
}
// function s.indcon(e)
fn indcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.indfilter1,e:GetHandlerPlayer(),LOCATION_GRAVE,0,1,nil,e:GetHandlerPlayer())
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&|d, c| indfilter1(d, c, p)), p, LOCATION_GRAVE, 0, 1, NO_EX)
}
// function s.gycfilter(c,tp)
fn gycfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_monster(d, c) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
// function s.gycheck(sg,e,tp)
fn gycheck(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetOriginalAttribute)==1 and sg:GetClassCount(Card.GetOriginalRace)==1
    //     and sg:GetClassCount(Card.GetCode)==#sg
    //     and Duel.IsExistingTarget(Card.IsAbleToGrave,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,sg)
    let res = group::get_class_count(d, sg, &|d, c| card::get_original_attribute(d, c) as i64) == 1
        && group::get_class_count(d, sg, &|d, c| card::get_original_race(d, c) as i64) == 1
        && group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64) == sg.len()
        && duel::is_existing_target(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, sg.into());
    (res, false)
}
fn gycost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.gycfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| gycfilter(d, c)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    let rs: aux::Rescon = &gycheck;
    // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,2,2,s.gycheck,0) end
    if a.chk == Some(0) {
        return matches!(aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(rs), 0, tp, None, None, None, false), Err(true));
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.gycheck,1,tp,HINTMSG_REMOVE)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(rs), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, false)
        .unwrap_or_default();
    // Duel.Remove(sg,POS_FACEUP,REASON_COST)
    duel::remove(d, &sg, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
fn gytg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsAbleToGrave() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_able_to_grave(d, chkc, None);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToGrave,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, g, 1, 0, 0);
    true
}
fn gyop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SendtoGrave(tc,REASON_EFFECT)
            duel::send_to_grave(d, tc, REASON_EFFECT, None, None);
        }
    }
}
