//! Morphtronic Staplen
use super::prelude::*;

pub const ID: u32 = 2250266;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e1, EVENT_LEAVE_FIELD_P);
    // e1:SetOperation(s.check)
    effect::set_operation(d, e1, Some(operation(check)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    // e2:SetCondition(s.cona)
    effect::set_condition(d, e2, Some(condition(cona)));
    // e2:SetOperation(s.opa)
    effect::set_operation(d, e2, Some(operation(opa)));
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e3, 0, LOCATION_MZONE);
    // e3:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e3:SetCondition(s.cona2)
    effect::set_condition(d, e3, Some(lua_fn(cona2)));
    // e3:SetValue(s.atlimit)
    effect::set_value_fn(d, e3, effect_card_fn(atlimit));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // def
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_BATTLE);
    // e4:SetCondition(s.cond)
    effect::set_condition(d, e4, Some(lua_fn(cond)));
    // e4:SetValue(1)
    effect::set_value(d, e4, 1);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e5, CATEGORY_POSITION);
    // e5:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e5, EFFECT_FLAG_CARD_TARGET, 0);
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e5:SetCode(EVENT_BATTLED)
    effect::set_code(d, e5, EVENT_BATTLED);
    // e5:SetCondition(s.cond)
    effect::set_condition(d, e5, Some(lua_fn(cond)));
    // e5:SetTarget(s.tgd2)
    effect::set_target(d, e5, Some(cost_target(tgd2)));
    // e5:SetOperation(s.opd2)
    effect::set_operation(d, e5, Some(operation(opd2)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.check(e,tp,eg,ep,ev,re,r,rp)`
fn check(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsDisabled() and c:IsAttackPos() then e:SetLabel(1)
    if !card::is_disabled(d, c) && card::is_attack_pos(d, c) {
        effect::set_label(d, a.e, &[1]);
    } else {
        // else e:SetLabel(0) end
        effect::set_label(d, a.e, &[0]);
    }
}
/// `s.cona(e,tp,eg,ep,ev,re,r,rp)`
fn cona(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetLabelObject():GetLabel()==1
    let le = effect::get_label_object(d, a.e).as_effect().nil();
    effect::get_label(d, le) == 1
}
/// `s.opa(e,tp,eg,ep,ev,re,r,rp)`
fn opa(d: &mut Duel, a: &EffArgs) {
    // local rc=e:GetHandler():GetReasonCard()
    let c = effect::get_handler(d, a.e).nil();
    let rc = card::get_reason_card(d, c).nil();
    // if rc:IsRelateToBattle() then
    if card::is_relate_to_battle(d, rc) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(-300)
        effect::set_value(d, e1, -300);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // rc:RegisterEffect(e1)
        card::register_effect(d, rc, e1, false);
    }
}
/// `s.cona2(e)`
fn cona2(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():IsAttackPos()
    let c = effect::get_handler(d, a.e(0)).nil();
    card::is_attack_pos(d, c)
}
/// `s.atlimit(e,c)`
fn atlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c~=e:GetHandler()
    c != effect::get_handler(d, e).nil()
}
/// `s.cond(e)`
fn cond(d: &mut Duel, a: Args) -> bool {
    // return not e:GetHandler():IsDisabled() and e:GetHandler():IsDefensePos()
    let c = effect::get_handler(d, a.e(0)).nil();
    !card::is_disabled(d, c) && card::is_defense_pos(d, c)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPosition(POS_FACEUP_ATTACK) and c:IsCanChangePosition()
    card::is_position(d, c, POS_FACEUP_ATTACK) && card::is_can_change_position(d, c)
}
/// `s.tgd2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tgd2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && filter(d, chkc);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTACK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATTACK as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g), n, 0, 0);
    true
}
/// `s.opd2(e,tp,eg,ep,ev,re,r,rp)`
fn opd2(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:IsAttackPos()
    //     and Duel.ChangePosition(tc,POS_FACEUP_DEFENSE)~=0 and c:IsRelateToBattle() then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e)
            && card::is_faceup(d, tc)
            && card::is_attack_pos(d, tc)
            && duel::change_position(d, tc, POS_FACEUP_DEFENSE as u8, None, None, None, false) != 0
            && card::is_relate_to_battle(d, c)
        {
            // Duel.ChangePosition(e:GetHandler(),POS_FACEUP_ATTACK,0,POS_FACEUP_ATTACK,0)
            duel::change_position(d, c, POS_FACEUP_ATTACK as u8, Some(0), Some(POS_FACEUP_ATTACK as u8), Some(0), false);
        }
    }
}
