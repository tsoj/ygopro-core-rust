//! Cyber Blader
//! サイバー・ブレイダー
use super::prelude::*;

pub const ID: u32 = 10248389;

pub const INFO: ScriptInfo = ScriptInfo { material_setcode: &[SET_CYBER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,false,false,97023549,11460577)
    fusion::add_proc_mix(d, c, false, false, vec![fusion::Mat::Code(97023549), fusion::Mat::Code(11460577)]);
    // Cannot be destroyed by battle
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetCondition(s.con)
    effect::set_condition(d, e1, Some(condition(con)));
    // e1:SetLabel(1)
    effect::set_label(d, e1, &[1]);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Increase ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_SET_ATTACK_FINAL)
    effect::set_code(d, e2, EFFECT_SET_ATTACK_FINAL);
    // e2:SetLabel(2)
    effect::set_label(d, e2, &[2]);
    // e2:SetCondition(s.con)
    effect::set_condition(d, e2, Some(condition(con)));
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // negate effects
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e3, EVENT_CHAIN_SOLVING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetLabel(3)
    effect::set_label(d, e3, &[3]);
    // e3:SetCondition(s.con)
    effect::set_condition(d, e3, Some(condition(con)));
    // e3:SetOperation(s.disop)
    effect::set_operation(d, e3, Some(operation(disop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // aux.DoubleSnareValidity(c,LOCATION_MZONE)
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}
/// `s.con(e)`
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),0,LOCATION_MZONE)==e:GetLabel()
    let p = effect::get_handler_player(d, a.e);
    duel::get_field_group_count(d, p, 0, LOCATION_MZONE) as i64 == effect::get_label(d, a.e)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetAttack()*2
    card::get_attack(d, c) as i64 * 2
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // if rp~=tp then
    if a.rp != a.tp {
        // Duel.NegateEffect(ev)
        duel::negate_effect(d, a.ev as u8);
    }
}
