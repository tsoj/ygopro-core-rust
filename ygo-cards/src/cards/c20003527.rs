//! Cloudian - Nimbusman
use super::prelude::*;

pub const ID: u32 = 20003527;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_FOG], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // battle indestructable
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // selfdes
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_SELF_DESTROY)
    effect::set_code(d, e2, EFFECT_SELF_DESTROY);
    // e2:SetCondition(s.sdcon)
    effect::set_condition(d, e2, Some(condition(sdcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // summon proc
    // local e3=aux.AddNormalSummonProcedure(c,true,true,1,99,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),s.cfilter)
    // (s.cfilter is not defined in the script, so it is nil)
    let _e3 = aux::add_normal_summon_procedure(d, c, true, true, Some(1), Some(99), Some(SUMMON_TYPE_TRIBUTE), Some(aux::stringid(ID, 0)), None, None);
    // tribute check
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e4, EFFECT_MATERIAL_CHECK);
    // e4:SetValue(s.valcheck)
    effect::set_value_fn(d, e4, lua_fn(valcheck));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // add counter
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e5, CATEGORY_COUNTER);
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e5:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e5, EVENT_SUMMON_SUCCESS);
    // e5:SetCondition(s.addcon)
    effect::set_condition(d, e5, Some(condition(addcon)));
    // e5:SetOperation(s.addc)
    effect::set_operation(d, e5, Some(operation(addc)));
    // e5:SetLabelObject(e4)
    effect::set_label_object(d, e5, Value::Effect(e4));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // atk
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE);
    // e6:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e6, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e6, EFFECT_UPDATE_ATTACK);
    // e6:SetValue(s.atkval)
    effect::set_value_fn(d, e6, effect_card_value(atkval));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.sdcon(e)`
fn sdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPosition(POS_FACEUP_DEFENSE)
    let c = effect::get_handler(d, a.e).nil();
    card::is_position(d, c, POS_FACEUP_DEFENSE)
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=c:GetMaterial()
    let g = card::get_material(d, a.c(1));
    // e:SetLabel(g:FilterCount(Card.IsAttribute,nil,ATTRIBUTE_WATER))
    let n = group::filter_count(d, &g, &|d, x| card::is_attribute(d, x, ATTRIBUTE_WATER), NO_EX);
    effect::set_label(d, e, &[n as i64]);
}
/// `s.addcon(e,tp,eg,ep,ev,re,r,rp)`
fn addcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsTributeSummoned()
    let c = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, c)
}
/// `s.addc(e,tp,eg,ep,ev,re,r,rp)`
fn addc(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) then
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        // e:GetHandler():AddCounter(COUNTER_NEED_ENABLE+COUNTER_FOG,e:GetLabelObject():GetLabel())
        let lbl = effect::get_label_object(d, a.e).as_effect().nil();
        let n = effect::get_label(d, lbl);
        card::add_counter(d, c, (COUNTER_NEED_ENABLE + COUNTER_FOG) as u16, n as u16, false);
    }
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, _c: CardId) -> i64 {
    // return Duel.GetCounter(0,1,1,COUNTER_FOG)*500
    duel::get_counter(d, 0, 1, 1, COUNTER_FOG as u16) as i64 * 500
}
