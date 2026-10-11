//! Wake Up Centur-Ion!
use super::prelude::*;

pub const ID: u32 = 92907248;

pub const TOKEN_CENTURION: u32 = ID + 1;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CENTUR_ION], listed_names: &[TOKEN_CENTURION, ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Centur-Ion Token"
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
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.tokencond)
    effect::set_condition(d, e1, Some(condition(tokencond)));
    // e1:SetTarget(s.tokentg)
    effect::set_target(d, e1, Some(cost_target(tokentg)));
    // e1:SetOperation(s.tokenop)
    effect::set_operation(d, e1, Some(operation(tokenop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Send 1 "Centur-Ion" card from your Deck to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.tgytg)
    effect::set_target(d, e2, Some(cost_target(tgytg)));
    // e2:SetOperation(s.tgyop)
    effect::set_operation(d, e2, Some(operation(tgyop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsOriginalType(TYPE_MONSTER)
    card::is_faceup(d, c) && card::is_original_type(d, c, TYPE_MONSTER)
}
/// `s.tokencond(e,tp,eg,ep,ev,re,r,rp)`
fn tokencond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_STZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_STZONE, 0, 1, NO_EX)
}
/// `s.tokentg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tokentg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local lv4=Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_CENTURION,SET_CENTUR_ION,TYPES_TOKEN,0,0,4,RACE_PYRO,ATTRIBUTE_DARK,POS_FACEUP)
    let lv4 = duel::is_player_can_special_summon_monster(d, tp, TOKEN_CENTURION, token_data(4), Some(POS_FACEUP as u8), None, 0);
    // local lv8=Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_CENTURION,SET_CENTUR_ION,TYPES_TOKEN,0,0,8,RACE_PYRO,ATTRIBUTE_DARK,POS_FACEUP)
    let lv8 = duel::is_player_can_special_summon_monster(d, tp, TOKEN_CENTURION, token_data(8), Some(POS_FACEUP as u8), None, 0);
    // if chk==0 then return (lv4 or lv8) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end
    if a.chk == Some(0) {
        return (lv4 || lv8) && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    // local levels={}
    let mut levels: Vec<u64> = Vec::new();
    // if lv4 then table.insert(levels,4) end
    if lv4 {
        levels.push(4);
    }
    // if lv8 then table.insert(levels,8) end
    if lv8 {
        levels.push(8);
    }
    // local lvl=Duel.AnnounceNumber(tp,levels)
    let lvl = duel::announce_number(d, tp, &levels).nil().0;
    // e:SetLabel(lvl)
    effect::set_label(d, e, &[lvl as i64]);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    true
}
/// The token's data for `Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_CENTURION,SET_CENTUR_ION,TYPES_TOKEN,0,0,lv,RACE_PYRO,ATTRIBUTE_DARK,POS_FACEUP)`.
fn token_data(lv: u32) -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_CENTUR_ION as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(lv),
        race: Some(RACE_PYRO),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
/// `s.tokenop(e,tp,eg,ep,ev,re,r,rp)`
fn tokenop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local lvl=e:GetLabel()
    let lvl = effect::get_label(d, e) as u32;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0
    //     or not Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_CENTURION,SET_CENTUR_ION,TYPES_TOKEN,0,0,lvl,RACE_PYRO,ATTRIBUTE_DARK,POS_FACEUP) then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0
        || !duel::is_player_can_special_summon_monster(d, tp, TOKEN_CENTURION, token_data(lvl), Some(POS_FACEUP as u8), None, 0)
    {
        return;
    }
    // local token=Duel.CreateToken(tp,TOKEN_CENTURION)
    let token = duel::create_token(d, tp, TOKEN_CENTURION).nil();
    // Set the Token's Level
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EFFECT_CHANGE_LEVEL)
    effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
    // e1:SetValue(lvl)
    effect::set_value(d, e1, lvl as i64);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD&~RESET_TOFIELD)
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
    // token:RegisterEffect(e1)
    card::register_effect(d, token, e1, false);
    // if Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP) then
    if duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
        // Cannot be used as Fusion material
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetDescription(3309)
        effect::set_description(d, e2, 3309);
        // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e2:SetCode(EFFECT_CANNOT_BE_FUSION_MATERIAL)
        effect::set_code(d, e2, EFFECT_CANNOT_BE_FUSION_MATERIAL);
        // e2:SetValue(1)
        effect::set_value(d, e2, 1);
        // token:RegisterEffect(e2,true)
        card::register_effect(d, token, e2, true);
        // Cannot be used as Link material
        // local e3=e2:Clone()
        let e3 = effect::clone(d, e2);
        // e3:SetDescription(3312)
        effect::set_description(d, e3, 3312);
        // e3:SetCode(EFFECT_CANNOT_BE_LINK_MATERIAL)
        effect::set_code(d, e3, EFFECT_CANNOT_BE_LINK_MATERIAL);
        // token:RegisterEffect(e3,true)
        card::register_effect(d, token, e3, true);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.tgyfilter(c)`
fn tgyfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_CENTUR_ION) and not c:IsCode(id) and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_CENTUR_ION as u16]) && !card::is_code(d, c, &[ID]) && card::is_able_to_grave(d, c, None)
}
/// `s.tgytg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgytg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgyfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgyfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.tgyop(e,tp,eg,ep,ev,re,r,rp)`
fn tgyop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgyfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&tgyfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, &g, REASON_EFFECT, None, None);
    }
}
