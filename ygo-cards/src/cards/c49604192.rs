//! Mitsurugi Tempest
use super::prelude::*;

pub const ID: u32 = 49604192;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[19899073, 55397172, 13332685], listed_series: &[SET_MITSURUGI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, EFFECT_COUNT_CODE_OATH, 0, 0);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCondition(function(e,tp) return Duel.IsExistingMatchingCard(s.rmconfilter,tp,LOCATION_GRAVE,0,1,nil) end)
    effect::set_condition(d, e1, Some(condition(|d, a| duel::is_existing_matching_card(d, Some(&rmconfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX))));
    // e1:SetCost(s.rmcost)
    effect::set_cost(d, e1, Some(cost_target(rmcost)));
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.rmconfilter(c)`
fn rmconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MITSURUGI) and c:IsRitualSpell()
    card::is_set_card(d, c, &[SET_MITSURUGI as u16]) && card::is_ritual_spell(d, c)
}
/// `s.rmcostrescon(sg)`
fn rmcostrescon(d: &mut Duel, sg: &CardSet) -> bool {
    // return sg:GetClassCount(Card.GetOriginalCodeRule)==3
    group::get_class_count(d, sg, &|d, c| card::get_original_code_rule(d, c)[0] as i64) == 3
}
/// `s.rmcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,Card.IsOriginalCodeRule,3,false,s.rmcostrescon,nil,19899073,55397172,13332685) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(
            d,
            a.tp,
            Some(&|d, c| card::is_original_code_rule(d, c, &[19899073, 55397172, 13332685])),
            3,
            3,
            false,
            Some(&|d, sg, _tp, _exg| (rmcostrescon(d, sg), false)),
            NO_EX,
        );
    }
    // local g=Duel.SelectReleaseGroupCost(tp,Card.IsOriginalCodeRule,3,3,false,s.rmcostrescon,nil,19899073,55397172,13332685)
    let g = duel::select_release_group_cost(
        d,
        a.tp,
        Some(&|d, c| card::is_original_code_rule(d, c, &[19899073, 55397172, 13332685])),
        3,
        3,
        false,
        Some(&|d, sg, _tp, _exg| (rmcostrescon(d, sg), false)),
        NO_EX,
    );
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(1-tp,30459350)
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD|LOCATION_HAND|LOCATION_EXTRA|LOCATION_GRAVE,8,nil,1-tp) end
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, 1 - a.tp, 30459350).is_none()
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| card::is_able_to_remove(d, c, Some(1 - a.tp), None, None)),
                a.tp,
                0,
                LOCATION_ONFIELD | LOCATION_HAND | LOCATION_EXTRA | LOCATION_GRAVE,
                8,
                NO_EX,
            );
    }
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local p=1-tp
    let p = 1 - a.tp;
    // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(p,Card.IsAbleToRemove,p,LOCATION_ONFIELD|LOCATION_HAND|LOCATION_EXTRA|LOCATION_GRAVE,0,8,8,nil,p)
    let g = duel::select_matching_card(
        d,
        p,
        Some(&|d, c| card::is_able_to_remove(d, c, Some(p), None, None)),
        p,
        LOCATION_ONFIELD | LOCATION_HAND | LOCATION_EXTRA | LOCATION_GRAVE,
        0,
        8,
        8,
        false,
        NO_EX,
    )
    .nil();
    // if #g==8 then
    if g.len() == 8 {
        // Duel.Remove(g,POS_FACEUP,REASON_RULE,PLAYER_NONE,p)
        duel::remove(d, g, POS_FACEUP as u8, REASON_RULE, Some(PLAYER_NONE as u8), Some(p));
    }
}
