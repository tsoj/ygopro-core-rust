//! Tackle Crusader
use super::prelude::*;

pub const ID: u32 = 12467005;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSpellTrap() and c:IsAbleToHand()
    card::is_faceup(d, c) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local b1=Duel.IsExistingTarget(aux.FaceupFilter(Card.IsCanTurnSet),tp,0,LOCATION_MZONE,1,nil)
    let b1 = duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c) && card::is_can_turn_set(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    // local b2=Duel.IsExistingTarget(s.filter2,tp,0,LOCATION_ONFIELD,1,nil)
    let b2 = duel::is_existing_target(d, Some(&|d, c| filter2(d, c)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=0
    // if b1 and b2 then op=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))
    // elseif b1 then op=Duel.SelectOption(tp,aux.Stringid(id,1))
    // else op=Duel.SelectOption(tp,aux.Stringid(id,2))+1 end
    let op = if b1 && b2 {
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)])
    } else if b1 {
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1)])
    } else {
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 2)]) + 1
    };
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    if op == 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_POSCHANGE as u64);
        // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsCanTurnSet),tp,0,LOCATION_MZONE,1,1,nil)
        let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c) && card::is_can_turn_set(d, c)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
        // e:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
        effect::set_category(d, a.e, CATEGORY_POSITION + CATEGORY_SET);
        // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,1,tp,POS_FACEDOWN_DEFENSE)
        duel::set_operation_info(d, 0, CATEGORY_POSITION, g, 1, a.tp, POS_FACEDOWN_DEFENSE as i32);
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
        // local g=Duel.SelectTarget(tp,s.filter2,tp,0,LOCATION_ONFIELD,1,1,nil)
        let g = duel::select_target(d, a.tp, Some(&|d, c| filter2(d, c)), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
        // e:SetCategory(CATEGORY_TOHAND)
        effect::set_category(d, a.e, CATEGORY_TOHAND);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // if e:GetLabel()==0 then
    if effect::get_label(d, a.e) == 0 {
        // Duel.ChangePosition(tc,POS_FACEDOWN_DEFENSE)
        duel::change_position(d, tc, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    } else {
        // local code=tc:GetCode()
        let code = card::get_code(d, tc)[0];
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
        effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
        // e1:SetTargetRange(0,1)
        effect::set_target_range(d, e1, 0, 1);
        // e1:SetValue(s.aclimit)
        effect::set_value_fn(d, e1, lua_fn(aclimit));
        // e1:SetLabel(code)
        effect::set_label(d, e1, &[code as i64]);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let re = a.e(1);
    // return re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:GetHandler():IsCode(e:GetLabel())
    if !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        return false;
    }
    let rc = effect::get_handler(d, re).nil();
    let code = effect::get_label(d, e) as u32;
    card::is_code(d, rc, &[code])
}
