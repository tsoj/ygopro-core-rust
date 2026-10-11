//! Rocket Arrow Express
use super::prelude::*;

pub const ID: u32 = 79850798;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Special Summon procedure
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summoning condition
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e2, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Cannot activate cards or effects
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e3, EFFECT_CANNOT_ACTIVATE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Cannot Set cards
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetCode(EFFECT_CANNOT_MSET)
    effect::set_code(d, e4, EFFECT_CANNOT_MSET);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTargetRange(1,0)
    effect::set_target_range(d, e4, 1, 0);
    // e4:SetTarget(aux.TRUE)
    effect::set_target(d, e4, Some(aux_true()));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EFFECT_CANNOT_SSET)
    effect::set_code(d, e5, EFFECT_CANNOT_SSET);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // local e6=e4:Clone()
    let e6 = effect::clone(d, e4);
    // e6:SetCode(EFFECT_CANNOT_TURN_SET)
    effect::set_code(d, e6, EFFECT_CANNOT_TURN_SET);
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // local e7=e4:Clone()
    let e7 = effect::clone(d, e4);
    // e7:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e7, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e7:SetTarget(function(_,_,_,_,sumpos) return (sumpos&POS_FACEDOWN)>0 end)
    effect::set_target(d, e7, Some(lua_fn(e7target)));
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
    // Maintenance cost
    // local e8=Effect.CreateEffect(c)
    let e8 = effect::create_effect(d, c);
    // e8:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e8, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e8:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e8, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e8:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e8, EVENT_PHASE | PHASE_STANDBY);
    // e8:SetRange(LOCATION_MZONE)
    effect::set_range(d, e8, LOCATION_MZONE);
    // e8:SetCountLimit(1)
    effect::set_count_limit(d, e8, 1, 0, 0, 0);
    // e8:SetCondition(function(_,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e8, Some(lua_fn(|d, a| duel::is_turn_player(d, a.p(1)))));
    // e8:SetOperation(s.mtop)
    effect::set_operation(d, e8, Some(operation(mtop)));
    // c:RegisterEffect(e8)
    card::register_effect(d, c, e8, false);
    // Check/apply Battle Phase restriction
    // local e9=Effect.CreateEffect(c)
    let e9 = effect::create_effect(d, c);
    // e9:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e9, EFFECT_TYPE_SINGLE);
    // e9:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e9, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e9:SetCode(EFFECT_SPSUMMON_COST)
    effect::set_code(d, e9, EFFECT_SPSUMMON_COST);
    // e9:SetCost(function(_,_,tp) return Duel.GetActivityCount(tp,ACTIVITY_BATTLE_PHASE)==0 end)
    effect::set_cost(d, e9, Some(lua_fn(|d, a| duel::get_activity_count(d, a.p(2), &[ACTIVITY_BATTLE_PHASE]).first() == Some(&0))));
    // e9:SetOperation(s.spcostop)
    effect::set_operation(d, e9, Some(operation(spcostop)));
    // c:RegisterEffect(e9)
    card::register_effect(d, c, e9, false);
}
// function s.spcon(e,c)
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let c = match a.card(1) {
        Some(c) => c,
        None => return true,
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetFieldGroupCount(tp,LOCATION_ONFIELD,0)==0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    duel::get_field_group_count(d, tp, LOCATION_ONFIELD, 0) == 0 && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
}
// function(_,_,_,_,sumpos) return (sumpos&POS_FACEDOWN)>0 end
fn e7target(_d: &mut Duel, a: Args) -> bool {
    (a.int(4) & POS_FACEDOWN as i64) > 0
}
// function s.mtop(e,tp,eg,ep,ev,re,r,rp)
fn mtop(d: &mut Duel, a: &EffArgs) {
    // local hg=Duel.GetFieldGroup(tp,LOCATION_HAND,0)
    let hg = duel::get_field_group(d, a.tp, LOCATION_HAND, 0);
    // if #hg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if !hg.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
        // Duel.SendtoGrave(hg,REASON_COST)
        duel::send_to_grave(d, hg, REASON_COST, None, None);
    } else {
        // Duel.Destroy(e:GetHandler(),REASON_COST)
        let h = effect::get_handler(d, a.e).nil();
        duel::destroy(d, h, REASON_COST, None, None);
    }
}
// function s.spcostop(e,tp,eg,ep,ev,re,r,rp)
fn spcostop(d: &mut Duel, a: &EffArgs) {
    // Cannot conduct your Battle Phase
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_BP)
    effect::set_code(d, e1, EFFECT_CANNOT_BP);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
