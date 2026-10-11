//! Damage Eater
use super::prelude::*;

pub const ID: u32 = 60741115;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e1, Some(cost::self_banish()));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // if rp==tp or Duel.IsTurnPlayer(tp) then return false end
    if a.rp == a.tp || duel::is_turn_player(d, a.tp) {
        return false;
    }
    // local ex,cg,ct,cp,cv=Duel.GetOperationInfo(ev,CATEGORY_DAMAGE)
    // if ex then return true end
    if duel::get_operation_info(d, a.ev, CATEGORY_DAMAGE).is_some() {
        return true;
    }
    // ex,cg,ct,cp,cv=Duel.GetOperationInfo(ev,CATEGORY_RECOVER)
    // if not ex then return false end
    let Some(info) = duel::get_operation_info(d, a.ev, CATEGORY_RECOVER) else {
        return false;
    };
    // if cp~=PLAYER_ALL then return Duel.IsPlayerAffectedByEffect(cp,EFFECT_REVERSE_RECOVER)
    if info.op_player as u32 != PLAYER_ALL {
        return duel::is_player_affected_by_effect(d, info.op_player, EFFECT_REVERSE_RECOVER).is_some();
    }
    // else return Duel.IsPlayerAffectedByEffect(0,EFFECT_REVERSE_RECOVER) or Duel.IsPlayerAffectedByEffect(1,EFFECT_REVERSE_RECOVER) end
    duel::is_player_affected_by_effect(d, 0, EFFECT_REVERSE_RECOVER).is_some() || duel::is_player_affected_by_effect(d, 1, EFFECT_REVERSE_RECOVER).is_some()
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local cid=Duel.GetChainInfo(ev,CHAININFO_CHAIN_ID)
    let cid = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_CHAIN_ID])[0].int();
    // damage conversion
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_REVERSE_DAMAGE)
    effect::set_code(d, e1, EFFECT_REVERSE_DAMAGE);
    // e1:SetTargetRange(1,1)
    effect::set_target_range(d, e1, 1, 1);
    // e1:SetValue(s.rev)
    effect::set_value_fn(d, e1, lua_fn(rev));
    // e1:SetReset(RESET_CHAIN)
    effect::set_reset(d, e1, RESET_CHAIN, 1);
    // e1:SetLabel(cid)
    effect::set_label(d, e1, &[cid]);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
// function s.rev(e,re,r,rp,rc)
fn rev(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // local cc=Duel.GetCurrentChain()
    let cc = duel::get_current_chain(d, false);
    // if cc==0 or (r&REASON_EFFECT)==0 then return false end
    if cc == 0 || (a.int(2) as u32 & REASON_EFFECT) == 0 {
        return false;
    }
    // local cid=Duel.GetChainInfo(0,CHAININFO_CHAIN_ID)
    let cid = duel::get_chain_info(d, 0, &[CHAININFO_CHAIN_ID])[0].int();
    // return cid==e:GetLabel()
    cid == effect::get_label(d, e)
}
