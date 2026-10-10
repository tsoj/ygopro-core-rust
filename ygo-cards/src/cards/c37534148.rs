//! Ribbon of Rebirth
use super::prelude::*;

pub const ID: u32 = 37534148;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,s.filter)
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, c, _e, _tp| filter(d, c))), None, None, None, None, None, None);
    // special summon
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    effect::set_condition(d, e3, Some(condition(regcon)));
    effect::set_operation(d, e3, Some(operation(regop)));
    card::register_effect(d, c, e3, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsHasEffect(EFFECT_CANNOT_SPECIAL_SUMMON)
    card::is_has_effect(d, c, EFFECT_CANNOT_SPECIAL_SUMMON, None).is_empty()
}
/// `s.regcon(e,tp,eg,ep,ev,re,r,rp)`
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ec=c:GetPreviousEquipTarget()
    let ec = card::get_previous_equip_target(d, c);
    // return c:IsReason(REASON_LOST_TARGET) and ec:IsReason(REASON_BATTLE) and ec:IsLocation(LOCATION_GRAVE)
    card::is_reason(d, c, REASON_LOST_TARGET) && card::is_reason(d, ec.nil(), REASON_BATTLE) && card::is_location(d, ec.nil(), LOCATION_GRAVE)
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ec=c:GetPreviousEquipTarget()
    let ec = card::get_previous_equip_target(d, c);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // e1:SetLabelObject(ec)
    effect::set_label_object(d, e1, ec.into());
    // e1:SetReset(RESET_EVENT|RESET_TOGRAVE|RESET_REMOVE|RESET_TOHAND|RESET_TODECK|RESET_TOFIELD|RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_EVENT | RESET_TOGRAVE | RESET_REMOVE | RESET_TOHAND | RESET_TODECK | RESET_TOFIELD | RESET_PHASE | PHASE_END, 1);
    // e:GetHandler():RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // ec:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
    card::register_flag_effect(d, ec.nil(), ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetPreviousEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_previous_equip_target(d, h);
    // if chk==0 then return ec:GetFlagEffect(id)~=0 end
    if a.chk == Some(0) {
        return card::get_flag_effect(d, ec.nil(), ID) != 0;
    }
    // Duel.SetTargetCard(ec)
    duel::set_target_card(d, ec.nil());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,ec,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(ec.nil())), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    if let Some(tc) = duel::first_target(d) {
        // if tc and tc:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
