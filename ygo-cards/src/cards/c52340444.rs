//! Sky Striker Mecha - Hornet Drones
use super::prelude::*;

pub const ID: u32 = 52340444;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[52340445], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Sky Striker Ace Token" to your field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,LOCATION_MMZONE,0)==0
    duel::get_field_group_count(d, a.tp, LOCATION_MMZONE, 0) == 0
}
// Token data of Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_SKY_STRIKER_ACE,TYPES_TOKEN,atk,atk,1,RACE_WARRIOR,ATTRIBUTE_DARK,POS_FACEUP_DEFENSE)
fn token_data(atk: i32) -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_SKY_STRIKER_ACE as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(atk),
        defense: Some(atk),
        level: Some(1),
        race: Some(RACE_WARRIOR),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local atk=0
    let mut atk = 0;
    // if Duel.GetMatchingGroupCount(Card.IsSpell,tp,LOCATION_GRAVE,0,nil)>=3 then atk=1500 end
    if duel::get_matching_group_count(d, Some(&card::is_spell), a.tp, LOCATION_GRAVE, 0, NO_EX) >= 3 {
        atk = 1500;
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsPlayerCanSpecialSummonMonster(...) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(atk), Some(POS_FACEUP_DEFENSE as u8), None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local atk=0
    let mut atk = 0;
    // if Duel.GetMatchingGroupCount(Card.IsSpell,tp,LOCATION_GRAVE,0,nil)>=3 then atk=1500 end
    if duel::get_matching_group_count(d, Some(&card::is_spell), a.tp, LOCATION_GRAVE, 0, NO_EX) >= 3 {
        atk = 1500;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0
    //   or not Duel.IsPlayerCanSpecialSummonMonster(...) then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0
        || !duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(atk), Some(POS_FACEUP_DEFENSE as u8), None, 0)
    {
        return;
    }
    // local token=Duel.CreateToken(tp,id+1)
    let token = duel::create_token(d, a.tp, ID + 1).nil();
    // if Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP_DEFENSE) then
    if duel::special_summon_step(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None) {
        let h = effect::get_handler(d, a.e).nil();
        // Cannot be tributed
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(3303)
        effect::set_description(d, e1, 3303);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UNRELEASABLE_SUM)
        effect::set_code(d, e1, EFFECT_UNRELEASABLE_SUM);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // token:RegisterEffect(e1)
        card::register_effect(d, token, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_UNRELEASABLE_NONSUM)
        effect::set_code(d, e2, EFFECT_UNRELEASABLE_NONSUM);
        // token:RegisterEffect(e2)
        card::register_effect(d, token, e2, false);
        // Gains 1500 ATK/DEF if 3+ spells are in the GY when resolving
        // if Duel.GetMatchingGroupCount(Card.IsSpell,tp,LOCATION_GRAVE,0,nil)>=3 then
        if duel::get_matching_group_count(d, Some(&card::is_spell), a.tp, LOCATION_GRAVE, 0, NO_EX) >= 3 {
            // local e3=Effect.CreateEffect(e:GetHandler())
            let e3 = effect::create_effect(d, h);
            // e3:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            // e3:SetCode(EFFECT_SET_ATTACK)
            effect::set_code(d, e3, EFFECT_SET_ATTACK);
            // e3:SetValue(1500)
            effect::set_value(d, e3, 1500);
            // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
            // token:RegisterEffect(e3)
            card::register_effect(d, token, e3, false);
            // local e4=e3:Clone()
            let e4 = effect::clone(d, e3);
            // e4:SetCode(EFFECT_SET_DEFENSE)
            effect::set_code(d, e4, EFFECT_SET_DEFENSE);
            // token:RegisterEffect(e4)
            card::register_effect(d, token, e4, false);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
