//! Samurai Destroyer
use super::prelude::*;

pub const ID: u32 = 40509732;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // actlimit
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetCondition(s.con)
    effect::set_condition(d, e1, Some(condition(con)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // disable
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.con)
    effect::set_condition(d, e2, Some(condition(con)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e3, EVENT_PRE_DAMAGE_CALCULATE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e4, EFFECT_DISABLE);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e4, 0, LOCATION_MZONE);
    // e4:SetTarget(s.distg)
    effect::set_target(d, e4, Some(effect_card_fn(distg)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // special summon
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e5, aux::stringid(ID, 0));
    // e5:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e5, CATEGORY_SPECIAL_SUMMON);
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e5:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e5, EVENT_LEAVE_FIELD);
    // e5:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e5, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e5:SetCondition(s.spcon)
    effect::set_condition(d, e5, Some(condition(spcon)));
    // e5:SetTarget(s.sptg)
    effect::set_target(d, e5, Some(cost_target(sptg)));
    // e5:SetOperation(s.spop)
    effect::set_operation(d, e5, Some(operation(spop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.con(e)`
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return (Duel.GetAttacker()==c and c:GetBattleTarget()) or Duel.GetAttackTarget()==c
    (duel::get_attacker(d) == Some(c) && card::get_battle_target(d, c).is_some()) || duel::get_attack_target(d) == Some(c)
}
/// `s.distg(e,c)`
fn distg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c==e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, e).nil();
    card::get_battle_target(d, h) == Some(c)
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetAttackTarget()
    let mut tc = duel::get_attack_target(d);
    // if not tc then return end
    let Some(t) = tc else {
        return;
    };
    // if tc:IsControler(tp) then tc=Duel.GetAttacker() end
    if card::is_controler(d, t, a.tp) {
        tc = duel::get_attacker(d);
    }
    let tc = tc.nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e1, EFFECT_DISABLE);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_DISABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 1);
    // tc:RegisterEffect(e2)
    card::register_effect(d, tc, e2, false);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return rp~=tp and c:IsReason(REASON_EFFECT) and c:GetPreviousControler()==tp and c:IsPreviousPosition(POS_FACEUP)
    a.rp != a.tp
        && card::is_reason(d, c, REASON_EFFECT)
        && card::get_previous_controler(d, c) == a.tp
        && card::is_previous_position(d, c, POS_FACEUP as u32)
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_MACHINE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_MACHINE) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.filter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && filter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
