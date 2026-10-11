//! Hippo Carnival
use super::prelude::*;

pub const ID: u32 = 18027138;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_HIPPO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon 3 tokens to your field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON | CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE as u32));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>2
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,0,0,1,RACE_BEAST,ATTRIBUTE_EARTH) end
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 2
            && duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,3,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 3, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,3,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 3, a.tp, 0);
    true
}
/// The token data of `TYPES_TOKEN, 0, 0, 1, RACE_BEAST, ATTRIBUTE_EARTH`.
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(1),
        race: Some(RACE_BEAST),
        attribute: Some(ATTRIBUTE_EARTH),
    }
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and Duel.GetLocationCount(tp,LOCATION_MZONE)>2
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,0,0,1,RACE_BEAST,ATTRIBUTE_EARTH) then
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
        && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 2
        && duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), None, None, 0)
    {
        // for i=1,3 do
        for i in 1..=3u32 {
            // local token=Duel.CreateToken(tp,id+i)
            let token = duel::create_token(d, a.tp, ID + i).nil();
            // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
            // Cannot be tributed
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(3303)
            effect::set_description(d, e1, 3303);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UNRELEASABLE_SUM)
            effect::set_code(d, e1, EFFECT_UNRELEASABLE_SUM);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetValue(1)
            effect::set_value(d, e1, 1);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, (RESET_EVENT | RESETS_STANDARD) as u32, 1);
            // token:RegisterEffect(e1)
            card::register_effect(d, token, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_UNRELEASABLE_NONSUM)
            effect::set_code(d, e2, EFFECT_UNRELEASABLE_NONSUM);
            // token:RegisterEffect(e2)
            card::register_effect(d, token, e2, false);
            // Cannot special summon from extra deck while you control the token
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e3, EFFECT_TYPE_FIELD);
            // e3:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
            effect::set_code(d, e3, EFFECT_CANNOT_SPECIAL_SUMMON);
            // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
            effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
            // e3:SetRange(LOCATION_MZONE)
            effect::set_range(d, e3, LOCATION_MZONE);
            // e3:SetAbsoluteRange(tp,1,0)
            effect::set_absolute_range(d, e3, a.tp, 1, 0);
            // e3:SetTarget(aux.TargetBoolFunction(Card.IsLocation,LOCATION_EXTRA))
            effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA))));
            // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e3, (RESET_EVENT | RESETS_STANDARD) as u32, 1);
            // token:RegisterEffect(e3)
            card::register_effect(d, token, e3, false);
            // Clock Lizard check
            // local e4=aux.createContinuousLizardCheck(e:GetHandler(),LOCATION_MZONE)
            let e4 = aux::create_continuous_lizard_check(d, c, LOCATION_MZONE, None, None, None);
            // e4:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e4, (RESET_EVENT | RESETS_STANDARD) as u32, 1);
            // token:RegisterEffect(e4,true)
            card::register_effect(d, token, e4, true);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetValue(aux.NOT(aux.TargetBoolFunction(Card.IsCode,TOKEN_HIPPO)))
    effect::set_value_fn(d, e1, effect_card_fn(|d, _e, c| !card::is_code(d, c, &[TOKEN_HIPPO])));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, (RESET_PHASE | PHASE_END) as u32, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
