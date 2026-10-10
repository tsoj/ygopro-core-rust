//! Magi Magi ☆ Magician Gal
//! マジマジ☆マジシャンギャル
use super::prelude::*;

pub const ID: u32 = 10000030;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 6 Spellcaster monsters
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_SPELLCASTER),6,2)
    xyz::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_SPELLCASTER, sc, st, Some(tp)))), Some(6), 2, None, None, None, None, false, None);
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.AND(Cost.DetachFromSelf(1),s.effcost))
    effect::set_cost(d, e1, Some(cost::and(vec![cost::detach_from_self(cost::Count::N(1), None, None), cost_target(effcost)])));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.effcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToRemoveAsCost,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove_as_cost(d, c, None)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToRemoveAsCost,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_remove_as_cost(d, c, None)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then
    if let Some(chkc) = a.chkc() {
        // if not chkc:IsControler(1-tp) then return false end
        if !card::is_controler(d, chkc, 1 - tp) {
            return false;
        }
        // local op=e:GetLabel()
        let op = effect::get_label(d, a.e);
        // if op==1 then
        if op == 1 {
            // return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControlerCanBeChanged()
            return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler_can_be_changed(d, chkc, false, None);
        // elseif op==2 then
        } else if op == 2 {
            // return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsCanBeSpecialSummoned(e,0,tp,false,false)
            return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_can_be_special_summoned(d, chkc, a.e, 0, tp, false, false, None, None, None);
        }
    }
    // local b1=Duel.IsExistingTarget(Card.IsControlerCanBeChanged,tp,0,LOCATION_MZONE,1,nil)
    let b1 = duel::is_existing_target(d, Some(&|d, c| card::is_controler_can_be_changed(d, c, false, None)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    // local b2=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(Card.IsCanBeSpecialSummoned,tp,0,LOCATION_GRAVE,1,nil,e,0,tp,false,false)
    let b2 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_target(d, Some(&|d, c| card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None)), tp, 0, LOCATION_GRAVE, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op.map_or(0, |v| v as i64)]);
    // if op==1 then
    if op == Some(1) {
        // e:SetCategory(CATEGORY_CONTROL)
        effect::set_category(d, a.e, CATEGORY_CONTROL);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
        // local g=Duel.SelectTarget(tp,Card.IsControlerCanBeChanged,tp,0,LOCATION_MZONE,1,1,nil)
        let g = duel::select_target(d, tp, Some(&|d, c| card::is_controler_can_be_changed(d, c, false, None)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, tp, 0);
    // elseif op==2 then
    } else if op == Some(2) {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectTarget(tp,Card.IsCanBeSpecialSummoned,tp,0,LOCATION_GRAVE,1,1,nil,e,0,tp,false,false)
        let g = duel::select_target(d, tp, Some(&|d, c| card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None)), tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, 0);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    // if not tc:IsRelateToEffect(e) then return end
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // Duel.GetControl(tc,tp,PHASE_END,1)
        duel::get_control(d, tc, tp, PHASE_END as u16, 1, None, None);
    // elseif op==2 then
    } else if op == 2 {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
