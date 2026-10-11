//! Ruin, Angel of Oblivion
use super::prelude::*;

pub const ID: u32 = 50139096;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[32828635, 46427957], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Change this card's name
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e0, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e0:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e0, EFFECT_CHANGE_CODE);
    // e0:SetRange(LOCATION_MZONE|LOCATION_HAND)
    effect::set_range(d, e0, LOCATION_MZONE | LOCATION_HAND);
    // e0:SetValue(46427957)
    effect::set_value(d, e0, 46427957);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Multiple attacks
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Opponent cannot activate cards/effects when your Ritual attacks
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetTarget(s.aclimtg)
    effect::set_target(d, e2, Some(cost_target(aclimtg)));
    // e2:SetOperation(s.aclimop)
    effect::set_operation(d, e2, Some(operation(aclimop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `aux.FaceupFilter(Card.IsType,TYPE_RITUAL)`
fn ritual_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_RITUAL)
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsAbleToEnterBP() and e:GetHandler():IsRitualSummoned()
    if !duel::is_able_to_enter_bp(d) {
        return false;
    }
    let c = effect::get_handler(d, a.e).nil();
    card::is_ritual_summoned(d, c)
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Can make up to 2 attacks on monsters
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_EXTRA_ATTACK_MONSTER)
        effect::set_code(d, e1, EFFECT_EXTRA_ATTACK_MONSTER);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
fn aclimtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and chkc:IsFaceup() and chkc:IsType(TYPE_RITUAL) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && card::is_faceup(d, chkc) && card::is_type(d, chkc, TYPE_RITUAL);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsType,TYPE_RITUAL),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&ritual_faceup), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsType,TYPE_RITUAL),tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&ritual_faceup), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
fn aclimop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // Cannot activate cards/effects when your Ritual attacks
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
        effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetCondition(s.actcon)
        effect::set_condition(d, e1, Some(condition(actcon)));
        // e1:SetOperation(s.actop)
        effect::set_operation(d, e1, Some(operation(actop)));
        // e1:SetLabelObject(tc)
        effect::set_label_object(d, e1, Value::Card(tc));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
        // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_CONTROL,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,2))
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD | RESET_CONTROL, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 2));
    }
}
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // if tc:GetFlagEffect(id)==0 then
    if card::get_flag_effect(d, tc, ID) == 0 {
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        return false;
    }
    // local ac=Duel.GetAttacker()
    // return ac and ac:IsControler(tp) and ac:IsType(TYPE_RITUAL) and tc:GetFlagEffect(id)>0
    match duel::get_attacker(d) {
        Some(ac) => card::is_controler(d, ac, a.tp) && card::is_type(d, ac, TYPE_RITUAL) && card::get_flag_effect(d, tc, ID) > 0,
        None => false,
    }
}
fn actop(d: &mut Duel, a: &EffArgs) {
    // Cannot activate
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetCondition(s.actlimitcon)
    effect::set_condition(d, e1, Some(condition(actlimitcon)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetReset(RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
fn actlimitcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.CheckEvent(EVENT_ATTACK_ANNOUNCE)
    duel::check_event(d, EVENT_ATTACK_ANNOUNCE).is_some()
}
