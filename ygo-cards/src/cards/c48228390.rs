//! Pyrite Knight
//! 黄鉄の愚騎士
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 48228390;
/// `local TOKEN_PYRITE=id+1`
const TOKEN_PYRITE: u32 = ID + 1;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[ID, TOKEN_PYRITE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Return 1 EARTH or FIRE Warrior monster to the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_GRAVE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);

    // Special Summon 1 Warrior monster from your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetHintTiming(0,TIMING_BATTLE_START|TIMING_BATTLE_END)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_BATTLE_START | TIMING_BATTLE_END));
    // e2:SetCondition(function() return Duel.IsBattlePhase() end)
    effect::set_condition(d, e2, Some(condition(|d, _a| duel::is_battle_phase(d, None))));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thfilter(c,tp)`
fn thfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_EARTH|ATTRIBUTE_FIRE) and c:IsRace(RACE_WARRIOR)
    //     and c:IsFaceup() and c:IsAbleToHand() and Duel.GetMZoneCount(tp,c)>0
    card::is_attribute(d, c, ATTRIBUTE_EARTH | ATTRIBUTE_FIRE)
        && card::is_race(d, c, RACE_WARRIOR)
        && card::is_faceup(d, c)
        && card::is_able_to_hand(d, c, None)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
/// The token of this card: `Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_PYRITE,0,TYPES_TOKEN,0,2000,4,RACE_WARRIOR,ATTRIBUTE_FIRE,POS_FACEUP_DEFENSE,1-tp)`
fn can_summon_token(d: &mut Duel, tp: u8) -> bool {
    let data = duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(2000),
        level: Some(4),
        race: Some(RACE_WARRIOR),
        attribute: Some(ATTRIBUTE_FIRE),
    };
    duel::is_player_can_special_summon_monster(d, tp, TOKEN_PYRITE, data, Some(POS_FACEUP_DEFENSE as u8), Some(1 - tp), 0)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.thfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && thfilter(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_MZONE,0,1,nil,tp)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(...) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| thfilter(d, x, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            && duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && can_summon_token(d, a.tp);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, x| thfilter(d, x, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if tc:IsRelateToEffect(e) and Duel.SendtoHand(tc,nil,REASON_EFFECT)>0
    if !card::is_relate_to_effect(d, tc, a.e) || duel::send_to_hand(d, tc, None, REASON_EFFECT, None) <= 0 {
        return;
    }
    // and tc:IsLocation(LOCATION_HAND) and c:IsRelateToEffect(e)
    if !card::is_location(d, tc, LOCATION_HAND) || !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0
    if duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) <= 0 {
        return;
    }
    // and Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0
    if duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // and Duel.IsPlayerCanSpecialSummonMonster(...) then
    if !can_summon_token(d, a.tp) {
        return;
    }
    // local token=Duel.CreateToken(tp,TOKEN_PYRITE)
    let token = duel::create_token(d, a.tp, TOKEN_PYRITE).nil();
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.SpecialSummon(token,0,tp,1-tp,false,false,POS_FACEUP_DEFENSE)
    duel::special_summon(d, token, 0, a.tp, 1 - a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_WARRIOR) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_WARRIOR) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
