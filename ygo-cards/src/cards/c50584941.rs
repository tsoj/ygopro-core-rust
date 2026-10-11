//! Red Supremacy
use super::prelude::*;

pub const ID: u32 = 50584941;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RED_DRAGON_ARCHFIEND], ..ScriptInfo::DEFAULT };

// s_var key of `s.check` (the Lua script table field)
const CHECK: u64 = 0;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.cfilter(c,tp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // local code=c:GetOriginalCode()
    let code = card::get_original_code(d, c);
    // return c:IsSetCard(SET_RED_DRAGON_ARCHFIEND) and c:IsType(TYPE_SYNCHRO) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    //   and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,c,code)
    card::is_set_card(d, c, &[SET_RED_DRAGON_ARCHFIEND as u16])
        && card::is_type(d, c, TYPE_SYNCHRO)
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, false)
        && duel::is_existing_target(d, Some(&|d, x| filter(d, x, code)), tp, LOCATION_MZONE, 0, 1, c.into())
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, _a: &EffArgs) -> bool {
    // s.check=true
    set_s_var(d, ID, CHECK, Value::Bool(true));
    // return true
    true
}
// function s.filter(c,code)
fn filter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_RED_DRAGON_ARCHFIEND) and c:IsType(TYPE_SYNCHRO)
    //   and (not c:IsCode(code) or not c:IsOriginalCode(code))
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_RED_DRAGON_ARCHFIEND as u16])
        && card::is_type(d, c, TYPE_SYNCHRO)
        && (!card::is_code(d, c, &[code]) || !card::is_original_code(d, c, &[code]))
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc,e:GetLabel()) end
    if let Some(chkc) = a.chkc() {
        let label = effect::get_label(d, a.e) as u32;
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc, label);
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // if not s.check then return false end
        if !s_var(d, ID, CHECK).truthy() {
            return false;
        }
        // s.check=false
        set_s_var(d, ID, CHECK, Value::Bool(false));
        // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil,tp)
        return duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| cfilter(d, c, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_COST, None, None);
    // local code=g:GetFirst():GetOriginalCode()
    let first = group::get_first(d, &g).nil();
    let code = card::get_original_code(d, first);
    // e:SetLabel(code)
    effect::set_label(d, a.e, &[code as i64]);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil,code)
    duel::select_target(d, tp, Some(&|d, c| filter(d, c, code)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local code=e:GetLabel()
    let code = effect::get_label(d, a.e) as u32;
    // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CHANGE_CODE)
            effect::set_code(d, e1, EFFECT_CHANGE_CODE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetValue(code)
            effect::set_value(d, e1, code as i64);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // tc:ReplaceEffect(code,RESET_EVENT|RESETS_STANDARD)
            card::replace_effect(d, tc, code, RESET_EVENT | RESETS_STANDARD, 0);
        }
    }
}
