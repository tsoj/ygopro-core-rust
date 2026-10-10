//! Mithra the Thunder Vassal
use super::prelude::*;

pub const ID: u32 = 22404675;
/// `local TOKEN_VASSAL=id+1`
const TOKEN_VASSAL: u32 = ID + 1;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_VASSAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card, and if you do, Special Summon 1 "Vassal Token" (Thunder/LIGHT/Level 1/ATK 800/DEF 1000) to your opponent's field in Defense Position
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Apply a "you can Tribute Summon 1 monster during your Main Phase this turn, in addition to your Normal Summon/Set" effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_RELEASE)
    effect::set_code(d, e2, EVENT_RELEASE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp) return e:GetHandler():IsReason(REASON_SUMMON) and Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_reason(d, h, REASON_SUMMON) && duel::is_turn_player(d, a.tp)
    })));
    // e2:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return Duel.IsPlayerCanSummon(tp) and Duel.IsPlayerCanAdditionalTributeSummon(tp) end end)
    effect::set_target(d, e2, Some(cost_target(|d, a| {
        if a.chk == Some(0) {
            return duel::is_player_can_summon(d, a.tp, None) && duel::is_player_can_additional_tribute_summon(d, a.tp);
        }
        true
    })));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// The Vassal Token's data: `TYPES_TOKEN,800,1000,1,RACE_THUNDER,ATTRIBUTE_LIGHT`
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(800),
        defense: Some(1000),
        level: Some(1),
        race: Some(RACE_THUNDER),
        attribute: Some(ATTRIBUTE_LIGHT),
    }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        // return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
        //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0
        //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
        //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_VASSAL,0,TYPES_TOKEN,800,1000,1,RACE_THUNDER,ATTRIBUTE_LIGHT,POS_FACEUP_DEFENSE,1-tp)
        return duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            && duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_VASSAL, token_data(), Some(POS_FACEUP_DEFENSE as u8), Some(1 - a.tp), 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 2, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.SpecialSummonStep(c,0,tp,tp,false,false,POS_FACEUP)
    //     and Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_VASSAL,0,TYPES_TOKEN,800,1000,1,RACE_THUNDER,ATTRIBUTE_LIGHT,POS_FACEUP_DEFENSE,1-tp) then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
        && duel::special_summon_step(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None)
        && duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, Some(a.tp), None, None).0 > 0
        && duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_VASSAL, token_data(), Some(POS_FACEUP_DEFENSE as u8), Some(1 - a.tp), 0)
    {
        // local token=Duel.CreateToken(tp,TOKEN_VASSAL)
        let token = duel::create_token(d, a.tp, TOKEN_VASSAL).nil();
        // Duel.SpecialSummonStep(token,0,tp,1-tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon_step(d, token, 0, a.tp, 1 - a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // You cannot Special Summon from the Extra Deck for the rest of this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // "Clock Lizard" check
    // aux.addTempLizardCheck(c,tp)
    aux::add_temp_lizard_check(d, c, a.tp, None, None, None, None, None);
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // if not Duel.IsPlayerCanAdditionalTributeSummon(tp) then return end
    if !duel::is_player_can_additional_tribute_summon(d, a.tp) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,3))
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 3)), 0, None);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,4))
    effect::set_description(d, e1, aux::stringid(ID, 4));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_EXTRA_SUMMON_COUNT)
    effect::set_code(d, e1, EFFECT_EXTRA_SUMMON_COUNT);
    // e1:SetTargetRange(LOCATION_HAND,0)
    effect::set_target_range(d, e1, LOCATION_HAND, 0);
    // e1:SetValue(0x1)
    effect::set_value(d, e1, 0x1);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_EXTRA_SET_COUNT)
    effect::set_code(d, e2, EFFECT_EXTRA_SET_COUNT);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
}
