//! Performapal Five-Rainbow Magician
use super::prelude::*;

pub const ID: u32 = 19619755;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Place itself from the GY in the Pendulum Zone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SSET)
    effect::set_code(d, e1, EVENT_SSET);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCondition(s.pencon)
    effect::set_condition(d, e1, Some(condition(pencon)));
    // e1:SetTarget(s.pentg)
    effect::set_target(d, e1, Some(cost_target(pentg)));
    // e1:SetOperation(s.penop)
    effect::set_operation(d, e1, Some(operation(penop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You cannot Pendulum Summon, except from the Extra Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CANNOT_NEGATE)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CANNOT_NEGATE, 0);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetTarget(s.splimit)
    effect::set_target(d, e2, Some(lua_fn(splimit)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Monsters cannot attack
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e3, EFFECT_CANNOT_ATTACK);
    // e3:SetRange(LOCATION_PZONE)
    effect::set_range(d, e3, LOCATION_PZONE);
    // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    // e3:SetTarget(s.atktg)
    effect::set_target(d, e3, Some(effect_card_fn(atktg)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Monsters cannot activate their effects
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e4, EFFECT_CANNOT_ACTIVATE);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetRange(LOCATION_PZONE)
    effect::set_range(d, e4, LOCATION_PZONE);
    // e4:SetTargetRange(1,1)
    effect::set_target_range(d, e4, 1, 1);
    // e4:SetValue(s.limval)
    effect::set_value_fn(d, e4, lua_fn(limval));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Double the ATK of monsters on the field
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    // e5:SetCode(EFFECT_SET_ATTACK_FINAL)
    effect::set_code(d, e5, EFFECT_SET_ATTACK_FINAL);
    // e5:SetRange(LOCATION_PZONE)
    effect::set_range(d, e5, LOCATION_PZONE);
    // e5:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e5, LOCATION_MZONE, 0);
    // e5:SetCondition(s.atkcon0)
    effect::set_condition(d, e5, Some(lua_fn(atkcon0)));
    // e5:SetValue(s.atkval)
    effect::set_value_fn(d, e5, effect_card_value(atkval));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // local e6=e5:Clone()
    let e6 = effect::clone(d, e5);
    // e6:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e6, 0, LOCATION_MZONE);
    // e6:SetCondition(s.atkcon1)
    effect::set_condition(d, e6, Some(lua_fn(atkcon1)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.pencon(e,tp,eg,ep,ev,re,r,rp)`
fn pencon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(Card.IsControler,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| card::is_controler(d, c, tp), 1, NO_EX)
}
/// `s.pentg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pentg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckPendulumZones(tp) end
    if a.chk == Some(0) {
        return duel::check_pendulum_zones(d, a.tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.penop(e,tp,eg,ep,ev,re,r,rp)`
fn penop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) and Duel.CheckPendulumZones(tp) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) && duel::check_pendulum_zones(d, a.tp) {
        // Duel.MoveToField(e:GetHandler(),tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, h, a.tp, a.tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
/// `s.splimit(e,c,tp,sumtp,sumpos)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return (sumtp&SUMMON_TYPE_PENDULUM)==SUMMON_TYPE_PENDULUM and not c:IsLocation(LOCATION_EXTRA)
    let sumtp = a.int(3) as u32;
    let c = a.c(1);
    (sumtp & SUMMON_TYPE_PENDULUM) == SUMMON_TYPE_PENDULUM && !card::is_location(d, c, LOCATION_EXTRA)
}
/// `s.countfilter(c)`
fn countfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() and c:GetSequence()<5
    card::is_facedown(d, c) && card::get_sequence(d, c) < 5
}
/// `s.atktg(e,c)`
fn atktg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetMatchingGroupCount(s.countfilter,tp,LOCATION_SZONE,0,nil)==0
    duel::get_matching_group_count(d, Some(&countfilter), tp, LOCATION_SZONE, 0, NO_EX) == 0
}
/// `s.limval(e,re,rp)`
fn limval(d: &mut Duel, a: Args) -> bool {
    // local rc=re:GetHandler()
    let re = a.e(1);
    let rc = effect::get_handler(d, re).nil();
    // local tp=rc:GetControler()
    let tp = card::get_controler(d, rc);
    // return rc:IsLocation(LOCATION_MZONE) and re:IsMonsterEffect()
    //     and Duel.GetMatchingGroupCount(s.countfilter,tp,LOCATION_SZONE,0,nil)==0
    card::is_location(d, rc, LOCATION_MZONE)
        && effect::is_monster_effect(d, re)
        && duel::get_matching_group_count(d, Some(&countfilter), tp, LOCATION_SZONE, 0, NO_EX) == 0
}
/// `s.atkcon0(e)`
fn atkcon0(d: &mut Duel, a: Args) -> bool {
    // return Duel.GetMatchingGroupCount(s.countfilter,e:GetHandlerPlayer(),LOCATION_SZONE,0,nil)>=4
    let hp = effect::get_handler_player(d, a.e(0));
    duel::get_matching_group_count(d, Some(&countfilter), hp, LOCATION_SZONE, 0, NO_EX) >= 4
}
/// `s.atkcon1(e)`
fn atkcon1(d: &mut Duel, a: Args) -> bool {
    // return Duel.GetMatchingGroupCount(s.countfilter,e:GetHandlerPlayer(),0,LOCATION_SZONE,nil)>=4
    let hp = effect::get_handler_player(d, a.e(0));
    duel::get_matching_group_count(d, Some(&countfilter), hp, 0, LOCATION_SZONE, NO_EX) >= 4
}
/// `s.atkval(e,c)`: `c:GetBaseAttack()*2`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    card::get_base_attack(d, c) as i64 * 2
}
