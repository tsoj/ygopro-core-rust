//! Empty Machine
use super::prelude::*;

pub const ID: u32 = 9409625;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TIMELORD], listed_names: &[36894320], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // insd via destroy replace (workaround)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_END_PHASE));
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // (the commented-out "untested version of Fluo's destruction count" in the Lua is not translated)
    // draw
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e3, CATEGORY_DRAW);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCost(s.drcost)
    effect::set_cost(d, e3, Some(cost_target(drcost)));
    // e3:SetTarget(s.drtg)
    effect::set_target(d, e3, Some(cost_target(drtg)));
    // e3:SetOperation(s.drop)
    effect::set_operation(d, e3, Some(operation(drop_op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // to deck
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_TODECK+CATEGORY_SET)
    effect::set_category(d, e4, CATEGORY_TODECK + CATEGORY_SET);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e4, EVENT_FREE_CHAIN);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetCondition(s.tdcon)
    effect::set_condition(d, e4, Some(condition(tdcon)));
    // e4:SetCost(s.cost)
    effect::set_cost(d, e4, Some(cost_target(cost)));
    // e4:SetTarget(s.tdtg)
    effect::set_target(d, e4, Some(cost_target(tdtg)));
    // e4:SetOperation(s.tdop)
    effect::set_operation(d, e4, Some(operation(tdop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsReason(REASON_EFFECT) and c:GetReasonPlayer()~=tp and e:GetHandler():GetFlagEffect(id+1)==0 end
    if a.chk == Some(0) {
        return card::is_reason(d, c, REASON_EFFECT) && card::get_reason_player(d, c) != a.tp && card::get_flag_effect(d, c, ID + 1) == 0;
    }
    // c:RegisterFlagEffect(id+1,RESET_EVENT|RESETS_STANDARD,0,1)
    card::register_flag_effect(d, c, ID + 1, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
    // return true
    true
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetFlagEffect(id)==0 end
    if a.chk == Some(0) {
        return card::get_flag_effect(d, c, ID) == 0;
    }
    // c:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
    card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    true
}
/// `s.drfilter(c)`
fn drfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetLevel()==10 and c:IsDiscardable()
    card::get_level(d, c) == 10 && card::is_discardable(d, c, None, None)
}
/// `s.drcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.drfilter,tp,LOCATION_HAND,0,1,nil) and s.cost(e,tp,eg,ep,ev,re,r,rp,0) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&drfilter), tp, LOCATION_HAND, 0, 1, NO_EX) && cost(d, &EffArgs { chk: Some(0), ..a.clone() });
    }
    // Duel.DiscardHand(tp,s.drfilter,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, tp, Some(&drfilter), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    // s.cost(e,tp,eg,ep,ev,re,r,rp,1)
    cost(d, &EffArgs { chk: Some(1), ..a.clone() });
    true
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
/// `s.stfilter(c)`
fn stfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetSequence()<5
    card::get_sequence(d, c) < 5
}
/// `s.tdcon(e,tp,eg,ep,ev,re,r,rp)`
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(s.stfilter,tp,LOCATION_SZONE,0,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    !duel::is_existing_matching_card(d, Some(&stfilter), a.tp, LOCATION_SZONE, 0, 1, h.into())
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_TIMELORD) and c:IsMonster() and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_TIMELORD as u16]) && card::is_monster(d, c) && card::is_able_to_deck(d, c)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.tdfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && tdfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tdfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.tdfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 1, 0, 0);
    true
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(36894320) and c:IsSSetable()
    card::is_code(d, c, &[36894320]) && card::is_s_setable(d, c, false, None)
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)~=0 then
    if card::is_relate_to_effect(d, tc, a.e) && duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) != 0 {
        // local g=Duel.GetMatchingGroup(s.setfilter,tp,LOCATION_HAND|LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&setfilter), tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
        // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            // local sc=g:Select(tp,1,1,nil):GetFirst()
            let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            let sc = group::get_first(d, &sel).nil();
            // Duel.SSet(tp,sc)
            duel::s_set(d, tp, sc, None, true);
        }
    }
}
