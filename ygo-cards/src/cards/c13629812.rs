//! Ninjitsu Art of Shadow Sealing
use super::prelude::*;

pub const ID: u32 = 13629812;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NINJA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
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
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // return
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    // e2:SetCondition(s.retcon)
    effect::set_condition(d, e2, Some(condition(retcon)));
    // e2:SetOperation(s.retop)
    effect::set_operation(d, e2, Some(operation(retop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_NINJA)
    card::is_set_card(d, c, &[SET_NINJA as u16])
}
// function s.filter(c,e)
fn filter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsAbleToRemove() and c:IsCanBeEffectTarget(e)
    card::is_able_to_remove(d, c, None, None, None) && card::is_can_be_effect_target(d, c, Some(e))
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // local dg=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_MZONE,nil,e)
    let dg = duel::get_matching_group(d, Some(&|d, c| filter(d, c, a.e)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // aux.ReleaseCheckTarget with the extra argument dg
    let rc = |d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| aux::release_check_target(d, sg, tp, exg, &dg);
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,aux.ReleaseCheckTarget,nil,dg) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&cfilter), 1, 1, false, Some(&rc), NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,aux.ReleaseCheckTarget,nil,dg)
    let g = duel::select_release_group_cost(d, a.tp, Some(&cfilter), 1, 1, false, Some(&rc), NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsAbleToRemove() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, 1 - a.tp)
            && card::is_able_to_remove(d, chkc, None, None, None);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToRemove,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not c:IsRelateToEffect(e) or not tc then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let Some(tc) = tc else { return };
    // local seq=tc:GetSequence()
    let mut seq = card::get_sequence(d, tc) as i64;
    // if tc:IsControler(1-tp) then seq=seq+16 end
    if card::is_controler(d, tc, 1 - a.tp) {
        seq += 16;
    }
    // if tc:IsRelateToEffect(e) and Duel.Remove(tc,0,REASON_EFFECT|REASON_TEMPORARY)~=0 and tc:IsLocation(LOCATION_REMOVED) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::remove(d, tc, 0, REASON_EFFECT | REASON_TEMPORARY, None, None) != 0
        && card::is_location(d, tc, LOCATION_REMOVED)
    {
        // c:SetCardTarget(tc)
        card::set_card_target(d, c, tc);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_DISABLE_FIELD)
        effect::set_code(d, e1, EFFECT_DISABLE_FIELD);
        // e1:SetRange(LOCATION_SZONE)
        effect::set_range(d, e1, LOCATION_SZONE);
        // e1:SetLabel(seq)
        effect::set_label(d, e1, &[seq]);
        // e1:SetCondition(s.discon)
        effect::set_condition(d, e1, Some(condition(discon)));
        // e1:SetOperation(s.disop)
        effect::set_operation(d, e1, Some(lua_fn(disop)));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
// function s.discon(e)
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetCardTargetCount()>0
    let c = effect::get_handler(d, a.e).nil();
    card::get_card_target_count(d, c) > 0
}
// function s.disop(e,tp)
fn disop(d: &mut Duel, a: Args) -> i64 {
    // return 0x1<<e:GetLabel()
    let label = effect::get_label(d, a.e(0));
    0x1i64 << label
}
// function s.retcon(e,tp,eg,ep,ev,re,r,rp)
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=c:GetFirstCardTarget()
    let tc = card::get_first_card_target(d, c);
    // if tc and tc:IsLocation(LOCATION_REMOVED) and not c:IsLocation(LOCATION_DECK) then
    if let Some(tc) = tc {
        if card::is_location(d, tc, LOCATION_REMOVED) && !card::is_location(d, c, LOCATION_DECK) {
            // e:SetLabelObject(tc)
            effect::set_label_object(d, a.e, Value::Card(tc));
            // tc:CreateEffectRelation(e)
            card::create_effect_relation(d, tc, a.e);
            // return true
            return true;
        }
    }
    // else return false end
    false
}
// function s.retop(e,tp,eg,ep,ev,re,r,rp)
fn retop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, a.e).nil();
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card();
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // local seq=tc:GetPreviousSequence()
            let seq = card::get_previous_sequence(d, tc);
            // if seq>4 then
            if seq > 4 {
                // Duel.SendtoGrave(tc,REASON_RULE|REASON_RETURN)
                duel::send_to_grave(d, tc, REASON_RULE | REASON_RETURN, None, None);
            }
            // local zone=0x1<<seq
            let zone = 0x1u32 << seq;
            // Duel.ReturnToField(tc,tc:GetPreviousPosition(),zone)
            let pos = card::get_previous_position(d, tc) as u8;
            duel::return_to_field(d, tc, Some(pos), Some(zone));
        }
    }
}
