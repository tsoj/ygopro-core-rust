//! The Weather Painter Sun
use super::prelude::*;

pub const ID: u32 = 54895237;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_THE_WEATHER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon (grave)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.gspcost)
    effect::set_cost(d, e1, Some(cost_target(gspcost)));
    // e1:SetTarget(s.gsptg)
    effect::set_target(d, e1, Some(cost_target(gsptg)));
    // e1:SetOperation(s.gspop)
    effect::set_operation(d, e1, Some(operation(gspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // spsummon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_REMOVE)
    effect::set_code(d, e2, EVENT_REMOVE);
    // e2:SetOperation(s.spreg)
    effect::set_operation(d, e2, Some(operation(spreg)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetRange(LOCATION_REMOVED)
    effect::set_range(d, e3, LOCATION_REMOVED);
    // e3:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e3, EVENT_PHASE | PHASE_STANDBY);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.gspcfilter(c,ft,tp,sft)
fn gspcfilter(d: &mut Duel, c: CardId, ft: i32, tp: u8, sft: i32) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_CONTINUOUS) and c:IsAbleToGraveAsCost()
    //     and (ft>0 or (c:IsLocation(LOCATION_MZONE) and c:GetSequence()<5))
    //     and (sft>0 or (c:IsLocation(LOCATION_SZONE) and c:GetSequence()<5))
    //     and Duel.IsExistingMatchingCard(s.gspfilter,tp,LOCATION_HAND,0,1,nil,c,tp)
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_CONTINUOUS)
        && card::is_able_to_grave_as_cost(d, c)
        && (ft > 0 || (card::is_location(d, c, LOCATION_MZONE) && card::get_sequence(d, c) < 5))
        && (sft > 0 || (card::is_location(d, c, LOCATION_SZONE) && card::get_sequence(d, c) < 5))
        && duel::is_existing_matching_card(d, Some(&|d, x| gspfilter(d, x, Some(c), tp)), tp, LOCATION_HAND, 0, 1, NO_EX)
}
// function s.gspfilter(c,cc,tp)
fn gspfilter(d: &mut Duel, c: CardId, cc: Option<CardId>, tp: u8) -> bool {
    // return c:IsSpellTrap() and c:IsSetCard(SET_THE_WEATHER) and not c:IsForbidden()
    //     and c:CheckUniqueOnField(tp,LOCATION_ONFIELD,cc) and not c:IsType(TYPE_FIELD)
    card::is_spell_trap(d, c)
        && card::is_set_card(d, c, &[SET_THE_WEATHER as u16])
        && !card::is_forbidden(d, c)
        && card::check_unique_on_field(d, c, tp, Some(LOCATION_ONFIELD), cc)
        && !card::is_type(d, c, TYPE_FIELD)
}
// function s.gspcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn gspcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // local sft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let sft = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0;
    // if chk==0 then return ft>-1 and sft>-1 and Duel.IsExistingMatchingCard(s.gspcfilter,tp,LOCATION_ONFIELD,0,1,nil,ft,tp,sft) end
    if a.chk == Some(0) {
        return ft > -1
            && sft > -1
            && duel::is_existing_matching_card(d, Some(&|d, c| gspcfilter(d, c, ft, a.tp, sft)), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.gspcfilter,tp,LOCATION_ONFIELD,0,1,1,nil,ft,tp,sft)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| gspcfilter(d, c, ft, a.tp, sft)), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
// function s.gsptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn gsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
// function s.gspop(e,tp,eg,ep,ev,re,r,rp)
fn gspop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP_DEFENSE)>0
    //     and Duel.GetLocationCount(tp,LOCATION_SZONE)>0 then
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None) > 0
        && duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
        // local tc=Duel.SelectMatchingCard(tp,s.gspfilter,tp,LOCATION_HAND,0,1,1,nil,nil,tp):GetFirst()
        let g = duel::select_matching_card(d, a.tp, Some(&|d, x| gspfilter(d, x, None, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // if tc then
        if let Some(tc) = group::get_first(d, &g) {
            // Duel.MoveToField(tc,tp,tp,LOCATION_SZONE,POS_FACEUP,true)
            duel::move_to_field(d, tc, a.tp, a.tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
        }
    }
}
// function s.spreg(e,tp,eg,ep,ev,re,r,rp)
fn spreg(d: &mut Duel, a: &EffArgs) {
    // if not re then return end
    let Some(re) = a.re else { return };
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, re).nil();
    // if c:IsReason(REASON_COST) and rc:IsSetCard(SET_THE_WEATHER) then
    if card::is_reason(d, c, REASON_COST) && card::is_set_card(d, rc, &[SET_THE_WEATHER as u16]) {
        // e:SetLabel(Duel.GetTurnCount()+1)
        let turn = duel::get_turn_count(d, None) as i64 + 1;
        effect::set_label(d, a.e, &[turn]);
        // c:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,2)
        card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 2, 0, 0);
    }
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetLabelObject():GetLabel()==Duel.GetTurnCount() and e:GetHandler():GetFlagEffect(id)>0
    let label_obj = effect::get_label_object(d, a.e).as_effect().nil();
    let label = effect::get_label(d, label_obj);
    let c = effect::get_handler(d, a.e).nil();
    label == duel::get_turn_count(d, None) as i64 && card::get_flag_effect(d, c, ID) > 0
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    // e:GetHandler():ResetFlagEffect(id)
    card::reset_flag_effect(d, c, ID);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
