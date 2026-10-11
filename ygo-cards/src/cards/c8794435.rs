//! Maju Garzett
use super::prelude::*;

pub const ID: u32 = 8794435;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // tribute check
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e1, EFFECT_MATERIAL_CHECK);
    // e1:SetValue(s.valcheck)
    effect::set_value_fn(d, e1, effect_card_value(valcheck));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // give atk effect only when summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_SUMMON_COST)
    effect::set_code(d, e2, EFFECT_SUMMON_COST);
    // e2:SetOperation(s.facechk)
    effect::set_operation(d, e2, Some(operation(facechk)));
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // local tc=g:GetFirst()
    let _ = group::get_first(d, &g);
    // local atk=0
    let mut atk: i64 = 0;
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // local catk=tc:GetTextAttack()
        let catk = card::get_text_attack(d, tc);
        // atk=atk+(catk>=0 and catk or 0)
        atk += if catk >= 0 { catk as i64 } else { 0 };
    }
    // if e:GetLabel()==1 then
    if effect::get_label(d, e) == 1 {
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK)
        effect::set_code(d, e1, EFFECT_SET_ATTACK);
        // e1:SetValue(atk)
        effect::set_value(d, e1, atk);
        // e1:SetReset(RESET_EVENT|RESET_DISABLE|RESET_TURN_SET|RESET_TOGRAVE|RESET_REMOVE|RESET_TEMP_REMOVE|RESET_TOHAND|RESET_TODECK|RESET_LEAVE)
        effect::set_reset(
            d,
            e1,
            RESET_EVENT | RESET_DISABLE | RESET_TURN_SET | RESET_TOGRAVE | RESET_REMOVE | RESET_TEMP_REMOVE | RESET_TOHAND | RESET_TODECK | RESET_LEAVE,
            1,
        );
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
    0
}
/// `s.facechk(e,tp,eg,ep,ev,re,r,rp)`
fn facechk(d: &mut Duel, a: &EffArgs) {
    // e:GetLabelObject():SetLabel(1)
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_label(d, lo, &[1]);
}
