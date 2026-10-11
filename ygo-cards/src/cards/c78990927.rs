//! Megalith Phul
use super::prelude::*;

pub const ID: u32 = 78990927;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MEGALITH], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Change level and add target to the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.thcon)
    effect::set_condition(d, e1, Some(condition(thcon)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Ritual Summon
    // local e2=Ritual.CreateProc(c,RITPROC_GREATER,aux.FilterBoolFunction(Card.IsSetCard,SET_MEGALITH),nil,aux.Stringid(id,1),nil,nil,nil,nil,LOCATION_HAND|LOCATION_DECK)
    let e2 = ritual::create_proc(
        d,
        c,
        ritual::RitualProc {
            lvtype: RITPROC_GREATER,
            filter: Some(std::sync::Arc::new(|d: &mut Duel, x: CardId| card::is_set_card(d, x, &[SET_MEGALITH as u16]))),
            desc: Some(aux::stringid(ID, 1)),
            location: Some(LOCATION_HAND | LOCATION_DECK),
            ..Default::default()
        },
    );
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(0,TIMING_MAIN_END)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END as u32));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.ritcon)
    effect::set_condition(d, e2, Some(condition(ritcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.ritcon()
fn ritcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsMainPhase()
    duel::is_main_phase(d, None)
}
// function s.thcon(e,tp,eg,ep,ev,re,r,rp)
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsRitualSummoned()
    let c = effect::get_handler(d, a.e).nil();
    card::is_ritual_summoned(d, c)
}
// function s.tgfilter(c,lv)
fn tgfilter(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return c:IsRitualMonster() and c:IsAbleToHand() and c:GetLevel()~=lv
    card::is_ritual_monster(d, c) && card::is_able_to_hand(d, c, None) && card::get_level(d, c) != lv
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local lv=e:GetHandler():GetLevel()
    let c = effect::get_handler(d, a.e).nil();
    let lv = card::get_level(d, c);
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.tgfilter(chkc,lv) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && tgfilter(d, chkc, lv);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_GRAVE,0,1,nil,lv) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| tgfilter(d, x, lv)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_GRAVE,0,1,1,nil,lv)
    let g = duel::select_target(d, tp, Some(&|d, x| tgfilter(d, x, lv)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if tc and tc:IsRelateToEffect(e) and c:IsFaceup() and c:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CHANGE_LEVEL)
            effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
            // e1:SetValue(tc:GetLevel())
            let lv = card::get_level(d, tc) as i64;
            effect::set_value(d, e1, lv);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(tc,nil,REASON_EFFECT)
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,tc)
            duel::confirm_cards(d, 1 - tp, tc);
        }
    }
}
