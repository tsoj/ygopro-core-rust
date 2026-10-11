//! Medium of the Ice Barrier
//! 氷結界の交霊師
use super::prelude::*;

pub const ID: u32 = 88494899;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // activate limit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetOperation(s.aclimit1)
    effect::set_operation(d, e2, Some(operation(aclimit1)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e3:SetCode(EVENT_CHAIN_NEGATED)
    effect::set_code(d, e3, EVENT_CHAIN_NEGATED);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetOperation(s.aclimit2)
    effect::set_operation(d, e3, Some(operation(aclimit2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e4, EFFECT_CANNOT_ACTIVATE);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTargetRange(0,1)
    effect::set_target_range(d, e4, 0, 1);
    // e4:SetCondition(s.econ)
    effect::set_condition(d, e4, Some(condition(econ)));
    // e4:SetValue(s.elimit)
    effect::set_value_fn(d, e4, lua_fn(elimit));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.spcon(e,c)
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let c = match a.card(1) {
        None => return true,
        Some(c) => c,
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)-Duel.GetFieldGroupCount(tp,LOCATION_ONFIELD,0)>=4
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD) - duel::get_field_group_count(d, tp, LOCATION_ONFIELD, 0) >= 4
}
// function s.aclimit1(e,tp,eg,ep,ev,re,r,rp)
fn aclimit1(d: &mut Duel, a: &EffArgs) {
    // if ep==tp or not re:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if a.ep == a.tp || !effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD_DISABLE|RESET_CONTROL|RESET_PHASE|PHASE_END,0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD_DISABLE | RESET_CONTROL | RESET_PHASE | PHASE_END, 0, 1, 0, 0);
}
// function s.aclimit2(e,tp,eg,ep,ev,re,r,rp)
fn aclimit2(d: &mut Duel, a: &EffArgs) {
    // if ep==tp or not re:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if a.ep == a.tp || !effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // e:GetHandler():ResetFlagEffect(id)
    let h = effect::get_handler(d, a.e).nil();
    card::reset_flag_effect(d, h, ID);
}
// function s.econ(e)
fn econ(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(id)~=0
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) != 0
}
// function s.elimit(e,te,tp)
fn elimit(d: &mut Duel, a: Args) -> bool {
    // return te:IsHasType(EFFECT_TYPE_ACTIVATE)
    let te = a.e(1);
    effect::is_has_type(d, te, EFFECT_TYPE_ACTIVATE)
}
