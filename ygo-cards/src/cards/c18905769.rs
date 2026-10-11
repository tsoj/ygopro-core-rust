//! Guivre
use super::prelude::*;

pub const ID: u32 = 18905769;
pub const TOKEN_WYVERN: u32 = ID + 1;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID + 1], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Gains 400 ATK for each Dragon monster you control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(function(e,c) return 400*Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsRace,RACE_DRAGON),e:GetHandlerPlayer(),LOCATION_MZONE,0,nil) end)
    effect::set_value_fn(
        d,
        e1,
        effect_card_value(|d, e, _c| {
            let p = effect::get_handler_player(d, e);
            400 * duel::get_matching_group_count(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_DRAGON)), p, LOCATION_MZONE, 0, NO_EX) as i64
        }),
    );
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // When this card destroys a monster by battle: You can Special Summon 1 "Wyvern Token" (Dragon/LIGHT/Level 1/ATK 400/DEF 400)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    // e2:SetTarget(s.tokentg)
    effect::set_target(d, e2, Some(cost_target(tokentg)));
    // e2:SetOperation(s.tokenop)
    effect::set_operation(d, e2, Some(operation(tokenop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // During the Battle Phase or Main Phase 2, you can (Quick Effect): Immediately after this effect resolves, Tribute Summon 1 Dragon monster face-up. You can only use this effect of "Guivre" once per turn
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SUMMON)
    effect::set_category(d, e3, CATEGORY_SUMMON);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(function() return Duel.IsBattlePhase() or Duel.IsMainPhase2() end)
    effect::set_condition(d, e3, Some(lua_fn(cond3)));
    // e3:SetTarget(s.tribsumtg)
    effect::set_target(d, e3, Some(cost_target(tribsumtg)));
    // e3:SetOperation(s.tribsumop)
    effect::set_operation(d, e3, Some(operation(tribsumop)));
    // e3:SetHintTiming(0,TIMING_BATTLE_END|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e3, 0, Some(TIMING_BATTLE_END | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function() return Duel.IsBattlePhase() or Duel.IsMainPhase2() end` (the condition of e3)
fn cond3(d: &mut Duel, _a: Args) -> bool {
    duel::is_battle_phase(d, None) || duel::is_main_phase2(d, None)
}
/// `s.tokentg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tokentg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_WYVERN,0,TYPES_TOKEN,400,400,1,RACE_DRAGON,ATTRIBUTE_LIGHT) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, tp, TOKEN_WYVERN, token_data(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    true
}
/// `s.tokenop(e,tp,eg,ep,ev,re,r,rp)`
fn tokenop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_WYVERN,0,TYPES_TOKEN,400,400,1,RACE_DRAGON,ATTRIBUTE_LIGHT) then
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_player_can_special_summon_monster(d, tp, TOKEN_WYVERN, token_data(), None, None, 0)
    {
        // local token=Duel.CreateToken(tp,TOKEN_WYVERN)
        let token = duel::create_token(d, tp, TOKEN_WYVERN).nil();
        // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.tribsumfilter(c)`
fn tribsumfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:IsSummonable(true,nil,1)
    card::is_race(d, c, RACE_DRAGON) && card::is_summonable(d, c, true, None, 1, None)
}
/// `s.tribsumtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tribsumtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tribsumfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tribsumfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.tribsumop(e,tp,eg,ep,ev,re,r,rp)`
fn tribsumop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.tribsumfilter,tp,LOCATION_HAND,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&tribsumfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if sc then
    if let Some(sc) = sc {
        // Duel.Summon(tp,sc,true,nil,1)
        duel::summon(d, tp, sc, true, None, 1, None);
    }
}
/// `s.token_data`: the Wyvern token (`TOKEN_WYVERN`, `TYPES_TOKEN`, 400/400, level 1, `RACE_DRAGON`, `ATTRIBUTE_LIGHT`)
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(400),
        defense: Some(400),
        level: Some(1),
        race: Some(RACE_DRAGON),
        attribute: Some(ATTRIBUTE_LIGHT),
        ..Default::default()
    }
}
