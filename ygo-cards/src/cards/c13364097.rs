//! Virtual World Gate - Chuche
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 13364097;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VIRTUAL_WORLD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Shuffle 2 of your banished "Virtual World" cards with different names from each other into the Deck and destroy 1 face-up card on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E|TIMING_EQUIP)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E | TIMING_EQUIP));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Increase or decrease the Level/Rank of 1 "Virtual World" monster you control by 3 until the end of this turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e2, CATEGORY_LVCHANGE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsMainPhase() and Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e2, Some(condition(|d: &mut Duel, a: &EffArgs| duel::is_main_phase(d, None) && duel::is_turn_player(d, a.tp))));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.lvrnktg)
    effect::set_target(d, e2, Some(cost_target(lvrnktg)));
    // e2:SetOperation(s.lvrnkop)
    effect::set_operation(d, e2, Some(operation(lvrnkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VIRTUAL_WORLD) and c:IsFaceup() and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_VIRTUAL_WORLD as u16]) && card::is_faceup(d, c) && card::is_able_to_deck(d, c)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local exc=not c:IsStatus(STATUS_EFFECT_ENABLED) and c or nil
    let exc: Option<CardId> = if !card::is_status(d, c, STATUS_EFFECT_ENABLED) { Some(c) } else { None };
    // if chkc then return chkc:IsOnField() and chkc:IsFaceup() and (not exc or chkc~=exc) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_faceup(d, chkc) && exc.map_or(true, |x| chkc != x);
    }
    // local g=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_REMOVED,0,nil)
    let g = duel::get_matching_group(d, Some(&tdfilter), a.tp, LOCATION_REMOVED, 0, NO_EX);
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,exc)
    //     and aux.SelectUnselectGroup(g,e,tp,2,2,aux.dncheck,0) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| card::is_faceup(d, x)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, exc.map_or(NO_EX, |x| x.into()))
            && matches!(
                aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&aux::dncheck()), 0, a.tp, None, None, None, false),
                Err(true)
            );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local tg=Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let tg = duel::select_target(d, a.tp, Some(&|d, x| card::is_faceup(d, x)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,2,tp,LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 2, a.tp, LOCATION_REMOVED as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, tg, 1, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_REMOVED,0,nil)
    let g = duel::get_matching_group(d, Some(&tdfilter), a.tp, LOCATION_REMOVED, 0, NO_EX);
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dncheck,1,tp,HINTMSG_TODECK)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&aux::dncheck()), 1, a.tp, Some(HINTMSG_TODECK as u64), None, None, false)
        .unwrap_or_default();
    // if #sg~=2 then return end
    if sg.len() != 2 {
        return;
    }
    // Duel.HintSelection(sg)
    duel::hint_selection(d, &sg, true);
    // if Duel.SendtoDeck(sg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)~=2 then return end
    if duel::send_to_deck(d, &sg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) != 2 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
/// `s.lvrnkfilter(c)`
fn lvrnkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VIRTUAL_WORLD) and c:IsFaceup() and (c:HasLevel() or c:HasRank())
    card::is_set_card(d, c, &[SET_VIRTUAL_WORLD as u16]) && card::is_faceup(d, c) && (card::has_level(d, c) || card::has_rank(d, c))
}
/// `s.lvrnktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn lvrnktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.lvrnkfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && lvrnkfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.lvrnkfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&lvrnkfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.lvrnkfilter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, a.tp, Some(&lvrnkfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.lvrnkop(e,tp,eg,ep,ev,re,r,rp)`
fn lvrnkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and (tc:HasLevel() or tc:HasRank()) then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && (card::has_level(d, tc) || card::has_rank(d, tc)) {
        // local lvrnk=tc:HasLevel() and tc:GetLevel() or tc:GetRank()
        let lvrnk = if card::has_level(d, tc) { card::get_level(d, tc) as i64 } else { card::get_rank(d, tc) as i64 };
        // local b2=lvrnk>3
        let b2 = lvrnk > 3;
        // local op=nil
        // if b2 then op=Duel.SelectEffect(tp,{true,aux.Stringid(id,2)},{b2,aux.Stringid(id,3)})
        // else op=1 end
        let op: Option<usize> = if b2 {
            duel::select_effect(d, a.tp, &[(true, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))])
        } else {
            Some(1)
        };
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local val=op==1 and 3 or -3
        let val: i64 = if op == Some(1) { 3 } else { -3 };
        // if tc:HasLevel() then
        if card::has_level(d, tc) {
            // Increase or decrease its Level by 3
            // tc:UpdateLevel(val,RESETS_STANDARD_PHASE_END,c)
            card::update_level(d, tc, val, Some(RESETS_STANDARD_PHASE_END), Some(c));
        } else if card::has_rank(d, tc) {
            // Increase or decrease its Rank by 3
            // tc:UpdateRank(val,RESETS_STANDARD_PHASE_END,c)
            card::update_rank(d, tc, val, Some(RESETS_STANDARD_PHASE_END), Some(c));
        }
    }
}
