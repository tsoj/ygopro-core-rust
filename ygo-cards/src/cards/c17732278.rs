//! Neo-Spacian Glow Moss
use super::prelude::*;

pub const ID: u32 = 17732278;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler()==Duel.GetAttacker() or e:GetHandler()==Duel.GetAttackTarget()
    let h = effect::get_handler(d, a.e);
    h == duel::get_attacker(d) || h == duel::get_attack_target(d)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():GetFlagEffect(id)==0 end
    if a.chk == Some(0) {
        return card::get_flag_effect(d, h, ID) == 0;
    }
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE,0,1)
    card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 0, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,1-tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, 1 - tp, 1);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if Duel.Draw(1-tp,1,REASON_EFFECT)==0 then return end
    if duel::draw(d, 1 - tp, 1, REASON_EFFECT) == 0 {
        return;
    }
    // local tc=Duel.GetOperatedGroup():GetFirst()
    let og = duel::get_operated_group(d);
    let tc = group::get_first(d, &og).nil();
    // Duel.ConfirmCards(tp,tc)
    duel::confirm_cards(d, tp, tc);
    // if tc:IsMonster() then
    if card::is_monster(d, tc) {
        // if not Duel.IsPlayerAffectedByEffect(Duel.GetTurnPlayer(),EFFECT_SKIP_BP) then
        let turn_player = duel::get_turn_player(d);
        if duel::is_player_affected_by_effect(d, turn_player, EFFECT_SKIP_BP).is_none() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SkipPhase(Duel.GetTurnPlayer(),PHASE_BATTLE,RESET_PHASE|PHASE_BATTLE_STEP,1)
            let turn_player = duel::get_turn_player(d);
            duel::skip_phase(d, turn_player, PHASE_BATTLE, RESET_PHASE | PHASE_BATTLE_STEP, 1, 0);
        }
    // elseif tc:IsSpell() then
    } else if card::is_spell(d, tc) {
        // if c==Duel.GetAttacker() and not c:IsHasEffect(EFFECT_CANNOT_DIRECT_ATTACK)
        //   and c:IsRelateToEffect(e) and c:IsFaceup() and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if Some(c) == duel::get_attacker(d)
            && card::is_has_effect(d, c, EFFECT_CANNOT_DIRECT_ATTACK, None).is_empty()
            && card::is_relate_to_effect(d, c, e)
            && card::is_faceup(d, c)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.ChangeAttackTarget(nil)
            duel::change_attack_target(d, None, false);
        }
    // else
    } else {
        // if c:IsRelateToEffect(e) and c:IsFaceup() then
        if card::is_relate_to_effect(d, c, e) && card::is_faceup(d, c) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.ChangePosition(c,POS_FACEUP_DEFENSE)
            duel::change_position(d, c, POS_FACEUP_DEFENSE as u8, None, None, None, false);
        }
    }
    // Duel.ShuffleHand(1-tp)
    duel::shuffle_hand(d, 1 - tp);
}
