//! Throne of the Archfiends
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 63679166;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[48469380], listed_series: &[SET_ARCHFIEND], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual.AddProcGreater({handler=c,filter=s.ritualmonsterfilter,location=LOCATION_HAND|LOCATION_GRAVE|LOCATION_EXTRA|LOCATION_REMOVED})
    let _e0 = ritual::add_proc_greater(
        d,
        c,
        ritual::RitualProc {
            lvtype: RITPROC_GREATER,
            filter: Some(Arc::new(|d: &mut Duel, x: CardId| ritualmonsterfilter(d, x))),
            location: Some(LOCATION_HAND | LOCATION_GRAVE | LOCATION_EXTRA | LOCATION_REMOVED),
            ..Default::default()
        },
    );
    // Add this card from your GY or banishment to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_PHASE+PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_STANDBY);
    // e1:SetRange(LOCATION_GRAVE|LOCATION_REMOVED)
    effect::set_range(d, e1, LOCATION_GRAVE | LOCATION_REMOVED);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,48469380),tp,LOCATION_EXTRA,0,1,nil) end)
    effect::set_condition(d, e1, Some(condition(thcond)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function(e,tp) of e1: Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,48469380),tp,LOCATION_EXTRA,0,1,nil)
fn thcond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[48469380]))), a.tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
// function s.ritualmonsterfilter(c)
fn ritualmonsterfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ARCHFIEND) and c:IsRitualMonster() and (c:IsFaceup() or c:IsLocation(LOCATION_HAND))
    card::is_set_card(d, c, &[SET_ARCHFIEND as u16]) && card::is_ritual_monster(d, c) && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND))
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp,chk)
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
    }
}
