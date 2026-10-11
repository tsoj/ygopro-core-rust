//! Darkfall
use super::prelude::*;

pub const ID: u32 = 30127518;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_DECKDES)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_DECKDES);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c,sp,e)`
fn cfilter(d: &mut Duel, c: CardId, sp: u8, e: Option<EffectId>) -> bool {
    // return c:IsFaceup() and c:GetSummonPlayer()==sp and (not e or c:IsRelateToEffect(e))
    card::is_faceup(d, c) && card::get_summon_player(d, c) == sp && e.map_or(true, |e| card::is_relate_to_effect(d, c, e))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return eg:IsExists(s.cfilter,2,nil,1-tp) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return group::is_exists(d, &eg, &|d, c| cfilter(d, c, 1 - tp, None), 2, NO_EX);
    }
    // local g=eg:Filter(s.cfilter,nil,1-tp)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| cfilter(d, c, 1 - tp, None), NO_EX);
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS):Filter(s.cfilter,nil,1-tp,e)
    let tg = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    let g = group::filter(d, &tg, &|d, c| cfilter(d, c, 1 - tp, Some(e)), NO_EX);
    // Duel.SendtoGrave(g,REASON_EFFECT)
    duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    // local exg=Group.CreateGroup()
    let mut exg = CardSet::new();
    // local g1=Duel.GetOperatedGroup()
    let g1 = duel::get_operated_group(d);
    // local tc=g1:GetFirst()
    let _ = group::get_first(d, &g1);
    // for tc in aux.Next(g1) do
    for tc in group::iter(d, &g1) {
        // if tc:IsLocation(LOCATION_GRAVE) then
        if card::is_location(d, tc, LOCATION_GRAVE) {
            // local fg=Duel.GetMatchingGroup(Card.IsCode,tp,0,LOCATION_DECK|LOCATION_HAND,nil,tc:GetCode())
            let codes = card::get_code(d, tc);
            let fg = duel::get_matching_group(d, Some(&|d, c| card::is_code(d, c, &codes)), tp, 0, LOCATION_DECK | LOCATION_HAND, NO_EX);
            // exg:Merge(fg)
            group::add_card(&mut exg, &fg);
        }
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.SendtoGrave(exg,REASON_EFFECT)
    duel::send_to_grave(d, exg, REASON_EFFECT, None, None);
}
