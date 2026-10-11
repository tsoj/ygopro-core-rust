//! Ghostrick Mansion
//! ゴーストリック・ハウス
use super::prelude::*;

pub const ID: u32 = 99795159;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GHOSTRICK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Limit attacks
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetValue(aux.TargetBoolFunction(Card.IsFacedown))
    effect::set_value_fn(d, e2, effect_card_fn(|d, _e, c| card::is_facedown(d, c)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Direct attack
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e3, EFFECT_DIRECT_ATTACK);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    // e3:SetTarget(s.dirtg)
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| dirtg(d, c))));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Change damage
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetCode(EFFECT_CHANGE_DAMAGE)
    effect::set_code(d, e4, EFFECT_CHANGE_DAMAGE);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetTargetRange(1,1)
    effect::set_target_range(d, e4, 1, 1);
    // e4:SetValue(s.val)
    effect::set_value_fn(d, e4, lua_fn(|d, a| val(d, a)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e5, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e5:SetValue(s.damval)
    effect::set_value_fn(d, e5, lua_fn(|d, a| damval(d, a)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
// function s.dirtg(e,c)
fn dirtg(d: &mut Duel, c: CardId) -> bool {
    // return not Duel.IsExistingMatchingCard(Card.IsFaceup,c:GetControler(),0,LOCATION_MZONE,1,nil)
    let p = card::get_controler(d, c);
    !duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x)), p, 0, LOCATION_MZONE, 1, NO_EX)
}
// function s.val(e,re,dam,r,rp,rc)
fn val(_d: &mut Duel, a: Args<'_>) -> i64 {
    let dam = a.int(2);
    // if r&REASON_EFFECT~=0 then
    if (a.int(3) as u32 & REASON_EFFECT) != 0 {
        // return math.floor(dam/2)
        dam.div_euclid(2)
    } else {
        // return dam
        dam
    }
}
// function s.damval(e,rc)
fn damval(d: &mut Duel, a: Args<'_>) -> i64 {
    // if rc:IsSetCard(SET_GHOSTRICK) then return -1 end
    let rc = a.c(1);
    if card::is_set_card(d, rc, &[SET_GHOSTRICK as u16]) {
        return -1;
    }
    // return HALF_DAMAGE
    HALF_DAMAGE as i64
}
