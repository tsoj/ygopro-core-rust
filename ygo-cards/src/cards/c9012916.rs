//! Black-Winged Dragon
use super::prelude::*;

pub const ID: u32 = 9012916;
pub const INFO: ScriptInfo = ScriptInfo { counter_list: &[COUNTER_FEATHER as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Allow Feather Counters
    // c:EnableCounterPermit(COUNTER_FEATHER)
    card::enable_counter_permit(d, c, COUNTER_FEATHER as u16, None, None);
    // Synchro Summon
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Replace effect damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CHANGE_DAMAGE)
    effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(s.damval)
    effect::set_value_fn(d, e1, lua_fn(damval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_NO_EFFECT_DAMAGE)
    effect::set_code(d, e2, EFFECT_NO_EFFECT_DAMAGE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Decrease its own ATK by each Feather Counter
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
    // e3:SetValue(s.atkval)
    effect::set_value_fn(d, e3, effect_card_value(atkval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Decrease ATK of a monster and inflict damage
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DAMAGE)
    effect::set_category(d, e4, CATEGORY_ATKCHANGE + CATEGORY_DAMAGE);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCost(s.cost)
    effect::set_cost(d, e4, Some(cost_target(cost_fn)));
    // e4:SetTarget(s.target)
    effect::set_target(d, e4, Some(cost_target(target)));
    // e4:SetOperation(s.operation)
    effect::set_operation(d, e4, Some(operation(operation_fn)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.damval(e,re,val,r,rp,rc)`
fn damval(d: &mut Duel, a: Args) -> i64 {
    let e = a.e(0);
    let val = a.int(2);
    let r = a.int(3) as u32;
    // if (r&REASON_EFFECT)~=0 then
    if (r & REASON_EFFECT) != 0 {
        // e:GetHandler():AddCounter(COUNTER_FEATHER,1)
        let h = effect::get_handler(d, e).nil();
        card::add_counter(d, h, COUNTER_FEATHER as u16, 1, false);
        // return 0
        return 0;
    }
    // return val
    val
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetCounter(COUNTER_FEATHER)*-700
    card::get_counter(d, c, COUNTER_FEATHER as u16) as i64 * -700
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():GetCounter(COUNTER_FEATHER)>0 end
    if a.chk == Some(0) {
        return card::get_counter(d, h, COUNTER_FEATHER as u16) > 0;
    }
    // local ct=e:GetHandler():GetCounter(COUNTER_FEATHER)
    let ct = card::get_counter(d, h, COUNTER_FEATHER as u16);
    // e:SetLabel(ct*700)
    effect::set_label(d, a.e, &[ct as i64 * 700]);
    // e:GetHandler():RemoveCounter(tp,COUNTER_FEATHER,ct,REASON_COST)
    card::remove_counter(d, h, a.tp, COUNTER_FEATHER as u16, ct as u16, REASON_COST);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.HasNonZeroAttack,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| card::has_non_zero_attack(d, x)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
    // local tc=Duel.SelectTarget(tp,Card.HasNonZeroAttack,tp,0,LOCATION_MZONE,1,1,nil)
    let tc = duel::select_target(d, tp, Some(&|d, x| card::has_non_zero_attack(d, x)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    let label = effect::get_label(d, a.e);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,e:GetLabel())
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, label as i32);
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,tc,1,0,-e:GetLabel())
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, tc, 1, 0, -(label as i32));
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and not tc:IsImmuneToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && !card::is_immune_to_effect(d, tc, a.e) {
        // local val=e:GetLabel()
        let val = effect::get_label(d, a.e);
        // local atk=tc:GetAttack()
        let atk = card::get_attack(d, tc) as i64;
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(-val)
        effect::set_value(d, e1, -val);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // if val>atk then
        if val > atk {
            // Duel.Damage(1-tp,atk,REASON_EFFECT)
            duel::damage(d, 1 - tp, atk, REASON_EFFECT, false, None);
        } else {
            // Duel.Damage(1-tp,val,REASON_EFFECT)
            duel::damage(d, 1 - tp, val, REASON_EFFECT, false, None);
        }
    }
}
