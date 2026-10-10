//! The Hunter with 7 Weapons
use super::prelude::*;

pub const ID: u32 = 1525329;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // arm
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.armtg)
    effect::set_target(d, e1, Some(cost_target(armtg)));
    // e1:SetOperation(s.armop)
    effect::set_operation(d, e1, Some(operation(armop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.armtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn armtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RACE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RACE as u64);
    // local rc=Duel.AnnounceRace(tp,1,RACE_ALL)
    let rc = duel::announce_race(d, a.tp, 1, RACE_ALL);
    // e:SetLabel(rc)
    effect::set_label(d, a.e, &[rc as i64]);
    true
}
/// `s.armop(e,tp,eg,ep,ev,re,r,rp)`
fn armop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local rc=e:GetLabel()
        let rc = effect::get_label(d, a.e);
        // e:GetHandler():SetHint(CHINT_RACE,rc)
        card::set_hint(d, c, CHINT_RACE as u8, rc as u64);
        // atk up
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e1, aux::stringid(ID, 1));
        // e1:SetCategory(CATEGORY_ATKCHANGE)
        effect::set_category(d, e1, CATEGORY_ATKCHANGE);
        // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
        // e1:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
        effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
        // e1:SetCondition(s.upcon)
        effect::set_condition(d, e1, Some(condition(upcon)));
        // e1:SetOperation(s.upop)
        effect::set_operation(d, e1, Some(operation(upop)));
        // e1:SetLabel(rc)
        effect::set_label(d, e1, &[rc]);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.upcon(e,tp,eg,ep,ev,re,r,rp)`
fn upcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    // return bc and bc:IsRace(e:GetLabel())
    match card::get_battle_target(d, c) {
        Some(bc) => card::is_race(d, bc, effect::get_label(d, a.e) as u64),
        None => false,
    }
}
/// `s.upop(e,tp,eg,ep,ev,re,r,rp)`
fn upop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESET_PHASE|PHASE_DAMAGE_CAL)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE_CAL, 1);
        // e1:SetValue(1000)
        effect::set_value(d, e1, 1000);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
