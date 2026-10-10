//! Warning Point
use super::prelude::*;

pub const ID: u32 = 11429811;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate on Normal, Flip, or Special Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `aux.FaceupFilter(Card.IsLocation,LOCATION_MZONE)`
fn faceup_in_mzone(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_location(d, c, LOCATION_MZONE)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return eg and eg:IsExists(aux.FaceupFilter(Card.IsLocation,LOCATION_MZONE),1,nil) end
    if a.chk == Some(0) {
        return match a.eg.clone() {
            Some(eg) => group::is_exists(d, &eg, &faceup_in_mzone, 1, NO_EX),
            None => false,
        };
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if not eg or #eg<1 then return end
    let Some(eg) = a.eg.clone() else { return };
    if eg.is_empty() {
        return;
    }
    // local g=eg:Filter(aux.FaceupFilter(Card.IsLocation,LOCATION_MZONE),nil)
    let g = group::filter(d, &eg, &faceup_in_mzone, NO_EX);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // Duel.NegateRelatedChain(tc,RESET_TURN_SET)
        duel::negate_related_chain(d, tc, RESET_TURN_SET as u32);
        // Negate the effects of the Summoned monster(s)
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // Cannot attack
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetCode(EFFECT_CANNOT_ATTACK)
        effect::set_code(d, e3, EFFECT_CANNOT_ATTACK);
        // e3:SetValue(1)
        effect::set_value(d, e3, 1);
        // e3:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e3)
        card::register_effect(d, tc, e3, false);
        // Cannot be used as material for a Fusion/Synchro/Xyz/Link Summon
        // local e4=Effect.CreateEffect(c)
        let e4 = effect::create_effect(d, c);
        // e4:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e4, aux::stringid(ID, 1));
        // e4:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
        // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e4:SetCode(EFFECT_CANNOT_BE_MATERIAL)
        effect::set_code(d, e4, EFFECT_CANNOT_BE_MATERIAL);
        // e4:SetValue(aux.cannotmatfilter(SUMMON_TYPE_FUSION,SUMMON_TYPE_SYNCHRO,SUMMON_TYPE_XYZ,SUMMON_TYPE_LINK))
        effect::set_value_fn(d, e4, aux::cannotmatfilter(&[SUMMON_TYPE_FUSION, SUMMON_TYPE_SYNCHRO, SUMMON_TYPE_XYZ, SUMMON_TYPE_LINK]));
        // e4:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e4, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e4)
        card::register_effect(d, tc, e4, false);
    }
}
