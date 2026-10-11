//! Shien's Squire
use super::prelude::*;

pub const ID: u32 = 33883834;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SIX_SAMURAI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Your battling "Six Samurai" monster cannot be destroyed by battle
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    // e1:SetCondition(s.con)
    effect::set_condition(d, e1, Some(condition(con)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetOperation(s.op)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.con(e,tp,eg,ep,ev,re,r,rp)`
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let atk = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let dt = duel::get_attack_target(d);
    // return d and ((a:IsControler(tp) and a:IsSetCard(SET_SIX_SAMURAI)) or (d:IsControler(tp) and d:IsSetCard(SET_SIX_SAMURAI)))
    //   and Duel.GetFlagEffect(tp,id)==0
    let Some(dt) = dt else {
        return false;
    };
    let ac = atk.nil();
    let ok = (card::is_controler(d, ac, a.tp) && card::is_set_card(d, ac, &[SET_SIX_SAMURAI as u16]))
        || (card::is_controler(d, dt, a.tp) && card::is_set_card(d, dt, &[SET_SIX_SAMURAI as u16]));
    ok && duel::get_flag_effect(d, a.tp, ID) == 0
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsAbleToGraveAsCost() end
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, h);
    }
    // Duel.SendtoGrave(e:GetHandler(),REASON_COST)
    duel::send_to_grave(d, h, REASON_COST, None, None);
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_DAMAGE_CAL,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_DAMAGE_CAL, 0, 1, 0);
    true
}
/// `s.op(e,tp,eg,ep,ev,re,r,rp,chk)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let atk = duel::get_attacker(d).nil();
    // local d=Duel.GetAttackTarget()
    // if not a:IsRelateToBattle() or not d:IsRelateToBattle() then return end
    if !card::is_relate_to_battle(d, atk) || !card::is_relate_to_battle(d, duel::get_attack_target(d).nil()) {
        return;
    }
    // Cannot be destroyed by battle
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(3000)
    effect::set_description(d, e1, 3000);
    // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetOwnerPlayer(tp)
    effect::set_owner_player(d, e1, a.tp);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // if a:IsControler(tp) then a:RegisterEffect(e1) else d:RegisterEffect(e1) end
    if card::is_controler(d, atk, a.tp) {
        card::register_effect(d, atk, e1, false);
    } else {
        let dt = duel::get_attack_target(d).nil();
        card::register_effect(d, dt, e1, false);
    }
}
