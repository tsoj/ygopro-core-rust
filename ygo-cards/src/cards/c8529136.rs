//! Scrap Fist
use super::prelude::*;

pub const ID: u32 = 8529136;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_JUNK_WARRIOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply effects if 1 "Junk Warrior" you control battles this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_BATTLE_PHASE)
    effect::set_hint_timing(d, e1, TIMING_BATTLE_PHASE, None);
    // e1:SetCondition(function() return Duel.IsAbleToEnterBP() or (Duel.IsBattlePhase() and not Duel.IsPhase(PHASE_BATTLE)) end)
    effect::set_condition(d, e1, Some(condition(enter_bp_cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function() return Duel.IsAbleToEnterBP() or (Duel.IsBattlePhase() and not Duel.IsPhase(PHASE_BATTLE)) end
fn enter_bp_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    if duel::is_able_to_enter_bp(d) {
        return true;
    }
    duel::is_battle_phase(d, None) && !duel::is_phase(d, PHASE_BATTLE)
}
// function s.tgfilter(c)
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_JUNK_WARRIOR) and c:IsFaceup() and not c:HasFlagEffect(id)
    card::is_code(d, c, &[CARD_JUNK_WARRIOR]) && card::is_faceup(d, c) && !card::has_flag_effect(d, c, ID, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.tgfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tgfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&tgfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
// Condition of the two continuous effects and of the destruction effect, with the values
// captured by the Lua closure `condition` of s.activate.
// local function condition()
fn scrap_cond(d: &mut Duel, tp: u8, tc: CardId, fid: u32) -> bool {
    // return (Duel.GetAttacker()==tc or Duel.GetAttackTarget()==tc) and tc:GetBattleTarget() and tc:GetBattleTarget():IsControler(1-tp)
    //     and tc:HasFlagEffect(id) and tc:GetFlagEffectLabel(id)==fid and tc:IsControler(tp)
    if !(duel::get_attacker(d) == Some(tc) || duel::get_attack_target(d) == Some(tc)) {
        return false;
    }
    let bt = match card::get_battle_target(d, tc) {
        Some(bt) => bt,
        None => return false,
    };
    if !card::is_controler(d, bt, 1 - tp) {
        return false;
    }
    card::has_flag_effect(d, tc, ID, None)
        && card::get_flag_effect_label(d, tc, ID).first().copied() == Some(fid as i64)
        && card::is_controler(d, tc, tp)
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local fid=tc:GetFieldID()
    let fid = card::get_field_id(d, tc);
    // --If it battles an opponent's monster this turn while you control it, apply these effects
    // if not tc:HasFlagEffect(id) then
    if !card::has_flag_effect(d, tc, ID, None) {
        // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,EFFECT_FLAG_CLIENT_HINT,1,fid,aux.Stringid(id,1))
        card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, EFFECT_FLAG_CLIENT_HINT, 1, fid as i64, aux::stringid(ID, 1));
        // --Your opponent cannot activate cards or effects until the end of the Damage Step
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
        effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
        // e1:SetTargetRange(0,1)
        effect::set_target_range(d, e1, 0, 1);
        // e1:SetCondition(condition)
        effect::set_condition(d, e1, Some(condition(move |d, _a| scrap_cond(d, tp, tc, fid))));
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
        // --If it attacks a Defense Position monster, inflict piercing battle damage to your opponent
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD);
        // e2:SetCode(EFFECT_PIERCE)
        effect::set_code(d, e2, EFFECT_PIERCE);
        // e2:SetTargetRange(LOCATION_MZONE,0)
        effect::set_target_range(d, e2, LOCATION_MZONE, 0);
        // e2:SetCondition(condition)
        effect::set_condition(d, e2, Some(condition(move |d, _a| scrap_cond(d, tp, tc, fid))));
        // e2:SetTarget(condition)
        effect::set_target(d, e2, Some(effect_card_fn(move |d, _e, _c| scrap_cond(d, tp, tc, fid))));
        // e2:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, tp);
        // --Double any battle damage your opponent takes
        // local e3=e2:Clone()
        let e3 = effect::clone(d, e2);
        // e3:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e3:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
        effect::set_code(d, e3, EFFECT_CHANGE_BATTLE_DAMAGE);
        // e3:SetValue(aux.ChangeBattleDamage(1,DOUBLE_DAMAGE))
        effect::set_value_fn(d, e3, aux::change_battle_damage(1, DOUBLE_DAMAGE as i64));
        // Duel.RegisterEffect(e3,tp)
        duel::register_effect(d, e3, tp);
        // --It cannot be destroyed by battle
        // local e4=e2:Clone()
        let e4 = effect::clone(d, e2);
        // e4:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
        effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_BATTLE);
        // e4:SetValue(1)
        effect::set_value(d, e4, 1);
        // Duel.RegisterEffect(e4,tp)
        duel::register_effect(d, e4, tp);
    }
    // --Destroy the opponent's monster that battled it at the end of the Damage Step
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e5:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e5, EVENT_DAMAGE_STEP_END);
    // e5:SetCondition(function() return condition() and tc:GetBattleTarget():IsRelateToBattle() end)
    effect::set_condition(
        d,
        e5,
        Some(condition(move |d, _a| {
            scrap_cond(d, tp, tc, fid) && {
                let bt = card::get_battle_target(d, tc).nil();
                card::is_relate_to_battle(d, bt)
            }
        })),
    );
    // e5:SetOperation(function() Duel.Hint(HINT_CARD,0,id) Duel.Destroy(tc:GetBattleTarget(),REASON_EFFECT) end)
    effect::set_operation(
        d,
        e5,
        Some(operation(move |d, _a| {
            duel::hint(d, HINT_CARD as u8, 0, ID as u64);
            let bt = card::get_battle_target(d, tc).nil();
            duel::destroy(d, bt, REASON_EFFECT, None, None);
        })),
    );
    // e5:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e5, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e5,tp)
    duel::register_effect(d, e5, tp);
}
