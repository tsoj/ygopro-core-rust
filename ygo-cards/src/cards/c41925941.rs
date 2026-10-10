//! Bark of Dark Ruler
use super::prelude::*;

pub const ID: u32 = 41925941;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atkdown
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local phase=Duel.GetCurrentPhase()
    let phase = duel::get_current_phase(d);
    // if phase~=PHASE_DAMAGE or Duel.IsDamageCalculated() then return false end
    if phase != PHASE_DAMAGE || duel::is_damage_calculated(d) {
        return false;
    }
    // local a=Duel.GetAttacker()
    let atk = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let dd = duel::get_attack_target(d);
    // if a:IsControler(tp) then
    if card::is_controler(d, atk.nil(), tp) {
        // e:SetLabelObject(d)
        effect::set_label_object(d, a.e, dd.map_or(Value::Nil, Value::Card));
        let atk = atk.nil();
        // return a:IsFaceup() and a:IsRace(RACE_FIEND) and a:IsRelateToBattle()
        //     and d and d:IsFaceup() and d:IsRelateToBattle()
        return match dd {
            Some(x) => {
                card::is_faceup(d, atk)
                    && card::is_race(d, atk, RACE_FIEND)
                    && card::is_relate_to_battle(d, atk)
                    && card::is_faceup(d, x)
                    && card::is_relate_to_battle(d, x)
            }
            None => false,
        };
    }
    // elseif d and d:IsControler(tp) then
    let dd_mine = match dd {
        Some(x) => card::is_controler(d, x, tp),
        None => false,
    };
    if dd_mine {
        let x = dd.nil();
        // e:SetLabelObject(a)
        effect::set_label_object(d, a.e, atk.map_or(Value::Nil, Value::Card));
        // return d:IsFaceup() and d:IsRace(RACE_FIEND) and d:IsRelateToBattle()
        //     and a and a:IsFaceup() and a:IsRelateToBattle()
        return match atk {
            Some(y) => {
                card::is_faceup(d, x)
                    && card::is_race(d, x, RACE_FIEND)
                    && card::is_relate_to_battle(d, x)
                    && card::is_faceup(d, y)
                    && card::is_relate_to_battle(d, y)
            }
            None => false,
        };
    }
    // end (falls off: nil, which is false)
    false
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // if chk==0 then return true end
    true
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card();
    // if chkc then return chkc==tc end
    if let Some(chkc) = a.chkc() {
        return Some(chkc) == tc;
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=1 then return false end
        if effect::get_label(d, a.e) != 1 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return Duel.CheckLPCost(tp,100) and (tc:IsAttackAbove(100) or tc:IsDefenseAbove(100)) and tc:IsCanBeEffectTarget(e) end
        return duel::check_lp_cost(d, tp, 100)
            && (card::is_attack_above(d, tc.nil(), 100) || card::is_defense_above(d, tc.nil(), 100))
            && card::is_can_be_effect_target(d, tc.nil(), Some(a.e));
    }
    // local maxc=Duel.GetLP(tp)
    let mut maxc = duel::get_lp(d, tp) as i64;
    let tc = tc.nil();
    // local maxpay=tc:GetAttack()
    let mut maxpay = card::get_attack(d, tc) as i64;
    // local def=tc:GetDefense()
    let def = card::get_defense(d, tc) as i64;
    // if maxpay<def then maxpay=def end
    if maxpay < def {
        maxpay = def;
    }
    // if maxpay<maxc then maxc=maxpay end
    if maxpay < maxc {
        maxc = maxpay;
    }
    // if maxc>5000 then maxc=5000 end
    if maxc > 5000 {
        maxc = 5000;
    }
    // maxc=math.floor(maxc/100)*100
    maxc = maxc.div_euclid(100) * 100;
    // local t={}
    // for i=1,maxc/100 do t[i]=i*100 end
    let t: Vec<u64> = (1..=(maxc / 100)).map(|i| (i * 100) as u64).collect();
    // local cost=Duel.AnnounceNumber(tp,table.unpack(t))
    let cost = duel::announce_number(d, tp, &t).nil().0 as i64;
    // Duel.PayLPCost(tp,cost)
    duel::pay_lp_cost(d, tp, cost as u32);
    // Duel.SetTargetCard(tc)
    duel::set_target_card(d, tc);
    // Duel.SetTargetParam(cost)
    duel::set_target_param(d, cost as u32);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp,chk)
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local bc=Duel.GetFirstTarget()
    let bc = duel::first_target(d);
    // local val=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let val = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // if not bc or not bc:IsRelateToEffect(e) or not bc:IsControler(1-tp) then return end
    let Some(bc) = bc else {
        return;
    };
    if !card::is_relate_to_effect(d, bc, a.e) || !card::is_controler(d, bc, 1 - tp) {
        return;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(-val)
    effect::set_value(d, e1, -val);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // bc:RegisterEffect(e1)
    card::register_effect(d, bc, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // bc:RegisterEffect(e2)
    card::register_effect(d, bc, e2, false);
}
