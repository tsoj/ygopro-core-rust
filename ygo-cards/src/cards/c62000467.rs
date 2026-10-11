//! Gladiator Beast Dragases
use super::prelude::*;

pub const ID: u32 = 62000467;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GLADIATOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunction(Card.IsSetCard,SET_GLADIATOR),2,2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_GLADIATOR as u16], sc, st, Some(tp)))), 2, Some(2), None, None, None);
    // indes
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(s.indtg)
    effect::set_target(d, e1, Some(effect_card_fn(indtg)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // actlimit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // e2:SetCondition(s.actcon)
    effect::set_condition(d, e2, Some(condition(actcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // special summon
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e6, aux::stringid(ID, 0));
    // e6:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e6, CATEGORY_SPECIAL_SUMMON);
    // e6:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e6:SetCode(EVENT_PHASE|PHASE_BATTLE)
    effect::set_code(d, e6, EVENT_PHASE | PHASE_BATTLE);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetCountLimit(1,id)
    effect::set_count_limit(d, e6, 1, ID, 0, 0);
    // e6:SetCondition(s.spcon)
    effect::set_condition(d, e6, Some(condition(spcon)));
    // e6:SetCost(s.spcost)
    effect::set_cost(d, e6, Some(cost_target(spcost)));
    // e6:SetTarget(s.sptg)
    effect::set_target(d, e6, Some(cost_target(sptg)));
    // e6:SetOperation(s.spop)
    effect::set_operation(d, e6, Some(operation(spop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
// function s.indtg(e,c)
fn indtg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_GLADIATOR) and Duel.GetAttacker()==c
    card::is_set_card(d, c, &[SET_GLADIATOR as u16]) && duel::get_attacker(d) == Some(c)
}
// function s.actcon(e)
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=Duel.GetAttacker()
    let tc = duel::get_attacker(d);
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e);
    // return tc and tc:IsControler(tp) and tc:IsSetCard(SET_GLADIATOR)
    match tc {
        Some(tc) => card::is_controler(d, tc, tp) && card::is_set_card(d, tc, &[SET_GLADIATOR as u16]),
        None => false,
    }
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetBattledGroupCount()>0
    let c = effect::get_handler(d, a.e).nil();
    card::get_battled_group_count(d, c) > 0
}
// function s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToExtraAsCost() end
    if a.chk == Some(0) {
        return card::is_able_to_extra_as_cost(d, c);
    }
    // Duel.SendtoDeck(c,nil,SEQ_DECKTOP,REASON_COST)
    duel::send_to_deck(d, CardSet::from_card(c), None, SEQ_DECKTOP as i32, REASON_COST, None);
    true
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GLADIATOR) and c:IsCanBeSpecialSummoned(e,125,tp,false,false)
    card::is_set_card(d, c, &[SET_GLADIATOR as u16]) && card::is_can_be_special_summoned(d, c, e, 125, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
        let c = effect::get_handler(d, a.e).nil();
        // if e:GetHandler():GetSequence()<5 then ft=ft+1 end
        if card::get_sequence(d, c) < 5 {
            ft += 1;
        }
        // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp)
        let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, NO_EX);
        // return ft>1 and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
        //     and g:GetClassCount(Card.GetCode)>=2
        return ft > 1
            && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 2;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, LOCATION_DECK as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<2 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 2 {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp)
    let mut g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, NO_EX);
    // if #g>=2 and g:GetClassCount(Card.GetCode)>=2 then
    if g.len() >= 2 && group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sg1=g:Select(tp,1,1,nil)
        let mut sg1 = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        // local tc1=sg1:GetFirst()
        let tc1 = group::get_first(d, &sg1).nil();
        // g:Remove(Card.IsCode,nil,tc1:GetCode())
        let codes1 = card::get_code(d, tc1);
        group::remove(d, &mut g, &|d, c| card::is_code(d, c, &codes1), NO_EX);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sg2=g:Select(tp,1,1,nil)
        let sg2 = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        // local tc2=sg2:GetFirst()
        let tc2 = group::get_first(d, &sg2).nil();
        // sg1:Merge(sg2)
        group::add_card(&mut sg1, &sg2);
        // Duel.SpecialSummonStep(tc1,125,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, tc1, 125, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        // Duel.SpecialSummonStep(tc2,125,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, tc2, 125, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        // tc1:RegisterFlagEffect(tc1:GetOriginalCode(),RESET_EVENT|RESETS_STANDARD_DISABLE,0,0)
        let code1 = card::get_original_code(d, tc1);
        card::register_flag_effect(d, tc1, code1, RESET_EVENT | RESETS_STANDARD_DISABLE, 0, 0, 0, 0);
        // tc2:RegisterFlagEffect(tc2:GetOriginalCode(),RESET_EVENT|RESETS_STANDARD_DISABLE,0,0)
        let code2 = card::get_original_code(d, tc2);
        card::register_flag_effect(d, tc2, code2, RESET_EVENT | RESETS_STANDARD_DISABLE, 0, 0, 0, 0);
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
