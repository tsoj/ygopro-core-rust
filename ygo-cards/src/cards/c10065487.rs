//! Branded Loss
use super::prelude::*;

pub const ID: u32 = 10065487;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ALBAZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON);
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
/// `s.tdfilter(c,e,tp)`
fn tdfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeEffectTarget(e) and c:IsAbleToDeck()
    //     and ((c:IsControler(tp) and c:IsFaceup() and c:IsType(TYPE_FUSION))
    //     or (c:IsControler(1-tp) and c:IsSpecialSummoned() and c:IsSummonLocation(LOCATION_EXTRA)))
    card::is_can_be_effect_target(d, c, Some(e))
        && card::is_able_to_deck(d, c)
        && ((card::is_controler(d, c, tp) && card::is_faceup(d, c) && card::is_type(d, c, TYPE_FUSION))
            || (card::is_controler(d, c, 1 - tp) && card::is_special_summoned(d, c) && card::is_summon_location(d, c, LOCATION_EXTRA)))
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsControler,nil,tp)==1
    (group::filter_count(d, sg, &|d, c| card::is_controler(d, c, tp), NO_EX) == 1, false)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local rg=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,e,tp)
    let rg = duel::get_matching_group(d, Some(&|d, c| tdfilter(d, c, a.e, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return matches!(aux::select_unselect_group(d, &rg, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true));
    }
    // local tg=aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,1,tp,HINTMSG_TODECK)
    let tg = aux::select_unselect_group(d, &rg, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_TODECK as u64), None, None, false)
        .unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,tg,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(tg), 2, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,PLAYER_EITHER,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, PLAYER_EITHER as u8, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).unwrap_or_default();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.SendtoDeck(tg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, tg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
    // During the End Phase of this turn, each player can Special Summon 1 Fusion Monster that mentions "Fallen of Albaz" as material from their own Extra Deck
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.spfilter(c,e)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return c:IsType(TYPE_FUSION) and c:ListsCodeAsMaterial(CARD_ALBAZ) and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_type(d, c, TYPE_FUSION)
        && card::lists_code_as_material(d, c, &[CARD_ALBAZ])
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.spfilter,0,LOCATION_EXTRA,LOCATION_EXTRA,1,nil,e)
    duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e)), 0, LOCATION_EXTRA, LOCATION_EXTRA, 1, NO_EX)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.spfilter,0,LOCATION_EXTRA,LOCATION_EXTRA,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e)), 0, LOCATION_EXTRA, LOCATION_EXTRA, NO_EX);
    // local turn_p=Duel.GetTurnPlayer()
    let turn_p = duel::get_turn_player(d);
    // local step=turn_p==0 and 1 or -1
    // for p=turn_p,1-turn_p,step do (the turn player first, then the other player)
    for p in [turn_p, 1 - turn_p] {
        // if g:IsExists(Card.IsControler,1,nil,p) and Duel.SelectYesNo(p,aux.Stringid(id,1)) then
        if group::is_exists(d, &g, &|d, c| card::is_controler(d, c, p), 1, NO_EX) && duel::select_yes_no(d, p, aux::stringid(ID, 1)) {
            // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_SPSUMMON as u64);
            // local sc=g:Filter(Card.IsControler,nil,p):Select(p,1,1,nil):GetFirst()
            let filtered = group::filter(d, &g, &|d, c| card::is_controler(d, c, p), NO_EX);
            let selected = group::select(d, &filtered, p, 1, 1, false, NO_EX).nil();
            let sc = group::get_first(d, &selected);
            // if sc then
            if let Some(sc) = sc {
                // Duel.SpecialSummonStep(sc,0,p,p,false,false,POS_FACEUP)
                duel::special_summon_step(d, sc, 0, p, p, false, false, POS_FACEUP as u8, None);
            }
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
