//! Battle of Sleeping Spirits
use super::prelude::*;

pub const ID: u32 = 46570372;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_NEOS],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_BATTLE_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_BATTLE_END));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter(c,e,tp,turn)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, turn: u32) -> bool {
    // return c:IsReason(REASON_BATTLE) and c:GetTurnID()==turn
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_ATTACK)
    card::is_reason(d, c, REASON_BATTLE)
        && card::get_turn_id(d, c) == turn
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_ATTACK as u8), None, None)
}
/// `s.cansummon(e,p,turn)`
fn cansummon(d: &mut Duel, e: EffectId, p: u8, turn: u32) -> bool {
    // return Duel.GetLocationCount(p,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,p,LOCATION_GRAVE,0,1,nil,e,p,turn)
    duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, p, turn)), p, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local turn=Duel.GetTurnCount()
        let turn = duel::get_turn_count(d, None) as u32;
        // return s.cansummon(e,tp,turn) or s.cansummon(e,1-tp,turn)
        return cansummon(d, a.e, a.tp, turn) || cansummon(d, a.e, 1 - a.tp, turn);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,PLAYER_ALL,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, PLAYER_ALL as u8, LOCATION_GRAVE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local turn_p=Duel.GetTurnPlayer()
    let turn_p = duel::get_turn_player(d);
    // local turn=Duel.GetTurnCount()
    let turn = duel::get_turn_count(d, None) as u32;
    // s.spsummon(e,turn_p,turn,tp)
    spsummon(d, a.e, turn_p, turn, a.tp);
    // s.spsummon(e,1-turn_p,turn,tp)
    spsummon(d, a.e, 1 - turn_p, turn, a.tp);
}
/// `s.spsummon(e,p,turn,tp)`
fn spsummon(d: &mut Duel, e: EffectId, p: u8, turn: u32, tp: u8) {
    // if not s.cansummon(e,p,turn) or not Duel.SelectYesNo(p,aux.Stringid(id,1)) then return end
    if !cansummon(d, e, p, turn) || !duel::select_yes_no(d, p, aux::stringid(ID, 1)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(p,s.spfilter,p,LOCATION_GRAVE,0,1,1,nil,e,p,turn):GetFirst()
    let sel = duel::select_matching_card(d, p, Some(&|d, c| spfilter(d, c, e, p, turn)), p, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc and Duel.SpecialSummon(tc,0,p,p,false,false,POS_FACEUP_ATTACK)>0 and p==tp and tc:IsCode(CARD_NEOS) then
    if let Some(tc) = tc {
        if duel::special_summon(d, tc, 0, p, p, false, false, POS_FACEUP_ATTACK as u8, None) > 0 && p == tp && card::is_code(d, tc, &[CARD_NEOS]) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
            // e1:SetCode(EVENT_BATTLE_START)
            effect::set_code(d, e1, EVENT_BATTLE_START);
            // e1:SetCountLimit(1)
            effect::set_count_limit(d, e1, 1, 0, 0, 0);
            // e1:SetOperation(s.atkop)
            effect::set_operation(d, e1, Some(operation(atkop)));
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetHandler()
    let tc = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(e:GetOwner())
    let owner = effect::get_owner(d, a.e).nil();
    let e1 = effect::create_effect(d, owner);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
    effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(tc:GetAttack()*2)
    let atk = card::get_attack(d, tc) as i64 * 2;
    effect::set_value(d, e1, atk);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
}
