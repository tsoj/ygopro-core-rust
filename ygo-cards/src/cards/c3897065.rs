//! Super Vehicroid - Stealth Union
use super::prelude::*;

pub const ID: u32 = 3897065;

pub const INFO: ScriptInfo = ScriptInfo {
    material_setcode: &[SET_ROID],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,61538782,98049038,71218746,984114)
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![fusion::Mat::Code(61538782), fusion::Mat::Code(98049038), fusion::Mat::Code(71218746), fusion::Mat::Code(984114)],
    );
    // equip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.AddEREquipLimit(c,nil,aux.NOT(aux.FilterBoolFunction(Card.IsRace,RACE_MACHINE)),s.equipop,e1)
    aux::add_er_equip_limit(d, c, None, lua_fn(eqval), lua_fn(equipop_lua), e1, None, None, None);
    // attack all
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_ATTACK_ALL)
    effect::set_code(d, e2, EFFECT_ATTACK_ALL);
    // e2:SetCondition(s.atcon)
    effect::set_condition(d, e2, Some(condition(atcon)));
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // pierce
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e3, EFFECT_PIERCE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // atk
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e4, EVENT_ATTACK_ANNOUNCE);
    // e4:SetOperation(s.atkop)
    effect::set_operation(d, e4, Some(operation(atkop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.eqfilter(c,tp)`
fn eqfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and not c:IsRace(RACE_MACHINE) and (c:IsControler(tp) or c:IsAbleToChangeControler())
    card::is_faceup(d, c) && !card::is_race(d, c, RACE_MACHINE) && (card::is_controler(d, c, tp) || card::is_able_to_change_controler(d, c))
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.eqfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && eqfilter(d, chkc, tp);
    }
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(s.eqfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,e:GetHandler(),tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, x| eqfilter(d, x, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectTarget(tp,s.eqfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,e:GetHandler(),tp)
    let g = duel::select_target(d, tp, Some(&|d, x| eqfilter(d, x, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, c.into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(g), 1, 0, 0);
    true
}
/// `s.equipop(c,e,tp,tc)`
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) {
    // if not c:EquipByEffectAndLimitRegister(e,tp,tc) then return end
    if !card::equip_by_effect_and_limit_register(d, c, e, tp, tc, None, None) {
        return;
    }
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetCode(id)
    effect::set_code(d, e2, ID);
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    // tc:RegisterEffect(e2)
    card::register_effect(d, tc, e2, false);
}
/// Lua wrapper of `s.equipop(c,e,tp,tc)` (called by `aux.AddEREquipLimit`)
fn equipop_lua(d: &mut Duel, a: Args) {
    equipop(d, a.c(0), a.e(1), a.p(2), a.c(3));
}
/// `aux.NOT(aux.FilterBoolFunction(Card.IsRace,RACE_MACHINE))` (the `equipval` of `aux.AddEREquipLimit`)
fn eqval(d: &mut Duel, a: Args) -> bool {
    !card::is_race(d, a.c(0), RACE_MACHINE)
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsFaceup() and tc:IsMonster() and not tc:IsRace(RACE_MACHINE) and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_monster(d, tc) && !card::is_race(d, tc, RACE_MACHINE) && card::is_relate_to_effect(d, tc, a.e) {
            // s.equipop(c,e,tp,tc)
            equipop(d, c, a.e, a.tp, tc);
        }
    }
}
/// `s.atcon(e)`
fn atcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsHasEffect(id)
    let c = effect::get_handler(d, a.e).nil();
    !card::is_has_effect(d, c, ID, None).is_empty()
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local atk=c:GetBaseAttack()
    let atk = card::get_base_attack(d, c);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_BASE_ATTACK)
    effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
    // e1:SetValue(atk/2)
    effect::set_value(d, e1, lua_int(atk as f64 / 2.0));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
