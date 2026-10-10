//! Grand Convergence
use super::prelude::*;

pub const ID: u32 = 38430673;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_MACRO_COSMOS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Inflict 300 damage to your opponent, and if you do, destroy all monsters on the field (if any)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DAMAGE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DAMAGE + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(function(e,tp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_MACRO_COSMOS),tp,LOCATION_ONFIELD,0,1,nil) end)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMING_BATTLE_START|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMING_BATTLE_START | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `function(e,tp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_MACRO_COSMOS),tp,LOCATION_ONFIELD,0,1,nil) end`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_MACRO_COSMOS])),
        a.tp,
        LOCATION_ONFIELD,
        0,
        1,
        NO_EX,
    )
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,300)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 300);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, a.tp, LOCATION_MZONE as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.Damage(1-tp,300,REASON_EFFECT)==0 then return end
    if duel::damage(d, 1 - a.tp, 300, REASON_EFFECT, false, None) == 0 {
        return;
    }
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
