//! 速攻召喚
//! Quick Summon
use super::prelude::*;

pub const ID: u32 = 10086952;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Immediately after this effect resolves, Normal Summon/Set 1 monster. When you do, if your opponent controls a monster, you can Normal Summon a Level 5 or higher monster without Tributing
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SUMMON)
    effect::set_category(d, e1, CATEGORY_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During your Main Phase, if this card is in your GY, except the turn it was sent there: You can banish it; add 1 Level 5 or higher monster that can be Normal Summoned/Set from your Deck or GY to your hand, then immediately after this effect resolves, Tribute Summon it. You can only use this effect of "Quick Summon" once per turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(aux.exccon)
    effect::set_condition(d, e2, Some(aux::exccon()));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.nsfilter(c,no_tribute_chk,handler)`
fn nsfilter(d: &mut Duel, c: CardId, no_tribute_chk: bool, handler: CardId) -> bool {
    // if c:CanSummonOrSet(true,nil) then return true end
    if card::can_summon_or_set(d, c, true, None, 0, None) {
        return true;
    }
    // if c:IsLevelAbove(5) and c:IsLocation(LOCATION_HAND) and no_tribute_chk then
    if card::is_level_above(d, c, 5) && card::is_location(d, c, LOCATION_HAND) && no_tribute_chk {
        // local e1=Effect.CreateEffect(handler)
        let e1 = effect::create_effect(d, handler);
        // e1:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e1, aux::stringid(ID, 2));
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SUMMON_PROC)
        effect::set_code(d, e1, EFFECT_SUMMON_PROC);
        // e1:SetCondition(function(e,c,minc) return c==nil or minc==0 end)
        effect::set_condition(d, e1, Some(lua_fn(|_d, a: Args| a.card(1).is_none() || a.int(2) == 0)));
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local res=c:IsSummonable(true,nil)
        let res = card::is_summonable(d, c, true, None, 0, None);
        // e1:Reset()
        effect::reset(d, e1);
        // return res
        return res;
    }
    false
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // local no_tribute_chk=Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        let no_tribute_chk = duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE) > 0
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
        // return Duel.IsExistingMatchingCard(s.nsfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil,no_tribute_chk,e:GetHandler())
        let handler = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| nsfilter(d, c, no_tribute_chk, handler)),
            a.tp,
            LOCATION_HAND | LOCATION_MZONE,
            0,
            1,
            NO_EX,
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_MZONE) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local no_tribute_chk=Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let no_tribute_chk = duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) > 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.nsfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil,no_tribute_chk,c):GetFirst()
    let sg = duel::select_matching_card(
        d,
        tp,
        Some(&|d, x| nsfilter(d, x, no_tribute_chk, c)),
        tp,
        LOCATION_HAND | LOCATION_MZONE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if not sc then return end
    let Some(sc) = group::get_first(d, &sg) else {
        return;
    };
    // if sc:IsLevelAbove(5) and sc:IsLocation(LOCATION_HAND) and no_tribute_chk then
    if card::is_level_above(d, sc, 5) && card::is_location(d, sc, LOCATION_HAND) && no_tribute_chk {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e1, aux::stringid(ID, 2));
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SUMMON_PROC)
        effect::set_code(d, e1, EFFECT_SUMMON_PROC);
        // e1:SetCondition(function(e,c,minc) return c==nil or minc==0 end)
        effect::set_condition(d, e1, Some(lua_fn(|_d, a: Args| a.card(1).is_none() || a.int(2) == 0)));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // sc:RegisterEffect(e1)
        card::register_effect(d, sc, e1, false);
    }
    // Duel.SummonOrSet(tp,sc,true,nil)
    duel::summon_or_set(d, tp, sc, true, None, 0, None);
}
/// `s.thfilter(c,tp)`
fn thfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsLevelAbove(5) and c:IsSummonableCard() and Duel.CheckTribute(c,c:GetTributeRequirement()) and c:CanSummonOrSet(true,nil,1)
    //     and Duel.IsPlayerCanSummon(tp,SUMMON_TYPE_TRIBUTE,c) and c:IsAbleToHand()
    if !(card::is_level_above(d, c, 5) && card::is_summonable_card(d, c)) {
        return false;
    }
    let (req_min, req_max) = card::get_tribute_requirement(d, c);
    if !duel::check_tribute(d, c, req_min as u16, Some(req_max as u16), None, None, None) {
        return false;
    }
    card::can_summon_or_set(d, c, true, None, 1, None)
        && duel::is_player_can_summon(d, tp, Some((SUMMON_TYPE_TRIBUTE, c)))
        && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        // return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,tp)
        return duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c, tp)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,tp):GetFirst()
    let nv = aux::necro_valley_filter(|d, c| thfilter(d, c, tp));
    let sg = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if not sc then return end
    let Some(sc) = group::get_first(d, &sg) else {
        return;
    };
    // if sc:IsLocation(LOCATION_GRAVE) then Duel.HintSelection(sc) end
    if card::is_location(d, sc, LOCATION_GRAVE) {
        duel::hint_selection(d, sc, true);
    }
    // if Duel.SendtoHand(sc,nil,REASON_EFFECT)>0 then
    if duel::send_to_hand(d, sc, None, REASON_EFFECT, None) > 0 {
        // if sc:IsPreviousLocation(LOCATION_DECK) then Duel.ConfirmCards(1-tp,sc) end
        if card::is_previous_location(d, sc, LOCATION_DECK) {
            duel::confirm_cards(d, 1 - tp, sc);
        }
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SummonOrSet(tp,sc,true,nil,1)
        duel::summon_or_set(d, tp, sc, true, None, 1, None);
    }
}
