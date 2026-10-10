//! Hyper Blaze
use super::prelude::*;

pub const ID: u32 = 16317140;

// s.listed_names={6007213,32491822,69890967}
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[6007213, 32491822, 69890967], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Procedure enhancement
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(16317140)
    effect::set_code(d, e2, 16317140);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --Change ATK/DEF
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCondition(s.atkcon)
    effect::set_condition(d, e3, Some(condition(atkcon)));
    // e3:SetCost(s.atkcost)
    effect::set_cost(d, e3, Some(cost_target(atkcost)));
    // e3:SetTarget(s.atktg)
    effect::set_target(d, e3, Some(cost_target(atktg)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // --Add to hand or Special Summon 1 Sacred Beast from your GY
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e4, EVENT_FREE_CHAIN);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e4, 0, Some(TIMING_END_PHASE));
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCost(s.spcost)
    effect::set_cost(d, e4, Some(cost_target(spcost)));
    // e4:SetTarget(s.sptg)
    effect::set_target(d, e4, Some(cost_target(sptg)));
    // e4:SetOperation(s.spop)
    effect::set_operation(d, e4, Some(operation(spop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.atkcon(e,tp,eg,ep,ev,re,r,rp)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=Duel.GetBattleMonster(tp)
    let (bc, _) = duel::get_battle_monster(d, a.tp);
    // if not (bc and bc:IsCode(6007213) and bc:IsFaceup()) then return false end
    let Some(bc) = bc else {
        return false;
    };
    if !(card::is_code(d, bc, &[6007213]) && card::is_faceup(d, bc)) {
        return false;
    }
    // e:SetLabelObject(bc)
    effect::set_label_object(d, a.e, Value::Card(bc));
    // return true
    true
}
// function s.atkcfilter(c)
fn atkcfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsTrap() and c:IsAbleToGraveAsCost()
    card::is_trap(d, c) && card::is_able_to_grave_as_cost(d, c)
}
// function s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.atkcfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&atkcfilter), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.atkcfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&atkcfilter), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, &g, REASON_COST, None, None);
    true
}
// function s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetFlagEffect(id)==0 end
    if a.chk == Some(0) {
        return card::get_flag_effect(d, c, ID) == 0;
    }
    // Duel.SetTargetCard(e:GetLabelObject())
    let lo = effect::get_label_object(d, a.e).as_card().nil();
    duel::set_target_card(d, lo);
    // e:SetLabelObject(nil)
    effect::set_label_object(d, a.e, Value::Nil);
    // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE_CAL,0,1)
    card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE_CAL, 0, 1, 0, 0);
    true
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) and tc:IsRelateToBattle() then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && card::is_relate_to_battle(d, tc) {
        // local atk=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsTrap),tp,LOCATION_GRAVE|LOCATION_ONFIELD,LOCATION_GRAVE|LOCATION_ONFIELD,nil)*1000
        let atk = duel::get_matching_group_count(d, Some(&|d, c| card::is_faceup(d, c) && card::is_trap(d, c)), a.tp, LOCATION_GRAVE | LOCATION_ONFIELD, LOCATION_GRAVE | LOCATION_ONFIELD, NO_EX) as i64 * 1000;
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(atk)
        effect::set_value(d, e1, atk);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_SET_DEFENSE_FINAL)
        effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
// function s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), a.tp, LOCATION_HAND, 0, 1, h.into());
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, a.tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
// function s.filter(c,e,tp,ft)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // return c:IsCode(6007213,32491822,69890967) and (c:IsAbleToHand() or (ft>0 and c:IsCanBeSpecialSummoned(e,0,tp,true,false)))
    card::is_code(d, c, &[6007213, 32491822, 69890967])
        && (card::is_able_to_hand(d, c, None) || (ft > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)))
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_GRAVE,0,1,nil,e,tp,ft) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp, ft)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
    // local hc=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,ft):GetFirst()
    let hc = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp, ft)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let Some(hc) = group::get_first(d, &hc) else {
        return;
    };
    // aux.ToHandOrElse(hc,tp,
    //   function() return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and hc:IsCanBeSpecialSummoned(e,0,tp,true,false) end,
    //   function() Duel.SpecialSummon(hc,0,tp,tp,true,false,POS_FACEUP) end,
    //   aux.Stringid(id,3))
    aux::to_hand_or_else(
        d,
        hc,
        a.tp,
        Some(&|d, _c| {
            duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && card::is_can_be_special_summoned(d, hc, a.e, 0, a.tp, true, false, None, None, None)
        }),
        Some(&|d, _g| {
            duel::special_summon(d, hc, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
            0
        }),
        Some(aux::stringid(ID, 3)),
    );
}
