//! A Wingbeat of Giant Dragon
use super::prelude::*;

pub const ID: u32 = 28596933;

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
    // return c:IsFaceup() and c:IsRace(RACE_DRAGON) and c:IsLevelAbove(5) and c:IsAbleToHand()
    card::is_faceup(d, c) && card::is_race(d, c, RACE_DRAGON) && card::is_level_above(d, c, 5) && card::is_able_to_hand(d, c, None)
}
/// `s.dfilter(c)`
fn dfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap()
    card::is_spell_trap(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler() (the handler is the card excluded from the searches below)
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE,0,1,nil)
    //   and Duel.IsExistingMatchingCard(s.dfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&dfilter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    }
    // local sg=Duel.GetMatchingGroup(s.dfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler())
    let sg = duel::get_matching_group(d, Some(&dfilter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_MZONE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,sg,#sg,0,0)
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(sg), n, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc and Duel.SendtoHand(tc,nil,REASON_EFFECT)~=0 and tc:IsLocation(LOCATION_HAND) then
    if let Some(tc) = tc {
        if duel::send_to_hand(d, tc, None, REASON_EFFECT, None) != 0 && card::is_location(d, tc, LOCATION_HAND) {
            // local sg=Duel.GetMatchingGroup(s.dfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler())
            let c = effect::get_handler(d, a.e).nil();
            let sg = duel::get_matching_group(d, Some(&dfilter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
            // Duel.Destroy(sg,REASON_EFFECT)
            duel::destroy(d, sg, REASON_EFFECT, None, None);
        }
    }
}
