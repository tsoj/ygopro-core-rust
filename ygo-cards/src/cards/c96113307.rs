//! Ritual Beast Ulti-Reirautari
//! 聖霊獣騎 レイラウタリ
use super::prelude::*;

pub const ID: u32 = 96113307;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RITUAL_BEAST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon Procedure
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),3)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))), 3, None, None, None, None);
    // Neither player can Tribute cards to activate a card effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_RELEASE)
    effect::set_code(d, e1, EFFECT_CANNOT_RELEASE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(1,1)
    effect::set_target_range(d, e1, 1, 1);
    // e1:SetTarget(function(e,c,rp,r,re) return r&REASON_COST>0 and re and re:IsActivated() end)
    effect::set_target(d, e1, Some(lua_fn(cannot_release)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return 1 "Ritual Beast" card to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_TOEXTRA+CATEGORY_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_TOEXTRA + CATEGORY_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Banish 1 of your "Ritual Beast" cards and 1 of your opponent's cards
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_REMOVE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e3:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e3, Some(condition(|d, a| duel::is_turn_player(d, 1 - a.tp))));
    // e3:SetTarget(s.rmvtg)
    effect::set_target(d, e3, Some(cost_target(rmvtg)));
    // e3:SetOperation(s.rmvop)
    effect::set_operation(d, e3, Some(operation(rmvop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e,c,rp,r,re) return r&REASON_COST>0 and re and re:IsActivated() end`
fn cannot_release(d: &mut Duel, a: Args) -> bool {
    let r = a.int(3) as u32;
    let Some(re) = a.v(4).as_effect() else { return false };
    r & REASON_COST > 0 && effect::is_activated(d, re)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_RITUAL_BEAST) and c:IsFaceup() and (c:IsAbleToHand() or c:IsAbleToExtra())
    card::is_set_card(d, c, &[SET_RITUAL_BEAST as u16]) && card::is_faceup(d, c) && (card::is_able_to_hand(d, c, None) || card::is_able_to_extra(d, c))
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_REMOVED) and chkc:IsControler(tp) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_REMOVED) && card::is_controler(d, chkc, a.tp) && thfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
    // local tc=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_REMOVED,0,1,1,nil):GetFirst()
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g).nil();
    // local category=tc:IsType(TYPE_EXTRA) and CATEGORY_TOEXTRA or CATEGORY_TOHAND
    let category = if card::is_type(d, tc, TYPE_EXTRA) { CATEGORY_TOEXTRA } else { CATEGORY_TOHAND };
    // Duel.SetOperationInfo(0,category,tc,1,tp,0)
    duel::set_operation_info(d, 0, category, Some(CardSet::from_card(tc)), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.nsfilter(c)`
fn nsfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_RITUAL_BEAST) and c:IsSummonable(true,nil)
    card::is_set_card(d, c, &[SET_RITUAL_BEAST as u16]) && card::is_summonable(d, c, true, None, 0, None)
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and Duel.SendtoHand(tc,nil,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_HAND|LOCATION_EXTRA)) then return end
    if !(card::is_relate_to_effect(d, tc, a.e)
        && duel::send_to_hand(d, CardSet::from_card(tc), None, REASON_EFFECT, None) > 0
        && card::is_location(d, tc, LOCATION_HAND | LOCATION_EXTRA))
    {
        return;
    }
    // if tc:IsLocation(LOCATION_HAND) then Duel.ShuffleHand(tp) end
    if card::is_location(d, tc, LOCATION_HAND) {
        duel::shuffle_hand(d, a.tp);
    }
    // if Duel.IsExistingMatchingCard(s.nsfilter,tp,LOCATION_HAND,0,1,nil)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
    if duel::is_existing_matching_card(d, Some(&nsfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 3))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.nsfilter,tp,LOCATION_HAND,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&nsfilter), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.Summon(tp,g:GetFirst(),true,nil)
            let sc = group::get_first(d, &g).nil();
            duel::summon(d, a.tp, sc, true, None, 0, None);
        }
    }
}
/// `s.rmvfilter(c,e,tp)`
fn rmvfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsAbleToRemove() and c:IsCanBeEffectTarget(e)
    //     and ((c:IsFaceup() and c:IsSetCard(SET_RITUAL_BEAST)) or c:IsControler(1-tp))
    card::is_able_to_remove(d, c, None, None, None)
        && card::is_can_be_effect_target(d, c, Some(e))
        && ((card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_RITUAL_BEAST as u16])) || card::is_controler(d, c, 1 - tp))
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsControler,nil,tp)==1
    let res = group::filter_count(d, sg, &|d, c| card::is_controler(d, c, tp), NO_EX) == 1;
    (res, false)
}
/// `s.rmvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.rmvfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| rmvfilter(d, c, a.e, a.tp)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    let rs: aux::Rescon = &rescon;
    // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return matches!(aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(rs), 0, a.tp, None, None, None, false), Err(true));
    }
    // local rg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_REMOVE)
    let rg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(rs), 1, a.tp, Some(HINTMSG_REMOVE as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(rg)
    duel::set_target_card(d, rg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,rg,#rg,0,0)
    let n = rg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(rg), n, 0, 0);
    true
}
/// `s.rmvop(e,tp,eg,ep,ev,re,r,rp)`
fn rmvop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
