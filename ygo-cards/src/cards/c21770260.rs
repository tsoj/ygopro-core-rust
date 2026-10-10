//! Jam Breeding Machine
use super::prelude::*;

pub const ID: u32 = 21770260;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_SLIME], ..ScriptInfo::DEFAULT };

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
    // special summon token
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // summon limit
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e3, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // e3:SetTarget(s.sumlimit)
    effect::set_target(d, e3, Some(lua_fn(sumlimit)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e4, EFFECT_CANNOT_SUMMON);
    // e4:SetTargetRange(1,0)
    effect::set_target_range(d, e4, 1, 0);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EFFECT_CANNOT_FLIP_SUMMON)
    effect::set_code(d, e5, EFFECT_CANNOT_FLIP_SUMMON);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.sumlimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn sumlimit(d: &mut Duel, a: Args) -> bool {
    // return not c:IsCode(TOKEN_SLIME)
    !card::is_code(d, a.c(1), &[TOKEN_SLIME])
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    true
}
/// `TOKEN` data of `Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_SLIME,0x54b,TYPES_TOKEN,500,500,1,RACE_AQUA,ATTRIBUTE_WATER)`
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0x54b]),
        type_: Some(TYPES_TOKEN),
        attack: Some(500),
        defense: Some(500),
        level: Some(1),
        race: Some(RACE_AQUA),
        attribute: Some(ATTRIBUTE_WATER),
    }
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_SLIME,0x54b,TYPES_TOKEN,500,500,1,RACE_AQUA,ATTRIBUTE_WATER) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e)
        && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_SLIME, token_data(), None, None, 0)
    {
        // local token=Duel.CreateToken(tp,TOKEN_SLIME)
        let token = duel::create_token(d, a.tp, TOKEN_SLIME).nil();
        // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP_ATTACK)
        duel::special_summon(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP_ATTACK as u8, None);
    }
}
