//! Command Duel JP002
use super::prelude::*;

pub const ID: u32 = 95200102;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_SPECIAL_SUMMON);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
// function s.filter(c,e,tp)
//     return c:IsCanBeSpecialSummoned(e,0,tp,true,false) and c.illegal and c:IsMonster()
// (`c.illegal` is set by no official script: always nil, so no card matches and both groups of
// `s.activate` are empty)
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,PLAYER_ALL,LOCATION_DECK|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, PLAYER_ALL as u8, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, _a: &EffArgs) {
    // local g1=Duel.GetMatchingGroup(s.filter,tp,LOCATION_EXTRA,0,nil,e,tp)
    // local g2=Duel.GetMatchingGroup(s.filter,1-tp,LOCATION_EXTRA,0,nil,e,tp)
    // (both empty: see s.filter; the `#g1>0 and ...` / `#g2>0 and ...` branches never run)
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
