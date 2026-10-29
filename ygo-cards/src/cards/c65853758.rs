//! Yummy★Redemption
//! ヤミー★リデンプション
use super::prelude::*;

pub const ID: u32 = 65853758;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_YUMMY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e0, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // e0:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e0, TIMING_DAMAGE_STEP as u32, None);
    // e0:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e0, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Monsters your opponent controls lose 200 ATK for each LIGHT Beast monster you control and in your GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Draw 1 card, then place 1 card from your hand on the bottom of the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DRAW+CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_DRAW + CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,0,EFFECT_COUNT_CODE_CHAIN)
    effect::set_count_limit(d, e2, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    // e2:SetCondition(s.drwcon)
    effect::set_condition(d, e2, Some(condition(drwcon)));
    // e2:SetTarget(s.drwtg)
    effect::set_target(d, e2, Some(cost_target(drwtg)));
    // e2:SetOperation(s.drwop)
    effect::set_operation(d, e2, Some(operation(drwop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Switch control of 1 "Yummy" monster you control and 1 monster your opponent controls
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e3, CATEGORY_CONTROL);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e3, Some(cost::self_banish()));
    // e3:SetTarget(s.ctrltg)
    effect::set_target(d, e3, Some(cost_target(ctrltg)));
    // e3:SetOperation(s.ctrlop)
    effect::set_operation(d, e3, Some(operation(ctrlop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.atkvalfilter(c)`
fn atkvalfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_BEAST) and c:IsFaceup()
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_BEAST) && card::is_faceup(d, c)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    // return -200*Duel.GetMatchingGroupCount(s.atkvalfilter,e:GetHandlerPlayer(),LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let p = effect::get_handler_player(d, e);
    -200 * duel::get_matching_group_count(d, Some(&atkvalfilter), p, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX) as i64
}
/// `s.drwconfilter(c,tp)`
fn drwconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSynchroSummoned() and c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_BEAST) and c:IsSummonPlayer(tp) and c:IsFaceup()
    card::is_synchro_summoned(d, c)
        && card::is_attribute(d, c, ATTRIBUTE_LIGHT)
        && card::is_race(d, c, RACE_BEAST)
        && card::is_summon_player(d, c, tp)
        && card::is_faceup(d, c)
}
/// `s.drwcon(e,tp,eg,ep,ev,re,r,rp)`
fn drwcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.drwconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| drwconfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.drwtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drwtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.drwop(e,tp,eg,ep,ev,re,r,rp)`
fn drwop(d: &mut Duel, a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let count = info[1].int() as u32;
    // if Duel.Draw(p,d,REASON_EFFECT)==0 then return end
    if duel::draw(d, p, count, REASON_EFFECT) == 0 {
        return;
    }
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToDeck,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_able_to_deck(d, c)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SendtoDeck(g,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
        duel::send_to_deck(d, g, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
    }
}
/// `s.selffilter(c,e,tp)`
fn selffilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_YUMMY) and c:IsAbleToChangeControler() and c:IsFaceup() and Duel.GetMZoneCount(tp,c,tp,LOCATION_REASON_CONTROL)>0
    //   and c:IsCanBeEffectTarget(e)
    card::is_set_card(d, c, &[SET_YUMMY as u16])
        && card::is_able_to_change_controler(d, c)
        && card::is_faceup(d, c)
        && duel::get_m_zone_count(d, tp, c.into(), Some(tp), Some(LOCATION_REASON_CONTROL), None).0 > 0
        && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.oppfilter(c,e,tp)`
fn oppfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsAbleToChangeControler() and Duel.GetMZoneCount(tp,c,tp,LOCATION_REASON_CONTROL)>0 and c:IsCanBeEffectTarget(e)
    card::is_able_to_change_controler(d, c)
        && duel::get_m_zone_count(d, tp, c.into(), Some(tp), Some(LOCATION_REASON_CONTROL), None).0 > 0
        && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.ctrltg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn ctrltg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.selffilter,tp,LOCATION_MZONE,0,1,nil,e,tp)
    //   and Duel.IsExistingTarget(s.oppfilter,tp,0,LOCATION_MZONE,1,nil,e,1-tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| selffilter(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| oppfilter(d, c, a.e, 1 - a.tp)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local g1=Duel.GetMatchingGroup(s.selffilter,tp,LOCATION_MZONE,0,nil,e,tp)
    let mut g1 = duel::get_matching_group(d, Some(&|d, c| selffilter(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.oppfilter,tp,0,LOCATION_MZONE,nil,e,1-tp)
    let g2 = duel::get_matching_group(d, Some(&|d, c| oppfilter(d, c, a.e, 1 - a.tp)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // local tg=aux.SelectUnselectGroup(g1,e,tp,2,2,aux.dpcheck(Card.GetControler),1,tp,HINTMSG_CONTROL)
    let dp = aux::dpcheck(card_controler_value);
    let tg = aux::select_unselect_group(d, &g1, Some(a.e), a.tp, Some(2), Some(2), Some(&dp), 1, a.tp, Some(HINTMSG_CONTROL as u64), None, None, false)
        .unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,tg,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(tg), 2, a.tp, 0);
    true
}
/// `Card.GetControler` as the value of `aux.dpcheck`
fn card_controler_value(d: &mut Duel, c: CardId) -> i64 {
    card::get_controler(d, c) as i64
}
/// `s.ctrlop(e,tp,eg,ep,ev,re,r,rp)`
fn ctrlop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg==2 then
    if tg.len() == 2 {
        // Duel.SwapControl(tg:GetFirst(),tg:GetNext())
        let order = group::iter(d, &tg);
        duel::swap_control(d, order[0], order[1], 0, 0);
    }
}
