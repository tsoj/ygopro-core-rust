//! Wolf in Sheep's Clothing
//! 無抵抗の真相
use super::prelude::*;

pub const ID: u32 = 83546647;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetLabel(1)
    effect::set_label(d, e1, &[1]);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:GetFirst():IsControler(1-tp) and ep==tp and Duel.GetAttackTarget()==nil
    let eg = a.eg.clone().nil();
    let first = group::get_first(d, &eg).nil();
    card::is_controler(d, first, 1 - a.tp) && a.ep == a.tp && duel::get_attack_target(d).is_none()
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:GetLevel()==1 and not c:IsPublic() and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingMatchingCard(s.spfilter2,tp,LOCATION_DECK,0,1,nil,e,tp,c:GetCode())
    if card::get_level(d, c) != 1 || card::is_public(d, c) {
        return false;
    }
    if !card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None) {
        return false;
    }
    let code = card::get_code(d, c)[0];
    duel::is_existing_matching_card(d, Some(&|d, cc| spfilter2(d, cc, e, tp, code)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.spfilter2(c,e,tp,code)`
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    // return c:IsCode(code) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[code]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    // return true
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // if e:GetLabel()~=0 then return false end
        if effect::get_label(d, a.e) != 0 {
            return false;
        }
        // e:SetLabel(1)
        effect::set_label(d, a.e, &[1]);
        // return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
        //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>1
        //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp)
        return duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g).nil();
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - a.tp, g.clone());
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_PUBLIC)
    effect::set_code(d, e1, EFFECT_PUBLIC);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_CHAIN)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // local e2=Effect.CreateEffect(e:GetHandler())
    let e2 = effect::create_effect(d, h);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetOperation(s.clearop)
    effect::set_operation(d, e2, Some(operation(clearop)));
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_CHAIN)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 1);
    // e2:SetLabel(Duel.GetCurrentChain())
    let chain = duel::get_current_chain(d, false) as i64;
    effect::set_label(d, e2, &[chain]);
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // tc:RegisterEffect(e2)
    card::register_effect(d, tc, e2, false);
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<2 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 2 {
        return;
    }
    // local tc1=Duel.GetFirstTarget()
    let tc1 = duel::first_target(d).nil();
    // if not tc1:IsRelateToEffect(e) or not tc1:IsCanBeSpecialSummoned(e,0,tp,false,false) then return end
    if !card::is_relate_to_effect(d, tc1, a.e) {
        return;
    }
    if !card::is_can_be_special_summoned(d, tc1, a.e, 0, a.tp, false, false, None, None, None) {
        return;
    }
    // local tc2=Duel.GetFirstMatchingCard(s.spfilter2,tp,LOCATION_DECK,0,nil,e,tp,tc1:GetCode())
    let code = card::get_code(d, tc1)[0];
    let tc2 = duel::get_first_matching_card(d, Some(&|d, c| spfilter2(d, c, a.e, a.tp, code)), a.tp, LOCATION_DECK, 0, NO_EX);
    // if tc2 then
    if let Some(tc2) = tc2 {
        // Duel.SpecialSummonStep(tc1,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, tc1, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        // Duel.SpecialSummonStep(tc2,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, tc2, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
/// `s.clearop(e,tp,eg,ep,ev,re,r,rp)`
fn clearop(d: &mut Duel, a: &EffArgs) {
    // if ev~=e:GetLabel() then return end
    if a.ev as i64 != effect::get_label(d, a.e) {
        return;
    }
    // e:GetLabelObject():Reset()
    let e1 = effect::get_label_object(d, a.e).as_effect().nil();
    effect::reset(d, e1);
    // e:Reset()
    effect::reset(d, a.e);
}
