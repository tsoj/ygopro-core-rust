//! Inspector Boarder
use super::prelude::*;

pub const ID: u32 = 15397015;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon limit
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e0, EFFECT_CANNOT_SUMMON);
    // e0:SetCondition(s.sumcon)
    effect::set_condition(d, e0, Some(lua_fn(sumcon)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(s.sumlimit)
    effect::set_value_fn(d, e1, lua_fn(sumlimit));
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
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e4, EFFECT_CANNOT_ACTIVATE);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTargetRange(1,0)
    effect::set_target_range(d, e4, 1, 0);
    // e4:SetCondition(s.econ1)
    effect::set_condition(d, e4, Some(lua_fn(econ1)));
    // e4:SetValue(s.elimit)
    effect::set_value_fn(d, e4, lua_fn(elimit));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e2:Clone()
    let e5 = effect::clone(d, e2);
    // e5:SetOperation(s.aclimit3)
    effect::set_operation(d, e5, Some(operation(aclimit3)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // local e6=e3:Clone()
    let e6 = effect::clone(d, e3);
    // e6:SetOperation(s.aclimit4)
    effect::set_operation(d, e6, Some(operation(aclimit4)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // local e7=e4:Clone()
    let e7 = effect::clone(d, e4);
    // e7:SetCondition(s.econ2)
    effect::set_condition(d, e7, Some(lua_fn(econ2)));
    // e7:SetTargetRange(0,1)
    effect::set_target_range(d, e7, 0, 1);
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
}
/// `s.sumcon(e)`
fn sumcon(d: &mut Duel, a: Args) -> bool {
    // return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),LOCATION_MZONE,0)>0
    let p = effect::get_handler_player(d, a.e(0));
    duel::get_field_group_count(d, p, LOCATION_MZONE, 0) > 0
}
/// `s.sumlimit(e,se,sp,st,pos,tp)`
fn sumlimit(d: &mut Duel, a: Args) -> bool {
    // return Duel.GetFieldGroupCount(sp,LOCATION_MZONE,0)==0
    duel::get_field_group_count(d, a.p(2), LOCATION_MZONE, 0) == 0
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_FUSION+TYPE_RITUAL+TYPE_SYNCHRO+TYPE_XYZ+TYPE_PENDULUM+TYPE_LINK)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_FUSION + TYPE_RITUAL + TYPE_SYNCHRO + TYPE_XYZ + TYPE_PENDULUM + TYPE_LINK)
}
/// `s.typecount(c)`
fn typecount(d: &mut Duel, c: CardId) -> i64 {
    // return c:GetType()&TYPE_FUSION+TYPE_RITUAL+TYPE_SYNCHRO+TYPE_XYZ+TYPE_PENDULUM+TYPE_LINK
    (card::get_type(d, c) & (TYPE_FUSION + TYPE_RITUAL + TYPE_SYNCHRO + TYPE_XYZ + TYPE_PENDULUM + TYPE_LINK)) as i64
}
/// `s.aclimit1(e,tp,eg,ep,ev,re,r,rp)`
fn aclimit1(d: &mut Duel, a: &EffArgs) {
    // if ep==1-tp or not re:IsMonsterEffect() then return end
    if a.ep == 1 - a.tp || !effect::is_monster_effect(d, a.re.nil()) {
        return;
    }
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD_DISABLE|RESET_CONTROL|RESET_PHASE|PHASE_END,0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD_DISABLE | RESET_CONTROL | RESET_PHASE | PHASE_END, 0, 1, 0, 0);
}
/// `s.aclimit2(e,tp,eg,ep,ev,re,r,rp)`
fn aclimit2(d: &mut Duel, a: &EffArgs) {
    // if ep~=tp or not re:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if a.ep != a.tp || !effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // e:GetHandler():ResetFlagEffect(id)
    let h = effect::get_handler(d, a.e).nil();
    card::reset_flag_effect(d, h, ID);
}
/// `s.econ1(e)`
fn econ1(d: &mut Duel, a: Args) -> bool {
    // local g=Duel.GetMatchingGroup(s.cfilter,e:GetHandlerPlayer(),LOCATION_MZONE,LOCATION_MZONE,nil)
    let p = effect::get_handler_player(d, a.e(0));
    let g = duel::get_matching_group(d, Some(&cfilter), p, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local ct=g:GetClassCount(s.typecount)
    let ct = group::get_class_count(d, &g, &|d, c| typecount(d, c));
    // return e:GetHandler():GetFlagEffect(id)>=ct
    let h = effect::get_handler(d, a.e(0)).nil();
    card::get_flag_effect(d, h, ID) >= ct
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
/// `s.aclimit4(e,tp,eg,ep,ev,re,r,rp)`
fn aclimit4(d: &mut Duel, a: &EffArgs) {
    // if ep==tp or not re:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if a.ep == a.tp || !effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // e:GetHandler():ResetFlagEffect(id+1)
    let h = effect::get_handler(d, a.e).nil();
    card::reset_flag_effect(d, h, ID + 1);
}
/// `s.econ2(e)`
fn econ2(d: &mut Duel, a: Args) -> bool {
    // local g=Duel.GetMatchingGroup(s.cfilter,e:GetHandlerPlayer(),LOCATION_MZONE,LOCATION_MZONE,nil)
    let p = effect::get_handler_player(d, a.e(0));
    let g = duel::get_matching_group(d, Some(&cfilter), p, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local ct=g:GetClassCount(s.typecount)
    let ct = group::get_class_count(d, &g, &|d, c| typecount(d, c));
    // return e:GetHandler():GetFlagEffect(id+1)>=ct
    let h = effect::get_handler(d, a.e(0)).nil();
    card::get_flag_effect(d, h, ID + 1) >= ct
}
/// `s.elimit(e,re,tp)`
fn elimit(d: &mut Duel, a: Args) -> bool {
    // return re:IsMonsterEffect()
    effect::is_monster_effect(d, a.e(1))
}
