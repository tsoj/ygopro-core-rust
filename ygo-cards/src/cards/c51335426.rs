//! Link Bound
use super::prelude::*;

pub const ID: u32 = 51335426;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return (c:IsLocation(LOCATION_GRAVE) or c:IsFaceup()) and c:IsLinkMonster() and c:IsAbleToExtra()
    //     and Duel.IsPlayerCanDraw(tp,c:GetLink())
    if !((card::is_location(d, c, LOCATION_GRAVE) || card::is_faceup(d, c)) && card::is_link_monster(d, c) && card::is_able_to_extra(d, c)) {
        return false;
    }
    let link = card::get_link(d, c);
    duel::is_player_can_draw(d, tp, link)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil,tp) end
    if a.chk == Some(0) {
        let tp = a.tp;
        return duel::is_existing_target(d, Some(&|d, x| filter(d, x, tp)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil,tp)
    let tp = a.tp;
    let g = duel::select_target(d, a.tp, Some(&|d, x| filter(d, x, tp)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g.clone()), 1, 0, 0);
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(g:GetFirst():GetLink())
    let first = group::get_first(d, &g).nil();
    let link = card::get_link(d, first);
    duel::set_target_param(d, link);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,g:GetFirst():GetLink())
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, link as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,0,tp,g:GetFirst():GetLink())
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 0, a.tp, link as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local ct=tc:GetLink()
    let ct = card::get_link(d, tc);
    // if tc and tc:IsRelateToEffect(e) and Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)~=0 and tc:IsLocation(LOCATION_EXTRA) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::send_to_deck(d, CardSet::from_card(tc), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) != 0
        && card::is_location(d, tc, LOCATION_EXTRA)
    {
        // if Duel.Draw(p,ct,REASON_EFFECT)==ct then
        if duel::draw(d, p, ct, REASON_EFFECT) == ct as i32 {
            // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,p,LOCATION_HAND,0,nil)
            let g = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_deck(d, x)), p, LOCATION_HAND, 0, NO_EX);
            // if #g==0 then return end
            if g.is_empty() {
                return;
            }
            // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_TODECK)
            duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_TODECK as u64);
            // local sg=g:Select(p,ct,ct,nil)
            let sg = group::select(d, &g, p, ct as u16, ct as u16, false, NO_EX).nil();
            // Duel.SendtoDeck(sg,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
            duel::send_to_deck(d, sg, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
            // local ct=Duel.GetOperatedGroup():FilterCount(Card.IsLocation,nil,LOCATION_DECK)
            let og = duel::get_operated_group(d);
            let ct = group::filter_count(d, &og, &|d, x| card::is_location(d, x, LOCATION_DECK), NO_EX);
            // if ct>0 then
            if ct > 0 {
                // Duel.SortDeckbottom(p,p,ct)
                duel::sort_deckbottom(d, p, p, ct as u16);
            }
        }
    }
}
