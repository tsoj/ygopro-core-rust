//! King Scarlet
use super::prelude::*;

pub const ID: u32 = 60433216;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RED_DRAGON_ARCHFIEND], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// the attacking card, or the attack target when the opponent attacks (`tc` in the Lua)
fn battle_card(d: &mut Duel, tp: u8) -> Option<CardId> {
    // local tc=Duel.GetAttacker()
    // if tc:IsControler(1-tp) then tc=Duel.GetAttackTarget() end
    let atk = duel::get_attacker(d).nil();
    if card::is_controler(d, atk, 1 - tp) {
        duel::get_attack_target(d)
    } else {
        Some(atk)
    }
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return tc and tc:IsSetCard(SET_RED_DRAGON_ARCHFIEND) and tc:IsRelateToBattle()
    match battle_card(d, a.tp) {
        Some(tc) => card::is_set_card(d, tc, &[SET_RED_DRAGON_ARCHFIEND as u16]) && card::is_relate_to_battle(d, tc),
        None => false,
    }
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_NORMAL|TYPE_TUNER,0,0,1,RACE_FIEND,ATTRIBUTE_FIRE) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && can_summon(d, tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,tp,0)
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
// Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_NORMAL|TYPE_TUNER,0,0,1,RACE_FIEND,ATTRIBUTE_FIRE)
fn can_summon(d: &mut Duel, tp: u8) -> bool {
    duel::is_player_can_special_summon_monster(
        d,
        tp,
        ID,
        duel::MonsterData {
            setcodes: Some(vec![0]),
            type_: Some(TYPE_MONSTER | TYPE_NORMAL | TYPE_TUNER),
            attack: Some(0),
            defense: Some(0),
            level: Some(1),
            race: Some(RACE_FIEND),
            attribute: Some(ATTRIBUTE_FIRE),
        },
        None,
        None,
        0,
    )
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetAttacker()
    // if tc:IsControler(1-tp) then tc=Duel.GetAttackTarget() end
    // if not tc or not tc:IsRelateToBattle() then return end
    let Some(tc) = battle_card(d, tp) else { return };
    if !card::is_relate_to_battle(d, tc) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetReset(RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0
    //   or not Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_NORMAL|TYPE_TUNER,0,0,1,RACE_FIEND,ATTRIBUTE_FIRE) then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 || !can_summon(d, tp) {
        return;
    }
    // c:AddMonsterAttribute(TYPE_NORMAL+TYPE_TUNER+TYPE_TRAP)
    card::add_monster_attribute(d, c, TYPE_NORMAL + TYPE_TUNER + TYPE_TRAP, 0, 0, 0, 0, 0);
    // Duel.SpecialSummonStep(c,0,tp,tp,true,false,POS_FACEUP)
    duel::special_summon_step(d, c, 0, tp, tp, true, false, POS_FACEUP as u8, None);
    // c:AddMonsterAttributeComplete()
    card::add_monster_attribute_complete(d, c);
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
