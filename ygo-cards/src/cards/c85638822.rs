//! Gouki Cage Match
use super::prelude::*;

pub const ID: u32 = 85638822;

const COUNTER_GOUKI_CAGE_MATCH: u16 = 0x46;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GOUKI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_GOUKI_CAGE_MATCH)
    card::enable_counter_permit(d, c, COUNTER_GOUKI_CAGE_MATCH, None, None);
    // When this card is activated: Place 3 counters on this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If your "Gouki" monster destroys an opponent's monster by battle: Remove 1 counter from this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCondition(s.rmvcountercon)
    effect::set_condition(d, e2, Some(condition(rmvcountercon)));
    // e2:SetOperation(s.rmvcounterop)
    effect::set_operation(d, e2, Some(operation(rmvcounterop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Once per turn, at the end of the Battle Phase, if the last of these counters has been removed this way: ...
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_PHASE+PHASE_BATTLE)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_BATTLE);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsCanAddCounter(tp,COUNTER_GOUKI_CAGE_MATCH,3,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_can_add_counter(d, a.tp, Some((COUNTER_GOUKI_CAGE_MATCH, 3, h)));
    }
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,3,tp,COUNTER_GOUKI_CAGE_MATCH)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, 3, a.tp, COUNTER_GOUKI_CAGE_MATCH as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // c:AddCounter(COUNTER_GOUKI_CAGE_MATCH,3)
        card::add_counter(d, c, COUNTER_GOUKI_CAGE_MATCH, 3, false);
    }
}
/// `s.rmvcountercon(e,tp,eg,ep,ev,re,r,rp)`
fn rmvcountercon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=Duel.GetBattleMonster(tp)
    let (bc, _) = duel::get_battle_monster(d, a.tp);
    // if not (bc and bc:IsStatus(STATUS_OPPO_BATTLE)) then return false end
    let bc = match bc {
        Some(bc) => bc,
        None => return false,
    };
    if !card::is_status(d, bc, STATUS_OPPO_BATTLE) {
        return false;
    }
    // if bc:IsRelateToBattle() then
    if card::is_relate_to_battle(d, bc) {
        // return bc:IsSetCard(SET_GOUKI) and bc:IsControler(tp)
        card::is_set_card(d, bc, &[SET_GOUKI as u16]) && card::is_controler(d, bc, a.tp)
    } else {
        // return bc:IsPreviousSetCard(SET_GOUKI) and bc:IsPreviousControler(tp)
        card::is_previous_set_card(d, bc, &[SET_GOUKI as u16]) && card::is_previous_controler(d, bc, a.tp)
    }
}
/// `s.rmvcounterop(e,tp,eg,ep,ev,re,r,rp)`
fn rmvcounterop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:RemoveCounter(tp,COUNTER_GOUKI_CAGE_MATCH,1,REASON_EFFECT) then
    if card::is_relate_to_effect(d, c, a.e) && card::remove_counter(d, c, a.tp, COUNTER_GOUKI_CAGE_MATCH, 1, REASON_EFFECT) {
        // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CANNOT_DISABLE,1)
        card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CANNOT_DISABLE, 1, 0, 0);
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:GetCounter(COUNTER_GOUKI_CAGE_MATCH)==0 and c:HasFlagEffect(id,3)
    card::get_counter(d, c, COUNTER_GOUKI_CAGE_MATCH) == 0 && card::has_flag_effect(d, c, ID, Some(3))
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GOUKI) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_GOUKI as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d: &mut Duel, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // local tg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,nil,e,tp)
    let tg = duel::get_matching_group(d, Some(&|d: &mut Duel, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
    // if #tg==0 then return end
    if tg.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT as u32).is_some() {
        ft = 1;
    }
    // local g=aux.SelectUnselectGroup(tg,e,tp,1,ft,aux.dncheck,1,tp,HINTMSG_SPSUMMON)
    let dn = aux::dncheck();
    let g = aux::select_unselect_group(d, &tg, Some(a.e), a.tp, Some(1), Some(ft as usize), Some(&dn), 1, a.tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
        .unwrap_or_default();
    // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)>0 and c:IsRelateToEffect(e) then
    if !g.is_empty()
        && duel::special_summon(d, g.clone(), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0
        && card::is_relate_to_effect(d, c, a.e)
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // c:AddCounter(COUNTER_GOUKI_CAGE_MATCH,3)
        card::add_counter(d, c, COUNTER_GOUKI_CAGE_MATCH, 3, false);
        // c:ResetFlagEffect(id)
        card::reset_flag_effect(d, c, ID);
    }
}
