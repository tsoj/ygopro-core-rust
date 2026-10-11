//! Apocavities
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 28531163;

/// `local COUNTER_CARIES=0x215`
const COUNTER_CARIES: u16 = 0x215;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[0x215], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_CARIES)
    card::enable_counter_permit(d, c, COUNTER_CARIES, None, None);
    // When this card is activated: Place C Counters on this card equal to the number of cards your opponent controls
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(function(e,tp) e:GetHandler():AddCounter(COUNTER_CARIES,Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)) end)
    effect::set_operation(d, e1, Some(operation(activate_op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Apply effects to a target based on the number of C Counters removed from this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp) e:GetHandler():AddCounter(COUNTER_CARIES,Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)) end`
fn activate_op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let n = duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD);
    card::add_counter(d, c, COUNTER_CARIES, n as u16, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)
    let ct = duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD);
    // if chk==0 then return ct>0 and Duel.IsCanAddCounter(tp,COUNTER_CARIES,ct,c) end
    if a.chk == Some(0) {
        return ct > 0 && duel::is_can_add_counter(d, a.tp, Some((COUNTER_CARIES, ct as u32, c)));
    }
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,c,1,COUNTER_CARIES,ct)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, Some(CardSet::from_card(c)), 1, COUNTER_CARIES as u8, ct);
    true
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc) && card::is_controler(d, chkc, 1 - a.tp);
    }
    // if chk==0 then return Duel.IsCanRemoveCounter(tp,1,0,COUNTER_CARIES,1,REASON_COST)
    //     and Duel.IsExistingTarget(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_can_remove_counter(d, a.tp, 1, 0, COUNTER_CARIES, 1, REASON_COST)
            && duel::is_existing_target(d, Some(&|d, x| card::is_faceup(d, x)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_APPLYTO)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_APPLYTO as u64);
    // local tc=Duel.SelectTarget(tp,Card.IsFaceup,tp,0,LOCATION_MZONE,1,1,nil):GetFirst()
    let sel = duel::select_target(d, a.tp, Some(&|d, x| card::is_faceup(d, x)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    // local b1=Duel.IsCanRemoveCounter(tp,1,0,COUNTER_CARIES,1,REASON_COST)
    let b1 = duel::is_can_remove_counter(d, a.tp, 1, 0, COUNTER_CARIES, 1, REASON_COST);
    // local b2=Duel.IsCanRemoveCounter(tp,1,0,COUNTER_CARIES,2,REASON_COST) and not tc:IsAttack(0)
    let b2 = duel::is_can_remove_counter(d, a.tp, 1, 0, COUNTER_CARIES, 2, REASON_COST) && !card::is_attack(d, tc, &[0]);
    // local b3=Duel.IsCanRemoveCounter(tp,1,0,COUNTER_CARIES,3,REASON_COST) and tc:IsNegatableMonster()
    let b3 = duel::is_can_remove_counter(d, a.tp, 1, 0, COUNTER_CARIES, 3, REASON_COST) && card::is_negatable_monster(d, tc);
    // local b4=Duel.IsCanRemoveCounter(tp,1,0,COUNTER_CARIES,4,REASON_COST)
    let b4 = duel::is_can_remove_counter(d, a.tp, 1, 0, COUNTER_CARIES, 4, REASON_COST);
    // local ct=Duel.SelectEffect(tp,{b1,aux.Stringid(id,2)},{b2,aux.Stringid(id,3)},{b3,aux.Stringid(id,4)},{b4,aux.Stringid(id,5)})
    let ct = duel::select_effect(
        d,
        a.tp,
        &[
            (b1, aux::stringid(ID, 2)),
            (b2, aux::stringid(ID, 3)),
            (b3, aux::stringid(ID, 4)),
            (b4, aux::stringid(ID, 5)),
        ],
    )
    .nil() as u32;
    // Duel.RemoveCounter(tp,1,1,COUNTER_CARIES,ct,REASON_COST)
    duel::remove_counter(d, a.tp, 1, 1, COUNTER_CARIES, ct, REASON_COST);
    // e:SetLabel(ct)
    effect::set_label(d, a.e, &[ct as i64]);
    // if (ct==1 or ct==2) then
    if ct == 1 || ct == 2 {
        // e:SetCategory(CATEGORY_ATKCHANGE)
        effect::set_category(d, a.e, CATEGORY_ATKCHANGE);
        // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,tc,1,tp,(ct==1 and -500 or -tc:GetAttack()))
        let param = if ct == 1 { -500 } else { -card::get_attack(d, tc) };
        duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(tc)), 1, a.tp, param);
    } else if ct == 3 {
        // e:SetCategory(CATEGORY_DISABLE)
        effect::set_category(d, a.e, CATEGORY_DISABLE);
        // Duel.SetOperationInfo(0,CATEGORY_DISABLE,tc,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(CardSet::from_card(tc)), 1, a.tp, 0);
    } else if ct == 4 {
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, a.e, CATEGORY_DESTROY);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(tc)), 1, a.tp, 0);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if not tc:IsRelateToEffect(e) then return end
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    if ct == 1 && card::is_faceup(d, tc) {
        // It loses 500 ATK
        // tc:UpdateAttack(-500,nil,c)
        card::update_attack(d, tc, -500, None, Some(c), None);
    } else if ct == 2 && card::is_faceup(d, tc) {
        // Change its ATK to 0
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(0)
        effect::set_value(d, e1, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    } else if ct == 3 && card::is_faceup(d, tc) {
        // Negate its effects
        // tc:NegateEffects(c)
        card::negate_effects(d, tc, c, None, false, None);
    } else if ct == 4 && card::is_monster(d, tc) {
        // Destroy it
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
