//! Return of the Zombies
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 91742238;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish 1 Zombie monster on the field, then Special Summon 1 Zombie monster from the GY of the player who controlled it, to their field, in Defense Position
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E as u32));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Shuffle 1 of your banished Zombie monsters into the Deck, and if you do, Set this card, but banish it when it leaves the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_LEAVE_GRAVE+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_LEAVE_GRAVE + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E as u32));
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.rmfilter(c,e,tp)`
fn rmfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local ctrl=c:GetControler()
    let ctrl = card::get_controler(d, c);
    // return c:IsRace(RACE_ZOMBIE) and c:IsFaceup() and c:IsAbleToRemove() and Duel.GetMZoneCount(ctrl,c,tp)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,ctrl,LOCATION_GRAVE,0,1,nil,e,tp,ctrl)
    if !(card::is_race(d, c, RACE_ZOMBIE) && card::is_faceup(d, c) && card::is_able_to_remove(d, c, None, None, None)) {
        return false;
    }
    if duel::get_m_zone_count(d, ctrl, c.into(), Some(tp), None, None).0 <= 0 {
        return false;
    }
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, ctrl)), ctrl, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,ctrl)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ctrl: u8) -> bool {
    // return c:IsRace(RACE_ZOMBIE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE,ctrl)
    card::is_race(d, c, RACE_ZOMBIE) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), Some(ctrl), None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rmfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| rmfilter(d, c, e, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,PLAYER_EITHER,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, PLAYER_EITHER as u8, LOCATION_MZONE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,PLAYER_EITHER,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, PLAYER_EITHER as u8, LOCATION_GRAVE as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local rc=Duel.SelectMatchingCard(tp,s.rmfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil,e,tp):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| rmfilter(d, c, e, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // if not rc then return end
    let Some(rc) = group::get_first(d, &sel) else { return };
    // Duel.HintSelection(rc)
    duel::hint_selection(d, CardSet::from_card(rc), true);
    // if Duel.Remove(rc,POS_FACEUP,REASON_EFFECT)>0 then
    if duel::remove(d, CardSet::from_card(rc), POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 {
        // local ctrl=rc:GetPreviousControler()
        let ctrl = card::get_previous_controler(d, rc);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,ctrl,LOCATION_GRAVE,0,1,1,nil,e,tp,ctrl)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, ctrl)), ctrl, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(g,0,tp,ctrl,false,false,POS_FACEUP_DEFENSE)
            duel::special_summon(d, g, 0, tp, ctrl, false, false, POS_FACEUP_DEFENSE as u8, None);
        }
    }
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_ZOMBIE) and c:IsFaceup() and c:IsAbleToDeck()
    card::is_race(d, c, RACE_ZOMBIE) && card::is_faceup(d, c) && card::is_able_to_deck(d, c)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsSSetable() and Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return card::is_s_setable(d, c, false, None) && duel::is_existing_matching_card(d, Some(&tdfilter), tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, LOCATION_REMOVED as i32);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.tdfilter,tp,LOCATION_REMOVED,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&tdfilter), tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    // if not sc then return end
    let Some(sc) = group::get_first(d, &sel) else { return };
    // Duel.HintSelection(sc)
    duel::hint_selection(d, CardSet::from_card(sc), true);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.SendtoDeck(sc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_DECK|LOCATION_EXTRA)
    //     and c:IsRelateToEffect(e) and Duel.SSet(tp,c)>0 then
    if duel::send_to_deck(d, CardSet::from_card(sc), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0
        && card::is_location(d, sc, LOCATION_DECK | LOCATION_EXTRA)
        && card::is_relate_to_effect(d, c, a.e)
        && duel::s_set(d, tp, CardSet::from_card(c), None, true) > 0
    {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3300)
        effect::set_description(d, e1, 3300);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        // e1:SetValue(LOCATION_REMOVED)
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
        effect::set_reset(d, e1, RESET_EVENT + RESETS_REDIRECT, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
