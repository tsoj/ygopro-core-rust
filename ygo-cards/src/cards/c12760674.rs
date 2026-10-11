//! Mordschlag
use super::prelude::*;

pub const ID: u32 = 12760674;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,0,aux.FilterBoolFunction(Card.IsSummonType,SUMMON_TYPE_NORMAL))
    aux::add_equip_procedure(d, c, Some(0), Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_summon_type(d, c, &[SUMMON_TYPE_NORMAL]))), None, None, None, None, None, None);
    // immune
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetValue(s.immval)
    effect::set_value_fn(d, e1, lua_fn(immval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atk/def
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e2, EVENT_PRE_DAMAGE_CALCULATE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.adcon)
    effect::set_condition(d, e2, Some(condition(adcon)));
    // e2:SetOperation(s.adop)
    effect::set_operation(d, e2, Some(operation(adop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.immval(e,te)`
fn immval(d: &mut Duel, a: Args) -> bool {
    // return te:GetOwnerPlayer()~=e:GetHandlerPlayer() and te:IsMonsterEffect() and te:IsActivated()
    //     and te:GetHandler():IsSpecialSummoned()
    let e = a.e(0);
    let te = a.e(1);
    effect::get_owner_player(d, te) != effect::get_handler_player(d, e)
        && effect::is_monster_effect(d, te)
        && effect::is_activated(d, te)
        && {
            let h = effect::get_handler(d, te).nil();
            card::is_special_summoned(d, h)
        }
}
/// `s.adcon(e,tp,...)`
fn adcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let c = card::get_equip_target(d, h).nil();
    // local bc=c:GetBattleTarget()
    // return bc and bc:IsFaceup() and (bc:GetSummonType()&SUMMON_TYPE_SPECIAL)==SUMMON_TYPE_SPECIAL
    match card::get_battle_target(d, c) {
        Some(bc) => card::is_faceup(d, bc) && (card::get_summon_type(d, bc) & SUMMON_TYPE_SPECIAL) == SUMMON_TYPE_SPECIAL,
        None => false,
    }
}
/// `s.adop(e,tp,...)`
fn adop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local ec=e:GetHandler():GetEquipTarget()
    let ec = card::get_equip_target(d, h).nil();
    // local bc=ec:GetBattleTarget()
    let bc = card::get_battle_target(d, ec).nil();
    // if bc:IsRelateToBattle() then
    if card::is_relate_to_battle(d, bc) {
        // local value=ec:GetBaseAttack()
        let value = card::get_base_attack(d, ec) as i64;
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(-value)
        effect::set_value(d, e1, -value);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE_CAL)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE_CAL, 1);
        // bc:RegisterEffect(e1)
        card::register_effect(d, bc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_UPDATE_DEFENSE)
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        // bc:RegisterEffect(e2)
        card::register_effect(d, bc, e2, false);
    }
}
