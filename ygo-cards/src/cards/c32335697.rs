//! Haunted Zombies
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 32335697;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID + 1], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish a Zombie from either GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Set self from GY
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
    // e2:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_END_PHASE as u32));
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.rmfilter(c,tp)`
fn rmfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRace(RACE_ZOMBIE) and c:HasLevel() and c:IsAbleToRemove()
    //     and aux.SpElimFilter(c,true) and Duel.GetMZoneCount(tp,c)>1
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,0,0,c:GetLevel(),RACE_ZOMBIE,ATTRIBUTE_DARK)
    if !(card::is_race(d, c, RACE_ZOMBIE)
        && card::has_level(d, c)
        && card::is_able_to_remove(d, c, None, None, None)
        && aux::sp_elim_filter(d, c, true, false))
    {
        return false;
    }
    if duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 <= 1 {
        return false;
    }
    let level = card::get_level(d, c);
    duel::is_player_can_special_summon_monster(
        d,
        tp,
        ID + 1,
        duel::MonsterData {
            setcodes: Some(vec![0]),
            type_: Some(TYPES_TOKEN),
            attack: Some(0),
            defense: Some(0),
            level: Some(level),
            race: Some(RACE_ZOMBIE),
            attribute: Some(ATTRIBUTE_DARK),
        },
        None,
        None,
        0,
    )
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.IsExistingMatchingCard(s.rmfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::is_existing_matching_card(d, Some(&|d, c| rmfilter(d, c, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,PLAYER_EITHER,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, PLAYER_EITHER as u8, LOCATION_GRAVE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 2, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.rmfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,1,1,nil,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| rmfilter(d, c, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
    // if #g<1 or Duel.Remove(g,POS_FACEUP,REASON_COST)<1 then return end
    if g.is_empty() || duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_COST, None, None) < 1 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local lv=g:GetFirst():GetLevel()
    let first = group::get_first(d, &g).nil();
    let lv = card::get_level(d, first);
    // for i=1,2 do
    for _ in 0..2 {
        // local token=Duel.CreateToken(tp,id+1)
        let token = duel::create_token(d, tp, ID + 1).nil();
        // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        // e1:SetValue(lv)
        effect::set_value(d, e1, lv as i64);
        // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
        effect::set_reset(d, e1, RESET_EVENT + (RESETS_STANDARD & !RESET_TOFIELD), 1);
        // token:RegisterEffect(e1)
        card::register_effect(d, token, e1, false);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_ZOMBIE) and c:IsAbleToDeck()
    card::is_faceup(d, c) && card::is_race(d, c, RACE_ZOMBIE) && card::is_able_to_deck(d, c)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsSSetable() and Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return card::is_s_setable(d, c, false, None) && duel::is_existing_matching_card(d, Some(&setfilter), tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, LOCATION_REMOVED as i32);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_REMOVED,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&setfilter), tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.HintSelection(g,true)
    duel::hint_selection(d, g.clone(), true);
    // if Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 and g:GetFirst():IsLocation(LOCATION_DECK|LOCATION_EXTRA)
    //     and c:IsRelateToEffect(e) and c:IsSSetable() then
    if duel::send_to_deck(d, g.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0 {
        let first = group::get_first(d, &g).nil();
        if card::is_location(d, first, LOCATION_DECK | LOCATION_EXTRA) && card::is_relate_to_effect(d, c, a.e) && card::is_s_setable(d, c, false, None) {
            // Duel.SSet(tp,c)
            duel::s_set(d, tp, CardSet::from_card(c), None, true);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(3300)
            effect::set_description(d, e1, 3300);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
            effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
            effect::set_reset(d, e1, RESET_EVENT + RESETS_REDIRECT, 1);
            // e1:SetValue(LOCATION_REMOVED)
            effect::set_value(d, e1, LOCATION_REMOVED as i64);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
    }
}
