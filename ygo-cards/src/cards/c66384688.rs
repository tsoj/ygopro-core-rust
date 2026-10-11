//! Majespecter Orthrus - Nue
use super::prelude::*;

pub const ID: u32 = 66384688;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAJESPECTER, SET_DRACOSLAYER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // 2 Pendulum Monsters, including a "Majespecter" monster
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_PENDULUM),2,2,s.lcheck)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_PENDULUM, sc, st, Some(tp)))),
        2,
        Some(2),
        Some(group_check(|d, g, lc, st, tp| lcheck(d, g, lc, st, tp))),
        None,
        None,
    );
    // Add 2 "Majespecter" Pendulum Monsters to the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_TOEXTRA)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_TOEXTRA);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsLinkSummoned() end)
    effect::set_condition(d, e1, Some(condition(link_summoned_cond)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.lcheck(g,lc,sumtype,tp)`
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsSetCard,1,nil,SET_MAJESPECTER,lc,sumtype,tp)
    group::is_exists(d, g, &|d, c| card::is_set_card_ctx(d, c, &[SET_MAJESPECTER as u16], Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
/// `function(e) return e:GetHandler():IsLinkSummoned() end`
fn link_summoned_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, h)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_MAJESPECTER) and c:IsType(TYPE_PENDULUM) and c:IsAbleToHand()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MAJESPECTER as u16]) && card::is_type(d, c, TYPE_PENDULUM) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,...,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_EXTRA as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOEXTRA,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOEXTRA, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.tefilter(c)`
fn tefilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MAJESPECTER) and c:IsType(TYPE_PENDULUM) and not c:IsForbidden()
    card::is_set_card(d, c, &[SET_MAJESPECTER as u16]) && card::is_type(d, c, TYPE_PENDULUM) && !card::is_forbidden(d, c)
}
/// `s.thop(e,tp,...)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Cannot Special Summon from the Extra Deck, except Xyz Monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(_,c) return not c:IsSetCard({SET_MAJESPECTER,SET_DRACOSLAYER}) and c:IsLocation(LOCATION_EXTRA) end)
    effect::set_target(
        d,
        e1,
        Some(effect_card_fn(|d, _e, c| !card::is_set_card(d, c, &[SET_MAJESPECTER as u16, SET_DRACOSLAYER as u16]) && card::is_location(d, c, LOCATION_EXTRA))),
    );
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // Clock Lizard check
    // aux.addTempLizardCheck(c,tp,function(_,c) return not c:IsOriginalSetCard({SET_MAJESPECTER,SET_DRACOSLAYER}) end)
    aux::add_temp_lizard_check(
        d,
        c,
        tp,
        Some(effect_card_fn(|d, _e, c| !card::is_original_set_card(d, c, &[SET_MAJESPECTER as u16, SET_DRACOSLAYER as u16]))),
        None,
        None,
        None,
        None,
    );
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_EXTRA,0,1,2,nil)
    let mut g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_EXTRA, 0, 1, 2, false, NO_EX).nil();
    // if #g==0 or Duel.SendtoHand(g,tp,REASON_EFFECT)==0 or #g:Match(Card.IsLocation,nil,LOCATION_HAND)==0 then return end
    if g.is_empty() {
        return;
    }
    if duel::send_to_hand(d, g.clone(), Some(tp), REASON_EFFECT, None) == 0 {
        return;
    }
    group::match_(d, &mut g, &|d, c| card::is_location(d, c, LOCATION_HAND), NO_EX);
    if g.is_empty() {
        return;
    }
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g);
    // local dg=Duel.GetMatchingGroup(s.tefilter,tp,LOCATION_DECK,0,nil)
    let dg = duel::get_matching_group(d, Some(&tefilter), tp, LOCATION_DECK, 0, NO_EX);
    // if #dg==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,2)) then return end
    if dg.is_empty() {
        return;
    }
    if !duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        return;
    }
    // local teg=aux.SelectUnselectGroup(dg,e,tp,1,2,aux.dncheck,1,tp,HINTMSG_CONFIRM)
    let teg = aux::select_unselect_group(d, &dg, Some(a.e), tp, Some(1), Some(2), Some(&aux::dncheck()), 1, tp, Some(HINTMSG_CONFIRM as u64), None, None, false).unwrap_or_default();
    // if #teg>0 then
    if !teg.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SendtoExtraP(teg,tp,REASON_EFFECT)
        duel::send_to_extra_p(d, teg, Some(tp), REASON_EFFECT, None);
    }
}
