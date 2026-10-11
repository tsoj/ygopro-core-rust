//! World Legacy Trap Globe
use super::prelude::*;

pub const ID: u32 = 16329071;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WORLD_LEGACY], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_WORLD_LEGACY) and not c:IsCode(id) and (c:IsLocation(LOCATION_HAND|LOCATION_GRAVE) or c:IsFaceup()) and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_WORLD_LEGACY as u16])
        && !card::is_code(d, c, &[ID])
        && (card::is_location(d, c, LOCATION_HAND | LOCATION_GRAVE) || card::is_faceup(d, c))
        && card::is_able_to_deck(d, c)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_ONFIELD|LOCATION_REMOVED,0,nil)
    let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_HAND | LOCATION_GRAVE | LOCATION_ONFIELD | LOCATION_REMOVED, 0, NO_EX);
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,2) and g:GetClassCount(Card.GetCode)>=5 end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 2)
            && group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 5;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,5,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 5, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 2);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.filter),tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_ONFIELD|LOCATION_REMOVED,0,nil)
    let nf = aux::necro_valley_filter(filter);
    let mut g = duel::get_matching_group(d, Some(&nf), tp, LOCATION_HAND | LOCATION_GRAVE | LOCATION_ONFIELD | LOCATION_REMOVED, 0, NO_EX);
    // if g:GetClassCount(Card.GetCode)<5 then return end
    if group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) < 5 {
        return;
    }
    // local sg=Group.CreateGroup()
    let mut sg = CardSet::new();
    // for i=1,5 do
    for _ in 1..=5 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        // local g1=g:Select(tp,1,1,nil)
        let g1 = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // g:Remove(Card.IsCode,nil,g1:GetFirst():GetCode())
        let first = group::get_first(d, &g1).nil();
        let codes = card::get_code(d, first);
        group::remove(d, &mut g, &|d, c| card::is_code(d, c, &codes), NO_EX);
        // sg:Merge(g1)
        group::add_card(&mut sg, &g1);
    }
    // local cg=sg:Filter(Card.IsLocation,nil,LOCATION_HAND)
    let cg = group::filter(d, &sg, &|d, c| card::is_location(d, c, LOCATION_HAND), NO_EX);
    // if #cg>0 then
    if !cg.is_empty() {
        // Duel.ConfirmCards(1-tp,cg)
        duel::confirm_cards(d, 1 - tp, cg);
    }
    // Duel.SendtoDeck(sg,nil,SEQ_DECKTOP,REASON_EFFECT)
    duel::send_to_deck(d, sg.clone(), None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    // local og=Duel.GetOperatedGroup()
    let og = duel::get_operated_group(d);
    // if og:IsExists(Card.IsLocation,1,nil,LOCATION_DECK) then Duel.ShuffleDeck(tp) end
    if group::is_exists(d, &og, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX) {
        duel::shuffle_deck(d, tp);
    }
    // local ct=og:FilterCount(Card.IsLocation,nil,LOCATION_DECK|LOCATION_EXTRA)
    let ct = group::filter_count(d, &og, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), NO_EX);
    // if ct==5 then
    if ct == 5 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,2,REASON_EFFECT)
        duel::draw(d, tp, 2, REASON_EFFECT);
    }
}
