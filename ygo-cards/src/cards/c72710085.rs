//! 星邪の神喰
//! Jewels of the Valiant
use super::prelude::*;

pub const ID: u32 = 72710085;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_REMOVE)
    effect::set_code(d, e2, EVENT_REMOVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.tgcon)
    effect::set_condition(d, e2, Some(condition(tgcon)));
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.tgcon(e,tp,eg,ep,ev,re,r,rp)`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local tc=eg:GetFirst()
    // return #eg==1 and tc:IsPreviousLocation(LOCATION_GRAVE) and tc:IsControler(tp) and tc:IsMonster()
    let eg = a.eg.clone().nil();
    if eg.len() != 1 {
        return false;
    }
    let tc = group::get_first(d, &eg).nil();
    card::is_previous_location(d, tc, LOCATION_GRAVE) && card::is_controler(d, tc, tp) && card::is_monster(d, tc)
}
/// `s.filter(c,att)`
fn filter(d: &mut Duel, c: CardId, att: u32) -> bool {
    // return not c:IsAttribute(att) and c:IsMonster() and c:IsAbleToGrave()
    !card::is_attribute(d, c, att) && card::is_monster(d, c) && card::is_able_to_grave(d, c, None)
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // if chk==0 then return e:GetHandler():IsRelateToEffect(e)
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,eg:GetFirst():GetAttribute()) end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, e).nil();
        if !card::is_relate_to_effect(d, c, e) {
            return false;
        }
        let eg = a.eg.clone().nil();
        let att = card::get_attribute(d, group::get_first(d, &eg).nil()) as u32;
        return duel::is_existing_matching_card(d, Some(&|d, cc| filter(d, cc, att)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // e:SetLabel(eg:GetFirst():GetAttribute())
    let eg = a.eg.clone().nil();
    let att = card::get_attribute(d, group::get_first(d, &eg).nil());
    effect::set_label(d, e, &[att as i64]);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil,e:GetLabel())
    let att = effect::get_label(d, e) as u32;
    let g = duel::select_matching_card(d, tp, Some(&|d, cc| filter(d, cc, att)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
