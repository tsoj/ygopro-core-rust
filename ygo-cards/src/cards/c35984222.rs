//! Ido the Supreme Magical Force
use super::prelude::*;

pub const ID: u32 = 35984222;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // spsummon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetOperation(s.spr)
    effect::set_operation(d, e1, Some(operation(spr)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e3, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_CANNOT_FLIP_SUMMON)
    effect::set_code(d, e4, EFFECT_CANNOT_FLIP_SUMMON);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e3:Clone()
    let e5 = effect::clone(d, e3);
    // e5:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e5, EFFECT_CANNOT_SUMMON);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // local e6=e3:Clone()
    let e6 = effect::clone(d, e3);
    // e6:SetCode(EFFECT_CANNOT_MSET)
    effect::set_code(d, e6, EFFECT_CANNOT_MSET);
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.spr(e,tp,eg,ep,ev,re,r,rp)`
fn spr(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if (r&(REASON_DESTROY|REASON_EFFECT)~=(REASON_DESTROY|REASON_EFFECT)) or c:IsPreviousLocation(LOCATION_SZONE) then return end
    if (a.r & (REASON_DESTROY | REASON_EFFECT)) != (REASON_DESTROY | REASON_EFFECT) || card::is_previous_location(d, c, LOCATION_SZONE) {
        return;
    }
    // c:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,2)
    card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 2, 0, 0);
}
/// `s.spcon(e,tp,...)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:GetTurnID()~=Duel.GetTurnCount() and c:GetFlagEffect(id)>0
    card::get_turn_id(d, c) as i32 != duel::get_turn_count(d, None) && card::get_flag_effect(d, c, ID) > 0
}
/// `s.sptg(e,tp,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_MZONE,0,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, 0, h.into());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,1,tp,tp,false,false,POS_FACEUP)~=0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 1, a.tp, a.tp, false, false, POS_FACEUP as u8, None) != 0 {
        // local g=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_MZONE,0,c)
        let g = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, 0, c.into());
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
