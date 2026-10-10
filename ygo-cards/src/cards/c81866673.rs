//! Destiny HERO - Dasher
use super::prelude::*;

pub const ID: u32 = 81866673;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Increase ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(s.atkcost)
    effect::set_cost(d, e1, Some(cost_target(atkcost)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change it to Defense Position
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_PHASE|PHASE_BATTLE)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_BATTLE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.poscon)
    effect::set_condition(d, e2, Some(condition(poscon)));
    // e2:SetOperation(s.posop)
    effect::set_operation(d, e2, Some(operation(posop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_NO_TURN_RESET)
    effect::set_property(d, e3, EFFECT_FLAG_NO_TURN_RESET, 0);
    // e3:SetCode(EVENT_DRAW)
    effect::set_code(d, e3, EVENT_DRAW);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,nil,1,false,nil,c) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, None, 1, 1, false, None, c.into());
    }
    // local g=Duel.SelectReleaseGroupCost(tp,nil,1,1,false,nil,c)
    let g = duel::select_release_group_cost(d, a.tp, None, 1, 1, false, None, c.into());
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFacedown() or not c:IsRelateToEffect(e) then return end
    if card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // --Increase ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(1000)
    effect::set_value(d, e1, 1000);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.poscon(e,tp,eg,ep,ev,re,r,rp)`
fn poscon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetAttackedCount()>0
    let h = effect::get_handler(d, a.e).nil();
    card::get_attacked_count(d, h) > 0
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsAttackPos() then
    if card::is_attack_pos(d, c) {
        // Duel.ChangePosition(c,POS_FACEUP_DEFENSE)
        duel::change_position(d, c, POS_FACEUP_DEFENSE as u8, None, None, None, false);
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==tp and Duel.IsPhase(PHASE_DRAW) and Duel.IsTurnPlayer(tp)
    a.ep == a.tp && duel::is_phase(d, PHASE_DRAW) && duel::is_turn_player(d, a.tp)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLocation(LOCATION_HAND) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_location(d, c, LOCATION_HAND) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and eg:IsExists(s.spfilter,1,nil,e,tp) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && group::is_exists(d, &eg, &|d, c| spfilter(d, c, a.e, a.tp), 1, NO_EX);
    }
    let eg = a.eg.clone().nil();
    // if #eg==1 then
    if eg.len() == 1 {
        // Duel.ConfirmCards(1-tp,eg)
        duel::confirm_cards(d, 1 - a.tp, eg.clone());
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, a.tp);
        // Duel.SetTargetCard(eg)
        duel::set_target_card(d, eg.clone());
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(eg), 1, 0, 0);
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g=eg:FilterSelect(tp,s.spfilter,1,1,nil,e,tp)
        let g = group::filter_select(d, &eg, a.tp, &|d, c| spfilter(d, c, a.e, a.tp), 1, 1, false, NO_EX).nil();
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g.clone());
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, a.tp);
        // Duel.SetTargetCard(g)
        duel::set_target_card(d, g.clone());
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, 0);
    }
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
