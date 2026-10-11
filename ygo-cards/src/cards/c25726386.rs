//! Megalith Aratron
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 25726386;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_MEGALITH],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Ritual Summon 1 "Megalith" Ritual Monster from your hand, by Tributing monsters from your hand or field whose total Levels equal or exceed its Level
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_RELEASE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_RELEASE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(main_phase_cond)));
    // e1:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e1, Some(cost::self_discard()));
    // e1:SetTarget(Ritual.Target(ritual_target_params))
    let target = ritual::target(d, target_params(c));
    effect::set_target(d, e1, Some(target));
    // e1:SetOperation(Ritual.Operation(ritual_operation_params))
    let operation_fn = ritual::operation(d, operation_params());
    effect::set_operation(d, e1, Some(operation_fn));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate the activation of an opponent's effect that targets a card you control
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.negcon)
    effect::set_condition(d, e2, Some(condition(negcon)));
    // e2:SetTarget(s.negtg)
    effect::set_target(d, e2, Some(cost_target(negtg)));
    // e2:SetOperation(s.negop)
    effect::set_operation(d, e2, Some(operation(negop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// local ritual_target_params={handler=c,lvtype=RITPROC_GREATER,filter=function(ritual_c) return ritual_c:IsSetCard(SET_MEGALITH) and ritual_c~=c end,forcedselection=s.forcedselection}
fn target_params(c: CardId) -> ritual::RitualProc {
    ritual::RitualProc {
        lvtype: RITPROC_GREATER,
        filter: Some(Arc::new(move |d: &mut Duel, x: CardId| card::is_set_card(d, x, &[SET_MEGALITH as u16]) && x != c)),
        forcedselection: Some(Arc::new(|d: &mut Duel, e: EffectId, _tp: u8, g: &CardSet, _sc: CardId| forcedselection(d, e, g))),
        ..Default::default()
    }
}
// local ritual_operation_params={handler=c,lvtype=RITPROC_GREATER,filter=function(ritual_c) return ritual_c:IsSetCard(SET_MEGALITH) end}
// (the Ritual Summon of e1 uses the operation parameters with the same filter as the target only without the self check)
fn operation_params() -> ritual::RitualProc {
    ritual::RitualProc {
        lvtype: RITPROC_GREATER,
        filter: Some(Arc::new(|d: &mut Duel, x: CardId| card::is_set_card(d, x, &[SET_MEGALITH as u16]))),
        ..Default::default()
    }
}
// function s.forcedselection(e,tp,g,sc)
fn forcedselection(d: &mut Duel, e: EffectId, g: &CardSet) -> (bool, bool) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // return not g:IsContains(c),g:IsContains(c)
    (!g.contains(c), g.contains(c))
}
// function() return Duel.IsMainPhase() end
fn main_phase_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
}
// function s.negconfilter(c,tp)
fn negconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsOnField() and c:IsControler(tp)
    card::is_on_field(d, c) && card::is_controler(d, c, tp)
}
// function s.negcon(e,tp,eg,ep,ev,re,r,rp)
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    // if not (re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) and rp==1-tp and Duel.IsChainNegatable(ev)) then return false end
    if !(effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) && a.rp == 1 - a.tp && duel::is_chain_negatable(d, a.ev as u8)) {
        return false;
    }
    // local tg=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    // return tg and tg:IsExists(s.negconfilter,1,nil,tp)
    match tg.first().and_then(|v| v.as_group()).cloned() {
        Some(g) => group::is_exists(d, &g, &|d, x| negconfilter(d, x, a.tp), 1, NO_EX),
        None => false,
    }
}
// function s.tdfilter(c)
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRitualMonster() and c:IsAbleToDeck()
    card::is_ritual_monster(d, c) && card::is_able_to_deck(d, c)
}
// function s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tdfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_GRAVE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, a.tp, 0);
    // if rc:IsDestructable() and rc:IsRelateToEffect(re) then
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, a.tp, 0);
    }
    true
}
// function s.negop(e,tp,eg,ep,ev,re,r,rp)
fn negop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.tdfilter,tp,LOCATION_GRAVE,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&tdfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let Some(sc) = group::get_first(d, &g) else { return };
    // Duel.HintSelection(sc)
    duel::hint_selection(d, CardSet::from_card(sc), true);
    // if Duel.SendtoDeck(sc,nil,SEQ_DECKBOTTOM,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_DECK)
    //     and Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::send_to_deck(d, CardSet::from_card(sc), None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None) > 0
        && card::is_location(d, sc, LOCATION_DECK)
        && duel::negate_activation(d, a.ev as u8)
        && {
            let re = a.re.nil();
            let rc = effect::get_handler(d, re).nil();
            card::is_relate_to_effect(d, rc, re)
        }
    {
        // Duel.Destroy(eg,REASON_EFFECT)
        duel::destroy(d, a.eg.clone().nil(), REASON_EFFECT, None, None);
    }
}
