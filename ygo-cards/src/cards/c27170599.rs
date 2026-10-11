//! Ghostrick or Treat
use super::prelude::*;

pub const ID: u32 = 27170599;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GHOSTRICK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_DISABLE + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_GHOSTRICK) and (c:IsLinkMonster() or c:IsFieldSpell())
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GHOSTRICK as u16]) && (card::is_link_monster(d, c) || card::is_field_spell(d, c))
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_ONFIELD,0,1,nil)
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,Card.IsFaceup,tp,0,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    // if not (tc:IsFaceup() and tc:IsRelateToEffect(e)) then return end
    let tc = duel::first_target(d).nil();
    if !(card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e)) {
        return;
    }
    // if Duel.CheckLPCost(1-tp,2000) and c:IsSSetable(true) and e:IsHasType(EFFECT_TYPE_ACTIVATE)
    //     and Duel.SelectYesNo(1-tp,aux.Stringid(id,1)) then
    if duel::check_lp_cost(d, 1 - a.tp, 2000)
        && card::is_s_setable(d, c, true, None)
        && effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE)
        && duel::select_yes_no(d, 1 - a.tp, aux::stringid(ID, 1))
    {
        // Duel.PayLPCost(1-tp,2000)
        duel::pay_lp_cost(d, 1 - a.tp, 2000);
        // c:CancelToGrave()
        card::cancel_to_grave(d, c, true);
        // Duel.ChangePosition(c,POS_FACEDOWN)
        duel::change_position(d, c, POS_FACEDOWN as u8, None, None, None, false);
        // Duel.RaiseEvent(c,EVENT_SSET,e,REASON_EFFECT,tp,tp,0)
        duel::raise_event(d, c, EVENT_SSET, Some(a.e), REASON_EFFECT, a.tp, a.tp, 0);
    } else {
        // Duel.NegateRelatedChain(tc,RESET_TURN_SET)
        duel::negate_related_chain(d, tc, RESET_TURN_SET);
        // Negate its effects
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        // e2:SetValue(RESET_TURN_SET)
        effect::set_value(d, e2, RESET_TURN_SET as i64);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // Cannot attack
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetDescription(3206)
        effect::set_description(d, e3, 3206);
        // e3:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e3, EFFECT_FLAG_CLIENT_HINT, 0);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetCode(EFFECT_CANNOT_ATTACK)
        effect::set_code(d, e3, EFFECT_CANNOT_ATTACK);
        // e3:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e3)
        card::register_effect(d, tc, e3, false);
        // if not tc:IsImmuneToEffect(e) then
        if !card::is_immune_to_effect(d, tc, a.e) {
            // Change Position during the End Phase
            // local fid=c:GetFieldID()
            let fid = card::get_field_id(d, c);
            // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1,fid)
            card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, fid as i64, 0);
            // local e4=Effect.CreateEffect(c)
            let e4 = effect::create_effect(d, c);
            // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e4:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
            effect::set_property(d, e4, EFFECT_FLAG_IGNORE_IMMUNE, 0);
            // e4:SetCode(EVENT_PHASE+PHASE_END)
            effect::set_code(d, e4, EVENT_PHASE + PHASE_END);
            // e4:SetCountLimit(1)
            effect::set_count_limit(d, e4, 1, 0, 0, 0);
            // e4:SetLabel(fid)
            effect::set_label(d, e4, &[fid as i64]);
            // e4:SetLabelObject(tc)
            effect::set_label_object(d, e4, Value::Card(tc));
            // e4:SetCondition(s.flipcon)
            effect::set_condition(d, e4, Some(condition(flipcon)));
            // e4:SetOperation(s.flipop)
            effect::set_operation(d, e4, Some(operation(flipop)));
            // Duel.RegisterEffect(e4,tp)
            duel::register_effect(d, e4, a.tp);
        }
    }
}
/// `s.flipcon(e,tp,eg,ep,ev,re,r,rp)`
fn flipcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if e:GetLabelObject():GetFlagEffectLabel(id)~=e:GetLabel() then
    let lo = effect::get_label_object(d, a.e).as_card().nil();
    let labels = card::get_flag_effect_label(d, lo, ID);
    if labels.first().copied() != Some(effect::get_label(d, a.e)) {
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        return false;
    }
    // else return true end
    true
}
/// `s.flipop(e,tp,eg,ep,ev,re,r,rp)`
fn flipop(d: &mut Duel, a: &EffArgs) {
    // Duel.ChangePosition(e:GetLabelObject(),POS_FACEDOWN_DEFENSE)
    let lo = effect::get_label_object(d, a.e).as_card().nil();
    duel::change_position(d, lo, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
}
