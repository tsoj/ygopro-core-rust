//! Burial Summit Xenovader△ Helheim
use super::prelude::*;

pub const ID: u32 = 100458036;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_XENOVADER],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // During the Main Phase (Quick Effect): You can reveal this card in your hand; banish it (face-down), and if you do, Special Summon 1 of your face-down banished "Xenovader△" monsters, except a Level 6 monster. You can only use this effect of "Burial Summit Xenovader△ Helheim" once per turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    // e1:SetCost(Cost.SelfReveal)
    effect::set_cost(d, e1, Some(cost::self_reveal()));
    // e1:SetTarget(s.selfbantg)
    effect::set_target(d, e1, Some(cost_target(selfbantg)));
    // e1:SetOperation(s.selfbanop)
    effect::set_operation(d, e1, Some(operation(selfbanop)));
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is Normal or Special Summoned: Banish (face-down) the top 6 cards of your Deck
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    // e2a:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2a, CATEGORY_REMOVE);
    // e2a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2a, EVENT_SUMMON_SUCCESS);
    // e2a:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) ... end)
    effect::set_target(d, e2a, Some(cost_target(e2a_tg)));
    // e2a:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e2a, Some(operation(e2a_op)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
    // Other "Xenovader△" cards you control cannot be destroyed by your opponent's card effects, also your opponent cannot target them with card effects
    // local e3a=Effect.CreateEffect(c)
    let e3a = effect::create_effect(d, c);
    // e3a:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3a, EFFECT_TYPE_FIELD);
    // e3a:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e3a, EFFECT_INDESTRUCTABLE_EFFECT);
    // e3a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3a, LOCATION_MZONE);
    // e3a:SetTargetRange(LOCATION_ONFIELD,0)
    effect::set_target_range(d, e3a, LOCATION_ONFIELD, 0);
    // e3a:SetTarget(function(e,c) return c:IsSetCard(SET_XENOVADER) and c~=e:GetHandler() end)
    effect::set_target(d, e3a, Some(effect_card_fn(|d, e, c| card::is_set_card(d, c, &[SET_XENOVADER as u16]) && c != effect::get_handler(d, e).nil())));
    // e3a:SetValue(aux.indoval)
    effect::set_value_fn(d, e3a, aux::indoval());
    // c:RegisterEffect(e3a)
    card::register_effect(d, c, e3a, false);
    // local e3b=e3a:Clone()
    let e3b = effect::clone(d, e3a);
    // e3b:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e3b, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e3b:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e3b, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e3b:SetValue(aux.tgoval)
    effect::set_value_fn(d, e3b, aux::tgoval());
    // c:RegisterEffect(e3b)
    card::register_effect(d, c, e3b, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return Duel.IsMainPhase() end`
fn e1_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFacedown() and c:IsSetCard(SET_XENOVADER) and not c:IsLevel(6) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_facedown(d, c)
        && card::is_set_card(d, c, &[SET_XENOVADER as u16])
        && !card::is_level(d, c, &[6])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.selfbantg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfbantg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemove(tp,POS_FACEDOWN) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_REMOVED,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let e = a.e;
        let tp = a.tp;
        return card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,c,1,tp,POS_FACEDOWN)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(c)), 1, a.tp, POS_FACEDOWN as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_REMOVED as i32);
    true
}
/// `s.selfbanop(e,tp,eg,ep,ev,re,r,rp)`
fn selfbanop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) and Duel.Remove(c,POS_FACEDOWN,REASON_EFFECT)>0 and c:IsLocation(LOCATION_REMOVED) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if card::is_relate_to_effect(d, c, e)
        && duel::remove(d, CardSet::from_card(c), POS_FACEDOWN as u8, REASON_EFFECT, None, None) > 0
        && card::is_location(d, c, LOCATION_REMOVED)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_REMOVED,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP) end
        if !g.is_empty() {
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
/// The target of e2a: banish the top 6 cards of the Deck
fn e2a_tg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 6, a.tp, LOCATION_DECK as i32);
    true
}
/// The operation of e2a: banish the top 6 cards of the Deck face-down
fn e2a_op(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetDecktopGroup(tp,6)
    let g = duel::get_decktop_group(d, a.tp, 6);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.Remove(g,POS_FACEDOWN,REASON_EFFECT)
        duel::remove(d, g, POS_FACEDOWN as u8, REASON_EFFECT, None, None);
    }
}
