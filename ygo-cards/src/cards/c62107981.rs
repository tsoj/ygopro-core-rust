//! Elemental HERO Knospe
use super::prelude::*;

pub const ID: u32 = 62107981;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ELEMENTAL_HERO],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // direct attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    // e1:SetCondition(s.atcon)
    effect::set_condition(d, e1, Some(condition(atcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atk/def
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e2, EVENT_BATTLE_DAMAGE);
    // e2:SetCondition(s.adcon)
    effect::set_condition(d, e2, Some(condition(adcon)));
    // e2:SetOperation(s.adop)
    effect::set_operation(d, e2, Some(operation(adop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_CANNOT_BE_BATTLE_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_BE_BATTLE_TARGET);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.atcon)
    effect::set_condition(d, e3, Some(condition(atcon)));
    // e3:SetValue(aux.imval1)
    effect::set_value_fn(d, e3, aux::imval1());
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.adcon(e,tp,eg,ep,ev,re,r,rp)`
fn adcon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return ep~=tp
    a.ep != a.tp
}
/// `s.adop(e,tp,eg,ep,ev,re,r,rp)`
fn adop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() and c:GetDefense()>=100 then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && card::get_defense(d, c) >= 100 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetProperty(EFFECT_FLAG_COPY_INHERIT)
        effect::set_property(d, e1, EFFECT_FLAG_COPY_INHERIT, 0);
        // e1:SetValue(100)
        effect::set_value(d, e1, 100);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_UPDATE_DEFENSE)
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        // e2:SetValue(-100)
        effect::set_value(d, e2, -100);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_ELEMENTAL_HERO)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16])
}
/// `s.atcon(e)`
fn atcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.filter,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,e:GetHandler())
    let p = effect::get_handler_player(d, a.e);
    let h = effect::get_handler(d, a.e).nil();
    duel::is_existing_matching_card(d, Some(&filter), p, LOCATION_MZONE, 0, 1, h.into())
}
