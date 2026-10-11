//! Worm Jetelikpse
use super::prelude::*;

pub const ID: u32 = 84932271;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_FLIP)
    effect::set_code(d, e1, EVENT_FLIP);
    // e1:SetOperation(s.flipop)
    effect::set_operation(d, e1, Some(operation(flipop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.flipop(e,tp,eg,ep,ev,re,r,rp)`
fn flipop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) then
    if card::is_location(d, c, LOCATION_MZONE | LOCATION_GRAVE) {
        // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD_EXC_GRAVE|RESET_PHASE|PHASE_END,0,1)
        card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD_EXC_GRAVE | RESET_PHASE | PHASE_END, 0, 1, 0, 0);
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsReason(REASON_DESTROY)
    let h = effect::get_handler(d, a.e).nil();
    card::is_reason(d, h, REASON_DESTROY)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():GetFlagEffect(id)~=0 end
    if a.chk == Some(0) {
        return card::get_flag_effect(d, h, ID) != 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let h = effect::get_handler(d, e).nil();
    // if e:GetHandler():IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, h, e) {
        // Duel.SpecialSummon(e:GetHandler(),0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, CardSet::from_card(h), 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
