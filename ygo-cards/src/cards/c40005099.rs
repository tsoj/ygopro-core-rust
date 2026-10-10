//! Shiranui Style Synthesis
use super::prelude::*;

pub const ID: u32 = 40005099;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_SHIRANUI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate 1 of these effects
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(e,tp) return Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0 end)
    effect::set_condition(d, e2, Some(condition(e2_condition)));
    // e2:SetCost(s.effcost)
    effect::set_cost(d, e2, Some(cost_target(effcost)));
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp) return Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0 end`
fn e2_condition(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) == 0
}
/// `s.effcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToGraveAsCost,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,Card.IsAbleToGraveAsCost,1,1,REASON_COST)
    duel::discard_hand(d, a.tp, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), 1, 1, REASON_COST, NO_EX);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_ZOMBIE) and c:IsDefense(0) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_ZOMBIE) && card::is_defense(d, c, &[0]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_ZOMBIE) and c:IsDefense(0) and c:IsFaceup()
    card::is_race(d, c, RACE_ZOMBIE) && card::is_defense(d, c, &[0]) && card::is_faceup(d, c)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then
    if let Some(chkc) = a.chkc() {
        // if not chkc:IsControler(tp) then return false end
        if !card::is_controler(d, chkc, a.tp) {
            return false;
        }
        // local op=e:GetLabel()
        let op = effect::get_label(d, a.e);
        // if op==1 then
        if op == 1 {
            // return chkc:IsLocation(LOCATION_GRAVE) and s.spfilter(chkc,e,tp)
            return card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, a.e, a.tp);
        } else if op == 2 {
            // return chkc:IsLocation(LOCATION_REMOVED) and s.tgfilter(chkc)
            return card::is_location(d, chkc, LOCATION_REMOVED) && tgfilter(d, chkc);
        }
    }
    // local b1=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp)
    let b1 = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    // local b2=Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_REMOVED,0,1,nil)
    let b2 = duel::is_existing_target(d, Some(&tgfilter), a.tp, LOCATION_REMOVED, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).nil();
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
        let g = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, a.tp, 0);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_TOGRAVE)
        effect::set_category(d, a.e, CATEGORY_TOGRAVE);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_REMOVED,0,1,1,nil)
        let g = duel::select_target(d, a.tp, Some(&tgfilter), a.tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, g, 1, a.tp, 0);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    } else if op == 2 {
        // Duel.SendtoGrave(tc,REASON_EFFECT|REASON_RETURN)
        duel::send_to_grave(d, tc, REASON_EFFECT | REASON_RETURN, None, None);
    }
}
