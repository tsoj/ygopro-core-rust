//! Dark Valkyria
use super::prelude::*;

pub const ID: u32 = 83269557;
pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_SPELL as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_SPELL,LOCATION_MZONE,Gemini.EffectStatusCondition)
    card::enable_counter_permit(d, c, COUNTER_SPELL as u16, Some(LOCATION_MZONE), Some(gemini::effect_status_condition()));
    // This card is treated as a Normal Monster while face-up on the field or in the GY. While this card is a Normal Monster on the field, you can Normal Summon it to have it become an Effect Monster with these effects
    // Gemini.AddProcedure(c)
    gemini::add_procedure(d, c);
    // Once, while this card is face-up on the field: You can place 1 Spell Counter on it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_NO_TURN_RESET)
    effect::set_property(d, e1, EFFECT_FLAG_NO_TURN_RESET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(Gemini.EffectStatusCondition)
    effect::set_condition(d, e1, Some(gemini::effect_status_condition()));
    // e1:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) ... end)
    effect::set_target(d, e1, Some(cost_target(e1_target)));
    // e1:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e1, Some(operation(e1_operation)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // This card gains 300 ATK for each Spell Counter on it
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(Gemini.EffectStatusCondition)
    effect::set_condition(d, e2, Some(gemini::effect_status_condition()));
    // e2:SetValue(function(e,c) return c:GetCounter(COUNTER_SPELL)*300 end)
    effect::set_value_fn(d, e2, effect_card_value(e2_value));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // You can remove 1 Spell Counter from this card, then target 1 monster on the field; destroy that target
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(Gemini.EffectStatusCondition)
    effect::set_condition(d, e3, Some(gemini::effect_status_condition()));
    // e3:SetCost(Cost.RemoveCounterFromSelf(COUNTER_SPELL,1))
    effect::set_cost(d, e3, Some(cost::remove_counter_from_self(COUNTER_SPELL as u16, 1)));
    // e3:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk,chkc) ... end)
    effect::set_target(d, e3, Some(cost_target(e3_target)));
    // e3:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e3, Some(operation(e3_operation)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk)` of e1
fn e1_target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanAddCounter(COUNTER_SPELL,1) end
    if a.chk == Some(0) {
        return card::is_can_add_counter(d, c, COUNTER_SPELL as u16, 1, false, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,c,1,tp,COUNTER_SPELL)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, Some(CardSet::from_card(c)), 1, a.tp, COUNTER_SPELL as i32);
    true
}
/// `function(e,tp,eg,ep,ev,re,r,rp)` of e1
fn e1_operation(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // c:AddCounter(COUNTER_SPELL,1)
        card::add_counter(d, c, COUNTER_SPELL as u16, 1, false);
    }
}
/// `function(e,c) return c:GetCounter(COUNTER_SPELL)*300 end` of e2
fn e2_value(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    card::get_counter(d, c, COUNTER_SPELL as u16) as i64 * 300
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk,chkc)` of e3
fn e3_target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, a.tp, 0);
    true
}
/// `function(e,tp,eg,ep,ev,re,r,rp)` of e3
fn e3_operation(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc:IsRelateToEffect(e) and tc:IsMonster() then
    if card::is_relate_to_effect(d, tc.nil(), a.e) && card::is_monster(d, tc.nil()) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc.nil(), REASON_EFFECT, None, None);
    }
}
