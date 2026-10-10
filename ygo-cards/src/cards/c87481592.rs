//! Dogmatikacism
use super::prelude::*;

pub const ID: u32 = 87481592;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DOGMATIKA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // Change ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(s.value)
    effect::set_value_fn(d, e1, effect_card_value(value));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Send from Extra Deck to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.tgcond)
    effect::set_condition(d, e2, Some(condition(tgcond)));
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.value(e,c)
fn value(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, e).nil();
    let ec = card::get_equip_target(d, h).nil();
    // if ec:IsSetCard(SET_DOGMATIKA) then
    if card::is_set_card(d, ec, &[SET_DOGMATIKA as u16]) {
        // if c:HasLevel() then return 100*ec:GetLevel() end
        if card::has_level(d, c) {
            return 100 * card::get_level(d, ec) as i64;
        }
        // (falls off the end: nil, which the core reads as 0)
        0
    } else {
        // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,SET_DOGMATIKA),e:GetHandler():GetControler(),LOCATION_MZONE,0,nil)
        let p = card::get_controler(d, h);
        let ct = duel::get_matching_group_count(d, Some(&aux::faceup_filter(|d, x| card::is_set_card(d, x, &[SET_DOGMATIKA as u16]))), p, LOCATION_MZONE, 0, NO_EX);
        // return -200*ct
        -200 * ct as i64
    }
}
// function s.tgcond(e,tp,eg,ep,ev,re,r,rp)
fn tgcond(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ec=c:GetPreviousEquipTarget()
    let ec = card::get_previous_equip_target(d, c);
    // return c:IsReason(REASON_LOST_TARGET) and ec and ec:IsReason(REASON_DESTROY)
    card::is_reason(d, c, REASON_LOST_TARGET) && ec.map_or(false, |ec| card::is_reason(d, ec, REASON_DESTROY))
}
// function s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToGrave,tp,LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_grave(d, x, None)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
// function s.tgop(e,tp,eg,ep,ev,re,r,rp)
fn tgop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToGrave,tp,LOCATION_EXTRA,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| card::is_able_to_grave(d, x, None)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
