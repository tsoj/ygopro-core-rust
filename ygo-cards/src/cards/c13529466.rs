//! Invoked Caliga
use super::prelude::*;

pub const ID: u32 = 13529466;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,86120751,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_DARK))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Code(86120751),
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_attribute_ctx(d, x, ATTRIBUTE_DARK, Some(fc), st, Some(tp)))),
        ],
    );
    // activate limit
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetOperation(s.aclimit1)
    effect::set_operation(d, e1, Some(operation(aclimit1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e3, EFFECT_CANNOT_ACTIVATE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // e3:SetCondition(s.econ1)
    effect::set_condition(d, e3, Some(condition(econ1)));
    // e3:SetValue(s.elimit)
    effect::set_value_fn(d, e3, lua_fn(elimit));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e1:Clone()
    // e4:SetOperation(s.aclimit3)
    // c:RegisterEffect(e4)
    let e4 = effect::clone(d, e1);
    effect::set_operation(d, e4, Some(operation(aclimit3)));
    card::register_effect(d, c, e4, false);
    // local e6=e3:Clone()
    // e6:SetCondition(s.econ2)
    // e6:SetTargetRange(0,1)
    // c:RegisterEffect(e6)
    let e6 = effect::clone(d, e3);
    effect::set_condition(d, e6, Some(condition(econ2)));
    effect::set_target_range(d, e6, 0, 1);
    card::register_effect(d, c, e6, false);
    // attack limit
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e7, EFFECT_TYPE_FIELD);
    // e7:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e7, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e7:SetCode(EFFECT_CANNOT_ATTACK_ANNOUNCE)
    effect::set_code(d, e7, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    // e7:SetRange(LOCATION_MZONE)
    effect::set_range(d, e7, LOCATION_MZONE);
    // e7:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e7, LOCATION_MZONE, LOCATION_MZONE);
    // e7:SetCondition(s.atkcon)
    effect::set_condition(d, e7, Some(condition(atkcon)));
    // e7:SetTarget(s.atktg)
    effect::set_target(d, e7, Some(effect_card_fn(atktg)));
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
    // local e8=Effect.CreateEffect(c)
    let e8 = effect::create_effect(d, c);
    // e8:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e8, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e8:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e8, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e8:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e8, EVENT_ATTACK_ANNOUNCE);
    // e8:SetRange(LOCATION_MZONE)
    effect::set_range(d, e8, LOCATION_MZONE);
    // e8:SetOperation(s.checkop)
    effect::set_operation(d, e8, Some(operation(checkop)));
    // e8:SetLabelObject(e7)
    effect::set_label_object(d, e8, Value::Effect(e7));
    // c:RegisterEffect(e8)
    card::register_effect(d, c, e8, false);
}
/// `s.aclimit1(e,tp,eg,ep,ev,re,r,rp)`
fn aclimit1(d: &mut Duel, a: &EffArgs) {
    // if ep~=tp or not re:IsMonsterEffect() then return end
    if a.ep != a.tp || !effect::is_monster_effect(d, a.re.nil()) {
        return;
    }
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD_DISABLE|RESET_CONTROL|RESET_PHASE|PHASE_END,0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD_DISABLE | RESET_CONTROL | RESET_PHASE | PHASE_END, 0, 1, 0, 0);
}
/// `s.econ1(e)`
fn econ1(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(id)~=0
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) != 0
}
/// `s.aclimit3(e,tp,eg,ep,ev,re,r,rp)`
fn aclimit3(d: &mut Duel, a: &EffArgs) {
    // if ep==tp or not re:IsMonsterEffect() then return end
    if a.ep == a.tp || !effect::is_monster_effect(d, a.re.nil()) {
        return;
    }
    // e:GetHandler():RegisterFlagEffect(id+1,RESET_EVENT|RESETS_STANDARD_DISABLE|RESET_CONTROL|RESET_PHASE|PHASE_END,0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID + 1, RESET_EVENT | RESETS_STANDARD_DISABLE | RESET_CONTROL | RESET_PHASE | PHASE_END, 0, 1, 0, 0);
}
/// `s.econ2(e)`
fn econ2(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(id+1)~=0
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID + 1) != 0
}
/// `s.elimit(e,re,tp)`
fn elimit(d: &mut Duel, a: Args) -> bool {
    // return re:IsMonsterEffect()
    effect::is_monster_effect(d, a.e(1))
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(id+2)~=0
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID + 2) != 0
}
/// `s.atktg(e,c)`
fn atktg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:GetFieldID()~=e:GetLabel()
    card::get_field_id(d, c) as i64 != effect::get_label(d, e)
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():GetFlagEffect(id+2)~=0 then return end
    let h = effect::get_handler(d, a.e).nil();
    if card::get_flag_effect(d, h, ID + 2) != 0 {
        return;
    }
    // local fid=eg:GetFirst():GetFieldID()
    let eg = a.eg.clone().nil();
    let first = group::get_first(d, &eg).nil();
    let fid = card::get_field_id(d, first) as i64;
    // e:GetHandler():RegisterFlagEffect(id+2,RESETS_STANDARD_PHASE_END,0,1)
    card::register_flag_effect(d, h, ID + 2, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    // e:GetLabelObject():SetLabel(fid)
    let e7 = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_label(d, e7, &[fid]);
}
