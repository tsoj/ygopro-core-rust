//! Split Mirror of the Underworld
use super::prelude::*;

pub const ID: u32 = 42952160;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 monster, OR inflict damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e1:SetCode(EVENT_DAMAGE)
    effect::set_code(d, e1, EVENT_DAMAGE);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.actcon)
    effect::set_condition(d, e1, Some(condition(actcon)));
    // e1:SetTarget(s.acttg)
    effect::set_target(d, e1, Some(cost_target(acttg)));
    // e1:SetOperation(s.actop)
    effect::set_operation(d, e1, Some(operation(actop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.actcon(e,tp,eg,ep,ev,re,r,rp)`
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if ep==1-tp then return false end
    if a.ep == 1 - a.tp {
        return false;
    }
    // if (r&REASON_EFFECT)==REASON_EFFECT then
    if a.r & REASON_EFFECT == REASON_EFFECT {
        // return rp==1-tp
        return a.rp == 1 - a.tp;
    }
    // return Duel.GetAttacker():IsControler(1-tp)
    let attacker = duel::get_attacker(d).nil();
    card::is_controler(d, attacker, 1 - a.tp)
}
/// `s.spfilter(c,e,tp,dam)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, dam: u32) -> bool {
    // return c:IsAttackBelow(dam) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_attack_below(d, c, dam as i32) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.acttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn acttg(d: &mut Duel, a: &EffArgs) -> bool {
    // local is_eff=(r&REASON_EFFECT)==REASON_EFFECT
    let is_eff = a.r & REASON_EFFECT == REASON_EFFECT;
    // if chk==0 then return is_eff or (Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,ev))
    if a.chk == Some(0) {
        return is_eff
            || (duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, a.ev)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX));
    }
    // if is_eff then
    if is_eff {
        // e:SetCategory(CATEGORY_DAMAGE)
        effect::set_category(d, a.e, CATEGORY_DAMAGE);
        // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,ev*2)
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, (a.ev * 2) as i32);
    } else {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    }
    true
}
/// `s.actop(e,tp,eg,ep,ev,re,r,rp)`
fn actop(d: &mut Duel, a: &EffArgs) {
    // if (r&REASON_EFFECT)==REASON_EFFECT then
    if a.r & REASON_EFFECT == REASON_EFFECT {
        // Duel.Damage(1-tp,ev*2,REASON_EFFECT)
        duel::damage(d, 1 - a.tp, (a.ev * 2) as i64, REASON_EFFECT, false, None);
        // return
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,ev)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp, a.ev)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)>0 then
    if !g.is_empty() && duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        // End the Battle Phase after the Damage Step
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_DAMAGE_STEP_END)
        effect::set_code(d, e1, EVENT_DAMAGE_STEP_END);
        // e1:SetOperation(function() Duel.SkipPhase(Duel.GetTurnPlayer(),PHASE_BATTLE,RESET_PHASE|PHASE_BATTLE_STEP,1) end)
        effect::set_operation(d, e1, Some(operation(skip_battle)));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
/// `function() Duel.SkipPhase(Duel.GetTurnPlayer(),PHASE_BATTLE,RESET_PHASE|PHASE_BATTLE_STEP,1) end`
fn skip_battle(d: &mut Duel, _a: &EffArgs) {
    // Duel.SkipPhase(Duel.GetTurnPlayer(),PHASE_BATTLE,RESET_PHASE|PHASE_BATTLE_STEP,1)
    let tp = duel::get_turn_player(d);
    duel::skip_phase(d, tp, PHASE_BATTLE, RESET_PHASE | PHASE_BATTLE_STEP, 1, 0);
}
