//! Madolche Promenade
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 68159562;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MADOLCHE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_DISABLE + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some((TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Attach 1 "Madolche" monster from your hand, Deck, or GY to 1 "Madolche" Xyz Monster you control
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some((TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32));
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.attachtg)
    effect::set_target(d, e2, Some(cost_target(attachtg)));
    // e2:SetOperation(s.attachop)
    effect::set_operation(d, e2, Some(operation(attachop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MADOLCHE) and c:IsMonster() and c:IsFaceup() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_MADOLCHE as u16]) && card::is_monster(d, c) && card::is_faceup(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsNegatable,tp,0,LOCATION_ONFIELD,1,nil)
    //     and Duel.IsExistingTarget(s.thfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&card::is_negatable), tp, 0, LOCATION_ONFIELD, 1, NO_EX)
            && duel::is_existing_target(d, Some(&thfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
    // local dis_tc=Duel.SelectTarget(tp,Card.IsNegatable,tp,0,LOCATION_ONFIELD,1,1,nil):GetFirst()
    let sel = duel::select_target(d, tp, Some(&card::is_negatable), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    let dis_tc = group::get_first(d, &sel).nil();
    // e:SetLabelObject(dis_tc)
    effect::set_label_object(d, a.e, Value::Card(dis_tc));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&thfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,dis_tc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(CardSet::from_card(dis_tc)), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, tp, 0);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local dis_tc=e:GetLabelObject()
    let dis_tc = effect::get_label_object(d, e).as_card().nil();
    // if dis_tc:IsRelateToEffect(e) and dis_tc:IsFaceup() and dis_tc:IsControler(1-tp)
    //     and dis_tc:IsCanBeDisabledByEffect(e) then
    if card::is_relate_to_effect(d, dis_tc, e)
        && card::is_faceup(d, dis_tc)
        && card::is_controler(d, dis_tc, 1 - tp)
        && card::is_can_be_disabled_by_effect(d, dis_tc, e, true)
    {
        // Negate its effects until the end of this turn
        // dis_tc:NegateEffects(e:GetHandler(),RESET_PHASE|PHASE_END,true)
        let h = effect::get_handler(d, e).nil();
        card::negate_effects(d, dis_tc, h, Some(RESET_PHASE + PHASE_END), true, None);
        // Duel.AdjustInstantly(dis_tc)
        duel::adjust_instantly(d, Some(dis_tc));
        // local ret_tc=(Duel.GetTargetCards(e)-dis_tc):GetFirst()
        let mut rest = duel::get_target_cards(d, e).nil();
        group::remove_card(&mut rest, &CardSet::from_card(dis_tc));
        // if ret_tc and ret_tc:IsControler(tp) then
        if let Some(ret_tc) = group::get_first(d, &rest) {
            if card::is_controler(d, ret_tc, tp) {
                // Duel.SendtoHand(ret_tc,nil,REASON_EFFECT)
                duel::send_to_hand(d, CardSet::from_card(ret_tc), None, REASON_EFFECT, None);
            }
        }
    }
}
// function s.xyzfilter(c)
fn xyzfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MADOLCHE) and c:IsType(TYPE_XYZ) and c:IsFaceup()
    card::is_set_card(d, c, &[SET_MADOLCHE as u16]) && card::is_type(d, c, TYPE_XYZ) && card::is_faceup(d, c)
}
// function s.attachfilter(c)
fn attachfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MADOLCHE) and c:IsMonster() and not c:IsForbidden()
    card::is_set_card(d, c, &[SET_MADOLCHE as u16]) && card::is_monster(d, c) && !card::is_forbidden(d, c)
}
// function s.attachtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.xyzfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && xyzfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.xyzfilter,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.attachfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&xyzfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&attachfilter), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.xyzfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let _ = duel::select_target(d, tp, Some(&xyzfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
// function s.attachop(e,tp,eg,ep,ev,re,r,rp)
fn attachop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and not tc:IsImmuneToEffect(e)) then return end
    if !(card::is_relate_to_effect(d, tc, e) && !card::is_immune_to_effect(d, tc, e)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTACH)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATTACH as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.attachfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil)
    let nvf = aux::necro_valley_filter(|d, c| attachfilter(d, c));
    let g = duel::select_matching_card(d, tp, Some(&nvf), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Overlay(tc,g)
        duel::overlay(d, tc, g, false);
    }
}
