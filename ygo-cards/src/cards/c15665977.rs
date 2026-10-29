//! Kewl Tune RS
//! キラーチューン・レッドシール
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 15665977;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[89392810], material: &[89392810], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: "Kewl Tune Reco" + 1+ Tuners
    // Synchro.AddProcedure(c,aux.FALSE,1,1,s.tunerfilter,1,99,aux.FilterSummonCode(89392810))
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|_d, _c, _sc, _st, _tp| false)),
        1,
        1,
        Some(mat_filter(tunerfilter)),
        1,
        99,
        Some(aux::filter_summon_code(&[89392810])),
        None,
        None,
        None,
        None,
    );
    // Gains 300 ATK for each Tuner in the GYs
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(function(e,c) return 300*Duel.GetMatchingGroupCount(Card.IsType,0,LOCATION_GRAVE,LOCATION_GRAVE,nil,TYPE_TUNER) end)
    effect::set_value_fn(
        d,
        e1,
        effect_value(|d, _e| {
            300 * duel::get_matching_group_count(d, Some(&|d, x| card::is_type(d, x, TYPE_TUNER)), 0, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX) as i64
        }),
    );
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // The Levels of monsters your opponent controls with 1700 or less original ATK are increased by 1
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_LEVEL)
    effect::set_code(d, e2, EFFECT_UPDATE_LEVEL);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    // e2:SetTarget(function(e,c) return c:GetBaseAttack()<=1700 end)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::get_base_attack(d, c) <= 1700)));
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Negate the effects of 1 face-up card your opponent controls until the end of this turn
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e3, CATEGORY_DISABLE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e3, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e3:SetCost(s.discost)
    effect::set_cost(d, e3, Some(cost_target(discost)));
    // e3:SetTarget(s.distg)
    effect::set_target(d, e3, Some(cost_target(distg)));
    // e3:SetOperation(s.disop)
    effect::set_operation(d, e3, Some(operation(disop)));
    // e3:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e3, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Multiple tuners
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e4, EFFECT_MATERIAL_CHECK);
    // e4:SetValue(s.valcheck)
    effect::set_value_fn(d, e4, effect_card_value(valcheck));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.tunerfilter(c,scard,sumtype,tp)`
fn tunerfilter(d: &mut Duel, c: CardId, scard: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    // return c:IsType(TYPE_TUNER,scard,sumtype,tp) or c:IsHasEffect(EFFECT_CAN_BE_TUNER)
    card::is_type_ctx(d, c, TYPE_TUNER, scard, sumtype, Some(tp)) || !card::is_has_effect(d, c, EFFECT_CAN_BE_TUNER, None).is_empty()
}
/// `s.costfilter(c)`
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_TUNER) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_type(d, c, TYPE_TUNER) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.discost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&costfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) and chkc:IsNegatable() end
    // (ygoai override lua_fixes/c15665977.lua: the original indexed the nil global c here, a Lua
    // error in the core; the fix checks chkc)
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - tp) && card::is_negatable(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsNegatable,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| card::is_negatable(d, x)), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsNegatable,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, x| card::is_negatable(d, x)), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, tp, 0);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // Negate its effects until the end of this turn
        // tc:NegateEffects(e:GetHandler(),RESET_PHASE|PHASE_END,true)
        let h = effect::get_handler(d, a.e).nil();
        card::negate_effects(d, tc, h, Some(RESET_PHASE | PHASE_END), true, None);
    }
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // if g:IsExists(function(c) return c:IsType(TYPE_TUNER) or c:IsHasEffect(EFFECT_CAN_BE_TUNER) end,2,nil) then
    if group::is_exists(d, &g, &|d, x| card::is_type(d, x, TYPE_TUNER) || !card::is_has_effect(d, x, EFFECT_CAN_BE_TUNER, None).is_empty(), 2, NO_EX) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
        // e1:SetCode(EFFECT_MULTIPLE_TUNERS)
        effect::set_code(d, e1, EFFECT_MULTIPLE_TUNERS);
        // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD)|RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD) | RESET_PHASE | PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
    // the Lua function returns nothing: the core reads nil as 0
    0
}
