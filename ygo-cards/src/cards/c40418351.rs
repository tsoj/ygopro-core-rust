//! Cyberdark Dragon
use super::prelude::*;

pub const ID: u32 = 40418351;

pub const INFO: ScriptInfo = ScriptInfo { material_setcode: &[SET_CYBER, SET_CYBERDARK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,41230939,77625948,3019642)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(41230939), fusion::Mat::Code(77625948), fusion::Mat::Code(3019642)]);
    // equip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(Cyberdark.EquipTarget(aux.FilterBoolFunction(Card.IsRace,RACE_DRAGON),true,true))
    effect::set_target(d, e1, Some(aux::cyberdark::equip_target(Some(dragon_filter()), true, true)));
    // e1:SetOperation(Cyberdark.EquipOperation(aux.FilterBoolFunction(Card.IsRace,RACE_DRAGON),s.equipop,true))
    effect::set_operation(d, e1, Some(aux::cyberdark::equip_operation(Some(dragon_filter()), lua_fn(equipop), true)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.AddEREquipLimit(c,nil,s.eqval,s.equipop,e1)
    aux::add_er_equip_limit(d, c, None, lua_fn(eqval), lua_fn(equipop), e1, None, None, None);
    // atkup
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, lua_fn(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // spsummon condition
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e3:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e3, EFFECT_SPSUMMON_CONDITION);
    // e3:SetValue(aux.fuslimit)
    effect::set_value_fn(d, e3, aux::fuslimit());
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `aux.FilterBoolFunction(Card.IsRace,RACE_DRAGON)`
fn dragon_filter() -> CardFilter {
    std::sync::Arc::new(|d: &mut Duel, c: CardId| card::is_race(d, c, RACE_DRAGON))
}
/// `function(ec,c,tp) return ec:IsControler(tp) and ec:IsRace(RACE_DRAGON) end`
fn eqval(d: &mut Duel, a: Args) -> bool {
    card::is_controler(d, a.c(0), a.p(2)) && card::is_race(d, a.c(0), RACE_DRAGON)
}
/// `s.equipop(c,e,tp,tc)`
fn equipop(d: &mut Duel, a: Args) {
    let (c, e, tp, tc) = (a.c(0), a.e(1), a.p(2), a.c(3));
    // local atk=tc:GetTextAttack()
    let mut atk = card::get_text_attack(d, tc);
    // if atk<0 then atk=0 end
    if atk < 0 {
        atk = 0;
    }
    // if not c:EquipByEffectAndLimitRegister(e,tp,tc,nil,true) then return end
    if !card::equip_by_effect_and_limit_register(d, c, e, tp, tc, None, Some(true)) {
        return;
    }
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetProperty(EFFECT_FLAG_OWNER_RELATE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_OWNER_RELATE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    // e2:SetValue(atk)
    effect::set_value(d, e2, atk as i64);
    // tc:RegisterEffect(e2)
    card::register_effect(d, tc, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    // e3:SetCode(EFFECT_DESTROY_SUBSTITUTE)
    effect::set_code(d, e3, EFFECT_DESTROY_SUBSTITUTE);
    // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
    // e3:SetValue(s.repval)
    effect::set_value_fn(d, e3, lua_fn(repval));
    // tc:RegisterEffect(e3)
    card::register_effect(d, tc, e3, false);
}
/// `s.repval(e,re,r,rp)`
fn repval(_d: &mut Duel, a: Args) -> bool {
    // return (r&REASON_BATTLE)~=0
    ((a.int(2) as u32) & REASON_BATTLE) != 0
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, a: Args) -> i64 {
    // return Duel.GetMatchingGroupCount(Card.IsMonster,e:GetHandlerPlayer(),LOCATION_GRAVE,0,nil)*100
    let p = effect::get_handler_player(d, a.e(0));
    duel::get_matching_group_count(d, Some(&|d, c| card::is_monster(d, c)), p, LOCATION_GRAVE, 0, NO_EX) as i64 * 100
}
