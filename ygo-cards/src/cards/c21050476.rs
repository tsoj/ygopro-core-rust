//! Argostars - Fierce Parthe
use super::prelude::*;

pub const ID: u32 = 21050476;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARGOSTARS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Gains 700 ATK/DEF for each "Argostars" monster with a different name you control, except this card
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE);
    // e1a:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1a, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1a:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1a, EFFECT_UPDATE_ATTACK);
    // e1a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1a, LOCATION_MZONE);
    // e1a:SetValue(function(e,c) ... end)
    effect::set_value_fn(d, e1a, effect_value(atkval));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e1b, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // (Quick Effect): You can reveal this card in your hand; return 1 "Argostars" Continuous Trap you control to the hand, and if you do, Special Summon this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.SelfReveal)
    effect::set_cost(d, e2, Some(cost::self_reveal()));
    // e2:SetTarget(s.rthsptg)
    effect::set_target(d, e2, Some(cost_target(rthsptg)));
    // e2:SetOperation(s.rthspop)
    effect::set_operation(d, e2, Some(operation(rthspop)));
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // You can banish this card you control; Set 1 Continuous Trap from your hand. It can be activated this turn
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SET)
    effect::set_category(d, e3, CATEGORY_SET);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e3, Some(cost::self_banish()));
    // e3:SetTarget(s.settg)
    effect::set_target(d, e3, Some(cost_target(settg)));
    // e3:SetOperation(s.setop)
    effect::set_operation(d, e3, Some(operation(setop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function(e,c) of e1a/e1b
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_ARGOSTARS),e:GetHandlerPlayer(),LOCATION_MZONE,0,e:GetHandler())
    let p = effect::get_handler_player(d, e);
    let h = effect::get_handler(d, e).nil();
    let g = duel::get_matching_group(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ARGOSTARS as u16])),
        p,
        LOCATION_MZONE,
        0,
        h.into(),
    );
    // return g:GetClassCount(Card.GetCode)*700
    group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) as i64 * 700
}
// function s.rthfilter(c,tp)
fn rthfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ARGOSTARS) and c:IsContinuousTrap() and Duel.GetMZoneCount(tp,c)>0 and c:IsFaceup() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_ARGOSTARS as u16])
        && card::is_continuous_trap(d, c)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
        && card::is_faceup(d, c)
        && card::is_able_to_hand(d, c, None)
}
// function s.rthsptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rthsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rthfilter,tp,LOCATION_ONFIELD,0,1,nil,tp)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        let tp = a.tp;
        return duel::is_existing_matching_card(d, Some(&|d, x| rthfilter(d, x, tp)), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_ONFIELD as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
// function s.rthspop(e,tp,eg,ep,ev,re,r,rp)
fn rthspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.rthfilter,tp,LOCATION_ONFIELD,0,1,1,nil,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, x| rthfilter(d, x, tp)), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if not sc then return end
    let Some(sc) = sc else { return };
    // Duel.HintSelection(sc)
    duel::hint_selection(d, CardSet::from_card(sc), true);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.SendtoHand(sc,nil,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_HAND) and c:IsRelateToEffect(e) then
    if duel::send_to_hand(d, CardSet::from_card(sc), None, REASON_EFFECT, None) > 0
        && card::is_location(d, sc, LOCATION_HAND)
        && card::is_relate_to_effect(d, c, a.e)
    {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
// function s.setfilter(c)
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsContinuousTrap() and c:IsSSetable()
    card::is_continuous_trap(d, c) && card::is_s_setable(d, c, false, None)
}
// function s.settg(e,tp,eg,ep,ev,re,r,rp,chk)
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    true
}
// function s.setop(e,tp,eg,ep,ev,re,r,rp)
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_HAND,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&setfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SSet(tp,sc)>0 then
    if let Some(sc) = sc {
        if duel::s_set(d, tp, CardSet::from_card(sc), None, true) > 0 {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let e1 = effect::create_effect(d, effect::get_handler(d, a.e).nil());
            // e1:SetDescription(aux.Stringid(id,2))
            effect::set_description(d, e1, aux::stringid(ID, 2));
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_TRAP_ACT_IN_SET_TURN)
            effect::set_code(d, e1, EFFECT_TRAP_ACT_IN_SET_TURN);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // sc:RegisterEffect(e1)
            card::register_effect(d, sc, e1, false);
        }
    }
}
