//! Void Expansion
use super::prelude::*;

pub const ID: u32 = 34822850;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID + 1], listed_series: &[SET_INFERNOID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Infernoid Token" during your Standby Phase
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e2, Some(condition(|d, a| duel::is_turn_player(d, a.tp))));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Custom effect for cost modification
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetTargetRange(LOCATION_HAND|LOCATION_GRAVE,0)
    effect::set_target_range(d, e3, LOCATION_HAND | LOCATION_GRAVE, 0);
    // e3:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_INFERNOID))
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_INFERNOID as u16]))));
    // e3:SetCode(id)
    effect::set_code(d, e3, ID);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // "Infernoid" monsters, except the ones with the highest level, cannot be targeted by opponent's effects or battle
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e4, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e4, 0, LOCATION_MZONE);
    // e4:SetValue(s.atlimit)
    effect::set_value_fn(d, e4, effect_card_fn(atlimit));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    // e5:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e5, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e5:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e5, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e5:SetRange(LOCATION_FZONE)
    effect::set_range(d, e5, LOCATION_FZONE);
    // e5:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e5, LOCATION_MZONE, 0);
    // e5:SetTarget(s.tglimit)
    effect::set_target(d, e5, Some(effect_card_fn(tglimit)));
    // e5:SetValue(aux.tgoval)
    effect::set_value_fn(d, e5, aux::tgoval());
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_INFERNOID,TYPES_TOKEN,0,0,1,RACE_FIEND,ATTRIBUTE_FIRE) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && can_summon_token(d, tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    true
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_INFERNOID,TYPES_TOKEN,0,0,1,RACE_FIEND,ATTRIBUTE_FIRE)`
fn can_summon_token(d: &mut Duel, tp: u8) -> bool {
    duel::is_player_can_special_summon_monster(
        d,
        tp,
        ID + 1,
        duel::MonsterData {
            setcodes: Some(vec![SET_INFERNOID as u16]),
            type_: Some(TYPES_TOKEN),
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
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0
    //     or not Duel.IsPlayerCanSpecialSummonMonster(...) then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 || !can_summon_token(d, tp) {
        return;
    }
    // local token=Duel.CreateToken(tp,id+1)
    let token = duel::create_token(d, tp, ID + 1).nil();
    // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
}
/// `s.filter(c,lv)`
fn filter(d: &mut Duel, c: CardId, lv: i64) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_INFERNOID) and c:GetLevel()>lv
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_INFERNOID as u16]) && card::get_level(d, c) as i64 > lv
}
/// `s.atlimit(e,c)`
fn atlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_INFERNOID) and (not c:HasLevel()
    //     or Duel.IsExistingMatchingCard(s.filter,c:GetControler(),LOCATION_MZONE,0,1,nil,c:GetLevel()))
    let _ = e;
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_INFERNOID as u16])
        && (!card::has_level(d, c) || {
            let p = card::get_controler(d, c);
            let lv = card::get_level(d, c) as i64;
            duel::is_existing_matching_card(d, Some(&|d, cc| filter(d, cc, lv)), p, LOCATION_MZONE, 0, 1, NO_EX)
        })
}
/// `s.tglimit(e,c)`
fn tglimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_INFERNOID) and (not c:HasLevel()
    //     or Duel.IsExistingMatchingCard(s.filter,c:GetControler(),LOCATION_MZONE,0,1,nil,c:GetLevel()))
    card::is_set_card(d, c, &[SET_INFERNOID as u16])
        && (!card::has_level(d, c) || {
            let p = card::get_controler(d, c);
            let lv = card::get_level(d, c) as i64;
            duel::is_existing_matching_card(d, Some(&|d, cc| filter(d, cc, lv)), p, LOCATION_MZONE, 0, 1, NO_EX)
        })
}
