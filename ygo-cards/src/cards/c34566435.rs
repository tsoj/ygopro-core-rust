//! Edge Imp Frightfuloid
use super::prelude::*;

pub const ID: u32 = 34566435;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_FRIGHTFUR],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Increase ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsLocation(LOCATION_GRAVE) or c:IsFaceup()) and c:IsType(TYPE_FUSION) and c:IsSetCard(SET_FRIGHTFUR)
    (card::is_location(d, c, LOCATION_GRAVE) || card::is_faceup(d, c))
        && card::is_type(d, c, TYPE_FUSION)
        && card::is_set_card(d, c, &[SET_FRIGHTFUR as u16])
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil)
    let _ = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and c:IsFaceup() and tc and tc:IsRelateToEffect(e) and (tc:IsLocation(LOCATION_GRAVE) or tc:IsFaceup()) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, c, a.e)
            && card::is_faceup(d, c)
            && card::is_relate_to_effect(d, tc, a.e)
            && (card::is_location(d, tc, LOCATION_GRAVE) || card::is_faceup(d, tc))
        {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
            effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
            // e1:SetValue(tc:GetBaseAttack())
            let base_atk = card::get_base_attack(d, tc) as i64;
            effect::set_value(d, e1, base_atk);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_SET_DEFENSE_FINAL)
            effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
            // e2:SetValue(tc:GetBaseDefense())
            let base_def = card::get_base_defense(d, tc) as i64;
            effect::set_value(d, e2, base_def);
            // c:RegisterEffect(e2)
            card::register_effect(d, c, e2, false);
        }
    }
}
