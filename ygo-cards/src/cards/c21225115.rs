//! Fossil Dragon Skullgios
use super::prelude::*;

pub const ID: u32 = 21225115;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_FOSSIL_FUSION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 Rock monster + 1 Level 7 or higher monster in your opponent's GY
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsRace,RACE_ROCK),s.matfilter)
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_ROCK, Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(matfilter)),
        ],
    );
    // "Clock Lizard" check
    // Auxiliary.addLizardCheck(c)
    aux::add_lizard_check(d, c);
    // Must be Special Summoned with "Fossil Fusion"
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_SINGLE_RANGE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    // e0:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e0, LOCATION_EXTRA);
    // e0:SetValue(aux.FossilLimit)
    effect::set_value_fn(d, e0, aux::fossil_limit());
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Switch the current ATK and DEF of that opponent's monster this card is battling until the end of that Damage Step
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLE_CONFIRM)
    effect::set_code(d, e1, EVENT_BATTLE_CONFIRM);
    // e1:SetCondition(s.switchcon)
    effect::set_condition(d, e1, Some(condition(switchcon)));
    // e1:SetOperation(s.switchop)
    effect::set_operation(d, e1, Some(operation(switchop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card attacks a Defense Position monster, inflict piercing battle damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e2, EFFECT_PIERCE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // If this Fusion Summoned card battles an opponent's monster, any battle damage it inflicts to your opponent is doubled
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e3, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e3:SetCondition(s.doubledamcon)
    effect::set_condition(d, e3, Some(condition(doubledamcon)));
    // e3:SetValue(aux.ChangeBattleDamage(1,DOUBLE_DAMAGE))
    effect::set_value_fn(d, e3, aux::change_battle_damage(1, DOUBLE_DAMAGE as i64));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.matfilter(c,sc,st,tp)
fn matfilter(d: &mut Duel, c: CardId, _sc: CardId, _st: u64, tp: u8) -> bool {
    // return c:IsLevelAbove(7) and c:IsControler(1-tp) and c:IsLocation(LOCATION_GRAVE)
    card::is_level_above(d, c, 7) && card::is_controler(d, c, 1 - tp) && card::is_location(d, c, LOCATION_GRAVE)
}
// function s.switchcon(e,tp,eg,ep,ev,re,r,rp)
fn switchcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let bc = match card::get_battle_target(d, h) {
        Some(bc) => bc,
        None => return false,
    };
    // return bc and bc:IsRelateToBattle() and bc:HasDefense() and not bc:IsAttack(bc:GetDefense()) and bc:IsControler(1-tp)
    if !card::is_relate_to_battle(d, bc) || !card::has_defense(d, bc) {
        return false;
    }
    let def = card::get_defense(d, bc);
    !card::is_attack(d, bc, &[def as u32]) && card::is_controler(d, bc, 1 - a.tp)
}
// function s.switchop(e,tp,eg,ep,ev,re,r,rp)
fn switchop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c).nil();
    // if bc:IsRelateToBattle() and bc:IsFaceup() and bc:IsControler(1-tp) then
    if card::is_relate_to_battle(d, bc) && card::is_faceup(d, bc) && card::is_controler(d, bc, 1 - a.tp) {
        // local atk=bc:GetAttack()
        let atk = card::get_attack(d, bc);
        // local def=bc:GetDefense()
        let def = card::get_defense(d, bc);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_SWAP_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SWAP_ATTACK_FINAL);
        // e1:SetValue(def)
        effect::set_value(d, e1, def as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
        // bc:RegisterEffect(e1)
        card::register_effect(d, bc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_SWAP_DEFENSE_FINAL)
        effect::set_code(d, e2, EFFECT_SWAP_DEFENSE_FINAL);
        // e2:SetValue(atk)
        effect::set_value(d, e2, atk as i64);
        // bc:RegisterEffect(e2)
        card::register_effect(d, bc, e2, false);
    }
}
// function s.doubledamcon(e)
fn doubledamcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // return c:IsFusionSummoned() and bc and bc:IsControler(1-e:GetHandlerPlayer())
    card::is_fusion_summoned(d, c) && bc.is_some() && card::is_controler(d, bc.nil(), 1 - effect::get_handler_player(d, a.e))
}
