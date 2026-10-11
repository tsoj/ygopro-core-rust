//! Pig Iron vs. Pen Peg
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 46497537;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Reset monster's ATK/DEF/Level and pay LP
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE+CATEGORY_LVCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE + CATEGORY_LVCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Set itself if the LPs are equal
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function (_,tp) return Duel.GetLP(tp)==Duel.GetLP(1-tp) end)
    effect::set_condition(d, e2, Some(condition(|d, a| duel::get_lp(d, a.tp) == duel::get_lp(d, 1 - a.tp))));
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and (c:GetLevel()>c:GetOriginalLevel() or c:GetAttack()>c:GetBaseAttack() or c:GetDefense()>c:GetBaseDefense())
    if !card::is_faceup(d, c) {
        return false;
    }
    card::get_level(d, c) > card::get_original_level(d, c)
        || card::get_attack(d, c) > card::get_base_attack(d, c)
        || card::get_defense(d, c) > card::get_base_defense(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.cfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && cfilter(d, chkc);
    }
    // if chk==0 then return Duel.GetLP(tp)>=100 and Duel.IsExistingTarget(s.cfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::get_lp(d, tp) >= 100 && duel::is_existing_target(d, Some(&cfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.cfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let _ = duel::select_target(d, tp, Some(&cfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and tc:IsFaceup()) or tc:IsImmuneToEffect(e) then return end
    if !(card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc)) || card::is_immune_to_effect(d, tc, e) {
        return;
    }
    // local paylp=false
    let mut paylp = false;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local current,origin=tc:GetAttack(),tc:GetBaseAttack()
    let current = card::get_attack(d, tc);
    let origin = card::get_base_attack(d, tc);
    // if current~=origin then
    if current != origin {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(origin)
        effect::set_value(d, e1, origin as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // paylp=true
        paylp = true;
    }
    // current,origin=tc:GetDefense(),tc:GetBaseDefense()
    let current = card::get_defense(d, tc);
    let origin = card::get_base_defense(d, tc);
    // if current~=origin then
    if current != origin {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_DEFENSE_FINAL)
        effect::set_code(d, e1, EFFECT_SET_DEFENSE_FINAL);
        // e1:SetValue(origin)
        effect::set_value(d, e1, origin as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // paylp=true
        paylp = true;
    }
    // current,origin=tc:GetLevel(),tc:GetOriginalLevel()
    let current = card::get_level(d, tc);
    let origin = card::get_original_level(d, tc);
    // if current~=origin then
    if current != origin {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_LEVEL_FINAL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL_FINAL);
        // e1:SetValue(origin)
        effect::set_value(d, e1, origin as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // paylp=true
        paylp = true;
    }
    // if paylp and Duel.GetLP(tp)>=100 then
    if paylp && duel::get_lp(d, tp) >= 100 {
        // local maxval=math.floor(math.min(Duel.GetLP(tp),1000)/100)
        let maxval = duel::get_lp(d, tp).min(1000) / 100;
        // local t={}
        // for i=1,maxval do t[i]=i*100 end
        let t: Vec<u64> = (1..=maxval as u64).map(|i| i * 100).collect();
        // local value=Duel.AnnounceNumber(tp,table.unpack(t))
        let value = duel::announce_number(d, tp, &t).nil().0;
        // Duel.PayLPCost(tp,value)
        duel::pay_lp_cost(d, tp, value as u32);
    }
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsSSetable() end
    if a.chk == Some(0) {
        return card::is_s_setable(d, c, false, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsSSetable() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_s_setable(d, c, false, None) {
        // Duel.SSet(tp,c)
        duel::s_set(d, tp, c, None, true);
    }
}
