//! Noble Knight Iyvanne
use super::prelude::*;

pub const ID: u32 = 88923963;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[88923964], listed_series: &[SET_NOBLE_KNIGHT, SET_NOBLE_ARMS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // token
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_EQUIP)
    effect::set_code(d, e1, EVENT_EQUIP);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.tkcon)
    effect::set_condition(d, e1, Some(condition(tkcon)));
    // e1:SetTarget(s.tktg)
    effect::set_target(d, e1, Some(cost_target(tktg)));
    // e1:SetOperation(s.tkop)
    effect::set_operation(d, e1, Some(operation(tkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atk up
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(lua_fn(|d, a| atkcon(d, a.e(0)))));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(effect_card_fn(atktg)));
    // e2:SetValue(500)
    effect::set_value(d, e2, 500);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// s.tkcon(e,tp,eg,ep,ev,re,r,rp)
fn tkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(Card.IsSetCard,1,nil,SET_NOBLE_ARMS)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, x| card::is_set_card(d, x, &[SET_NOBLE_ARMS as u16]), 1, NO_EX)
}
// s.tktg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_NOBLE_KNIGHT,TYPES_TOKEN,1000,1000,4,RACE_WARRIOR,ATTRIBUTE_LIGHT) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 0, 0);
    true
}
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_NOBLE_KNIGHT as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(1000),
        defense: Some(1000),
        level: Some(4),
        race: Some(RACE_WARRIOR),
        attribute: Some(ATTRIBUTE_LIGHT),
    }
}
// s.tkop(e,tp,eg,ep,ev,re,r,rp)
fn tkop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsPlayerCanSpecialSummonMonster(...) then
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), None, None, 0)
    {
        // local token=Duel.CreateToken(tp,id+1)
        let token = duel::create_token(d, a.tp, ID + 1).nil();
        // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
// s.splimit(e,c,sump,sumtype,sumpos,targetp,se)
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsSetCard(SET_NOBLE_KNIGHT)
    !card::is_set_card(d, c, &[SET_NOBLE_KNIGHT as u16])
}
// s.atkcon(e)
fn atkcon(d: &mut Duel, e: EffectId) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local eg=c:GetEquipGroup()
    let eg = card::get_equip_group(d, c);
    // return #eg>0 and eg:IsExists(Card.IsSetCard,1,nil,SET_NOBLE_ARMS)
    !eg.is_empty() && group::is_exists(d, &eg, &|d, x| card::is_set_card(d, x, &[SET_NOBLE_ARMS as u16]), 1, NO_EX)
}
// s.atktg(e,c)
fn atktg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_NOBLE_KNIGHT) and c~=e:GetHandler()
    let h = effect::get_handler(d, e).nil();
    card::is_set_card(d, c, &[SET_NOBLE_KNIGHT as u16]) && c != h
}
