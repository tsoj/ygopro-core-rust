//! Apoqliphort Skybase
//! アポクリフォート・カーネル
use super::prelude::*;

pub const ID: u32 = 40061558;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_QLI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // tribute limit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_TRIBUTE_LIMIT)
    effect::set_code(d, e2, EFFECT_TRIBUTE_LIMIT);
    // e2:SetValue(s.tlimit)
    effect::set_value_fn(d, e2, effect_card_fn(tlimit));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // summon with 3 tribute
    // local e3=aux.AddNormalSummonProcedure(c,true,false,3,3,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0))
    let _e3 = aux::add_normal_summon_procedure(d, c, true, false, Some(3), Some(3), Some(SUMMON_TYPE_TRIBUTE), Some(aux::stringid(ID, 0)), None, None);
    // local e4=aux.AddNormalSetProcedure(c,true,false,3,3,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0))
    let _e4 = aux::add_normal_set_procedure(d, c, true, false, Some(3), Some(3), Some(SUMMON_TYPE_TRIBUTE), Some(aux::stringid(ID, 0)), None, None);
    // immune
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e5, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e5, EFFECT_IMMUNE_EFFECT);
    // e5:SetCondition(s.immcon)
    effect::set_condition(d, e5, Some(condition(immcon)));
    // e5:SetValue(s.efilter)
    effect::set_value_fn(d, e5, lua_fn(efilter));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // control
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e6, CATEGORY_CONTROL);
    // e6:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e6, EFFECT_TYPE_IGNITION);
    // e6:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e6, EFFECT_FLAG_CARD_TARGET, 0);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetCountLimit(1)
    effect::set_count_limit(d, e6, 1, 0, 0, 0);
    // e6:SetTarget(s.cttg)
    effect::set_target(d, e6, Some(cost_target(cttg)));
    // e6:SetOperation(s.ctop)
    effect::set_operation(d, e6, Some(operation(ctop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
// function s.tlimit(e,c)
fn tlimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsSetCard(SET_QLI)
    !card::is_set_card(d, c, &[SET_QLI as u16])
}
// function s.immcon(e)
fn immcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsNormalSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_normal_summoned(d, h)
}
// function s.efilter(e,te)
fn efilter(d: &mut Duel, a: Args) -> bool {
    let (e, te) = (a.e(0), a.e(1));
    // if te:IsSpellTrapEffect() then return true
    if effect::is_spell_trap_effect(d, te) {
        return true;
    }
    // else return aux.qlifilter(e,te) end
    aux::qlifilter().call(d, &[Value::Effect(e), Value::Effect(te)]).truthy()
}
// function s.cttg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsControlerCanBeChanged() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && card::is_controler_can_be_changed(d, chkc, false, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsControlerCanBeChanged,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_controler_can_be_changed(d, c, false, None)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectTarget(tp,Card.IsControlerCanBeChanged,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_controler_can_be_changed(d, c, false, None)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, 0, 0);
    true
}
// function s.ctop(e,tp,eg,ep,ev,re,r,rp)
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.GetControl(tc,tp,PHASE_END,1)
            duel::get_control(d, tc, a.tp, PHASE_END as u16, 1, None, None);
        }
    }
}
