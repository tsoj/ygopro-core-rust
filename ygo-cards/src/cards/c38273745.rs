//! Evilswarm Ouroboros
use super::prelude::*;

pub const ID: u32 = 38273745;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure
    // Xyz.AddProcedure(c,nil,4,3)
    xyz::add_procedure(d, c, None, Some(4), 3, None, None, None, None, false, None);
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then
    if let Some(chkc) = a.chkc() {
        // local op=e:GetLabel()
        let op = effect::get_label(d, a.e);
        // if op==2 or not chkc:IsControler(1-tp) then return false end
        if op == 2 || !card::is_controler(d, chkc, 1 - a.tp) {
            return false;
        }
        // if op==1 then
        if op == 1 {
            // return chkc:IsOnField() and chkc:IsAbleToHand()
            return card::is_on_field(d, chkc) && card::is_able_to_hand(d, chkc, None);
        } else if op == 3 {
            // return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsAbleToRemove()
            return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_able_to_remove(d, chkc, None, None, None);
        }
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Return 1 card your opponent controls to the hand
    // local b1=not c:HasFlagEffect(id)
    //     and Duel.IsExistingTarget(Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,1,nil)
    let b1 = !card::has_flag_effect(d, c, ID, None)
        && duel::is_existing_target(d, Some(&|d, x| card::is_able_to_hand(d, x, None)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    // Send 1 random card from your opponent's hand to the GY
    // local b2=not c:HasFlagEffect(id+1)
    //     and Duel.GetFieldGroupCount(1-tp,LOCATION_HAND,0)>0
    let b2 = !card::has_flag_effect(d, c, ID + 1, None)
        && duel::get_field_group_count(d, 1 - a.tp, LOCATION_HAND, 0) > 0;
    // Banish 1 card from your opponent's GY
    // local b3=not c:HasFlagEffect(id+2)
    //     and Duel.IsExistingTarget(Card.IsAbleToRemove,tp,0,LOCATION_GRAVE,1,nil)
    let b3 = !card::has_flag_effect(d, c, ID + 2, None)
        && duel::is_existing_target(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), a.tp, 0, LOCATION_GRAVE, 1, NO_EX);
    // if chk==0 then return b1 or b2 or b3 end
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)},
    //     {b3,aux.Stringid(id,3)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))]).nil() as i64;
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    // c:RegisterFlagEffect(id+op-1,RESET_EVENT|RESETS_STANDARD,0,1)
    card::register_flag_effect(d, c, (ID as i64 + op - 1) as u32, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
    if op == 1 {
        // e:SetCategory(CATEGORY_TOHAND)
        effect::set_category(d, a.e, CATEGORY_TOHAND);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
        // local g=Duel.SelectTarget(tp,Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,1,1,nil)
        let g = duel::select_target(d, a.tp, Some(&|d, x| card::is_able_to_hand(d, x, None)), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, a.tp, 0);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_TOGRAVE)
        effect::set_category(d, a.e, CATEGORY_TOGRAVE);
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
        // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,1-tp,LOCATION_HAND)
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, 1 - a.tp, LOCATION_HAND as i32);
    } else if op == 3 {
        // e:SetCategory(CATEGORY_REMOVE)
        effect::set_category(d, a.e, CATEGORY_REMOVE);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // local g=Duel.SelectTarget(tp,Card.IsAbleToRemove,tp,0,LOCATION_GRAVE,1,1,nil)
        let g = duel::select_target(d, a.tp, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), a.tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, a.tp, 0);
    }
    true
}
// s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // Return 1 card your opponent controls to the hand
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsRelateToEffect(e) and tc:IsControler(1-tp) then
        if card::is_relate_to_effect(d, tc, a.e) && card::is_controler(d, tc, 1 - a.tp) {
            // Duel.SendtoHand(tc,nil,REASON_EFFECT)
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        }
    } else if op == 2 {
        // Send 1 random card from your opponent's hand to the GY
        // local g=Duel.GetFieldGroup(1-tp,LOCATION_HAND,0)
        let g = duel::get_field_group(d, 1 - a.tp, LOCATION_HAND, 0);
        // if #g>0 then
        if !g.is_empty() {
            // local sg=g:RandomSelect(1-tp,1)
            let sg = group::random_select(d, &g, 1 - a.tp, 1);
            // Duel.SendtoGrave(sg,REASON_EFFECT)
            duel::send_to_grave(d, &sg, REASON_EFFECT, None, None);
        }
    } else if op == 3 {
        // Banish 1 card from your opponent's GY
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsRelateToEffect(e) and tc:IsControler(1-tp) then
        if card::is_relate_to_effect(d, tc, a.e) && card::is_controler(d, tc, 1 - a.tp) {
            // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
