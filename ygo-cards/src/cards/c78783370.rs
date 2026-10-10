//! Barrel Behind the Door
use super::prelude::*;

pub const ID: u32 = 78783370;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // reflect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // if re:IsSpellTrapEffect() and not re:IsHasType(EFFECT_TYPE_ACTIVATE) then return false end
    let re = a.re.nil();
    if effect::is_spell_trap_effect(d, re) && !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        return false;
    }
    // local ex,cg,ct,cp,cv=Duel.GetOperationInfo(ev,CATEGORY_DAMAGE)
    // if ex and (cp==tp or cp==PLAYER_ALL) then return true end
    if let Some(info) = duel::get_operation_info(d, a.ev, CATEGORY_DAMAGE) {
        if info.op_player == a.tp || info.op_player as u32 == PLAYER_ALL {
            return true;
        }
    }
    // ex,cg,ct,cp,cv=Duel.GetOperationInfo(ev,CATEGORY_RECOVER)
    // return ex and (cp==tp or cp==PLAYER_ALL) and Duel.IsPlayerAffectedByEffect(tp,EFFECT_REVERSE_RECOVER)
    match duel::get_operation_info(d, a.ev, CATEGORY_RECOVER) {
        Some(info) => (info.op_player == a.tp || info.op_player as u32 == PLAYER_ALL) && duel::is_player_affected_by_effect(d, a.tp, EFFECT_REVERSE_RECOVER).is_some(),
        None => false,
    }
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local cid=Duel.GetChainInfo(ev,CHAININFO_CHAIN_ID)
    let cid = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_CHAIN_ID]).first().map_or(0, |v| v.int());
    // local e1=Effect.CreateEffect(e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_REFLECT_DAMAGE)
    effect::set_code(d, e1, EFFECT_REFLECT_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetLabel(cid)
    effect::set_label(d, e1, &[cid]);
    // e1:SetValue(s.refcon)
    effect::set_value_fn(d, e1, lua_fn(refcon));
    // e1:SetReset(RESET_CHAIN)
    effect::set_reset(d, e1, RESET_CHAIN, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.refcon(e,re,val,r,rp,rc)`
fn refcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let r = a.int(3) as u32;
    // local cc=Duel.GetCurrentChain()
    // if cc==0 or (r&REASON_EFFECT)==0 then return end
    let cc = duel::get_current_chain(d, false);
    if cc == 0 || (r & REASON_EFFECT) == 0 {
        return false;
    }
    // local cid=Duel.GetChainInfo(0,CHAININFO_CHAIN_ID)
    let cid = duel::get_chain_info(d, 0, &[CHAININFO_CHAIN_ID]).first().map_or(0, |v| v.int());
    // return cid==e:GetLabel()
    cid == effect::get_label(d, e)
}
