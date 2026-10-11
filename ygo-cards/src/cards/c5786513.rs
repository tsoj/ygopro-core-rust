//! Turn Silence
use super::prelude::*;

pub const ID: u32 = 5786513;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_SHINING_SARCOPHAGUS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_LVCHANGE+CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_LVCHANGE + CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // End the Battle Phase
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e2, EVENT_PRE_DAMAGE_CALCULATE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCondition(function(_,tp)
    //     local tc=Duel.GetBattleMonster(tp)
    //     return tc and tc:ListsCode(CARD_SHINING_SARCOPHAGUS)
    // end)
    effect::set_condition(d, e2, Some(condition(battle_cond)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetOperation(s.skipop)
    effect::set_operation(d, e2, Some(operation(skipop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function(_,tp) local tc=Duel.GetBattleMonster(tp) return tc and tc:ListsCode(CARD_SHINING_SARCOPHAGUS) end
fn battle_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=Duel.GetBattleMonster(tp)
    let (tc, _) = duel::get_battle_monster(d, a.tp);
    // return tc and tc:ListsCode(CARD_SHINING_SARCOPHAGUS)
    match tc {
        Some(tc) => card::lists_code(d, tc, &[CARD_SHINING_SARCOPHAGUS]),
        None => false,
    }
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and chkc:IsFaceup() and chkc:HasLevel() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, tp)
            && card::is_faceup(d, chkc)
            && card::has_level(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.HasLevel),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c) && card::has_level(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,aux.FaceupFilter(Card.HasLevel),tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c) && card::has_level(d, c)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // local ch=Duel.GetCurrentChain()-1
    let ch = duel::get_current_chain(d, false) - 1;
    // local trig_p,trig_e=Duel.GetChainInfo(ch,CHAININFO_TRIGGERING_PLAYER,CHAININFO_TRIGGERING_EFFECT)
    let info = duel::get_chain_info(d, ch as u8, &[CHAININFO_TRIGGERING_PLAYER, CHAININFO_TRIGGERING_EFFECT]);
    let trig_p_is_opp = !info[0].is_nil() && info[0].int() == (1 - tp) as i64;
    let trig_e = info[1].as_effect();
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) and ch>0 and trig_p==1-tp and trig_e:IsMonsterEffect() and Duel.IsChainDisablable(ch)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_SHINING_SARCOPHAGUS),tp,LOCATION_ONFIELD,0,1,nil)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.ListsCode,CARD_SHINING_SARCOPHAGUS),tp,LOCATION_MZONE,0,1,nil) then
    let cond = effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE)
        && ch > 0
        && trig_p_is_opp
        && effect::is_monster_effect(d, trig_e.nil())
        && duel::is_chain_disablable(d, ch as u8)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_SHINING_SARCOPHAGUS])), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::lists_code(d, c, &[CARD_SHINING_SARCOPHAGUS])), tp, LOCATION_MZONE, 0, 1, NO_EX);
    // e:SetLabel(1)
    // else e:SetLabel(0) end
    effect::set_label(d, a.e, &[if cond { 1 } else { 0 }]);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) and tc:HasLevel() then
    let tc = tc.nil();
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && card::has_level(d, tc) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_UPDATE_LEVEL)
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        // e1:SetValue(3)
        effect::set_value(d, e1, 3);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
    // local ch=Duel.GetCurrentChain()-1
    let ch = duel::get_current_chain(d, false) - 1;
    // if e:GetLabel()==1 then
    if effect::get_label(d, a.e) == 1 {
        // Duel.NegateEffect(ch)
        duel::negate_effect(d, ch as u8);
    }
}
// function s.skipop(e,tp,eg,ep,ev,re,r,rp)
fn skipop(d: &mut Duel, _a: &EffArgs) {
    // Duel.GetAttacker():SetStatus(STATUS_ATTACK_CANCELED,true)
    let atk = duel::get_attacker(d).nil();
    card::set_status(d, atk, STATUS_ATTACK_CANCELED, true);
    // Duel.SkipPhase(Duel.GetTurnPlayer(),PHASE_BATTLE,RESET_PHASE|PHASE_BATTLE_STEP,1)
    let tp = duel::get_turn_player(d);
    duel::skip_phase(d, tp, PHASE_BATTLE, RESET_PHASE | PHASE_BATTLE_STEP, 1, 0);
}
