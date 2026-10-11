//! Hazy Flame Hydra
//! --陽炎獣 ヒュドラー
use super::prelude::*;

pub const ID: u32 = 8696773;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot be target
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(aux.tgoval)
    effect::set_value_fn(d, e1, aux::tgoval());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // sum limit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetTarget(s.splimit)
    effect::set_target(d, e2, Some(effect_card_fn(splimit)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // effect gain
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e3, EVENT_BE_MATERIAL);
    // e3:SetCondition(s.effcon)
    effect::set_condition(d, e3, Some(condition(effcon)));
    // e3:SetOperation(s.effop)
    effect::set_operation(d, e3, Some(operation(effop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:GetAttribute()~=ATTRIBUTE_FIRE
    card::get_attribute(d, c) != ATTRIBUTE_FIRE
}
fn effcon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return r==REASON_XYZ
    a.r == REASON_XYZ
}
fn effop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rc=c:GetReasonCard()
    let rc = card::get_reason_card(d, c).nil();
    // local e1=Effect.CreateEffect(rc)
    let e1 = effect::create_effect(d, rc);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.matcon)
    effect::set_condition(d, e1, Some(condition(matcon)));
    // e1:SetTarget(s.mattg)
    effect::set_target(d, e1, Some(cost_target(mattg)));
    // e1:SetOperation(s.matop)
    effect::set_operation(d, e1, Some(operation(matop)));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // rc:RegisterEffect(e1,true)
    card::register_effect(d, rc, e1, true);
    // if not rc:IsType(TYPE_EFFECT) then
    if !card::is_type(d, rc, TYPE_EFFECT) {
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_ADD_TYPE)
        effect::set_code(d, e2, EFFECT_ADD_TYPE);
        // e2:SetValue(TYPE_EFFECT)
        effect::set_value(d, e2, TYPE_EFFECT as i64);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // rc:RegisterEffect(e2,true)
        card::register_effect(d, rc, e2, true);
    }
}
fn matcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsXyzSummoned()
    let c = effect::get_handler(d, a.e).nil();
    card::is_xyz_summoned(d, c)
}
fn matfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_HAZY_FLAME) and c:IsMonster()
    card::is_set_card(d, c, &[SET_HAZY_FLAME as u16]) && card::is_monster(d, c)
}
fn mattg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.matfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && matfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.matfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&matfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_XMATERIAL as u64);
    // local g=Duel.SelectTarget(tp,s.matfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&matfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, g, 1, 0, 0);
    true
}
fn matop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        let tc = duel::first_target(d).nil();
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Overlay(c,tc)
            duel::overlay(d, c, tc, false);
        }
    }
}
