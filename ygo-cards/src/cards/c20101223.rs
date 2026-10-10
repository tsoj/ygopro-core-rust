//! Breath of Light
use super::prelude::*;

pub const ID: u32 = 20101223;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
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
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_ROCK) and c:IsFaceup()
    card::is_race(d, c, RACE_ROCK) && card::is_faceup(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // local sg=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let sg = duel::get_matching_group(d, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,sg,#sg,0,0)
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(sg), n, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local sg=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let sg = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.Destroy(sg,REASON_EFFECT)
    duel::destroy(d, sg, REASON_EFFECT, None, None);
}
