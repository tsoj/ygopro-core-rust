//! Dracotail Sting
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 80208225;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRACOTAIL], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish 1 monster and/or 1 Spell/Trap from your opponent's GY, then you can place 1 "Dracotail" card from your GY or banishment on the bottom of the Deck, except "Dracotail Sting", then draw 1 card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_TODECK + CATEGORY_DRAW);
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
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.rmfilter(c,e)`: `return c:IsAbleToRemove() and c:IsCanBeEffectTarget(e)`
fn rmfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    card::is_able_to_remove(d, c, None, None, None) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.rescon(sg,e,tp,mg)`: `return sg:FilterCount(Card.IsMonster,nil)<=1 and sg:FilterCount(Card.IsSpellTrap,nil)<=1`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let res = group::filter_count(d, sg, &|d, c| card::is_monster(d, c), NO_EX) <= 1
        && group::filter_count(d, sg, &|d, c| card::is_spell_trap(d, c), NO_EX) <= 1;
    (res, false)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_GRAVE) and chkc:IsAbleToRemove() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_GRAVE) && card::is_able_to_remove(d, chkc, None, None, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToRemove,tp,0,LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_GRAVE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.rmfilter,tp,0,LOCATION_GRAVE,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| rmfilter(d, c, e)), tp, 0, LOCATION_GRAVE, NO_EX);
    // local tg=aux.SelectUnselectGroup(g,e,tp,1,2,s.rescon,1,tp,HINTMSG_REMOVE)
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,tg,#tg,tp,0)
    let n = tg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(tg), n, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,1,tp,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 1, tp, 1);
    true
}
/// `s.tdfilter(c)`: `return c:IsSetCard(SET_DRACOTAIL) and c:IsFaceup() and c:IsAbleToDeck() and not c:IsCode(id)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_DRACOTAIL as u16]) && card::is_faceup(d, c) && card::is_able_to_deck(d, c) && !card::is_code(d, c, &[ID])
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, e).nil();
    // if #tg>0 and Duel.Remove(tg,POS_FACEUP,REASON_EFFECT)>0
    //     and Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.tdfilter),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil)
    //     and Duel.IsPlayerCanDraw(tp,1) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if !tg.is_empty()
        && duel::remove(d, tg, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0
        && duel::is_existing_matching_card(d, Some(&aux::necro_valley_filter(tdfilter)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX)
        && duel::is_player_can_draw(d, tp, 1)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.tdfilter),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(tdfilter)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if Duel.SendtoDeck(g,nil,SEQ_DECKBOTTOM,REASON_EFFECT)>0 then
        if duel::send_to_deck(d, g, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None) > 0 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Draw(tp,1,REASON_EFFECT)
            duel::draw(d, tp, 1, REASON_EFFECT);
        }
    }
}
