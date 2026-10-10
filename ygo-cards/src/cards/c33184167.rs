//! Tribe-Infecting Virus
use super::prelude::*;

pub const ID: u32 = 33184167;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, a.tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local tc=g:GetFirst()
    let _tc = group::get_first(d, &g);
    // local race=0
    let mut race = 0u64;
    // for tc in aux.Next(g) do race=(race|tc:GetRace()) end
    for tc in group::iter(d, &g) {
        race |= card::get_race(d, tc);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,0)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, 0);
    // local arc=Duel.AnnounceRace(tp,1,race)
    let arc = duel::announce_race(d, a.tp, 1, race);
    // e:SetLabel(arc)
    effect::set_label(d, a.e, &[arc as i64]);
    // local dg=g:Filter(Card.IsRace,nil,arc)
    let dg = group::filter(d, &g, &|d, c| card::is_race(d, c, arc), NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,#dg,0,0)
    let n = dg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), n, 0, 0);
    true
}
fn filter2(d: &mut Duel, c: CardId, rc: u64) -> bool {
    // return c:IsFaceup() and c:IsRace(rc)
    card::is_faceup(d, c) && card::is_race(d, c, rc)
}
fn op(d: &mut Duel, a: &EffArgs) {
    // local arc=e:GetLabel()
    let arc = effect::get_label(d, a.e);
    // local g=Duel.GetMatchingGroup(s.filter2,tp,LOCATION_MZONE,LOCATION_MZONE,nil,arc)
    let g = duel::get_matching_group(d, Some(&|d, c| filter2(d, c, arc as u64)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
