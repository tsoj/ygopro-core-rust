//! Swords at Dawn
use super::prelude::*;

pub const ID: u32 = 25067275;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // oath effects
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // aux.RegisterClientHint(e:GetHandler(),nil,tp,1,0,aux.Stringid(id,2),nil)
    aux::register_client_hint(d, h, 0, a.tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
    true
}
// function s.tcfilter(tc,ec)
fn tcfilter(d: &mut Duel, tc: CardId, ec: CardId) -> bool {
    // return tc:IsFaceup() and ec:CheckEquipTarget(tc)
    card::is_faceup(d, tc) && card::check_equip_target(d, ec, tc)
}
// function s.ecfilter(c)
fn ecfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_EQUIP) and Duel.IsExistingTarget(s.tcfilter,0,LOCATION_MZONE,LOCATION_MZONE,1,nil,c)
    card::is_type(d, c, TYPE_EQUIP)
        && duel::is_existing_target(d, Some(&|d, tc| tcfilter(d, tc, c)), 0, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // if not Duel.IsExistingTarget(s.ecfilter,tp,LOCATION_GRAVE,0,1,nil) then return false end
        if !duel::is_existing_target(d, Some(&ecfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX) {
            return false;
        }
        // if e:GetHandler():IsLocation(LOCATION_HAND) then
        let h = effect::get_handler(d, a.e).nil();
        if card::is_location(d, h, LOCATION_HAND) {
            // return Duel.GetLocationCount(tp,LOCATION_SZONE)>1
            return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 1;
        } else {
            // return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
            return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0;
        }
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 0));
    // local g=Duel.SelectTarget(tp,s.ecfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&ecfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // local ec=g:GetFirst()
    let ec = group::get_first(d, &g).nil();
    // e:SetLabelObject(ec)
    effect::set_label_object(d, a.e, Value::Card(ec));
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
    // Duel.SelectTarget(tp,s.tcfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,ec:GetEquipTarget(),ec)
    let eqt = card::get_equip_target(d, ec);
    duel::select_target(d, tp, Some(&|d, tc| tcfilter(d, tc, ec)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, eqt.into());
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,ec,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(ec)), 1, 0, 0);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ec=e:GetLabelObject()
    let ec = effect::get_label_object(d, a.e).as_card();
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // local tc=g:GetFirst()
    let order = group::iter(d, &g);
    let mut tc = order.first().copied();
    // if tc==ec then tc=g:GetNext() end
    if tc == ec {
        tc = order.get(1).copied();
    }
    // if ec:IsFaceup() and ec:IsRelateToEffect(e) then
    if card::is_faceup(d, ec.nil()) && card::is_relate_to_effect(d, ec.nil(), a.e) {
        // Duel.Equip(tp,ec,tc)
        duel::equip(d, tp, ec.nil(), tc.nil(), true, false);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetRange(LOCATION_SZONE)
        effect::set_range(d, e1, LOCATION_SZONE);
        // e1:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetOperation(s.desop)
        effect::set_operation(d, e1, Some(operation(desop)));
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // ec:RegisterEffect(e1)
        card::register_effect(d, ec.nil(), e1, false);
    }
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::destroy(d, h, REASON_EFFECT, None, None);
}
