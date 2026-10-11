//! Crusader of Endymion
use super::prelude::*;

pub const ID: u32 = 73853830;
pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_SPELL as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Gemini.AddProcedure(c)
    gemini::add_procedure(d, c);
    // Place 1 Spell Counter
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(Gemini.EffectStatusCondition)
    effect::set_condition(d, e1, Some(gemini::effect_status_condition()));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsCanAddCounter(COUNTER_SPELL,1) end
    if let Some(chkc) = a.chkc() {
        return card::is_can_add_counter(d, chkc, COUNTER_SPELL as u16, 1, false, 0);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsCanAddCounter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil,COUNTER_SPELL,1) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_can_add_counter(d, c, COUNTER_SPELL as u16, 1, false, 0)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
    // local g=Duel.SelectTarget(tp,Card.IsCanAddCounter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil,COUNTER_SPELL,1)
    let _g = duel::select_target(d, a.tp, Some(&|d, c| card::is_can_add_counter(d, c, COUNTER_SPELL as u16, 1, false, 0)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and tc:IsCanAddCounter(COUNTER_SPELL,1) and tc:AddCounter(COUNTER_SPELL,1) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_can_add_counter(d, tc, COUNTER_SPELL as u16, 1, false, 0) && card::add_counter(d, tc, COUNTER_SPELL as u16, 1, false) {
            // local c=e:GetHandler()
            let c = effect::get_handler(d, a.e).nil();
            // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
            if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
                return;
            }
            // Gain 600 ATK
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(600)
            effect::set_value(d, e1, 600);
            // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
    }
}
