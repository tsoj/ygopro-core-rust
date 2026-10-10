//! Card Guard
use super::prelude::*;

pub const ID: u32 = 4694209;

// s.counter_place_list={0x1021}
pub const INFO: ScriptInfo = ScriptInfo {
    counter_place_list: &[0x1021],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon success
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.addct)
    effect::set_target(d, e1, Some(cost_target(addct)));
    // e1:SetOperation(s.addc)
    effect::set_operation(d, e1, Some(operation(addc)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // attackup
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetValue(s.attackup)
    effect::set_value_fn(d, e3, effect_card_value(attackup));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // destroy rep
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e4, CATEGORY_COUNTER);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetTarget(s.addct2)
    effect::set_target(d, e4, Some(cost_target(addct2)));
    // e4:SetOperation(s.addc2)
    effect::set_operation(d, e4, Some(operation(addc2)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.addct(e,tp,eg,ep,ev,re,r,rp,chk)`
fn addct(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,1,0,0x1021)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, 1, 0, 0x1021);
    true
}
/// `s.addc(e,tp,eg,ep,ev,re,r,rp)`
fn addc(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        // e:GetHandler():AddCounter(0x1021+COUNTER_NEED_ENABLE,1)
        card::add_counter(d, h, (0x1021 + COUNTER_NEED_ENABLE) as u16, 1, false);
    }
}
/// `s.attackup(e,c)`
fn attackup(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetCounter(0x1021)*300
    card::get_counter(d, c, 0x1021) as i64 * 300
}
/// `s.addct2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn addct2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsControler(tp) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, a.tp) && card::is_faceup(d, chkc);
    }
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsCanRemoveCounter(tp,0x1021,1,REASON_EFFECT)
    //     and Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_ONFIELD,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return card::is_can_remove_counter(d, h, a.tp, 0x1021, 1, REASON_EFFECT)
            && duel::is_existing_target(d, Some(&|d, x| card::is_faceup(d, x)), a.tp, LOCATION_ONFIELD, 0, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
    // Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_ONFIELD,0,1,1,e:GetHandler())
    duel::select_target(d, a.tp, Some(&|d, x| card::is_faceup(d, x)), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, h.into());
    true
}
/// `s.addc2(e,tp,eg,ep,ev,re,r,rp)`
fn addc2(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:GetCounter(0x1021)==0 then return end
    if card::get_counter(d, c, 0x1021) == 0 {
        return;
    }
    // c:RemoveCounter(tp,0x1021,1,REASON_EFFECT)
    card::remove_counter(d, c, a.tp, 0x1021, 1, REASON_EFFECT);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // tc:AddCounter(0x1021,1)
        card::add_counter(d, tc, 0x1021, 1, false);
        // if tc:GetFlagEffect(id)~=0 then return end
        if card::get_flag_effect(d, tc, ID) != 0 {
            return;
        }
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EFFECT_DESTROY_REPLACE)
        effect::set_code(d, e1, EFFECT_DESTROY_REPLACE);
        // e1:SetTarget(s.reptg)
        effect::set_target(d, e1, Some(cost_target(reptg)));
        // e1:SetOperation(s.repop)
        effect::set_operation(d, e1, Some(operation(repop)));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,0)
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 0, 0, 0);
    }
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return not e:GetHandler():IsReason(REASON_REPLACE+REASON_RULE) and e:GetHandler():GetCounter(0x1021)>0 end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return !card::is_reason(d, h, REASON_REPLACE + REASON_RULE) && card::get_counter(d, h, 0x1021) > 0;
    }
    // return true
    true
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn repop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RemoveCounter(tp,0x1021,1,REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    card::remove_counter(d, h, a.tp, 0x1021, 1, REASON_EFFECT);
}
