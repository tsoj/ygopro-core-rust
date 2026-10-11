//! Trap of Board Eraser
use super::prelude::*;

pub const ID: u32 = 3055837;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // reflect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCategory(CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_HANDES);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    // if re:IsSpellTrapEffect() and not re:IsHasType(EFFECT_TYPE_ACTIVATE) then return false end
    if effect::is_spell_trap_effect(d, re) && !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
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
    let cp = info.op_player;
    // if cp~=PLAYER_ALL then return Duel.IsPlayerAffectedByEffect(cp,EFFECT_REVERSE_RECOVER)
    if cp as u32 != PLAYER_ALL {
        return duel::is_player_affected_by_effect(d, cp, EFFECT_REVERSE_RECOVER).is_some();
    }
    // else return Duel.IsPlayerAffectedByEffect(0,EFFECT_REVERSE_RECOVER)
    //     or Duel.IsPlayerAffectedByEffect(1,EFFECT_REVERSE_RECOVER) end
    duel::is_player_affected_by_effect(d, 0, EFFECT_REVERSE_RECOVER).is_some() || duel::is_player_affected_by_effect(d, 1, EFFECT_REVERSE_RECOVER).is_some()
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,1-tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - a.tp, 1);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // Duel.DiscardHand(1-tp,nil,1,1,REASON_EFFECT|REASON_DISCARD)
    duel::discard_hand(d, 1 - tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
    // local cid=Duel.GetChainInfo(ev,CHAININFO_CHAIN_ID)
    let cid = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_CHAIN_ID])[0].int();
    // local e1=Effect.CreateEffect(e:GetHandler())
    let c = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CHANGE_DAMAGE)
    effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
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
    duel::register_effect(d, e1, tp);
}
/// `s.refcon(e,re,val,r,rp,rc)`
fn refcon(d: &mut Duel, a: Args) -> i64 {
    let r = a.int(3) as u32;
    // local cc=Duel.GetCurrentChain()
    // if cc==0 or (r&REASON_EFFECT)==0 then return end
    if duel::get_current_chain(d, false) == 0 || (r & REASON_EFFECT) == 0 {
        return 0;
    }
    // local cid=Duel.GetChainInfo(0,CHAININFO_CHAIN_ID)
    let cid = duel::get_chain_info(d, 0, &[CHAININFO_CHAIN_ID])[0].int();
    // if cid==e:GetLabel() then return 0 else return val end
    if cid == effect::get_label(d, a.e(0)) {
        0
    } else {
        a.int(2)
    }
}
