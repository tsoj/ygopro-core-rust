//! Necro Defender
use super::prelude::*;

pub const ID: u32 = 77700347;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply effects to 1 monster you control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e1, Some(cost::self_banish()));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.tgfilter(c,tp)
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return not (c:HasFlagEffect(id) and c:GetFlagEffectLabel(id)==tp)
    !(card::has_flag_effect(d, c, ID, None) && card::get_flag_effect_label(d, c, ID).first() == Some(&(tp as i64)))
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.tgfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| tgfilter(d, x, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_APPLYTO)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_APPLYTO as u64);
    // Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    duel::select_target(d, tp, Some(&|d, x| tgfilter(d, x, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and s.tgfilter(tc,tp) then
    if card::is_relate_to_effect(d, tc, a.e) && tgfilter(d, tc, tp) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,2,tp)
        card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 2, tp as i64, 0);
        // Cannot be destroyed by battle
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e1, aux::stringid(ID, 1));
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESETS_STANDARD_PHASE_END,2)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 2);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // You take no battle damage from battles involving it
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD);
        // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e2:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
        effect::set_code(d, e2, EFFECT_AVOID_BATTLE_DAMAGE);
        // e2:SetTargetRange(1,0)
        effect::set_target_range(d, e2, 1, 0);
        // e2:SetTarget(function(e,c) return s.tgfilter(c,tp) end)
        effect::set_target(d, e2, Some(effect_card_fn(move |d, _e, x| tgfilter(d, x, tp))));
        // e2:SetValue(1)
        effect::set_value(d, e2, 1);
        // e2:SetReset(RESET_PHASE|PHASE_END,2)
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 2);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, tp);
    }
}
