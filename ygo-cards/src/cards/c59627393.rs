//! Number 105: Battlin' Boxer Star Cestus
use super::prelude::*;

pub const ID: u32 = 59627393;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BATTLIN_BOXER], xyz_number: Some(105), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 3 Level 4 monsters
    // Xyz.AddProcedure(c,nil,4,3)
    xyz::add_procedure(d, c, None, Some(4), 3, None, None, None, None, false, None);
    // Negate the effects of that opponent's monster ...
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_ATTACK|TIMING_BATTLE_PHASE)
    effect::set_hint_timing(d, e1, TIMING_ATTACK | TIMING_BATTLE_PHASE, None);
    // e1:SetCountLimit(1,0,EFFECT_COUNT_CODE_CHAIN)
    effect::set_count_limit(d, e1, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    // e1:SetCondition(s.discon)
    effect::set_condition(d, e1, Some(condition(discon)));
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(operation(disop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc1,bc2=Duel.GetBattleMonster(tp)
    let (bc1, bc2) = duel::get_battle_monster(d, a.tp);
    // return bc1 and bc2 and bc1:IsSetCard(SET_BATTLIN_BOXER) and bc1:IsFaceup()
    let Some(bc1) = bc1 else {
        return false;
    };
    if bc2.is_none() {
        return false;
    }
    card::is_set_card(d, bc1, &[SET_BATTLIN_BOXER as u16]) && card::is_faceup(d, bc1)
}
fn disop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc1,bc2=Duel.GetBattleMonster(tp)
    let (bc1, bc2) = duel::get_battle_monster(d, a.tp);
    // if bc2:IsRelateToBattle() and bc2:IsNegatableMonster() then
    let bc2 = bc2.nil();
    if card::is_relate_to_battle(d, bc2) && card::is_negatable_monster(d, bc2) {
        // bc2:NegateEffects(c,RESET_PHASE|PHASE_END)
        card::negate_effects(d, bc2, c, Some(RESET_PHASE | PHASE_END), false, None);
    }
    // if bc1:IsRelateToBattle() then
    let bc1 = bc1.nil();
    if card::is_relate_to_battle(d, bc1) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3000)
        effect::set_description(d, e1, 3000);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
        // bc1:RegisterEffect(e1,true)
        card::register_effect(d, bc1, e1, true);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetDescription(3112)
        effect::set_description(d, e2, 3112);
        // e2:SetCode(EFFECT_REFLECT_BATTLE_DAMAGE)
        effect::set_code(d, e2, EFFECT_REFLECT_BATTLE_DAMAGE);
        // bc1:RegisterEffect(e2,true)
        card::register_effect(d, bc1, e2, true);
    }
}
