//! Morphtronic Engine
use super::prelude::*;

pub const ID: u32 = 7817703;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,s.filter,nil,nil,nil,s.operation)
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, c, _e, _tp| filter(d, c))),
        None,
        None,
        None,
        Some(operation(op)),
        None,
        None,
    );
    // Atk Change
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_SET_ATTACK)
    effect::set_code(d, e2, EFFECT_SET_ATTACK);
    // e2:SetValue(s.value)
    effect::set_value_fn(d, e2, effect_card_value(value));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MORPHTRONIC) and c:GetLevel()==3
    card::is_set_card(d, c, &[SET_MORPHTRONIC as u16]) && card::get_level(d, c) == 3
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::get_first_target(d).first().copied();
    // if e:GetHandler():IsRelateToEffect(e) and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) && card::is_relate_to_effect(d, tc.nil(), a.e) && card::is_faceup(d, tc.nil()) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
        effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
        // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
        effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
        // e1:SetRange(LOCATION_SZONE)
        effect::set_range(d, e1, LOCATION_SZONE);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetLabel(Duel.GetTurnCount())
        let turn = duel::get_turn_count(d, None) as i64;
        effect::set_label(d, e1, &[turn]);
        // e1:SetCondition(s.descon)
        effect::set_condition(d, e1, Some(condition(descon)));
        // e1:SetTarget(s.destg)
        effect::set_target(d, e1, Some(cost_target(destg)));
        // e1:SetOperation(s.desop)
        effect::set_operation(d, e1, Some(operation(desop)));
        // e1:SetReset(RESETS_STANDARD_PHASE_END|RESET_SELF_TURN,3)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END | RESET_SELF_TURN, 3);
        // e:GetHandler():RegisterEffect(e1)
        card::register_effect(d, h, e1, false);
    }
}
// function s.value(e,c)
fn value(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetBaseAttack()*2
    card::get_base_attack(d, c) as i64 * 2
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetTurnCount()-e:GetLabel()==4
    duel::get_turn_count(d, None) as i64 - effect::get_label(d, a.e) == 4
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(h)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, a.tp, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // local dam=c:GetEquipTarget():GetBaseAttack()
        let dam = card::get_base_attack(d, card::get_equip_target(d, c).nil()) as i64;
        // if Duel.Destroy(e:GetHandler(),REASON_EFFECT)==0 then return end
        if duel::destroy(d, CardSet::from_card(c), REASON_EFFECT, None, None) == 0 {
            return;
        }
        // Duel.Damage(tp,dam,REASON_EFFECT)
        duel::damage(d, a.tp, dam, REASON_EFFECT, false, None);
    }
}
