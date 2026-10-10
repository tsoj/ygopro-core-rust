//! E・HERO Prisma
use super::prelude::*;

pub const ID: u32 = 89312388;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cos
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCost(s.coscost)
    effect::set_cost(d, e1, Some(cost_target(coscost)));
    // e1:SetOperation(s.cosoperation)
    effect::set_operation(d, e1, Some(operation(cosoperation)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter2(c,fc)`
fn filter2(d: &mut Duel, c: CardId, fc: CardId) -> bool {
    // if not c:IsAbleToGraveAsCost() then return false end
    if !card::is_able_to_grave_as_cost(d, c) {
        return false;
    }
    // return c:IsCode(table.unpack(fc.material))
    let codes = crate::class::material(d, fc);
    card::is_code(d, c, &codes)
}
/// `s.filter1(c,tp)`
fn filter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c.material and c:IsType(TYPE_FUSION) and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_DECK,0,1,nil,c)
    !crate::class::material(d, c).is_empty()
        && card::is_type(d, c, TYPE_FUSION)
        && duel::is_existing_matching_card(d, Some(&|d, x| filter2(d, x, c)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.coscost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn coscost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter1,tp,LOCATION_EXTRA,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| filter1(d, x, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter1,tp,LOCATION_EXTRA,0,1,1,nil,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| filter1(d, x, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - a.tp, g.clone());
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local cg=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_DECK,0,1,1,nil,g:GetFirst())
    let fc = group::get_first(d, &g);
    let cg = duel::select_matching_card(d, a.tp, Some(&|d, x| filter2(d, x, fc.nil())), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(cg,REASON_COST)
    duel::send_to_grave(d, cg.clone(), REASON_COST, None, None);
    // e:SetLabel(cg:GetFirst():GetCode())
    let first = group::get_first(d, &cg).nil();
    let code = card::get_code(d, first).first().copied().map_or(0, |x| x as i64);
    effect::set_label(d, a.e, &[code]);
    true
}
/// `s.cosoperation(e,tp,eg,ep,ev,re,r,rp)`
fn cosoperation(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // e1:SetValue(e:GetLabel())
    let label = effect::get_label(d, a.e);
    effect::set_value(d, e1, label);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
