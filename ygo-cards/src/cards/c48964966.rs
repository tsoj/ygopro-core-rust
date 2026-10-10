//! Athena
use super::prelude::*;

pub const ID: u32 = 48964966;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(s.condition2)
    effect::set_condition(d, e2, Some(condition(condition2)));
    // e2:SetTarget(s.target2)
    effect::set_target(d, e2, Some(cost_target(target2)));
    // e2:SetOperation(s.operation2)
    effect::set_operation(d, e2, Some(operation(operation2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetCondition(s.condition2)
    effect::set_condition(d, e3, Some(condition(condition2)));
    // e3:SetTarget(s.target2)
    effect::set_target(d, e3, Some(cost_target(target2)));
    // e3:SetOperation(s.operation2)
    effect::set_operation(d, e3, Some(operation(operation2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e4, CATEGORY_DAMAGE);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_FLIP_SUMMON_SUCCESS);
    // e4:SetCondition(s.condition2)
    effect::set_condition(d, e4, Some(condition(condition2)));
    // e4:SetTarget(s.target2)
    effect::set_target(d, e4, Some(cost_target(target2)));
    // e4:SetOperation(s.operation2)
    effect::set_operation(d, e4, Some(operation(operation2)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.filter1(c,ft)`
fn filter1(d: &mut Duel, c: CardId, ft: i32) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_FAIRY) and c:GetCode()~=id and c:IsAbleToGraveAsCost()
    //     and (ft>0 or c:GetSequence()<5)
    card::is_faceup(d, c)
        && card::is_race(d, c, RACE_FAIRY as u64)
        && !card::get_code(d, c).contains(&ID)
        && card::is_able_to_grave_as_cost(d, c)
        && (ft > 0 || card::get_sequence(d, c) < 5)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return ft>-1 and Duel.IsExistingMatchingCard(s.filter1,tp,LOCATION_MZONE,0,1,nil,ft) end
    if a.chk == Some(0) {
        return ft > -1 && duel::is_existing_matching_card(d, Some(&|d, c| filter1(d, c, ft)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter1,tp,LOCATION_MZONE,0,1,1,nil,ft)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| filter1(d, c, ft)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.filter2(c,e,sp)`
fn filter2(d: &mut Duel, c: CardId, e: EffectId, sp: u8) -> bool {
    // return c:IsRace(RACE_FAIRY) and c:GetCode()~=id and c:IsCanBeSpecialSummoned(e,0,sp,false,false)
    card::is_race(d, c, RACE_FAIRY as u64)
        && !card::get_code(d, c).contains(&ID)
        && card::is_can_be_special_summoned(d, c, e, 0, sp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter2(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && filter2(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter2,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter2(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.filter2,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter2(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
/// `s.condition2(e,tp,eg,ep,ev,re,r,rp)`
fn condition2(d: &mut Duel, a: &EffArgs) -> bool {
    // return not eg:IsContains(e:GetHandler()) and eg:FilterCount(Card.IsRace,nil,RACE_FAIRY)>0
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    !group::is_contains(&eg, h) && group::filter_count(d, &eg, &|d, c| card::is_race(d, c, RACE_FAIRY as u64), NO_EX) > 0
}
/// `s.target2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(600)
    duel::set_target_param(d, 600);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,600)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 600);
    true
}
/// `s.operation2(e,tp,eg,ep,ev,re,r,rp)`
fn operation2(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let dmg = info[1].int();
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, p, dmg, REASON_EFFECT, false, None);
}
