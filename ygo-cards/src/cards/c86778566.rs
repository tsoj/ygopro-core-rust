//! Impenetrable Attack
use super::prelude::*;

pub const ID: u32 = 86778566;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsBattlePhase()
    duel::is_battle_phase(d, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local opt=0
    // if Duel.IsExistingTarget(nil,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) then
    let opt = if duel::is_existing_target(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX) {
        // opt=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1))
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)])
    } else {
        // else opt=Duel.SelectOption(tp,aux.Stringid(id,1))+1 end
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1)]) + 1
    };
    // e:SetLabel(opt)
    effect::set_label(d, a.e, &[opt as i64]);
    // if opt==0 then
    if opt == 0 {
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
        // Duel.SelectTarget(tp,nil,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
        duel::select_target(d, a.tp, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    } else {
        // else e:SetProperty(0) end
        effect::set_property(d, a.e, 0, 0);
    }
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // if e:GetLabel()==0 then
    if effect::get_label(d, a.e) == 0 {
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, tc, a.e) {
            let h = effect::get_handler(d, a.e).nil();
            // local e1=Effect.CreateEffect(e:GetHandler())
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
            effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
            // e1:SetValue(1)
            effect::set_value(d, e1, 1);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    } else {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
        effect::set_code(d, e1, EFFECT_AVOID_BATTLE_DAMAGE);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESET_PHASE|PHASE_BATTLE)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_BATTLE, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
