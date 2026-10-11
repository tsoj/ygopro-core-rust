//! Crossmind Archfiend
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 52644170;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Psychic Tuner + 1+ non-Tuner monsters
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_PSYCHIC),1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, Some(mat_filter(|d, x, sc, st, tp| card::is_race_ctx(d, x, RACE_PSYCHIC, sc, st, Some(tp)))), 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Special Summon 1 of your banished Level 7 or lower Psychic monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some((TIMING_MAIN_END | TIMINGS_CHECK_MONSTER) as u32));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Gain LP equal to the original ATK of 1 Tributed monster, and if you do, this card gains that much ATK until the end of this turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_RECOVER+CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_RECOVER + CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(TIMING_DAMAGE_STEP,TIMING_DAMAGE_STEP|TIMING_BATTLE_START|TIMING_BATTLE_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, TIMING_DAMAGE_STEP, Some((TIMING_DAMAGE_STEP | TIMING_BATTLE_START | TIMING_BATTLE_END | TIMINGS_CHECK_MONSTER) as u32));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function() return Duel.IsBattlePhase() and aux.StatChangeDamageStepCondition() end)
    effect::set_condition(d, e2, Some(condition(|d, _a| duel::is_battle_phase(d, None) && aux::stat_change_damage_step_condition(d))));
    // e2:SetCost(s.lpatkost)
    effect::set_cost(d, e2, Some(cost_target(lpatkost)));
    // e2:SetTarget(s.lpatktg)
    effect::set_target(d, e2, Some(cost_target(lpatktg)));
    // e2:SetOperation(s.lpatkop)
    effect::set_operation(d, e2, Some(operation(lpatkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}

/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(7) and c:IsRace(RACE_PSYCHIC) and c:IsFaceup() and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_below(d, c, 7)
        && card::is_race(d, c, RACE_PSYCHIC)
        && card::is_faceup(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}

/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_REMOVED) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_REMOVED) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, a.e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_REMOVED,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_REMOVED,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, 0);
    true
}

/// `s.spop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP) end
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}

/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetTextAttack()>0
    card::get_text_attack(d, c) > 0
}

/// `s.lpatkost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn lpatkost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    let tp = a.tp;
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.atkfilter,1,false,nil,c) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&atkfilter), 1, 1, false, None, c.into());
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.atkfilter,1,1,false,nil,c)
    let g = duel::select_release_group_cost(d, tp, Some(&atkfilter), 1, 1, false, None, c.into());
    // Duel.Release(g,REASON_COST)
    duel::release(d, g.clone(), REASON_COST, None);
    // e:SetLabel(g:GetFirst():GetTextAttack())
    let first = group::get_first(d, &g).nil();
    let atk = card::get_text_attack(d, first);
    effect::set_label(d, a.e, &[atk as i64]);
    true
}

/// `s.lpatktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn lpatktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,1,tp,e:GetLabel())
    let lp = effect::get_label(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 1, a.tp, lp as i32);
    true
}

/// `s.lpatkop(e,tp,eg,ep,ev,re,r,rp)`
fn lpatkop(d: &mut Duel, a: &EffArgs) {
    // local lpgain=e:GetLabel()
    let lpgain = effect::get_label(d, a.e);
    // local atk=Duel.Recover(tp,lpgain,REASON_EFFECT)
    let atk = duel::recover(d, a.tp, lpgain, REASON_EFFECT, false, None);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if atk>0 and c:IsFaceup() and c:IsRelateToEffect(e) then
    if atk > 0 && card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(atk)
        effect::set_value(d, e1, atk as i64);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
