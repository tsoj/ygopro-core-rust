//! Ragnaraika Wisteria of Woe
use super::prelude::*;

pub const ID: u32 = 77573354;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RAGNARAIKA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card as a Normal Monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Shuffle 2 Insect, Plant, and/or Reptile monsters into the Deck and Set this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_SET);
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
    // e2:SetCondition(function() return Duel.IsPhase(PHASE_END) end)
    effect::set_condition(d, e2, Some(condition(|d, _a| duel::is_phase(d, PHASE_END))));
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id,SET_RAGNARAIKA,TYPE_MONSTER|TYPE_NORMAL,1600,0,4,RACE_PLANT,ATTRIBUTE_DARK) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && duel::is_player_can_special_summon_monster(d, tp, ID, monster_data(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, h.map(CardSet::from_card), 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - tp, LOCATION_GRAVE as i32);
    true
}
// The monster data of the Special Summon in sptg and spop (the Lua arguments of IsPlayerCanSpecialSummonMonster).
fn monster_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_RAGNARAIKA as u16]),
        type_: Some(TYPE_MONSTER | TYPE_NORMAL),
        attack: Some(1600),
        defense: Some(0),
        level: Some(4),
        race: Some(RACE_PLANT),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
// function s.ragnaraikafilter(c)
fn ragnaraikafilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_RAGNARAIKA) and c:IsLinkMonster() and c:IsFaceup()
    card::is_set_card(d, c, &[SET_RAGNARAIKA as u16]) && card::is_link_monster(d, c) && card::is_faceup(d, c)
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsRelateToEffect(e) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id,SET_RAGNARAIKA,TYPE_MONSTER|TYPE_NORMAL,1600,0,4,RACE_PLANT,ATTRIBUTE_DARK)) then return end
    if !(card::is_relate_to_effect(d, c, a.e)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_player_can_special_summon_monster(d, tp, ID, monster_data(), None, None, 0))
    {
        return;
    }
    // c:AddMonsterAttribute(TYPE_NORMAL|TYPE_TRAP)
    card::add_monster_attribute(d, c, TYPE_NORMAL | TYPE_TRAP, 0, 0, 0, 0, 0);
    // Duel.SpecialSummonStep(c,1,tp,tp,true,false,POS_FACEUP)
    duel::special_summon_step(d, c, 1, tp, tp, true, false, POS_FACEUP as u8, None);
    // c:AddMonsterAttributeComplete()
    card::add_monster_attribute_complete(d, c);
    // if Duel.SpecialSummonComplete()==0 then return end
    if duel::special_summon_complete(d) == 0 {
        return;
    }
    // if Duel.IsExistingMatchingCard(s.ragnaraikafilter,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_GRAVE,1,nil)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if duel::is_existing_matching_card(d, Some(&ragnaraikafilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), tp, 0, LOCATION_GRAVE, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToRemove,tp,0,LOCATION_GRAVE,1,2,nil)
        let g = duel::select_matching_card(d, tp, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), tp, 0, LOCATION_GRAVE, 1, 2, false, NO_EX).nil();
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
// function s.tdfilter(c)
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_INSECT|RACE_PLANT|RACE_REPTILE) and c:IsAbleToDeck() and c:IsFaceup()
    card::is_race(d, c, RACE_INSECT | RACE_PLANT | RACE_REPTILE) && card::is_able_to_deck(d, c) && card::is_faceup(d, c)
}
// function s.settg(e,tp,eg,ep,ev,re,r,rp,chk)
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsSSetable() and Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,2,nil) end
    if a.chk == Some(0) {
        return card::is_s_setable(d, c, false, None)
            && duel::is_existing_matching_card(d, Some(&tdfilter), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 2, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,2,tp,LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 2, tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
// function s.setop(e,tp,eg,ep,ev,re,r,rp)
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.tdfilter),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,2,2,nil)
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(tdfilter)),
        tp,
        LOCATION_GRAVE | LOCATION_REMOVED,
        0,
        2,
        2,
        false,
        NO_EX,
    )
    .nil();
    // if #g~=2 then return end
    if g.len() != 2 {
        return;
    }
    // Duel.HintSelection(g)
    duel::hint_selection(d, g.clone(), true);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 and g:IsExists(Card.IsLocation,1,nil,LOCATION_DECK|LOCATION_EXTRA)
    //     and c:IsRelateToEffect(e) and c:IsSSetable() then
    if duel::send_to_deck(d, g.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0
        && group::is_exists(d, &g, &|d, x| card::is_location(d, x, LOCATION_DECK | LOCATION_EXTRA), 1, NO_EX)
        && card::is_relate_to_effect(d, c, a.e)
        && card::is_s_setable(d, c, false, None)
    {
        // Duel.SSet(tp,c)
        duel::s_set(d, tp, c, None, true);
    }
}
