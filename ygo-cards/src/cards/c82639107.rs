//! Convert Contact
use super::prelude::*;

pub const ID: u32 = 82639107;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_NEO_SPACIAN as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(function(_,tp) return Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0 end)
    effect::set_condition(
        d,
        e1,
        Some(condition(|d, a| duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) == 0)),
    );
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_NEO_SPACIAN) and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_NEO_SPACIAN as u16]) && card::is_able_to_grave(d, c, None)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsLocation,nil,LOCATION_HAND)==1
    (group::filter_count(d, sg, &|d, c| card::is_location(d, c, LOCATION_HAND), NO_EX) == 1, false)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_HAND|LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&tgfilter), tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,2) and Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>=3
    //     and #g>1 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 2)
            && duel::get_field_group_count(d, tp, LOCATION_DECK, 0) >= 3
            && g.len() > 1
            && matches!(
                aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,2,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 2, tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 2);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_HAND|LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&tgfilter), tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_TOGRAVE)
    let sg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        tp,
        Some(2),
        Some(2),
        Some(&rescon),
        1,
        tp,
        Some(HINTMSG_TOGRAVE as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // if #sg~=2 then return end
    if sg.len() != 2 {
        return;
    }
    // if Duel.SendtoGrave(sg,REASON_EFFECT)==2 and sg:FilterCount(Card.IsLocation,nil,LOCATION_GRAVE)==2
    //     and Duel.IsPlayerCanDraw(tp) then
    if duel::send_to_grave(d, sg.clone(), REASON_EFFECT, None, None) == 2
        && group::filter_count(d, &sg, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX) == 2
        && duel::is_player_can_draw(d, tp, 0)
    {
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, tp);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,2,REASON_EFFECT)
        duel::draw(d, tp, 2, REASON_EFFECT);
    }
}
