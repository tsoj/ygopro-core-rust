//! Eldlich the Mad Golden Lord
use super::prelude::*;

pub const ID: u32 = 74889525;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_GOLDEN_LORD],
    listed_series: &[SET_ELDLICH],
    material_setcode: &[SET_ELDLICH],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_ELDLICH),s.matfilter)
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_ELDLICH as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| matfilter(d, c, fc, st, tp))),
        ],
    );
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(CARD_GOLDEN_LORD)
    effect::set_value(d, e1, CARD_GOLDEN_LORD as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e4, CATEGORY_CONTROL);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetCost(s.ctcost)
    effect::set_cost(d, e4, Some(cost_target(ctcost)));
    // e4:SetTarget(s.cttg)
    effect::set_target(d, e4, Some(cost_target(cttg)));
    // e4:SetOperation(s.ctop)
    effect::set_operation(d, e4, Some(operation(ctop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.matfilter(c,fc,sumtype,tp)
fn matfilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsRace(RACE_ZOMBIE,fc,sumtype,tp) and c:IsLevelAbove(5)
    card::is_race_ctx(d, c, RACE_ZOMBIE, Some(fc), sumtype, Some(tp)) && card::is_level_above(d, c, 5)
}
// function s.filter(c,e)
fn filter(d: &mut Duel, c: CardId, e: Option<EffectId>) -> bool {
    // return c:IsFaceup() and c:IsAbleToChangeControler() and (not e or c:IsCanBeEffectTarget(e))
    card::is_faceup(d, c) && card::is_able_to_change_controler(d, c) && (e.is_none() || card::is_can_be_effect_target(d, c, e))
}
// function s.cfilter(c,ft,tp)  (the Lua `ft` argument is overwritten; the release check passes the group and tp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_CONTROL)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_CONTROL), None).0;
    // if c:IsControler(tp) and c:GetSequence()<5 then ft=ft+1 end
    if card::is_controler(d, c, tp) && card::get_sequence(d, c) < 5 {
        ft += 1;
    }
    // return c:IsRace(RACE_ZOMBIE) and ft>0 and Duel.IsExistingTarget(s.filter,tp,0,LOCATION_MZONE,1,c)
    card::is_race(d, c, RACE_ZOMBIE)
        && ft > 0
        && duel::is_existing_target(d, Some(&|d, x| filter(d, x, None)), tp, 0, LOCATION_MZONE, 1, c.into())
}
// function s.ctcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn ctcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local dg=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_MZONE,nil,e)
    let _dg = duel::get_matching_group(d, Some(&|d, c| filter(d, c, Some(e))), tp, 0, LOCATION_MZONE, NO_EX);
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,nil,nil,dg,tp) end
    if a.chk == Some(0) {
        return aux::check_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, tp)), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,nil,nil,dg,tp)
    let g = aux::select_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, tp)), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
// function s.cttg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && filter(d, chkc, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, None)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, None)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, 0, 0);
    true
}
// function s.ctop(e,tp,eg,ep,ev,re,r,rp)
fn ctop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() and Duel.GetControl(tc,tp) then
    if let Some(tc) = tc {
        // (GetControl returns a number, which is always true in Lua, even 0)
        if card::is_relate_to_effect(d, tc, a.e)
            && card::is_faceup(d, tc)
            && { duel::get_control(d, tc, tp, 0, 0, None, None); true }
        {
            // local c=e:GetHandler()
            let c = effect::get_handler(d, a.e).nil();
            // local reset=RESET_EVENT|(RESETS_STANDARD_PHASE_END&~RESET_TURN_SET)
            let reset = RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TURN_SET);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(3206)
            effect::set_description(d, e1, 3206);
            // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CANNOT_ATTACK)
            effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
            // e1:SetReset(reset)
            effect::set_reset(d, e1, reset, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetDescription(3302)
            effect::set_description(d, e2, 3302);
            // e2:SetCode(EFFECT_CANNOT_TRIGGER)
            effect::set_code(d, e2, EFFECT_CANNOT_TRIGGER);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
}
