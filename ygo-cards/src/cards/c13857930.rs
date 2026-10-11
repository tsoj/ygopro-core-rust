//! Neo-Spacian Twinkle Moss
use super::prelude::*;

pub const ID: u32 = 13857930;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e3, CATEGORY_DRAW);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.condition)
    effect::set_condition(d, e3, Some(condition(cond)));
    // e3:SetTarget(s.target)
    effect::set_target(d, e3, Some(cost_target(target)));
    // e3:SetOperation(s.activate)
    effect::set_operation(d, e3, Some(operation(activate)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // add code
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e4:SetCode(EFFECT_ADD_CODE)
    effect::set_code(d, e4, EFFECT_ADD_CODE);
    // e4:SetValue(17732278)
    effect::set_value(d, e4, 17732278);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler()==Duel.GetAttacker() or e:GetHandler()==Duel.GetAttackTarget()
    let h = effect::get_handler(d, a.e);
    h.is_some() && (h == duel::get_attacker(d) || h == duel::get_attack_target(d))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():GetFlagEffect(id)==0 end
    if a.chk == Some(0) {
        return card::get_flag_effect(d, h, ID) == 0;
    }
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE,0,1)
    card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 0, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.Draw(tp,1,REASON_EFFECT)==0 then return end
    if duel::draw(d, a.tp, 1, REASON_EFFECT) == 0 {
        return;
    }
    // local tc=Duel.GetOperatedGroup():GetFirst()
    let og = duel::get_operated_group(d);
    let tc = group::get_first(d, &og).nil();
    // Duel.ConfirmCards(1-tp,tc)
    duel::confirm_cards(d, 1 - a.tp, tc);
    // if tc:IsMonster() then
    if card::is_monster(d, tc) {
        // Duel.SkipPhase(Duel.GetTurnPlayer(),PHASE_BATTLE,RESET_PHASE|PHASE_BATTLE_STEP,1)
        let tp = duel::get_turn_player(d);
        duel::skip_phase(d, tp, PHASE_BATTLE, RESET_PHASE | PHASE_BATTLE_STEP, 1, 0);
    // elseif tc:IsSpell() then
    } else if card::is_spell(d, tc) {
        // if c==Duel.GetAttacker() and not c:IsHasEffect(EFFECT_CANNOT_DIRECT_ATTACK)
        //   and c:IsRelateToEffect(e) and c:IsFaceup() and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if duel::get_attacker(d) == Some(c)
            && card::is_has_effect(d, c, EFFECT_CANNOT_DIRECT_ATTACK, None).is_empty()
            && card::is_relate_to_effect(d, c, a.e)
            && card::is_faceup(d, c)
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
        {
            // Duel.ChangeAttackTarget(nil)
            duel::change_attack_target(d, None, false);
        }
    } else {
        // if c:IsRelateToEffect(e) and c:IsFaceup() then
        if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
            // Duel.ChangePosition(c,POS_FACEUP_DEFENSE)
            duel::change_position(d, c, POS_FACEUP_DEFENSE as u8, None, None, None, false);
        }
    }
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, a.tp);
}
