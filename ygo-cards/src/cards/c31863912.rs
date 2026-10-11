//! Dice-nied
use super::prelude::*;

pub const ID: u32 = 31863912;

pub const INFO: ScriptInfo = ScriptInfo { roll_dice: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_SPSUMMON)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_SPSUMMON));
    // e1:SetTarget(s.target1)
    effect::set_target(d, e1, Some(cost_target(target1)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // roll a die on summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTarget(s.target2)
    effect::set_target(d, e2, Some(cost_target(target2)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(operation_fn)));
    // e2:SetLabel(1)
    effect::set_label(d, e2, &[1]);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c,sp)`
fn cfilter(d: &mut Duel, c: CardId, sp: u8) -> bool {
    // return c:IsFaceup() and c:GetSummonPlayer()==sp
    card::is_faceup(d, c) && card::get_summon_player(d, c) == sp
}
/// `s.target1(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target1(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local res,teg,tep,tev,tre,tr,trp=Duel.CheckEvent(EVENT_SPSUMMON_SUCCESS,true)
    let ev = duel::check_event(d, EVENT_SPSUMMON_SUCCESS);
    // if res and teg:IsExists(s.cfilter,1,nil,1-tp) then
    let teg = ev.as_ref().and_then(|ev| ev.event_cards.clone());
    if let Some(teg) = teg.filter(|g| group::is_exists(d, g, &|d, c| cfilter(d, c, 1 - tp), 1, NO_EX)) {
        // e:SetLabel(1)
        effect::set_label(d, e, &[1]);
        // Duel.SetTargetCard(teg)
        duel::set_target_card(d, teg.clone());
        // Duel.SetOperationInfo(0,CATEGORY_DICE,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, tp, 1);
    } else {
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
    }
    true
}
/// `s.target2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    // if chk==0 then return eg:IsExists(s.cfilter,1,nil,1-tp) end
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, c| cfilter(d, c, 1 - tp), 1, NO_EX);
    }
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, eg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_DICE,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, tp, 1);
    true
}
/// `s.filter(c,sp,e,lv)`
fn filter(d: &mut Duel, c: CardId, sp: u8, e: EffectId, lv: u32) -> bool {
    // return c:IsFaceup() and c:GetSummonPlayer()==sp and c:GetLevel()==lv and c:IsAbleToHand() and c:IsRelateToEffect(e)
    card::is_faceup(d, c)
        && card::get_summon_player(d, c) == sp
        && card::get_level(d, c) == lv
        && card::is_able_to_hand(d, c, None)
        && card::is_relate_to_effect(d, c, e)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if e:GetLabel()==0 or not e:GetHandler():IsRelateToEffect(e) then return end
    let label = effect::get_label(d, e);
    if label == 0 {
        return;
    }
    let c = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local dc=Duel.TossDice(tp,1)
    let dc = duel::toss_dice(d, tp, 1, 0)[0] as u32;
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS):Filter(s.filter,nil,1-tp,e,dc)
    let tg = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned()).nil();
    let g = group::filter(d, &tg, &|d, x| filter(d, x, 1 - tp, e, dc), NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
