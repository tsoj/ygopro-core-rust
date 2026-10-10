//! Number 47: Nightmare Shark
use super::prelude::*;

pub const ID: u32 = 31320433;

pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(47), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must first be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure
    // Xyz.AddProcedure(c,nil,3,2)
    xyz::add_procedure(d, c, None, Some(3), 2, None, None, None, None, false, None);
    // Attach 1 Level 3 WATER monster from hand or field to this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetTarget(s.mattg)
    effect::set_target(d, e1, Some(cost_target(mattg)));
    // e1:SetOperation(s.matop)
    effect::set_operation(d, e1, Some(operation(matop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Targeted monster can attack directly
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.dacon)
    effect::set_condition(d, e2, Some(condition(dacon)));
    // e2:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e2:SetTarget(s.datg)
    effect::set_target(d, e2, Some(cost_target(datg)));
    // e2:SetOperation(s.daop)
    effect::set_operation(d, e2, Some(operation(daop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.matfilter(c)`
fn matfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsLocation(LOCATION_HAND) or c:IsFaceup()) and not c:IsType(TYPE_TOKEN)
    //     and c:IsLevel(3) and c:IsAttribute(ATTRIBUTE_WATER)
    (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
        && !card::is_type(d, c, TYPE_TOKEN)
        && card::is_level(d, c, &[3])
        && card::is_attribute(d, c, ATTRIBUTE_WATER)
}
/// `s.mattg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn mattg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.matfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&matfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    true
}
/// `s.matop(e,tp,eg,ep,ev,re,r,rp)`
fn matop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_XMATERIAL as u64);
    // local g=Duel.SelectMatchingCard(tp,s.matfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&matfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Overlay(c,g)
        duel::overlay(d, c, g, false);
    }
}
/// `s.dacon(e,tp,eg,ep,ev,re,r,rp)`
fn dacon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsAbleToEnterBP()
    duel::is_able_to_enter_bp(d)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_WATER) and not c:IsHasEffect(EFFECT_CANNOT_DIRECT_ATTACK)
    card::is_faceup(d, c)
        && card::is_attribute(d, c, ATTRIBUTE_WATER)
        && card::is_has_effect(d, c, EFFECT_CANNOT_DIRECT_ATTACK, None).is_empty()
}
/// `s.datg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn datg(d: &mut Duel, a: &EffArgs) -> bool {
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
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // e:SetLabel(g:GetFirst():GetFieldID())
    let tc = group::get_first(d, &g).nil();
    let fid = card::get_field_id(d, tc);
    effect::set_label(d, a.e, &[fid as i64]);
    true
}
/// `s.daop(e,tp,eg,ep,ev,re,r,rp)`
fn daop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3205)
        effect::set_description(d, e1, 3205);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DIRECT_ATTACK)
        effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
    // Other monsters cannot attack
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(s.ftarget)
    effect::set_target(d, e2, Some(effect_card_fn(ftarget)));
    // e2:SetLabel(e:GetLabel())
    let label = effect::get_label(d, a.e);
    effect::set_label(d, e2, &[label]);
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
}
/// `s.ftarget(e,c)`
fn ftarget(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return e:GetLabel()~=c:GetFieldID()
    effect::get_label(d, e) != card::get_field_id(d, c) as i64
}
