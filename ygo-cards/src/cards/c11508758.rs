//! Mutant Mindmaster
use super::prelude::*;

pub const ID: u32 = 11508758;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetCondition(s.ctlcon)
    effect::set_condition(d, e1, Some(condition(ctlcon)));
    // e1:SetTarget(s.ctltg)
    effect::set_target(d, e1, Some(cost_target(ctltg)));
    // e1:SetOperation(s.ctlop)
    effect::set_operation(d, e1, Some(operation(ctlop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.ctlcon(e,tp,eg,ep,ev,re,r,rp)
fn ctlcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttackTarget()~=nil and Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>1
    duel::get_attack_target(d).is_some() && duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE) > 1
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPosition(POS_FACEUP_ATTACK) and c:IsControlerCanBeChanged() and c:CanAttack()
    card::is_position(d, c, POS_FACEUP_ATTACK) && card::is_controler_can_be_changed(d, c, false, None) && card::can_attack(d, c)
}
// function s.ctltg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn ctltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&filter), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(g), 1, 0, 0);
    true
}
// function s.ctlop(e,tp,eg,ep,ev,re,r,rp)
fn ctlop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        let h = effect::get_handler(d, a.e).nil();
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
        effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // Duel.AdjustInstantly(tc)
        duel::adjust_instantly(d, Some(tc));
        // if Duel.GetControl(tc,tp,PHASE_BATTLE,1)~=0 then
        if duel::get_control(d, tc, tp, PHASE_BATTLE as u16, 1, None, None) != 0 {
            // if tc:CanAttack() and not tc:IsImmuneToEffect(e) then
            if card::can_attack(d, tc) && !card::is_immune_to_effect(d, tc, a.e) {
                // local ats=tc:GetAttackableTarget()
                let (ats, _) = card::get_attackable_target(d, tc);
                // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
                duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
                // if #ats>0 then
                if !ats.is_empty() {
                    // local g=ats:Select(tp,1,1,nil)
                    let g = group::select(d, &ats, tp, 1, 1, false, NO_EX).nil();
                    // Duel.CalculateDamage(tc,g:GetFirst())
                    let target = group::get_first(d, &g);
                    duel::calculate_damage(d, tc, target, false);
                }
            }
            // local e2=Effect.CreateEffect(e:GetHandler())
            let e2 = effect::create_effect(d, h);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_CANNOT_ATTACK)
            effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
            // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
            // e:GetHandler():RegisterEffect(e2,true)
            card::register_effect(d, h, e2, true);
        }
    }
}
