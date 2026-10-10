//! Chaos Grepher
//! Logical Nonsense
use super::prelude::*;

pub const ID: u32 = 18859369;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Also treated as a DARK monster on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_ADD_ATTRIBUTE)
    effect::set_code(d, e1, EFFECT_ADD_ATTRIBUTE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(ATTRIBUTE_DARK)
    effect::set_value(d, e1, ATTRIBUTE_DARK as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Send 1 LIGHT/DARK monster from your Deck to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(s.tgcost)
    effect::set_cost(d, e2, Some(cost_target(tgcost)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT|ATTRIBUTE_DARK) and c:IsDiscardable()
    //     and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil,c:GetAttribute())
    if !(card::is_attribute(d, c, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK) && card::is_discardable(d, c, None, None)) {
        return false;
    }
    let att = card::get_attribute(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| tgfilter(d, x, att)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.tgfilter(c,att)`
fn tgfilter(d: &mut Duel, c: CardId, att: u32) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT|ATTRIBUTE_DARK) and not c:IsAttribute(att) and c:IsAbleToGrave()
    card::is_attribute(d, c, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK) && !card::is_attribute(d, c, att) && card::is_able_to_grave(d, c, None)
}
/// `s.tgcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,nil,tp) end
    if a.chk == Some(0) {
        let tp = a.tp;
        return duel::is_existing_matching_card(d, Some(&|d, x| cfilter(d, x, tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,s.cfilter,1,1,REASON_COST|REASON_DISCARD,nil,tp)
    let tp = a.tp;
    duel::discard_hand(d, a.tp, Some(&|d, x| cfilter(d, x, tp)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    // e:SetLabel(Duel.GetOperatedGroup():GetFirst():GetAttribute())
    let og = duel::get_operated_group(d);
    let first = group::get_first(d, &og).nil();
    let att = card::get_attribute(d, first);
    effect::set_label(d, a.e, &[att as i64]);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil,e:GetLabel()):GetFirst()
    let label = effect::get_label(d, a.e) as u32;
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| tgfilter(d, x, label)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc and Duel.SendtoGrave(tc,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_GRAVE) then
    if let Some(tc) = tc {
        if duel::send_to_grave(d, CardSet::from_card(tc), REASON_EFFECT, None, None) > 0 && card::is_location(d, tc, LOCATION_GRAVE) {
            // Cannot Special Summon monsters with the same name
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetDescription(aux.Stringid(id,1))
            effect::set_description(d, e1, aux::stringid(ID, 1));
            // e1:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
            effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
            // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
            effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
            // e1:SetTargetRange(1,0)
            effect::set_target_range(d, e1, 1, 0);
            // e1:SetTarget(function(e,c) return c:IsCode(e:GetLabel()) end)
            effect::set_target(d, e1, Some(effect_card_fn(|d, e, x| {
                let label = effect::get_label(d, e) as u32;
                card::is_code(d, x, &[label])
            })));
            // e1:SetLabel(tc:GetCode())
            let code = card::get_code(d, tc)[0];
            effect::set_label(d, e1, &[code as i64]);
            // e1:SetReset(RESET_PHASE|PHASE_END)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, a.tp);
        }
    }
}
