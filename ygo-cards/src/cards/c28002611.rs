//! Gadget Hauler
use super::prelude::*;

pub const ID: u32 = 28002611;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MORPHTRONIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(s.tg)
    effect::set_target(d, e1, Some(cost_target(tg)));
    // e1:SetOperation(s.op)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MORPHTRONIC) and c:IsMonster()
    card::is_set_card(d, c, &[SET_MORPHTRONIC as u16]) && card::is_monster(d, c)
}
// s.tg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
// s.op(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFacedown() or not c:IsRelateToEffect(e) then return end
    if card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_HAND,0,1,63,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_HAND, 0, 1, 63, false, NO_EX).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    let n = g.len() as i64;
    // Duel.SendtoGrave(g,REASON_EFFECT)
    duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(#g*800)
    effect::set_value(d, e1, n * 800);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
