//! Level Warrior
use super::prelude::*;

pub const ID: u32 = 97385276;

/// `function(c) c:AssumeProperty(ASSUME_LEVEL,2) end` (the label object's function)
fn assume_level2(d: &mut Duel, a: Args) -> Value {
    card::assume_property(d, a.c(0), ASSUME_LEVEL, 2);
    Value::Nil
}
/// `function(c) c:AssumeProperty(ASSUME_LEVEL,4) end` (the label object's function)
fn assume_level4(d: &mut Duel, a: Args) -> Value {
    card::assume_property(d, a.c(0), ASSUME_LEVEL, 4);
    Value::Nil
}

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If there are no monsters on the field, you can Normal Summon this card as a Level 2 monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SUMMON_PROC);
    // e1:SetCondition(s.nscon)
    effect::set_condition(d, e1, Some(lua_fn(nscon)));
    // e1:SetOperation(s.nsop)
    effect::set_operation(d, e1, Some(lua_fn(nsop)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetLabelObject({function(c) c:AssumeProperty(ASSUME_LEVEL,2) end})
    effect::set_label_object(d, e1, Value::Multi(vec![Value::Fn(lua_fn(assume_level2))]));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If your opponent controls a monster and you control no monsters, you can Special Summon this card (from your hand) as a Level 4 monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(lua_fn(spop)));
    // e2:SetLabelObject({function(c) c:AssumeProperty(ASSUME_LEVEL,4) end})
    effect::set_label_object(d, e2, Value::Multi(vec![Value::Fn(lua_fn(assume_level4))]));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.nscon(e,c,minc)`
fn nscon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let c = match a.card(1) {
        None => return true,
        Some(c) => c,
    };
    // return minc==0 and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    //     and Duel.GetFieldGroupCount(0,LOCATION_MZONE,LOCATION_MZONE)==0
    let minc = a.int(2);
    let tp = card::get_controler(d, c);
    minc == 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::get_field_group_count(d, 0, LOCATION_MZONE, LOCATION_MZONE) == 0
}
/// `s.nsop(e,tp,eg,ep,ev,re,r,rp,c)`
fn nsop(d: &mut Duel, a: Args) -> Value {
    // c:AssumeProperty(ASSUME_LEVEL,2)
    let c = a.c(8);
    card::assume_property(d, c, ASSUME_LEVEL, 2);
    // Normal Summon this card as a Level 2 monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CHANGE_LEVEL)
    effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
    // e1:SetValue(2)
    effect::set_value(d, e1, 2);
    // e1:SetReset(RESET_EVENT|(RESETS_STANDARD_DISABLE&~RESET_TOFIELD))
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    Value::Nil
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let c = match a.card(1) {
        None => return true,
        Some(c) => c,
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0
    //     and Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) > 0
        && duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) == 0
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) -> Value {
    // c:AssumeProperty(ASSUME_LEVEL,4)
    let c = a.c(8);
    card::assume_property(d, c, ASSUME_LEVEL, 4);
    // Special Summon this card (from your hand) as a Level 4 monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CHANGE_LEVEL)
    effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
    // e1:SetValue(4)
    effect::set_value(d, e1, 4);
    // e1:SetReset(RESET_EVENT|(RESETS_STANDARD_DISABLE&~RESET_TOFIELD))
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    Value::Nil
}
