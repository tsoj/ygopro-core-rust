//! Makourai, the Lightning Strike
use super::prelude::*;

pub const ID: u32 = 15725501;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARCHFIEND], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(stat_change_damage_step_condition)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP as u32, None);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `aux.FaceupFilter(Card.IsSetCard,SET_ARCHFIEND)`
fn archfiend_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ARCHFIEND as u16])
}
/// `s.target(e,tp,...,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsSetCard(SET_ARCHFIEND) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp)
            && card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_set_card(d, chkc, &[SET_ARCHFIEND as u16])
            && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsSetCard,SET_ARCHFIEND),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&archfiend_faceup), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
    // Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsSetCard,SET_ARCHFIEND),tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&archfiend_faceup), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,1-tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 1 - tp, LOCATION_MZONE as i32);
    true
}
/// `s.desfilter(c,atk)`
fn desfilter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    // return c:GetBaseAttack()<atk and c:IsFaceup()
    card::get_base_attack(d, c) < atk && card::is_faceup(d, c)
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and not tc:IsImmuneToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && !card::is_immune_to_effect(d, tc, a.e) {
        // local prev_atk=tc:GetAttack()
        let prev_atk = card::get_attack(d, tc);
        // tc:UpdateAttack(600,nil,e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        card::update_attack(d, tc, 600, None, Some(h), None);
        // local atk=tc:GetAttack()
        let atk = card::get_attack(d, tc);
        // if atk>=prev_atk and Duel.IsExistingMatchingCard(s.desfilter,tp,0,LOCATION_MZONE,1,nil,atk)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if atk >= prev_atk
            && duel::is_existing_matching_card(d, Some(&|d, c| desfilter(d, c, atk)), tp, 0, LOCATION_MZONE, 1, NO_EX)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
        {
            // local g=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_MZONE,nil,atk)
            let g = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, atk)), tp, 0, LOCATION_MZONE, NO_EX);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Destroy(g,REASON_EFFECT)
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevel(6) and c:IsRace(RACE_FIEND) and c:IsAttack(2500) and c:IsAbleToHand()
    card::is_level(d, c, &[6]) && card::is_race(d, c, RACE_FIEND as u64) && card::is_attack(d, c, &[2500]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,...,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && thfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, tp, 0);
    true
}
/// `s.thop(e,tp,...)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(tc), None, REASON_EFFECT, None);
    }
}
/// `aux.StatChangeDamageStepCondition()` used as a condition
fn stat_change_damage_step_condition(d: &mut Duel, _a: &EffArgs) -> bool {
    aux::stat_change_damage_step_condition(d)
}
