//! Time-Tearing Morganite
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 19403423;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply effects for the rest of the Duel
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Prevent activations when you Normal Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCost(s.gycost)
    effect::set_cost(d, e2, Some(cost_target(gycost)));
    // e2:SetTarget(s.gytg)
    effect::set_target(d, e2, Some(cost_target(gytg)));
    // e2:SetOperation(s.gyop)
    effect::set_operation(d, e2, Some(operation(gyop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFlagEffect(tp,id)==0 end
    if a.chk == Some(0) {
        return duel::get_flag_effect(d, a.tp, ID) == 0;
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.RegisterFlagEffect(tp,id,0,0,1)
    duel::register_flag_effect(d, tp, ID, 0, 0, 1, 0);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // --Cannot activate monsters effects from the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(s.aclimit)
    effect::set_value_fn(d, e1, lua_fn(aclimit));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // --Draw 2 cards during the Draw Phase
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_DRAW_COUNT)
    effect::set_code(d, e2, EFFECT_DRAW_COUNT);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetValue(2)
    effect::set_value(d, e2, 2);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // --Can Normal Summon/Set twice per turn
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EFFECT_SET_SUMMON_COUNT_LIMIT)
    effect::set_code(d, e3, EFFECT_SET_SUMMON_COUNT_LIMIT);
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // e3:SetValue(2)
    effect::set_value(d, e3, 2);
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, tp);
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // local rc=re:GetHandler()
    let re = a.e(1);
    let rc = effect::get_handler(d, re);
    // return rc and rc:IsLocation(LOCATION_HAND) and re:IsMonsterEffect()
    match rc {
        Some(rc) => card::is_location(d, rc, LOCATION_HAND) && effect::is_monster_effect(d, re),
        None => false,
    }
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(id) and c:IsDiscardable()
    card::is_code(d, c, &[ID]) && card::is_discardable(d, c, None, None)
}
/// `s.gycost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn gycost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemoveAsCost() and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None) && duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Remove(c,POS_FACEUP,REASON_COST)
    duel::remove(d, c, POS_FACEUP as u8, REASON_COST, None, None);
    // Duel.DiscardHand(tp,s.cfilter,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, tp, Some(&cfilter), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.gytg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn gytg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFlagEffect(tp,id+1)==0 end
    if a.chk == Some(0) {
        return duel::get_flag_effect(d, a.tp, ID + 1) == 0;
    }
    true
}
/// `s.gyop(e,tp,eg,ep,ev,re,r,rp)`
fn gyop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
    // --Prevent activations when you Normal Summon
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCondition(s.limcon)
    effect::set_condition(d, e1, Some(condition(limcon)));
    // e1:SetOperation(s.limop)
    effect::set_operation(d, e1, Some(operation(limop)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.limcon(e,tp,eg,ep,ev,re,r,rp)`
fn limcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(Card.IsSummonPlayer,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, tp), 1, NO_EX)
}
/// `s.limop(e,tp,eg,ep,ev,re,r,rp)`
fn limop(d: &mut Duel, _a: &EffArgs) {
    // Duel.SetChainLimitTillChainEnd(s.chainlm)
    duel::set_chain_limit_till_chain_end(d, lua_fn(chainlm));
}
/// `s.chainlm(e,rp,tp)`
fn chainlm(d: &mut Duel, a: Args) -> bool {
    // return tp==rp or not e:IsMonsterEffect()
    let rp = a.int(1);
    let tp = a.int(2);
    if tp == rp {
        return true;
    }
    let e = a.e(0);
    !effect::is_monster_effect(d, e)
}
