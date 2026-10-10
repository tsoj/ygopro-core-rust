//! Mirror of Oaths
use super::prelude::*;

pub const ID: u32 = 46656406;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DRAW);
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
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:GetSummonPlayer()~=tp and c:IsPreviousLocation(LOCATION_DECK)
    card::get_summon_player(d, c) != tp && card::is_previous_location(d, c, LOCATION_DECK)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return eg:IsExists(s.filter,1,nil,tp) and Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        let eg = a.eg();
        return group::is_exists(d, &eg, &|d, c| filter(d, c, tp), 1, NO_EX) && duel::is_player_can_draw(d, tp, 1);
    }
    // local g=eg:Filter(s.filter,nil,tp)
    let eg = a.eg();
    let g = group::filter(d, &eg, &|d, c| filter(d, c, tp), NO_EX);
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, eg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = group::get_count(&g) as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRelateToEffect(e) and not c:IsSummonPlayer(tp) and c:IsPreviousLocation(LOCATION_DECK)
    card::is_relate_to_effect(d, c, e) && card::get_summon_player(d, c) != tp && card::is_previous_location(d, c, LOCATION_DECK)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=eg:Filter(s.filter2,nil,e,tp)
    let eg = a.eg();
    let g = group::filter(d, &eg, &|d, c| filter2(d, c, e, tp), NO_EX);
    // if Duel.Destroy(g,REASON_EFFECT)~=0 then
    if duel::destroy(d, &g, REASON_EFFECT, None, None) != 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, tp, 1, REASON_EFFECT);
    }
}
