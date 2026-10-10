//! Beast Rising
use super::prelude::*;

pub const ID: u32 = 84962466;

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
    // Increase the ATK of 1 Beast or Beast-Warrior monster you control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP,TIMING_DAMAGE_STEP|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP as u32, Some((TIMING_DAMAGE_STEP | TIMING_END_PHASE) as u32));
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(function() return not (Duel.IsPhase(PHASE_DAMAGE) and Duel.IsDamageCalculated()) end)
    effect::set_condition(d, e1, Some(condition(|d, _a| !(duel::is_phase(d, PHASE_DAMAGE as u32) && duel::is_damage_calculated(d)))));
    // e1:SetCost(s.atkcost)
    effect::set_cost(d, e1, Some(cost_target(atkcost)));
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(cost_target(atktg)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.atkcostfilter(c,tp)
fn atkcostfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRace(RACE_BEAST|RACE_BEASTWARRIOR) and c:IsFaceup() and c:IsAbleToRemoveAsCost() and c:GetTextAttack()>0
    //     and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsRace,RACE_BEAST|RACE_BEASTWARRIOR),tp,LOCATION_MZONE,0,1,c)
    card::is_race(d, c, RACE_BEAST | RACE_BEASTWARRIOR)
        && card::is_faceup(d, c)
        && card::is_able_to_remove_as_cost(d, c, None)
        && card::get_text_attack(d, c) > 0
        && duel::is_existing_target(
            d,
            Some(&aux::faceup_filter(|d, x| card::is_race(d, x, RACE_BEAST | RACE_BEASTWARRIOR))),
            tp,
            LOCATION_MZONE,
            0,
            1,
            c.into(),
        )
}
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.atkcostfilter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| atkcostfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.atkcostfilter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| atkcostfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, &g, POS_FACEUP as u8, REASON_COST, None, None);
    // e:SetLabel(g:GetFirst():GetTextAttack())
    let tc = group::get_first(d, &g).nil();
    let atk = card::get_text_attack(d, tc) as i64;
    effect::set_label(d, a.e, &[atk]);
    true
}
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and chkc:IsFaceup() and chkc:IsRace(RACE_BEAST|RACE_BEASTWARRIOR) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, tp)
            && card::is_faceup(d, chkc)
            && card::is_race(d, chkc, RACE_BEAST | RACE_BEASTWARRIOR);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
    // Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsRace,RACE_BEAST|RACE_BEASTWARRIOR),tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(
        d,
        tp,
        Some(&aux::faceup_filter(|d, x| card::is_race(d, x, RACE_BEAST | RACE_BEASTWARRIOR))),
        tp,
        LOCATION_MZONE,
        0,
        1,
        1,
        false,
        NO_EX,
    );
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:IsControler(tp)
    //     and tc:IsRace(RACE_BEAST|RACE_BEASTWARRIOR) then
    if card::is_relate_to_effect(d, tc, a.e)
        && card::is_faceup(d, tc)
        && card::is_controler(d, tc, a.tp)
        && card::is_race(d, tc, RACE_BEAST | RACE_BEASTWARRIOR)
    {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(e:GetLabel())
        let lv = effect::get_label(d, a.e);
        effect::set_value(d, e1, lv);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
