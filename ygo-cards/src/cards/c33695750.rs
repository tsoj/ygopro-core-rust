//! Des Mosquito
//! デス・モスキート
use super::prelude::*;

pub const ID: u32 = 33695750;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0x27)
    card::enable_counter_permit(d, c, 0x27, Some(LOCATION_MZONE), None);
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
    // destroy replace
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e4:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e4, EFFECT_DESTROY_REPLACE);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTarget(s.reptg)
    effect::set_target(d, e4, Some(cost_target(reptg)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.addct(e,tp,eg,ep,ev,re,r,rp,chk)`
fn addct(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,1,0,0x27)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, 1, 0, 0x27);
    true
}
/// `s.addc(e,tp,eg,ep,ev,re,r,rp)`
fn addc(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        // e:GetHandler():AddCounter(0x27,2)
        card::add_counter(d, h, 0x27, 2, false);
    }
}
/// `s.attackup(e,c)`
fn attackup(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetCounter(0x27)*500
    card::get_counter(d, c, 0x27) as i64 * 500
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsReason(REASON_BATTLE)
    //     and e:GetHandler():IsCanRemoveCounter(tp,0x27,1,REASON_COST) end
    if a.chk == Some(0) {
        return card::is_reason(d, h, REASON_BATTLE) && card::is_can_remove_counter(d, h, a.tp, 0x27, 1, REASON_COST);
    }
    // e:GetHandler():RemoveCounter(tp,0x27,1,REASON_EFFECT)
    card::remove_counter(d, h, a.tp, 0x27, 1, REASON_EFFECT);
    // return true
    true
}
