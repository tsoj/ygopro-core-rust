//! Starry Knight Sky
//! 煌めく聖夜
use super::prelude::*;

pub const ID: u32 = 58406094;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_STARRY_KNIGHT], ..ScriptInfo::DEFAULT };

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
    // Additional Normal summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_EXTRA_SUMMON_COUNT)
    effect::set_code(d, e2, EFFECT_EXTRA_SUMMON_COUNT);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_HAND|LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_HAND | LOCATION_MZONE, 0);
    // e2:SetTarget(s.extg)
    effect::set_target(d, e2, Some(effect_card_fn(extg)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Draw if a Level 7 Dragon monster(s) returns to hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e3, CATEGORY_DRAW);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_TO_HAND)
    effect::set_code(d, e3, EVENT_TO_HAND);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(s.dcon)
    effect::set_condition(d, e3, Some(condition(dcon)));
    // e3:SetTarget(s.dtg)
    effect::set_target(d, e3, Some(cost_target(dtg)));
    // e3:SetOperation(s.dop)
    effect::set_operation(d, e3, Some(operation(dop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.ldlv7filter(c)`
fn ldlv7filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_DRAGON) and c:IsLevel(7)
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_DRAGON as u64) && card::is_level(d, c, &[7])
}
/// `s.extg(e,c)`
fn extg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return (c:IsSetCard(SET_STARRY_KNIGHT) and c:IsMonster()) or s.ldlv7filter(c)
    (card::is_set_card(d, c, &[SET_STARRY_KNIGHT as u16]) && card::is_monster(d, c)) || ldlv7filter(d, c)
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsPreviousLocation(LOCATION_MZONE) and s.ldlv7filter(c)
    card::is_controler(d, c, tp) && card::is_previous_location(d, c, LOCATION_MZONE) && ldlv7filter(d, c)
}
/// `s.dcon(e,tp,eg,ev,ep,re,r,rp)`
fn dcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and eg:IsExists(s.filter,1,nil,tp)
    if !duel::is_turn_player(d, a.tp) {
        return false;
    }
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| filter(d, c, a.tp), 1, NO_EX)
}
/// `s.dtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn dtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.dop(e,tp,eg,ep,ev,re,r,rp)`
fn dop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
