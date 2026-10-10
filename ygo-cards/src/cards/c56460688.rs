//! 異次元隔離マシーン
//! Different Dimension Gate
use super::prelude::*;

pub const ID: u32 = 56460688;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // remove
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // return
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetOperation(s.retop)
    effect::set_operation(d, e2, Some(operation(retop)));
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToRemove,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingTarget(Card.IsAbleToRemove,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g1=Duel.SelectTarget(tp,Card.IsAbleToRemove,tp,LOCATION_MZONE,0,1,1,nil)
    let mut g1 = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g2=Duel.SelectTarget(tp,Card.IsAbleToRemove,tp,0,LOCATION_MZONE,1,1,nil)
    let g2 = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // if e:GetLabelObject() then
    if !effect::get_label_object(d, e).is_nil() {
        // e:GetLabelObject():DeleteGroup()
        // e:SetLabelObject(nil)
        effect::set_label_object(d, e, Value::Nil);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g1,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g1), 2, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, e).unwrap_or_default();
    // if Duel.Remove(tg,0,REASON_EFFECT|REASON_TEMPORARY)~=0 then
    if duel::remove(d, tg, 0, REASON_EFFECT | REASON_TEMPORARY, None, None) != 0 {
        // local g=Duel.GetOperatedGroup()
        let g = duel::get_operated_group(d);
        // local tc=g:GetFirst()
        let _ = group::get_first(d, &g);
        // for tc in aux.Next(g) do
        for tc in group::iter(d, &g) {
            // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
            card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
        }
        // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD_EXC_GRAVE,0,1)
        card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD_EXC_GRAVE, 0, 1, 0, 0);
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
    }
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // if e:GetHandler():GetFlagEffect(id)==0 then return end
    let c = effect::get_handler(d, e).nil();
    if card::get_flag_effect(d, c, ID) == 0 {
        return;
    }
    // if (r&REASON_DESTROY)~=0 then
    if (a.r & REASON_DESTROY) != 0 {
        // local g=e:GetLabelObject():GetLabelObject()
        let e1 = effect::get_label_object(d, e).as_effect().nil();
        let g = effect::get_label_object(d, e1).as_group().cloned().nil();
        // local tc=g:GetFirst()
        // for tc in aux.Next(g) do
        for tc in group::iter(d, &g) {
            // if tc:GetFlagEffect(id)>0 then
            if card::get_flag_effect(d, tc, ID) > 0 {
                // Duel.ReturnToField(tc)
                duel::return_to_field(d, tc, None, None);
            }
        }
        // g:DeleteGroup()
        // e:GetLabelObject():SetLabelObject(nil)
        effect::set_label_object(d, e1, Value::Nil);
    }
}
