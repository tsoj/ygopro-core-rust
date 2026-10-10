//! The Winged Dragon of Ra
use super::prelude::*;

pub const ID: u32 = 10000010;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Normal Summon with 3 Tributes
    // aux.AddNormalSummonProcedure(c,true,false,3,3)
    let _e1 = aux::add_normal_summon_procedure(d, c, true, false, Some(3), Some(3), None, None, None, None);
    // aux.AddNormalSetProcedure(c)
    let _e2 = aux::add_normal_set_procedure(d, c, false, false, None, None, None, None, None, None);
    // Cannot be Special Summoned
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    // e0:SetValue(s.spval)
    effect::set_value_fn(d, e0, lua_fn(spval));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // This card's Normal Summon cannot be negated
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_CANNOT_DISABLE_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_DISABLE_SUMMON);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Other cards and effects cannot be activated when Normal Summoned
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetOperation(function(e) Duel.SetChainLimitTillChainEnd(s.genchainlm(e:GetHandler())) end)
    effect::set_operation(d, e2, Some(operation(chainlim_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // One Turn Kill
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetCost(s.atkcost)
    effect::set_cost(d, e3, Some(cost_target(atkcost)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Destroy 1 monster on the field
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCost(Cost.PayLP(1000))
    effect::set_cost(d, e4, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    // e4:SetTarget(s.destg)
    effect::set_target(d, e4, Some(cost_target(destg)));
    // e4:SetOperation(s.desop)
    effect::set_operation(d, e4, Some(operation(desop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.spval(e,se,sp,st)`
fn spval(d: &mut Duel, a: Args) -> bool {
    // return st==SUMMON_TYPE_SPECIAL+SUMMON_WITH_MONSTER_REBORN and Duel.IsPlayerAffectedByEffect(sp,41044418) and e:GetHandler():IsControler(sp)
    let e = a.e(0);
    let sp = a.p(2);
    let st = a.int(3);
    if st != (SUMMON_TYPE_SPECIAL + SUMMON_WITH_MONSTER_REBORN) as i64 {
        return false;
    }
    if duel::is_player_affected_by_effect(d, sp, 41044418).is_none() {
        return false;
    }
    let h = effect::get_handler(d, e).nil();
    card::is_controler(d, h, sp)
}
/// `s.genchainlm(c)`: the chain limit `function(e,rp,tp) return e:GetHandler()==c end`.
fn genchainlm(c: CardId) -> LuaFn {
    lua_fn(move |d, a: Args| {
        let h = effect::get_handler(d, a.e(0)).nil();
        h == c
    })
}
/// `function(e) Duel.SetChainLimitTillChainEnd(s.genchainlm(e:GetHandler())) end`
fn chainlim_op(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    duel::set_chain_limit_till_chain_end(d, genchainlm(h));
}
/// `s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local lp_cost=Duel.GetLP(tp)-100
    let lp_cost = duel::get_lp(d, tp) as i64 - 100;
    // if chk==0 then return lp_cost>0 and Duel.CheckLPCost(tp,lp_cost) end
    if a.chk == Some(0) {
        return lp_cost > 0 && duel::check_lp_cost(d, tp, lp_cost as u32);
    }
    // e:SetLabel(lp_cost)
    effect::set_label(d, a.e, &[lp_cost]);
    // Duel.PayLPCost(tp,lp_cost)
    duel::pay_lp_cost(d, tp, lp_cost as u32);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // Increase ATK/DEF
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetValue(e:GetLabel())
        let lbl = effect::get_label(d, a.e);
        effect::set_value(d, e1, lbl);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_UPDATE_DEFENSE)
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, None, tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
