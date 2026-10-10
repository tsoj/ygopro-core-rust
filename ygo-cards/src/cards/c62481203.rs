//! Trickstar Festival
use super::prelude::*;

pub const ID: u32 = 62481203;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_TRICKSTAR], listed_series: &[SET_TRICKSTAR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy replace
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
    // e2:SetValue(s.repval)
    effect::set_value_fn(d, e2, lua_fn(repval));
    // e2:SetOperation(s.repop)
    effect::set_operation(d, e2, Some(operation(repop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SUMMON,s.counterfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SUMMON, lua_fn(counterfilter));
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,s.counterfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(counterfilter));
}
/// `s.counterfilter(c)`
fn counterfilter(d: &mut Duel, a: Args) -> bool {
    // return c:IsSetCard(SET_TRICKSTAR)
    card::is_set_card(d, a.c(0), &[SET_TRICKSTAR as u16])
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_SUMMON)==0
    //     and Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0 end
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_SUMMON) == 0 && duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_SPSUMMON) == 0;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SUMMON);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    // aux.RegisterClientHint(e:GetHandler(),nil,tp,1,0,aux.Stringid(id,1),nil)
    aux::register_client_hint(d, h, 0, a.tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    true
}
/// `s.splimit(e,c)`
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsSetCard(SET_TRICKSTAR)
    !card::is_set_card(d, c, &[SET_TRICKSTAR as u16])
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and Duel.GetLocationCount(tp,LOCATION_MZONE)>1
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_TRICKSTAR,SET_TRICKSTAR,TYPES_TOKEN,0,0,1,RACE_FAIRY,ATTRIBUTE_LIGHT) end
    if a.chk == Some(0) {
        return !duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some()
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            && duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_TRICKSTAR, token_data(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 2, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, 0);
    true
}
/// The Trickstar token: `TOKEN_TRICKSTAR`, `SET_TRICKSTAR`, `TYPES_TOKEN`, 0, 0, 1, `RACE_FAIRY`, `ATTRIBUTE_LIGHT`
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_TRICKSTAR as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(1),
        race: Some(RACE_FAIRY),
        attribute: Some(ATTRIBUTE_LIGHT),
    }
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and Duel.GetLocationCount(tp,LOCATION_MZONE)>1
    //     and Duel.IsPlayerCanSpecialSummonMonster(...) then
    if !duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some()
        && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 1
        && duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_TRICKSTAR, token_data(), None, None, 0)
    {
        // for i=1,2 do
        for _ in 0..2 {
            // local token=Duel.CreateToken(tp,TOKEN_TRICKSTAR)
            let token = duel::create_token(d, a.tp, TOKEN_TRICKSTAR).nil();
            // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_TRICKSTAR) and c:IsLocation(LOCATION_MZONE) and c:IsSummonLocation(LOCATION_EXTRA)
    //     and c:IsControler(tp) and c:IsReason(REASON_EFFECT|REASON_BATTLE) and not c:IsReason(REASON_REPLACE)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_TRICKSTAR as u16])
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_summon_location(d, c, LOCATION_EXTRA)
        && card::is_controler(d, c, tp)
        && card::is_reason(d, c, REASON_EFFECT | REASON_BATTLE)
        && !card::is_reason(d, c, REASON_REPLACE)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsAbleToRemove() and eg:IsExists(s.repfilter,1,nil,tp) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        let eg = a.eg.clone().nil();
        return card::is_able_to_remove(d, h, None, None, None) && group::is_exists(d, &eg, &|d, c| repfilter(d, c, a.tp), 1, NO_EX);
    }
    // return Duel.SelectEffectYesNo(tp,e:GetHandler(),96)
    let h = effect::get_handler(d, a.e).nil();
    duel::select_effect_yes_no(d, a.tp, h, Some(96))
}
/// `s.repval(e,c)`
fn repval(d: &mut Duel, a: Args) -> bool {
    // return s.repfilter(c,e:GetHandlerPlayer())
    let p = effect::get_handler_player(d, a.e(0));
    repfilter(d, a.c(1), p)
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: &EffArgs) {
    // Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::remove(d, h, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
