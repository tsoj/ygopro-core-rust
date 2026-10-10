//! Labrynth Cooclock
use super::prelude::*;

pub const ID: u32 = 2511;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_LABRYNTH], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Allow Trap activation in the same turn it's set
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e1, Some(cost::self_discard()));
    // e1:SetOperation(s.acop)
    effect::set_operation(d, e1, Some(operation(acop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add to hand or Special Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.acop(e,tp,eg,ep,ev,re,r,rp)
fn acop(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_TRAP_ACT_IN_SET_TURN)
    effect::set_code(d, e1, EFFECT_TRAP_ACT_IN_SET_TURN);
    // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e1, 1, ID, 2, 0);
    // e1:SetTargetRange(LOCATION_SZONE,0)
    effect::set_target_range(d, e1, LOCATION_SZONE, 0);
    // e1:SetCondition(s.accon)
    effect::set_condition(d, e1, Some(lua_fn(accon)));
    // e1:SetTarget(function(e,c) return c:IsNormalTrap() end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_normal_trap(d, c))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
// function s.accon(e)
fn accon(d: &mut Duel, a: Args) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_LABRYNTH),e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let p = effect::get_handler_player(d, a.e(0));
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_LABRYNTH as u16])), p, LOCATION_MZONE, 0, 1, NO_EX)
}
// function s.thcon(e,tp,eg,ep,ev,re,r,rp)
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not (rp==tp and r&REASON_COST==REASON_COST and re and re:IsActivated()) then return false end
    if !(a.rp == a.tp && (a.r & REASON_COST) == REASON_COST) {
        return false;
    }
    let Some(re) = a.re else { return false };
    if !effect::is_activated(d, re) {
        return false;
    }
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, re).nil();
    // return eg:IsExists(Card.IsPreviousLocation,1,e:GetHandler(),LOCATION_HAND)
    //     and ((re:IsTrapEffect() and re:IsHasType(EFFECT_TYPE_ACTIVATE))
    //     or (rc:IsSetCard(SET_LABRYNTH) and not rc:IsCode(id)))
    let h = effect::get_handler(d, a.e).nil();
    group::is_exists(d, &a.eg.clone().nil(), &|d, c| card::is_previous_location(d, c, LOCATION_HAND), 1, h.into())
        && ((effect::is_trap_effect(d, re) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE))
            || (card::is_set_card(d, rc, &[SET_LABRYNTH as u16]) && !card::is_code(d, rc, &[ID])))
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() or (Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)) end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None)
            || (duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None));
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // aux.ToHandOrElse(c,tp,
    //     function(sc) return sc:IsCanBeSpecialSummoned(e,0,tp,false,false) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end,
    //     function(sc) Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP) end,
    //     aux.Stringid(id,3))
    let (e, tp) = (a.e, a.tp);
    aux::to_hand_or_else(
        d,
        c,
        tp,
        Some(&move |d: &mut Duel, sc: CardId| {
            card::is_can_be_special_summoned(d, sc, e, 0, tp, false, false, None, None, None) && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        }),
        Some(&move |d: &mut Duel, sc: &CardSet| duel::special_summon(d, sc.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None)),
        Some(aux::stringid(ID, 3)),
    );
}
