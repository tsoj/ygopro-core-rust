//! Purushaddoll Aeon
use super::prelude::*;

pub const ID: u32 = 78942513;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_SHADDOLL}
    listed_series: &[SET_SHADDOLL],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE+CATEGORY_TOGRAVE+CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE + CATEGORY_TOGRAVE + CATEGORY_POSITION + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SHADDOLL) and c:IsFaceup()
    card::is_set_card(d, c, &[SET_SHADDOLL as u16]) && card::is_faceup(d, c)
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SHADDOLL) and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_SHADDOLL as u16]) && card::is_able_to_grave(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_HAND as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoGrave(g,REASON_EFFECT)>0 and g:IsExists(Card.IsLocation,1,nil,LOCATION_GRAVE) then
    if !g.is_empty()
        && duel::send_to_grave(d, g.clone(), REASON_EFFECT, None, None) > 0
        && group::is_exists(d, &g, &|d, x| card::is_location(d, x, LOCATION_GRAVE), 1, NO_EX)
    {
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
        if card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // e1:SetValue(1000)
            effect::set_value(d, e1, 1000);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_UPDATE_DEFENSE)
            effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
            // local fid=c:GetFieldID()
            let fid = card::get_field_id(d, c) as i64;
            // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
            card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid, 0);
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e3:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
            effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
            // e3:SetCode(EVENT_PHASE+PHASE_END)
            effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
            // e3:SetCountLimit(1)
            effect::set_count_limit(d, e3, 1, 0, 0, 0);
            // e3:SetLabel(fid)
            effect::set_label(d, e3, &[fid]);
            // e3:SetLabelObject(tc)
            effect::set_label_object(d, e3, Value::Card(tc));
            // e3:SetCondition(s.flipcon)
            effect::set_condition(d, e3, Some(condition(flipcon)));
            // e3:SetOperation(s.flipop)
            effect::set_operation(d, e3, Some(operation(flipop)));
            // Duel.RegisterEffect(e3,tp)
            duel::register_effect(d, e3, tp);
        }
    }
}
fn flipcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if e:GetLabelObject():GetFlagEffectLabel(id)~=e:GetLabel() then
    let lo = effect::get_label_object(d, a.e).as_card().nil();
    let flabel = card::get_flag_effect_label(d, lo, ID).first().copied();
    if flabel != Some(effect::get_label(d, a.e)) {
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        return false;
    }
    // else return true end
    true
}
fn flipop(d: &mut Duel, a: &EffArgs) {
    // Duel.ChangePosition(e:GetLabelObject(),POS_FACEDOWN_DEFENSE)
    let lo = effect::get_label_object(d, a.e).as_card().nil();
    duel::change_position(d, lo, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
}
