//! Mayakashi Winter
use super::prelude::*;

pub const ID: u32 = 51225407;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAYAKASHI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // reduce atk/def
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // draw
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e4, CATEGORY_DRAW);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetCost(s.drcost)
    effect::set_cost(d, e4, Some(cost_target(drcost)));
    // e4:SetTarget(s.drtg)
    effect::set_target(d, e4, Some(cost_target(drtg)));
    // e4:SetOperation(s.drop)
    effect::set_operation(d, e4, Some(operation(drop_op)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // spsummon
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e5, CATEGORY_SPECIAL_SUMMON);
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e5, EFFECT_FLAG_CARD_TARGET, 0);
    // e5:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e5, LOCATION_GRAVE);
    // e5:SetCountLimit(1,id)
    effect::set_count_limit(d, e5, 1, ID, 0, 0);
    // e5:SetCost(s.spcost)
    effect::set_cost(d, e5, Some(cost_target(spcost)));
    // e5:SetTarget(s.sptg)
    effect::set_target(d, e5, Some(cost_target(sptg)));
    // e5:SetOperation(s.spop)
    effect::set_operation(d, e5, Some(operation(spop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_MAYAKASHI)
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_MAYAKASHI as u16])
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    // local g=Duel.GetMatchingGroup(s.atkfilter,e:GetHandlerPlayer(),LOCATION_GRAVE,0,nil)
    let p = effect::get_handler_player(d, e);
    let g = duel::get_matching_group(d, Some(&atkfilter), p, LOCATION_GRAVE, 0, NO_EX);
    // return g:GetClassCount(Card.GetCode)*-100
    group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) as i64 * -100
}
/// `s.drcfilter(c)`
fn drcfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_MAYAKASHI) and c:IsAbleToGraveAsCost()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MAYAKASHI as u16]) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.drcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToGraveAsCost()
    //   and Duel.IsExistingMatchingCard(s.drcfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, c) && duel::is_existing_matching_card(d, Some(&drcfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.drcfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let mut g = duel::select_matching_card(d, a.tp, Some(&drcfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
/// `s.cfilter(c,e,tp)`
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_ZOMBIE) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    //   and (Duel.GetLocationCount(tp,LOCATION_MZONE)>0 or (c:IsLocation(LOCATION_MZONE) and c:GetSequence()<5))
    //   and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,c,e,tp)
    card::is_race(d, c, RACE_ZOMBIE)
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, false)
        && (duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            || (card::is_location(d, c, LOCATION_MZONE) && card::get_sequence(d, c) < 5))
        && duel::is_existing_target(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_GRAVE, 0, 1, c.into())
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_MAYAKASHI) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_MAYAKASHI as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemoveAsCost() and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None)
            && duel::is_existing_matching_card(d, Some(&|d, x| cfilter(d, x, a.e, a.tp)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil,e,tp)
    let mut g = duel::select_matching_card(d, a.tp, Some(&|d, x| cfilter(d, x, a.e, a.tp)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,1,nil,e,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, CardSet::from_card(tc), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
