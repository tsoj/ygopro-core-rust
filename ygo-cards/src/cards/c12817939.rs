//! Dark Lucius LV6
//! Dark Lucius LV6
use super::prelude::*;

pub const ID: u32 = 12817939;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[58206034, 85313220], lv: Some((6, SET_DARK_LUCIUS)), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // disable
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_BATTLED)
    effect::set_code(d, e1, EVENT_BATTLED);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(operation(disop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e2, Some(cost::self_to_grave()));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.disop(e,tp,eg,ep,ev,re,r,rp)
fn disop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local d=Duel.GetAttackTarget()
    let mut target = duel::get_attack_target(d);
    // if d==c then d=Duel.GetAttacker() end
    if target == Some(c) {
        target = duel::get_attacker(d);
    }
    // if d and d:IsStatus(STATUS_BATTLE_DESTROYED) and d:IsType(TYPE_EFFECT)
    //   and c:GetFlagEffect(85313220)~=0 and not c:IsStatus(STATUS_BATTLE_DESTROYED) then
    if let Some(dc) = target {
        if card::is_status(d, dc, STATUS_BATTLE_DESTROYED)
            && card::is_type(d, dc, TYPE_EFFECT)
            && card::get_flag_effect(d, c, 85313220) != 0
            && !card::is_status(d, c, STATUS_BATTLE_DESTROYED)
        {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD_EXC_GRAVE)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_EXC_GRAVE, 1);
            // d:RegisterEffect(e1)
            card::register_effect(d, dc, e1, false);
            // c:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END|RESET_SELF_TURN,0,2)
            card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END | RESET_SELF_TURN, 0, 2, 0, 0);
        }
    }
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return tp==Duel.GetTurnPlayer() and e:GetHandler():GetFlagEffect(id)~=0
    let c = effect::get_handler(d, a.e).nil();
    a.tp == duel::get_turn_player(d) && card::get_flag_effect(d, c, ID) != 0
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(58206034) and c:IsCanBeSpecialSummoned(e,0,tp,true,true)
    card::is_code(d, c, &[58206034]) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, true, Some(POS_FACEUP as u8), None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if e:GetHandler():GetSequence()<5 then ft=ft+1 end
    let c = effect::get_handler(d, a.e).nil();
    if card::get_sequence(d, c) < 5 {
        ft += 1;
    }
    // if chk==0 then return ft>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return ft > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if tc and Duel.SpecialSummon(tc,0,tp,tp,true,true,POS_FACEUP)>0 then
    if let Some(tc) = group::get_first(d, &sel) {
        if duel::special_summon(d, tc, 0, tp, tp, true, true, POS_FACEUP as u8, None) > 0 {
            // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD&~(RESET_LEAVE|RESET_TEMP_REMOVE),0,0)
            card::register_flag_effect(d, tc, ID, RESET_EVENT | (RESETS_STANDARD & !(RESET_LEAVE | RESET_TEMP_REMOVE)), 0, 0, 0, 0);
            // tc:CompleteProcedure()
            card::complete_procedure(d, tc);
        }
    }
}
