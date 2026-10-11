//! Donyoribo @Ignister
use super::prelude::*;

pub const ID: u32 = 14146794;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_IGNISTER, SET_AI],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // battle Damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e1, Some(cost::self_discard()));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // double damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.damcon)
    effect::set_condition(d, e2, Some(condition(damcon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetOperation(s.damop)
    effect::set_operation(d, e2, Some(operation(damop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=Duel.GetAttackTarget()
    let tc = duel::get_attack_target(d);
    // return tc and tc:IsControler(tp) and tc:IsSetCard(SET_IGNISTER) and Duel.GetBattleDamage(tp)>0
    let Some(tc) = tc else {
        return false;
    };
    card::is_controler(d, tc, a.tp)
        && card::is_set_card(d, tc, &[SET_IGNISTER as u16])
        && duel::get_battle_damage(d, a.tp) > 0
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e1, EFFECT_AVOID_BATTLE_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    let (tp, ev) = (a.tp, a.ev as u8);
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if not ((re:IsMonsterEffect() and rc:IsSetCard(SET_IGNISTER)) or (re:IsSpellTrapEffect() and rc:IsSetCard(SET_AI))) then return false end
    let cond = (effect::is_monster_effect(d, re) && card::is_set_card(d, rc, &[SET_IGNISTER as u16]))
        || (effect::is_spell_trap_effect(d, re) && card::is_set_card(d, rc, &[SET_AI as u16]));
    if !cond {
        return false;
    }
    // local p=1-tp
    let p = 1 - tp;
    // local e1=Duel.IsPlayerAffectedByEffect(p,EFFECT_REVERSE_DAMAGE)
    let e1 = duel::is_player_affected_by_effect(d, p, EFFECT_REVERSE_DAMAGE).is_some();
    // local e2=Duel.IsPlayerAffectedByEffect(p,EFFECT_REVERSE_RECOVER)
    let e2 = duel::is_player_affected_by_effect(d, p, EFFECT_REVERSE_RECOVER).is_some();
    // local rd=e1 and not e2
    let rd = e1 && !e2;
    // local rr=not e1 and e2
    let rr = !e1 && e2;
    // local ex,cg,ct,cp,cv=Duel.GetOperationInfo(ev,CATEGORY_DAMAGE)
    let info = duel::get_operation_info(d, ev as u32, CATEGORY_DAMAGE);
    // if ex and (cp==p or p==PLAYER_ALL) and not rd and not Duel.IsPlayerAffectedByEffect(p,EFFECT_NO_EFFECT_DAMAGE) then
    if let Some(op) = &info {
        if (op.op_player == p || p as u32 == PLAYER_ALL)
            && !rd
            && duel::is_player_affected_by_effect(d, p, EFFECT_NO_EFFECT_DAMAGE).is_none()
        {
            return true;
        }
    }
    // ex,cg,ct,cp,cv=Duel.GetOperationInfo(ev,CATEGORY_RECOVER)
    let info = duel::get_operation_info(d, ev as u32, CATEGORY_RECOVER);
    // if ex and (p==tp or p==PLAYER_ALL) and rr and not Duel.IsPlayerAffectedByEffect(p,EFFECT_NO_EFFECT_DAMAGE) then
    if info.is_some()
        && (p == tp || p as u32 == PLAYER_ALL)
        && rr
        && duel::is_player_affected_by_effect(d, p, EFFECT_NO_EFFECT_DAMAGE).is_none()
    {
        return true;
    }
    // return false
    false
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // local cid=Duel.GetChainInfo(ev,CHAININFO_CHAIN_ID)
    let cid = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_CHAIN_ID])[0].int();
    // local e2=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e2 = effect::create_effect(d, h);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CHANGE_DAMAGE)
    effect::set_code(d, e2, EFFECT_CHANGE_DAMAGE);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetLabel(cid)
    effect::set_label(d, e2, &[cid]);
    // e2:SetValue(s.damval)
    effect::set_value_fn(d, e2, lua_fn(damval));
    // e2:SetReset(RESET_CHAIN)
    effect::set_reset(d, e2, RESET_CHAIN, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
}
/// `s.damval(e,re,val,r,rp,rc)`
fn damval(d: &mut Duel, a: Args) -> i64 {
    let e = a.e(0);
    let val = a.int(2);
    let r = a.int(3) as u64;
    // local cc=Duel.GetCurrentChain()
    let cc = duel::get_current_chain(d, false);
    // if cc==0 or r&REASON_EFFECT==0 then return val end
    if cc == 0 || (r & REASON_EFFECT as u64) == 0 {
        return val;
    }
    // local cid=Duel.GetChainInfo(0,CHAININFO_CHAIN_ID)
    let cid = duel::get_chain_info(d, 0, &[CHAININFO_CHAIN_ID])[0].int();
    // if cid~=e:GetLabel() then return val end
    if cid != effect::get_label(d, e) {
        return val;
    }
    // return val*2
    val * 2
}
