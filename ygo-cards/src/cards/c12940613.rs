//! Madolche Lesson
//! マドルチェ・マナー
use super::prelude::*;

pub const ID: u32 = 12940613;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MADOLCHE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tdfilter1(c)`
fn tdfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_MADOLCHE) and c:IsAbleToDeck()
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_MADOLCHE as u16]) && card::is_able_to_deck(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.tdfilter1(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && tdfilter1(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_MADOLCHE),tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingTarget(s.tdfilter1,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MADOLCHE as u16])),
            tp,
            LOCATION_MZONE,
            0,
            1,
            NO_EX,
        ) && duel::is_existing_target(d, Some(&tdfilter1), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.tdfilter1,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&tdfilter1), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.tdfilter2(c)`
fn tdfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToDeck()
    card::is_monster(d, c) && card::is_able_to_deck(d, c)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local tc=Duel.GetFirstTarget()
    // if not (tc and tc:IsRelateToEffect(e)) or Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)==0 then return end
    let Some(tc) = duel::first_target(d) else {
        return;
    };
    if !card::is_relate_to_effect(d, tc, e) || duel::send_to_deck(d, CardSet::from_card(tc), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) == 0 {
        return;
    }
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_MADOLCHE),tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MADOLCHE as u16])), tp, LOCATION_MZONE, 0, NO_EX);
    // local ct=0
    let mut ct = 0;
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // local preatk=tc:GetAttack()
        let preatk = card::get_attack(d, tc);
        // local predef=tc:GetDefense()
        let predef = card::get_defense(d, tc);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(800)
        effect::set_value(d, e1, 800);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_UPDATE_DEFENSE)
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // if preatk~=tc:GetAttack() or predef~=tc:GetDefense() then ct=ct+1 end
        if preatk != card::get_attack(d, tc) || predef != card::get_defense(d, tc) {
            ct += 1;
        }
    }
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // local dg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.tdfilter2),tp,LOCATION_GRAVE,0,nil)
    let dg = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| tdfilter2(d, c))), tp, LOCATION_GRAVE, 0, NO_EX);
    // if #dg~=0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if !dg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        // Duel.SendtoDeck(dg:Select(tp,1,1,nil),nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        let sel = group::select(d, &dg, tp, 1, 1, false, NO_EX).nil();
        duel::send_to_deck(d, sel, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
