//! Strong Wind Dragon
use super::prelude::*;

pub const ID: u32 = 23770284;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon success
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // tribute check
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e2, EFFECT_MATERIAL_CHECK);
    // e2:SetValue(s.valcheck)
    effect::set_value_fn(d, e2, lua_fn(valcheck));
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // indes
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_BATTLE);
    // e3:SetValue(s.indval)
    effect::set_value_fn(d, e3, lua_fn(indval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // pierce
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e4, EFFECT_PIERCE);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, a: Args) -> Value {
    let e = a.e(0);
    let c = a.c(1);
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g).nil();
    // if tc:IsRace(RACE_DRAGON) then
    if card::is_race(d, tc, RACE_DRAGON) {
        // e:GetLabelObject():SetLabel(1)
        let le = effect::get_label_object(d, e).as_effect().nil();
        effect::set_label(d, le, &[1]);
    } else {
        // e:GetLabelObject():SetLabel(0)
        let le = effect::get_label_object(d, e).as_effect().nil();
        effect::set_label(d, le, &[0]);
    }
    Value::Nil
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsTributeSummoned() and e:GetLabel()==1
    let h = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, h) && effect::get_label(d, a.e) == 1
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // local atk=c:GetMaterial():GetFirst():GetTextAttack()
        let m = card::get_material(d, c);
        let mf = group::get_first(d, &m).nil();
        let mut atk = card::get_text_attack(d, mf);
        // if atk<0 then atk=0 end
        if atk < 0 {
            atk = 0;
        }
        // if atk>0 then
        if atk > 0 {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(atk/2)
            effect::set_value(d, e1, lua_int(atk as f64 / 2.0));
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
            effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD_DISABLE, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
    }
}
/// `s.indval(e,c)`
fn indval(d: &mut Duel, a: Args) -> bool {
    // return c:GetAttack()==e:GetHandler():GetAttack()
    let c = a.c(1);
    let h = effect::get_handler(d, a.e(0)).nil();
    card::get_attack(d, c) == card::get_attack(d, h)
}
