//! Regenesis Birth
use super::prelude::*;

pub const ID: u32 = 27781371;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_REGENESIS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Change 1 face-up monster your opponent controls to face-down Defense Position
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.poscost)
    effect::set_cost(d, e1, Some(cost_target(poscost)));
    // e1:SetTarget(s.postg)
    effect::set_target(d, e1, Some(cost_target(postg)));
    // e1:SetOperation(s.posop)
    effect::set_operation(d, e1, Some(operation(posop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon as many "Regenesis" monsters with different names as possible from your hand, GY, and/or banishment
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e2, Some(condition(opponent_turn_cond)));
    // e2:SetCost(s.spcost)
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp) return Duel.IsTurnPlayer(1-tp) end`
fn opponent_turn_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.poscostfilter(c)`
fn poscostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_REGENESIS) and c:IsMonster() and c:IsAbleToGraveAsCost()
    card::is_set_card(d, c, &[SET_REGENESIS as u16]) && card::is_monster(d, c) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.poscost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn poscost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.poscostfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&poscostfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.poscostfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&poscostfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.postg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsCanTurnSet() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && card::is_can_turn_set(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsCanTurnSet,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&card::is_can_turn_set), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsCanTurnSet,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&card::is_can_turn_set), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,1,tp,POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, g, 1, tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc.nil(), a.e) {
        // Duel.ChangePosition(tc,POS_FACEDOWN_DEFENSE)
        duel::change_position(d, CardSet::from_card(tc.nil()), POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    }
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToGraveAsCost() and c:IsStatus(STATUS_EFFECT_ENABLED) end
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, c) && card::is_status(d, c, STATUS_EFFECT_ENABLED);
    }
    // Duel.SendtoGrave(c,REASON_COST)
    duel::send_to_grave(d, c, REASON_COST, None, None);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_REGENESIS) and (c:IsFaceup() or c:IsLocation(LOCATION_HAND))
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_REGENESIS as u16])
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| spfilter(d, c, e, tp)),
                tp,
                LOCATION_HAND | LOCATION_GRAVE | LOCATION_REMOVED,
                0,
                1,
                NO_EX,
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,0,LOCATION_HAND|LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 0, (LOCATION_HAND | LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,0,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, 0, LOCATION_MZONE as i32);
    true
}
/// `function(dg) Duel.SendtoGrave(dg,REASON_EFFECT) end` (the delayed operation)
fn send_to_grave_dg(d: &mut Duel, dg: &CardSet, _a: &EffArgs) -> bool {
    duel::send_to_grave(d, dg.clone(), REASON_EFFECT, None, None);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local tg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_REMOVED,0,nil,e,tp)
    let tg = duel::get_matching_group(
        d,
        Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp))),
        tp,
        LOCATION_HAND | LOCATION_GRAVE | LOCATION_REMOVED,
        0,
        NO_EX,
    );
    // if #tg==0 or ft<=0 then return end
    if tg.is_empty() || ft <= 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // local ct=math.min(ft,tg:GetClassCount(Card.GetCode))
    let classes = group::get_class_count(d, &tg, &|d, x| card::get_code(d, x)[0] as i64) as i32;
    let ct = ft.min(classes);
    // local g=aux.SelectUnselectGroup(tg,e,tp,ct,ct,aux.dncheck,1,tp,HINTMSG_SPSUMMON)
    let g = aux::select_unselect_group(d, &tg, Some(e), tp, Some(ct as usize), Some(ct as usize), Some(&aux::dncheck()), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
    // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)>0 then
    if !g.is_empty() && duel::special_summon(d, g.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // Send them to the GY during the End Phase
        // aux.DelayedOperation(g,PHASE_END,id,e,tp, function(dg) ... end, nil, 0,1,aux.Stringid(id,2))
        aux::delayed_operation(
            d,
            g,
            PHASE_END,
            ID,
            e,
            tp,
            Some(std::sync::Arc::new(send_to_grave_dg)),
            None,
            Some(0),
            Some(1),
            Some(aux::stringid(ID, 2)),
            None,
        );
    }
}
