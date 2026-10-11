//! Generaider Boss Loot
//! 王の報酬
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 1942635;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GENERAIDER], ..ScriptInfo::DEFAULT };

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
    // Your opponent's monsters cannot target "Generaider" Effect Monsters for attacks
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    // e2:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e2:SetCondition(s.atcon)
    effect::set_condition(d, e2, Some(condition(atcon)));
    // e2:SetValue(s.atlimit)
    effect::set_value_fn(d, e2, effect_card_fn(atlimit));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Your opponent draws 1 card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e3, CATEGORY_DRAW);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(s.drcon)
    effect::set_condition(d, e3, Some(condition(drcon)));
    // e3:SetTarget(s.drtg)
    effect::set_target(d, e3, Some(cost_target(drtg)));
    // e3:SetOperation(s.drop)
    effect::set_operation(d, e3, Some(operation(drop_op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.tknfilter(c)`
fn tknfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_TOKEN) or c:IsOriginalType(TYPE_TOKEN)
    card::is_type(d, c, TYPE_TOKEN) || card::is_original_type(d, c, TYPE_TOKEN)
}
/// `s.atcon(e)`
fn atcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.tknfilter,e:GetHandlerPlayer(),LOCATION_ONFIELD,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&tknfilter), p, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.atlimit(e,c)`
fn atlimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_EFFECT) and c:IsSetCard(SET_GENERAIDER)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT) && card::is_set_card(d, c, &[SET_GENERAIDER as u16])
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsReason(REASON_BATTLE) and c:IsPreviousSetCard(SET_GENERAIDER) and c:IsType(TYPE_EFFECT)
    card::is_reason(d, c, REASON_BATTLE) && card::is_previous_set_card(d, c, &[SET_GENERAIDER as u16]) && card::is_type(d, c, TYPE_EFFECT)
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &cfilter, 1, NO_EX)
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,1-tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, 1 - a.tp, 1);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
