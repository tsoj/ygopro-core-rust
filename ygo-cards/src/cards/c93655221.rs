//! Shield Handler
use super::prelude::*;

pub const ID: u32 = 93655221;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    true
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsOnField() and c:IsMonster()
    card::is_on_field(d, c) && card::is_monster(d, c)
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if not Duel.IsChainNegatable(ev) then return false end
    if !duel::is_chain_negatable(d, a.ev as u8) {
        return false;
    }
    // local ex,tg,tc=Duel.GetOperationInfo(ev,CATEGORY_DESTROY)
    let Some(info) = duel::get_operation_info(d, a.ev as u32, CATEGORY_DESTROY) else { return false };
    // return ex and tg~=nil and tc+tg:FilterCount(s.filter,nil)-#tg>0
    let Some(tg) = info.op_cards else { return false };
    let fc = group::filter_count(d, &tg, &filter, NO_EX) as i64;
    info.op_count as i64 + fc - tg.len() as i64 > 0
}
// function s.cfilter(c,sf)
fn cfilter(d: &mut Duel, c: CardId, sf: bool) -> bool {
    // return c:IsLinkMonster() and c:IsFaceup() and (sf or c:IsNegatableMonster())
    card::is_link_monster(d, c) && card::is_faceup(d, c) && (sf || card::is_negatable_monster(d, c))
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingTarget(s.cfilter,tp,0,LOCATION_MZONE,1,nil,false)
    //     and Duel.IsExistingTarget(s.cfilter,tp,LOCATION_MZONE,0,1,nil,true) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| cfilter(d, c, false)), tp, 0, LOCATION_MZONE, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| cfilter(d, c, true)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // if e:GetLabel()==1 then aux.RemainFieldCost(e,tp,eg,ep,ev,re,r,rp,1) end
    if effect::get_label(d, a.e) == 1 {
        let args = [
            Value::Effect(a.e),
            Value::Int(tp as i64),
            Value::Nil,
            Value::Nil,
            Value::Int(a.ev as i64),
            Value::Nil,
            Value::Int(a.r as i64),
            Value::Nil,
            Value::Int(1),
        ];
        aux::remain_field_cost().call(d, &args);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_OPPO)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_OPPO as u64);
    // local g=Duel.SelectTarget(tp,s.cfilter,tp,0,LOCATION_MZONE,1,1,nil,false)
    let g = duel::select_target(d, tp, Some(&|d, c| cfilter(d, c, false)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // e:SetLabelObject(g:GetFirst())
    let first = group::get_first(d, &g).nil();
    effect::set_label_object(d, a.e, Value::Card(first));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SELF)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SELF as u64);
    // Duel.SelectTarget(tp,s.cfilter,tp,LOCATION_MZONE,0,1,1,nil,true)
    let _ = duel::select_target(d, tp, Some(&|d, c| cfilter(d, c, true)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local hc=e:GetLabelObject()
    let hc = effect::get_label_object(d, a.e).as_card().nil();
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // local tc=g:GetFirst()
    let mut tc = group::get_first(d, &g);
    // if tc==hc then tc=g:GetNext() end
    if tc == Some(hc) {
        tc = group::iter(d, &g).get(1).copied();
    }
    // if hc:IsFaceup() and hc:IsRelateToEffect(e) and not hc:IsDisabled() then
    if card::is_faceup(d, hc) && card::is_relate_to_effect(d, hc, a.e) && !card::is_disabled(d, hc) {
        // Duel.NegateRelatedChain(hc,RESET_TURN_SET)
        duel::negate_related_chain(d, hc, RESET_TURN_SET);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // hc:RegisterEffect(e1)
        card::register_effect(d, hc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        // e2:SetValue(RESET_TURN_SET)
        effect::set_value(d, e2, RESET_TURN_SET as i64);
        // hc:RegisterEffect(e2)
        card::register_effect(d, hc, e2, false);
        // if not hc:IsImmuneToEffect(e1) and not hc:IsImmuneToEffect(e2)
        //     and tc:IsRelateToEffect(e) and tc:IsFaceup() and c:IsRelateToEffect(e) then
        if !card::is_immune_to_effect(d, hc, e1)
            && !card::is_immune_to_effect(d, hc, e2)
            && card::is_relate_to_effect(d, tc.nil(), a.e)
            && card::is_faceup(d, tc.nil())
            && card::is_relate_to_effect(d, c, a.e)
        {
            let tc = tc.nil();
            // Duel.Equip(tp,c,tc)
            duel::equip(d, a.tp, c, tc, true, false);
            // c:CancelToGrave()
            card::cancel_to_grave(d, c, true);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_EQUIP)
            effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
            // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
            effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // e1:SetValue(1)
            effect::set_value(d, e1, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_EQUIP_LIMIT)
            effect::set_code(d, e2, EFFECT_EQUIP_LIMIT);
            // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e2:SetValue(s.eqlimit)
            effect::set_value_fn(d, e2, effect_card_fn(eqlimit));
            // e2:SetLabelObject(tc)
            effect::set_label_object(d, e2, Value::Card(tc));
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // c:RegisterEffect(e2)
            card::register_effect(d, c, e2, false);
        } else {
            // c:CancelToGrave(false)
            card::cancel_to_grave(d, c, false);
        }
    } else {
        // c:CancelToGrave(false)
        card::cancel_to_grave(d, c, false);
    }
}
// function s.eqlimit(e,c)
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c==e:GetLabelObject()
    effect::get_label_object(d, e).as_card() == Some(c)
}
