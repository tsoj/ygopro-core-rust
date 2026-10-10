//! The Immortal of Thunder
use super::prelude::*;

pub const ID: u32 = 84926738;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Register a flag to this card when it is flipped face-up
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0:SetCode(EVENT_FLIP)
    effect::set_code(d, e0, EVENT_FLIP);
    // e0:SetOperation(function(e) e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1) end)
    effect::set_operation(d, e0, Some(operation(|d, a| {
        let c = effect::get_handler(d, a.e).nil();
        card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
    })));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Gain 3000 LP if it is flipped face-up
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e1, CATEGORY_RECOVER);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    // e1:SetOperation(function(_,tp) Duel.Recover(tp,3000,REASON_EFFECT) end)
    effect::set_operation(d, e1, Some(operation(|d, a| {
        duel::recover(d, a.tp, 3000, REASON_EFFECT, false, None);
    })));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Lose 5000 LP when it is sent to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCondition(function(e) return e:GetLabel()>0 end)
    effect::set_condition(d, e2, Some(condition(|d, a| effect::get_label(d, a.e) > 0)));
    // e2:SetOperation(s.lpop)
    effect::set_operation(d, e2, Some(operation(lpop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Enable the effect above when it is about to leave the field if it was flipped face-up
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e3, EVENT_LEAVE_FIELD_P);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3:SetOperation(s.regop)
    effect::set_operation(d, e3, Some(operation(regop)));
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.lpop(e,tp,eg,ep,ev,re,r,rp)`
fn lpop(d: &mut Duel, a: &EffArgs) {
    // Duel.SetLP(tp,Duel.GetLP(tp)-5000)
    let lp = duel::get_lp(d, a.tp);
    duel::set_lp(d, a.tp, lp - 5000);
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    let e2 = effect::get_label_object(d, a.e).as_effect().nil();
    // if e:GetHandler():HasFlagEffect(id) then e:GetLabelObject():SetLabel(1)
    if card::has_flag_effect(d, c, ID, None) {
        effect::set_label(d, e2, &[1]);
    } else {
        // else e:GetLabelObject():SetLabel(0) end
        effect::set_label(d, e2, &[0]);
    }
}
