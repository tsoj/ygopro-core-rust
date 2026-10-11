//! Alien Brain
//! エーリアン・ブレイン
use super::prelude::*;

pub const ID: u32 = 17490535;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ec=eg:GetFirst()
    let eg = a.eg.clone().nil();
    // return #eg==1 and ec:IsPreviousControler(tp) and ec:IsRace(RACE_REPTILE)
    //   and (ec:GetPreviousRaceOnField()&RACE_REPTILE)~=0
    //   and ec==Duel.GetAttackTarget() and ec:IsLocation(LOCATION_GRAVE) and ec:IsReason(REASON_BATTLE)
    if eg.len() != 1 {
        return false;
    }
    let ec = group::get_first(d, &eg).nil();
    card::is_previous_controler(d, ec, tp)
        && card::is_race(d, ec, RACE_REPTILE)
        && (card::get_previous_race_on_field(d, ec) & RACE_REPTILE) != 0
        && duel::get_attack_target(d) == Some(ec)
        && card::is_location(d, ec, LOCATION_GRAVE)
        && card::is_reason(d, ec, REASON_BATTLE)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=eg:GetFirst():GetReasonCard()
    let eg = a.eg.clone().nil();
    let tc = card::get_reason_card(d, group::get_first(d, &eg).nil());
    // if chk==0 then return tc:IsControler(1-tp) and tc:IsRelateToBattle() and tc:IsControlerCanBeChanged() end
    if a.chk == Some(0) {
        let tc = tc.nil();
        return card::is_controler(d, tc, 1 - a.tp)
            && card::is_relate_to_battle(d, tc)
            && card::is_controler_can_be_changed(d, tc, false, None);
    }
    let tc = tc.nil();
    // Duel.SetTargetCard(tc)
    duel::set_target_card(d, CardSet::from_card(tc));
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(CardSet::from_card(tc)), 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, e) {
        // if Duel.GetControl(tc,tp)~=0 then
        if duel::get_control(d, CardSet::from_card(tc), tp, 0, 0, None, None) != 0 {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CHANGE_RACE)
            effect::set_code(d, e1, EFFECT_CHANGE_RACE);
            // e1:SetValue(RACE_REPTILE)
            effect::set_value(d, e1, RACE_REPTILE as i64);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
