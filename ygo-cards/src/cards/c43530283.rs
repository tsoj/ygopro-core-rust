//! Hourglass of Courage
//! 勇気の砂時計
use super::prelude::*;

pub const ID: u32 = 43530283;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atkup
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetOperation(s.adop)
    effect::set_operation(d, e1, Some(operation(adop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.adop(e,tp,eg,ep,ev,re,r,rp)`
fn adop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_BASE_ATTACK)
        effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
        // e1:SetValue(s.atkval)
        effect::set_value_fn(d, e1, effect_card_value(atkval));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_SET_BASE_DEFENSE)
        effect::set_code(d, e2, EFFECT_SET_BASE_DEFENSE);
        // e2:SetValue(s.defval)
        effect::set_value_fn(d, e2, effect_card_value(defval));
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
        // if Duel.IsTurnPlayer(tp) then
        if duel::is_turn_player(d, a.tp) {
            // c:RegisterFlagEffect(id,RESETS_STANDARD_DISABLE_PHASE_END|RESET_SELF_TURN,0,2)
            card::register_flag_effect(d, c, ID, RESETS_STANDARD_DISABLE_PHASE_END | RESET_SELF_TURN, 0, 2, 0, 0);
        // else
        } else {
            // c:RegisterFlagEffect(id,RESETS_STANDARD_DISABLE_PHASE_END|RESET_SELF_TURN,0,1)
            card::register_flag_effect(d, c, ID, RESETS_STANDARD_DISABLE_PHASE_END | RESET_SELF_TURN, 0, 1, 0, 0);
        }
    }
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // if c:GetFlagEffect(id)==0 then return c:GetBaseAttack()*2
    // else return c:GetBaseAttack()/2 end
    if card::get_flag_effect(d, c, ID) == 0 {
        (card::get_base_attack(d, c) * 2) as i64
    } else {
        lua_int(card::get_base_attack(d, c) as f64 / 2.0)
    }
}
/// `s.defval(e,c)`
fn defval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // if c:GetFlagEffect(id)==0 then return c:GetBaseDefense()*2
    // else return c:GetBaseDefense()/2 end
    if card::get_flag_effect(d, c, ID) == 0 {
        (card::get_base_defense(d, c) * 2) as i64
    } else {
        lua_int(card::get_base_defense(d, c) as f64 / 2.0)
    }
}
