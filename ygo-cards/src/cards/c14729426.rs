//! Interplanetary Invader "A"
use super::prelude::*;

pub const ID: u32 = 14729426;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // return to hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e1, EVENT_BATTLE_START);
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler()==Duel.GetAttackTarget() and e:GetHandler():IsAttackPos() then
    let h = effect::get_handler(d, a.e).nil();
    if duel::get_attack_target(d) == Some(h) && card::is_attack_pos(d, h) {
        // local a=Duel.GetAttacker()
        let at = duel::get_attacker(d).nil();
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,0))
        effect::set_description(d, e1, aux::stringid(ID, 0));
        // e1:SetCategory(CATEGORY_CONTROL)
        effect::set_category(d, e1, CATEGORY_CONTROL);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
        // e1:SetCode(EVENT_PHASE|PHASE_BATTLE)
        effect::set_code(d, e1, EVENT_PHASE | PHASE_BATTLE);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetTarget(s.cttg)
        effect::set_target(d, e1, Some(cost_target(cttg)));
        // e1:SetOperation(s.ctop)
        effect::set_operation(d, e1, Some(operation(ctop)));
        // e1:SetLabelObject(a)
        effect::set_label_object(d, e1, Value::Card(at));
        // e1:SetLabel(a:GetRealFieldID())
        let fid = card::get_real_field_id(d, at);
        effect::set_label(d, e1, &[fid as i64]);
        // e1:SetReset(RESET_PHASE|PHASE_BATTLE)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_BATTLE, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
// function s.cttg(e,tp,eg,ep,ev,re,r,rp,chk)
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local a=e:GetLabelObject()
    let at = effect::get_label_object(d, a.e).as_card().nil();
    // if a:IsControler(1-tp) and a:GetRealFieldID()==e:GetLabel() then
    if card::is_controler(d, at, 1 - a.tp) && card::get_real_field_id(d, at) as i64 == effect::get_label(d, a.e) {
        // Duel.SetTargetCard(a)
        duel::set_target_card(d, at);
        // Duel.SetOperationInfo(0,CATEGORY_CONTROL,a,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(CardSet::from_card(at)), 1, 0, 0);
    }
    true
}
// function s.ctop(e,tp,eg,ep,ev,re,r,rp)
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.GetControl(tc,tp)
            duel::get_control(d, tc, a.tp, 0, 0, None, None);
        }
    }
}
