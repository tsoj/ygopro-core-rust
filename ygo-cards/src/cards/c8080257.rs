//! Flame Swordsdance
use super::prelude::*;

pub const ID: u32 = 8080257;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_FLAME_SWORDSMAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 monster from your Extra Deck and change the position of a monster on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_POSITION + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetTarget(s.exsptg)
    effect::set_target(d, e1, Some(cost_target(exsptg)));
    // e1:SetOperation(s.exspop)
    effect::set_operation(d, e1, Some(operation(exspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Flame Swordsman" or 1 monster that mentions it from your Deck or Extra Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetHintTiming(0,TIMING_BATTLE_START|TIMING_BATTLE_END)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_BATTLE_START | TIMING_BATTLE_END));
    // e2:SetCondition(function(_,tp) return Duel.IsBattlePhase() and Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetCost(s.deckexspcost)
    effect::set_cost(d, e2, Some(cost_target(deckexspcost)));
    // e2:SetTarget(s.deckexsptg)
    effect::set_target(d, e2, Some(cost_target(deckexsptg)));
    // e2:SetOperation(s.deckexspop)
    effect::set_operation(d, e2, Some(operation(deckexspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.exspfilter(c,e,tp)
fn exspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(7) and c:IsType(TYPE_FUSION) and (c:IsCode(CARD_FLAME_SWORDSMAN) or c:ListsCode(CARD_FLAME_SWORDSMAN))
    if !(card::is_level_below(d, c, 7)
        && card::is_type(d, c, TYPE_FUSION)
        && (card::is_code(d, c, &[CARD_FLAME_SWORDSMAN]) || card::lists_code(d, c, &[CARD_FLAME_SWORDSMAN])))
    {
        return false;
    }
    // and Duel.GetLocationCountFromEx(tp,tp,nil,c) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    // (a number is always true in a condition: the call is kept, its value is not used)
    let _ = duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None);
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.exsptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn exsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.exspfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| exspfilter(d, c, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_POSITION,nil,1,PLAYER_ALL,POS_FACEDOWN_DEFENSE)
    duel::set_possible_operation_info(d, 0, CATEGORY_POSITION as u64, None, 1, PLAYER_ALL as u8, POS_FACEDOWN_DEFENSE as i32);
    true
}
// function s.exspop(e,tp,eg,ep,ev,re,r,rp)
fn exspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.exspfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| exspfilter(d, c, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g==0 or Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)==0 then return end
    if g.is_empty() || duel::special_summon(d, g.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    // local pg=Duel.GetMatchingGroup(Card.IsCanTurnSet,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let pg = duel::get_matching_group(d, Some(&|d, c| card::is_can_turn_set(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #pg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if !pg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
        // local tc=pg:Select(tp,1,1,nil)
        let tc = group::select(d, &pg, tp, 1, 1, false, NO_EX).nil();
        // Duel.HintSelection(tc,true)
        duel::hint_selection(d, tc.clone(), true);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.ChangePosition(tc,POS_FACEDOWN_DEFENSE)
        duel::change_position(d, tc, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    }
}
// function s.costfilter(c,e,tp)
fn costfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsPosition(POS_FACEDOWN_DEFENSE)
    //     and Duel.IsExistingMatchingCard(s.deckexspfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,nil,e,tp,c)
    card::is_position(d, c, POS_FACEDOWN_DEFENSE)
        && duel::is_existing_matching_card(d, Some(&|d, x| deckexspfilter(d, x, e, tp, Some(c))), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX)
}
// function s.deckexspfilter(c,e,tp,exc)
fn deckexspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, exc: Option<CardId>) -> bool {
    // (nil exc is NO_EX)
    let ex = exc.map_or(NO_EX, Into::into);
    // if c:IsLocation(LOCATION_DECK) and Duel.GetMZoneCount(tp,exc)<=0 then return false end
    if card::is_location(d, c, LOCATION_DECK) && duel::get_m_zone_count(d, tp, ex, None, None, None).0 <= 0 {
        return false;
    }
    // if c:IsLocation(LOCATION_EXTRA) and Duel.GetLocationCountFromEx(tp,tp,exc,c)<=0 then return false end
    if card::is_location(d, c, LOCATION_EXTRA) && duel::get_location_count_from_ex(d, tp, Some(tp), ex, duel::ExtraCard::Card(c), None).0 <= 0 {
        return false;
    }
    // return (c:IsCode(CARD_FLAME_SWORDSMAN) or c:ListsCode(CARD_FLAME_SWORDSMAN)) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    (card::is_code(d, c, &[CARD_FLAME_SWORDSMAN]) || card::lists_code(d, c, &[CARD_FLAME_SWORDSMAN]))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.deckexspcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn deckexspcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemoveAsCost() and Duel.CheckReleaseGroupCost(tp,s.costfilter,1,false,nil,nil,e,tp) end
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None)
            && aux::check_release_group_cost(d, tp, Some(&|d, x| costfilter(d, x, a.e, tp)), 1, 1, false, None, NO_EX);
    }
    // Duel.Remove(c,POS_FACEUP,REASON_COST)
    duel::remove(d, c, POS_FACEUP as u8, REASON_COST, None, None);
    // local g=Duel.SelectReleaseGroupCost(tp,s.costfilter,1,1,false,nil,nil,e,tp)
    let g = aux::select_release_group_cost(d, tp, Some(&|d, x| costfilter(d, x, a.e, tp)), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
// function s.deckexsptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn deckexsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.deckexspfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| deckexspfilter(d, x, a.e, tp, None)), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    true
}
// function s.deckexspop(e,tp,eg,ep,ev,re,r,rp)
fn deckexspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.deckexspfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, x| deckexspfilter(d, x, a.e, tp, None)), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if sc then
    if let Some(sc) = group::get_first(d, &sel) {
        // Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
// function(_,tp) return Duel.IsBattlePhase() and Duel.IsTurnPlayer(1-tp) end
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_battle_phase(d, None) && duel::is_turn_player(d, 1 - a.tp)
}
