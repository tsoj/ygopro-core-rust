//! Motor Frenzy
use super::prelude::*;

pub const ID: u32 = 77672444;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_ENGINE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Grant ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(cost_target(atktg)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon tokens
    // local e2=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e2:SetCondition(s.tkcon)
    effect::set_condition(d, e3, Some(condition(tkcon)));
    // e2:SetTarget(s.tktg)
    effect::set_target(d, e3, Some(cost_target(tktg)));
    // e2:SetOperation(s.tkop)
    effect::set_operation(d, e3, Some(operation(tkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e3, false);
}
/// `s.atkconfilter(c,ec,tp)`
fn atkconfilter(d: &mut Duel, c: CardId, ec: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and (ec==c or c:IsRace(RACE_MACHINE)) and c:IsControler(tp) and c:GetBaseDefense()>0
    card::is_faceup(d, c)
        && (ec == c || card::is_race(d, c, RACE_MACHINE as u64))
        && card::is_controler(d, c, tp)
        && card::get_base_defense(d, c) > 0
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.atkconfilter,1,nil,e:GetHandler(),tp)
    let h = effect::get_handler(d, a.e).nil();
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| atkconfilter(d, c, h, a.tp), 1, NO_EX)
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    // if chk==0 then return eg:IsExists(Card.IsLocation,1,nil,LOCATION_MZONE) end
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, c| card::is_location(d, c, LOCATION_MZONE), 1, NO_EX);
    }
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, eg);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local g=eg:Filter(Card.IsRelateToEffect,nil,e)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // if tc:IsFaceup() and tc:GetBaseDefense()>0 then
        if card::is_faceup(d, tc) && card::get_base_defense(d, tc) > 0 {
            // Increase ATK
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetReset(RESETS_STANDARD_PHASE_END|RESET_OPPO_TURN)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END | RESET_OPPO_TURN, 1);
            // e1:SetValue(tc:GetBaseDefense()/2)
            let bd = card::get_base_defense(d, tc);
            effect::set_value(d, e1, lua_int(bd as f64 / 2.0));
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
        // Cannot change battle position
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3313)
        effect::set_description(d, e1, 3313);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CANNOT_CHANGE_POSITION)
        effect::set_code(d, e1, EFFECT_CANNOT_CHANGE_POSITION);
        // e1:SetReset(RESETS_STANDARD_PHASE_END|RESET_OPPO_TURN)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END | RESET_OPPO_TURN, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.tkcon(e,tp,eg,ep,ev,re,r,rp)`
fn tkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsTributeSummoned()
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_tribute_summoned(d, c)
}
/// Engine token data for `Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_ENGINE,0,TYPES_TOKEN,200,200,1,RACE_MACHINE,ATTRIBUTE_EARTH,POS_FACEUP_ATTACK)`.
fn engine_token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(200),
        defense: Some(200),
        level: Some(1),
        race: Some(RACE_MACHINE),
        attribute: Some(ATTRIBUTE_EARTH),
    }
}
/// The `chk == 0` test of `s.tktg`, which `s.tkop` also runs.
fn token_check(d: &mut Duel, tp: u8) -> bool {
    // return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>1
    //     and Duel.IsPlayerCanSpecialSummonMonster(...)
    !duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 1
        && duel::is_player_can_special_summon_monster(d, tp, TOKEN_ENGINE, engine_token_data(), Some(POS_FACEUP_ATTACK as u8), None, 0)
}
/// `s.tktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then ... end
    if a.chk == Some(0) {
        return token_check(d, a.tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 2, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, 0);
    true
}
/// `s.tkop(e,tp,eg,ep,ev,re,r,rp)`
fn tkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not s.tktg(e,tp,eg,ep,ev,re,r,rp,0) then return end
    if !token_check(d, tp) {
        return;
    }
    // for i=1,2 do
    for _i in 1..=2 {
        // local token=Duel.CreateToken(tp,TOKEN_ENGINE)
        let token = duel::create_token(d, tp, TOKEN_ENGINE).nil();
        // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP_ATTACK)
        duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP_ATTACK as u8, None);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
