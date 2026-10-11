//! Celestial Sword - Eatos
use super::prelude::*;

pub const ID: u32 = 55569674;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[34022290], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // Atk up
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetValue(500)
    effect::set_value(d, e2, 500);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // atk
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e4, CATEGORY_ATKCHANGE);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e4, EVENT_TO_GRAVE);
    // e4:SetCondition(s.atkcon)
    effect::set_condition(d, e4, Some(condition(atkcon)));
    // e4:SetTarget(s.atktg)
    effect::set_target(d, e4, Some(cost_target(atktg)));
    // e4:SetOperation(s.atkop)
    effect::set_operation(d, e4, Some(operation(atkop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// s.atkcon(e,tp,eg,ep,ev,re,r,rp)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD)
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_ONFIELD)
}
// s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCode(34022290)
    card::is_faceup(d, c) && card::is_code(d, c, &[34022290])
}
// s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
// s.atkfilter(c)
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsMonster()
    card::is_faceup(d, c) && card::is_monster(d, c)
}
// s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // local ct=Duel.GetMatchingGroupCount(s.atkfilter,tp,LOCATION_REMOVED,LOCATION_REMOVED,nil)
        let ct = duel::get_matching_group_count(d, Some(&atkfilter), a.tp, LOCATION_REMOVED, LOCATION_REMOVED, NO_EX);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // e1:SetValue(500*ct)
        effect::set_value(d, e1, 500 * ct as i64);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
