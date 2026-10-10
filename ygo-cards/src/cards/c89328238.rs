//! Backup Team
use super::prelude::*;

pub const ID: u32 = 89328238;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Draw cards then return cards to the Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW+CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Set itself but banish it when it leaves the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetHintTiming(TIMING_END_PHASE)
    effect::set_hint_timing(d, e2, TIMING_END_PHASE, None);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.setcon)
    effect::set_condition(d, e2, Some(condition(setcon)));
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ct=Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)
    let ct = duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD);
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,ct) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, ct as u32);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,ct)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, ct);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,ct,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, ct as u8, tp, LOCATION_HAND as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local ct=Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)
    let ct = duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD);
    // if Duel.Draw(p,ct,REASON_EFFECT)==ct then
    if duel::draw(d, p, ct as u32, REASON_EFFECT) == ct {
        // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,p,LOCATION_HAND,0,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_deck(d, c)), p, LOCATION_HAND, 0, NO_EX);
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_TODECK as u64);
        // local sg=g:Select(p,ct,ct,nil)
        let sg = group::select(d, &g, p, ct as u16, ct as u16, false, NO_EX).nil();
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if Duel.SendtoDeck(sg,nil,SEQ_DECKBOTTOM,REASON_EFFECT)==0 then return end
        if duel::send_to_deck(d, sg, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None) == 0 {
            return;
        }
        // local rt=Duel.GetOperatedGroup():FilterCount(Card.IsLocation,nil,LOCATION_DECK)
        let operated = duel::get_operated_group(d);
        let rt = group::filter_count(d, &operated, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX);
        // if rt>0 then
        if rt > 0 {
            // Duel.SortDeckbottom(p,p,rt)
            duel::sort_deckbottom(d, p, p, rt as u16);
        }
    }
}
/// `s.setcon(e,tp,eg,ep,ev,re,r,rp)`
fn setcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_END) and Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)>Duel.GetFieldGroupCount(tp,LOCATION_ONFIELD,0)
    duel::is_phase(d, PHASE_END)
        && duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD) > duel::get_field_group_count(d, a.tp, LOCATION_ONFIELD, 0)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsSSetable() end
    if a.chk == Some(0) {
        return card::is_s_setable(d, c, false, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsSSetable() and Duel.SSet(tp,c) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_s_setable(d, c, false, None) {
        duel::s_set(d, a.tp, c, None, true);
        // Banish it if it leaves the field
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3300)
        effect::set_description(d, e1, 3300);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        // e1:SetValue(LOCATION_REMOVED)
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
