//! Greed Quasar
use super::prelude::*;

pub const ID: u32 = 50263751;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // base attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_BASE_ATTACK)
    effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_SET_BASE_DEFENSE)
    effect::set_code(d, e2, EFFECT_SET_BASE_DEFENSE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // lvup
    // local e2=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYING);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e3, Some(condition(cond)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e3, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e3, false);
}
// function s.atkval(e,c) return c:GetLevel()*300 end
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    card::get_level(d, c) as i64 * 300
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsRelateToBattle() and e:GetHandler():IsFaceup()
    let c = effect::get_handler(d, a.e).nil();
    card::is_relate_to_battle(d, c) && card::is_faceup(d, c)
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c).nil();
    // local lv=bc:GetLevel()
    let lv = card::get_level(d, bc) as i64;
    // if lv>0 then
    if lv > 0 {
        // if c:GetFlagEffect(id)==0 then
        if card::get_flag_effect(d, c, ID) == 0 {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_LEVEL)
            effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
            // e1:SetValue(lv)
            effect::set_value(d, e1, lv);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
            // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD_DISABLE,0,0)
            card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD_DISABLE, 0, 0, 0, 0);
            // e:SetLabelObject(e1)
            effect::set_label_object(d, a.e, Value::Effect(e1));
            // e:SetLabel(lv)
            effect::set_label(d, a.e, &[lv]);
        } else {
            // local pe=e:GetLabelObject()
            let pe = effect::get_label_object(d, a.e).as_effect().nil();
            // local ct=e:GetLabel()+lv
            let ct = effect::get_label(d, a.e) + lv;
            // e:SetLabel(ct)
            effect::set_label(d, a.e, &[ct]);
            // pe:SetValue(ct)
            effect::set_value(d, pe, ct);
        }
    }
}
