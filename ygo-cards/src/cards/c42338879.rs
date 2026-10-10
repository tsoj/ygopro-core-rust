//! Cockadoodledoo
use super::prelude::*;

pub const ID: u32 = 42338879;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If there are no monsters on the field, you can Special Summon this card (from your hand) as a Level 3 monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.lv3spcon)
    effect::set_condition(d, e1, Some(lua_fn(|d, a: Args| lv3spcon(d, a.e(0), a.card(1)))));
    // e1:SetOperation(s.spop(3))
    effect::set_operation(d, e1, Some(spop(3)));
    // e1:SetLabelObject({function(c) c:AssumeProperty(ASSUME_LEVEL,3) end})
    effect::set_label_object(d, e1, label_table(3));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If your opponent controls a monster and you control no cards, you can Special Summon this card (from your hand) as a Level 4 monster
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCondition(s.lv4spcon)
    effect::set_condition(d, e2, Some(lua_fn(|d, a: Args| lv4spcon(d, a.e(0), a.card(1)))));
    // e2:SetOperation(s.spop(4))
    effect::set_operation(d, e2, Some(spop(4)));
    // e2:SetLabelObject({function(c) c:AssumeProperty(ASSUME_LEVEL,4) end})
    effect::set_label_object(d, e2, label_table(4));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // If this face-up card would leave the field, banish it instead
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
    effect::set_code(d, e3, EFFECT_LEAVE_FIELD_REDIRECT);
    // e3:SetCondition(function(e) return e:GetHandler():IsFaceup() end)
    effect::set_condition(
        d,
        e3,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::is_faceup(d, h)
        })),
    );
    // e3:SetValue(LOCATION_REMOVED)
    effect::set_value(d, e3, LOCATION_REMOVED as i64);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// The table `{function(c) c:AssumeProperty(ASSUME_LEVEL,lv) end}` set as the label object of the
/// Special Summon procedures (the core never reads it for these effects).
fn label_table(lv: u32) -> Value {
    Value::Multi(vec![Value::Fn(lua_fn(move |d, a: Args| {
        let c = a.c(0);
        card::assume_property(d, c, ASSUME_LEVEL, lv as u64);
    }))])
}
/// `s.lv3spcon(e,c)`
fn lv3spcon(d: &mut Duel, _e: EffectId, c: Option<CardId>) -> bool {
    // if c==nil then return true end
    let Some(c) = c else { return true };
    // return Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0 and Duel.GetFieldGroupCount(0,LOCATION_MZONE,LOCATION_MZONE)==0
    let tp = card::get_controler(d, c);
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && duel::get_field_group_count(d, 0, LOCATION_MZONE, LOCATION_MZONE) == 0
}
/// `s.lv4spcon(e,c)`
fn lv4spcon(d: &mut Duel, _e: EffectId, c: Option<CardId>) -> bool {
    // if c==nil then return true end
    let Some(c) = c else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0
    //   and Duel.GetFieldGroupCount(tp,LOCATION_ONFIELD,0)==0
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) > 0
        && duel::get_field_group_count(d, tp, LOCATION_ONFIELD, 0) == 0
}
/// `s.spop(lv)`: the operation `function(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(lv: u32) -> LuaFn {
    lua_fn(move |d, a: Args| {
        let c = a.c(8);
        // c:AssumeProperty(ASSUME_LEVEL,lv)
        card::assume_property(d, c, ASSUME_LEVEL, lv as u64);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        // e1:SetValue(lv)
        effect::set_value(d, e1, lv as i64);
        // e1:SetReset(RESET_EVENT|(RESETS_STANDARD_DISABLE&~RESET_TOFIELD))
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    })
}
