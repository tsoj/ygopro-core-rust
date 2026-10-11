//! Vanquish Soul Pluton HG
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 55688914;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VANQUISH_SOUL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetCost(s.opccost)
    effect::set_cost(d, e1, Some(cost_target(opccost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Gain 3000 DEF, OR Gain 3000 ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(TIMING_DAMAGE_STEP,TIMING_MAIN_END|TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e2, TIMING_DAMAGE_STEP, Some(TIMING_MAIN_END | TIMING_DAMAGE_STEP));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e2, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // e2:SetCost(s.opccost)
    effect::set_cost(d, e2, Some(cost_target(opccost)));
    // e2:SetTarget(s.vstg)
    effect::set_target(d, e2, Some(cost_target(vstg)));
    // e2:SetOperation(s.vsop)
    effect::set_operation(d, e2, Some(operation(vsop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.opccost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn opccost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFlagEffect(tp,id)==0 end
    if a.chk == Some(0) {
        return duel::get_flag_effect(d, a.tp, ID) == 0;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_CHAIN,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_CHAIN, 0, 1, 0);
    true
}
/// `s.spconfilter(c,tp)`
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() or not c:IsSetCard(SET_VANQUISH_SOUL)
    card::is_facedown(d, c) || !card::is_set_card(d, c, &[SET_VANQUISH_SOUL as u16])
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return Duel.IsTurnPlayer(1-tp) and not Duel.IsExistingMatchingCard(s.spconfilter,tp,LOCATION_MMZONE,0,1,nil)
    duel::is_turn_player(d, 1 - tp) && !duel::is_existing_matching_card(d, Some(&spconfilter), tp, LOCATION_MMZONE, 0, 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.vscostfilter(c,att)`
fn vscostfilter(d: &mut Duel, c: CardId, att: u32) -> bool {
    // return c:IsAttribute(att) and not c:IsPublic()
    card::is_attribute(d, c, att) && !card::is_public(d, c)
}
/// `s.vsrescon(sg)`
fn vsrescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsAttribute,nil,ATTRIBUTE_DARK)>0 and sg:FilterCount(Card.IsAttribute,nil,ATTRIBUTE_EARTH)>0
    let res = group::filter_count(d, sg, &|d, c| card::is_attribute(d, c, ATTRIBUTE_DARK), NO_EX) > 0
        && group::filter_count(d, sg, &|d, c| card::is_attribute(d, c, ATTRIBUTE_EARTH), NO_EX) > 0;
    (res, false)
}
/// `s.vstg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn vstg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local cg1=Duel.GetMatchingGroup(s.vscostfilter,tp,LOCATION_HAND,0,nil,ATTRIBUTE_FIRE)
    let cg1 = duel::get_matching_group(d, Some(&|d, c| vscostfilter(d, c, ATTRIBUTE_FIRE)), tp, LOCATION_HAND, 0, NO_EX);
    // local b1=#cg1>0
    let b1 = cg1.len() > 0;
    // local cg2=Duel.GetMatchingGroup(s.vscostfilter,tp,LOCATION_HAND,0,nil,ATTRIBUTE_DARK|ATTRIBUTE_EARTH)
    let cg2 = duel::get_matching_group(d, Some(&|d, c| vscostfilter(d, c, ATTRIBUTE_DARK | ATTRIBUTE_EARTH)), tp, LOCATION_HAND, 0, NO_EX);
    // local b2=aux.SelectUnselectGroup(cg2,e,tp,1,2,s.vsrescon,0)
    let rs: aux::Rescon = &vsrescon;
    let b2 = matches!(aux::select_unselect_group(d, &cg2, Some(a.e), tp, Some(1), Some(2), Some(rs), 0, tp, None, None, None, false), Err(true));
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,2)}, {b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op.map_or(0, |v| v as i64)]);
    // if op==1 then
    if op == Some(1) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
        // local g=cg1:Select(tp,1,1,nil)
        let g = group::select(d, &cg1, tp, 1, 1, false, NO_EX).nil();
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // e:SetCategory(CATEGORY_DEFCHANGE)
        effect::set_category(d, a.e, CATEGORY_DEFCHANGE);
    // elseif op==2 then
    } else if op == Some(2) {
        // local g=aux.SelectUnselectGroup(cg2,e,tp,1,2,s.vsrescon,1,tp,HINTMSG_CONFIRM,s.vsrescon)
        let g = aux::select_unselect_group(
            d,
            &cg2,
            Some(a.e),
            tp,
            Some(1),
            Some(2),
            Some(rs),
            1,
            tp,
            Some(HINTMSG_CONFIRM as u64),
            Some(rs),
            None,
            false,
        )
        .unwrap_or_default();
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // e:SetCategory(CATEGORY_ATKCHANGE)
        effect::set_category(d, a.e, CATEGORY_ATKCHANGE);
    }
    true
}
/// `s.vsop(e,tp,eg,ep,ev,re,r,rp)`
fn vsop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // c:UpdateDefense(3000,RESETS_STANDARD_DISABLE_PHASE_END)
        card::update_defense(d, c, 3000, Some(RESETS_STANDARD_DISABLE_PHASE_END), None, None);
    // elseif op==2 then
    } else if op == 2 {
        // c:UpdateAttack(3000,RESETS_STANDARD_DISABLE_PHASE_END)
        card::update_attack(d, c, 3000, Some(RESETS_STANDARD_DISABLE_PHASE_END), None, None);
    }
}
