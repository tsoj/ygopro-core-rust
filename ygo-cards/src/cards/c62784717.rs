//! Tour of Doom
use super::prelude::*;

pub const ID: u32 = 62784717;

pub const INFO: ScriptInfo = ScriptInfo { toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE as u32));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Toss a coin and apply the appropriate effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_COIN)
    effect::set_category(d, e2, CATEGORY_COIN);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(_,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetTarget(s.cointg)
    effect::set_target(d, e2, Some(cost_target(cointg)));
    // e2:SetOperation(s.coinop)
    effect::set_operation(d, e2, Some(operation(coinop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(_,tp) return Duel.IsTurnPlayer(1-tp) end`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.cointg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cointg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.tp, 1);
    false
}
/// `s.coinop(e,tp,eg,ep,ev,re,r,rp)`
fn coinop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local res=Duel.TossCoin(tp,1)
    let res = duel::toss_coin(d, a.tp, 1)[0];
    // if res==COIN_HEADS then
    if res == COIN_HEADS as i32 {
        // register_eff(1-tp)
        register_eff(d, a, 1 - a.tp);
    } else if res == COIN_TAILS as i32 {
        // register_eff(tp)
        register_eff(d, a, a.tp);
    }
}
/// `local function register_eff(player)`
fn register_eff(d: &mut Duel, a: &EffArgs, player: u8) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SUMMON);
    // if player==tp then
    if player == a.tp {
        // e1:SetReset(RESET_PHASE|PHASE_END,2)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // local cur_turn=Duel.GetTurnCount()
        let cur_turn = duel::get_turn_count(d, None);
        // e1:SetCondition(function()return Duel.GetTurnCount()~=cur_turn end)
        effect::set_condition(d, e1, Some(lua_fn(move |d: &mut Duel, _a: Args| duel::get_turn_count(d, None) != cur_turn)));
    } else {
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // e1:SetTargetRange(0,1)
        effect::set_target_range(d, e1, 0, 1);
    }
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_CANNOT_FLIP_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_FLIP_SUMMON);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
}
