//! Free-Range Monsters
use super::prelude::*;

pub const ID: u32 = 75025112;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Normal Summon/Set
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SUMMON+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_SUMMON + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsMainPhase()
    duel::is_main_phase(d, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.CanSummonOrSet,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil,true,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::can_summon_or_set(d, c, true, None, 0, None)),
            tp,
            LOCATION_HAND | LOCATION_MZONE,
            0,
            1,
            NO_EX,
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,Card.CanSummonOrSet,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil,true,nil):GetFirst()
    let sg = duel::select_matching_card(
        d,
        tp,
        Some(&|d, c| card::can_summon_or_set(d, c, true, None, 0, None)),
        tp,
        LOCATION_HAND | LOCATION_MZONE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if tc then
    if let Some(tc) = group::get_first(d, &sg) {
        // Duel.SummonOrSet(tp,tc,true,nil)
        duel::summon_or_set(d, tp, tc, true, None, 0, None);
    }
}
