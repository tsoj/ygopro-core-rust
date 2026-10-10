//! Flawless Perfection of the Tenyi
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 39730727;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // immune
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsNonEffectMonster))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_non_effect_monster(d, c))));
    // e2:SetValue(s.immfilter)
    effect::set_value_fn(d, e2, lua_fn(immfilter));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // draw
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e3, CATEGORY_DRAW);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.drcon)
    effect::set_condition(d, e3, Some(condition(drcon)));
    // e3:SetTarget(s.drtg)
    effect::set_target(d, e3, Some(cost_target(drtg)));
    // e3:SetOperation(s.drop)
    effect::set_operation(d, e3, Some(operation(drop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return (not c:IsType(TYPE_EFFECT) and c:IsFaceup()) or c:IsFacedown()
    (!card::is_type(d, c, TYPE_EFFECT) && card::is_faceup(d, c)) || card::is_facedown(d, c)
}
/// `s.immfilter(e,te)`
fn immfilter(d: &mut Duel, a: Args) -> bool {
    // return te:IsMonsterEffect()
    let te = a.e(1);
    effect::is_monster_effect(d, te)
}
/// `s.drfilter(c,tp)`
fn drfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return not c:IsSummonPlayer(tp) and c:IsType(TYPE_EFFECT) and c:IsFaceup()
    !card::is_summon_player(d, c, tp) && card::is_type(d, c, TYPE_EFFECT) && card::is_faceup(d, c)
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.drfilter,1,nil,tp)
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE,0,1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| drfilter(d, c, a.tp), 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,2) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 2);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(2)
    duel::set_target_param(d, 2);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
