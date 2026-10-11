//! Safe Zone
use super::prelude::*;

pub const ID: u32 = 38296564;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddPersistentProcedure(c,nil,s.filter,nil,nil,nil,TIMINGS_CHECK_MONSTER_E,nil,nil,nil,s.op)
    aux::add_persistent_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, c, _e, _tp| filter(d, c))),
        None,
        None,
        None,
        Some(TIMINGS_CHECK_MONSTER_E),
        None,
        None,
        None,
        Some(operation(op)),
        false,
    );
    // eff
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(aux.PersistentTargetFilter)
    effect::set_target(d, e1, Some(aux::persistent_target_filter()));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // e2:SetValue(s.efilter)
    effect::set_value_fn(d, e2, lua_fn(efilter));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e3:SetValue(s.tgval)
    effect::set_value_fn(d, e3, lua_fn(tgval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e1:Clone()
    let e4 = effect::clone(d, e1);
    // e4:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
    effect::set_code(d, e4, EFFECT_CANNOT_DIRECT_ATTACK);
    // e4:SetCondition(s.acon)
    effect::set_condition(d, e4, Some(condition(acon)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Destroy
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e5:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e5, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e5:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e5, EVENT_LEAVE_FIELD_P);
    // e5:SetOperation(s.checkop)
    effect::set_operation(d, e5, Some(operation(checkop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e6, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e6:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e6, EVENT_LEAVE_FIELD);
    // e6:SetOperation(s.desop)
    effect::set_operation(d, e6, Some(operation(desop)));
    // e6:SetLabelObject(e5)
    effect::set_label_object(d, e6, Value::Effect(e5));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // Destroy2
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e7, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e7:SetRange(LOCATION_SZONE)
    effect::set_range(d, e7, LOCATION_SZONE);
    // e7:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e7, EVENT_LEAVE_FIELD);
    // e7:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e7, EFFECT_FLAG_DELAY, 0);
    // e7:SetCondition(s.descon2)
    effect::set_condition(d, e7, Some(condition(descon2)));
    // e7:SetOperation(s.desop2)
    effect::set_operation(d, e7, Some(operation(desop2)));
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttackPos()
    card::is_faceup(d, c) && card::is_attack_pos(d, c)
}
/// `s.op(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc and tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, e) {
        if let Some(tc) = tc {
            if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, e) {
                // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,tp)
                card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, tp as i64, 0);
            }
        }
    }
}
/// `s.efilter(e,re)`: `return e:GetOwnerPlayer()~=re:GetOwnerPlayer()`
fn efilter(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let re = a.e(1);
    effect::get_owner_player(d, e) != effect::get_owner_player(d, re)
}
/// `s.tgval(e,re,rp)`: `return rp~=e:GetOwnerPlayer()`
fn tgval(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let rp = a.int(2);
    rp != effect::get_owner_player(d, e) as i64
}
/// `s.acon(e)`
fn acon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(id)>0 and e:GetHandler():GetFlagEffectLabel(id)==e:GetHandler():GetFirstCardTarget():GetControler()
    let h = effect::get_handler(d, a.e).nil();
    if card::get_flag_effect(d, h, ID) == 0 {
        return false;
    }
    let label = card::get_flag_effect_label(d, h, ID).first().copied();
    let tc = card::get_first_card_target(d, h).nil();
    label == Some(card::get_controler(d, tc) as i64)
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // if e:GetHandler():IsDisabled() then e:SetLabel(1)
    let h = effect::get_handler(d, e).nil();
    if card::is_disabled(d, h) {
        effect::set_label(d, e, &[1]);
    } else {
        // else e:SetLabel(0) end
        effect::set_label(d, e, &[0]);
    }
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // if e:GetLabelObject():GetLabel()~=0 then return end
    let le = effect::get_label_object(d, e).as_effect().nil();
    if effect::get_label(d, le) != 0 {
        return;
    }
    // local tc=e:GetHandler():GetFirstCardTarget()
    let h = effect::get_handler(d, e).nil();
    let tc = card::get_first_card_target(d, h);
    // if tc and tc:IsLocation(LOCATION_MZONE) then
    if let Some(tc) = tc {
        if card::is_location(d, tc, LOCATION_MZONE) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
/// `s.descon2(e,tp,eg,ep,ev,re,r,rp)`
fn descon2(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetHandler():GetFirstCardTarget()
    let h = effect::get_handler(d, a.e).nil();
    // return tc and eg:IsContains(tc)
    match card::get_first_card_target(d, h) {
        None => false,
        Some(tc) => {
            let eg = a.eg.clone().nil();
            group::is_contains(&eg, tc)
        }
    }
}
/// `s.desop2(e,tp,eg,ep,ev,re,r,rp)`
fn desop2(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetHandler(), REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::destroy(d, h, REASON_EFFECT, None, None);
}
