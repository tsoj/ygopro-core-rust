//! Absolute Powerforce
use super::prelude::*;

pub const ID: u32 = 51779204;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_RED_DRAGON_ARCHFIEND], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply effects when the targeted "Red Dragon Archfiend" you control battles an opponent's monster this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    // e1:SetCondition(function() return (Duel.IsAbleToEnterBP() or (Duel.IsBattlePhase() and not Duel.IsPhase(PHASE_BATTLE))) and aux.StatChangeDamageStepCondition() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| activate_cond(d))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn activate_cond(d: &mut Duel) -> bool {
    (duel::is_able_to_enter_bp(d) || (duel::is_battle_phase(d, None) && !duel::is_phase(d, PHASE_BATTLE)))
        && aux::stat_change_damage_step_condition(d)
}
fn faceup_rda(d: &mut Duel, c: CardId) -> bool {
    // aux.FaceupFilter(Card.IsCode,CARD_RED_DRAGON_ARCHFIEND)
    card::is_faceup(d, c) && card::is_code(d, c, &[CARD_RED_DRAGON_ARCHFIEND])
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsCode(CARD_RED_DRAGON_ARCHFIEND) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp)
            && card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_code(d, chkc, &[CARD_RED_DRAGON_ARCHFIEND])
            && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsCode,CARD_RED_DRAGON_ARCHFIEND),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&faceup_rda), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_APPLYTO)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_APPLYTO as u64);
    // Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsCode,CARD_RED_DRAGON_ARCHFIEND),tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, a.tp, Some(&faceup_rda), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
fn battle_cond(d: &mut Duel, tc: CardId, tp: u8, fid: u32) -> bool {
    // (Duel.GetAttacker()==tc or Duel.GetAttackTarget()==tc) and tc:GetBattleTarget() and tc:GetBattleTarget():IsControler(1-tp)
    //   and tc:HasFlagEffect(id) and tc:GetFlagEffectLabel(id)==fid and tc:IsControler(tp)
    (duel::get_attacker(d) == Some(tc) || duel::get_attack_target(d) == Some(tc))
        && card::get_battle_target(d, tc).is_some_and(|bt| card::is_controler(d, bt, 1 - tp))
        && card::has_flag_effect(d, tc, ID, None)
        && card::get_flag_effect_label(d, tc, ID).first() == Some(&(fid as i64))
        && card::is_controler(d, tc, tp)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    let tp = a.tp;
    // local fid=tc:GetFieldID()
    let fid = card::get_field_id(d, tc);
    // local function condition() ... end
    // --It gains 1000 ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetCondition(condition)
    effect::set_condition(d, e1, Some(condition(move |d, _a| battle_cond(d, tc, tp, fid))));
    // e1:SetTarget(function(e,c) return c==tc and condition() end)
    effect::set_target(d, e1, Some(effect_card_fn(move |d, _e, c| c == tc && battle_cond(d, tc, tp, fid))));
    // e1:SetValue(1000)
    effect::set_value(d, e1, 1000);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // if not tc:HasFlagEffect(id) then
    if !card::has_flag_effect(d, tc, ID, None) {
        // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,EFFECT_FLAG_CLIENT_HINT,1,fid,aux.Stringid(id,1))
        card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, EFFECT_FLAG_CLIENT_HINT, 1, fid as i64, aux::stringid(ID, 1));
        // --Your opponent cannot activate cards or effects
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD);
        // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
        effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
        // e2:SetTargetRange(0,1)
        effect::set_target_range(d, e2, 0, 1);
        // e2:SetCondition(condition)
        effect::set_condition(d, e2, Some(condition(move |d, _a| battle_cond(d, tc, tp, fid))));
        // e2:SetValue(1)
        effect::set_value(d, e2, 1);
        // e2:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, tp);
        // --If it attacks a Defense Position monster, inflict piercing battle damage to your opponent
        // local e3=e1:Clone()
        let e3 = effect::clone(d, e1);
        // e3:SetCode(EFFECT_PIERCE)
        effect::set_code(d, e3, EFFECT_PIERCE);
        // Duel.RegisterEffect(e3,tp)
        duel::register_effect(d, e3, tp);
        // --Any battle damage your opponent takes from that battle is doubled
        // local e4=e1:Clone()
        let e4 = effect::clone(d, e1);
        // e4:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e4, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e4:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
        effect::set_code(d, e4, EFFECT_CHANGE_BATTLE_DAMAGE);
        // e4:SetValue(aux.ChangeBattleDamage(1,DOUBLE_DAMAGE))
        effect::set_value_fn(d, e4, aux::change_battle_damage(1, DOUBLE_DAMAGE as i64));
        // Duel.RegisterEffect(e4,tp)
        duel::register_effect(d, e4, tp);
    }
}
