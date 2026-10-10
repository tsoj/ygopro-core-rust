//! Predaplant Pterapenthes
//! 捕食植物プテロペンテス
use super::prelude::*;

pub const ID: u32 = 26308721;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_PREDATOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // counter
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    // e1:SetCondition(s.ctcon1)
    effect::set_condition(d, e1, Some(condition(ctcon1)));
    // e1:SetTarget(s.cttg1)
    effect::set_target(d, e1, Some(cost_target(cttg1)));
    // e1:SetOperation(s.ctop1)
    effect::set_operation(d, e1, Some(operation(ctop1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // control
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e2, CATEGORY_CONTROL);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.cttg2)
    effect::set_target(d, e2, Some(cost_target(cttg2)));
    // e2:SetOperation(s.ctop2)
    effect::set_operation(d, e2, Some(operation(ctop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.ctcon1(e,tp,eg,ep,ev,re,r,rp)`
fn ctcon1(_d: &mut Duel, a: &EffArgs) -> bool {
    // return ep~=tp
    a.ep != a.tp
}
/// `s.cttg1(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn cttg1(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsCanAddCounter(COUNTER_PREDATOR,1) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_can_add_counter(d, chkc, COUNTER_PREDATOR as u16, 1, false, 0);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsCanAddCounter,tp,0,LOCATION_MZONE,1,nil,COUNTER_PREDATOR,1) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_can_add_counter(d, c, COUNTER_PREDATOR as u16, 1, false, 0)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,Card.IsCanAddCounter,tp,0,LOCATION_MZONE,1,1,nil,COUNTER_PREDATOR,1)
    duel::select_target(d, tp, Some(&|d, c| card::is_can_add_counter(d, c, COUNTER_PREDATOR as u16, 1, false, 0)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.ctop1(e,tp,eg,ep,ev,re,r,rp)`
fn ctop1(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:AddCounter(COUNTER_PREDATOR,1) and tc:GetLevel()>1 then
    if card::is_relate_to_effect(d, tc, a.e) && card::add_counter(d, tc, COUNTER_PREDATOR as u16, 1, false) && card::get_level(d, tc) > 1 {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // e1:SetCondition(s.lvcon)
        effect::set_condition(d, e1, Some(condition(lvcon)));
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.lvcon(e)`
fn lvcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetCounter(COUNTER_PREDATOR)>0
    let h = effect::get_handler(d, a.e).nil();
    card::get_counter(d, h, COUNTER_PREDATOR as u16) > 0
}
/// `s.ctfilter2(c,mc)`
fn ctfilter2(d: &mut Duel, c: CardId, mc: CardId) -> bool {
    // return c:IsFaceup() and c:IsLevelBelow(mc:GetLevel()) and c:IsControlerCanBeChanged()
    let lv = card::get_level(d, mc);
    card::is_faceup(d, c) && card::is_level_below(d, c, lv) && card::is_controler_can_be_changed(d, c, false, None)
}
/// `s.cttg2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn cttg2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.ctfilter2(chkc,c) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && ctfilter2(d, chkc, c);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.ctfilter2,tp,0,LOCATION_MZONE,1,nil,c) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| ctfilter2(d, x, c)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectTarget(tp,s.ctfilter2,tp,0,LOCATION_MZONE,1,1,nil,c)
    let g = duel::select_target(d, tp, Some(&|d, x| ctfilter2(d, x, c)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, 0, 0);
    true
}
/// `s.ctop2(e,tp,eg,ep,ev,re,r,rp)`
fn ctop2(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.GetControl(tc,tp,PHASE_END,1)
        duel::get_control(d, tc, a.tp, PHASE_END as u16, 1, None, None);
    }
}
