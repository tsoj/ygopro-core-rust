//! Galaxy Dragon
use super::prelude::*;

pub const ID: u32 = 9024367;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atklimit
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e1:SetValue(s.bttg)
    effect::set_value_fn(d, e1, effect_card_fn(bttg));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
    effect::set_code(d, e2, EFFECT_CANNOT_DIRECT_ATTACK);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // atkup
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetCondition(s.atkcon)
    effect::set_condition(d, e3, Some(condition(atkcon)));
    // e3:SetValue(1000)
    effect::set_value(d, e3, 1000);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // disable
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e4, EVENT_ATTACK_ANNOUNCE);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetOperation(s.disop)
    effect::set_operation(d, e4, Some(operation(disop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e5, EVENT_PRE_DAMAGE_CALCULATE);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.bttg(e,c)`
fn bttg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsFacedown() or not c:IsRace(RACE_DRAGON)
    card::is_facedown(d, c) || !card::is_race(d, c, RACE_DRAGON)
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ph=Duel.GetCurrentPhase()
    let ph = duel::get_current_phase(d);
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, h);
    // return (ph==PHASE_DAMAGE or ph==PHASE_DAMAGE_CAL) and bc and bc:IsRace(RACE_DRAGON)
    if !(ph == PHASE_DAMAGE || ph == PHASE_DAMAGE_CAL) {
        return false;
    }
    match bc {
        Some(bc) => card::is_race(d, bc, RACE_DRAGON),
        None => false,
    }
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // if bc and bc:IsRace(RACE_DRAGON) then
    if let Some(bc) = bc {
        if card::is_race(d, bc, RACE_DRAGON) {
            // c:CreateRelation(bc,RESET_EVENT|RESETS_STANDARD)
            card::create_relation(d, c, bc, RESET_EVENT | RESETS_STANDARD);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetCondition(s.discon)
            effect::set_condition(d, e1, Some(condition(discon)));
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 1);
            // bc:RegisterEffect(e1)
            card::register_effect(d, bc, e1, false);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_DISABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            // e2:SetCondition(s.discon)
            effect::set_condition(d, e2, Some(condition(discon)));
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 1);
            // bc:RegisterEffect(e2)
            card::register_effect(d, bc, e2, false);
        }
    }
}
/// `s.discon(e)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetOwner():IsRelateToCard(e:GetHandler())
    let owner = effect::get_owner(d, a.e).nil();
    let h = effect::get_handler(d, a.e).nil();
    card::is_relate_to_card(d, owner, h)
}
