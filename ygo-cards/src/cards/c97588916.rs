//! Deskbot 007
use super::prelude::*;

pub const ID: u32 = 97588916;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DESKBOT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // splimit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetCondition(s.splimcon)
    effect::set_condition(d, e2, Some(lua_fn(splimcon)));
    // e2:SetTarget(s.splimit)
    effect::set_target(d, e2, Some(lua_fn(splimit)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetValue(s.value)
    effect::set_value_fn(d, e3, effect_card_value(value));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // pierce
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e5, EFFECT_PIERCE);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e6, 0, LOCATION_MZONE);
    // e6:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e6, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e6:SetValue(s.atlimit)
    effect::set_value_fn(d, e6, effect_card_fn(atlimit));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.splimcon(e)`
fn splimcon(d: &mut Duel, a: Args) -> bool {
    // return not e:GetHandler():IsForbidden()
    let h = effect::get_handler(d, a.e(0)).nil();
    !card::is_forbidden(d, h)
}
/// `s.splimit(e,c,tp,sumtp,sumpos)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    // return not c:IsSetCard(SET_DESKBOT) and (sumtp&SUMMON_TYPE_PENDULUM)==SUMMON_TYPE_PENDULUM
    !card::is_set_card(d, c, &[SET_DESKBOT as u16]) && (a.int(3) as u64 & SUMMON_TYPE_PENDULUM as u64) == SUMMON_TYPE_PENDULUM as u64
}
/// `s.value(e,c)`
fn value(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(Card.IsSetCard,c:GetControler(),LOCATION_GRAVE,0,nil,SET_DESKBOT)*500
    let p = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&|d, x| card::is_set_card(d, x, &[SET_DESKBOT as u16])), p, LOCATION_GRAVE, 0, NO_EX) as i64 * 500
}
/// `s.atlimit(e,c)`
fn atlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_DESKBOT) and c~=e:GetHandler()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DESKBOT as u16]) && c != effect::get_handler(d, e).nil()
}
