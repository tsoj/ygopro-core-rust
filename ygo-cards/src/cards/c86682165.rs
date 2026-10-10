//! Icejade Gymir Aegirine
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 86682165;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // 1 WATER Tuner + 1+ non-Tuner monsters
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_WATER),1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_WATER, sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner(None)),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // Apply monster protection
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some((TIMINGS_CHECK_MONSTER | TIMING_MAIN_END) as u32));
    // e1:SetTarget(s.immtg)
    effect::set_target(d, e1, Some(cost_target(immtg)));
    // e1:SetOperation(s.immop)
    effect::set_operation(d, e1, Some(operation(immop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon this card
    // local e1=Effect.CreateEffect(c)
    let e1b = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1b, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1b, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1b, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1b, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_REMOVE)
    effect::set_code(d, e1b, EVENT_REMOVE);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1b, LOCATION_GRAVE);
    // e1:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e1b, 1, ID, 1, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1b, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1b, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1b, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1b, false);
}
/// `s.rmfilter(c,code)`
fn rmfilter(d: &mut Duel, c: CardId, code: i64) -> bool {
    // return c:IsFaceup() and c:IsCode(code) and c:IsAbleToRemove()
    card::is_faceup(d, c) && card::is_code(d, c, &[code as u32]) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.immtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn immtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_ONFIELD|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - a.tp, (LOCATION_ONFIELD | LOCATION_GRAVE) as i32);
    true
}
/// `s.immop(e,tp,eg,ep,ev,re,r,rp)`
fn immop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Monsters cannot be destroyed or banished opponent's effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetValue(aux.indoval)
    effect::set_value_fn(d, e1, aux::indoval());
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CANNOT_REMOVE)
    effect::set_code(d, e2, EFFECT_CANNOT_REMOVE);
    // e2:SetTargetRange(1,1)
    effect::set_target_range(d, e2, 1, 1);
    // e2:SetTarget(s.remtg)
    effect::set_target(d, e2, Some(lua_fn(remtg)));
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // aux.RegisterClientHint(c,0,tp,1,0,aux.Stringid(id,2),RESET_PHASE|PHASE_END,1)
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 2)), RESET_PHASE | PHASE_END, Some(1));
    // local ch=Duel.GetCurrentChain()
    let ch = duel::get_current_chain(d, false);
    // if ch<=1 then return end
    if ch <= 1 {
        return;
    }
    // local chp,chcode=Duel.GetChainInfo(ch-1,CHAININFO_TRIGGERING_PLAYER,CHAININFO_TRIGGERING_CODE)
    let info = duel::get_chain_info(d, (ch - 1) as u8, &[CHAININFO_TRIGGERING_PLAYER, CHAININFO_TRIGGERING_CODE]);
    let chp = info[0].int();
    // if chp~=1-tp or not chcode then return end
    if chp != (1 - tp) as i64 || info[1].is_nil() {
        return;
    }
    let chcode = info[1].int();
    // local g=Duel.GetMatchingGroup(s.rmfilter,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,nil,chcode)
    let g = duel::get_matching_group(d, Some(&|d, cc| rmfilter(d, cc, chcode)), tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
/// `s.remtg(e,c,rp,r,re)`
fn remtg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e) as i64;
    let c = a.c(1);
    let rp = a.int(2);
    let r = a.int(3) as u32;
    // return c:IsFaceup() and c:IsControler(tp) and c:IsLocation(LOCATION_MZONE) and rp==1-tp and r&REASON_EFFECT~=0
    card::is_faceup(d, c) && card::is_controler(d, c, tp as u8) && card::is_location(d, c, LOCATION_MZONE) && rp == 1 - tp && (r & REASON_EFFECT) != 0
}
/// `s.spconfilter(c,tp)`
fn spconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsReason(REASON_EFFECT) and c:GetReasonPlayer()==1-tp
    card::is_reason(d, c, REASON_EFFECT) && card::get_reason_player(d, c) == 1 - tp
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.spconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| spconfilter(d, c, tp), 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(c), 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
