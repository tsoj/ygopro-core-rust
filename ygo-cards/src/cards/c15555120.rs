//! Hi-Speed Re-Level
use super::prelude::*;

pub const ID: u32 = 15555120;

// s.listed_series={SET_SPEEDROID}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPEEDROID], ..ScriptInfo::DEFAULT };

/// `s.check` (a script variable, per duel)
const CHECK: u64 = 0;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // local lv=c:GetLevel()
    let lv = card::get_level(d, c);
    // return lv>0 and c:IsSetCard(SET_SPEEDROID) and c:IsMonster() and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    //   and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,c,lv)
    lv > 0
        && card::is_set_card(d, c, &[SET_SPEEDROID as u16])
        && card::is_monster(d, c)
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, false)
        && duel::is_existing_target(d, Some(&|d, x| filter(d, x, lv as i64)), tp, LOCATION_MZONE, 0, 1, c.into())
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, _a: &EffArgs) -> bool {
    // s.check=true
    set_s_var(d, ID, CHECK, Value::Bool(true));
    // if chk==0 then return true end
    true
}
/// `s.filter(c,lv)`
fn filter(d: &mut Duel, c: CardId, lv: i64) -> bool {
    // local clv=c:GetLevel()
    let clv = card::get_level(d, c) as i64;
    // return c:IsFaceup() and clv>0 and clv~=lv and c:IsType(TYPE_SYNCHRO)
    card::is_faceup(d, c) && clv > 0 && clv != lv && card::is_type(d, c, TYPE_SYNCHRO)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then
    if let Some(chkc) = a.chkc() {
        // local lv=e:GetLabel()
        let lv = effect::get_label(d, e);
        // return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.filter(chkc,lv)
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && filter(d, chkc, lv);
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
        return duel::is_existing_matching_card(d, Some(&|d, x| cfilter(d, x, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| cfilter(d, x, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_COST, None, None);
    // local lv=g:GetFirst():GetLevel()
    let first = group::get_first(d, &g).nil();
    let lv = card::get_level(d, first) as i64;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil,lv)
    let _ = duel::select_target(d, tp, Some(&|d, x| filter(d, x, lv)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // e:SetLabel(lv)
    effect::set_label(d, e, &[lv]);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local lv=e:GetLabel()
    let lv = effect::get_label(d, e);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:GetLevel()~=lv then
    let Some(tc) = tc else { return };
    if !(card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc)) {
        return;
    }
    if card::get_level(d, tc) as i64 == lv {
        return;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CHANGE_LEVEL)
    effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
    // e1:SetValue(lv)
    effect::set_value(d, e1, lv);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // local e2=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e2 = effect::create_effect(d, h);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetValue(lv*500)
    effect::set_value(d, e2, lv * 500);
    // e2:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
    // tc:RegisterEffect(e2)
    card::register_effect(d, tc, e2, false);
}
