//! Psychic Commander
use super::prelude::*;

pub const ID: u32 = 21454943;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atkdown
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,...)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // local phase=Duel.GetCurrentPhase()
    let phase = duel::get_current_phase(d);
    // if phase~=PHASE_DAMAGE or Duel.IsDamageCalculated() then return false end
    if phase != PHASE_DAMAGE || duel::is_damage_calculated(d) {
        return false;
    }
    // local a=Duel.GetAttacker()
    let at = duel::get_attacker(d).nil();
    // local d=Duel.GetAttackTarget()
    let dt = duel::get_attack_target(d);
    // if a:IsControler(tp) then
    if card::is_controler(d, at, a.tp) {
        // e:SetLabelObject(d)
        effect::set_label_object(d, a.e, Value::card(dt));
        // return a:IsFaceup() and a:IsRace(RACE_PSYCHIC) and a:IsRelateToBattle() and d and d:IsFaceup() and d:IsRelateToBattle()
        card::is_faceup(d, at) && card::is_race(d, at, RACE_PSYCHIC) && card::is_relate_to_battle(d, at) && match dt {
            Some(dt) => card::is_faceup(d, dt) && card::is_relate_to_battle(d, dt),
            None => false,
        }
    } else {
        // e:SetLabelObject(a)
        effect::set_label_object(d, a.e, Value::card(Some(at)));
        // return d and d:IsFaceup() and d:IsRace(RACE_PSYCHIC) and d:IsRelateToBattle() and a and a:IsFaceup() and a:IsRelateToBattle()
        match dt {
            Some(dt) => {
                card::is_faceup(d, dt) && card::is_race(d, dt, RACE_PSYCHIC) && card::is_relate_to_battle(d, dt) && card::is_faceup(d, at) && card::is_relate_to_battle(d, at)
            }
            None => false,
        }
    }
}
/// `s.cost(e,...,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=e:GetLabelObject()
    let bc_opt = effect::get_label_object(d, a.e).as_card();
    // if chk==0 then return Duel.CheckLPCost(tp,100) and e:GetHandler():GetFlagEffect(id)==0
    //     and (bc:IsAttackAbove(100) or bc:IsDefenseAbove(100)) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::check_lp_cost(d, a.tp, 100)
            && card::get_flag_effect(d, h, ID) == 0
            && (card::is_attack_above(d, bc_opt.nil(), 100) || card::is_defense_above(d, bc_opt.nil(), 100));
    }
    let bc = bc_opt.nil();
    // local lp=Duel.GetLP(tp)-1
    let mut lp = duel::get_lp(d, a.tp) as i64 - 1;
    // local alp=100
    let mut alp: u64 = 100;
    // local maxpay=bc:GetAttack()
    let mut maxpay = card::get_attack(d, bc) as i64;
    // local def=bc:GetDefense()
    let def = card::get_defense(d, bc) as i64;
    // if maxpay<def then maxpay=def end
    if maxpay < def {
        maxpay = def;
    }
    // if maxpay<lp then lp=maxpay end
    if maxpay < lp {
        lp = maxpay;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
    // if lp>=500 then alp=Duel.AnnounceNumber(tp,100,200,300,400,500)
    if lp >= 500 {
        alp = duel::announce_number(d, a.tp, &[100, 200, 300, 400, 500]).nil().0;
    }
    // elseif lp>=400 then alp=Duel.AnnounceNumber(tp,100,200,300,400)
    else if lp >= 400 {
        alp = duel::announce_number(d, a.tp, &[100, 200, 300, 400]).nil().0;
    }
    // elseif lp>=300 then alp=Duel.AnnounceNumber(tp,100,200,300)
    else if lp >= 300 {
        alp = duel::announce_number(d, a.tp, &[100, 200, 300]).nil().0;
    }
    // elseif lp>=200 then alp=Duel.AnnounceNumber(tp,100,200)
    else if lp >= 200 {
        alp = duel::announce_number(d, a.tp, &[100, 200]).nil().0;
    }
    // Duel.PayLPCost(tp,alp)
    duel::pay_lp_cost(d, a.tp, alp as u32);
    // e:SetLabel(-alp)
    effect::set_label(d, a.e, &[-(alp as i64)]);
    // e:GetHandler():RegisterFlagEffect(id,RESET_PHASE|PHASE_DAMAGE,0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_PHASE | PHASE_DAMAGE, 0, 1, 0, 0);
    true
}
/// `s.target(e,...,chk,chkc)`
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
/// `s.operation(e,...)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not tc or not tc:IsRelateToEffect(e) or not tc:IsControler(1-tp) then return end
    let Some(tc) = tc else {
        return;
    };
    if !card::is_relate_to_effect(d, tc, a.e) || !card::is_controler(d, tc, 1 - a.tp) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // e1:SetValue(e:GetLabel())
    let label = effect::get_label(d, a.e);
    effect::set_value(d, e1, label);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // tc:RegisterEffect(e2)
    card::register_effect(d, tc, e2, false);
}
