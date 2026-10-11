//! Attack of the Cornered Rat
//! 窮鼠の進撃
use super::prelude::*;

pub const ID: u32 = 84389640;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atkup
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e2, TIMING_DAMAGE_STEP, None);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetCost(s.cost)
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local phase=Duel.GetCurrentPhase()
    let phase = duel::get_current_phase(d);
    // if phase~=PHASE_DAMAGE or Duel.IsDamageCalculated() then return false end
    if phase != PHASE_DAMAGE || duel::is_damage_calculated(d) {
        return false;
    }
    // local a=Duel.GetAttacker()
    let mut at = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let mut df = duel::get_attack_target(d);
    // if not d then return false end
    let Some(dd) = df else { return false };
    // if d:IsControler(tp) then a,d=d,a end
    if card::is_controler(d, dd, a.tp) {
        std::mem::swap(&mut at, &mut df);
    }
    // e:SetLabelObject(d)
    effect::set_label_object(d, a.e, Value::card(df));
    // return a:IsFaceup() and a:IsLevelBelow(3) and a:IsType(TYPE_NORMAL) and a:IsRelateToBattle()
    //   and d:IsFaceup() and d:IsRelateToBattle()
    let at = at.nil();
    let df = df.nil();
    card::is_faceup(d, at)
        && card::is_level_below(d, at, 3)
        && card::is_type(d, at, TYPE_NORMAL)
        && card::is_relate_to_battle(d, at)
        && card::is_faceup(d, df)
        && card::is_relate_to_battle(d, df)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then
    if a.chk == Some(0) {
        // for _,eff in pairs({Duel.IsPlayerAffectedByEffect(tp,EFFECT_LPCOST_CHANGE)}) do
        if let Some(eff) = duel::is_player_affected_by_effect(d, a.tp, EFFECT_LPCOST_CHANGE) {
            // local val=eff:GetValue()
            let val = effect::get_value(d, eff);
            // if (type(val)=='integer' and val==0) or (type(val)=='function' and (val(eff,e,tp,100)~=100)) then
            let stop = match &val {
                Value::Int(v) => *v == 0,
                Value::Fn(f) => f.call(d, &[Value::Effect(eff), Value::Effect(a.e), Value::Int(a.tp as i64), Value::Int(100)]).int() != 100,
                _ => false,
            };
            // return false
            if stop {
                return false;
            }
        }
        // return e:GetHandler():GetFlagEffect(id)==0 and Duel.CheckLPCost(tp,100)
        //   and e:GetLabelObject():IsAttackAbove(100)
        let lo = effect::get_label_object(d, a.e).as_card().nil();
        return card::get_flag_effect(d, h, ID) == 0 && duel::check_lp_cost(d, a.tp, 100) && card::is_attack_above(d, lo, 100);
    }
    // local lp=Duel.GetLP(tp)
    let lp = duel::get_lp(d, a.tp);
    // local atk=e:GetLabelObject():GetAttack()
    let lo = effect::get_label_object(d, a.e).as_card().nil();
    let atk = card::get_attack(d, lo);
    // local t={}
    // for i=1,math.floor(math.min(lp,atk)/100) do t[i]=i*100 end
    let n = lp.min(atk).div_euclid(100);
    let t: Vec<u64> = (1..=n).map(|i| (i * 100) as u64).collect();
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
    // local pay=Duel.AnnounceNumber(tp,table.unpack(t))
    let (pay, _) = duel::announce_number(d, a.tp, &t).nil();
    // Duel.PayLPCost(tp,pay)
    duel::pay_lp_cost(d, a.tp, pay as u32);
    // e:SetLabel(-pay)
    effect::set_label(d, a.e, &[-(pay as i64)]);
    // e:GetHandler():RegisterFlagEffect(id,RESET_PHASE|PHASE_DAMAGE,0,1)
    card::register_flag_effect(d, h, ID, RESET_PHASE | PHASE_DAMAGE, 0, 1, 0, 0);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card();
    // if chkc then return chkc==tc end
    if let Some(chkc) = a.chkc() {
        return Some(chkc) == tc;
    }
    // if chk==0 then return tc:IsCanBeEffectTarget(e) end
    if a.chk == Some(0) {
        return card::is_can_be_effect_target(d, tc.nil(), Some(a.e));
    }
    // Duel.SetTargetCard(tc)
    duel::set_target_card(d, tc.nil());
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local bc=Duel.GetFirstTarget()
    let bc = duel::first_target(d);
    let h = effect::get_handler(d, a.e).nil();
    // if not e:GetHandler():IsRelateToEffect(e) or not bc or not bc:IsRelateToEffect(e) or not bc:IsControler(1-tp) then return end
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    let Some(bc) = bc else { return };
    if !card::is_relate_to_effect(d, bc, a.e) || !card::is_controler(d, bc, 1 - a.tp) {
        return;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetOwnerPlayer(tp)
    effect::set_owner_player(d, e1, a.tp);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // e1:SetValue(e:GetLabel())
    let label = effect::get_label(d, a.e);
    effect::set_value(d, e1, label);
    // bc:RegisterEffect(e1)
    card::register_effect(d, bc, e1, false);
}
