//! Slash Draw
use super::prelude::*;

pub const ID: u32 = 71344451;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DECKDES+CATEGORY_DRAW+CATEGORY_DESTROY+CATEGORY_DAMAGE+CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_DECKDES + CATEGORY_DRAW + CATEGORY_DESTROY + CATEGORY_DAMAGE + CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| card::is_discardable(d, x, None, None)), a.tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, a.tp, Some(&|d, x| card::is_discardable(d, x, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ct=Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)
    let ct = duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD);
    // if chk==0 then return ct>0 and Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>ct and Duel.IsPlayerCanDiscardDeck(tp,ct)
    //   and Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return ct > 0
            && duel::get_field_group_count(d, tp, LOCATION_DECK, 0) > ct
            && duel::is_player_can_discard_deck(d, tp, ct as u32)
            && duel::is_player_can_draw(d, tp, 1);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,ct)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, tp, ct);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,PLAYER_ALL,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, PLAYER_ALL as u8, LOCATION_ONFIELD as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,2000)
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 2000);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    let c = effect::get_handler(d, e).nil();
    // local ct=Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)
    let ct = duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD);
    // if ct==0 or Duel.DiscardDeck(tp,ct,REASON_EFFECT)==0 then return end
    if ct == 0 || duel::discard_deck(d, tp, ct as u16, REASON_EFFECT) == 0 {
        return;
    }
    // local grave_ct=Duel.GetOperatedGroup():FilterCount(Card.IsLocation,nil,LOCATION_GRAVE)
    let og = duel::get_operated_group(d);
    let grave_ct = group::filter_count(d, &og, &|d, x| card::is_location(d, x, LOCATION_GRAVE), NO_EX);
    // if grave_ct==0 or not Duel.IsPlayerCanDraw(tp) then return end
    if grave_ct == 0 || !duel::is_player_can_draw(d, tp, 0) {
        return;
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // if Duel.Draw(tp,1,REASON_EFFECT)==0 then return end
    if duel::draw(d, tp, 1, REASON_EFFECT) == 0 {
        return;
    }
    // local tc=Duel.GetOperatedGroup():GetFirst()
    let og = duel::get_operated_group(d);
    let tc = group::get_first(d, &og).nil();
    // Duel.ConfirmCards(1-tp,tc)
    duel::confirm_cards(d, 1 - tp, tc);
    // if tc:IsCode(id) and Duel.SendtoGrave(tc,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_GRAVE) then
    if card::is_code(d, tc, &[ID])
        && duel::send_to_grave(d, tc, REASON_EFFECT, None, None) > 0
        && card::is_location(d, tc, LOCATION_GRAVE)
    {
        // local sg=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler())
        let sg = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
        // if #sg==0 or Duel.Destroy(sg,REASON_EFFECT)==0 then return end
        if sg.is_empty() || duel::destroy(d, sg, REASON_EFFECT, None, None) == 0 {
            return;
        }
        // local tg=Duel.GetOperatedGroup():Filter(Card.IsLocation,nil,LOCATION_GRAVE)
        let og = duel::get_operated_group(d);
        let tg = group::filter(d, &og, &|d, x| card::is_location(d, x, LOCATION_GRAVE), NO_EX);
        // if #tg>0 then
        if !tg.is_empty() {
            // local dam=#tg*2000
            let dam = tg.len() as i64 * 2000;
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Damage(1-tp,dam,REASON_EFFECT)
            duel::damage(d, 1 - tp, dam, REASON_EFFECT, false, None);
        }
    } else {
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        // local dg=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(Card.IsAbleToDeck),tp,LOCATION_GRAVE,0,grave_ct,grave_ct,nil)
        let necro = aux::necro_valley_filter(|d, x| card::is_able_to_deck(d, x));
        let dg = duel::select_matching_card(d, tp, Some(&necro), tp, LOCATION_GRAVE, 0, grave_ct as u16, grave_ct as u16, false, NO_EX).nil();
        // if #dg>0 then
        if !dg.is_empty() {
            // Duel.HintSelection(dg,true)
            duel::hint_selection(d, dg.clone(), true);
            // Duel.SendtoDeck(dg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
            duel::send_to_deck(d, dg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        }
    }
}
